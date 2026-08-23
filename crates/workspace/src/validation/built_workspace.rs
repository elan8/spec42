//! Validation report assembly from a pre-built semantic workspace, plus the engine-driven entry
//! point ([`validate_paths`]) that builds one from a [`ValidationRequest`].
//!
//! Both `crates/server`'s production `spec42 check`/MCP/HTTP-API path and this crate's own test
//! suite build a [`Spec42Engine`] and end up here — there is exactly one implementation of "turn
//! a built graph into a validation report", and exactly one place the reporting policy is
//! applied to it.

use std::path::PathBuf;
use std::sync::Arc;

use sysml_diagnostics::{
    diagnostics_dominated_by_cascades, postprocess_document_diagnostics, PostprocessPolicy,
    ReportingPolicy, SemanticDiagnostic,
};
use sysml_query::resolved_slice::PublishedModel;
use sysml_query::source::Url;

use crate::engine::Spec42Engine;
use crate::provider::HostFilesystemProvider;
use crate::snapshot::discovery;
use crate::snapshot::{
    HostContext, HostValidatedDocument, HostValidationReport, HostWorkspaceSnapshot,
    ValidationTiming, WorkspaceLoadRequest,
};

use super::report::{build_advice, summarize};
use super::{PipelineHook, ValidationRequest};

/// Pre-built workspace ingredients for report assembly without rescanning or rebuilding.
///
/// Everything a report says comes from the publication: the diagnostics for a document that
/// failed the graph builder's strict parse are published as parse errors, so no second index of
/// raw text is needed here.
#[derive(Debug, Clone)]
pub struct BuiltWorkspaceInput {
    /// The publication validation reports from.
    pub published_model: Arc<PublishedModel>,
    pub workspace_root: Option<PathBuf>,
}

/// Converts an already-built [`HostWorkspaceSnapshot`] into the shape
/// [`report_from_built_workspace`] consumes.
pub fn built_workspace_input_from_snapshot(snapshot: &HostWorkspaceSnapshot) -> BuiltWorkspaceInput {
    BuiltWorkspaceInput {
        published_model: snapshot.published_model_arc(),
        workspace_root: Some(snapshot.workspace_root().to_path_buf()),
    }
}

/// Builds a fresh [`HostWorkspaceSnapshot`] via `engine` for `request.targets.first()`, then
/// delegates to [`report_from_built_workspace`]. `request.library_paths` is used only for
/// display/advice below — actual library resolution comes from `engine.package_roots()` (the
/// engine model has no per-request library paths; bake them into `engine` beforehand via
/// `EngineBuilder::library_paths`).
pub(super) fn validate_paths(
    engine: &Spec42Engine,
    hooks: &[PipelineHook],
    request: ValidationRequest,
) -> Result<HostValidationReport, String> {
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
        engine.services().clone(),
    );
    let load_request = WorkspaceLoadRequest::single_target(target)
        .with_workspace_root(workspace_root.clone())
        .with_strict_diagnostics(request.strict_diagnostics)
        .with_validation_timing(ValidationTiming::Deferred);
    let snapshot = engine
        .load_workspace(provider, load_request, HostContext::default())
        .map_err(|error| error.to_string())?;

    let built = built_workspace_input_from_snapshot(&snapshot);
    report_from_built_workspace(hooks, &built, request)
}

pub fn report_from_built_workspace(
    hooks: &[PipelineHook],
    built: &BuiltWorkspaceInput,
    request: ValidationRequest,
) -> Result<HostValidationReport, String> {
    for hook in hooks {
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

    let documents = collect_target_documents(
        &built.published_model,
        &target_files,
        request.strict_diagnostics,
    )?;
    let cascade_dominated = documents
        .iter()
        .any(|document| diagnostics_dominated_by_cascades(&document.diagnostics));
    let summary = summarize(&documents);
    let advice = build_advice(
        &documents,
        cascade_dominated,
        request.library_paths.is_empty(),
    );

    let mut report = HostValidationReport {
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
    for hook in hooks {
        hook.after_validate(&mut report)?;
    }
    Ok(report)
}

fn resolve_workspace_root(request: &ValidationRequest) -> Result<Option<PathBuf>, String> {
    discovery::resolve_workspace_root(&request.targets, request.workspace_root.as_deref())
        .map(Some)
        .map_err(|error| error.to_string())
}

/// The owner's discovery, with the empty case softened: the caller above turns "nothing found"
/// into its own message rather than the owner's.
fn discover_target_files(targets: &[PathBuf]) -> Result<Vec<PathBuf>, String> {
    match discovery::discover_target_files(targets) {
        Ok(files) => Ok(files),
        Err(error) if error.to_string().contains("No .sysml") => Ok(Vec::new()),
        Err(error) => Err(error.to_string()),
    }
}

fn collect_target_documents(
    model: &PublishedModel,
    target_files: &[PathBuf],
    strict_diagnostics: bool,
) -> Result<Vec<HostValidatedDocument>, String> {
    const DIAGNOSTICS_STACK_SIZE: usize = 2 * 1024 * 1024;

    std::thread::scope(|scope| {
        let worker = std::thread::Builder::new()
            .name("spec42-batch-diagnostics".into())
            .stack_size(DIAGNOSTICS_STACK_SIZE)
            .spawn_scoped(scope, || {
                collect_target_documents_inner(model, target_files, strict_diagnostics)
            })
            .map_err(|error| format!("failed to start diagnostics worker: {error}"))?;
        worker
            .join()
            .map_err(|_| "diagnostics worker panicked".to_string())?
    })
}

fn collect_target_documents_inner(
    model: &PublishedModel,
    target_files: &[PathBuf],
    strict_diagnostics: bool,
) -> Result<Vec<HostValidatedDocument>, String> {
    let urls = target_files
        .iter()
        .map(|path| discovery::path_to_file_url(path).map_err(|error| error.to_string()))
        .collect::<Result<std::collections::BTreeSet<Url>, String>>()?;

    Ok(urls
        .into_iter()
        .map(|uri| HostValidatedDocument {
            diagnostics: collect_diagnostics_for_document(model, &uri, strict_diagnostics),
            uri: uri.to_string(),
        })
        .collect())
}

/// The one entry point into the diagnostic core for batch validation: the reporting and
/// post-processing policies the caller asked for, applied to the publication's own answer.
fn collect_diagnostics_for_document(
    model: &PublishedModel,
    uri: &Url,
    strict_diagnostics: bool,
) -> Vec<SemanticDiagnostic> {
    let diagnostics =
        sysml_diagnostics::document_diagnostics(model, uri, ReportingPolicy::strict(strict_diagnostics));
    postprocess_document_diagnostics(
        diagnostics,
        PostprocessPolicy {
            suppress_semantic_after_parse_error: strict_diagnostics,
        },
    )
}
