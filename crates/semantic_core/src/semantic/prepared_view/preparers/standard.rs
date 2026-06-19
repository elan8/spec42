//! Browser, grid, and geometry prepared views.

use std::collections::{HashMap, HashSet};

use serde_json::{json, Value};

use crate::semantic::dto::{
    GraphEdgeDto, GraphNodeDto, SysmlGraphDto, SysmlVisualizationProjectionHintsDto,
    SysmlVisualizationResultDto,
};
use crate::semantic::prepared_view::dto::{PreparedEdgeDto, PreparedNodeDto, PreparedViewDto};

fn graph_for_standard_view(response: &SysmlVisualizationResultDto) -> SysmlGraphDto {
    response
        .general_view_graph
        .clone()
        .or_else(|| response.graph.clone())
        .unwrap_or(SysmlGraphDto {
            nodes: Vec::new(),
            edges: Vec::new(),
        })
}

fn qualified_name_of(node: &GraphNodeDto) -> String {
    node.id.clone()
}

fn traceability_link_count(node_id: &str, edges: &[GraphEdgeDto]) -> usize {
    edges
        .iter()
        .filter(|edge| {
            let rel = edge.rel_type.to_lowercase();
            (rel.contains("satisfy")
                || rel.contains("derivation")
                || rel.contains("derive")
                || rel.contains("verify")
                || rel.contains("subject"))
                && (edge.source == node_id || edge.target == node_id)
        })
        .count()
}

fn package_label_of(qualified_name: &str) -> String {
    let segments: Vec<&str> = qualified_name.split("::").filter(|s| !s.is_empty()).collect();
    if segments.len() > 1 {
        segments[0].to_string()
    } else {
        String::new()
    }
}

fn projection_hints(response: &SysmlVisualizationResultDto) -> Option<&SysmlVisualizationProjectionHintsDto> {
    response.projection_hints.as_ref()
}

fn browser_layout_hint(response: &SysmlVisualizationResultDto) -> Option<&str> {
    projection_hints(response)?
        .browser_layout
        .as_deref()
}

fn grid_layout_hint(response: &SysmlVisualizationResultDto) -> Option<&str> {
    projection_hints(response)?.grid_layout.as_deref()
}

fn grid_subtype_hint(response: &SysmlVisualizationResultDto) -> Option<&str> {
    projection_hints(response)?.grid_subtype.as_deref()
}

fn tree_root_hints(response: &SysmlVisualizationResultDto) -> Vec<String> {
    projection_hints(response)
        .map(|hints| hints.tree_roots.clone())
        .unwrap_or_default()
}

#[derive(Clone)]
struct BrowserRow {
    id: String,
    label: String,
    kind: String,
    parent_id: String,
    qualified_name: String,
    uri: Option<String>,
    range: Option<crate::semantic::dto::RangeDto>,
    depth: usize,
    has_children: bool,
}

fn build_hierarchy_rows(graph_nodes: &[GraphNodeDto], tree_roots: &[String]) -> Vec<BrowserRow> {
    let mut nodes: Vec<BrowserRow> = graph_nodes
        .iter()
        .map(|node| BrowserRow {
            id: node.id.clone(),
            label: if node.name.is_empty() {
                node.id.clone()
            } else {
                node.name.clone()
            },
            kind: node.element_type.clone(),
            parent_id: node.parent_id.clone().unwrap_or_default(),
            qualified_name: qualified_name_of(node),
            uri: node.uri.clone(),
            range: Some(node.range.clone()),
            depth: 0,
            has_children: false,
        })
        .collect();
    let by_id: HashMap<String, usize> = nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.id.clone(), index))
        .collect();
    let mut children_by_parent: HashMap<String, Vec<usize>> = HashMap::new();
    for (index, node) in nodes.iter().enumerate() {
        if node.parent_id.is_empty() || !by_id.contains_key(&node.parent_id) {
            continue;
        }
        children_by_parent
            .entry(node.parent_id.clone())
            .or_default()
            .push(index);
    }
    for indices in children_by_parent.values_mut() {
        indices.sort_by(|left, right| {
            nodes[*left]
                .qualified_name
                .cmp(&nodes[*right].qualified_name)
        });
    }
    let root_indices: Vec<usize> = if !tree_roots.is_empty() {
        tree_roots
            .iter()
            .filter_map(|id| by_id.get(id).copied())
            .collect()
    } else {
        nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| {
                node.parent_id.is_empty() || !by_id.contains_key(&node.parent_id)
            })
            .map(|(index, _)| index)
            .collect()
    };
    let mut rows = Vec::new();
    fn visit(nodes: &mut [BrowserRow], children_by_parent: &HashMap<String, Vec<usize>>, index: usize, depth: usize, rows: &mut Vec<BrowserRow>) {
        let children = children_by_parent
            .get(&nodes[index].id)
            .cloned()
            .unwrap_or_default();
        nodes[index].depth = depth;
        nodes[index].has_children = !children.is_empty();
        rows.push(nodes[index].clone());
        for child_index in children {
            visit(nodes, children_by_parent, child_index, depth + 1, rows);
        }
    }
    let mut sorted_roots = root_indices;
    sorted_roots.sort_by(|left, right| {
        nodes[*left].qualified_name.cmp(&nodes[*right].qualified_name)
    });
    for root_index in sorted_roots {
        visit(&mut nodes, &children_by_parent, root_index, 0, &mut rows);
    }
    rows
}

