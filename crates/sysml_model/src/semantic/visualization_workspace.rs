//! Full workspace visualization pipeline (parity with the Spec42 LSP kernel).
//!
//! This module is URL-agnostic beyond `url::Url` string prefix checks so callers using
//! `memory://` schemes behave consistently when roots are chosen appropriately.

use std::time::Instant;

use url::Url;

use crate::semantic::dto::SysmlVisualizationResultDto;
use crate::semantic::workspace_graph::WorkspaceParsedDocument;
use crate::SemanticGraph;

pub use crate::semantic::visualization::ibd_scope::{
    filter_ibd_by_visible_ids, select_interconnection_ibd_scope,
    select_interconnection_ibd_scope_with_trace, IbdScopeTrace,
};
pub use crate::semantic::visualization::projection::{
    attach_ibd_package_container_groups, build_ibd_package_container_groups,
    build_package_groups_from_graph, build_workspace_activity_diagrams,
    build_workspace_graph_dto_for_uris, filter_activity_diagrams_by_graph,
    top_level_package_for_node_id,
};
pub use crate::semantic::visualization::response::{
    build_merged_workspace_ibd, build_sysml_visualization_from_artifacts,
    build_sysml_visualization_workspace, build_sysml_visualization_workspace_with_meta,
    build_workspace_visualization_artifacts, empty_merged_ibd, interconnection_build_options,
    visualization_build_options, VisualizationBuildMeta, VisualizationBuildOptions, WorkspaceVisualizationArtifacts,
    WorkspaceVisualizationRequest,
};
pub use crate::semantic::visualization::scope::{uri_under_root, workspace_uris_for_root};

fn infer_workspace_root_uri(documents: &[WorkspaceParsedDocument]) -> Result<Url, String> {
    crate::semantic::visualization::response::infer_workspace_root_uri(documents)
}

/// Graph-first visualization when callers do not have an explicit workspace root URI.
pub fn build_sysml_visualization_from_graph_and_documents(
    semantic_graph: &SemanticGraph,
    documents: &[WorkspaceParsedDocument],
    view: &str,
    selected_view: Option<&str>,
    build_start: Instant,
) -> Result<SysmlVisualizationResultDto, String> {
    let library_paths = vec![Url::parse("file:///library/").map_err(|e| e.to_string())?];
    let workspace_root_uri = infer_workspace_root_uri(documents)?;
    build_sysml_visualization_workspace(
        semantic_graph,
        documents,
        &library_paths,
        &workspace_root_uri,
        view,
        selected_view,
        build_start,
    )
}
