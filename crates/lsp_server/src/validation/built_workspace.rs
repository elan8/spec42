//! Validation report assembly from a pre-built semantic workspace, plus the engine-driven
//! entry points (`validate_paths`/`validate_paths_with_semantics`) that build one from a
//! `ValidationRequest`. Both `crates/server/src/host_snapshot.rs` (the production `spec42
//! check`/MCP/HTTP-API path) and this crate's own test suite build a `workspace::Spec42Engine`
//! and end up here — there is exactly one implementation of "turn a built graph into a
//! validation report" (see `docs/architecture-audit.md` P1-2).

use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;
use std::sync::Arc;

use sysml_model::{SemanticGraph, SysmlDocument, WorkspaceParsedDocument};
use tower_lsp::lsp_types::{Diagnostic, Url};
use workspace::{
    HostContext, HostFilesystemProvider, HostWorkspaceSnapshot, Spec42Engine, WorkspaceLoadRequest,
};

use crate::analysis::diagnostics_core;
use crate::host::config::Spec42Config;
use crate::workspace::indexed_text_or_empty;
use crate::workspace::state::{IndexEntry, ParseMetadata, ServerState};

use super::discovery::{discover_target_files, path_to_file_url, resolve_workspace_root};
use super::report::{build_advice, summarize};
use super::{SemanticValidationReport, ValidatedDocument, ValidationReport, ValidationRequest};

/// Pre-built workspace ingredients for report assembly without rescanning or rebuilding.
#[derive(Debug, Clone)]
pub struct BuiltWorkspaceInput {
    pub semantic_graph: SemanticGraph,
    /// Every document the provider loaded, including ones that failed the graph builder's
    /// strict parse. Indexed for raw text below so `collect_diagnostics_for_document` can
    /// re-parse them with a tolerant parser and still report syntax errors; without this,
    /// documents dropped from `parsed_documents` silently vanish from the index and produce
    /// zero diagnostics instead of a parse error.
    pub all_documents: Vec<SysmlDocument>,
    pub parsed_documents: Vec<WorkspaceParsedDocument>,
    pub library_urls: Vec<Url>,
    pub workspace_root: Option<PathBuf>,
}

/// Converts an already-built `workspace::HostWorkspaceSnapshot` into the shape
/// [`semantic_report_from_built_workspace`] consumes.
pub fn built_workspace_input_from_snapshot(snapshot: &HostWorkspaceSnapshot) -> BuiltWorkspaceInput {
    BuiltWorkspaceInput {
        semantic_graph: snapshot.semantic_graph().clone(),
        all_documents: snapshot.documents().to_vec(),
        parsed_documents: snapshot.parsed_documents().to_vec(),
        library_urls: snapshot.library_urls().to_vec(),
        workspace_root: Some(snapshot.workspace_root().to_path_buf()),
    }
}

pub(super) fn validate_paths(
    engine: &Spec42Engine,
    config: &Arc<Spec42Config>,
    request: ValidationRequest,
) -> Result<ValidationReport, String> {
    Ok(validate_paths_with_semantics(engine, config, request)?.validation)
}

/// Builds a fresh `workspace::HostWorkspaceSnapshot` via `engine` for `request.targets.first()`,
/// then delegates to [`semantic_report_from_built_workspace`]. `request.library_paths` is used
/// only for display/advice below — actual library resolution comes from `engine.package_roots()`
/// (the engine model has no per-request library paths; bake them into `engine` beforehand via
/// `EngineBuilder::library_paths`).
pub(super) fn validate_paths_with_semantics(
    engine: &Spec42Engine,
    config: &Arc<Spec42Config>,
    request: ValidationRequest,
) -> Result<SemanticValidationReport, String> {
    let workspace_root = resolve_workspace_root(&request)?;
    let target = request
        .targets
        .first()
        .cloned()
        .ok_or_else(|| "No target path was provided.".to_string())?;

    let provider = HostFilesystemProvider::from_paths(
        &target,
        workspace_root.as_deref(),
        engine.package_roots(),
    );
    let load_request = WorkspaceLoadRequest::single_target(target)
        .with_workspace_root(workspace_root.clone())
        .with_strict_diagnostics(request.strict_diagnostics);
    let snapshot = engine
        .load_workspace(provider, load_request, HostContext::default())
        .map_err(|error| error.to_string())?;

    let built = built_workspace_input_from_snapshot(&snapshot);
    semantic_report_from_built_workspace(config, &built, request)
}

pub fn semantic_report_from_built_workspace(
    config: &Arc<Spec42Config>,
    built: &BuiltWorkspaceInput,
    request: ValidationRequest,
) -> Result<SemanticValidationReport, String> {
    for hook in &config.pipeline_hooks {
        hook.before_validate(&request)?;
    }

    let workspace_root = built
        .workspace_root
        .clone()
        .or(resolve_workspace_root(&request)?);
    let target_files = discover_target_files(&request.targets)?;
    if target_files.is_empty() {
        return Err("No .sysml or .kerml files were found under the requested path.".to_string());
    }

    let workspace_root_url = workspace_root
        .as_ref()
        .map(|path| path_to_file_url(path.as_path()))
        .transpose()?;

    let state = server_state_from_built(built, workspace_root_url.clone());

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

fn server_state_from_built(
    built: &BuiltWorkspaceInput,
    workspace_root_url: Option<Url>,
) -> ServerState {
    let mut index = HashMap::new();
    for document in &built.all_documents {
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
    for document in &built.parsed_documents {
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
    ServerState {
        workspace_roots: workspace_root_url.iter().cloned().collect(),
        library_paths: built.library_urls.clone(),
        semantic_graph: built.semantic_graph.clone(),
        index,
        session,
        ..ServerState::default()
    }
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

    fn test_engine(cache: &tempfile::TempDir, library_paths: Vec<PathBuf>) -> Spec42Engine {
        workspace::EngineBuilder::default()
            .cache_dir(cache.path().to_path_buf())
            .no_stdlib(true)
            .library_paths(library_paths)
            .build()
            .expect("engine")
    }

    #[test]
    fn validate_paths_with_semantics_validates_kerml_target() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cache = tempfile::tempdir().expect("cache dir");
        let workspace_dir = temp.path().join("workspace");
        std::fs::create_dir_all(&workspace_dir).expect("workspace dir");
        std::fs::write(
            workspace_dir.join("Core.kerml"),
            "package Core { classifier Thing; }",
        )
        .expect("kerml source");

        let engine = test_engine(&cache, Vec::new());
        let config = Arc::new(crate::default_server_config());
        let request = ValidationRequest {
            targets: vec![workspace_dir.clone()],
            workspace_root: Some(workspace_dir),
            library_paths: Vec::new(),
            parallel_enabled: false,
            strict_diagnostics: false,
        };

        let report = validate_paths_with_semantics(&engine, &config, request).expect("report");
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