pub fn prepare_browser_prepared_view(response: &SysmlVisualizationResultDto) -> PreparedViewDto {
    let graph = graph_for_standard_view(response);
    let hierarchy_layout = browser_layout_hint(response) == Some("hierarchy");
    let rows = if hierarchy_layout {
        build_hierarchy_rows(&graph.nodes, &tree_root_hints(response))
    } else {
        let mut flat: Vec<BrowserRow> = graph
            .nodes
            .iter()
            .map(|node| BrowserRow {
                id: node.id.clone(),
                label: if node.name.is_empty() {
                    node.id.clone()
                } else {
                    node.name.clone()
                },
                kind: node.element_type.clone(),
                parent_id: node.parent_id.clone().unwrap_or_default(),
                qualified_name: qualified_name_of(node),
                uri: node.uri.clone(),
                range: Some(node.range.clone()),
                depth: 0,
                has_children: false,
            })
            .collect();
        flat.sort_by(|left, right| left.qualified_name.cmp(&right.qualified_name));
        flat
    };
    let nodes: Vec<PreparedNodeDto> = rows
        .iter()
        .enumerate()
        .map(|(index, row)| PreparedNodeDto {
            id: if row.id.is_empty() {
                format!("browser-row-{index}")
            } else {
                row.id.clone()
            },
            label: row.label.clone(),
            kind: row.kind.clone(),
            source_path: None,
            uri: row.uri.clone(),
            range: row.range.clone(),
            attributes: Some(json!({
                "id": row.id,
                "label": row.label,
                "kind": row.kind,
                "parentId": row.parent_id,
                "qualifiedName": row.qualified_name,
                "uri": row.uri,
                "range": row.range,
                "depth": row.depth,
                "hasChildren": row.has_children,
            })),
        })
        .collect();
    PreparedViewDto {
        title: response
            .selected_view_name
            .clone()
            .unwrap_or_else(|| "Browser View".to_string()),
        view: "browser-view".to_string(),
        nodes,
        edges: Vec::new(),
        meta: Some(json!({
            "rows": rows.iter().map(|row| json!({
                "id": row.id,
                "label": row.label,
                "kind": row.kind,
                "parentId": row.parent_id,
                "qualifiedName": row.qualified_name,
                "uri": row.uri,
                "range": row.range,
                "depth": row.depth,
                "hasChildren": row.has_children,
            })).collect::<Vec<_>>(),
            "hierarchyLayout": hierarchy_layout,
            "provisional": !hierarchy_layout,
        })),
    }
}

fn build_relationship_matrix(
    node_ids: &[String],
    graph_edges: &[GraphEdgeDto],
) -> Vec<Value> {
    let mut edge_by_pair: HashMap<String, String> = HashMap::new();
    for edge in graph_edges {
        let label = edge.name.clone().unwrap_or_else(|| edge.rel_type.clone());
        edge_by_pair.insert(format!("{}::{}", edge.source, edge.target), label);
    }
    let mut cells = Vec::new();
    for source in node_ids {
        for target in node_ids {
            let label = edge_by_pair
                .get(&format!("{source}::{target}"))
                .cloned()
                .unwrap_or_default();
            cells.push(json!({
                "source": source,
                "target": target,
                "present": !label.is_empty(),
                "label": label,
            }));
        }
    }
    cells
}

