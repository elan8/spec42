//! Validation report assembly from a pre-built semantic workspace.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use sysml_model::{SemanticGraph, SysmlDocument, WorkspaceParsedDocument};
use tower_lsp::lsp_types::Url;

use crate::host::config::Spec42Config;
use crate::workspace::state::{IndexEntry, ParseMetadata, ServerState};

use super::discovery::{discover_target_files, path_to_file_url, resolve_workspace_root};
use super::pipeline::collect_target_documents;
use super::report::{build_advice, summarize};
use super::{SemanticValidationReport, ValidationReport, ValidationRequest};

/// Pre-built workspace ingredients for kernel validation without rescanning or rebuilding.
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
