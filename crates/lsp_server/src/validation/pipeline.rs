use std::collections::{BTreeSet, HashSet};
use std::sync::Arc;

use tower_lsp::lsp_types::{Diagnostic, Url};

use crate::analysis::diagnostics_core;
use crate::host::config::Spec42Config;
use crate::workspace::{
    indexed_text_or_empty, ingest_parsed_scan_entries, parse_scanned_entries,
    rebuild_all_document_links, scan_sysml_files, ServerState,
};

use super::discovery::{discover_target_files, path_to_file_url, resolve_workspace_root};
use super::report::{build_advice, summarize};
use super::{
    SemanticValidationReport, ValidatedDocument, ValidationReport, ValidationRequest,
};

pub(super) fn validate_paths(
    config: &Arc<Spec42Config>,
    request: ValidationRequest,
) -> Result<ValidationReport, String> {
    Ok(validate_paths_with_semantics(config, request)?.validation)
}

pub(super) fn validate_paths_with_semantics(
    config: &Arc<Spec42Config>,
    request: ValidationRequest,
) -> Result<SemanticValidationReport, String> {
    for hook in &config.pipeline_hooks {
        hook.before_validate(&request)?;
    }
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

    let mut state = initialize_state(workspace_root_url.clone(), library_root_urls.clone());
    let entries = collect_entries(
        workspace_root_url,
        &library_root_urls,
        &target_files,
        request.parallel_enabled,
    )?;
    ingest_parsed_scan_entries(
        &mut state,
        parse_scanned_entries(entries, request.parallel_enabled, None),
    );
    rebuild_all_document_links(&mut state);

    let documents =
        collect_target_documents(&state, config, &target_files, request.strict_diagnostics)?;
    let summary = summarize(&documents);
    let advice = build_advice(&documents, request.library_paths.is_empty());
    let semantic_model = workspace::project_semantic_model(&state.semantic_graph, &target_files)
        .map_err(|err| err.to_string())?;

    let mut report = ValidationReport {
        workspace_root: workspace_root.map(|path| path.display().to_string()),
        resolved_library_paths: request
            .library_paths
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        documents,
        summary,
        advice,
    };
    for hook in &config.pipeline_hooks {
        hook.after_validate(&mut report)?;
    }
    Ok(SemanticValidationReport {
        validation: report,
        semantic_model,
    })
}

fn initialize_state(workspace_root_url: Option<Url>, library_root_urls: Vec<Url>) -> ServerState {
    ServerState {
        workspace_roots: workspace_root_url.iter().cloned().collect(),
        library_paths: library_root_urls,
        ..ServerState::default()
    }
}

fn collect_entries(
    workspace_root_url: Option<Url>,
    library_root_urls: &[Url],
    target_files: &[std::path::PathBuf],
    parallel_enabled: bool,
) -> Result<Vec<(Url, String)>, String> {
    // Parallelism is currently applied in `parse_scanned_entries`, not during filesystem scan.
    let _ = parallel_enabled;
    let scan_roots: Vec<Url> = if crate::workspace::library_closure::library_full_scan_enabled() {
        workspace_root_url
            .iter()
            .cloned()
            .chain(library_root_urls.iter().cloned())
            .collect()
    } else {
        workspace_root_url.iter().cloned().collect()
    };
    let mut entries = Vec::new();
    if !scan_roots.is_empty() {
        let (scanned_entries, _) = scan_sysml_files(scan_roots);
        entries.extend(scanned_entries);
    }

    if !crate::workspace::library_closure::library_full_scan_enabled()
        && !library_root_urls.is_empty()
    {
        let workspace_sources: Vec<sysml_model::WorkspaceSource<'_>> = entries
            .iter()
            .map(|(uri, content)| sysml_model::WorkspaceSource {
                path: uri.as_str(),
                content: content.as_str(),
            })
            .collect();
        let library_entries = crate::workspace::library_closure::load_library_closure_scan_entries(
            &workspace_sources,
            library_root_urls,
        )?;
        entries.extend(library_entries);
    }

    let mut seen = HashSet::new();
    for (uri, _) in &entries {
        seen.insert(uri.as_str().to_string());
    }
    for path in target_files {
        let uri = path_to_file_url(path)?;
        if seen.insert(uri.as_str().to_string()) {
            let content = std::fs::read_to_string(path)
                .map_err(|err| format!("Failed to read {}: {err}", path.display()))?;
            entries.push((uri, content));
        }
    }
    Ok(entries)
}

pub(super) fn collect_target_documents(
    state: &ServerState,
    config: &Arc<Spec42Config>,
    target_files: &[std::path::PathBuf],
    strict_diagnostics: bool,
) -> Result<Vec<ValidatedDocument>, String> {
    let target_urls = target_file_urls(target_files)?;

    Ok(target_urls
        .into_iter()
        .map(|uri| {
            let text = indexed_text_or_empty(state, &uri);
            let diagnostics =
                collect_diagnostics_for_document(state, config, &uri, &text, strict_diagnostics);
            ValidatedDocument {
                uri: uri.to_string(),
                diagnostics,
            }
        })
        .collect::<Vec<_>>())
}

fn target_file_urls(target_files: &[std::path::PathBuf]) -> Result<BTreeSet<Url>, String> {
    target_files
        .iter()
        .map(|path| path_to_file_url(path.as_path()))
        .collect::<Result<BTreeSet<_>, _>>()
}

fn collect_diagnostics_for_document(
    state: &ServerState,
    config: &Arc<Spec42Config>,
    uri: &Url,
    text: &str,
    strict_diagnostics: bool,
) -> Vec<Diagnostic> {
    diagnostics_core::collect_document_diagnostics(
        &state.semantic_graph,
        &state.library_paths,
        &config.check_providers,
        uri,
        text,
        false,
        diagnostics_core::validation_postprocess_options(strict_diagnostics),
    )
}