pub fn prepare_grid_prepared_view(response: &SysmlVisualizationResultDto) -> PreparedViewDto {
    let graph = graph_for_standard_view(response);
    let traceability_layout = grid_layout_hint(response) == Some("traceability");
    let relationship_matrix = grid_subtype_hint(response) == Some("relationship_matrix");
    let mut cells: Vec<Value> = graph
        .nodes
        .iter()
        .map(|node| {
            let qualified_name = qualified_name_of(node);
            let link_count = traceability_link_count(&node.id, &graph.edges);
            json!({
                "id": node.id,
                "name": if node.name.is_empty() { &node.id } else { &node.name },
                "kind": node.element_type,
                "package": package_label_of(&qualified_name),
                "qualifiedName": qualified_name,
                "linkCount": link_count,
                "attributeCount": node.attributes.get("attributes").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0),
                "partCount": node.attributes.get("parts").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0),
                "portCount": node.attributes.get("ports").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0),
                "uri": node.uri,
                "range": node.range,
            })
        })
        .collect();
    cells.sort_by(|left, right| {
        left.get("qualifiedName")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .cmp(
                right
                    .get("qualifiedName")
                    .and_then(|v| v.as_str())
                    .unwrap_or(""),
            )
    });
    let node_ids: Vec<String> = cells
        .iter()
        .filter_map(|cell| cell.get("id").and_then(|v| v.as_str()).map(str::to_string))
        .collect();
    let matrix_cells = if relationship_matrix {
        build_relationship_matrix(&node_ids, &graph.edges)
    } else {
        Vec::new()
    };
    let nodes: Vec<PreparedNodeDto> = cells
        .iter()
        .enumerate()
        .map(|(index, cell)| PreparedNodeDto {
            id: cell
                .get("id")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .unwrap_or(&format!("grid-row-{index}"))
                .to_string(),
            label: cell
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("Unnamed")
                .to_string(),
            kind: cell
                .get("kind")
                .and_then(|v| v.as_str())
                .unwrap_or("element")
                .to_string(),
            source_path: None,
            uri: cell
                .get("uri")
                .and_then(|v| v.as_str())
                .map(str::to_string),
            range: None,
            attributes: Some(cell.clone()),
        })
        .collect();
    PreparedViewDto {
        title: response
            .selected_view_name
            .clone()
            .unwrap_or_else(|| "Grid View".to_string()),
        view: "grid-view".to_string(),
        nodes,
        edges: Vec::new(),
        meta: Some(json!({
            "cells": cells,
            "traceabilityTable": traceability_layout,
            "relationshipMatrix": relationship_matrix,
            "matrixRowIds": if relationship_matrix { node_ids.clone() } else { Vec::<String>::new() },
            "matrixColIds": if relationship_matrix { node_ids } else { Vec::<String>::new() },
            "matrixCells": matrix_cells,
            "provisional": !relationship_matrix && !traceability_layout,
        })),
    }
}

pub fn prepare_geometry_prepared_view(response: &SysmlVisualizationResultDto) -> PreparedViewDto {
    let graph = graph_for_standard_view(response);
    let hints = projection_hints(response);
    let elements: Vec<Value> = graph
        .nodes
        .iter()
        .map(|node| {
            json!({
                "id": node.id,
                "label": if node.name.is_empty() { &node.id } else { &node.name },
                "kind": node.element_type,
                "qualifiedName": qualified_name_of(node),
                "uri": node.uri,
                "range": node.range,
            })
        })
        .collect();
    let node_ids: HashSet<String> = elements
        .iter()
        .filter_map(|el| el.get("id").and_then(|v| v.as_str()).map(str::to_string))
        .collect();
    let nodes: Vec<PreparedNodeDto> = elements
        .iter()
        .enumerate()
        .map(|(index, element)| PreparedNodeDto {
            id: element
                .get("id")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .unwrap_or(&format!("geometry-node-{index}"))
                .to_string(),
            label: element
                .get("label")
                .and_then(|v| v.as_str())
                .unwrap_or("Unnamed")
                .to_string(),
            kind: element
                .get("kind")
                .and_then(|v| v.as_str())
                .unwrap_or("element")
                .to_string(),
            source_path: None,
            uri: element
                .get("uri")
                .and_then(|v| v.as_str())
                .map(str::to_string),
            range: None,
            attributes: Some(element.clone()),
        })
        .collect();
    let edges: Vec<PreparedEdgeDto> = graph
        .edges
        .iter()
        .enumerate()
        .filter(|(_, edge)| node_ids.contains(&edge.source) && node_ids.contains(&edge.target))
        .map(|(index, edge)| PreparedEdgeDto {
            id: format!("geometry-edge-{index}"),
            source: edge.source.clone(),
            target: edge.target.clone(),
            label: edge
                .name
                .clone()
                .unwrap_or_else(|| edge.rel_type.clone()),
            edge_kind: None,
            attributes: None,
        })
        .collect();
    PreparedViewDto {
        title: response
            .selected_view_name
            .clone()
            .unwrap_or_else(|| "Geometry View".to_string()),
        view: "geometry-view".to_string(),
        nodes,
        edges,
        meta: Some(json!({
            "elements": elements,
            "geometryMode": hints.and_then(|h| h.geometry_mode.clone()).unwrap_or_else(|| "2d".to_string()),
            "geometryProjection": hints.and_then(|h| h.geometry_projection.clone()).unwrap_or_else(|| "orthographic".to_string()),
            "provisional": true,
        })),
    }
}
