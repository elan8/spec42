//! Port of `render/layout.ts`, `views/behavior-common.ts` layout, and
//! `render/interconnection-elk-input.ts` onto `diagram_layout` (elkrs).

use std::collections::{BTreeMap, HashMap, HashSet};

use serde_json::{json, Map, Value};

use crate::graph_normalization::{
    is_connector_usage_element_type, is_overview_visual_element_type, normalize_edge_kind,
};
use crate::ibd_ports::{ibd_port_label_text, ibd_port_label_width};
use crate::ibd_route::{lca_offset_for_nodes, resolve_ibd_route_points};
use crate::json_util::{as_array, as_object, as_string, field};
use crate::sysml_node::{
    collect_compartments, compartments_to_value, compute_node_height, compute_node_width,
    node_chrome_state_from_attributes,
};
use crate::types::{attr_str, LaidOutEdge, LaidOutNode, Point, IBD_NODE_HEIGHT, IBD_NODE_WIDTH};

const NODE_WIDTH: f64 = 200.0;
const NODE_HEIGHT: f64 = 70.0;
const IBD_PORT_LABEL_HEIGHT: f64 = 10.0;

pub fn layout_to_draw_input(prepared: &Value) -> Result<Value, diagram_layout::LayoutError> {
    let view = as_string(field(prepared, "view"), "general-view");
    match view.as_str() {
        "sequence-view" | "browser-view" | "grid-view" | "geometry-view" => Ok(prepared.clone()),
        "action-flow-view" | "state-transition-view" => {
            let horizontal = as_string(field(field(prepared, "meta"), "layoutDirection"), "")
                .eq_ignore_ascii_case("horizontal");
            let mode = if view == "state-transition-view" {
                BehaviorMode::State
            } else {
                BehaviorMode::Action
            };
            let behavior_layout = layout_behavior_graph(prepared, horizontal, mode)?;
            Ok(json!({
                "prepared": prepared,
                "behaviorLayout": behavior_layout,
            }))
        }
        "interconnection-view" => layout_interconnection_prepared(prepared),
        _ => layout_general_prepared(prepared),
    }
}

#[derive(Clone, Copy)]
enum ElkViewKind {
    General,
    Interconnection,
    BehaviorState,
    BehaviorAction,
}

fn build_elk_layout_options(
    kind: ElkViewKind,
    overrides: &[(&str, Option<&str>)],
) -> Map<String, Value> {
    let mut merged = Map::new();
    let common = [
        ("elk.algorithm", "layered"),
        ("elk.edgeRouting", "ORTHOGONAL"),
        ("elk.layered.nodePlacement.strategy", "NETWORK_SIMPLEX"),
        ("elk.separateConnectedComponents", "true"),
        ("elk.json.edgeCoords", "ROOT"),
    ];
    let per_kind: &[(&str, &str)] = match kind {
        ElkViewKind::General => &[
            ("elk.direction", "DOWN"),
            ("elk.spacing.nodeNode", "140"),
            ("elk.layered.spacing.nodeNodeBetweenLayers", "180"),
            ("elk.spacing.edgeNode", "90"),
            ("elk.spacing.edgeEdge", "80"),
            ("elk.aspectRatio", "1.4"),
            ("elk.padding", "[top=100,left=100,bottom=100,right=100]"),
            ("org.eclipse.elk.portConstraints", "FIXED_SIDE"),
        ],
        ElkViewKind::Interconnection => &[
            ("elk.hierarchyHandling", "INCLUDE_CHILDREN"),
            ("elk.direction", "RIGHT"),
            ("elk.spacing.nodeNode", "150"),
            ("elk.layered.spacing.nodeNodeBetweenLayers", "220"),
            ("elk.spacing.edgeNode", "110"),
            ("elk.spacing.edgeEdge", "90"),
            ("elk.layered.crossingMinimization.strategy", "LAYER_SWEEP"),
            ("elk.padding", "[top=70,left=70,bottom=70,right=70]"),
            ("org.eclipse.elk.portConstraints", "FIXED_ORDER"),
            ("org.eclipse.elk.portAlignment.default", "CENTER"),
        ],
        ElkViewKind::BehaviorState => &[
            ("elk.hierarchyHandling", "INCLUDE_CHILDREN"),
            ("elk.layered.crossingMinimization.strategy", "LAYER_SWEEP"),
            ("elk.layered.spacing.nodeNodeBetweenLayers", "230"),
            ("elk.spacing.nodeNode", "190"),
            ("elk.spacing.edgeNode", "130"),
            ("elk.spacing.edgeEdge", "110"),
            ("elk.spacing.edgeLabel", "12"),
            ("elk.padding", "[top=100,left=90,bottom=90,right=90]"),
        ],
        ElkViewKind::BehaviorAction => &[
            ("elk.layered.crossingMinimization.strategy", "LAYER_SWEEP"),
            ("elk.spacing.edgeNode", "80"),
            ("elk.spacing.edgeEdge", "60"),
            ("elk.spacing.edgeLabel", "12"),
            ("elk.padding", "[top=80,left=80,bottom=80,right=80]"),
        ],
    };
    for (key, value) in common.iter().chain(per_kind.iter()) {
        merged.insert((*key).into(), Value::String((*value).into()));
    }
    for (key, value) in overrides {
        match value {
            None => {
                merged.remove(*key);
            }
            Some(value) => {
                merged.insert((*key).into(), Value::String((*value).into()));
            }
        }
    }
    merged
}

fn num(value: &Value) -> f64 {
    value.as_f64().unwrap_or(0.0)
}

fn node_attributes(node: &Value) -> std::collections::BTreeMap<String, Value> {
    field(node, "attributes")
        .as_object()
        .map(|map| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
        .unwrap_or_default()
}

fn general_node_box(node: &Value) -> (f64, f64, crate::sysml_node::Compartments) {
    let label = as_string(field(node, "label"), "");
    let kind = as_string(field(node, "kind"), "");
    let attributes = node_attributes(node);
    let compartments = collect_compartments(&label, &kind, &attributes);
    let state = node_chrome_state_from_attributes(&attributes);
    let width = compute_node_width(&compartments, &state);
    let height = compute_node_height(&compartments, width, &state);
    (width, height, compartments)
}

fn general_leaf_elk_node(node: &Value) -> Value {
    let (width, height, _) = general_node_box(node);
    json!({
        "id": field(node, "id"),
        "width": width,
        "height": height,
    })
}

fn chunk_general_elk_children(id_prefix: &str, elk_nodes: Vec<Value>) -> Vec<Value> {
    let wide_sibling_threshold = 8;
    if elk_nodes.len() <= wide_sibling_threshold {
        return elk_nodes;
    }
    let chunk_size = ((elk_nodes.len() as f64).sqrt() / 2.0).ceil().max(1.0) as usize;
    let mut chunks = Vec::new();
    let mut index = 0;
    while index < elk_nodes.len() {
        let end = (index + chunk_size).min(elk_nodes.len());
        chunks.push(json!({
            "id": format!("{id_prefix}#chunk{}", chunks.len()),
            "layoutOptions": {
                "elk.direction": "DOWN",
                "elk.padding": "[top=8,left=8,bottom=8,right=8]",
            },
            "children": elk_nodes[index..end].to_vec(),
        }));
        index = end;
    }
    chunks
}

fn general_diagram_elements(prepared: &Value) -> (Vec<Value>, Vec<Value>) {
    let represented_relationship_nodes: HashSet<String> = as_array(field(prepared, "edges"))
        .iter()
        .filter_map(|edge| {
            let origin = field(field(edge, "attributes"), "originNodeId")
                .as_str()
                .unwrap_or("");
            let kind = normalize_edge_kind(&as_string(
                first_nonempty([field(edge, "edgeKind"), field(edge, "label")]),
                "",
            ));
            if !origin.is_empty()
                && origin != as_string(field(edge, "source"), "")
                && origin != as_string(field(edge, "target"), "")
                && kind != "hierarchy"
            {
                Some(origin.to_string())
            } else {
                None
            }
        })
        .collect();
    let diagram_nodes: Vec<Value> = as_array(field(prepared, "nodes"))
        .iter()
        .filter(|node| {
            let kind = as_string(field(node, "kind"), "");
            is_overview_visual_element_type(&kind)
                && !is_connector_usage_element_type(&kind)
                && !represented_relationship_nodes.contains(&as_string(field(node, "id"), ""))
        })
        .cloned()
        .collect();
    let visible_ids: HashSet<String> = diagram_nodes
        .iter()
        .map(|node| as_string(field(node, "id"), ""))
        .collect();
    let diagram_edges: Vec<Value> = as_array(field(prepared, "edges"))
        .iter()
        .filter(|edge| {
            visible_ids.contains(&as_string(field(edge, "source"), ""))
                && visible_ids.contains(&as_string(field(edge, "target"), ""))
        })
        .cloned()
        .collect();
    (diagram_nodes, diagram_edges)
}

fn first_nonempty<'a>(values: impl IntoIterator<Item = &'a Value>) -> &'a Value {
    for value in values {
        if !as_string(value, "").is_empty() {
            return value;
        }
    }
    &Value::Null
}

