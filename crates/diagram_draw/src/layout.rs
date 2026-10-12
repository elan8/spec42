//! Port of `render/layout.ts`, `views/behavior-common.ts` layout, and
//! `render/interconnection-elk-input.ts` onto `diagram_layout` (elkrs).

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use serde_json::{json, Map, Value};

use crate::graph_normalization::{
    is_connector_usage_element_type, is_overview_visual_element_type, normalize_edge_kind,
};
use crate::ibd_ports::{ibd_port_detail_label, ibd_port_label_width};
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
            let direction = as_string(field(field(prepared, "meta"), "layoutDirection"), "");
            let horizontal = direction.eq_ignore_ascii_case("horizontal");
            if view == "state-transition-view" {
                // A state machine has no natural direction. Unless the caller chose one, lay it
                // out both ways and keep the one that is larger on the reference page.
                // States that own states are laid out level by level, each machine on its own.
                // Like a flat machine, it is laid out both ways and the one that is larger on
                // the reference page is kept; on a tie the horizontal one.
                let extent = |layout: &Value| {
                    let rects = as_object(field(layout, "positions"));
                    let far = |origin: &str, size: &str| {
                        rects
                            .values()
                            .map(|rect| num(field(rect, origin)) + num(field(rect, size)))
                            .fold(0.0_f64, f64::max)
                    };
                    (far("x", "width"), far("y", "height"))
                };
                let nested = match (
                    layout_nested_behavior(prepared, true, BehaviorMode::State)?,
                    layout_nested_behavior(prepared, false, BehaviorMode::State)?,
                ) {
                    (Some(across), Some(down)) => Some(
                        if reference_page_scale(extent(&down))
                            > reference_page_scale(extent(&across))
                        {
                            (down, false)
                        } else {
                            (across, true)
                        },
                    ),
                    _ => None,
                };
                let (behavior_layout, horizontal) = if let Some(nested) = nested {
                    nested
                } else if horizontal || direction.eq_ignore_ascii_case("vertical") {
                    let (layout, _) =
                        layout_behavior_graph(prepared, horizontal, BehaviorMode::State)?;
                    (layout, horizontal)
                } else {
                    let (across, across_size) =
                        layout_behavior_graph(prepared, true, BehaviorMode::State)?;
                    let (down, down_size) =
                        layout_behavior_graph(prepared, false, BehaviorMode::State)?;
                    // On a tie the horizontal layout wins, so the choice is deterministic.
                    if reference_page_scale(down_size) > reference_page_scale(across_size) {
                        (down, false)
                    } else {
                        (across, true)
                    }
                };
                let mut prepared = prepared.clone();
                prepared["meta"]["layoutDirection"] =
                    json!(if horizontal { "horizontal" } else { "vertical" });
                return Ok(json!({
                    "prepared": prepared,
                    "behaviorLayout": behavior_layout,
                }));
            }
            let behavior_layout = if let Some(nested) =
                layout_nested_behavior(prepared, horizontal, BehaviorMode::Action)?
            {
                nested
            } else if let Some(compact) = layout_simple_action_chain(prepared) {
                compact
            } else {
                layout_behavior_graph(prepared, horizontal, BehaviorMode::Action)?.0
            };
            Ok(json!({
                "prepared": prepared,
                "behaviorLayout": behavior_layout,
            }))
        }
        "interconnection-view" => layout_interconnection_prepared(prepared),
        _ => layout_general_prepared(prepared),
    }
}

/// The page a diagram is laid out to be readable on, in CSS pixels: the text area of an A4
/// sheet, upright (165 x 215 mm) or turned (245 x 135 mm). A diagram is placed on whichever of
/// the two shows it larger. Layout aims at this page so that a figure in a document keeps
/// legible text; on a screen the same compact layout simply needs less panning.
const REFERENCE_PAGE_UPRIGHT: (f64, f64) = (624.0, 813.0);
const REFERENCE_PAGE_TURNED: (f64, f64) = (926.0, 510.0);

