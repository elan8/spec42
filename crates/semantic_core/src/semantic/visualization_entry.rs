use std::collections::HashSet;

use crate::semantic::ibd::{build_ibd_for_uri, merge_ibd_payloads};
use crate::semantic::model_projection::{build_workspace_graph_dto, canonical_general_view_graph};
use crate::semantic::sequence_views::build_workspace_sequence_diagrams;
use crate::{SemanticGraph, SysmlVisualizationResultDto, SysmlVisualizationViewCandidateDto};
use url::Url;

/// Lightweight non-LSP visualization selection entrypoint.
///
/// This graph-first API keeps visualization logic independent from workspace/path scanning.
pub fn build_sysml_visualization_from_graph(
    graph: &SemanticGraph,
    view: &str,
    selected_view: Option<&str>,
) -> Result<SysmlVisualizationResultDto, String> {
    let library_paths = vec![Url::parse("file:///library/").map_err(|err| err.to_string())?];
    let workspace_uris = graph
        .workspace_uris_excluding_libraries(&library_paths)
        .into_iter()
        .filter(|uri| is_workspace_uri(uri))
        .collect::<Vec<_>>();
    let workspace_root_uri = workspace_uris
        .first()
        .map(|uri| uri.to_string())
        .unwrap_or_default();

    let workspace_graph = build_workspace_graph_dto(graph, &library_paths);
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

    let mut seen = HashSet::<String>::new();
    let mut view_candidates = workspace_uris
        .iter()
        .flat_map(|uri| graph.nodes_for_uri(uri))
        .filter(|node| node.element_kind == "view")
        .filter_map(|node| {
            if !seen.insert(node.id.qualified_name.clone()) {
                return None;
            }
            let lowercase_name = node.name.to_ascii_lowercase();
            let (renderer_view, view_type) = if lowercase_name.contains("sequence") {
                ("sequence-view", "sequence")
            } else if lowercase_name.contains("state") {
                ("state-transition-view", "state-transition")
            } else if lowercase_name.contains("interconnection") {
                ("interconnection-view", "interconnection")
            } else if lowercase_name.contains("action") || lowercase_name.contains("activity") {
                ("action-flow-view", "action-flow")
            } else {
                ("general-view", "general")
            };
            Some(SysmlVisualizationViewCandidateDto {
                id: node.id.qualified_name.clone(),
                name: node.name.clone(),
                renderer_view: Some(renderer_view.to_string()),
                supported: true,
                view_type: Some(view_type.to_string()),
                description: Some("model-defined view usage".to_string()),
            })
        })
        .collect::<Vec<_>>();
    view_candidates.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    let selected = selected_view
        .and_then(|value| {
            view_candidates.iter().find(|candidate| {
                candidate.id.eq_ignore_ascii_case(value)
                    || candidate.name.eq_ignore_ascii_case(value)
                    || candidate
                        .id
                        .rsplit("::")
                        .next()
                        .is_some_and(|suffix| suffix.eq_ignore_ascii_case(value))
            })
        })
        .or_else(|| view_candidates.first());
    let selected_view = selected.map(|candidate| candidate.id.clone());
    let selected_view_name = selected.map(|candidate| candidate.name.clone());
    let selected_renderer = selected
        .and_then(|candidate| candidate.renderer_view.clone())
        .unwrap_or_else(|| view.to_string());
    let selected_graph = if selected.is_some() {
        Some(general_graph.clone())
    } else {
        None
    };
    let empty_state_message = if view_candidates.is_empty() {
        Some("No model-defined views were found in this commit".to_string())
    } else {
        None
    };

    Ok(SysmlVisualizationResultDto {
        version: 1,
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

fn is_workspace_uri(uri: &Url) -> bool {
    let path = uri.path().to_ascii_lowercase();
    !path.starts_with("/library/")
}
