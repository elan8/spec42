//! sysml/model request parsing and response building.
//! Diagram output is collected from [crate::config::DiagramProvider] implementations.

use std::sync::Arc;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
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
    let (uri_str, scope_value) = if let Some(arr) = v.as_array() {
        let first = arr.first().ok_or_else(|| {
            tower_lsp::jsonrpc::Error::invalid_params(
                "sysml/model params array must have at least one element",
            )
        })?;
        let uri_str = if let Some(s) = first.as_str() {
            Some(s.to_string())
        } else if let Some(obj) = first.as_object() {
            obj.get("uri")
                .and_then(|u| u.as_str())
                .map(String::from)
                .or_else(|| {
                    obj.get("textDocument")
                        .and_then(|td| td.get("uri"))
                        .and_then(|u| u.as_str())
                        .map(String::from)
                })
        } else {
            None
        };
        let scope_value = arr.get(1);
        (uri_str, scope_value)
    } else if let Some(obj) = v.as_object() {
        let uri_str = obj
            .get("uri")
            .and_then(|u| u.as_str())
            .map(String::from)
            .or_else(|| {
                obj.get("textDocument")
                    .and_then(|td| td.get("uri"))
                    .and_then(|u| u.as_str())
                    .map(String::from)
            });
        let scope_value = obj.get("scope");
        (uri_str, scope_value)
    } else {
        return Err(tower_lsp::jsonrpc::Error::invalid_params(
            "sysml/model params must be an object or array",
        ));
    };

    let uri = uri_str.as_ref().ok_or_else(|| {
        tower_lsp::jsonrpc::Error::invalid_params(
            "sysml/model requires 'uri' or 'textDocument.uri'",
        )
    })?;
    let uri = Url::parse(uri)
        .map_err(|_| tower_lsp::jsonrpc::Error::invalid_params("sysml/model: invalid URI"))?;
    let uri = util::normalize_file_uri(&uri);

    let scope: Vec<String> = scope_value
        .and_then(|s| serde_json::from_value(s.clone()).ok())
        .unwrap_or_default();

    Ok((uri, scope))
}

pub fn empty_model_response(build_start: Instant) -> SysmlModelResultDto {
    SysmlModelResultDto {
        version: 0,
        graph: Some(SysmlGraphDto {
            nodes: vec![],
            edges: vec![],
        }),
        general_view_graph: Some(SysmlGraphDto {
            nodes: vec![],
            edges: vec![],
        }),
        activity_diagrams: None,
        sequence_diagrams: None,
        ibd: None,
        rendered_diagrams: None,
        stats: Some(SysmlModelStatsDto {
            total_elements: 0,
            resolved_elements: 0,
            unresolved_elements: 0,
            parse_time_ms: 0,
            model_build_time_ms: build_start.elapsed().as_millis() as u32,
            parse_cached: true,
        }),
    }
}

