use std::collections::{HashMap, HashSet};

use crate::semantic::dto::{GraphEdgeDto, GraphNodeDto, SysmlGraphDto};
use crate::semantic::explicit_views;
use crate::semantic::extracted_model::ActivityDiagramDto;
use crate::semantic::ibd::{build_ibd_for_uri, merge_ibd_payloads};
use crate::semantic::model_projection::{
    build_workspace_graph_dto, canonical_general_view_graph, strip_synthetic_nodes,
};
use crate::semantic::sequence_views::build_workspace_sequence_diagrams;
use crate::semantic::workspace_graph::WorkspaceParsedDocument;
use crate::{SemanticGraph, SysmlVisualizationResultDto};
use url::Url;

/// Graph-first visualization entrypoint aligned with the LSP kernel’s view catalog and selection.
///
/// Pass the same [`WorkspaceParsedDocument`] slice returned from
/// [`crate::semantic::workspace_graph::build_semantic_graph_with_provider`] (or
/// [`crate::semantic::workspace_graph::build_semantic_graph_from_documents`]) so view definitions
/// and usages are discovered from the AST, not from name heuristics.
pub fn build_sysml_visualization_from_graph(
    graph: &SemanticGraph,
    documents: &[WorkspaceParsedDocument],
    view: &str,
    selected_view: Option<&str>,
) -> Result<SysmlVisualizationResultDto, String> {
    let library_paths = vec![Url::parse("file:///library/").map_err(|err| err.to_string())?];
    let mut workspace_uris: Vec<Url> = documents.iter().map(|doc| doc.uri.clone()).collect();
    workspace_uris.sort_by(|left, right| left.as_str().cmp(right.as_str()));
    workspace_uris.retain(|uri| is_workspace_uri(uri));

    let workspace_root_uri = workspace_uris
        .first()
        .map(|uri| uri.to_string())
        .unwrap_or_default();

    let workspace_graph = strip_synthetic_nodes(&build_workspace_graph_dto(graph, &library_paths));
    let general_graph = canonical_general_view_graph(&workspace_graph, false);
    let sequence_diagrams = build_workspace_sequence_diagrams(graph, &workspace_uris);
    let ibd_payloads = workspace_uris
        .iter()
        .map(|uri| build_ibd_for_uri(graph, uri))
        .collect::<Vec<_>>();
    let merged_ibd = if ibd_payloads.is_empty() {
        None
    } else {
        Some(merge_ibd_payloads(ibd_payloads))
    };

    let catalog = explicit_views::build_view_catalog(&workspace_uris, documents);
    if catalog.usages.is_empty() {
        return Ok(SysmlVisualizationResultDto {
            version: 0,
            view: view.to_string(),
            workspace_root_uri,
            view_candidates: Vec::new(),
            selected_view: None,
            selected_view_name: None,
            empty_state_message: Some(
                "No model-defined views were found in this workspace.".to_string(),
            ),
            package_groups: None,
            graph: Some(SysmlGraphDto {
                nodes: Vec::new(),
                edges: Vec::new(),
            }),
            general_view_graph: Some(SysmlGraphDto {
                nodes: Vec::new(),
                edges: Vec::new(),
            }),
            workspace_model: None,
            activity_diagrams: None,
            sequence_diagrams: Some(Vec::new()),
            ibd: merged_ibd,
            stats: None,
        });
    }

    let evaluated_views = explicit_views::evaluate_views(&catalog, &workspace_graph);
    let mut projected_graphs: HashMap<&str, SysmlGraphDto> = HashMap::new();
    let empty_activity: HashMap<&str, Vec<ActivityDiagramDto>> = HashMap::new();
    for evaluated in &evaluated_views {
        let projected_ids = explicit_views::renderer_view_for_view_type(
            evaluated.effective_view_type.as_deref(),
        )
        .map(|renderer_view| {
            explicit_views::project_ids_for_renderer(evaluated, &workspace_graph, renderer_view)
        })
        .unwrap_or_default();
        let projected_graph = project_graph_by_ids(&workspace_graph, &projected_ids);
        projected_graphs.insert(evaluated.id.as_str(), projected_graph);
    }

    let view_candidates = explicit_views::build_view_candidates(
        &evaluated_views,
        &empty_activity,
        &projected_graphs,
    );

    let selected = selected_view
        .and_then(|selected| {
            view_candidates.iter().find(|candidate| {
                candidate.id == selected
                    || candidate.name == selected
                    || candidate.id.rsplit("::").next() == Some(selected)
            })
        })
        .or_else(|| {
            view_candidates.iter().find(|candidate| {
                candidate.supported && candidate.renderer_view.as_deref() == Some(view)
            })
        })
        .or_else(|| view_candidates.iter().find(|candidate| candidate.supported))
        .or_else(|| view_candidates.first());

    let selected_view = selected.map(|candidate| candidate.id.clone());
    let selected_view_name = selected.map(|candidate| candidate.name.clone());
    let selected_renderer = selected
        .and_then(|candidate| candidate.renderer_view.clone())
        .unwrap_or_else(|| view.to_string());

    let selected_graph = selected
        .and_then(|candidate| projected_graphs.get(candidate.id.as_str()).cloned())
        .or_else(|| Some(general_graph.clone()));

    let empty_state_message = if view_candidates.is_empty() {
        Some("No model-defined views were found in this commit".to_string())
    } else {
        None
    };

    Ok(SysmlVisualizationResultDto {
        version: 0,
        view: selected_renderer,
        workspace_root_uri,
        view_candidates,
        selected_view,
        selected_view_name,
        empty_state_message,
        package_groups: None,
        graph: selected_graph,
        general_view_graph: Some(general_graph),
        workspace_model: None,
        activity_diagrams: None,
        sequence_diagrams: Some(sequence_diagrams),
        ibd: merged_ibd,
        stats: None,
    })
}

fn project_graph_by_ids(graph: &SysmlGraphDto, ids: &HashSet<String>) -> SysmlGraphDto {
    let id_set: HashSet<&str> = ids.iter().map(String::as_str).collect();
    let nodes: Vec<GraphNodeDto> = graph
        .nodes
        .iter()
        .filter(|node| id_set.contains(node.id.as_str()))
        .cloned()
        .collect();
    let node_ids: HashSet<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let edges: Vec<GraphEdgeDto> = graph
        .edges
        .iter()
        .filter(|edge| node_ids.contains(&edge.source) && node_ids.contains(&edge.target))
        .cloned()
        .collect();
    SysmlGraphDto { nodes, edges }
}

fn is_workspace_uri(uri: &Url) -> bool {
    let path = uri.path().to_ascii_lowercase();
    !path.starts_with("/library/")
}
