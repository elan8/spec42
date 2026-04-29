#![allow(clippy::items_after_test_module)]

use std::collections::{HashMap, HashSet};

use serde_json::{json, Value};
use tower_lsp::lsp_types::Url;

use crate::semantic;
use crate::views::dto::{range_to_dto, GraphEdgeDto, GraphNodeDto, SysmlGraphDto};

pub fn canonical_general_view_graph(
    graph: &SysmlGraphDto,
    _include_all_roots: bool,
) -> SysmlGraphDto {
    let filtered_graph = fold_general_view_leaf_details_into_owners(graph);

    let mut node_by_id: HashMap<String, GraphNodeDto> = HashMap::new();
    for node in &filtered_graph.nodes {
        node_by_id
            .entry(node.id.clone())
            .or_insert_with(|| node.clone());
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
    SysmlGraphDto {
        nodes: out_nodes,
        edges: out_edges,
    }
}

fn fold_general_view_leaf_details_into_owners(graph: &SysmlGraphDto) -> SysmlGraphDto {
    let node_by_id: HashMap<&str, &GraphNodeDto> = graph
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect();
    let children_by_parent: HashMap<&str, Vec<&GraphNodeDto>> = {
        let mut map: HashMap<&str, Vec<&GraphNodeDto>> = HashMap::new();
        for node in &graph.nodes {
            if let Some(parent_id) = node.parent_id.as_deref() {
                map.entry(parent_id).or_default().push(node);
            }
        }
        map
    };
    let typing_targets: HashMap<&str, &str> = graph
        .edges
        .iter()
        .filter(|edge| edge.rel_type.eq_ignore_ascii_case("typing"))
        .map(|edge| (edge.source.as_str(), edge.target.as_str()))
        .collect();
    let specialization_targets: HashMap<&str, Vec<&str>> = {
        let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
        for edge in &graph.edges {
            if edge.rel_type.eq_ignore_ascii_case("specializes") {
                map.entry(edge.source.as_str())
                    .or_default()
                    .push(edge.target.as_str());
            }
        }
        map
    };

    let detail_ids: HashSet<&str> = graph
        .nodes
        .iter()
        .filter(|node| is_general_view_inline_detail(node))
        .map(|node| node.id.as_str())
        .collect();

    let has_requirement_inline_content = graph.nodes.iter().any(|node| {
        node.attributes
            .get("requirementConstraints")
            .and_then(|value| value.as_array())
            .is_some_and(|lines| !lines.is_empty())
    });

    if detail_ids.is_empty() && !has_requirement_inline_content {
        return graph.clone();
    }

    let mut out_nodes: Vec<GraphNodeDto> = graph
        .nodes
        .iter()
        .filter(|node| !detail_ids.contains(node.id.as_str()))
        .cloned()
        .collect();
    for node in &mut out_nodes {
        let direct_details = collect_direct_general_view_details(
            node.id.as_str(),
            &children_by_parent,
            &node_by_id,
            &typing_targets,
            node.attributes
                .get("requirementConstraints")
                .and_then(|value| value.as_array()),
        );
        let inherited_details = collect_inherited_general_view_details(
            node.id.as_str(),
            &children_by_parent,
            &node_by_id,
            &typing_targets,
            &specialization_targets,
            &direct_details,
        );

        insert_detail_items(
            &mut node.attributes,
            "generalViewDirectAttributes",
            direct_details.attributes,
        );
        insert_detail_items(
            &mut node.attributes,
            "generalViewDirectParts",
            direct_details.parts,
        );
        insert_detail_items(
            &mut node.attributes,
            "generalViewDirectPorts",
            direct_details.ports,
        );
        insert_detail_items(
            &mut node.attributes,
            "generalViewInheritedAttributes",
            inherited_details.attributes,
        );
        insert_detail_items(
            &mut node.attributes,
            "generalViewInheritedParts",
            inherited_details.parts,
        );
    }

    let out_edges: Vec<GraphEdgeDto> = graph
        .edges
        .iter()
        .filter(|edge| {
            !detail_ids.contains(edge.source.as_str()) && !detail_ids.contains(edge.target.as_str())
        })
        .cloned()
        .collect();

    SysmlGraphDto {
        nodes: out_nodes,
        edges: out_edges,
    }
}

fn is_general_view_inline_detail(node: &GraphNodeDto) -> bool {
    let lower = node.element_type.to_lowercase();
    is_port_like(&lower)
        || is_attribute_like(&lower)
        || is_parameter_like(&lower)
        // Anonymous redefinition stubs like `part :>> engines[5] = (...)` should not
        // surface as standalone structure nodes in General View.
        || is_anonymous_redefinition_stub(node)
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

fn is_anonymous_redefinition_stub(node: &GraphNodeDto) -> bool {
    node.name.trim().is_empty() && node.attributes.contains_key("redefines")
}

#[derive(Default)]
struct GeneralViewDetails {
    attributes: Vec<Value>,
    parts: Vec<Value>,
    ports: Vec<Value>,
    attribute_names: HashSet<String>,
    part_names: HashSet<String>,
}

fn collect_direct_general_view_details(
    owner_id: &str,
    children_by_parent: &HashMap<&str, Vec<&GraphNodeDto>>,
    node_by_id: &HashMap<&str, &GraphNodeDto>,
    typing_targets: &HashMap<&str, &str>,
    requirement_constraints: Option<&Vec<Value>>,
) -> GeneralViewDetails {
    let mut details = GeneralViewDetails::default();

    if let Some(children) = children_by_parent.get(owner_id) {
        for child in children {
            if is_port_like(&child.element_type) {
                if let Some(item) =
                    build_general_view_detail_item(child, node_by_id, typing_targets, None)
                {
                    details.ports.push(item);
                }
                continue;
            }
            if is_attribute_like(&child.element_type) {
                if let Some(item) =
                    build_general_view_detail_item(child, node_by_id, typing_targets, None)
                {
                    if let Some(name) = item.get("name").and_then(|value| value.as_str()) {
                        details.attribute_names.insert(name.to_lowercase());
                    }
                    details.attributes.push(item);
                }
                continue;
            }
            if is_part_compartment_item(child) {
                if let Some(item) =
                    build_general_view_detail_item(child, node_by_id, typing_targets, None)
                {
                    if let Some(name) = item.get("name").and_then(|value| value.as_str()) {
                        details.part_names.insert(name.to_lowercase());
                    }
                    details.parts.push(item);
                }
            }
        }
    }

    if let Some(lines) = requirement_constraints {
        for line in lines {
            if let Some(display_text) = line.as_str().map(|value| value.trim().to_string()) {
                if display_text.is_empty() {
                    continue;
                }
                details.attributes.push(json!({
                    "name": display_text,
                    "displayText": display_text,
                }));
            }
        }
    }

    details
}

fn collect_inherited_general_view_details(
    owner_id: &str,
    children_by_parent: &HashMap<&str, Vec<&GraphNodeDto>>,
    node_by_id: &HashMap<&str, &GraphNodeDto>,
    typing_targets: &HashMap<&str, &str>,
    specialization_targets: &HashMap<&str, Vec<&str>>,
    direct_details: &GeneralViewDetails,
) -> GeneralViewDetails {
    let mut details = GeneralViewDetails::default();
    let mut seen_ancestors: HashSet<&str> = HashSet::new();
    let mut queue: Vec<&str> = specialization_targets
        .get(owner_id)
        .cloned()
        .unwrap_or_default();

    while let Some(ancestor_id) = queue.first().copied() {
        queue.remove(0);
        if !seen_ancestors.insert(ancestor_id) {
            continue;
        }
        if let Some(next_targets) = specialization_targets.get(ancestor_id) {
            queue.extend(next_targets.iter().copied());
        }
        let declared_in = node_by_id
            .get(ancestor_id)
            .map(|node| node.name.clone())
            .filter(|name| !name.trim().is_empty());
        let Some(children) = children_by_parent.get(ancestor_id) else {
            continue;
        };
        for child in children {
            if is_attribute_like(&child.element_type) {
                if let Some(item) = build_general_view_detail_item(
                    child,
                    node_by_id,
                    typing_targets,
                    declared_in.clone(),
                ) {
                    let Some(name) = item.get("name").and_then(|value| value.as_str()) else {
                        continue;
                    };
                    let normalized = name.to_lowercase();
                    if direct_details.attribute_names.contains(&normalized)
                        || !details.attribute_names.insert(normalized)
                    {
                        continue;
                    }
                    details.attributes.push(item);
                }
                continue;
            }
            if is_part_compartment_item(child) {
                if let Some(item) = build_general_view_detail_item(
                    child,
                    node_by_id,
                    typing_targets,
                    declared_in.clone(),
                ) {
                    let Some(name) = item.get("name").and_then(|value| value.as_str()) else {
                        continue;
                    };
                    let normalized = name.to_lowercase();
                    if direct_details.part_names.contains(&normalized)
                        || !details.part_names.insert(normalized)
                    {
                        continue;
                    }
                    details.parts.push(item);
                }
            }
        }
    }

    details
}

fn is_part_compartment_item(node: &GraphNodeDto) -> bool {
    let lower = node.element_type.to_lowercase();
    if !lower.contains("part") || is_port_like(&lower) {
        return false;
    }
    if node.name.trim().is_empty() {
        return node.attributes.contains_key("redefines");
    }
    true
}

fn build_general_view_detail_item(
    detail: &GraphNodeDto,
    node_by_id: &HashMap<&str, &GraphNodeDto>,
    typing_targets: &HashMap<&str, &str>,
    declared_in: Option<String>,
) -> Option<Value> {
    let name = general_view_detail_name(detail)?;
    let type_name = detail_type_name(detail, node_by_id, typing_targets);
    let value_text = detail_value_text(detail);
    let display_text =
        format_general_view_detail_display_text(&name, type_name.as_deref(), value_text.as_deref());

    Some(json!({
        "name": name,
        "typeName": type_name,
        "valueText": value_text,
        "declaredIn": declared_in,
        "displayText": display_text,
    }))
}

fn general_view_detail_name(detail: &GraphNodeDto) -> Option<String> {
    let explicit_name = detail.name.trim();
    if !explicit_name.is_empty() {
        return Some(explicit_name.to_string());
    }
    if is_attribute_like(&detail.element_type) || is_part_compartment_item(detail) {
        return detail
            .attributes
            .get("redefines")
            .and_then(|value| value.as_str())
            .map(|value| value.split("::").last().unwrap_or(value).to_string());
    }
    None
}

fn detail_type_name(
    detail: &GraphNodeDto,
    node_by_id: &HashMap<&str, &GraphNodeDto>,
    typing_targets: &HashMap<&str, &str>,
) -> Option<String> {
    typing_targets
        .get(detail.id.as_str())
        .and_then(|target_id| node_by_id.get(target_id))
        .map(|target| target.name.as_str())
        .or_else(|| {
            if is_port_like(&detail.element_type) {
                detail
                    .attributes
                    .get("portType")
                    .and_then(|value| value.as_str())
            } else if is_part_compartment_item(detail) {
                detail
                    .attributes
                    .get("partType")
                    .and_then(|value| value.as_str())
                    .or_else(|| {
                        detail
                            .attributes
                            .get("type")
                            .and_then(|value| value.as_str())
                    })
            } else {
                detail
                    .attributes
                    .get("dataType")
                    .and_then(|value| value.as_str())
                    .or_else(|| {
                        detail
                            .attributes
                            .get("attributeType")
                            .and_then(|value| value.as_str())
                    })
                    .or_else(|| {
                        detail
                            .attributes
                            .get("type")
                            .and_then(|value| value.as_str())
                    })
                    .or_else(|| {
                        detail
                            .attributes
                            .get("parameterType")
                            .and_then(|value| value.as_str())
                    })
            }
        })
        .map(|type_name| {
            type_name
                .split("::")
                .last()
                .unwrap_or(type_name)
                .to_string()
        })
        .filter(|type_name| !type_name.trim().is_empty())
}

fn detail_value_text(detail: &GraphNodeDto) -> Option<String> {
    for key in ["value", "defaultValue", "valueText", "literal"] {
        let Some(value) = detail.attributes.get(key) else {
            continue;
        };
        if let Some(raw) = value.as_str() {
            if !raw.trim().is_empty() {
                return Some(raw.trim().to_string());
            }
        } else if value.is_number() || value.is_boolean() {
            return Some(value.to_string());
        }
    }
    None
}

fn format_general_view_detail_display_text(
    name: &str,
    type_name: Option<&str>,
    value_text: Option<&str>,
) -> String {
    match (type_name, value_text) {
        (Some(type_name), Some(value_text)) if !type_name.is_empty() && !value_text.is_empty() => {
            format!("{name} : {type_name} = {value_text}")
        }
        (Some(type_name), _) if !type_name.is_empty() => format!("{name} : {type_name}"),
        (_, Some(value_text)) if !value_text.is_empty() => format!("{name} = {value_text}"),
        _ => name.to_string(),
    }
}

fn insert_detail_items(attributes: &mut HashMap<String, Value>, key: &str, items: Vec<Value>) {
    if items.is_empty() {
        attributes.remove(key);
    } else {
        attributes.insert(key.to_string(), Value::Array(items));
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
                    uri: None,
                    parent_id: None,
                    range: range(),
                    attributes: Default::default(),
                },
                GraphNodeDto {
                    id: "Pkg::Req".to_string(),
                    element_type: "requirement def".to_string(),
                    name: "Req".to_string(),
                    uri: None,
                    parent_id: None,
                    range: range(),
                    attributes: Default::default(),
                },
                GraphNodeDto {
                    id: "Pkg::Root".to_string(),
                    element_type: "part def".to_string(),
                    name: "Root".to_string(),
                    uri: None,
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
            canonical.edges.iter().any(|edge| edge.rel_type == "subject"
                && edge.source == "Pkg::Req"
                && edge.target == "Pkg::Drone"),
            "subject edge should survive canonical General View projection"
        );
    }

    #[test]
    fn canonical_general_view_graph_preserves_requirement_constraints_as_inline_attributes() {
        let graph = SysmlGraphDto {
            nodes: vec![GraphNodeDto {
                id: "Pkg::Req".to_string(),
                element_type: "requirement def".to_string(),
                name: "Req".to_string(),
                uri: None,
                parent_id: None,
                range: range(),
                attributes: serde_json::json!({
                    "requirementConstraints": ["  flightTime >= 25 min."]
                })
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            }],
            edges: vec![],
        };

        let canonical = canonical_general_view_graph(&graph, false);
        let owner = canonical
            .nodes
            .iter()
            .find(|node| node.id == "Pkg::Req")
            .unwrap();
        assert_eq!(
            owner.attributes.get("generalViewDirectAttributes"),
            Some(&serde_json::json!([{
                "name": "flightTime >= 25 min.",
                "displayText": "flightTime >= 25 min."
            }])),
            "requirement constraints should be exposed through generalViewDirectAttributes"
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
                    uri: None,
                    parent_id: Some("Office".to_string()),
                    range: range(),
                    attributes: Default::default(),
                },
                GraphNodeDto {
                    id: "Office::my_computer::laptop".to_string(),
                    element_type: "part".to_string(),
                    name: "laptop".to_string(),
                    uri: None,
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
                    uri: None,
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
                    uri: None,
                    parent_id: None,
                    range: range(),
                    attributes: Default::default(),
                },
                GraphNodeDto {
                    id: "Pkg::Laptop::voltage".to_string(),
                    element_type: "attribute".to_string(),
                    name: "voltage".to_string(),
                    uri: None,
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
                    uri: None,
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
        assert_eq!(
            canonical.nodes.len(),
            1,
            "port and attribute nodes should be filtered from General View"
        );
        let owner = canonical
            .nodes
            .iter()
            .find(|node| node.id == "Pkg::Laptop")
            .expect("owner node");
        assert_eq!(
            owner.attributes.get("generalViewDirectAttributes"),
            Some(&serde_json::json!([{
                "name": "voltage",
                "typeName": "Volt",
                "valueText": null,
                "declaredIn": null,
                "displayText": "voltage : Volt"
            }])),
            "attribute should be preserved in owner node compartments"
        );
        assert_eq!(
            owner.attributes.get("generalViewDirectPorts"),
            Some(&serde_json::json!([{
                "name": "powerIn",
                "typeName": "PowerPort",
                "valueText": null,
                "declaredIn": null,
                "displayText": "powerIn : PowerPort"
            }])),
            "port should be preserved in owner node compartments"
        );
        assert!(
            canonical.edges.is_empty(),
            "contains edges to inlined details should be removed from General View"
        );
    }

    #[test]
    fn canonical_general_view_graph_groups_direct_and_inherited_member_details() {
        let graph = SysmlGraphDto {
            nodes: vec![
                GraphNodeDto {
                    id: "Pkg::Vehicle".to_string(),
                    element_type: "part def".to_string(),
                    name: "Vehicle".to_string(),
                    uri: None,
                    parent_id: None,
                    range: range(),
                    attributes: Default::default(),
                },
                GraphNodeDto {
                    id: "Pkg::Vehicle::mass".to_string(),
                    element_type: "attribute".to_string(),
                    name: "mass".to_string(),
                    uri: None,
                    parent_id: Some("Pkg::Vehicle".to_string()),
                    range: range(),
                    attributes:
                        serde_json::json!({ "dataType": "ScalarValues::Kilogram", "value": "1200" })
                            .as_object()
                            .unwrap()
                            .iter()
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect(),
                },
                GraphNodeDto {
                    id: "Pkg::Vehicle::engine".to_string(),
                    element_type: "part".to_string(),
                    name: "engine".to_string(),
                    uri: None,
                    parent_id: Some("Pkg::Vehicle".to_string()),
                    range: range(),
                    attributes: serde_json::json!({ "type": "Engine" })
                        .as_object()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                },
                GraphNodeDto {
                    id: "Pkg::Car".to_string(),
                    element_type: "part def".to_string(),
                    name: "Car".to_string(),
                    uri: None,
                    parent_id: None,
                    range: range(),
                    attributes: Default::default(),
                },
                GraphNodeDto {
                    id: "Pkg::Car::wheels".to_string(),
                    element_type: "part".to_string(),
                    name: "wheels".to_string(),
                    uri: None,
                    parent_id: Some("Pkg::Car".to_string()),
                    range: range(),
                    attributes: serde_json::json!({ "type": "WheelSet" })
                        .as_object()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                },
                GraphNodeDto {
                    id: "Pkg::Car::mass".to_string(),
                    element_type: "attribute".to_string(),
                    name: "mass".to_string(),
                    uri: None,
                    parent_id: Some("Pkg::Car".to_string()),
                    range: range(),
                    attributes:
                        serde_json::json!({ "dataType": "ScalarValues::Kilogram", "value": "1300" })
                            .as_object()
                            .unwrap()
                            .iter()
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect(),
                },
            ],
            edges: vec![GraphEdgeDto {
                source: "Pkg::Car".to_string(),
                target: "Pkg::Vehicle".to_string(),
                rel_type: "specializes".to_string(),
                name: None,
            }],
        };

        let canonical = canonical_general_view_graph(&graph, false);
        let owner = canonical
            .nodes
            .iter()
            .find(|node| node.id == "Pkg::Car")
            .expect("owner node");
        assert_eq!(
            owner.attributes.get("generalViewDirectAttributes"),
            Some(&serde_json::json!([{
                "name": "mass",
                "typeName": "Kilogram",
                "valueText": "1300",
                "declaredIn": null,
                "displayText": "mass : Kilogram = 1300"
            }])),
            "direct attributes should preserve type and value display text"
        );
        assert_eq!(
            owner.attributes.get("generalViewDirectParts"),
            Some(&serde_json::json!([{
                "name": "wheels",
                "typeName": "WheelSet",
                "valueText": null,
                "declaredIn": null,
                "displayText": "wheels : WheelSet"
            }])),
            "direct parts should remain in the owner node payload"
        );
        assert_eq!(
            owner.attributes.get("generalViewInheritedAttributes"),
            None,
            "redefined direct attributes should suppress inherited duplicates instead of emitting an empty compartment"
        );
        assert_eq!(
            owner.attributes.get("generalViewInheritedParts"),
            Some(&serde_json::json!([{
                "name": "engine",
                "typeName": "Engine",
                "valueText": null,
                "declaredIn": "Vehicle",
                "displayText": "engine : Engine"
            }])),
            "inherited parts should be grouped separately with provenance"
        );
    }

    #[test]
    fn canonical_general_view_graph_moves_redefined_parts_into_direct_parts() {
        let graph = SysmlGraphDto {
            nodes: vec![
                GraphNodeDto {
                    id: "Pkg::Stage".to_string(),
                    element_type: "part def".to_string(),
                    name: "Stage".to_string(),
                    uri: None,
                    parent_id: None,
                    range: range(),
                    attributes: Default::default(),
                },
                GraphNodeDto {
                    id: "Pkg::Stage::engine".to_string(),
                    element_type: "part".to_string(),
                    name: "engine".to_string(),
                    uri: None,
                    parent_id: Some("Pkg::Stage".to_string()),
                    range: range(),
                    attributes: serde_json::json!({ "type": "Engine" })
                        .as_object()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                },
                GraphNodeDto {
                    id: "Pkg::SIC".to_string(),
                    element_type: "part def".to_string(),
                    name: "S-IC".to_string(),
                    uri: None,
                    parent_id: None,
                    range: range(),
                    attributes: Default::default(),
                },
                GraphNodeDto {
                    id: "Pkg::SIC::engine-redef".to_string(),
                    element_type: "part".to_string(),
                    name: "".to_string(),
                    uri: None,
                    parent_id: Some("Pkg::SIC".to_string()),
                    range: range(),
                    attributes: serde_json::json!({ "redefines": "Pkg::Stage::engine" })
                        .as_object()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                },
            ],
            edges: vec![GraphEdgeDto {
                source: "Pkg::SIC".to_string(),
                target: "Pkg::Stage".to_string(),
                rel_type: "specializes".to_string(),
                name: None,
            }],
        };

        let canonical = canonical_general_view_graph(&graph, false);
        let owner = canonical
            .nodes
            .iter()
            .find(|node| node.id == "Pkg::SIC")
            .expect("owner node");
        assert_eq!(
            owner.attributes.get("generalViewDirectParts"),
            Some(&serde_json::json!([{
                "name": "engine",
                "typeName": null,
                "valueText": null,
                "declaredIn": null,
                "displayText": "engine"
            }])),
            "redefined part rows should be surfaced as direct parts for the current owner"
        );
        assert_eq!(
            owner.attributes.get("generalViewInheritedParts"),
            None,
            "redefined inherited part should not stay in inherited compartment"
        );
    }

    #[test]
    fn canonical_general_view_graph_moves_redefined_attributes_into_direct_attributes() {
        let graph = SysmlGraphDto {
            nodes: vec![
                GraphNodeDto {
                    id: "Pkg::Stage".to_string(),
                    element_type: "part def".to_string(),
                    name: "Stage".to_string(),
                    uri: None,
                    parent_id: None,
                    range: range(),
                    attributes: Default::default(),
                },
                GraphNodeDto {
                    id: "Pkg::Stage::mass".to_string(),
                    element_type: "attribute".to_string(),
                    name: "mass".to_string(),
                    uri: None,
                    parent_id: Some("Pkg::Stage".to_string()),
                    range: range(),
                    attributes: serde_json::json!({ "dataType": "ScalarValues::Kilogram" })
                        .as_object()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                },
                GraphNodeDto {
                    id: "Pkg::SIC".to_string(),
                    element_type: "part def".to_string(),
                    name: "S-IC".to_string(),
                    uri: None,
                    parent_id: None,
                    range: range(),
                    attributes: Default::default(),
                },
                GraphNodeDto {
                    id: "Pkg::SIC::mass-redef".to_string(),
                    element_type: "attribute".to_string(),
                    name: "".to_string(),
                    uri: None,
                    parent_id: Some("Pkg::SIC".to_string()),
                    range: range(),
                    attributes: serde_json::json!({
                        "redefines": "Pkg::Stage::mass",
                        "dataType": "ScalarValues::Kilogram",
                        "value": "28500"
                    })
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
                },
            ],
            edges: vec![GraphEdgeDto {
                source: "Pkg::SIC".to_string(),
                target: "Pkg::Stage".to_string(),
                rel_type: "specializes".to_string(),
                name: None,
            }],
        };

        let canonical = canonical_general_view_graph(&graph, false);
        let owner = canonical
            .nodes
            .iter()
            .find(|node| node.id == "Pkg::SIC")
            .expect("owner node");
        assert_eq!(
            owner.attributes.get("generalViewDirectAttributes"),
            Some(&serde_json::json!([{
                "name": "mass",
                "typeName": "Kilogram",
                "valueText": "28500",
                "declaredIn": null,
                "displayText": "mass : Kilogram = 28500"
            }])),
            "redefined attribute rows should be surfaced as direct attributes for the current owner"
        );
        assert_eq!(
            owner.attributes.get("generalViewInheritedAttributes"),
            None,
            "redefined inherited attribute should not stay in inherited compartment"
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
                    uri: None,
                    parent_id: None,
                    range: range(),
                    attributes: Default::default(),
                },
                GraphNodeDto {
                    id: "Pkg::Operate::p".to_string(),
                    element_type: "in out parameter".to_string(),
                    name: "p".to_string(),
                    uri: None,
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
        assert_eq!(
            canonical.nodes.len(),
            1,
            "parameter nodes should be filtered from General View"
        );
        assert!(
            canonical
                .nodes
                .iter()
                .all(|node| !node.element_type.to_lowercase().contains("parameter")),
            "parameter nodes should not remain in generalViewGraph"
        );
        assert!(
            canonical.edges.is_empty(),
            "contains edges to filtered parameter nodes should be removed too"
        );
    }

    #[test]
    fn canonical_general_view_graph_filters_anonymous_redefinition_stubs() {
        let graph = SysmlGraphDto {
            nodes: vec![
                GraphNodeDto {
                    id: "Pkg::Vehicle".to_string(),
                    element_type: "part def".to_string(),
                    name: "Vehicle".to_string(),
                    uri: None,
                    parent_id: None,
                    range: range(),
                    attributes: Default::default(),
                },
                GraphNodeDto {
                    id: "Pkg::Vehicle::engines-redef".to_string(),
                    element_type: "part".to_string(),
                    name: "".to_string(),
                    uri: None,
                    parent_id: Some("Pkg::Vehicle".to_string()),
                    range: range(),
                    attributes: serde_json::json!({ "redefines": "engines" })
                        .as_object()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                },
            ],
            edges: vec![GraphEdgeDto {
                source: "Pkg::Vehicle".to_string(),
                target: "Pkg::Vehicle::engines-redef".to_string(),
                rel_type: "contains".to_string(),
                name: None,
            }],
        };

        let canonical = canonical_general_view_graph(&graph, false);
        assert_eq!(
            canonical.nodes.len(),
            1,
            "anonymous redefinition stubs should not remain in General View"
        );
        assert!(
            canonical
                .nodes
                .iter()
                .all(|node| node.id != "Pkg::Vehicle::engines-redef"),
            "redefinition stub should be filtered out"
        );
        assert!(
            canonical.edges.is_empty(),
            "contains edges to redefinition stubs should be removed too"
        );
    }
}

pub fn build_workspace_graph_dto(
    semantic_graph: &semantic::SemanticGraph,
    library_paths: &[Url],
) -> SysmlGraphDto {
    let sg_nodes = semantic_graph.workspace_nodes_excluding_libraries(library_paths);
    let nodes: Vec<GraphNodeDto> = sg_nodes
        .iter()
        .map(|n| GraphNodeDto {
            id: n.id.qualified_name.clone(),
            element_type: n.element_kind.clone(),
            name: n.name.clone(),
            uri: Some(n.id.uri.as_str().to_string()),
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

pub fn workspace_visualization_enabled(scope: &[String]) -> bool {
    scope.iter().any(|s| s == "workspaceVisualization")
}