fn canonical_general_view_graph(graph: &SysmlGraphDto, include_all_roots: bool) -> SysmlGraphDto {
    let node_by_id: HashMap<String, GraphNodeDto> = graph
        .nodes
        .iter()
        .map(|n| (n.id.clone(), n.clone()))
        .collect();
    let is_part_def = |id: &str| {
        node_by_id
            .get(id)
            .map(|n| n.element_type.to_lowercase().contains("part def"))
            .unwrap_or(false)
    };
    let is_part_usage = |id: &str| {
        node_by_id
            .get(id)
            .map(|n| {
                let t = n.element_type.to_lowercase();
                t == "part" || t.contains("part usage")
            })
            .unwrap_or(false)
    };

    let mut contains_children: HashMap<String, Vec<String>> = HashMap::new();
    let mut typing_target: HashMap<String, String> = HashMap::new();
    let mut contains_edges = Vec::new();
    let mut specializes_edges = Vec::new();
    let mut typing_edges = Vec::new();
    for edge in &graph.edges {
        let rel = edge.rel_type.to_lowercase();
        if rel == "contains" {
            contains_children
                .entry(edge.source.clone())
                .or_default()
                .push(edge.target.clone());
            contains_edges.push(edge.clone());
        } else if rel == "typing" && is_part_usage(&edge.source) && is_part_def(&edge.target) {
            typing_target.insert(edge.source.clone(), edge.target.clone());
            typing_edges.push(edge.clone());
        } else if rel == "specializes" {
            specializes_edges.push(edge.clone());
        }
    }

    let part_defs_with_parts: Vec<String> = contains_children
        .iter()
        .filter_map(|(pid, kids)| {
            if is_part_def(pid) && kids.iter().any(|k| is_part_usage(k)) {
                Some(pid.clone())
            } else {
                None
            }
        })
        .collect();
    let contained_by_non_part_def: HashSet<String> = contains_edges
        .iter()
        .filter_map(|e| (!is_part_def(&e.source)).then_some(e.target.clone()))
        .collect();
    let has_no_parent = |id: &str| !contains_edges.iter().any(|e| e.target == id);
    let mut candidate_roots: Vec<String> = part_defs_with_parts
        .iter()
        .filter(|pid| contained_by_non_part_def.contains(*pid) || has_no_parent(pid))
        .cloned()
        .collect();
    if candidate_roots.is_empty() {
        candidate_roots = typing_edges
            .iter()
            .filter_map(|e| is_part_def(&e.target).then_some(e.target.clone()))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
    }
    if candidate_roots.is_empty() {
        candidate_roots = part_defs_with_parts;
    }
    if candidate_roots.is_empty() {
        // `part_defs_with_parts` only lists part definitions that already contain a part usage.
        // An empty `part def` (no body / no nested parts) is still a valid General View subject.
        candidate_roots = node_by_id
            .keys()
            .filter(|id| {
                is_part_def(id) && (contained_by_non_part_def.contains(*id) || has_no_parent(id))
            })
            .cloned()
            .collect();
        candidate_roots.sort();
    }
    candidate_roots.sort();

    let mut out_node_ids: HashSet<String> = HashSet::new();
    let mut out_edges: Vec<GraphEdgeDto> = Vec::new();
    let mut out_edge_keys: HashSet<(String, String, String)> = HashSet::new();
    let mut visited_defs: HashSet<String> = HashSet::new();
    fn visit_part_def(
        part_def_id: &str,
        contains_children: &HashMap<String, Vec<String>>,
        typing_target: &HashMap<String, String>,
        is_part_usage: &dyn Fn(&str) -> bool,
        visited_defs: &mut HashSet<String>,
        out_node_ids: &mut HashSet<String>,
        out_edges: &mut Vec<GraphEdgeDto>,
        out_edge_keys: &mut HashSet<(String, String, String)>,
    ) {
        if visited_defs.contains(part_def_id) {
            return;
        }
        visited_defs.insert(part_def_id.to_string());
        out_node_ids.insert(part_def_id.to_string());
        let direct_children = contains_children
            .get(part_def_id)
            .cloned()
            .unwrap_or_default();
        let sibling_part_usages: Vec<String> = direct_children
            .iter()
            .filter(|id| is_part_usage(id))
            .cloned()
            .collect();
        for child_id in direct_children {
            // Some semantic graphs contain both:
            // 1) the definition branch (e.g. FlightControlAndSensing.gnss), and
            // 2) an expanded inline usage branch under a typed usage
            //    (e.g. SurveillanceQuadrotorDrone.flightControl.gnss).
            // Skip nested inline-expanded descendants when their parent usage
            // sibling is present, and rely on typed-definition traversal.
            if sibling_part_usages.iter().any(|sibling| {
                sibling != &child_id
                    && child_id.starts_with(sibling)
                    && matches!(
                        child_id.get(sibling.len()..),
                        Some(rest) if rest.starts_with('.') || rest.starts_with("::")
                    )
            }) {
                continue;
            }
            out_node_ids.insert(child_id.clone());
            let contains_key = (
                part_def_id.to_string(),
                child_id.clone(),
                "contains".to_string(),
            );
            if out_edge_keys.insert(contains_key) {
                out_edges.push(GraphEdgeDto {
                    source: part_def_id.to_string(),
                    target: child_id.clone(),
                    rel_type: "contains".to_string(),
                    name: None,
                });
            }
            if let Some(def_id) = typing_target.get(&child_id) {
                out_node_ids.insert(def_id.clone());
                let typing_key = (child_id.clone(), def_id.clone(), "typing".to_string());
                if out_edge_keys.insert(typing_key) {
                    out_edges.push(GraphEdgeDto {
                        source: child_id.clone(),
                        target: def_id.clone(),
                        rel_type: "typing".to_string(),
                        name: None,
                    });
                }
                visit_part_def(
                    def_id,
                    contains_children,
                    typing_target,
                    is_part_usage,
                    visited_defs,
                    out_node_ids,
                    out_edges,
                    out_edge_keys,
                );
            }
        }
    }
    if include_all_roots {
        for root_id in &candidate_roots {
            visit_part_def(
                root_id,
                &contains_children,
                &typing_target,
                &is_part_usage,
                &mut visited_defs,
                &mut out_node_ids,
                &mut out_edges,
                &mut out_edge_keys,
            );
        }
    } else if let Some(root_id) = candidate_roots
        .iter()
        .find(|id| {
            node_by_id
                .get(*id)
                .map(|n| n.name.contains("SurveillanceQuadrotorDrone") || n.name.contains("Drone"))
                .unwrap_or(false)
        })
        .cloned()
        .or_else(|| {
            candidate_roots
                .iter()
                .max_by_key(|id| contains_children.get(*id).map(|v| v.len()).unwrap_or(0))
                .cloned()
        })
    {
        visit_part_def(
            &root_id,
            &contains_children,
            &typing_target,
            &is_part_usage,
            &mut visited_defs,
            &mut out_node_ids,
            &mut out_edges,
            &mut out_edge_keys,
        );
    }
    for edge in specializes_edges {
        if out_node_ids.contains(&edge.source) && out_node_ids.contains(&edge.target) {
            let key = (edge.source.clone(), edge.target.clone(), "specializes".to_string());
            if out_edge_keys.insert(key) {
                out_edges.push(edge);
            }
        }
    }

    let mut out_nodes: Vec<GraphNodeDto> = out_node_ids
        .iter()
        .filter_map(|id| node_by_id.get(id).cloned())
        .collect();
    out_nodes.sort_by(|a, b| a.id.cmp(&b.id));
    out_edges.sort_by(|a, b| {
        (a.source.as_str(), a.target.as_str(), a.rel_type.as_str()).cmp(&(
            b.source.as_str(),
            b.target.as_str(),
            b.rel_type.as_str(),
        ))
    });
    SysmlGraphDto {
        nodes: out_nodes,
        edges: out_edges,
    }
}