fn compute_containment_parent(diagram_edges: &[Value]) -> HashMap<String, String> {
    diagram_edges
        .iter()
        .filter(|edge| {
            normalize_edge_kind(&as_string(
                first_nonempty([field(edge, "edgeKind"), field(edge, "label")]),
                "",
            )) == "hierarchy"
        })
        .map(|edge| {
            (
                as_string(field(edge, "target"), ""),
                as_string(field(edge, "source"), ""),
            )
        })
        .collect()
}

struct GeneralElkGraphBuild {
    graph: Value,
    flat_graph: Value,
    diagram_nodes: Vec<Value>,
    diagram_edges: Vec<Value>,
    use_hierarchy: bool,
}

fn build_general_elk_graph(prepared: &Value) -> Option<GeneralElkGraphBuild> {
    let (diagram_nodes, diagram_edges) = general_diagram_elements(prepared);
    if diagram_nodes.is_empty() {
        return None;
    }
    let package_groups = as_array(field(field(prepared, "meta"), "packageContainerGroups"));
    let use_hierarchy = package_groups.len() >= 2;
    let containment_parent = compute_containment_parent(&diagram_edges);
    if !use_hierarchy && !containment_parent.is_empty() {
        return None;
    }
    let mut flat_children_were_chunked = false;
    let children = if use_hierarchy {
        let mut member_to_package = HashMap::new();
        for group in package_groups {
            for member_id in as_array(field(group, "memberIds")) {
                member_to_package
                    .insert(as_string(member_id, ""), as_string(field(group, "id"), ""));
            }
        }
        let mut by_package: HashMap<String, Vec<Value>> = HashMap::new();
        let mut orphans = Vec::new();
        for node in &diagram_nodes {
            let elk_node = general_leaf_elk_node(node);
            if let Some(package_id) = member_to_package.get(&as_string(field(node, "id"), "")) {
                by_package
                    .entry(package_id.clone())
                    .or_default()
                    .push(elk_node);
            } else {
                orphans.push(elk_node);
            }
        }
        let mut containers = Vec::new();
        for group in package_groups {
            let id = as_string(field(group, "id"), "");
            let members = by_package.remove(&id).unwrap_or_default();
            if members.is_empty() {
                continue;
            }
            containers.push(json!({
                "id": id,
                "layoutOptions": {
                    "elk.direction": "DOWN",
                    "elk.padding": "[top=36,left=20,bottom=20,right=20]",
                },
                "children": chunk_general_elk_children(&id, members),
            }));
        }
        containers.extend(orphans);
        containers
    } else {
        let flat_children: Vec<Value> = diagram_nodes.iter().map(general_leaf_elk_node).collect();
        let children = chunk_general_elk_children("root", flat_children.clone());
        flat_children_were_chunked =
            children.len() != flat_children.len() || (flat_children.len() > 8);
        children
    };
    let edges: Vec<Value> = diagram_edges
        .iter()
        .map(|edge| {
            json!({
                "id": field(edge, "id"),
                "sources": [field(edge, "source")],
                "targets": [field(edge, "target")],
            })
        })
        .collect();
    let hierarchy_handling = if use_hierarchy || flat_children_were_chunked {
        Some("INCLUDE_CHILDREN")
    } else {
        None
    };
    Some(GeneralElkGraphBuild {
        graph: json!({
            "id": "root",
            "layoutOptions": build_elk_layout_options(
                ElkViewKind::General,
                &[("elk.hierarchyHandling", hierarchy_handling)],
            ),
            "children": children,
            "edges": edges,
        }),
        flat_graph: json!({
            "id": "root",
            "layoutOptions": build_elk_layout_options(ElkViewKind::General, &[]),
            "children": diagram_nodes.iter().map(general_leaf_elk_node).collect::<Vec<_>>(),
            "edges": edges,
        }),
        diagram_nodes,
        diagram_edges,
        use_hierarchy,
    })
}

fn reshape_general_layout_result(
    laid_out: &Value,
    diagram_nodes: &[Value],
    diagram_edges: &[Value],
) -> Value {
    let mut layouts = HashMap::new();
    fn visit(elk_node: &Value, ox: f64, oy: f64, layouts: &mut HashMap<String, Value>) {
        let abs_x = ox + num(field(elk_node, "x"));
        let abs_y = oy + num(field(elk_node, "y"));
        let mut copy = elk_node.clone();
        copy["x"] = json!(abs_x);
        copy["y"] = json!(abs_y);
        layouts.insert(as_string(field(elk_node, "id"), ""), copy);
        for child in as_array(field(elk_node, "children")) {
            visit(child, abs_x, abs_y, layouts);
        }
    }
    for child in as_array(field(laid_out, "children")) {
        visit(child, 0.0, 0.0, &mut layouts);
    }
    let mut edges_by_id = HashMap::new();
    fn collect_edges(elk_node: &Value, edges_by_id: &mut HashMap<String, Value>) {
        for edge in as_array(field(elk_node, "edges")) {
            let id = as_string(field(edge, "id"), "");
            if !id.is_empty() {
                edges_by_id.insert(id, edge.clone());
            }
        }
        for child in as_array(field(elk_node, "children")) {
            collect_edges(child, edges_by_id);
        }
    }
    collect_edges(laid_out, &mut edges_by_id);

    let nodes: Vec<Value> = diagram_nodes
        .iter()
        .map(|node| {
            let (width, height, compartments) = general_node_box(node);
            let layout = layouts
                .get(&as_string(field(node, "id"), ""))
                .cloned()
                .unwrap_or(json!({}));
            let mut out = node.clone();
            out["compartments"] = compartments_to_value(&compartments);
            out["x"] = field(&layout, "x").clone();
            out["y"] = field(&layout, "y").clone();
            out["width"] = if field(&layout, "width").is_number() {
                field(&layout, "width").clone()
            } else {
                json!(width)
            };
            out["height"] = if field(&layout, "height").is_number() {
                field(&layout, "height").clone()
            } else {
                json!(height)
            };
            out
        })
        .collect();
    let edges: Vec<Value> = diagram_edges
        .iter()
        .map(|edge| {
            let mut out = edge.clone();
            out["layout"] = edges_by_id
                .get(&as_string(field(edge, "id"), ""))
                .cloned()
                .unwrap_or(Value::Null);
            out
        })
        .collect();
    json!({ "nodes": nodes, "edges": edges })
}