/// The factor a diagram of `size` is scaled by to fit the reference page. A diagram is never
/// enlarged, so 1.0 means it fits at its drawn size and its text keeps its authored size.
fn reference_page_scale((width, height): (f64, f64)) -> f64 {
    let fit = |(page_width, page_height): (f64, f64)| {
        (page_width / width.max(1.0)).min(page_height / height.max(1.0))
    };
    fit(REFERENCE_PAGE_UPRIGHT)
        .max(fit(REFERENCE_PAGE_TURNED))
        .min(1.0)
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
            ("elk.spacing.nodeNode", "36"),
            ("elk.layered.spacing.nodeNodeBetweenLayers", "88"),
            ("elk.spacing.edgeNode", "24"),
            ("elk.spacing.edgeEdge", "14"),
            ("elk.layered.crossingMinimization.strategy", "LAYER_SWEEP"),
            ("elk.padding", "[top=20,left=20,bottom=20,right=20]"),
            ("org.eclipse.elk.portConstraints", "FIXED_ORDER"),
            ("org.eclipse.elk.portAlignment.default", "CENTER"),
        ],
        ElkViewKind::BehaviorState => &[
            ("elk.hierarchyHandling", "INCLUDE_CHILDREN"),
            ("elk.layered.crossingMinimization.strategy", "LAYER_SWEEP"),
            // Room for one orthogonal channel with its transition label between two layers,
            // and for an arrowhead between two states of a layer.
            // Follow the transitions from the initial state, so that the layers read in the
            // order a run passes through the states and only the returns go backwards.
            ("elk.layered.cycleBreaking.strategy", "DEPTH_FIRST"),
            ("elk.layered.spacing.nodeNodeBetweenLayers", "40"),
            ("elk.spacing.nodeNode", "28"),
            ("elk.spacing.edgeNode", "18"),
            ("elk.spacing.edgeEdge", "14"),
            ("elk.spacing.edgeLabel", "6"),
            ("elk.padding", "[top=48,left=24,bottom=24,right=24]"),
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

/// Lays out a general view that shows containment as a tree: boxes by `tree_layout`, aimed at
/// the reference page, with each containment line following the tree and every other
/// relationship drawn as a three-segment line between the two boxes.
fn hierarchy_general_layout(
    nodes: &[Value],
    edges: &[Value],
    parent_by_id: &HashMap<String, String>,
) -> Value {
    let order: Vec<String> = nodes
        .iter()
        .map(|node| as_string(field(node, "id"), ""))
        .collect();
    let measured: Vec<(f64, f64, crate::sysml_node::Compartments)> =
        nodes.iter().map(general_node_box).collect();
    let sizes: HashMap<String, (f64, f64)> = order
        .iter()
        .zip(&measured)
        .map(|(id, (width, height, _))| (id.clone(), (*width, *height)))
        .collect();
    let tree =
        crate::tree_layout::layout_tree_for(&order, &sizes, parent_by_id, reference_page_scale);
    let laid_out_nodes: Vec<Value> = nodes
        .iter()
        .zip(&measured)
        .map(|(node, (width, height, compartments))| {
            let (x, y) = tree
                .positions
                .get(&as_string(field(node, "id"), ""))
                .copied()
                .unwrap_or((0.0, 0.0));
            let mut out = node.clone();
            out["compartments"] = compartments_to_value(compartments);
            out["x"] = json!(x);
            out["y"] = json!(y);
            out["width"] = json!(*width);
            out["height"] = json!(*height);
            out
        })
        .collect();
    let rect = |id: &str| -> Option<(f64, f64, f64, f64)> {
        let (x, y) = tree.positions.get(id).copied()?;
        let (width, height) = sizes.get(id).copied()?;
        Some((x, y, width, height))
    };
    let section = |points: &[(f64, f64)]| -> Value {
        let point = |(x, y): &(f64, f64)| json!({ "x": x, "y": y });
        json!({
            "startPoint": point(&points[0]),
            "bendPoints": points[1..points.len() - 1].iter().map(point).collect::<Vec<_>>(),
            "endPoint": point(&points[points.len() - 1]),
        })
    };
    let laid_out_edges: Vec<Value> = edges
        .iter()
        .map(|edge| {
            let source = as_string(field(edge, "source"), "");
            let target = as_string(field(edge, "target"), "");
            // A containment line follows the tree, from the parent to the child.
            let along_tree = |parent: &str, child: &str| {
                (parent_by_id.get(child).map(String::as_str) == Some(parent))
                    .then(|| tree.routes.get(child))
                    .flatten()
            };
            let points: Vec<(f64, f64)> = if let Some(route) = along_tree(&source, &target) {
                route.clone()
            } else if let Some(route) = along_tree(&target, &source) {
                route.iter().rev().copied().collect()
            } else {
                let (Some(from), Some(to)) = (rect(&source), rect(&target)) else {
                    return edge.clone();
                };
                let from_x = from.0 + from.2 / 2.0;
                let to_x = to.0 + to.2 / 2.0;
                if (from.1 - to.1).abs() < 1.0 {
                    // Two boxes at the same height: over the top of both.
                    let lane = from.1 - 14.0;
                    vec![(from_x, from.1), (from_x, lane), (to_x, lane), (to_x, to.1)]
                } else {
                    let downward = to.1 > from.1;
                    let start = if downward { from.1 + from.3 } else { from.1 };
                    let end = if downward { to.1 } else { to.1 + to.3 };
                    let middle = (start + end) / 2.0;
                    vec![
                        (from_x, start),
                        (from_x, middle),
                        (to_x, middle),
                        (to_x, end),
                    ]
                }
            };
            let mut out = edge.clone();
            out["layout"] = json!({ "sections": [section(&points)] });
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
    // A container sized by the layout of its own content.
    if let [width, height] = as_array(field(field(node, "attributes"), "layoutSize")) {
        if let (Some(width), Some(height)) = (width.as_f64(), height.as_f64()) {
            return (width, height);
        }
    }
    let kind = node_kind(node);
    match mode {
        BehaviorMode::State => {
            if kind.contains("initial") || kind.contains("final") {
                (34.0, 34.0)
            } else if kind.contains("composite") {
                (340.0, 320.0)
            } else {
                state_box(node)
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
                // An action is high enough for the parameters on its fuller side.
                let attributes = field(node, "attributes");
                let rows = [
                    ["inputs", "inputParameters"],
                    ["outputs", "outputParameters"],
                ]
                .iter()
                .map(|keys| {
                    keys.iter()
                        .map(|key| as_array(field(attributes, key)).len())
                        .max()
                        .unwrap_or(0)
                })
                .max()
                .unwrap_or(0);
                (220.0, (68.0_f64).max(26.0 + rows as f64 * 14.0))
            }
        }
    }
}

/// The box of a simple state: as wide as its name and its `entry`/`do`/`exit` lines need, and as
/// high as the lines it shows. The drawing truncates the name at 28 and an action line at 34
/// characters, so the box never has to grow beyond those.
fn state_box(node: &Value) -> (f64, f64) {
    const NAME_CHARACTER: f64 = 7.4; // 12 px bold
    const ACTION_CHARACTER: f64 = 5.6; // 10 px
    let attributes = field(node, "attributes");
    let characters = |text: &str, limit: usize| text.chars().count().min(limit) as f64;
    let name = as_string(
        first_nonempty([field(node, "label"), field(node, "name")]),
        "",
    );
    let mut width = characters(&name, 28) * NAME_CHARACTER + 32.0;
    let mut actions = 0.0;
    for (prefix, keys) in [
        ("entry ", ["entry", "entryAction"]),
        ("do ", ["do", "doAction"]),
        ("exit ", ["exit", "exitAction"]),
    ] {
        let action = as_string(first_nonempty(keys.map(|key| field(attributes, key))), "");
        if !action.is_empty() {
            actions += 1.0;
            let line = format!("{prefix}{action}");
            width = width.max(characters(&line, 34) * ACTION_CHARACTER + 24.0);
        }
    }
    // The keyword and the name sit on a 38 px header; the label of the state actions
    // compartment and its lines follow on a 16 px pitch.
    let height = if actions == 0.0 {
        42.0
    } else {
        width = width.max(96.0);
        62.0 + actions * 16.0
    };
    (width.max(96.0).ceil(), height)
}

/// The room the names of an action's parameters on one side take beside it: the square on the
/// boundary, a gap, and the longest name in 8 px text. Zero for an action without them.
fn parameter_label_room(node: &Value, keys: [&str; 2]) -> f64 {
    let attributes = field(node, "attributes");
    keys.iter()
        .map(|key| as_array(field(attributes, key)))
        .find(|items| !items.is_empty())
        .unwrap_or_default()
        .iter()
        .map(|item| {
            as_string(
                first_nonempty([field(item, "name"), field(item, "label"), item]),
                "",
            )
            .trim()
            .encode_utf16()
            .count()
        })
        .max()
        .map_or(0.0, |characters| characters as f64 * 4.6 + 12.0)
}

/// Space inside an action container: the name compartment on top, then padding around the flow.
pub(crate) const ACTION_CONTAINER_HEADER: f64 = 44.0;
const ACTION_CONTAINER_PADDING: f64 = 28.0;

/// The laid-out content of one action flow level, relative to its own origin.
struct ActionFlowLevel {
    positions: Map<String, Value>,
    sections: Map<String, Value>,
    labels: Map<String, Value>,
    /// Edges whose label another edge of their group carries (`shared_transition_ports`).
    shared_labels: Vec<Value>,
    /// Extent of the content, `(min_x, min_y, max_x, max_y)`.
    bounds: (f64, f64, f64, f64),
    /// Room above this content in the container that holds it (`container_header`).
    header: f64,
}

/// The room at the top of a container for its own name compartment: fixed for an action, and
/// for a state what the state alone would draw, its name and its state actions.
fn container_header(node: &Value, mode: BehaviorMode) -> f64 {
    match mode {
        BehaviorMode::Action => ACTION_CONTAINER_HEADER,
        BehaviorMode::State => state_box(node).1.max(42.0) - 4.0,
    }
}

/// An action flow in which some actions own other actions (`attributes.containerId`). Each such
/// action is a container holding its own action flow: every level is laid out on its own, innermost
/// first, and a container is then sized to fit its content. A succession or flow is routed at the
/// level where both ends are; one that crosses a container boundary gets the renderer's fallback
/// route. Returns `None` when no action contains another.
///
/// A state machine in which states own states is laid out the same way.
fn layout_nested_behavior(
    prepared: &Value,
    horizontal: bool,
    mode: BehaviorMode,
) -> Result<Option<Value>, diagram_layout::LayoutError> {
    let nodes = as_array(field(prepared, "nodes"));
    let ids: HashSet<String> = nodes
        .iter()
        .map(|node| as_string(field(node, "id"), ""))
        .collect();
    let mut children: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
    let mut roots = Vec::new();
    for node in nodes {
        let container = as_string(field(field(node, "attributes"), "containerId"), "");
        if !container.is_empty() && ids.contains(&container) {
            children.entry(container).or_default().push(node);
        } else {
            roots.push(node);
        }
    }
    if children.is_empty() {
        return Ok(None);
    }

    fn lay_out_level(
        prepared: &Value,
        members: &[&Value],
        children: &BTreeMap<String, Vec<&Value>>,
        horizontal: bool,
        mode: BehaviorMode,
        levels: &mut BTreeMap<String, ActionFlowLevel>,
        depth: usize,
    ) -> Result<ActionFlowLevel, diagram_layout::LayoutError> {
        let mut level_nodes = Vec::with_capacity(members.len());
        for member in members {
            let id = as_string(field(member, "id"), "");
            let mut node = (*member).clone();
            // `depth` bounds the recursion should `containerId` ever form a cycle.
            if let Some(inner) = children.get(&id).filter(|_| depth <= children.len()) {
                let mut content = lay_out_level(
                    prepared,
                    inner,
                    children,
                    horizontal,
                    mode,
                    levels,
                    depth + 1,
                )?;
                content.header = container_header(member, mode);
                let (min_x, min_y, max_x, max_y) = content.bounds;
                let (own_width, _) = node_dimensions(member, mode);
                node["attributes"]["layoutSize"] = json!([
                    ((max_x - min_x) + 2.0 * ACTION_CONTAINER_PADDING).max(own_width),
                    (max_y - min_y) + content.header + 2.0 * ACTION_CONTAINER_PADDING,
                ]);
                levels.insert(id, content);
            }
            level_nodes.push(node);
        }
        let member_ids: HashSet<String> = members
            .iter()
            .map(|member| as_string(field(member, "id"), ""))
            .collect();
        let level_edges: Vec<Value> = as_array(field(prepared, "edges"))
            .iter()
            .filter(|edge| {
                member_ids.contains(&as_string(field(edge, "source"), ""))
                    && member_ids.contains(&as_string(field(edge, "target"), ""))
            })
            .cloned()
            .collect();
        let level = json!({
            "title": field(prepared, "title"),
            "view": field(prepared, "view"),
            "nodes": level_nodes,
            "edges": level_edges,
            "meta": field(prepared, "meta"),
        });
        let chain = match mode {
            BehaviorMode::Action => layout_simple_action_chain(&level),
            BehaviorMode::State => None,
        };
        let laid_out = match chain {
            Some(compact) => compact,
            None if level_nodes.len() == 1 => json!({
                "positions": {
                    as_string(field(&level_nodes[0], "id"), ""): {
                        "x": 0.0, "y": 0.0,
                        "width": node_dimensions(&level_nodes[0], mode).0,
                        "height": node_dimensions(&level_nodes[0], mode).1,
                    }
                },
            }),
            None => layout_behavior_graph(&level, horizontal, mode)?.0,
        };
        let shared_labels = as_array(field(&laid_out, "sharedLabelEdgeIds")).to_vec();
        let positions = as_object(field(&laid_out, "positions"));
        let sections = as_object(field(&laid_out, "edgeSectionsById"));
        let labels = as_object(field(&laid_out, "edgeLabelsById"));
        let mut bounds = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
        let mut include = |x: f64, y: f64| {
            bounds.0 = bounds.0.min(x);
            bounds.1 = bounds.1.min(y);
            bounds.2 = bounds.2.max(x);
            bounds.3 = bounds.3.max(y);
        };
        for (id, rect) in positions.iter() {
            let (x, y) = (num(field(rect, "x")), num(field(rect, "y")));
            // The names of an action's parameters stand outside it, beside their squares.
            let member = members
                .iter()
                .find(|member| as_string(field(member, "id"), "") == *id);
            let (before, after) = member.map_or((0.0, 0.0), |member| {
                (
                    parameter_label_room(member, ["inputs", "inputParameters"]),
                    parameter_label_room(member, ["outputs", "outputParameters"]),
                )
            });
            include(x - before, y);
            include(
                x + num(field(rect, "width")) + after,
                y + num(field(rect, "height")),
            );
        }
        for section in sections.values().flat_map(as_array) {
            for point in [field(section, "startPoint"), field(section, "endPoint")]
                .into_iter()
                .chain(as_array(field(section, "bendPoints")))
            {
                include(num(field(point, "x")), num(field(point, "y")));
            }
        }
        for label in labels.values().flat_map(as_array) {
            let (x, y) = (num(field(label, "x")), num(field(label, "y")));
            include(x, y);
            include(
                x + num(field(label, "width")),
                y + num(field(label, "height")),
            );
        }
        if bounds.0 > bounds.2 {
            bounds = (0.0, 0.0, 0.0, 0.0);
        }
        Ok(ActionFlowLevel {
            positions,
            sections,
            labels,
            shared_labels,
            bounds,
            header: 0.0,
        })
    }

    /// Moves a level to `(dx, dy)` and each of its containers' content into that container.
    fn place(
        level: ActionFlowLevel,
        dx: f64,
        dy: f64,
        levels: &mut BTreeMap<String, ActionFlowLevel>,
        out: &mut ActionFlowLevel,
    ) {
        let shift = |point: &Value| json!({"x": num(field(point, "x")) + dx, "y": num(field(point, "y")) + dy});
        for (id, sections) in level.sections {
            let moved: Vec<Value> = as_array(&sections)
                .iter()
                .map(|section| {
                    let mut moved = section.clone();
                    moved["startPoint"] = shift(field(section, "startPoint"));
                    moved["endPoint"] = shift(field(section, "endPoint"));
                    moved["bendPoints"] = Value::Array(
                        as_array(field(section, "bendPoints"))
                            .iter()
                            .map(shift)
                            .collect(),
                    );
                    moved
                })
                .collect();
            out.sections.insert(id, Value::Array(moved));
        }
        out.shared_labels.extend(level.shared_labels);
        for (id, labels) in level.labels {
            let moved: Vec<Value> = as_array(&labels)
                .iter()
                .map(|label| {
                    let mut moved = label.clone();
                    moved["x"] = json!(num(field(label, "x")) + dx);
                    moved["y"] = json!(num(field(label, "y")) + dy);
                    moved
                })
                .collect();
            out.labels.insert(id, Value::Array(moved));
        }
        for (id, rect) in level.positions {
            let (x, y) = (num(field(&rect, "x")) + dx, num(field(&rect, "y")) + dy);
            if let Some(content) = levels.remove(&id) {
                let (min_x, min_y, _, _) = content.bounds;
                let header = content.header;
                place(
                    content,
                    x + ACTION_CONTAINER_PADDING - min_x,
                    y + header + ACTION_CONTAINER_PADDING - min_y,
                    levels,
                    out,
                );
            }
            let mut moved = rect;
            moved["x"] = json!(x);
            moved["y"] = json!(y);
            out.positions.insert(id, moved);
        }
    }

    let mut levels = BTreeMap::new();
    let top = lay_out_level(
        prepared,
        &roots,
        &children,
        horizontal,
        mode,
        &mut levels,
        0,
    )?;
    let (min_x, min_y, _, _) = top.bounds;
    let mut out = ActionFlowLevel {
        positions: Map::new(),
        sections: Map::new(),
        labels: Map::new(),
        shared_labels: Vec::new(),
        bounds: (0.0, 0.0, 0.0, 0.0),
        header: 0.0,
    };
    place(top, 80.0 - min_x, 80.0 - min_y, &mut levels, &mut out);
    Ok(Some(json!({
        "positions": out.positions,
        "edgeSectionsById": out.sections,
        "edgeLabelsById": out.labels,
        "sharedLabelEdgeIds": out.shared_labels,
    })))
}

/// A simple authored succession chain is easier to read in short alternating rows than in a
/// single, very tall ELK column. Other action graphs keep ELK's branching and cycle layout.
fn layout_simple_action_chain(prepared: &Value) -> Option<Value> {
    if field(field(prepared, "meta"), "typedProjection") != &json!(true) {
        return None;
    }
    let nodes = as_array(field(prepared, "nodes"));
    let edges = as_array(field(prepared, "edges"));
    if nodes.len() < 3 || edges.len() + 1 != nodes.len() {
        return None;
    }
    let mut by_id = HashMap::new();
    for node in nodes {
        if !as_string(field(field(node, "attributes"), "swimLane"), "").is_empty() {
            return None;
        }
        let id = as_string(field(node, "id"), "");
        if id.is_empty() || by_id.insert(id, node).is_some() {
            return None;
        }
    }
    let mut next: HashMap<String, (String, String)> = HashMap::new();
    let mut incoming = HashSet::new();
    for edge in edges {
        if field(field(edge, "attributes"), "succession") != &json!(true) {
            return None;
        }
        let source = as_string(field(edge, "source"), "");
        let target = as_string(field(edge, "target"), "");
        let id = as_string(field(edge, "id"), "");
        if !by_id.contains_key(&source)
            || !by_id.contains_key(&target)
            || id.is_empty()
            || next.insert(source, (target.clone(), id)).is_some()
            || !incoming.insert(target)
        {
            return None;
        }
    }
    let mut starts = by_id.keys().filter(|id| !incoming.contains(*id));
    let start = starts.next()?.clone();
    if starts.next().is_some() {
        return None;
    }
    let mut ordered = Vec::with_capacity(nodes.len());
    let mut current = start;
    let mut visited = HashSet::new();
    loop {
        if !visited.insert(current.clone()) {
            return None;
        }
        ordered.push(current.clone());
        let Some((target, _)) = next.get(&current) else {
            break;
        };
        current = target.clone();
    }
    if ordered.len() != nodes.len() {
        return None;
    }

    let cell_width = nodes
        .iter()
        .map(|node| node_dimensions(node, BehaviorMode::Action).0)
        .fold(0.0_f64, f64::max);
    let cell_height = nodes
        .iter()
        .map(|node| node_dimensions(node, BehaviorMode::Action).1)
        .fold(0.0_f64, f64::max);
    if nodes
        .iter()
        .any(|node| node_dimensions(node, BehaviorMode::Action) != (cell_width, cell_height))
    {
        return None;
    }
    // The gap between two actions holds a visible run of succession, its label when it has
    // one, and the names of the parameters on the facing sides. It follows what is drawn, so
    // a flow of plain steps is not spread over a fixed pitch.
    let longest = |texts: &mut dyn Iterator<Item = String>, limit: usize| {
        texts
            .map(|text| text.trim().encode_utf16().count().min(limit))
            .max()
            .unwrap_or(0) as f64
    };
    let succession_label = longest(
        &mut edges.iter().map(|edge| as_string(field(edge, "label"), "")),
        usize::MAX,
    );
    let parameter_room = |keys: [&str; 2]| {
        nodes
            .iter()
            .map(|node| parameter_label_room(node, keys))
            .fold(0.0_f64, f64::max)
    };
    let gap_x = (56.0_f64).max(succession_label * 6.0 + 36.0).max(
        parameter_room(["outputs", "outputParameters"])
            + parameter_room(["inputs", "inputParameters"])
            + 32.0,
    );
    let gap_y = if succession_label == 0.0 { 56.0 } else { 76.0 };
    let columns = (1..=nodes.len()).min_by(|&left, &right| {
        let score = |columns: usize| {
            let rows = nodes.len().div_ceil(columns);
            let width = columns as f64 * cell_width + (columns - 1) as f64 * gap_x;
            let height = rows as f64 * cell_height + (rows - 1) as f64 * gap_y;
            (width / height / 1.5).ln().abs()
        };
        score(left).total_cmp(&score(right)).then(left.cmp(&right))
    })?;
    let mut positions = Map::new();
    for (index, id) in ordered.iter().enumerate() {
        let row = index / columns;
        let offset = index % columns;
        let column = if row % 2 == 0 {
            offset
        } else {
            columns - 1 - offset
        };
        let (width, height) = node_dimensions(by_id[id], BehaviorMode::Action);
        positions.insert(
            id.clone(),
            json!({
                "x": 80.0 + column as f64 * (cell_width + gap_x) + (cell_width - width) / 2.0,
                "y": 80.0 + row as f64 * (cell_height + gap_y) + (cell_height - height) / 2.0,
                "width": width,
                "height": height,
            }),
        );
    }
    let mut sections = Map::new();
    for pair in ordered.windows(2) {
        let source = field(&positions[&pair[0]], "x").as_f64()?;
        let target = field(&positions[&pair[1]], "x").as_f64()?;
        let source_y = field(&positions[&pair[0]], "y").as_f64()?;
        let target_y = field(&positions[&pair[1]], "y").as_f64()?;
        let source_width = field(&positions[&pair[0]], "width").as_f64()?;
        let source_height = field(&positions[&pair[0]], "height").as_f64()?;
        let target_width = field(&positions[&pair[1]], "width").as_f64()?;
        let (start, end) = if (source_y - target_y).abs() < 1.0 {
            if source < target {
                (
                    (source + source_width, source_y + source_height / 2.0),
                    (target, target_y + source_height / 2.0),
                )
            } else {
                (
                    (source, source_y + source_height / 2.0),
                    (target + target_width, target_y + source_height / 2.0),
                )
            }
        } else {
            (
                (source + source_width / 2.0, source_y + source_height),
                (target + target_width / 2.0, target_y),
            )
        };
        let edge_id = &next[&pair[0]].1;
        sections.insert(
            edge_id.clone(),
            json!([{
                "startPoint": {"x": start.0, "y": start.1},
                "endPoint": {"x": end.0, "y": end.1},
            }]),
        );
    }
    Some(json!({
        "positions": positions,
        "edgeSectionsById": sections,
        "edgeLabelsById": {},
    }))
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

/// Transitions of a state machine that share a label and a target, when there are three or more:
/// each edge id with the id of the port of the target its group runs into, and whether it is
/// the one edge of its group that carries the label. A machine in which every state reacts to
/// the same trigger in the same way is then drawn with one labelled line into the target, not
/// with a labelled line from every state.
/// Two transitions with one label read well as two labelled lines; from three on, the labels
/// crowd out the lines.
const SHARED_TRANSITION_MINIMUM: usize = 3;

fn shared_transition_ports(prepared: &Value) -> BTreeMap<String, (String, bool)> {
    let mut groups: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for edge in as_array(field(prepared, "edges")) {
        let label = transition_display_label(&as_string(field(edge, "label"), ""));
        let source = as_string(field(edge, "source"), "");
        let target = as_string(field(edge, "target"), "");
        if label.is_empty() || source == target {
            continue;
        }
        groups
            .entry((target, label))
            .or_default()
            .push(as_string(field(edge, "id"), ""));
    }
    let mut shared = BTreeMap::new();
    for (index, ((target, _), edges)) in groups
        .into_iter()
        .filter(|(_, edges)| edges.len() >= SHARED_TRANSITION_MINIMUM)
        .enumerate()
    {
        let port = format!("{target}::shared-transition-{index}");
        for (position, edge) in edges.into_iter().enumerate() {
            shared.insert(edge, (port.clone(), position == 0));
        }
    }
    shared
}

fn build_behavior_elk_graph(prepared: &Value, horizontal: bool, mode: BehaviorMode) -> Value {
    let shared = match mode {
        BehaviorMode::State => shared_transition_ports(prepared),
        BehaviorMode::Action => BTreeMap::new(),
    };
    let children: Vec<Value> = as_array(field(prepared, "nodes"))
        .iter()
        .map(|node| {
            let (width, height) = node_dimensions(node, mode);
            let mut child = json!({
                "id": field(node, "id"),
                "width": width,
                "height": height,
            });
            let node_id = as_string(field(node, "id"), "");
            let ports: BTreeSet<&String> = shared
                .values()
                .map(|(port, _)| port)
                .filter(|port| port.starts_with(&format!("{node_id}::shared-transition-")))
                .collect();
            if !ports.is_empty() {
                child["ports"] = ports
                    .into_iter()
                    .map(|port| json!({ "id": port, "width": 1, "height": 1 }))
                    .collect();
            }
            // A state machine reads from its initial state, so that state leads the layout
            // whatever cycles the transitions form.
            if matches!(mode, BehaviorMode::State) && node_kind(node).contains("initial") {
                child["layoutOptions"] = json!({ "elk.layered.layering.layerConstraint": "FIRST" });
            }
            child
        })
        .collect();
    let edges: Vec<Value> = as_array(field(prepared, "edges"))
        .iter()
        .map(|edge| {
            let display_label = transition_display_label(&as_string(field(edge, "label"), ""));
            let group = shared.get(&as_string(field(edge, "id"), ""));
            let mut base = json!({
                "id": field(edge, "id"),
                "sources": [field(edge, "source")],
                "targets": [group.map_or_else(|| field(edge, "target").clone(), |(port, _)| json!(port))],
            });
            if display_label.is_empty() || group.is_some_and(|(_, labelled)| !labelled) {
                return base;
            }
            let id = format!("{}::label", as_string(field(edge, "id"), ""));
            let mut label_box = estimate_elk_label_box(&id, &display_label);
            label_box["layoutOptions"] = json!({
                // The label of a shared line sits at its target, where the lines have joined.
                "org.eclipse.elk.edgeLabels.placement": if group.is_some() { "HEAD" } else { "CENTER" },
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
    // The gaps of an action flow follow what stands in them: the longest label of a flow or
    // succession between two layers, and the names of parameters beside an action.
    let longest_label = as_array(field(prepared, "edges"))
        .iter()
        .map(|edge| {
            transition_display_label(&as_string(field(edge, "label"), ""))
                .encode_utf16()
                .count()
        })
        .max()
        .unwrap_or(0) as f64;
    let has_parameters = as_array(field(prepared, "nodes")).iter().any(|node| {
        let attributes = field(node, "attributes");
        ["inputs", "inputParameters", "outputs", "outputParameters"]
            .iter()
            .any(|key| !as_array(field(attributes, key)).is_empty())
    });
    let beside_parameters = if has_parameters {
        let room = |keys: [&str; 2]| {
            as_array(field(prepared, "nodes"))
                .iter()
                .map(|node| parameter_label_room(node, keys))
                .fold(0.0_f64, f64::max)
        };
        room(["outputs", "outputParameters"]) + room(["inputs", "inputParameters"]) + 32.0
    } else {
        40.0
    };
    let (action_layer_gap, action_node_gap) = if horizontal {
        (
            (64.0_f64)
                .max(longest_label * 6.0 + 48.0)
                .max(beside_parameters),
            40.0,
        )
    } else {
        (
            if longest_label == 0.0 { 64.0 } else { 84.0 },
            (40.0_f64).max(beside_parameters),
        )
    };
    let (action_layer_gap, action_node_gap) =
        (action_layer_gap.to_string(), action_node_gap.to_string());
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
            ("elk.spacing.nodeNode", Some(action_node_gap.as_str())),
            (
                "elk.layered.spacing.nodeNodeBetweenLayers",
                Some(action_layer_gap.as_str()),
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

/// Lays out a behaviour graph; returns the layout and the size of the laid-out graph.
fn layout_behavior_graph(
    prepared: &Value,
    horizontal: bool,
    mode: BehaviorMode,
) -> Result<(Value, (f64, f64)), diagram_layout::LayoutError> {
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
    let size = (
        num(field(&laid_out, "width")),
        num(field(&laid_out, "height")),
    );
    Ok((
        json!({
            "positions": positions,
            "edgeSectionsById": edge_sections,
            "edgeLabelsById": edge_labels,
            "sharedLabelEdgeIds": match mode {
                BehaviorMode::State => shared_transition_ports(prepared)
                    .into_iter()
                    .filter(|(_, (_, labelled))| !labelled)
                    .map(|(edge, _)| edge)
                    .collect::<Vec<_>>(),
                BehaviorMode::Action => Vec::new(),
            },
        }),
        size,
    ))
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

/// Spreads the children of every container of an ELK graph over several layers when too many of
/// them would share one.
///
/// A layered layout puts a part in the layer its connectors to its siblings call for. Parts that
/// connect only to the boundary of their container, or to nothing, all land in the first layer
/// and stack into one tall column. Here each layer the sibling connectors call for is split into
/// columns of at most about the square root of the number of siblings, and the order of the
/// columns is given to ELK as partitions. Connectors between siblings keep their direction:
/// a part never moves in front of a sibling that feeds it.
///
/// The pinned `elkrs` reads `elk.partitioning.activate` on the top-level graph only, so today
/// this spreads the roots of a view and not yet the parts inside one container.
fn spread_crowded_layers(graph: &mut Value) {
    let edges: Vec<(String, String)> = as_array(field(graph, "edges"))
        .iter()
        .filter_map(|edge| {
            Some((
                as_array(field(edge, "sources"))
                    .first()?
                    .as_str()?
                    .to_owned(),
                as_array(field(edge, "targets"))
                    .first()?
                    .as_str()?
                    .to_owned(),
            ))
        })
        .collect();
    fn collect_ids(node: &Value, ids: &mut Vec<String>) {
        ids.push(as_string(field(node, "id"), ""));
        for port in as_array(field(node, "ports")) {
            ids.push(as_string(field(port, "id"), ""));
        }
        for child in as_array(field(node, "children")) {
            collect_ids(child, ids);
        }
    }
    fn spread(node: &mut Value, edges: &[(String, String)]) {
        let count = as_array(field(node, "children")).len();
        // Which child each node or port below this container belongs to.
        let mut child_of: HashMap<String, usize> = HashMap::new();
        for (index, child) in as_array(field(node, "children")).iter().enumerate() {
            let mut ids = Vec::new();
            collect_ids(child, &mut ids);
            for id in ids {
                child_of.insert(id, index);
            }
        }
        let mut feeds: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); count];
        for (source, target) in edges {
            if let (Some(&from), Some(&to)) = (child_of.get(source), child_of.get(target)) {
                if from != to {
                    feeds[from].insert(to);
                }
            }
        }
        // The layer the sibling connectors call for: the longest chain of siblings that leads
        // to the child. A cycle stops counting after as many steps as there are children.
        let mut layer = vec![0usize; count];
        for _ in 0..count {
            let mut changed = false;
            for from in 0..count {
                for &to in &feeds[from] {
                    if layer[to] < layer[from] + 1 && layer[from] + 1 < count {
                        layer[to] = layer[from] + 1;
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        let column = ((count as f64).sqrt().ceil() as usize).max(3);
        let layers = layer.iter().copied().max().map_or(0, |deepest| deepest + 1);
        let crowded =
            (0..layers).any(|wanted| layer.iter().filter(|l| **l == wanted).count() > column);
        if crowded {
            let mut partition = vec![0usize; count];
            let mut next = 0;
            for wanted in 0..layers {
                let members: Vec<usize> =
                    (0..count).filter(|child| layer[*child] == wanted).collect();
                for chunk in members.chunks(column) {
                    for child in chunk {
                        partition[*child] = next;
                    }
                    next += 1;
                }
            }
            node["layoutOptions"]["elk.partitioning.activate"] = json!("true");
            if let Some(children) = node.get_mut("children").and_then(Value::as_array_mut) {
                for (child, partition) in children.iter_mut().zip(partition) {
                    child["layoutOptions"]["elk.partitioning.partition"] =
                        json!(partition.to_string());
                }
            }
        }
        if let Some(children) = node.get_mut("children").and_then(Value::as_array_mut) {
            for child in children {
                spread(child, edges);
            }
        }
    }
    spread(graph, &edges);
}

/// The part that contains `node` in a prepared interconnection view, when it has one.
fn container_of_node(node: &Value) -> Option<String> {
    match field(field(node, "attributes"), "containerId") {
        Value::String(id) if !id.is_empty() => Some(id.clone()),
        _ => None,
    }
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
        // A port of the outermost part is only used from inside: what it feeds lies to its
        // right, so it sits on the left, and what feeds it lies to its left.
        let used_from_inside = container_of_node(node).is_none()
            && children_by_parent.contains_key(&as_string(field(node, "id"), ""));
        let (toward_right, toward_left) = if used_from_inside {
            (usage.source_count, usage.target_count)
        } else {
            (usage.target_count, usage.source_count)
        };
        if toward_right > toward_left {
            return "WEST";
        }
        if toward_left > toward_right {
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
        let content_height = if content_line_count == 0 {
            0.0
        } else {
            (content_line_count.min(8) as f64) * 12.0 + 10.0
        };
        // A port row is the 10 px port with its label and the gap to the next one.
        let ports_height = if ports.is_empty() {
            0.0
        } else {
            port_rows as f64 * 16.0 + 12.0
        };
        // The note that inner parts are not shown takes one line at the bottom.
        let note_height = if field(attrs, "closedParts").as_u64().unwrap_or(0) > 0 {
            14.0
        } else {
            0.0
        };
        (header_height + content_height + ports_height + note_height + 8.0).clamp(56.0, 340.0)
    }

    // Two parts that face each other are far enough apart for a visible run of connector, the
    // labels of their facing ports, and the longest connector name set beside its line with
    // clear space to both parts. Unnamed connectors need only the first two: the layout adds
    // the width of the port labels to this gap, and they sit beside the line, not on it.
    let layer_gap = {
        const GENERIC: [&str; 7] = [
            "",
            "connect",
            "connection",
            "connector",
            "flow",
            "binding",
            "interface",
        ];
        let longest_name = as_array(field(prepared, "edges"))
            .iter()
            .map(|edge| as_string(field(edge, "label"), ""))
            .filter(|label| !GENERIC.contains(&label.trim().to_ascii_lowercase().as_str()))
            .map(|label| label.trim().encode_utf16().count().min(18))
            .max();
        match longest_name {
            Some(characters) => (characters as f64 * 6.5 + 10.0 + 44.0).max(88.0),
            None => 48.0,
        }
    };
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
        layer_gap: f64,
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
                            layer_gap,
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();
        let attrs = field(node, "attributes");
        let is_synthetic_package = field(attrs, "isSyntheticPackage") == &json!(true);
        let is_container =
            field(attrs, "isSyntheticContainer") == &json!(true) || !children.is_empty();
        // A part is as wide as its name and its type need; both are written in full.
        let characters = |text: &str| text.encode_utf16().count() as f64;
        let label_width = characters(&as_string(field(node, "label"), "")) * 6.8;
        let type_width = (characters(&as_string(field(attrs, "partType"), "")) + 2.0) * 5.4;
        let base_width: f64 = if is_container { 220.0 } else { 132.0 };
        // The expand control sits left of the centred name, which keeps clear of it on both sides.
        let control_reserve = if field(attrs, "disclosure").is_string() {
            2.0 * crate::sysml_node::DISCLOSURE_TARGET_SIZE
        } else {
            0.0
        };
        // Port labels sit outside the part, so they do not make it wider.
        let width = base_width
            .max(label_width.max(type_width) + 28.0 + control_reserve)
            .ceil();
        let mut height = if is_container {
            root_header_height + 72.0
        } else {
            compute_ibd_leaf_height(node, &ports, port_rows)
        };
        if is_container && !children.is_empty() {
            // A container is as large as the layout of its parts makes it; this is only the
            // least it needs for its own name and boundary ports.
            height = if is_synthetic_package {
                root_header_height + 72.0
            } else {
                root_header_height
                    + 72.0_f64.max(132.0_f64.min(58.0 + children.len() as f64 * 14.0))
            };
        }
        let ports_top = if is_container {
            container_top_inset
        } else if as_string(field(attrs, "partType"), "").is_empty() {
            38.0
        } else {
            50.0
        };
        let build_elk_port = |port: &PreparedPort, side: &str, index: usize| {
            let id = port_id_for(&as_string(field(node, "id"), ""), &port.name);
            let label_text = ibd_port_detail_label(&port.raw);
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
            "org.eclipse.elk.portLabels.placement": "OUTSIDE",
            "org.eclipse.elk.spacing.labelPortHorizontal": "6",
            "org.eclipse.elk.spacing.labelPortVertical": "4",
            // Ports start below the name and type of the part, never on top of them.
            "org.eclipse.elk.spacing.portsSurrounding":
                format!("[top={ports_top},left=0,bottom=6,right=0]"),
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
            // A gap leaves room for the connector channels between two parts and for the labels
            // of the boundary ports that face each other.
            // Two parts that face each other keep 88 px between them: the labels of their ports
            // sit outside the boxes, and a connector needs a visible run and room for its name.
            options.insert("elk.spacing.nodeNode".into(), json!("36"));
            options.insert(
                "elk.layered.spacing.nodeNodeBetweenLayers".into(),
                json!(layer_gap.to_string()),
            );
            options.insert("elk.spacing.edgeNode".into(), json!("24"));
            options.insert("elk.spacing.edgeEdge".into(), json!("14"));
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
            layer_gap,
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

    let mut elk_graph_input = json!({
        "id": "root",
        "layoutOptions": build_elk_layout_options(
            ElkViewKind::Interconnection,
            &[
                (
                    "elk.layered.spacing.nodeNodeBetweenLayers",
                    Some(&layer_gap.to_string()),
                ),
            ],
        ),
        "children": elk_children,
        "edges": elk_edges.iter().map(|edge| json!({
            "id": field(edge, "id"),
            "sources": field(edge, "sources"),
            "targets": field(edge, "targets"),
        })).collect::<Vec<_>>(),
    });

    spread_crowded_layers(&mut elk_graph_input);
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

#[cfg(test)]
mod crowded_layer_tests {
    use super::*;

    fn partitions(graph: &Value) -> Vec<Option<String>> {
        as_array(field(graph, "children"))
            .iter()
            .map(|child| {
                field(field(child, "layoutOptions"), "elk.partitioning.partition")
                    .as_str()
                    .map(str::to_owned)
            })
            .collect()
    }

    fn graph(children: usize, edges: &[(&str, &str)]) -> Value {
        json!({
            "id": "root",
            "children": (0..children).map(|i| json!({ "id": format!("n{i}") })).collect::<Vec<_>>(),
            "edges": edges
                .iter()
                .map(|(from, to)| json!({ "sources": [from], "targets": [to] }))
                .collect::<Vec<_>>(),
        })
    }

    #[test]
    fn unconnected_siblings_are_spread_over_columns() {
        let mut crowded = graph(10, &[]);
        spread_crowded_layers(&mut crowded);
        let columns: Vec<String> = partitions(&crowded).into_iter().flatten().collect();
        // Ten siblings: columns of four, in order.
        assert_eq!(columns, ["0", "0", "0", "0", "1", "1", "1", "1", "2", "2"]);
        assert_eq!(
            field(
                field(&crowded, "layoutOptions"),
                "elk.partitioning.activate"
            ),
            &json!("true")
        );
    }

    #[test]
    fn a_sibling_never_moves_in_front_of_one_that_feeds_it() {
        // n0 feeds n9, so n9 belongs to a later layer than every column of the first.
        let mut crowded = graph(10, &[("n0", "n9")]);
        spread_crowded_layers(&mut crowded);
        let columns: Vec<usize> = partitions(&crowded)
            .into_iter()
            .map(|partition| partition.unwrap().parse().unwrap())
            .collect();
        assert!(columns[9] > columns[0]);
        assert_eq!(columns[9], *columns.iter().max().unwrap());
    }

    #[test]
    fn a_layout_that_is_not_crowded_is_left_alone() {
        let mut small = graph(3, &[]);
        let before = small.clone();
        spread_crowded_layers(&mut small);
        assert_eq!(small, before);
    }

    #[test]
    fn transitions_with_one_label_and_target_share_a_line_from_three_on() {
        let edge = |id: &str, source: &str, target: &str, label: &str| json!({ "id": id, "source": source, "target": target, "label": label });
        let nodes =
            ["a", "b", "c", "home"].map(|id| json!({ "id": id, "label": id, "kind": "state" }));
        let prepared = json!({
            "nodes": nodes,
            "edges": [
                edge("a-home", "a", "home", "Reset"),
                edge("b-home", "b", "home", "Reset"),
                edge("c-home", "c", "home", "Reset"),
                edge("a-b", "a", "b", "Go"),
                edge("c-b", "c", "b", "Go"),
                edge("home-home", "home", "home", "Reset"),
            ],
        });
        let shared = shared_transition_ports(&prepared);
        // The three into `home` share one port and one of them carries the label. Two with
        // one label keep their own lines, and a self-loop is never part of a group.
        assert_eq!(shared.len(), 3);
        let ports: BTreeSet<&String> = shared.values().map(|(port, _)| port).collect();
        assert_eq!(ports.len(), 1);
        assert_eq!(shared.values().filter(|(_, labelled)| *labelled).count(), 1);

        let graph = build_behavior_elk_graph(&prepared, true, BehaviorMode::State);
        let port = ports.into_iter().next().unwrap();
        let home = &as_array(field(&graph, "children"))[3];
        assert_eq!(
            field(&as_array(field(home, "ports"))[0], "id"),
            &json!(port)
        );
        let labelled = as_array(field(&graph, "edges"))
            .iter()
            .filter(|edge| field(edge, "targets") == &json!([port]))
            .filter(|edge| !as_array(field(edge, "labels")).is_empty())
            .count();
        assert_eq!(labelled, 1);
        // In an action flow nothing is shared.
        let graph = build_behavior_elk_graph(&prepared, true, BehaviorMode::Action);
        assert!(as_array(field(&graph, "edges"))
            .iter()
            .all(|edge| field(edge, "targets")
                .to_string()
                .matches("shared-transition")
                .count()
                == 0));
    }
}
