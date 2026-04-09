use std::collections::{BTreeSet, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde::Serialize;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, Url};
use walkdir::WalkDir;

use crate::common::util;
use crate::host::config::Spec42Config;
use crate::workspace::{
    indexed_text_or_empty, ingest_parsed_scan_entries, parse_scanned_entries,
    rebuild_all_document_links, scan_sysml_files, ServerState,
};

#[derive(Debug, Clone)]
pub struct ValidationRequest {
    pub targets: Vec<PathBuf>,
    pub workspace_root: Option<PathBuf>,
    pub library_paths: Vec<PathBuf>,
    pub parallel_enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationReport {
    pub workspace_root: Option<String>,
    pub resolved_library_paths: Vec<String>,
    pub documents: Vec<ValidatedDocument>,
    pub summary: ValidationSummary,
    pub advice: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidatedDocument {
    pub uri: String,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ValidationSummary {
    pub document_count: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub information_count: usize,
}

pub fn validate_paths(
    config: &Arc<Spec42Config>,
    request: ValidationRequest,
) -> Result<ValidationReport, String> {
    let workspace_root = resolve_workspace_root(&request)?;
    let target_files = discover_target_files(&request.targets)?;
    if target_files.is_empty() {
        return Err("No .sysml or .kerml files were found under the requested path.".to_string());
    }

    let workspace_root_url = workspace_root
        .as_ref()
        .map(|path| path_to_file_url(path.as_path()))
        .transpose()?;
    let library_root_urls = request
        .library_paths
        .iter()
        .map(|path| path_to_file_url(path.as_path()))
        .collect::<Result<Vec<_>, _>>()?;

    let mut state = ServerState::default();
    state.workspace_roots = workspace_root_url.iter().cloned().collect();
    state.library_paths = library_root_urls.clone();

    let mut entries = Vec::new();
    let scan_roots: Vec<Url> = workspace_root_url
        .iter()
        .cloned()
        .chain(library_root_urls.iter().cloned())
        .collect();
    if !scan_roots.is_empty() {
        let (scanned_entries, _) = scan_sysml_files(scan_roots);
        entries.extend(scanned_entries);
    }

    let mut seen = HashSet::new();
    for (uri, _) in &entries {
        seen.insert(uri.as_str().to_string());
    }
    for path in &target_files {
        let uri = path_to_file_url(path)?;
        if seen.insert(uri.as_str().to_string()) {
            let content = std::fs::read_to_string(path)
                .map_err(|err| format!("Failed to read {}: {err}", path.display()))?;
            entries.push((uri, content));
        }
    }

    let parsed_entries = parse_scanned_entries(entries, request.parallel_enabled);
    ingest_parsed_scan_entries(&mut state, parsed_entries);
    rebuild_all_document_links(&mut state);

    let target_urls = target_files
        .iter()
        .map(|path| path_to_file_url(path.as_path()))
        .collect::<Result<BTreeSet<_>, _>>()?;

    let documents = target_urls
        .into_iter()
        .map(|uri| {
            let text = indexed_text_or_empty(&state, &uri);
            let diagnostics = collect_diagnostics_for_document(&state, config, &uri, &text);
            ValidatedDocument {
                uri: uri.to_string(),
                diagnostics,
            }
        })
        .collect::<Vec<_>>();

    let summary = summarize(&documents);
    let advice = build_advice(&documents, request.library_paths.is_empty());

    Ok(ValidationReport {
        workspace_root: workspace_root.map(|path| path.display().to_string()),
        resolved_library_paths: request
            .library_paths
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        documents,
        summary,
        advice,
    })
}

fn summarize(documents: &[ValidatedDocument]) -> ValidationSummary {
    let mut summary = ValidationSummary {
        document_count: documents.len(),
        ..ValidationSummary::default()
    };
    for document in documents {
        for diagnostic in &document.diagnostics {
            match diagnostic.severity.unwrap_or(DiagnosticSeverity::ERROR) {
                DiagnosticSeverity::ERROR => summary.error_count += 1,
                DiagnosticSeverity::WARNING => summary.warning_count += 1,
                DiagnosticSeverity::INFORMATION | DiagnosticSeverity::HINT => {
                    summary.information_count += 1
                }
                _ => {}
            }
        }
    }
    summary
}

fn build_advice(documents: &[ValidatedDocument], no_library_paths: bool) -> Vec<String> {
    if !no_library_paths {
        return Vec::new();
    }
    let has_missing_library_context = documents.iter().any(|document| {
        document.diagnostics.iter().any(|diagnostic| {
            diagnostic.code.as_ref()
                == Some(&NumberOrString::String(
                    "missing_library_context".to_string(),
                ))
        })
    });
    let has_unresolved_type_reference = documents.iter().any(|document| {
        document.diagnostics.iter().any(|diagnostic| {
            diagnostic.code.as_ref()
                == Some(&NumberOrString::String(
                    "unresolved_type_reference".to_string(),
                ))
        })
    });
    if has_missing_library_context || has_unresolved_type_reference {
        vec![
            "Install the managed SysML standard library with `spec42 stdlib install`, or pass `--stdlib-path`/`--library-path` explicitly."
                .to_string(),
        ]
    } else {
        Vec::new()
    }
}

fn resolve_workspace_root(request: &ValidationRequest) -> Result<Option<PathBuf>, String> {
    if let Some(root) = &request.workspace_root {
        return normalize_existing_path(root).map(Some);
    }
    let first = request
        .targets
        .first()
        .ok_or_else(|| "No target path was provided.".to_string())?;
    if first.is_dir() {
        return normalize_existing_path(first).map(Some);
    }
    normalize_existing_path(first)?
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| {
            format!(
                "Could not infer a workspace root from target file {}.",
                first.display()
            )
        })
        .map(Some)
}

fn discover_target_files(targets: &[PathBuf]) -> Result<Vec<PathBuf>, String> {
    let mut files = BTreeSet::new();
    for target in targets {
        let path = normalize_existing_path(target)?;
        if path.is_file() {
            if is_sysml_like(&path) {
                files.insert(path);
            }
            continue;
        }
        for entry in WalkDir::new(&path)
            .follow_links(false)
            .into_iter()
            .filter_map(Result::ok)
        {
            if !entry.file_type().is_file() {
                continue;
            }
            let entry_path = entry.path().to_path_buf();
            if is_sysml_like(&entry_path) {
                files.insert(entry_path);
            }
        }
    }
    Ok(files.into_iter().collect())
}

fn normalize_existing_path(path: &Path) -> Result<PathBuf, String> {
    let path = path
        .canonicalize()
        .map_err(|err| format!("Failed to resolve {}: {err}", path.display()))?;
    if !path.exists() {
        return Err(format!("Path does not exist: {}", path.display()));
    }
    Ok(path)
}

fn is_sysml_like(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("sysml") | Some("kerml")
    )
}

fn path_to_file_url(path: &Path) -> Result<Url, String> {
    Url::from_file_path(path)
        .map(|uri| util::normalize_file_uri(&uri))
        .map_err(|_| format!("Could not convert {} to file:// URL.", path.display()))
}

fn collect_diagnostics_for_document(
    state: &ServerState,
    config: &Arc<Spec42Config>,
    uri: &Url,
    text: &str,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let result = util::parse_for_editor(text);
    for error in &result.errors {
        let range = error
            .to_lsp_range()
            .map(|(sl, sc, el, ec)| Range {
                start: Position::new(sl, sc),
                end: Position::new(el, ec),
            })
            .unwrap_or_else(|| Range {
                start: Position::new(0, 0),
                end: Position::new(0, 0),
            });
        let severity = error
            .severity
            .map(|severity| match severity {
                sysml_v2_parser::DiagnosticSeverity::Error => DiagnosticSeverity::ERROR,
                sysml_v2_parser::DiagnosticSeverity::Warning => DiagnosticSeverity::WARNING,
            })
            .unwrap_or(DiagnosticSeverity::ERROR);
        diagnostics.push(Diagnostic {
            range,
            severity: Some(severity),
            code: error.code.clone().map(NumberOrString::String),
            code_description: None,
            source: Some("sysml".to_string()),
            message: error.message.clone(),
            related_information: None,
            tags: None,
            data: None,
        });
    }
    for usage in util::untyped_part_usage_diagnostics(text) {
        diagnostics.push(Diagnostic {
            range: usage.range,
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(NumberOrString::String("untyped_part_usage".to_string())),
            code_description: None,
            source: Some("sysml".to_string()),
            message: format!("Part '{}' has no declared type.", usage.name),
            related_information: None,
            tags: None,
            data: None,
        });
    }
    if result.errors.is_empty() {
        for range in util::missing_semicolon_ranges(text) {
            diagnostics.push(Diagnostic {
                range,
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("missing_semicolon".to_string())),
                code_description: None,
                source: Some("sysml".to_string()),
                message: "Missing ';' at end of statement.".to_string(),
                related_information: None,
                tags: None,
                data: None,
            });
        }
        for provider in &config.check_providers {
            diagnostics.extend(provider.compute_diagnostics(&state.semantic_graph, uri));
        }
        let has_unresolved_type_reference = diagnostics.iter().any(|diagnostic| {
            diagnostic.source.as_deref() == Some("semantic")
                && diagnostic.code.as_ref()
                    == Some(&NumberOrString::String(
                        "unresolved_type_reference".to_string(),
                    ))
        });
        if has_unresolved_type_reference && state.library_paths.is_empty() {
            if let Some(import_range) = util::import_statement_ranges(text).into_iter().next() {
                diagnostics.push(Diagnostic {
                    range: import_range,
                    severity: Some(DiagnosticSeverity::INFORMATION),
                    code: Some(NumberOrString::String(
                        "missing_library_context".to_string(),
                    )),
                    code_description: None,
                    source: Some("semantic".to_string()),
                    message: "This document imports external library symbols, but no SysML library paths are configured or indexed. Install or configure a library if these references should resolve.".to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }
        }
    }
    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;

    fn timer_like_model() -> &'static str {
        r#"
package KitchenTimer {
    private import ScalarValues::*;
    private import ISQ::DurationValue;

    part def Battery {
        attribute capacity : Real;
        attribute runtimeEstimate : DurationValue;
    }
}
"#
    }

    fn write_timer_fixture(temp: &tempfile::TempDir) -> PathBuf {
        let model_path = temp.path().join("KitchenTimer.sysml");
        std::fs::write(&model_path, timer_like_model()).expect("write timer fixture");
        model_path
    }

    fn write_stdlib_fixture(temp: &tempfile::TempDir) -> PathBuf {
        let stdlib_root = temp.path().join("sysml.library");
        std::fs::create_dir_all(&stdlib_root).expect("create stdlib root");
        std::fs::write(
            stdlib_root.join("ScalarValues.sysml"),
            "standard library package ScalarValues { attribute def Real; }",
        )
        .expect("write ScalarValues");
        std::fs::write(
            stdlib_root.join("ISQ.sysml"),
            "standard library package ISQ { attribute def DurationValue; }",
        )
        .expect("write ISQ");
        stdlib_root
    }

    #[test]
    fn validate_paths_suggests_stdlib_install_when_imported_types_are_unresolved() {
        let temp = tempfile::tempdir().expect("temp dir");
        let model_path = write_timer_fixture(&temp);
        let config = Arc::new(crate::default_server_config());

        let report = validate_paths(
            &config,
            ValidationRequest {
                targets: vec![model_path],
                workspace_root: None,
                library_paths: Vec::new(),
                parallel_enabled: false,
            },
        )
        .expect("validation report");

        let unresolved: Vec<&Diagnostic> = report.documents[0]
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.code.as_ref()
                    == Some(&NumberOrString::String(
                        "unresolved_type_reference".to_string(),
                    ))
            })
            .collect();
        assert!(
            unresolved
                .iter()
                .any(|diagnostic| diagnostic.message.contains("Real")),
            "expected unresolved Real diagnostic: {unresolved:#?}"
        );
        assert!(
            unresolved
                .iter()
                .any(|diagnostic| diagnostic.message.contains("DurationValue")),
            "expected unresolved DurationValue diagnostic: {unresolved:#?}"
        );
        assert_eq!(report.advice.len(), 1);
        assert!(report.advice[0].contains("spec42 stdlib install"));
    }

    #[test]
    fn validate_paths_resolves_timer_library_types_when_stdlib_root_is_present() {
        let temp = tempfile::tempdir().expect("temp dir");
        let model_path = write_timer_fixture(&temp);
        let stdlib_root = write_stdlib_fixture(&temp);
        let config = Arc::new(crate::default_server_config());

        let report = validate_paths(
            &config,
            ValidationRequest {
                targets: vec![model_path],
                workspace_root: None,
                library_paths: vec![stdlib_root],
                parallel_enabled: false,
            },
        )
        .expect("validation report");

        let unresolved: Vec<&Diagnostic> = report.documents[0]
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.code.as_ref()
                    == Some(&NumberOrString::String(
                        "unresolved_type_reference".to_string(),
                    ))
            })
            .collect();
        assert!(
            unresolved.is_empty(),
            "expected managed stdlib fixture to clear unresolved type refs: {unresolved:#?}"
        );
        assert!(report.advice.is_empty());
    }
}