fn fallback_general_layout(nodes: &[Value], edges: &[Value]) -> Value {
    let columns = (nodes.len() as f64).sqrt().ceil().max(1.0) as usize;
    let horizontal_gap = 90.0;
    let vertical_gap = 90.0;
    let measured: Vec<(Value, f64, f64, crate::sysml_node::Compartments)> = nodes
        .iter()
        .map(|node| {
            let (width, height, compartments) = general_node_box(node);
            (node.clone(), width, height, compartments)
        })
        .collect();
    let row_height = NODE_HEIGHT.max(
        measured
            .iter()
            .map(|(_, _, height, _)| *height)
            .fold(0.0, f64::max),
    );
    let column_width = NODE_WIDTH.max(
        measured
            .iter()
            .map(|(_, width, _, _)| *width)
            .fold(0.0, f64::max),
    );
    let laid_out_nodes: Vec<Value> = measured
        .into_iter()
        .enumerate()
        .map(|(index, (mut node, width, height, compartments))| {
            node["compartments"] = compartments_to_value(&compartments);
            node["x"] = json!((index % columns) as f64 * (column_width + horizontal_gap));
            node["y"] = json!((index / columns) as f64 * (row_height + vertical_gap));
            node["width"] = json!(width);
            node["height"] = json!(height);
            node
        })
        .collect();
    let by_id: HashMap<String, Value> = laid_out_nodes
        .iter()
        .map(|node| (as_string(field(node, "id"), ""), node.clone()))
        .collect();
    let laid_out_edges: Vec<Value> = edges
        .iter()
        .map(|edge| {
            let source = by_id.get(&as_string(field(edge, "source"), ""));
            let target = by_id.get(&as_string(field(edge, "target"), ""));
            let start = json!({
                "x": source.map(|n| num(field(n, "x")) + num(field(n, "width"))).unwrap_or(NODE_WIDTH),
                "y": source.map(|n| num(field(n, "y")) + num(field(n, "height")) / 2.0).unwrap_or(NODE_HEIGHT / 2.0),
            });
            let end = json!({
                "x": target.map(|n| num(field(n, "x"))).unwrap_or(0.0),
                "y": target.map(|n| num(field(n, "y")) + num(field(n, "height")) / 2.0).unwrap_or(NODE_HEIGHT / 2.0),
            });
            let mid_x = (num(&start["x"]) + num(&end["x"])) / 2.0;
            let mut out = edge.clone();
            out["layout"] = json!({
                "sections": [{
                    "startPoint": start,
                    "bendPoints": [
                        { "x": mid_x, "y": start["y"] },
                        { "x": mid_x, "y": end["y"] },
                    ],
                    "endPoint": end,
                }],
            });
            out
        })
        .collect();
    json!({ "nodes": laid_out_nodes, "edges": laid_out_edges })
}

fn hierarchy_general_layout(
    nodes: &[Value],
    edges: &[Value],
    parent_by_id: &HashMap<String, String>,
) -> Value {
    let node_order: HashMap<String, usize> = nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (as_string(field(node, "id"), ""), index))
        .collect();
    let mut children_by_parent: HashMap<String, Vec<Value>> = HashMap::new();
    for node in nodes {
        if let Some(parent) = parent_by_id.get(&as_string(field(node, "id"), "")) {
            children_by_parent
                .entry(parent.clone())
                .or_default()
                .push(node.clone());
        }
    }
    for children in children_by_parent.values_mut() {
        children.sort_by_key(|node| {
            node_order
                .get(&as_string(field(node, "id"), ""))
                .copied()
                .unwrap_or(0)
        });
    }
    let mut rows: Vec<Vec<Value>> = Vec::new();
    let mut visited = HashSet::new();
    let mut current: Vec<Value> = nodes
        .iter()
        .filter(|node| !parent_by_id.contains_key(&as_string(field(node, "id"), "")))
        .cloned()
        .collect();
    while !current.is_empty() {
        let row: Vec<Value> = current
            .iter()
            .filter(|node| !visited.contains(&as_string(field(node, "id"), "")))
            .cloned()
            .collect();
        if row.is_empty() {
            break;
        }
        for node in &row {
            visited.insert(as_string(field(node, "id"), ""));
        }
        current = row
            .iter()
            .flat_map(|node| {
                children_by_parent
                    .get(&as_string(field(node, "id"), ""))
                    .cloned()
                    .unwrap_or_default()
            })
            .collect();
        rows.push(row);
    }
    let unvisited: Vec<Value> = nodes
        .iter()
        .filter(|node| !visited.contains(&as_string(field(node, "id"), "")))
        .cloned()
        .collect();
    if !unvisited.is_empty() {
        rows.push(unvisited);
    }
    let horizontal_gap = 140.0;
    let vertical_gap = 180.0;
    let measured_rows: Vec<Vec<(Value, f64, f64, crate::sysml_node::Compartments)>> = rows
        .iter()
        .map(|row| {
            row.iter()
                .map(|node| {
                    let (width, height, compartments) = general_node_box(node);
                    (node.clone(), width, height, compartments)
                })
                .collect()
        })
        .collect();
    let row_widths: Vec<f64> = measured_rows
        .iter()
        .map(|row| {
            row.iter().map(|(_, width, _, _)| *width).sum::<f64>()
                + (row.len().saturating_sub(1) as f64) * horizontal_gap
        })
        .collect();
    let diagram_width = row_widths.iter().copied().fold(0.0, f64::max);
    let mut laid_out_nodes = Vec::new();
    let mut y = 0.0;
    for (row_index, row) in measured_rows.iter().enumerate() {
        let mut x = (diagram_width - row_widths[row_index]) / 2.0;
        let row_height = NODE_HEIGHT.max(
            row.iter()
                .map(|(_, _, height, _)| *height)
                .fold(0.0, f64::max),
        );
        for (node, width, height, compartments) in row {
            let mut out = node.clone();
            out["compartments"] = compartments_to_value(compartments);
            out["x"] = json!(x);
            out["y"] = json!(y);
            out["width"] = json!(*width);
            out["height"] = json!(*height);
            laid_out_nodes.push(out);
            x += width + horizontal_gap;
        }
        y += row_height + vertical_gap;
    }
    let by_id: HashMap<String, Value> = laid_out_nodes
        .iter()
        .map(|node| (as_string(field(node, "id"), ""), node.clone()))
        .collect();
    let row_for: HashMap<String, usize> = rows
        .iter()
        .enumerate()
        .flat_map(|(depth, row)| {
            row.iter()
                .map(move |node| (as_string(field(node, "id"), ""), depth))
        })
        .collect();
    let mut same_row_lane = 0usize;
    let laid_out_edges: Vec<Value> = edges
        .iter()
        .map(|edge| {
            let source_node = by_id.get(&as_string(field(edge, "source"), ""));
            let target_node = by_id.get(&as_string(field(edge, "target"), ""));
            let (Some(source_node), Some(target_node)) = (source_node, target_node) else {
                return edge.clone();
            };
            let source_x = num(field(source_node, "x")) + num(field(source_node, "width")) / 2.0;
            let target_x = num(field(target_node, "x")) + num(field(target_node, "width")) / 2.0;
            let source_depth = row_for
                .get(&as_string(field(edge, "source"), ""))
                .copied()
                .unwrap_or(0);
            let target_depth = row_for
                .get(&as_string(field(edge, "target"), ""))
                .copied()
                .unwrap_or(0);
            let section = if source_depth == target_depth {
                let lane_y = num(field(source_node, "y")).min(num(field(target_node, "y")))
                    - 28.0
                    - (same_row_lane % 12) as f64 * 10.0;
                same_row_lane += 1;
                json!({
                    "startPoint": { "x": source_x, "y": field(source_node, "y") },
                    "bendPoints": [
                        { "x": source_x, "y": lane_y },
                        { "x": target_x, "y": lane_y },
                    ],
                    "endPoint": { "x": target_x, "y": field(target_node, "y") },
                })
            } else {
                let downward = num(field(target_node, "y")) > num(field(source_node, "y"));
                let start_y = num(field(source_node, "y"))
                    + if downward {
                        num(field(source_node, "height"))
                    } else {
                        0.0
                    };
                let end_y = num(field(target_node, "y"))
                    + if downward {
                        0.0
                    } else {
                        num(field(target_node, "height"))
                    };
                let middle_y = (start_y + end_y) / 2.0;
                json!({
                    "startPoint": { "x": source_x, "y": start_y },
                    "bendPoints": [
                        { "x": source_x, "y": middle_y },
                        { "x": target_x, "y": middle_y },
                    ],
                    "endPoint": { "x": target_x, "y": end_y },
                })
            };
            let mut out = edge.clone();
            out["layout"] = json!({ "sections": [section] });
            out
        })
        .collect();
    json!({ "nodes": laid_out_nodes, "edges": laid_out_edges })
}

