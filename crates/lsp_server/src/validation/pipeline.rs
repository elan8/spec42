use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;

use sysml_model::{
    build_semantic_graph_from_documents, FileSystemDocumentProvider, SemanticGraph,
    SysmlDocument, SysmlDocumentProvider, WorkspaceParsedDocument,
};
use tower_lsp::lsp_types::{Diagnostic, Url};

use crate::analysis::diagnostics_core;
use crate::host::config::Spec42Config;
use crate::workspace::state::{IndexEntry, ParseMetadata, ServerState};
use crate::workspace::indexed_text_or_empty;

use super::discovery::{discover_target_files, path_to_file_url, resolve_workspace_root};
use super::report::{build_advice, summarize};
use super::{SemanticValidationReport, ValidatedDocument, ValidationReport, ValidationRequest};

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

    let provider_target = workspace_root
        .clone()
        .unwrap_or_else(|| target_files[0].clone());
    let provider = FileSystemDocumentProvider::new(
        provider_target,
        workspace_root.clone(),
        request.library_paths.clone(),
    )
    .with_full_library_scan(crate::workspace::library_closure::library_full_scan_enabled());

    // Load documents directly (rather than via `build_semantic_graph_with_provider`) so that
    // files which fail the graph builder's strict parse still get their raw text indexed below —
    // `collect_document_diagnostics` re-parses that text with a tolerant parser to report syntax
    // errors, which only works if the text made it into `state.index` in the first place.
    let all_documents = provider.load_documents()?;
    let (semantic_graph, parsed_documents) =
        build_semantic_graph_from_documents(&all_documents)?;

    let state = server_state_from_documents(
        &all_documents,
        &parsed_documents,
        semantic_graph,
        workspace_root.clone(),
        &request.library_paths,
    )?;

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

fn server_state_from_documents(
    all_documents: &[SysmlDocument],
    parsed_documents: &[WorkspaceParsedDocument],
    semantic_graph: SemanticGraph,
    workspace_root: Option<std::path::PathBuf>,
    library_paths: &[std::path::PathBuf],
) -> Result<ServerState, String> {
    let workspace_root_url = workspace_root
        .as_ref()
        .map(|path| path_to_file_url(path.as_path()))
        .transpose()?;
    let library_root_urls = library_paths
        .iter()
        .map(|path| path_to_file_url(path.as_path()))
        .collect::<Result<Vec<_>, _>>()?;

    let mut index = HashMap::new();
    for document in all_documents {
        index.insert(
            document.uri.clone(),
            IndexEntry {
                content: document.content.clone(),
                parsed: None,
                parse_metadata: ParseMetadata::default(),
                include_in_semantic_graph: true,
            },
        );
    }
    for document in parsed_documents {
        index.insert(
            document.uri.clone(),
            IndexEntry {
                content: document.content.clone(),
                parsed: Some(document.parsed.clone()),
                parse_metadata: ParseMetadata {
                    parse_time_ms: document.parse_time_ms,
                    parse_cached: document.parse_cached,
                },
                include_in_semantic_graph: true,
            },
        );
    }

    let mut session = workspace::WorkspaceSession::new();
    session.begin_startup();
    session.complete_startup();
    Ok(ServerState {
        workspace_roots: workspace_root_url.iter().cloned().collect(),
        library_paths: library_root_urls,
        semantic_graph,
        index,
        session,
        ..ServerState::default()
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the `SPEC42_LIBRARY_FULL_SCAN` env var actually reaches
    /// `FileSystemDocumentProvider::with_full_library_scan` through this module's call site —
    /// `sysml_model`'s own unit tests cover the provider's full-scan behavior directly, but not
    /// this wiring. `#[ignore]`d because it mutates process-global env state, which isn't safe
    /// to run concurrently with the rest of this crate's default parallel test run; run with
    /// `cargo test -p lsp_server --lib -- --ignored validate_paths_with_semantics_full_library_scan_env_var_loads_unreferenced_library_files`.
    #[test]
    #[ignore]
    fn validate_paths_with_semantics_full_library_scan_env_var_loads_unreferenced_library_files() {
        let temp = tempfile::tempdir().expect("tempdir");
        let workspace = temp.path().join("workspace");
        let lib = temp.path().join("lib");
        std::fs::create_dir_all(&workspace).expect("workspace dir");
        std::fs::create_dir_all(&lib).expect("lib dir");
        std::fs::write(workspace.join("App.sysml"), "package App { part appRoot; }")
            .expect("workspace model");
        std::fs::write(
            lib.join("Unused.sysml"),
            "package Unused { part def NeverLoaded; }",
        )
        .expect("unused library");

        let config = std::sync::Arc::new(crate::default_server_config());
        // `lib` is listed as both a library path (drives graph loading, gated by the env var
        // below) and a target (so the projection isn't scoped away from it — `project_semantic_model`
        // only includes nodes whose URI is in `target_files`, independent of library-scan mode).
        let request = ValidationRequest {
            targets: vec![workspace.clone(), lib.clone()],
            workspace_root: Some(workspace.clone()),
            library_paths: vec![lib],
            parallel_enabled: false,
            strict_diagnostics: false,
        };

        std::env::set_var("SPEC42_LIBRARY_FULL_SCAN", "1");
        let report = validate_paths_with_semantics(&config, request);
        std::env::remove_var("SPEC42_LIBRARY_FULL_SCAN");
        let report = report.expect("semantic validation report");

        assert!(
            report
                .semantic_model
                .nodes
                .iter()
                .any(|node| node.name == "NeverLoaded"),
            "SPEC42_LIBRARY_FULL_SCAN=1 should load library files outside the import closure"
        );
    }

    #[test]
    fn validate_paths_with_semantics_validates_kerml_target() {
        let temp = tempfile::tempdir().expect("tempdir");
        let workspace = temp.path().join("workspace");
        std::fs::create_dir_all(&workspace).expect("workspace dir");
        std::fs::write(
            workspace.join("Core.kerml"),
            "package Core { classifier Thing; }",
        )
        .expect("kerml source");

        let config = std::sync::Arc::new(crate::default_server_config());
        let request = ValidationRequest {
            targets: vec![workspace.clone()],
            workspace_root: Some(workspace),
            library_paths: Vec::new(),
            parallel_enabled: false,
            strict_diagnostics: false,
        };

        let report = validate_paths_with_semantics(&config, request).expect("report");
        assert!(
            report
                .semantic_model
                .nodes
                .iter()
                .any(|node| node.qualified_name == "Core::Thing"),
            "Core::Thing from the .kerml target should reach the projection, got {:?}",
            report
                .semantic_model
                .nodes
                .iter()
                .map(|n| &n.qualified_name)
                .collect::<Vec<_>>()
        );
    }
}
