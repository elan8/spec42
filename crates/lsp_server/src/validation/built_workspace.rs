//! Validation report assembly from a pre-built semantic workspace, plus the engine-driven
//! entry points (`validate_paths`/`validate_paths_with_semantics`) that build one from a
//! `ValidationRequest`. Both `crates/server/src/host_snapshot.rs` (the production `spec42
//! check`/MCP/HTTP-API path) and this crate's own test suite build a `workspace::Spec42Engine`
//! and end up here — there is exactly one implementation of "turn a built graph into a
//! validation report".

use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;
use std::sync::Arc;

use sysml_query::resolved_slice::PublishedModel;
use sysml_query::source::SourceDocument;
use tower_lsp::lsp_types::{Diagnostic, Url};
use workspace::{
    HostContext, HostFilesystemProvider, HostWorkspaceSnapshot, Spec42Engine, ValidationTiming,
    WorkspaceLoadRequest,
};

use crate::analysis::diagnostics_core;
use crate::host::config::Spec42Config;
use crate::workspace::state::{IndexEntry, ServerState};

use super::discovery::{discover_target_files, path_to_file_url, resolve_workspace_root};
use super::report::{build_advice, summarize};
use super::{SemanticValidationReport, ValidatedDocument, ValidationReport, ValidationRequest};

/// Pre-built workspace ingredients for report assembly without rescanning or rebuilding.
#[derive(Debug, Clone)]
pub struct BuiltWorkspaceInput {
    /// The publication validation reports from.
    pub published_model: Arc<PublishedModel>,
    /// Every document the provider loaded, including ones that failed the graph builder's
    /// strict parse. Indexed for raw text below so `collect_diagnostics_for_document` can
    /// re-parse them with a tolerant parser and still report syntax errors; without this,
    /// documents dropped from `parsed_documents` silently vanish from the index and produce
    /// zero diagnostics instead of a parse error.
    pub all_documents: Vec<SourceDocument>,
    pub library_urls: Vec<Url>,
    pub workspace_root: Option<PathBuf>,
}

/// Converts an already-built `workspace::HostWorkspaceSnapshot` into the shape
/// [`semantic_report_from_built_workspace`] consumes.
pub fn built_workspace_input_from_snapshot(
    snapshot: &HostWorkspaceSnapshot,
) -> BuiltWorkspaceInput {
    BuiltWorkspaceInput {
        published_model: snapshot.published_model_arc(),
        all_documents: snapshot.documents().to_vec(),
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

    let provider = HostFilesystemProvider::from_paths_with_standard_library(
        &target,
        workspace_root.as_deref(),
        engine.package_roots(),
        &engine.library_catalog().stdlib.roots,
    );
    let load_request = WorkspaceLoadRequest::single_target(target)
        .with_workspace_root(workspace_root.clone())
        .with_strict_diagnostics(request.strict_diagnostics)
        .with_validation_timing(ValidationTiming::Deferred);
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

    let state = server_state_from_built(built, workspace_root_url.clone(), &config.services);

    let documents = collect_target_documents(&state, &target_files, request.strict_diagnostics)?;
    let summary = summarize(&documents);
    let advice = build_advice(&documents, request.library_paths.is_empty());

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
    Ok(SemanticValidationReport { validation: report })
}

/// A ready server state over an already-built publication, sharing the host's services so the
/// documents the engine parsed are memo hits here rather than a second parse.
fn server_state_from_built(
    built: &BuiltWorkspaceInput,
    workspace_root_url: Option<Url>,
    services: &sysml_query::Services,
) -> ServerState {
    let mut index = HashMap::new();
    for document in &built.all_documents {
        index.insert(
            document.uri().clone(),
            IndexEntry {
                document: document.clone(),
                parsed: services.syntax.parse(document),
                admitted_to_publication: true,
            },
        );
    }
    let mut state =
        ServerState::with_initial_publication(services.clone(), built.published_model.clone());
    state.session.begin_startup();
    state.session.complete_startup();
    state.workspace_roots = workspace_root_url.iter().cloned().collect();
    state.library_paths = built.library_urls.clone();
    state.index = index;
    state
}

pub(super) fn collect_target_documents(
    state: &ServerState,
    target_files: &[std::path::PathBuf],
    strict_diagnostics: bool,
) -> Result<Vec<ValidatedDocument>, String> {
    const DIAGNOSTICS_STACK_SIZE: usize = 2 * 1024 * 1024;

    std::thread::scope(|scope| {
        let worker = std::thread::Builder::new()
            .name("spec42-batch-diagnostics".into())
            .stack_size(DIAGNOSTICS_STACK_SIZE)
            .spawn_scoped(scope, || {
                collect_target_documents_inner(state, target_files, strict_diagnostics)
            })
            .map_err(|error| format!("failed to start diagnostics worker: {error}"))?;
        worker
            .join()
            .map_err(|_| "diagnostics worker panicked".to_string())?
    })
}

fn collect_target_documents_inner(
    state: &ServerState,
    target_files: &[std::path::PathBuf],
    strict_diagnostics: bool,
) -> Result<Vec<ValidatedDocument>, String> {
    let target_urls = target_file_urls(target_files)?;

    Ok(target_urls
        .into_iter()
        .map(|uri| {
            let diagnostics = collect_diagnostics_for_document(state, &uri, strict_diagnostics);
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
    uri: &Url,
    strict_diagnostics: bool,
) -> Vec<Diagnostic> {
    diagnostics_core::collect_document_diagnostics(
        Some(state.published_model()),
        uri,
        diagnostics_core::validation_reporting(strict_diagnostics),
        diagnostics_core::validation_postprocess_options(strict_diagnostics),
    )
}