fn layout_general_prepared(prepared: &Value) -> Result<Value, diagram_layout::LayoutError> {
    if as_array(field(prepared, "nodes")).is_empty() {
        return Ok(json!({
            "title": field(prepared, "title"),
            "view": field(prepared, "view"),
            "meta": field(prepared, "meta"),
            "nodes": [],
            "edges": [],
        }));
    }
    let Some(build) = build_general_elk_graph(prepared) else {
        let (diagram_nodes, diagram_edges) = general_diagram_elements(prepared);
        if diagram_nodes.is_empty() {
            return Ok(json!({
                "title": field(prepared, "title"),
                "view": field(prepared, "view"),
                "meta": field(prepared, "meta"),
                "nodes": [],
                "edges": [],
            }));
        }
        let containment_parent = compute_containment_parent(&diagram_edges);
        let layout = hierarchy_general_layout(&diagram_nodes, &diagram_edges, &containment_parent);
        return Ok(json!({
            "title": field(prepared, "title"),
            "view": field(prepared, "view"),
            "meta": field(prepared, "meta"),
            "nodes": field(&layout, "nodes"),
            "edges": field(&layout, "edges"),
        }));
    };
    let laid_out = match diagram_layout::layout_value(&build.graph) {
        Ok(value) => value,
        Err(_) if !build.use_hierarchy => {
            let layout = fallback_general_layout(&build.diagram_nodes, &build.diagram_edges);
            return Ok(json!({
                "title": field(prepared, "title"),
                "view": field(prepared, "view"),
                "meta": field(prepared, "meta"),
                "nodes": field(&layout, "nodes"),
                "edges": field(&layout, "edges"),
            }));
        }
        Err(_) => match diagram_layout::layout_value(&build.flat_graph) {
            Ok(value) => value,
            Err(_) => {
                let layout = fallback_general_layout(&build.diagram_nodes, &build.diagram_edges);
                return Ok(json!({
                    "title": field(prepared, "title"),
                    "view": field(prepared, "view"),
                    "meta": field(prepared, "meta"),
                    "nodes": field(&layout, "nodes"),
                    "edges": field(&layout, "edges"),
                }));
            }
        },
    };
    let layout =
        reshape_general_layout_result(&laid_out, &build.diagram_nodes, &build.diagram_edges);
    Ok(json!({
        "title": field(prepared, "title"),
        "view": field(prepared, "view"),
        "meta": field(prepared, "meta"),
        "nodes": field(&layout, "nodes"),
        "edges": field(&layout, "edges"),
    }))
}

#[derive(Clone, Copy)]
enum BehaviorMode {
    Action,
    State,
}

fn node_kind(node: &Value) -> String {
    let kind = as_string(field(node, "kind"), "");
    if kind.is_empty() {
        "action".into()
    } else {
        kind.to_ascii_lowercase()
    }
}

fn node_dimensions(node: &Value, mode: BehaviorMode) -> (f64, f64) {
    let kind = node_kind(node);
    match mode {
        BehaviorMode::State => {
            if kind.contains("initial") || kind.contains("final") {
                (34.0, 34.0)
            } else if kind.contains("composite") {
                (340.0, 320.0)
            } else {
                (240.0, 180.0)
            }
        }
        BehaviorMode::Action => {
            if kind.contains("initial")
                || kind.contains("final")
                || kind.contains("start")
                || kind.contains("done")
            {
                (40.0, 40.0)
            } else if kind.contains("decision") || kind.contains("merge") {
                (76.0, 76.0)
            } else if kind.contains("fork") || kind.contains("join") {
                (220.0, 14.0)
            } else {
                (220.0, 68.0)
            }
        }
    }
}

fn transition_display_label(label: &str) -> String {
    let trimmed = label.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("entry") {
        String::new()
    } else {
        trimmed.to_string()
    }
}

fn estimate_elk_label_box(id: &str, text: &str) -> Value {
    let padding_x = 8.0;
    let padding_y = 6.0;
    let min_width: f64 = 38.0;
    let min_height: f64 = 16.0;
    let char_width = 6.0;
    json!({
        "id": id,
        "text": text,
        "x": 0,
        "y": 0,
        "width": min_width.max(text.encode_utf16().count() as f64 * char_width + padding_x),
        "height": min_height.max(padding_y + 10.0),
    })
}

fn build_behavior_elk_graph(prepared: &Value, horizontal: bool, mode: BehaviorMode) -> Value {
    let children: Vec<Value> = as_array(field(prepared, "nodes"))
        .iter()
        .map(|node| {
            let (width, height) = node_dimensions(node, mode);
            json!({
                "id": field(node, "id"),
                "width": width,
                "height": height,
            })
        })
        .collect();
    let edges: Vec<Value> = as_array(field(prepared, "edges"))
        .iter()
        .map(|edge| {
            let display_label = transition_display_label(&as_string(field(edge, "label"), ""));
            let mut base = json!({
                "id": field(edge, "id"),
                "sources": [field(edge, "source")],
                "targets": [field(edge, "target")],
            });
            if display_label.is_empty() {
                return base;
            }
            let id = format!("{}::label", as_string(field(edge, "id"), ""));
            let mut label_box = estimate_elk_label_box(&id, &display_label);
            label_box["layoutOptions"] = json!({
                "org.eclipse.elk.edgeLabels.placement": "CENTER",
                "org.eclipse.elk.edgeLabels.inline": "false",
            });
            base["labels"] = json!([label_box]);
            base
        })
        .collect();
    let kind = match mode {
        BehaviorMode::State => ElkViewKind::BehaviorState,
        BehaviorMode::Action => ElkViewKind::BehaviorAction,
    };
    let overrides: Vec<(&str, Option<&str>)> = match mode {
        BehaviorMode::State => vec![(
            "elk.direction",
            Some(if horizontal { "RIGHT" } else { "DOWN" }),
        )],
        BehaviorMode::Action => vec![
            (
                "elk.direction",
                Some(if horizontal { "RIGHT" } else { "DOWN" }),
            ),
            (
                "elk.spacing.nodeNode",
                Some(if horizontal { "90" } else { "120" }),
            ),
            (
                "elk.layered.spacing.nodeNodeBetweenLayers",
                Some(if horizontal { "190" } else { "170" }),
            ),
        ],
    };
    let graph_id = {
        let title = as_string(field(prepared, "title"), "");
        if title.is_empty() {
            "behavior".to_string()
        } else {
            title
        }
    };
    json!({
        "id": graph_id,
        "layoutOptions": build_elk_layout_options(kind, &overrides),
        "children": children,
        "edges": edges,
    })
}

fn collect_elk_edge_labels(
    elk_node: &Value,
    offset: Point,
    acc: &mut BTreeMap<String, Vec<Value>>,
) {
    for edge in as_array(field(elk_node, "edges")) {
        let id = as_string(field(edge, "id"), "");
        let labels = as_array(field(edge, "labels"));
        if id.is_empty() || labels.is_empty() {
            continue;
        }
        let collected: Vec<Value> = labels
            .iter()
            .filter_map(|label| {
                let label_id = as_string(field(label, "id"), "");
                let text = as_string(field(label, "text"), "");
                if label_id.is_empty() || text.is_empty() {
                    return None;
                }
                Some(json!({
                    "id": label_id,
                    "text": text,
                    "x": num(field(label, "x")) + offset.x,
                    "y": num(field(label, "y")) + offset.y,
                    "width": num(field(label, "width")),
                    "height": num(field(label, "height")),
                }))
            })
            .collect();
        if !collected.is_empty() {
            acc.insert(id, collected);
        }
    }
    for child in as_array(field(elk_node, "children")) {
        collect_elk_edge_labels(
            child,
            Point {
                x: offset.x + num(field(child, "x")),
                y: offset.y + num(field(child, "y")),
            },
            acc,
        );
    }
}

