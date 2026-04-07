//! sysml/model request parsing and response building.

#[path = "model_params.rs"]
mod model_params;
#[path = "model_projection.rs"]
mod model_projection;

use std::time::Instant;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{MessageType, Url};
use tower_lsp::Client;

use sysml_parser::RootNamespace;

use crate::common::util;
use crate::semantic_model;
use crate::views::dto::{
    range_to_dto, GraphEdgeDto, GraphNodeDto, SysmlGraphDto, SysmlModelResultDto,
    SysmlModelStatsDto,
};
use crate::views::extracted_model as model;
use crate::views::ibd;

pub fn parse_sysml_model_params(v: &serde_json::Value) -> Result<(Url, Vec<String>)> {
    model_params::parse_sysml_model_params(v)
}

pub fn empty_model_response(build_start: Instant) -> SysmlModelResultDto {
    model_params::empty_model_response(build_start)
}

fn canonical_general_view_graph(graph: &SysmlGraphDto, include_all_roots: bool) -> SysmlGraphDto {
    model_projection::canonical_general_view_graph(graph, include_all_roots)
}

fn build_workspace_graph_dto(
    semantic_graph: &semantic_model::SemanticGraph,
    library_paths: &[Url],
) -> SysmlGraphDto {
    model_projection::build_workspace_graph_dto(semantic_graph, library_paths)
}

fn strip_synthetic_nodes(graph: &SysmlGraphDto) -> SysmlGraphDto {
    model_projection::strip_synthetic_nodes(graph)
}

fn workspace_visualization_enabled(scope: &[String]) -> bool {
    model_projection::workspace_visualization_enabled(scope)
}

fn ibd_requested(scope: &[String]) -> bool {
    scope.iter().any(|s| s == "ibd")
}

fn elapsed_ms(start: Instant) -> u32 {
    start.elapsed().as_millis().max(1) as u32
}

async fn log_perf(client: &Client, event: &str, fields: Vec<(&str, String)>) {
    let details = fields
        .into_iter()
        .map(|(key, value)| format!("\"{}\":{}", key, value))
        .collect::<Vec<_>>()
        .join(",");
    client
        .log_message(
            MessageType::INFO,
            format!("[SysML][perf] {{\"event\":\"{}\",{}}}", event, details),
        )
        .await;
}

const TYPING_ATTRIBUTE_KEYS: &[&str] = &[
    "partType",
    "attributeType",
    "portType",
    "actionType",
    "actorType",
    "itemType",
    "occurrenceType",
    "flowType",
    "allocationType",
    "stateType",
    "requirementType",
    "useCaseType",
    "concernType",
    "endType",
    "refType",
    "parameterType",
];

fn node_expects_resolution(node: &semantic_model::SemanticNode) -> bool {
    TYPING_ATTRIBUTE_KEYS
        .iter()
        .any(|key| node.attributes.get(*key).and_then(|v| v.as_str()).is_some())
        || node
            .attributes
            .get("specializes")
            .and_then(|v| v.as_str())
            .is_some()
}

fn count_resolution_stats(semantic_graph: &semantic_model::SemanticGraph, uri: &Url) -> (u32, u32) {
    let mut resolved = 0_u32;
    let mut unresolved = 0_u32;

    for node in semantic_graph.nodes_for_uri(uri) {
        if !node_expects_resolution(node) {
            continue;
        }

        if semantic_graph
            .outgoing_typing_or_specializes_targets(node)
            .is_empty()
        {
            unresolved += 1;
        } else {
            resolved += 1;
        }
    }

    (resolved, unresolved)
}

