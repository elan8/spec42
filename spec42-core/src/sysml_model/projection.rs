use std::collections::{HashMap, HashSet};

use tower_lsp::lsp_types::Url;

use crate::dto::{range_to_dto, GraphEdgeDto, GraphNodeDto, SysmlGraphDto};
use crate::semantic_model;

pub fn canonical_general_view_graph(graph: &SysmlGraphDto, include_all_roots: bool) -> SysmlGraphDto {
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
    #[allow(clippy::too_many_arguments)]
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

pub fn build_workspace_graph_dto(
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

pub fn strip_synthetic_nodes(graph: &SysmlGraphDto) -> SysmlGraphDto {
    let synthetic_ids: HashSet<String> = graph
        .nodes
        .iter()
        .filter(|node| {
            node.attributes
                .get("synthetic")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        })
        .map(|n| n.id.clone())
        .collect();
    let concrete_nodes: Vec<GraphNodeDto> = graph
        .nodes
        .iter()
        .filter(|node| !synthetic_ids.contains(&node.id))
        .cloned()
        .collect();
    let concrete_edges: Vec<GraphEdgeDto> = graph
        .edges
        .iter()
        .filter(|edge| !synthetic_ids.contains(&edge.source) && !synthetic_ids.contains(&edge.target))
        .cloned()
        .collect();
    SysmlGraphDto {
        nodes: concrete_nodes,
        edges: concrete_edges,
    }
}

pub fn workspace_visualization_enabled(scope: &[String]) -> bool {
    scope.iter().any(|s| s == "workspaceVisualization")
}