fn layout_behavior_graph(
    prepared: &Value,
    horizontal: bool,
    mode: BehaviorMode,
) -> Result<Value, diagram_layout::LayoutError> {
    let graph = build_behavior_elk_graph(prepared, horizontal, mode);
    let laid_out = diagram_layout::layout_value(&graph)?;
    let mut positions = Map::new();
    for child in as_array(field(&laid_out, "children")) {
        positions.insert(
            as_string(field(child, "id"), ""),
            json!({
                "x": num(field(child, "x")),
                "y": num(field(child, "y")),
                "width": field(child, "width").as_f64().unwrap_or(200.0),
                "height": field(child, "height").as_f64().unwrap_or(80.0),
            }),
        );
    }
    let mut edge_sections = Map::new();
    for edge in as_array(field(&laid_out, "edges")) {
        if field(edge, "sections").is_array() {
            edge_sections.insert(
                as_string(field(edge, "id"), ""),
                field(edge, "sections").clone(),
            );
        }
    }
    let mut edge_labels = BTreeMap::new();
    collect_elk_edge_labels(&laid_out, Point { x: 0.0, y: 0.0 }, &mut edge_labels);
    Ok(json!({
        "positions": positions,
        "edgeSectionsById": edge_sections,
        "edgeLabelsById": edge_labels,
    }))
}

#[derive(Clone)]
struct PreparedPort {
    id: String,
    name: String,
    semantic_id: String,
    direction: String,
    port_side: String,
    side_hint: String,
    raw: Value,
}

fn port_details_for(node: &Value) -> Vec<PreparedPort> {
    let attrs = field(node, "attributes");
    let details = as_array(field(attrs, "portDetails"));
    if !details.is_empty() {
        return details
            .iter()
            .filter_map(|item| {
                if !item.is_object() {
                    return None;
                }
                let name = as_string(field(item, "name"), "");
                if name.is_empty() {
                    return None;
                }
                Some(PreparedPort {
                    id: as_string(field(item, "id"), ""),
                    name,
                    semantic_id: as_string(field(item, "semanticId"), ""),
                    direction: as_string(field(item, "direction"), ""),
                    port_side: as_string(
                        first_nonempty([
                            field(item, "portSide"),
                            field(field(item, "attributes"), "portSide"),
                        ]),
                        "",
                    ),
                    side_hint: as_string(field(field(item, "attributes"), "sideHint"), ""),
                    raw: item.clone(),
                })
            })
            .collect();
    }
    as_array(field(attrs, "ports"))
        .iter()
        .map(|name| {
            let name = as_string(name, "");
            PreparedPort {
                id: String::new(),
                name: name.clone(),
                semantic_id: String::new(),
                direction: String::new(),
                port_side: String::new(),
                side_hint: String::new(),
                raw: json!({ "name": name }),
            }
        })
        .collect()
}

fn normalize_endpoint(value: &Value) -> String {
    as_string(value, "").replace("::", ".").trim().to_string()
}

fn js_hash_stable_side(key: &str) -> &'static str {
    let mut hash: i32 = 0;
    for unit in key.encode_utf16() {
        hash = hash.wrapping_mul(31).wrapping_add(unit as i32);
    }
    if hash & 1 == 0 {
        "WEST"
    } else {
        "EAST"
    }
}

struct PortUsage {
    source_count: i32,
    target_count: i32,
}

