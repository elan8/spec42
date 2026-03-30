//! sysml/model request parsing and response building.
//! Diagram output is collected from [crate::config::DiagramProvider] implementations.

mod params;
mod projection;

use std::sync::Arc;
use std::time::Instant;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{MessageType, Url};
use tower_lsp::Client;

use sysml_parser::RootNamespace;

use crate::config::{DiagramContext, DiagramProvider};
use crate::dto::{
    range_to_dto, rendered_diagram_to_dto, GraphEdgeDto, GraphNodeDto, RenderedDiagramsDto,
    SysmlGraphDto, SysmlModelResultDto, SysmlModelStatsDto,
};
use crate::ibd;
use crate::model;
use crate::semantic_model;
use crate::util;

/// Parse sysml/model params from JSON-RPC value.
pub fn parse_sysml_model_params(v: &serde_json::Value) -> Result<(Url, Vec<String>)> {
    params::parse_sysml_model_params(v)
}

pub fn empty_model_response(build_start: Instant) -> SysmlModelResultDto {
    params::empty_model_response(build_start)
}

fn canonical_general_view_graph(graph: &SysmlGraphDto, include_all_roots: bool) -> SysmlGraphDto {
    projection::canonical_general_view_graph(graph, include_all_roots)
}

fn build_workspace_graph_dto(
    semantic_graph: &semantic_model::SemanticGraph,
    library_paths: &[Url],
) -> SysmlGraphDto {
    projection::build_workspace_graph_dto(semantic_graph, library_paths)
}

fn strip_synthetic_nodes(graph: &SysmlGraphDto) -> SysmlGraphDto {
    projection::strip_synthetic_nodes(graph)
}

fn workspace_visualization_enabled(scope: &[String]) -> bool {
    projection::workspace_visualization_enabled(scope)
}

/// Build sysml/model response. Uses `diagram_providers` to fill `rendered_diagrams` (generalView, interconnectionView by diagram_id).
#[allow(clippy::too_many_arguments)]
pub async fn build_sysml_model_response(
    content: &str,
    parsed: Option<&RootNamespace>,
    semantic_graph: &semantic_model::SemanticGraph,
    uri: &Url,
    library_paths: &[Url],
    scope: &[String],
    build_start: Instant,
    client: &Client,
    diagram_providers: &[Arc<dyn DiagramProvider>],
) -> SysmlModelResultDto {
    let want_graph = scope.is_empty()
        || scope.iter().any(|s| s == "graph")
        || scope.iter().any(|s| s == "elements")
        || scope.iter().any(|s| s == "relationships");
    let want_general_view_graph =
        scope.is_empty() || scope.iter().any(|s| s == "generalViewGraph") || want_graph;
    let want_stats = scope.is_empty() || scope.iter().any(|s| s == "stats");
    let want_activity_diagrams = scope.is_empty() || scope.iter().any(|s| s == "activityDiagrams");
    let want_sequence_diagrams = scope.is_empty() || scope.iter().any(|s| s == "sequenceDiagrams");
    let want_rendered_diagrams =
        scope.is_empty() || scope.iter().any(|s| s == "renderedDiagrams");

    let workspace_viz = workspace_visualization_enabled(scope);
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
    let graph = raw_graph.as_ref().map(strip_synthetic_nodes);
    let general_view_graph = if want_general_view_graph {
        graph.as_ref()
            .map(|g| canonical_general_view_graph(g, workspace_viz))
    } else {
        None
    };

    let doc = parsed;

    let activity_diagrams = if want_activity_diagrams {
        Some(
            doc.map(model::extract_activity_diagrams)
                .unwrap_or_default(),
        )
    } else {
        None
    };

    let sequence_diagrams = if want_sequence_diagrams {
        Some(
            doc.map(model::extract_sequence_diagrams)
                .unwrap_or_default(),
        )
    } else {
        None
    };

    let stats = if want_stats {
        let total = graph.as_ref().map(|g| g.nodes.len() as u32).unwrap_or(0);
        Some(SysmlModelStatsDto {
            total_elements: total,
            resolved_elements: 0,
            unresolved_elements: 0,
            parse_time_ms: 0,
            model_build_time_ms: build_start.elapsed().as_millis() as u32,
            parse_cached: true,
        })
    } else {
        None
    };

    let node_count = graph.as_ref().map(|g| g.nodes.len()).unwrap_or(0);
    let edge_count = graph.as_ref().map(|g| g.edges.len()).unwrap_or(0);
    let gv_node_count = general_view_graph.as_ref().map(|g| g.nodes.len()).unwrap_or(0);
    let gv_edge_count = general_view_graph.as_ref().map(|g| g.edges.len()).unwrap_or(0);
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

    let ibd = if want_graph && graph.is_some() && workspace_viz {
        let workspace_uris = semantic_graph.workspace_uris_excluding_libraries(library_paths);
        let ibds = workspace_uris
            .iter()
            .map(|workspace_uri| ibd::build_ibd_for_uri(semantic_graph, workspace_uri))
            .collect();
        Some(ibd::merge_ibd_payloads(ibds))
    } else if want_graph && graph.is_some() {
        Some(ibd::build_ibd_for_uri(semantic_graph, uri))
    } else {
        None
    };

    let rendered_diagrams = if want_rendered_diagrams && (graph.is_some() || ibd.is_some()) {
        let context = DiagramContext {
            graph: graph.as_ref(),
            ibd: ibd.as_ref(),
            uri,
        };
        let mut diagrams = std::collections::HashMap::new();
        for provider in diagram_providers {
            if let Some(diagram) = provider.render(&context) {
                let dto = rendered_diagram_to_dto(diagram);
                diagrams.insert(provider.diagram_id().to_string(), dto);
            }
        }
        Some(RenderedDiagramsDto(diagrams))
    } else {
        None
    };

    SysmlModelResultDto {
        version: 0,
        graph,
        general_view_graph,
        activity_diagrams,
        sequence_diagrams,
        ibd,
        rendered_diagrams,
        stats,
    }
}