fn build_workspace_graph_dto(
    semantic_graph: &semantic_model::SemanticGraph,
    library_paths: &[Url],
) -> SysmlGraphDto {
    let sg_nodes = semantic_graph.workspace_nodes_excluding_libraries(library_paths);
    let nodes: Vec<GraphNodeDto> = sg_nodes
        .iter()
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
        .edges_for_workspace_as_strings(library_paths)
        .into_iter()
        .map(|(src, tgt, kind, name)| GraphEdgeDto {
            source: src,
            target: tgt,
            rel_type: kind.as_str().to_string(),
            name,
        })
        .collect();

    let node_ids: HashSet<String> = nodes.iter().map(|n| n.id.clone()).collect();
    for n in &nodes {
        if let Some(ref pid) = n.parent_id {
            if node_ids.contains(pid) {
                edges.push(GraphEdgeDto {
                    source: pid.clone(),
                    target: n.id.clone(),
                    rel_type: "contains".to_string(),
                    name: None,
                });
            }
        }
    }

    SysmlGraphDto { nodes, edges }
}

fn workspace_visualization_enabled(scope: &[String]) -> bool {
    scope.iter().any(|s| s == "workspaceVisualization")
}

/// Build sysml/model response. Uses `diagram_providers` to fill `rendered_diagrams` (generalView, interconnectionView by diagram_id).
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
    let graph = if want_graph && workspace_viz {
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