#[allow(clippy::too_many_arguments)]
pub async fn build_sysml_model_response(
    content: &str,
    parsed: Option<&RootNamespace>,
    parse_time_ms: u32,
    parse_cached: bool,
    semantic_graph: &semantic_model::SemanticGraph,
    uri: &Url,
    library_paths: &[Url],
    scope: &[String],
    build_start: Instant,
    client: &Client,
) -> SysmlModelResultDto {
    let request_phase_start = Instant::now();
    let want_graph = scope.is_empty()
        || scope.iter().any(|s| s == "graph")
        || scope.iter().any(|s| s == "elements")
        || scope.iter().any(|s| s == "relationships");
    let want_general_view_graph =
        scope.is_empty() || scope.iter().any(|s| s == "generalViewGraph") || want_graph;
    let want_stats = scope.is_empty() || scope.iter().any(|s| s == "stats");
    let want_activity_diagrams = scope.is_empty() || scope.iter().any(|s| s == "activityDiagrams");
    let want_ibd = ibd_requested(scope);
    let scope_eval_ms = request_phase_start.elapsed().as_millis().max(1);

    let workspace_viz = workspace_visualization_enabled(scope);
    let graph_start = Instant::now();
    let raw_graph = if want_graph && workspace_viz {
        let graph = build_workspace_graph_dto(semantic_graph, library_paths);
        client
            .log_message(
                MessageType::INFO,
                format!(
                    "sysml/model: workspaceVisualization=true uri={} scope={:?} -> graph nodes={} edges={}",
                    uri.as_str(),
                    scope,
                    graph.nodes.len(),
                    graph.edges.len(),
                ),
            )
            .await;
        Some(graph)
    } else if want_graph {
        let sg_nodes = semantic_graph.nodes_for_uri(uri);
        let node_count = sg_nodes.len();
        let graph_uris = semantic_graph.uris_with_nodes();
        let parsed_ok = parsed.is_some();
        if !parsed_ok {
            let errs = util::parse_failure_diagnostics(content, 5);
            client
                .log_message(
                    MessageType::WARNING,
                    format!(
                        "sysml/model: document in index but parse failed (parsed_ok=false). uri={} parse_errors={}",
                        uri.as_str(),
                        errs.join("; "),
                    ),
                )
                .await;
        }
        client
            .log_message(
                MessageType::INFO,
                format!(
                    "sysml/model: req_uri={} index_ok=true parsed_ok={} semantic_nodes={} graph_uris_count={} graph_uris_sample={:?}",
                    uri.as_str(),
                    parsed_ok,
                    node_count,
                    graph_uris.len(),
                    graph_uris.iter().take(3).collect::<Vec<_>>(),
                ),
            )
            .await;
        let nodes: Vec<GraphNodeDto> = sg_nodes
            .into_iter()
            .map(|n| GraphNodeDto {
                id: n.id.qualified_name.clone(),
                element_type: n.element_kind.clone(),
                name: n.name.clone(),
                parent_id: n.parent_id.as_ref().map(|p| p.qualified_name.clone()),
                range: range_to_dto(n.range),
                attributes: n.attributes.clone(),
            })
            .collect();

        let mut edges: Vec<GraphEdgeDto> = semantic_graph
            .edges_for_uri_as_strings(uri)
            .into_iter()
            .map(|(src, tgt, kind, name)| GraphEdgeDto {
                source: src,
                target: tgt,
                rel_type: kind.as_str().to_string(),
                name,
            })
            .collect();

        for n in semantic_graph.nodes_for_uri(uri) {
            if let Some(ref pid) = n.parent_id {
                edges.push(GraphEdgeDto {
                    source: pid.qualified_name.clone(),
                    target: n.id.qualified_name.clone(),
                    rel_type: "contains".to_string(),
                    name: None,
                });
            }
        }

        Some(SysmlGraphDto { nodes, edges })
    } else {
        None
    };
    let raw_graph_ms = graph_start.elapsed().as_millis().max(1);
    let strip_start = Instant::now();
    let graph = raw_graph.as_ref().map(strip_synthetic_nodes);
    let strip_ms = strip_start.elapsed().as_millis().max(1);
    let general_view_start = Instant::now();
    let general_view_graph = if want_general_view_graph {
        graph
            .as_ref()
            .map(|g| canonical_general_view_graph(g, workspace_viz))
    } else {
        None
    };
    let general_view_ms = general_view_start.elapsed().as_millis().max(1);

    let doc = parsed;
    let activity_diagrams_start = Instant::now();
    let activity_diagrams = if want_activity_diagrams {
        Some(
            doc.map(model::extract_activity_diagrams)
                .unwrap_or_default(),
        )
    } else {
        None
    };
    let activity_diagrams_ms = activity_diagrams_start.elapsed().as_millis().max(1);

    let stats_start = Instant::now();
    let stats = if want_stats {
        let total = graph.as_ref().map(|g| g.nodes.len() as u32).unwrap_or(0);
        let (resolved_elements, unresolved_elements) = count_resolution_stats(semantic_graph, uri);
        Some(SysmlModelStatsDto {
            total_elements: total,
            resolved_elements,
            unresolved_elements,
            parse_time_ms,
            model_build_time_ms: elapsed_ms(build_start),
            parse_cached,
        })
    } else {
        None
    };
    let stats_ms = stats_start.elapsed().as_millis().max(1);

    let node_count = graph.as_ref().map(|g| g.nodes.len()).unwrap_or(0);
    let edge_count = graph.as_ref().map(|g| g.edges.len()).unwrap_or(0);
    let gv_node_count = general_view_graph
        .as_ref()
        .map(|g| g.nodes.len())
        .unwrap_or(0);
    let gv_edge_count = general_view_graph
        .as_ref()
        .map(|g| g.edges.len())
        .unwrap_or(0);
    client
        .log_message(
            MessageType::INFO,
            format!(
                "sysml/model: uri={} scope={:?} -> graph nodes={} edges={} generalViewGraph nodes={} edges={}",
                uri.as_str(),
                scope,
                node_count,
                edge_count,
                gv_node_count,
                gv_edge_count,
            ),
        )
        .await;

    let ibd_start = Instant::now();
    let ibd = if want_ibd && want_graph && graph.is_some() && workspace_viz {
        let workspace_uris = semantic_graph.workspace_uris_excluding_libraries(library_paths);
        let ibds = workspace_uris
            .iter()
            .map(|workspace_uri| ibd::build_ibd_for_uri(semantic_graph, workspace_uri))
            .collect();
        Some(ibd::merge_ibd_payloads(ibds))
    } else if want_ibd && want_graph && graph.is_some() {
        Some(ibd::build_ibd_for_uri(semantic_graph, uri))
    } else {
        None
    };
    let ibd_ms = ibd_start.elapsed().as_millis().max(1);
    let total_ms = request_phase_start.elapsed().as_millis().max(1);
    log_perf(
        client,
        "backend:buildSysmlModelResponse",
        vec![
            ("uri", format!("{:?}", uri.as_str())),
            ("scope", format!("{:?}", scope)),
            ("workspaceVisualization", workspace_viz.to_string()),
            ("wantIbd", want_ibd.to_string()),
            ("scopeEvalMs", scope_eval_ms.to_string()),
            ("rawGraphMs", raw_graph_ms.to_string()),
            ("stripSyntheticMs", strip_ms.to_string()),
            ("generalViewMs", general_view_ms.to_string()),
            ("activityDiagramsMs", activity_diagrams_ms.to_string()),
            ("statsMs", stats_ms.to_string()),
            ("ibdMs", ibd_ms.to_string()),
            ("graphNodes", node_count.to_string()),
            ("graphEdges", edge_count.to_string()),
            ("generalViewNodes", gv_node_count.to_string()),
            ("generalViewEdges", gv_edge_count.to_string()),
            ("totalMs", total_ms.to_string()),
        ],
    )
    .await;

    SysmlModelResultDto {
        version: 0,
        graph,
        general_view_graph,
        stats,
        activity_diagrams,
        ibd,
    }
}

#[cfg(test)]
mod tests {
    use super::ibd_requested;

    #[test]
    fn ibd_is_only_requested_when_scope_explicitly_includes_it() {
        assert!(!ibd_requested(&[]));
        assert!(!ibd_requested(&["graph".to_string(), "stats".to_string()]));
        assert!(ibd_requested(&["graph".to_string(), "ibd".to_string()]));
    }
}