#[cfg(test)]
mod tests {
    use super::canonical_general_view_graph;
    use crate::dto::{GraphEdgeDto, GraphNodeDto, PositionDto, RangeDto, SysmlGraphDto};
    use std::collections::HashMap;

    fn range() -> RangeDto {
        RangeDto {
            start: PositionDto {
                line: 0,
                character: 0,
            },
            end: PositionDto {
                line: 0,
                character: 1,
            },
        }
    }

    fn node(id: &str, ty: &str, name: &str, parent_id: Option<&str>) -> GraphNodeDto {
        GraphNodeDto {
            id: id.to_string(),
            element_type: ty.to_string(),
            name: name.to_string(),
            parent_id: parent_id.map(str::to_string),
            range: range(),
            attributes: HashMap::new(),
        }
    }

    fn edge(source: &str, target: &str, rel_type: &str) -> GraphEdgeDto {
        GraphEdgeDto {
            source: source.to_string(),
            target: target.to_string(),
            rel_type: rel_type.to_string(),
            name: None,
        }
    }

    #[test]
    fn canonical_general_view_graph_selects_single_root_drone_tree() {
        let graph = SysmlGraphDto {
            nodes: vec![
                node("Pkg", "package", "SurveillanceDrone", None),
                node("Drone", "part def", "SurveillanceQuadrotorDrone", Some("Pkg")),
                node("Alt", "part def", "FlightControlAndSensing", Some("Pkg")),
                node("Drone::fc", "part", "flightControlAndSensing", Some("Drone")),
                node("Alt::gnss", "part", "gnss", Some("Alt")),
                node("GNSSReceiver", "part def", "GNSSReceiver", Some("Pkg")),
            ],
            edges: vec![
                edge("Pkg", "Drone", "contains"),
                edge("Pkg", "Alt", "contains"),
                edge("Drone", "Drone::fc", "contains"),
                edge("Drone::fc", "Alt", "typing"),
                edge("Alt", "Alt::gnss", "contains"),
                edge("Alt::gnss", "GNSSReceiver", "typing"),
            ],
        };

        let projected = canonical_general_view_graph(&graph, false);
        let ids: std::collections::HashSet<String> =
            projected.nodes.iter().map(|n| n.id.clone()).collect();
        assert!(ids.contains("Drone"));
        assert!(ids.contains("Drone::fc"));
        assert!(ids.contains("Alt"));
        assert!(ids.contains("Alt::gnss"));
        assert!(ids.contains("GNSSReceiver"));
        assert!(!ids.contains("Pkg"), "package node should not be part of canonical projection");
    }

