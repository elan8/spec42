use std::collections::{HashMap, HashSet};

use tower_lsp::lsp_types::Url;

use crate::views::dto::{range_to_dto, GraphEdgeDto, GraphNodeDto, SysmlGraphDto};
use crate::semantic_model;

pub fn canonical_general_view_graph(
    graph: &SysmlGraphDto,
    _include_all_roots: bool,
) -> SysmlGraphDto {
    let filtered_graph = fold_general_view_leaf_details_into_owners(graph);

    let mut node_by_id: HashMap<String, GraphNodeDto> = HashMap::new();
    for node in &filtered_graph.nodes {
        node_by_id.entry(node.id.clone()).or_insert_with(|| node.clone());
    }

    let mut edge_keys: HashSet<(String, String, String, Option<String>)> = HashSet::new();
    let mut out_edges: Vec<GraphEdgeDto> = Vec::new();
    for edge in &filtered_graph.edges {
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

fn fold_general_view_leaf_details_into_owners(graph: &SysmlGraphDto) -> SysmlGraphDto {
    let node_by_id: HashMap<&str, &GraphNodeDto> = graph
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect();
    let typing_targets: HashMap<&str, &str> = graph
        .edges
        .iter()
        .filter(|edge| edge.rel_type.eq_ignore_ascii_case("typing"))
        .map(|edge| (edge.source.as_str(), edge.target.as_str()))
        .collect();

    let detail_ids: HashSet<&str> = graph
        .nodes
        .iter()
        .filter(|node| is_general_view_inline_detail(&node.element_type))
        .map(|node| node.id.as_str())
        .collect();

    if detail_ids.is_empty() {
        return graph.clone();
    }

    let mut owner_detail_lines: HashMap<String, (Vec<String>, Vec<String>)> = HashMap::new();
    for detail in graph.nodes.iter().filter(|node| detail_ids.contains(node.id.as_str())) {
        let Some(owner_id) = detail.parent_id.as_ref() else {
            continue;
        };
        if detail_ids.contains(owner_id.as_str()) {
            continue;
        }
        if !node_by_id.contains_key(owner_id.as_str()) {
            continue;
        }

        let detail_line = format_general_view_detail_line(detail, &node_by_id, &typing_targets);
        let entry = owner_detail_lines
            .entry(owner_id.clone())
            .or_insert_with(|| (Vec::new(), Vec::new()));
        if is_port_like(&detail.element_type) {
            push_unique_line(&mut entry.1, detail_line);
        } else if is_attribute_like(&detail.element_type) {
            push_unique_line(&mut entry.0, detail_line);
        }
    }

    let mut out_nodes: Vec<GraphNodeDto> = graph
        .nodes
        .iter()
        .filter(|node| !detail_ids.contains(node.id.as_str()))
        .cloned()
        .collect();
    for node in &mut out_nodes {
        if let Some((attributes, ports)) = owner_detail_lines.get(&node.id) {
            if !attributes.is_empty() {
                node.attributes.insert(
                    "generalViewAttributes".to_string(),
                    serde_json::Value::Array(
                        attributes
                            .iter()
                            .cloned()
                            .map(serde_json::Value::String)
                            .collect(),
                    ),
                );
            }
            if !ports.is_empty() {
                node.attributes.insert(
                    "generalViewPorts".to_string(),
                    serde_json::Value::Array(
                        ports
                            .iter()
                            .cloned()
                            .map(serde_json::Value::String)
                            .collect(),
                    ),
                );
            }
        }
    }

    let out_edges: Vec<GraphEdgeDto> = graph
        .edges
        .iter()
        .filter(|edge| {
            !detail_ids.contains(edge.source.as_str())
                && !detail_ids.contains(edge.target.as_str())
        })
        .cloned()
        .collect();

    SysmlGraphDto {
        nodes: out_nodes,
        edges: out_edges,
    }
}

fn is_general_view_inline_detail(element_type: &str) -> bool {
    let lower = element_type.to_lowercase();
    is_port_like(&lower) || is_attribute_like(&lower) || is_parameter_like(&lower)
}

fn is_port_like(element_type: &str) -> bool {
    element_type.to_lowercase().contains("port")
}

fn is_attribute_like(element_type: &str) -> bool {
    let lower = element_type.to_lowercase();
    lower.contains("attribute") || lower.contains("property")
}

fn is_parameter_like(element_type: &str) -> bool {
    element_type.to_lowercase().contains("parameter")
}

fn push_unique_line(lines: &mut Vec<String>, line: String) {
    if !lines.iter().any(|existing| existing == &line) {
        lines.push(line);
    }
}

fn format_general_view_detail_line(
    detail: &GraphNodeDto,
    node_by_id: &HashMap<&str, &GraphNodeDto>,
    typing_targets: &HashMap<&str, &str>,
) -> String {
    let name = detail.name.trim();
    let typed = typing_targets
        .get(detail.id.as_str())
        .and_then(|target_id| node_by_id.get(target_id))
        .map(|target| target.name.as_str())
        .or_else(|| {
            if is_port_like(&detail.element_type) {
                detail
                    .attributes
                    .get("portType")
                    .and_then(|value| value.as_str())
            } else {
                detail
                    .attributes
                    .get("dataType")
                    .and_then(|value| value.as_str())
                    .or_else(|| detail.attributes.get("type").and_then(|value| value.as_str()))
            }
        })
        .map(|type_name| type_name.split("::").last().unwrap_or(type_name).to_string());

    match typed {
        Some(type_name) if !type_name.is_empty() => format!("  {name} : {type_name}"),
        _ => format!("  {name}"),
    }
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

    #[test]
    fn canonical_general_view_graph_inlines_ports_and_attributes_into_owner_nodes() {
        let graph = SysmlGraphDto {
            nodes: vec![
                GraphNodeDto {
                    id: "Pkg::Laptop".to_string(),
                    element_type: "part def".to_string(),
                    name: "Laptop".to_string(),
                    parent_id: None,
                    range: range(),
                    attributes: Default::default(),
                },
                GraphNodeDto {
                    id: "Pkg::Laptop::voltage".to_string(),
                    element_type: "attribute".to_string(),
                    name: "voltage".to_string(),
                    parent_id: Some("Pkg::Laptop".to_string()),
                    range: range(),
                    attributes: serde_json::json!({ "dataType": "ScalarValues::Volt" })
                        .as_object()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                },
                GraphNodeDto {
                    id: "Pkg::Laptop::powerIn".to_string(),
                    element_type: "port".to_string(),
                    name: "powerIn".to_string(),
                    parent_id: Some("Pkg::Laptop".to_string()),
                    range: range(),
                    attributes: serde_json::json!({ "portType": "PowerPort" })
                        .as_object()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                },
            ],
            edges: vec![
                GraphEdgeDto {
                    source: "Pkg::Laptop".to_string(),
                    target: "Pkg::Laptop::voltage".to_string(),
                    rel_type: "contains".to_string(),
                    name: None,
                },
                GraphEdgeDto {
                    source: "Pkg::Laptop".to_string(),
                    target: "Pkg::Laptop::powerIn".to_string(),
                    rel_type: "contains".to_string(),
                    name: None,
                },
            ],
        };

        let canonical = canonical_general_view_graph(&graph, false);
        assert_eq!(canonical.nodes.len(), 1, "port and attribute nodes should be filtered from General View");
        let owner = canonical.nodes.iter().find(|node| node.id == "Pkg::Laptop").expect("owner node");
        assert_eq!(
            owner.attributes.get("generalViewAttributes"),
            Some(&serde_json::json!(["  voltage : Volt"])),
            "attribute should be preserved in owner node compartments"
        );
        assert_eq!(
            owner.attributes.get("generalViewPorts"),
            Some(&serde_json::json!(["  powerIn : PowerPort"])),
            "port should be preserved in owner node compartments"
        );
        assert!(
            canonical.edges.is_empty(),
            "contains edges to inlined details should be removed from General View"
        );
    }

    #[test]
    fn canonical_general_view_graph_filters_parameter_nodes() {
        let graph = SysmlGraphDto {
            nodes: vec![
                GraphNodeDto {
                    id: "Pkg::Operate".to_string(),
                    element_type: "action def".to_string(),
                    name: "Operate".to_string(),
                    parent_id: None,
                    range: range(),
                    attributes: Default::default(),
                },
                GraphNodeDto {
                    id: "Pkg::Operate::p".to_string(),
                    element_type: "in out parameter".to_string(),
                    name: "p".to_string(),
                    parent_id: Some("Pkg::Operate".to_string()),
                    range: range(),
                    attributes: serde_json::json!({ "parameterType": "Signal" })
                        .as_object()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                },
            ],
            edges: vec![GraphEdgeDto {
                source: "Pkg::Operate".to_string(),
                target: "Pkg::Operate::p".to_string(),
                rel_type: "contains".to_string(),
                name: None,
            }],
        };

        let canonical = canonical_general_view_graph(&graph, false);
        assert_eq!(canonical.nodes.len(), 1, "parameter nodes should be filtered from General View");
        assert!(
            canonical.nodes.iter().all(|node| !node.element_type.to_lowercase().contains("parameter")),
            "parameter nodes should not remain in generalViewGraph"
        );
        assert!(
            canonical.edges.is_empty(),
            "contains edges to filtered parameter nodes should be removed too"
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
