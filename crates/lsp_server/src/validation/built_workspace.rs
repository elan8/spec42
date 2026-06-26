//! Validation report assembly from a pre-built semantic workspace.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use sysml_model::{SemanticGraph, WorkspaceParsedDocument};
use tower_lsp::lsp_types::Url;

use crate::host::config::Spec42Config;
use crate::workspace::state::{IndexEntry, ParseMetadata, SemanticLifecycle, ServerState};

use super::discovery::{discover_target_files, path_to_file_url, resolve_workspace_root};
use super::pipeline::{collect_target_documents, project_semantic_model};
use super::report::{build_advice, summarize};
use super::{SemanticValidationReport, ValidationReport, ValidationRequest};

/// Pre-built workspace ingredients for kernel validation without rescanning or rebuilding.
#[derive(Debug, Clone)]
pub struct BuiltWorkspaceInput {
    pub semantic_graph: SemanticGraph,
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
    let target_urls = target_files
        .iter()
        .map(|path| path_to_file_url(path.as_path()))
        .collect::<Result<std::collections::BTreeSet<_>, _>>()?;
    let semantic_model = project_semantic_model(&state, &target_urls);

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

    ServerState {
        workspace_roots: workspace_root_url.iter().cloned().collect(),
        library_paths: built.library_urls.clone(),
        semantic_graph: built.semantic_graph.clone(),
        index,
        semantic_lifecycle: SemanticLifecycle::Ready,
        semantic_state_version: 1,
        ..ServerState::default()
    }
}