fn layout_interconnection_prepared(prepared: &Value) -> Result<Value, diagram_layout::LayoutError> {
    let nodes = as_array(field(prepared, "nodes"));
    let nodes_by_id: HashMap<String, Value> = nodes
        .iter()
        .map(|node| (as_string(field(node, "id"), ""), node.clone()))
        .collect();
    let mut children_by_parent: HashMap<String, Vec<Value>> = HashMap::new();
    let mut roots = Vec::new();
    for node in nodes {
        let parent_id = match field(field(node, "attributes"), "containerId") {
            Value::String(s) => s.clone(),
            _ => String::new(),
        };
        if !parent_id.is_empty() && nodes_by_id.contains_key(&parent_id) {
            children_by_parent
                .entry(parent_id)
                .or_default()
                .push(node.clone());
        } else {
            roots.push(node.clone());
        }
    }
    let sanitize_id = |value: &str| {
        value
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '-' {
                    c
                } else {
                    '_'
                }
            })
            .collect::<String>()
    };
    let mut prepared_id_for_elk_id = HashMap::new();
    let mut register_elk_id = |prepared_id: &str| {
        let elk_id = sanitize_id(prepared_id);
        prepared_id_for_elk_id.insert(elk_id.clone(), prepared_id.to_string());
        elk_id
    };
    let port_id_for = |node_id: &str, port_name: &str| {
        format!("{}__port__{}", sanitize_id(node_id), sanitize_id(port_name))
    };

    let mut port_usage: HashMap<String, PortUsage> = HashMap::new();
    let mut bump = |endpoint: &Value, source: bool| {
        let normalized = normalize_endpoint(endpoint);
        if normalized.is_empty() {
            return;
        }
        let entry = port_usage.entry(normalized).or_insert(PortUsage {
            source_count: 0,
            target_count: 0,
        });
        if source {
            entry.source_count += 1;
        } else {
            entry.target_count += 1;
        }
    };
    for edge in as_array(field(prepared, "edges")) {
        bump(
            first_nonempty([
                field(field(edge, "attributes"), "sourceId"),
                field(edge, "source"),
            ]),
            true,
        );
        bump(
            first_nonempty([
                field(field(edge, "attributes"), "targetId"),
                field(edge, "target"),
            ]),
            false,
        );
    }

    let port_layout_keys = |node: &Value, port: &PreparedPort| -> Vec<String> {
        let attrs = field(node, "attributes");
        let mut keys = Vec::new();
        let explicit = normalize_endpoint(&Value::String(port.id.clone()));
        if !explicit.is_empty() {
            keys.push(explicit);
        }
        let parent = normalize_endpoint(first_nonempty([
            field(field(&port.raw, "attributes"), "parentId"),
            field(&port.raw, "parentId"),
            field(attrs, "qualifiedName"),
            field(node, "id"),
            field(node, "label"),
        ]));
        if !parent.is_empty() {
            keys.push(format!("{parent}.{}", port.name));
        }
        keys.push(normalize_endpoint(&Value::String(port.name.clone())));
        keys.sort();
        keys.dedup();
        keys.into_iter().filter(|key| !key.is_empty()).collect()
    };

    let usage_for_port = |node: &Value, port: &PreparedPort| -> PortUsage {
        for key in port_layout_keys(node, port) {
            if let Some(explicit) = port_usage.get(&key) {
                return PortUsage {
                    source_count: explicit.source_count,
                    target_count: explicit.target_count,
                };
            }
        }
        let attrs = field(node, "attributes");
        let parent = normalize_endpoint(first_nonempty([
            field(attrs, "qualifiedName"),
            field(node, "id"),
            field(node, "label"),
        ]));
        let fallback_key = format!(
            "{}.{}",
            parent,
            normalize_endpoint(&Value::String(port.name.clone()))
        );
        if let Some(fallback) = port_usage.get(&fallback_key) {
            return PortUsage {
                source_count: fallback.source_count,
                target_count: fallback.target_count,
            };
        }
        let aliases = [
            normalize_endpoint(field(node, "id")),
            normalize_endpoint(field(node, "label")),
            normalize_endpoint(field(attrs, "qualifiedName")),
        ]
        .into_iter()
        .filter(|alias| !alias.is_empty())
        .collect::<Vec<_>>();
        let port_name = normalize_endpoint(&Value::String(port.name.clone()));
        let mut usage = PortUsage {
            source_count: 0,
            target_count: 0,
        };
        let node_label = as_string(field(node, "label"), "");
        for (endpoint, counts) in &port_usage {
            if !endpoint.ends_with(&format!(".{port_name}")) && endpoint != &port_name {
                continue;
            }
            let owner = if endpoint == &port_name {
                String::new()
            } else {
                endpoint[..endpoint.len() - port_name.len() - 1].to_string()
            };
            let matches_owner = aliases.iter().any(|alias| {
                owner == *alias
                    || owner.ends_with(&format!(".{alias}"))
                    || alias.ends_with(&format!(".{owner}"))
                    || owner.ends_with(&format!(".{node_label}"))
            });
            if !matches_owner {
                continue;
            }
            usage.source_count += counts.source_count;
            usage.target_count += counts.target_count;
        }
        usage
    };

    let side_for_port = |port: &PreparedPort, node: &Value| -> &'static str {
        let side_hint = port.side_hint.to_ascii_lowercase();
        if side_hint == "west" {
            return "WEST";
        }
        if side_hint == "east" {
            return "EAST";
        }
        let explicit = port.port_side.to_ascii_lowercase();
        if explicit == "left" || explicit == "west" {
            return "WEST";
        }
        if explicit == "right" || explicit == "east" {
            return "EAST";
        }
        let direction = port.direction.to_ascii_lowercase();
        if direction == "in" {
            return "WEST";
        }
        if direction == "out" {
            return "EAST";
        }
        let usage = usage_for_port(node, port);
        if usage.target_count > usage.source_count {
            return "WEST";
        }
        if usage.source_count > usage.target_count {
            return "EAST";
        }
        let fallback_key = format!("{}::{}", as_string(field(node, "id"), ""), port.name);
        js_hash_stable_side(if !port.id.is_empty() {
            &port.id
        } else if !port.semantic_id.is_empty() {
            &port.semantic_id
        } else {
            &fallback_key
        })
    };

    let compare_ports = |node: &Value, a: &PreparedPort, b: &PreparedPort| {
        let usage_a = usage_for_port(node, a);
        let usage_b = usage_for_port(node, b);
        let degree_a = usage_a.source_count + usage_a.target_count;
        let degree_b = usage_b.source_count + usage_b.target_count;
        degree_b.cmp(&degree_a).then_with(|| a.name.cmp(&b.name))
    };

    let split_ports = |node: &Value, ports: &[PreparedPort]| {
        let mut west = Vec::new();
        let mut east = Vec::new();
        for port in ports {
            if side_for_port(port, node) == "WEST" {
                west.push(port.clone());
            } else {
                east.push(port.clone());
            }
        }
        west.sort_by(|a, b| compare_ports(node, a, b));
        east.sort_by(|a, b| compare_ports(node, a, b));
        (west, east)
    };

    fn compute_ibd_leaf_height(node: &Value, ports: &[PreparedPort], port_rows: usize) -> f64 {
        let attrs = field(node, "attributes");
        let header_height = if field(attrs, "partType")
            .as_str()
            .filter(|s| !s.is_empty())
            .is_some()
        {
            50.0
        } else {
            38.0
        };
        let content_line_count = as_array(field(attrs, "children"))
            .iter()
            .filter(|child| child.is_object() && !as_string(field(child, "name"), "").is_empty())
            .count();
        let content_height = (content_line_count.min(8) as f64) * 12.0 + 10.0;
        let ports_height = if ports.is_empty() {
            0.0
        } else {
            port_rows as f64 * 26.0 + 22.0
        };
        IBD_NODE_HEIGHT
            .max(header_height + content_height + ports_height)
            .min(340.0)
    }

    let root_header_height = 28.0;
    let container_top_inset = root_header_height + 20.0;

    #[allow(clippy::too_many_arguments)]
    fn to_elk_node(
        node: &Value,
        children_by_parent: &HashMap<String, Vec<Value>>,
        register_elk_id: &mut impl FnMut(&str) -> String,
        port_id_for: &impl Fn(&str, &str) -> String,
        split_ports: &impl Fn(&Value, &[PreparedPort]) -> (Vec<PreparedPort>, Vec<PreparedPort>),
        port_details_for: &impl Fn(&Value) -> Vec<PreparedPort>,
        root_header_height: f64,
        container_top_inset: f64,
    ) -> Value {
        let ports = port_details_for(node);
        let (west_ports, east_ports) = split_ports(node, &ports);
        let port_rows = west_ports
            .len()
            .max(east_ports.len())
            .max(if ports.is_empty() { 0 } else { 1 });
        let children: Vec<Value> = children_by_parent
            .get(&as_string(field(node, "id"), ""))
            .map(|children| {
                children
                    .iter()
                    .map(|child| {
                        to_elk_node(
                            child,
                            children_by_parent,
                            register_elk_id,
                            port_id_for,
                            split_ports,
                            port_details_for,
                            root_header_height,
                            container_top_inset,
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();
        let attrs = field(node, "attributes");
        let is_synthetic_package = field(attrs, "isSyntheticPackage") == &json!(true);
        let is_container =
            field(attrs, "isSyntheticContainer") == &json!(true) || !children.is_empty();
        let base_width = if is_container { 420.0 } else { IBD_NODE_WIDTH };
        let label_width = as_string(field(node, "label"), "").encode_utf16().count() as f64 * 6.0;
        let port_width = ports
            .iter()
            .map(|port| port.name.encode_utf16().count() as f64 * 5.0)
            .fold(0.0, f64::max);
        let mut width = base_width.max(180.0 + label_width.max(port_width));
        let mut height = if is_container {
            root_header_height + 72.0
        } else {
            compute_ibd_leaf_height(node, &ports, port_rows)
        };
        if is_container && !children.is_empty() {
            let child_width_sum: f64 = children
                .iter()
                .map(|child| field(child, "width").as_f64().unwrap_or(IBD_NODE_WIDTH))
                .sum();
            width = if is_synthetic_package {
                width.max((child_width_sum + children.len() as f64 * 44.0).min(980.0))
            } else {
                width.max((child_width_sum + children.len() as f64 * 72.0).min(1040.0))
            };
            height = if is_synthetic_package {
                root_header_height + 72.0
            } else {
                root_header_height
                    + 72.0_f64.max(132.0_f64.min(58.0 + children.len() as f64 * 14.0))
            };
        }
        let build_elk_port = |port: &PreparedPort, side: &str, index: usize| {
            let id = port_id_for(&as_string(field(node, "id"), ""), &port.name);
            let label_text = ibd_port_label_text(&port.name);
            json!({
                "id": id,
                "width": 10,
                "height": 10,
                "labels": [{
                    "id": format!("{id}__label"),
                    "text": label_text,
                    "width": ibd_port_label_width(&label_text),
                    "height": IBD_PORT_LABEL_HEIGHT,
                }],
                "layoutOptions": {
                    "org.eclipse.elk.port.side": side,
                    "org.eclipse.elk.port.index": index.to_string(),
                },
            })
        };
        let node_layout_options = json!({
            "org.eclipse.elk.portConstraints": "FIXED_ORDER",
            "org.eclipse.elk.portAlignment.default": "CENTER",
            "org.eclipse.elk.portLabels.placement": "INSIDE",
            "org.eclipse.elk.spacing.labelPortHorizontal": "6",
            "org.eclipse.elk.spacing.labelPortVertical": "4",
            "org.eclipse.elk.nodeSize.constraints": "PORTS PORT_LABELS MINIMUM_SIZE",
            "org.eclipse.elk.nodeSize.minimum": format!("({width},{height})"),
        });
        let layout_options = if children.is_empty() {
            node_layout_options
        } else {
            let mut options = as_object(&node_layout_options);
            // ELK does not inherit the root graph's spacing for nested layered graphs. Without
            // these options, sibling parts are separated by the default 40 px, leaving only a
            // 20 px line between two 10 px boundary ports (the office two-monitor case).
            options.insert("elk.spacing.nodeNode".into(), json!("90"));
            options.insert(
                "elk.layered.spacing.nodeNodeBetweenLayers".into(),
                json!("140"),
            );
            options.insert("elk.spacing.edgeNode".into(), json!("50"));
            options.insert("elk.spacing.edgeEdge".into(), json!("30"));
            options.insert(
                "elk.padding".into(),
                Value::String(if is_synthetic_package {
                    format!(
                        "[top={},left=16,bottom=16,right=16]",
                        root_header_height + 12.0
                    )
                } else {
                    format!("[top={container_top_inset},left=24,bottom=24,right=24]")
                }),
            );
            options.insert(
                "elk.direction".into(),
                Value::String(if is_synthetic_package {
                    "DOWN".into()
                } else {
                    "RIGHT".into()
                }),
            );
            Value::Object(options)
        };
        json!({
            "id": register_elk_id(&as_string(field(node, "id"), "")),
            "width": width,
            "height": height,
            "ports": west_ports.iter().enumerate().map(|(index, port)| build_elk_port(port, "WEST", index))
                .chain(east_ports.iter().enumerate().map(|(index, port)| build_elk_port(port, "EAST", index)))
                .collect::<Vec<_>>(),
            "children": children,
            "layoutOptions": layout_options,
        })
    }

    let mut elk_children = Vec::new();
    for node in &roots {
        elk_children.push(to_elk_node(
            node,
            &children_by_parent,
            &mut register_elk_id,
            &port_id_for,
            &split_ports,
            &port_details_for,
            root_header_height,
            container_top_inset,
        ));
    }

    let connector_port_name = |node: &Value, endpoint: &Value| -> Option<String> {
        let endpoint_text = as_string(endpoint, "").trim().to_string();
        if endpoint_text.is_empty() {
            return None;
        }
        port_details_for(node).into_iter().find_map(|port| {
            if port.id == endpoint_text
                || as_string(field(field(&port.raw, "attributes"), "scenePortId"), "")
                    == endpoint_text
            {
                Some(port.name)
            } else {
                None
            }
        })
    };

    let elk_edges: Vec<Value> = as_array(field(prepared, "edges"))
        .iter()
        .filter_map(|edge| {
            let source_node = nodes_by_id.get(&as_string(field(edge, "source"), ""))?;
            let target_node = nodes_by_id.get(&as_string(field(edge, "target"), ""))?;
            let source_endpoint = first_nonempty([
                field(field(edge, "attributes"), "sourcePortId"),
                field(field(edge, "attributes"), "sourceId"),
            ]);
            let target_endpoint = first_nonempty([
                field(field(edge, "attributes"), "targetPortId"),
                field(field(edge, "attributes"), "targetId"),
            ]);
            let source_port_name = connector_port_name(source_node, source_endpoint);
            let target_port_name = connector_port_name(target_node, target_endpoint);
            Some(json!({
                "id": field(edge, "id"),
                "sources": [source_port_name.as_ref().map(|name| port_id_for(&as_string(field(source_node, "id"), ""), name)).unwrap_or_else(|| sanitize_id(&as_string(field(source_node, "id"), "")))],
                "targets": [target_port_name.as_ref().map(|name| port_id_for(&as_string(field(target_node, "id"), ""), name)).unwrap_or_else(|| sanitize_id(&as_string(field(target_node, "id"), "")))],
                "sourcePortId": source_port_name.as_ref().map(|name| port_id_for(&as_string(field(source_node, "id"), ""), name)),
                "targetPortId": target_port_name.as_ref().map(|name| port_id_for(&as_string(field(target_node, "id"), ""), name)),
            }))
        })
        .collect();

    let elk_graph_input = json!({
        "id": "root",
        "layoutOptions": build_elk_layout_options(ElkViewKind::Interconnection, &[]),
        "children": elk_children,
        "edges": elk_edges.iter().map(|edge| json!({
            "id": field(edge, "id"),
            "sources": field(edge, "sources"),
            "targets": field(edge, "targets"),
        })).collect::<Vec<_>>(),
    });

    let laid_out = diagram_layout::layout_value(&elk_graph_input)?;

    let mut laid_out_nodes: HashMap<String, LaidOutNode> = HashMap::new();
    let mut port_centers: HashMap<String, Point> = HashMap::new();
    let mut node_port_anchors: HashMap<String, BTreeMap<String, Value>> = HashMap::new();
    let mut layout_nodes: Vec<Value> = Vec::new();
    let mut layout_containers: Vec<Value> = Vec::new();
    let mut diagnostics: Vec<String> = Vec::new();

    #[allow(clippy::too_many_arguments)]
    fn visit_ibd(
        elk_node: &Value,
        ox: f64,
        oy: f64,
        depth: usize,
        prepared_id_for_elk_id: &HashMap<String, String>,
        nodes_by_id: &HashMap<String, Value>,
        split_ports: &impl Fn(&Value, &[PreparedPort]) -> (Vec<PreparedPort>, Vec<PreparedPort>),
        port_details_for: &impl Fn(&Value) -> Vec<PreparedPort>,
        laid_out_nodes: &mut HashMap<String, LaidOutNode>,
        port_centers: &mut HashMap<String, Point>,
        node_port_anchors: &mut HashMap<String, BTreeMap<String, Value>>,
        layout_nodes: &mut Vec<Value>,
        layout_containers: &mut Vec<Value>,
    ) {
        let abs_x = ox + num(field(elk_node, "x"));
        let abs_y = oy + num(field(elk_node, "y"));
        let elk_id = as_string(field(elk_node, "id"), "");
        let prepared_id = prepared_id_for_elk_id
            .get(&elk_id)
            .cloned()
            .unwrap_or(elk_id);
        let base = nodes_by_id.get(&prepared_id);
        for port in as_array(field(elk_node, "ports")) {
            let pw = field(port, "width").as_f64().unwrap_or(10.0);
            let ph = field(port, "height").as_f64().unwrap_or(10.0);
            let side = as_string(
                field(field(port, "layoutOptions"), "org.eclipse.elk.port.side"),
                "",
            );
            let x = if side == "WEST" {
                abs_x + num(field(port, "x"))
            } else if side == "EAST" {
                abs_x + num(field(port, "x")) + pw
            } else {
                abs_x + num(field(port, "x")) + pw / 2.0
            };
            let y = abs_y + num(field(port, "y")) + ph / 2.0;
            port_centers.insert(as_string(field(port, "id"), ""), Point { x, y });
            if let Some(base) = base {
                let port_name = as_string(field(port, "id"), "")
                    .rsplit("__port__")
                    .next()
                    .unwrap_or("")
                    .to_string();
                let mut anchor = json!({
                    "x": x - abs_x,
                    "y": y - abs_y,
                    "side": side,
                });
                if let Some(label) = as_array(field(port, "labels")).first() {
                    anchor["label"] = json!({
                        "x": num(field(port, "x")) + num(field(label, "x")),
                        "y": num(field(port, "y")) + num(field(label, "y")),
                        "width": num(field(label, "width")),
                        "height": num(field(label, "height")),
                        "text": as_string(field(label, "text"), ""),
                    });
                }
                node_port_anchors
                    .entry(as_string(field(base, "id"), ""))
                    .or_default()
                    .insert(port_name, anchor);
            }
        }
        if let Some(base) = base {
            let attrs = field(base, "attributes");
            let has_layout_children = !as_array(field(elk_node, "children")).is_empty();
            let is_container_frame =
                has_layout_children || field(attrs, "isSyntheticContainer") == &json!(true);
            let ports = port_details_for(base);
            let (west, east) = split_ports(base, &ports);
            let port_draw_order = json!({
                "west": west.iter().map(|port| port.name.clone()).collect::<Vec<_>>(),
                "east": east.iter().map(|port| port.name.clone()).collect::<Vec<_>>(),
            });
            let port_anchors = node_port_anchors
                .get(&as_string(field(base, "id"), ""))
                .cloned()
                .unwrap_or_default();
            let laid_out_width = field(elk_node, "width").as_f64().unwrap_or(IBD_NODE_WIDTH);
            let laid_out_height = field(elk_node, "height")
                .as_f64()
                .unwrap_or(IBD_NODE_HEIGHT);
            layout_nodes.push(json!({
                "id": field(base, "id"),
                "x": abs_x,
                "y": abs_y,
                "width": laid_out_width,
                "height": laid_out_height,
                "portAnchors": port_anchors,
                "portDrawOrder": port_draw_order,
            }));
            if is_container_frame {
                layout_containers.push(json!({
                    "id": field(base, "id"),
                    "label": field(base, "label"),
                    "x": abs_x,
                    "y": abs_y,
                    "width": laid_out_width,
                    "height": laid_out_height,
                }));
            }
            let mut attributes = node_attributes(base);
            attributes.insert("_isLayoutContainer".into(), json!(has_layout_children));
            attributes.insert("_layoutDepth".into(), json!(depth));
            laid_out_nodes.insert(
                as_string(field(base, "id"), ""),
                LaidOutNode {
                    id: as_string(field(base, "id"), ""),
                    label: as_string(field(base, "label"), ""),
                    kind: as_string(field(base, "kind"), ""),
                    attributes,
                    x: abs_x,
                    y: abs_y,
                    width: laid_out_width,
                    height: laid_out_height,
                    compartments: None,
                },
            );
        }
        for child in as_array(field(elk_node, "children")) {
            visit_ibd(
                child,
                abs_x,
                abs_y,
                depth + 1,
                prepared_id_for_elk_id,
                nodes_by_id,
                split_ports,
                port_details_for,
                laid_out_nodes,
                port_centers,
                node_port_anchors,
                layout_nodes,
                layout_containers,
            );
        }
    }

    for child in as_array(field(&laid_out, "children")) {
        visit_ibd(
            child,
            0.0,
            0.0,
            0,
            &prepared_id_for_elk_id,
            &nodes_by_id,
            &split_ports,
            &port_details_for,
            &mut laid_out_nodes,
            &mut port_centers,
            &mut node_port_anchors,
            &mut layout_nodes,
            &mut layout_containers,
        );
    }

    let mut edge_layout: HashMap<String, (Value, Point)> = HashMap::new();
    fn collect_elk_edges(
        elk_node: &Value,
        container_offset: Point,
        edge_layout: &mut HashMap<String, (Value, Point)>,
    ) {
        for edge in as_array(field(elk_node, "edges")) {
            let edge_id = as_string(field(edge, "id"), "");
            if !edge_id.is_empty() {
                edge_layout.insert(edge_id, (edge.clone(), container_offset));
            }
        }
        for child in as_array(field(elk_node, "children")) {
            collect_elk_edges(
                child,
                Point {
                    x: container_offset.x + num(field(child, "x")),
                    y: container_offset.y + num(field(child, "y")),
                },
                edge_layout,
            );
        }
    }
    collect_elk_edges(&laid_out, Point { x: 0.0, y: 0.0 }, &mut edge_layout);
    for edge in as_array(field(&laid_out, "edges")) {
        let edge_id = as_string(field(edge, "id"), "");
        if !edge_id.is_empty() {
            edge_layout.insert(edge_id, (edge.clone(), Point { x: 0.0, y: 0.0 }));
        }
    }

    let out_nodes: Vec<Value> = as_array(field(prepared, "nodes"))
        .iter()
        .filter_map(|node| {
            let laid = laid_out_nodes.get(&as_string(field(node, "id"), ""))?;
            let mut out = node.clone();
            out["x"] = json!(laid.x);
            out["y"] = json!(laid.y);
            out["width"] = json!(laid.width);
            out["height"] = json!(laid.height);
            out["attributes"] = json!(laid.attributes);
            Some(out)
        })
        .collect();

    let mut out_edges = Vec::new();
    for edge in as_array(field(prepared, "edges")) {
        let layout_record = edge_layout.get(&as_string(field(edge, "id"), ""));
        let elk_edge = elk_edges
            .iter()
            .find(|item| as_string(field(item, "id"), "") == as_string(field(edge, "id"), ""));
        let source_node = laid_out_nodes.get(&as_string(field(edge, "source"), ""));
        let target_node = laid_out_nodes.get(&as_string(field(edge, "target"), ""));
        let source_port_center = elk_edge.and_then(|item| {
            field(item, "sourcePortId")
                .as_str()
                .and_then(|id| port_centers.get(id).copied())
        });
        let target_port_center = elk_edge.and_then(|item| {
            field(item, "targetPortId")
                .as_str()
                .and_then(|id| port_centers.get(id).copied())
        });
        if (field(field(edge, "attributes"), "sourcePortId")
            .as_str()
            .is_some()
            || field(field(edge, "attributes"), "targetPortId")
                .as_str()
                .is_some())
            && (source_port_center.is_none() || target_port_center.is_none())
        {
            diagnostics.push(format!(
                "node-boundary fallback for edge {}",
                as_string(field(edge, "id"), "")
            ));
        }
        let sections = layout_record
            .and_then(|(elk_edge, _)| field(elk_edge, "sections").as_array())
            .filter(|sections| !sections.is_empty())
            .cloned();
        let layout = if let Some(sections) = sections {
            json!({
                "sections": sections,
                "edgeOwnerOffset": layout_record.map(|(_, offset)| json!({ "x": offset.x, "y": offset.y })).unwrap_or(json!({ "x": 0, "y": 0 })),
                "lcaOffset": match (source_node, target_node) {
                    (Some(source), Some(target)) => {
                        let offset = lca_offset_for_nodes(source, target, &laid_out_nodes);
                        json!({ "x": offset.x, "y": offset.y })
                    }
                    _ => json!({ "x": 0, "y": 0 }),
                },
            })
        } else {
            let fallback = fallback_edge_sections(
                source_node,
                target_node,
                source_port_center,
                target_port_center,
            );
            json!({
                "sections": fallback,
                "edgeOwnerOffset": { "x": 0, "y": 0 },
                "lcaOffset": { "x": 0, "y": 0 },
            })
        };
        let mut attributes = as_object(field(edge, "attributes"));
        if let Some(point) = source_port_center {
            attributes.insert(
                "_sourcePortCenter".into(),
                json!({ "x": point.x, "y": point.y }),
            );
        }
        if let Some(point) = target_port_center {
            attributes.insert(
                "_targetPortCenter".into(),
                json!({ "x": point.x, "y": point.y }),
            );
        }
        let mut out = edge.clone();
        out["layout"] = layout;
        out["attributes"] = Value::Object(attributes);
        out_edges.push(out);
    }

    let layout_edges: Vec<Value> = out_edges
        .iter()
        .map(|edge| {
            let laid = LaidOutEdge {
                id: as_string(field(edge, "id"), ""),
                source: as_string(field(edge, "source"), ""),
                target: as_string(field(edge, "target"), ""),
                label: as_string(field(edge, "label"), ""),
                edge_kind: field(edge, "edgeKind").as_str().map(str::to_string),
                attributes: field(edge, "attributes")
                    .as_object()
                    .map(|map| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                    .unwrap_or_default(),
                layout: serde_json::from_value(field(edge, "layout").clone()).ok(),
            };
            json!({
                "id": laid.id,
                "routePoints": resolve_ibd_route_points(&laid).unwrap_or_default(),
                "sourcePortId": attr_str(&laid.attributes, "sourcePortId").unwrap_or_default(),
                "targetPortId": attr_str(&laid.attributes, "targetPortId").unwrap_or_default(),
            })
        })
        .collect();

    Ok(json!({
        "title": field(prepared, "title"),
        "view": field(prepared, "view"),
        "meta": field(prepared, "meta"),
        "nodes": out_nodes,
        "edges": out_edges,
        "interconnectionLayout": {
            "nodes": layout_nodes,
            "edges": layout_edges,
            "containers": layout_containers,
            "diagnostics": diagnostics,
        },
    }))
}

fn fallback_edge_sections(
    source_node: Option<&LaidOutNode>,
    target_node: Option<&LaidOutNode>,
    source_port_center: Option<Point>,
    target_port_center: Option<Point>,
) -> Option<Vec<Value>> {
    let source = source_node?;
    let target = target_node?;
    let start = source_port_center.unwrap_or(Point {
        x: source.x + source.width,
        y: source.y + source.height / 2.0,
    });
    let end = target_port_center.unwrap_or(Point {
        x: target.x,
        y: target.y + target.height / 2.0,
    });
    let mid_x = (start.x + end.x) / 2.0;
    Some(vec![json!({
        "startPoint": { "x": start.x, "y": start.y },
        "bendPoints": [
            { "x": mid_x, "y": start.y },
            { "x": mid_x, "y": end.y },
        ],
        "endPoint": { "x": end.x, "y": end.y },
    })])
}