    #[test]
    fn canonical_general_view_graph_edges_reference_existing_nodes_and_are_unique() {
        let graph = SysmlGraphDto {
            nodes: vec![
                node("Root", "part def", "Root", None),
                node("Root::a", "part", "a", Some("Root")),
                node("AType", "part def", "AType", None),
            ],
            edges: vec![
                edge("Root", "Root::a", "contains"),
                edge("Root::a", "AType", "typing"),
                edge("Root::a", "AType", "typing"),
            ],
        };

        let projected = canonical_general_view_graph(&graph, false);
        let ids: std::collections::HashSet<String> =
            projected.nodes.iter().map(|n| n.id.clone()).collect();
        let mut seen = std::collections::HashSet::new();
        for e in &projected.edges {
            assert!(ids.contains(&e.source), "missing edge source node {}", e.source);
            assert!(ids.contains(&e.target), "missing edge target node {}", e.target);
            let key = (e.source.clone(), e.target.clone(), e.rel_type.clone());
            assert!(seen.insert(key), "duplicate edge found in projected graph");
        }
    }

    #[test]
    fn canonical_general_view_graph_includes_part_def_with_no_part_usages() {
        let graph = SysmlGraphDto {
            nodes: vec![
                node("Pkg", "package", "MyPkg", None),
                node("Widget", "part def", "Widget", Some("Pkg")),
            ],
            edges: vec![edge("Pkg", "Widget", "contains")],
        };

        let projected = canonical_general_view_graph(&graph, false);
        let ids: std::collections::HashSet<String> =
            projected.nodes.iter().map(|n| n.id.clone()).collect();
        assert!(
            ids.contains("Widget"),
            "empty part def under package should still appear in general view graph: {:?}",
            ids
        );
        assert!(!ids.contains("Pkg"));
        assert!(projected.edges.is_empty());
    }

    #[test]
    fn canonical_general_view_graph_skips_inline_nested_usage_duplicates() {
        let graph = SysmlGraphDto {
            nodes: vec![
                node("Drone", "part def", "Drone", None),
                node("Drone::flightControl", "part", "flightControl", Some("Drone")),
                node(
                    "Drone::flightControl::gnss",
                    "part",
                    "gnss",
                    Some("Drone::flightControl"),
                ),
                node(
                    "FlightControlAndSensing",
                    "part def",
                    "FlightControlAndSensing",
                    None,
                ),
                node(
                    "FlightControlAndSensing::gnss",
                    "part",
                    "gnss",
                    Some("FlightControlAndSensing"),
                ),
                node("GNSSReceiver", "part def", "GNSSReceiver", None),
            ],
            edges: vec![
                edge("Drone", "Drone::flightControl", "contains"),
                edge("Drone", "Drone::flightControl::gnss", "contains"),
                edge("Drone::flightControl", "FlightControlAndSensing", "typing"),
                edge("Drone::flightControl::gnss", "GNSSReceiver", "typing"),
                edge("FlightControlAndSensing", "FlightControlAndSensing::gnss", "contains"),
                edge("FlightControlAndSensing::gnss", "GNSSReceiver", "typing"),
            ],
        };

        let projected = canonical_general_view_graph(&graph, false);
        let gnss_nodes: Vec<_> = projected
            .nodes
            .iter()
            .filter(|n| n.name == "gnss")
            .map(|n| n.id.as_str())
            .collect();
        assert_eq!(
            gnss_nodes.len(),
            1,
            "expected inline expanded gnss usage to be removed, got: {:?}",
            gnss_nodes
        );
        assert!(
            gnss_nodes.contains(&"FlightControlAndSensing::gnss"),
            "expected canonical branch to retain typed-definition gnss"
        );
    }
}
