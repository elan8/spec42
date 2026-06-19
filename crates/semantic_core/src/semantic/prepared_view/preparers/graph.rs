//! General and graph-backed diagram prepared views.

use std::collections::{HashMap, HashSet};

use serde_json::{json, Value};

use crate::semantic::dto::{GraphEdgeDto, GraphNodeDto, SysmlGraphDto, SysmlVisualizationResultDto};
use crate::semantic::prepared_view::dto::{PreparedEdgeDto, PreparedNodeDto, PreparedViewDto};
use crate::semantic::prepared_view::graph_norm::{
    is_definition_kind, is_overview_visual_element_type, is_reference_kind, normalize_edge_kind,
};

fn is_synthetic_package(node: &GraphNodeDto) -> bool {
    node.attributes
        .get("syntheticPackage")
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
}

fn build_package_container_groups(nodes: &[PreparedNodeDto]) -> Vec<Value> {
    let mut by_package: HashMap<String, Vec<String>> = HashMap::new();
    for node in nodes {
        let Some(attrs) = node.attributes.as_ref() else {
            continue;
        };
        let Some(qn) = attrs.get("qualifiedName").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some((pkg, _)) = qn.split_once("::") else {
            continue;
        };
        by_package
            .entry(pkg.to_string())
            .or_default()
            .push(node.id.clone());
    }
    if by_package.len() < 2 {
        return Vec::new();
    }
    by_package
        .into_iter()
        .map(|(name, member_ids)| {
            json!({
                "id": format!("package:{name}"),
                "name": name,
                "memberIds": member_ids,
            })
        })
        .collect()
}

pub fn prepare_graph_prepared_view(response: &SysmlVisualizationResultDto) -> PreparedViewDto {
    let graph = response
        .general_view_graph
        .as_ref()
        .or(response.graph.as_ref())
        .cloned()
        .unwrap_or(SysmlGraphDto {
            nodes: Vec::new(),
            edges: Vec::new(),
        });
    prepare_graph_from_dto(&graph, response)
}

pub fn prepare_graph_from_dto(
    graph: &SysmlGraphDto,
    response: &SysmlVisualizationResultDto,
) -> PreparedViewDto {
    let source_nodes: Vec<&GraphNodeDto> = graph
        .nodes
        .iter()
        .filter(|node| !is_synthetic_package(node) && is_overview_visual_element_type(&node.element_type))
        .collect();
    let node_ids: HashSet<String> = source_nodes.iter().map(|node| node.id.clone()).collect();
    let nodes: Vec<PreparedNodeDto> = source_nodes
        .iter()
        .map(|node| PreparedNodeDto {
            id: node.id.clone(),
            label: if node.name.is_empty() {
                node.id.clone()
            } else {
                node.name.clone()
            },
            kind: node.element_type.clone(),
            source_path: node
                .attributes
                .get("sourcePath")
                .or_else(|| node.attributes.get("source_path"))
                .and_then(|v| v.as_str())
                .map(str::to_string),
            uri: node.uri.clone(),
            range: Some(node.range.clone()),
            attributes: Some(json!({
                "qualifiedName": node.id,
                "isPackage": node.element_type.to_lowercase().contains("package"),
                "isDefinition": is_definition_kind(&node.element_type),
                "isReference": is_reference_kind(&node.element_type),
            })),
        })
        .collect();
    let edges: Vec<PreparedEdgeDto> = graph
        .edges
        .iter()
        .filter(|edge| node_ids.contains(&edge.source) && node_ids.contains(&edge.target))
        .enumerate()
        .map(|(index, edge)| graph_edge_to_prepared(edge, index))
        .collect();
    let package_container_groups = build_package_container_groups(&nodes);
    let title = response
        .selected_view_name
        .clone()
        .unwrap_or_else(|| "SysML View".to_string());
    PreparedViewDto {
        title,
        view: response.view.clone(),
        nodes,
        edges,
        meta: if package_container_groups.is_empty() {
            None
        } else {
            Some(json!({ "packageContainerGroups": package_container_groups }))
        },
    }
}

fn graph_edge_to_prepared(edge: &GraphEdgeDto, index: usize) -> PreparedEdgeDto {
    let rel_type = edge.rel_type.clone();
    let label = edge
        .name
        .clone()
        .unwrap_or_else(|| rel_type.clone());
    PreparedEdgeDto {
        id: format!("edge-{index}"),
        source: edge.source.clone(),
        target: edge.target.clone(),
        label,
        edge_kind: Some(normalize_edge_kind(&rel_type)),
        attributes: Some(json!({ "relationType": normalize_edge_kind(&rel_type) })),
    }
}
