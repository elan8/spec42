use std::collections::{HashMap, HashSet};

use tower_lsp::lsp_types::Url;

use crate::views::dto::{range_to_dto, GraphEdgeDto, GraphNodeDto, SysmlGraphDto};
use crate::semantic_model;

pub fn canonical_general_view_graph(
    graph: &SysmlGraphDto,
    _include_all_roots: bool,
) -> SysmlGraphDto {
    let mut node_by_id: HashMap<String, GraphNodeDto> = HashMap::new();
    for node in &graph.nodes {
        node_by_id.entry(node.id.clone()).or_insert_with(|| node.clone());
    }

    let mut edge_keys: HashSet<(String, String, String, Option<String>)> = HashSet::new();
    let mut out_edges: Vec<GraphEdgeDto> = Vec::new();
    for edge in &graph.edges {
        let key = (
            edge.source.clone(),
            edge.target.clone(),
            edge.rel_type.to_lowercase(),
            edge.name.clone(),
        );
        if edge_keys.insert(key) {
            out_edges.push(edge.clone());
        }
    }

    let mut out_nodes: Vec<GraphNodeDto> = node_by_id.into_values().collect();
    out_nodes.sort_by(|a, b| a.id.cmp(&b.id));
    out_edges.sort_by(|a, b| {
        (
            a.source.as_str(),
            a.target.as_str(),
            a.rel_type.to_lowercase(),
            a.name.as_deref().unwrap_or(""),
        )
            .cmp(&(
                b.source.as_str(),
                b.target.as_str(),
                b.rel_type.to_lowercase(),
                b.name.as_deref().unwrap_or(""),
            ))
    });
    SysmlGraphDto { nodes: out_nodes, edges: out_edges }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views::dto::{GraphEdgeDto, GraphNodeDto, PositionDto, RangeDto};

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

    #[test]
    fn canonical_general_view_graph_preserves_subject_edges_for_retained_nodes() {
        let graph = SysmlGraphDto {
            nodes: vec![
                GraphNodeDto {
                    id: "Pkg::Drone".to_string(),
                    element_type: "part def".to_string(),
                    name: "Drone".to_string(),
                    parent_id: None,
                    range: range(),
                    attributes: Default::default(),
                },
                GraphNodeDto {
                    id: "Pkg::Req".to_string(),
                    element_type: "requirement def".to_string(),
                    name: "Req".to_string(),
                    parent_id: None,
                    range: range(),
                    attributes: Default::default(),
                },
                GraphNodeDto {
                    id: "Pkg::Root".to_string(),
                    element_type: "part def".to_string(),
                    name: "Root".to_string(),
                    parent_id: None,
                    range: range(),
                    attributes: Default::default(),
                },
            ],
            edges: vec![
                GraphEdgeDto {
                    source: "Pkg::Root".to_string(),
                    target: "Pkg::Drone".to_string(),
                    rel_type: "contains".to_string(),
                    name: None,
                },
                GraphEdgeDto {
                    source: "Pkg::Req".to_string(),
                    target: "Pkg::Drone".to_string(),
                    rel_type: "subject".to_string(),
                    name: None,
                },
            ],
        };

        let canonical = canonical_general_view_graph(&graph, false);
        assert!(
            canonical.nodes.iter().any(|node| node.id == "Pkg::Req"),
            "subject source node should be pulled into the canonical General View"
        );
        assert!(
            canonical
                .edges
                .iter()
                .any(|edge| edge.rel_type == "subject"
                    && edge.source == "Pkg::Req"
                    && edge.target == "Pkg::Drone"),
            "subject edge should survive canonical General View projection"
        );
    }

    #[test]
    fn strip_synthetic_nodes_removes_auto_expanded_instantiation_content() {
        let graph = SysmlGraphDto {
            nodes: vec![
                GraphNodeDto {
                    id: "Office::my_computer".to_string(),
                    element_type: "part".to_string(),
                    name: "my_computer".to_string(),
                    parent_id: Some("Office".to_string()),
                    range: range(),
                    attributes: Default::default(),
                },
                GraphNodeDto {
                    id: "Office::my_computer::laptop".to_string(),
                    element_type: "part".to_string(),
                    name: "laptop".to_string(),
                    parent_id: Some("Office::my_computer".to_string()),
                    range: range(),
                    attributes: serde_json::json!({ "synthetic": true })
                        .as_object()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                },
                GraphNodeDto {
                    id: "Office::Laptop".to_string(),
                    element_type: "part def".to_string(),
                    name: "Laptop".to_string(),
                    parent_id: Some("Office".to_string()),
                    range: range(),
                    attributes: Default::default(),
                },
            ],
            edges: vec![
                GraphEdgeDto {
                    source: "Office::my_computer".to_string(),
                    target: "Office::my_computer::laptop".to_string(),
                    rel_type: "contains".to_string(),
                    name: None,
                },
                GraphEdgeDto {
                    source: "Office::my_computer::laptop".to_string(),
                    target: "Office::Laptop".to_string(),
                    rel_type: "typing".to_string(),
                    name: None,
                },
            ],
        };

        let stripped = strip_synthetic_nodes(&graph);

        assert!(
            stripped
                .nodes
                .iter()
                .all(|node| node.id != "Office::my_computer::laptop"),
            "synthetic instantiation-expanded parts should be removed from general-view input"
        );
        assert!(
            stripped.edges.is_empty(),
            "edges touching synthetic instantiation-expanded content should be removed too: {:?}",
            stripped
                .edges
                .iter()
                .map(|edge| (&edge.source, &edge.target, &edge.rel_type))
                .collect::<Vec<_>>()
        );
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
        .filter(|edge| {
            !synthetic_ids.contains(&edge.source) && !synthetic_ids.contains(&edge.target)
        })
        .cloned()
        .collect();
    SysmlGraphDto {
        nodes: concrete_nodes,
        edges: concrete_edges,
    }
}

#[allow(dead_code)]
pub fn workspace_visualization_enabled(scope: &[String]) -> bool {
    scope.iter().any(|s| s == "workspaceVisualization")
}
