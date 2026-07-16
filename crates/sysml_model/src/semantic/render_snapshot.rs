//! Workspace render snapshot: eager view index and lazy bundle materialization hooks.

use std::time::Instant;

use url::Url;

use crate::semantic::dto::{
    SysmlGraphDto, SysmlModelStatsDto, SysmlVisualizationResultDto,
    SysmlVisualizationViewCandidateDto, WorkspaceModelDto,
};
use crate::semantic::explicit_views::EvaluatedView;
use crate::semantic::ibd::IbdDataDto;
use crate::semantic::model_projection::canonical_general_view_graph;
use crate::semantic::visualization::projection::build_workspace_model_dto_from_graph;
use crate::semantic::visualization::response::{
    build_merged_workspace_ibd, build_sysml_visualization_from_artifacts,
    build_workspace_visualization_artifacts, VisualizationBuildMeta, VisualizationBuildOptions,
};
use crate::semantic::visualization::scope::IbdArtifactMode;
use crate::semantic::workspace_graph::WorkspaceParsedDocument;
use crate::SemanticGraph;

/// Cheap catalog + evaluated metadata shared by diagram views and Model Explorer.
#[derive(Debug, Clone)]
pub struct ViewIndex {
    pub version: u64,
    pub workspace_root_uri: String,
    pub workspace_uris: Vec<Url>,
    pub graph: SysmlGraphDto,
    pub evaluated_views: Vec<EvaluatedView>,
    pub view_candidates: Vec<SysmlVisualizationViewCandidateDto>,
}

/// Heavy workspace payload for Model Explorer (`sysml/model` workspaceVisualization scope).
#[derive(Debug, Clone)]
pub struct ModelExplorerBundle {
    pub workspace_graph: SysmlGraphDto,
    pub general_view_graph: SysmlGraphDto,
    pub workspace_model: WorkspaceModelDto,
    pub full_ibd: IbdDataDto,
    pub stats: SysmlModelStatsDto,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ViewBundleKey {
    pub view: String,
    pub selected_view: Option<String>,
}

/// Snapshot entry keyed by `(semantic_state_version, workspace_root_uri)`.
#[derive(Debug, Clone)]
pub struct WorkspaceRenderSnapshot {
    pub version: u64,
    pub workspace_root_uri: String,
    pub workspace_uris: Vec<Url>,
    pub view_index: ViewIndex,
}

pub fn build_view_index(
    semantic_graph: &SemanticGraph,
    documents: &[WorkspaceParsedDocument],
    library_paths: &[Url],
    workspace_root_uri: &Url,
    semantic_state_version: u64,
) -> Result<ViewIndex, String> {
    let artifacts = build_workspace_visualization_artifacts(
        semantic_graph,
        documents,
        library_paths,
        workspace_root_uri,
        IbdArtifactMode::Deferred,
    )?;
    Ok(ViewIndex {
        version: semantic_state_version,
        workspace_root_uri: artifacts.workspace_root_uri,
        workspace_uris: artifacts.workspace_uris,
        graph: artifacts.graph,
        evaluated_views: artifacts.evaluated_views,
        view_candidates: artifacts.view_candidates,
    })
}

pub fn build_render_snapshot(
    semantic_graph: &SemanticGraph,
    documents: &[WorkspaceParsedDocument],
    library_paths: &[Url],
    workspace_root_uri: &Url,
    semantic_state_version: u64,
) -> Result<WorkspaceRenderSnapshot, String> {
    let view_index = build_view_index(
        semantic_graph,
        documents,
        library_paths,
        workspace_root_uri,
        semantic_state_version,
    )?;
    Ok(WorkspaceRenderSnapshot {
        version: semantic_state_version,
        workspace_root_uri: view_index.workspace_root_uri.clone(),
        workspace_uris: view_index.workspace_uris.clone(),
        view_index,
    })
}

pub fn materialize_model_explorer_bundle(
    semantic_graph: &SemanticGraph,
    snapshot: &WorkspaceRenderSnapshot,
) -> ModelExplorerBundle {
    let full_ibd = build_merged_workspace_ibd(semantic_graph, &snapshot.workspace_uris);
    let workspace_graph = snapshot.view_index.graph.clone();
    let general_view_graph = canonical_general_view_graph(&workspace_graph, true);
    let workspace_model =
        build_workspace_model_dto_from_graph(&workspace_graph, &snapshot.workspace_uris);
    let stats = SysmlModelStatsDto {
        total_elements: workspace_graph.nodes.len() as u32,
        resolved_elements: 0,
        unresolved_elements: 0,
        parse_time_ms: 0,
        model_build_time_ms: 0,
        parse_cached: false,
    };
    ModelExplorerBundle {
        workspace_graph,
        general_view_graph,
        workspace_model,
        full_ibd,
        stats,
    }
}

pub fn view_index_to_artifacts(
    index: &ViewIndex,
    full_ibd: IbdDataDto,
) -> crate::semantic::visualization::response::WorkspaceVisualizationArtifacts {
    crate::semantic::visualization::response::WorkspaceVisualizationArtifacts {
        workspace_root_uri: index.workspace_root_uri.clone(),
        workspace_uris: index.workspace_uris.clone(),
        graph: index.graph.clone(),
        full_ibd,
        evaluated_views: index.evaluated_views.clone(),
        view_candidates: index.view_candidates.clone(),
    }
}

/// Resolve merged IBD for a render snapshot, optionally reusing a cached value.
pub fn full_ibd_for_render_snapshot(
    semantic_graph: &SemanticGraph,
    snapshot: &WorkspaceRenderSnapshot,
    cached_full_ibd: Option<&IbdDataDto>,
) -> IbdDataDto {
    if let Some(ibd) = cached_full_ibd {
        return ibd.clone();
    }
    build_merged_workspace_ibd(semantic_graph, &snapshot.workspace_uris)
}

/// Build a single-view visualization response from a precomputed render snapshot.
#[allow(clippy::too_many_arguments)]
pub fn build_sysml_visualization_from_render_snapshot(
    semantic_graph: &SemanticGraph,
    documents: &[WorkspaceParsedDocument],
    snapshot: &WorkspaceRenderSnapshot,
    view: &str,
    selected_view: Option<&str>,
    build_start: Instant,
    full_ibd: IbdDataDto,
    options: VisualizationBuildOptions,
) -> Result<SysmlVisualizationResultDto, String> {
    let (response, _) = build_sysml_visualization_from_render_snapshot_with_meta(
        semantic_graph,
        documents,
        snapshot,
        view,
        selected_view,
        build_start,
        full_ibd,
        options,
    )?;
    Ok(response)
}

/// Build a single-view visualization response with perf metadata.
#[allow(clippy::too_many_arguments)]
pub fn build_sysml_visualization_from_render_snapshot_with_meta(
    semantic_graph: &SemanticGraph,
    documents: &[WorkspaceParsedDocument],
    snapshot: &WorkspaceRenderSnapshot,
    view: &str,
    selected_view: Option<&str>,
    build_start: Instant,
    full_ibd: IbdDataDto,
    options: VisualizationBuildOptions,
) -> Result<(SysmlVisualizationResultDto, VisualizationBuildMeta), String> {
    let artifacts = view_index_to_artifacts(&snapshot.view_index, full_ibd);
    build_sysml_visualization_from_artifacts(
        semantic_graph,
        documents,
        &artifacts,
        view,
        selected_view,
        build_start,
        options,
    )
}
