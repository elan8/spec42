//! Port of `views/action-flow.ts`.

use std::collections::HashMap;

use serde_json::Value;

use crate::behavior_common::{
    edge_label_position_from_sections, fallback_edge_path, node_kind, path_from_sections,
    truncate_label, BehaviorLayoutResult, PreparedNode, PreparedView,
};
use crate::hit_target::{mark_visible_edge, path_edge_hit_target};
use crate::svg::{format_number as n, Element};
use crate::theme::Theme;
use crate::tooltip::{behavior_edge_tooltip_descriptor, tooltip_fallback_text};
use crate::types::attr_text;

fn is_initial(kind: &str) -> bool {
    kind.contains("initial") || kind.contains("start")
}
fn is_final(kind: &str) -> bool {
    // `send` contains `end`, and a send action is not a final node.
    kind.contains("final")
        || kind.contains("done")
        || (kind.contains("end") && !kind.contains("send"))
}
fn is_flow_final(kind: &str) -> bool {
    kind.contains("flow-final") || kind.contains("flow final") || kind.contains("terminate")
}
fn is_decision(kind: &str) -> bool {
    kind.contains("decision") || kind.contains("merge")
}
fn is_fork(kind: &str) -> bool {
    kind.contains("fork") || kind.contains("join")
}

/// Port of `activityNodeKind`: prefers `attrs.stateType`/`attrs.kind` over the node's own `kind`.
fn activity_node_kind(node: &PreparedNode) -> String {
    let typed = attr_text(&node.attributes, "stateType");
    let typed = if !typed.is_empty() {
        typed
    } else {
        attr_text(&node.attributes, "kind")
    };
    let typed = if !typed.is_empty() {
        typed
    } else {
        node.kind.clone()
    };
    let typed = typed.to_lowercase();
    if !typed.is_empty() {
        typed
    } else {
        node_kind(node)
    }
}

fn attr_array<'a>(
    attributes: &'a std::collections::BTreeMap<String, Value>,
    primary: &str,
    alias: &str,
) -> &'a [Value] {
    match attributes.get(primary).or_else(|| attributes.get(alias)) {
        Some(Value::Array(items)) => items.as_slice(),
        _ => &[],
    }
}

fn parameter_name(item: &Value) -> String {
    match item {
        Value::Object(map) => map
            .get("name")
            .or_else(|| map.get("label"))
            .map(|v| match v {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            })
            .unwrap_or_default(),
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

/// Side of the square a parameter is drawn as.
const PARAMETER_SIZE: f64 = 10.0;

fn draw_parameters(
    mut g: Element,
    items: &[Value],
    side: &str,
    layout: &crate::behavior_common::LaidOutRect,
    theme: &Theme,
) -> Element {
    for (index, item) in items.iter().enumerate() {
        let name = parameter_name(item);
        let y = 20.0 + index as f64 * 14.0;
        let boundary = if side == "input" { 0.0 } else { layout.width };
        let x = if side == "input" {
            -PARAMETER_SIZE / 2.0
        } else {
            layout.width + PARAMETER_SIZE / 2.0
        };
        // SysML 8.2.3.17: a parameter is a small square on the boundary of its action.
        g = g.child(
            Element::new("rect")
                .attr(
                    "class",
                    format!("action-parameter-badge action-parameter-{side}"),
                )
                .attr_f("x", boundary - PARAMETER_SIZE / 2.0)
                .attr_f("y", y - PARAMETER_SIZE / 2.0)
                .attr_f("width", PARAMETER_SIZE)
                .attr_f("height", PARAMETER_SIZE)
                .attr("rx", "2")
                .style("fill", theme.canvas_background)
                .style("stroke", theme.node_border)
                .style("stroke-width", "1.5px"),
        );
        g = g.child(
            Element::new("text")
                .attr(
                    "class",
                    format!("action-parameter-label action-parameter-{side}-label"),
                )
                .attr_f("x", if side == "input" { x - 4.0 } else { x + 4.0 })
                .attr_f("y", y + 3.0)
                .attr("text-anchor", if side == "input" { "end" } else { "start" })
                .style("font-size", "8px")
                .style("fill", theme.text_secondary)
                .text(name),
        );
    }
    g
}

fn draw_action_node(
    node: &PreparedNode,
    layout: &crate::behavior_common::LaidOutRect,
    theme: &Theme,
) -> Element {
    let kind = activity_node_kind(node);
    let inputs = attr_array(&node.attributes, "inputs", "inputParameters");
    let outputs = attr_array(&node.attributes, "outputs", "outputParameters");
    let action_type = {
        let t = attr_text(&node.attributes, "actionType");
        if !t.is_empty() {
            t
        } else {
            attr_text(&node.attributes, "type")
        }
    };
    let is_perform = kind.contains("perform") || action_type.to_lowercase().contains("perform");
    let typed_action = !attr_text(&node.attributes, "notationRole").is_empty();

    let mut g = Element::new("g")
        .attr(
            "class",
            format!(
                "activity-action action-flow-node{}",
                if is_perform {
                    " perform-action-node"
                } else {
                    ""
                }
            ),
        )
        .attr("data-node-id", node.id.clone())
        .attr(
            "transform",
            format!("translate({},{})", n(layout.x), n(layout.y)),
        );

    if is_initial(&kind) || is_final(&kind) {
        g = g.child(
            Element::new("circle")
                .attr("class", "node-background")
                .attr("data-original-stroke", theme.node_border)
                .attr("data-original-width", "2px")
                .attr_f("cx", layout.width / 2.0)
                .attr_f("cy", layout.height / 2.0)
                .attr_f("r", layout.width / 2.0 - 2.0)
                .style(
                    "fill",
                    if is_initial(&kind) {
                        theme.edge_default
                    } else {
                        theme.canvas_background
                    },
                )
                .style("stroke", theme.node_border)
                .style("stroke-width", "2px"),
        );
        if is_final(&kind) && !is_flow_final(&kind) {
            g = g.child(
                Element::new("circle")
                    .attr_f("cx", layout.width / 2.0)
                    .attr_f("cy", layout.height / 2.0)
                    .attr_f("r", 10.0)
                    .style("fill", theme.edge_default)
                    .style("stroke", "none"),
            );
        }
        if is_flow_final(&kind) {
            let cx = layout.width / 2.0;
            let cy = layout.height / 2.0;
            g = g.child(
                Element::new("path")
                    .attr("class", "flow-final-x")
                    .attr(
                        "d",
                        format!(
                            "M{},{} L{},{} M{},{} L{},{}",
                            n(cx - 8.0),
                            n(cy - 8.0),
                            n(cx + 8.0),
                            n(cy + 8.0),
                            n(cx + 8.0),
                            n(cy - 8.0),
                            n(cx - 8.0),
                            n(cy + 8.0)
                        ),
                    )
                    .style("stroke", theme.edge_default)
                    .style("stroke-width", "2px"),
            );
        }
    } else if is_decision(&kind) {
        let cx = layout.width / 2.0;
        let cy = layout.height / 2.0;
        g = g.child(
            Element::new("path")
                .attr("class", "node-background")
                .attr("data-original-stroke", theme.edge_default)
                .attr("data-original-width", "2px")
                .attr(
                    "d",
                    format!(
                        "M{},{} L{},{} L{},{} L{},{} Z",
                        n(cx),
                        n(0.0),
                        n(layout.width),
                        n(cy),
                        n(cx),
                        n(layout.height),
                        n(0.0),
                        n(cy)
                    ),
                )
                .style("fill", theme.canvas_background)
                .style("stroke", theme.edge_default)
                .style("stroke-width", "2px"),
        );
    } else if is_fork(&kind) {
        g = g.child(
            Element::new("rect")
                .attr("class", "node-background")
                .attr("data-original-stroke", "none")
                .attr("data-original-width", "0px")
                .attr_f("width", layout.width)
                .attr_f("height", layout.height)
                .attr("rx", "3")
                .style("fill", theme.node_border)
                .style("stroke", "none"),
        );
    } else {
        g = g.child(
            Element::new("rect")
                .attr("class", "node-background")
                .attr("data-original-stroke", theme.node_border)
                .attr("data-original-width", "2px")
                .attr_f("width", layout.width)
                .attr_f("height", layout.height)
                .attr("rx", "8")
                .style("fill", theme.node_fill)
                .style("stroke", theme.node_border)
                .style("stroke-width", "2px")
                .style("stroke-dasharray", if is_perform { "5,3" } else { "none" }),
        );
        // SysML 8.2.3.17: an accept action carries a notched flag in its upper left corner, a
        // send action a pointed one.
        let signal_icon = if kind.contains("accept") {
            Some(("accept-action-icon", "M8,8 H22 V18 H8 L12,13 Z"))
        } else if kind.contains("send") {
            Some(("send-action-icon", "M8,8 H18 L22,13 L18,18 H8 Z"))
        } else {
            None
        };
        if let Some((class, path)) = signal_icon {
            g = g.child(
                Element::new("path")
                    .attr("class", class)
                    .attr("d", path)
                    .style("fill", theme.node_border)
                    .style("stroke", "none"),
            );
        }
        if typed_action {
            g = g.child(
                Element::new("text")
                    .attr("class", "action-stereotype")
                    .attr_f("x", layout.width / 2.0)
                    .attr_f("y", layout.height / 2.0 - 7.0)
                    .attr("text-anchor", "middle")
                    .style("font-size", "10px")
                    .style("fill", theme.text_secondary)
                    .text(action_keyword(node, is_perform)),
            );
        } else {
            g = g.child(
                Element::new("rect")
                    .attr_f("width", layout.width)
                    .attr_f("height", 6.0)
                    .attr("rx", "8")
                    .style("fill", theme.node_border)
                    .style("stroke", "none"),
            );
        }
        if is_perform && !typed_action {
            g = g.child(
                Element::new("text")
                    .attr("class", "perform-action-stereotype")
                    .attr_f("x", layout.width / 2.0)
                    .attr_f("y", 20.0)
                    .attr("text-anchor", "middle")
                    .style("font-size", "9px")
                    .style("fill", theme.text_secondary)
                    .text("perform"),
            );
        }
    }

    let label_y = if is_fork(&kind) {
        layout.height + 14.0
    } else {
        layout.height / 2.0
            + if typed_action || is_perform {
                12.0
            } else {
                4.0
            }
    };
    g = g.child(
        Element::new("text")
            .attr_f("x", layout.width / 2.0)
            .attr_f("y", label_y)
            .attr("text-anchor", "middle")
            .style("font-size", "12px")
            .style("font-weight", "600")
            .style("fill", theme.text_primary)
            .text(truncate_label(&node.label, 24)),
    );

    if !is_initial(&kind) && !is_final(&kind) && !is_decision(&kind) && !is_fork(&kind) {
        g = draw_parameters(g, inputs, "input", layout, theme);
        g = draw_parameters(g, outputs, "output", layout, theme);
    }

    g
}

/// The keyword shown in a typed action's name compartment.
fn action_keyword(node: &PreparedNode, is_perform: bool) -> &'static str {
    if is_perform {
        "«perform action»"
    } else if attr_text(&node.attributes, "notationRole") == "definition" {
        "«action def»"
    } else {
        "«action»"
    }
}

/// An action that owns other actions: its name compartment on top and, below the divider, its own
/// action flow (drawn by the caller), as in the `action` / `action-def` notation of SysML v2
/// 8.2.3.17.
fn draw_action_container(
    node: &PreparedNode,
    layout: &crate::behavior_common::LaidOutRect,
    theme: &Theme,
) -> Element {
    let is_perform = activity_node_kind(node).contains("perform");
    Element::new("g")
        .attr(
            "class",
            "activity-action action-flow-node action-flow-container",
        )
        .attr("data-node-id", node.id.clone())
        .attr(
            "transform",
            format!("translate({},{})", n(layout.x), n(layout.y)),
        )
        .child(
            Element::new("rect")
                .attr("class", "node-background")
                .attr("data-original-stroke", theme.node_border)
                .attr("data-original-width", "2px")
                .attr_f("width", layout.width)
                .attr_f("height", layout.height)
                .attr("rx", "12")
                .style("fill", theme.canvas_background)
                .style("stroke", theme.node_border)
                .style("stroke-width", "2px")
                .style("stroke-dasharray", if is_perform { "5,3" } else { "none" }),
        )
        .child(
            Element::new("text")
                .attr("class", "action-stereotype")
                .attr_f("x", layout.width / 2.0)
                .attr_f("y", 17.0)
                .attr("text-anchor", "middle")
                .style("font-size", "10px")
                .style("fill", theme.text_secondary)
                .text(action_keyword(node, is_perform)),
        )
        .child(
            Element::new("text")
                .attr_f("x", layout.width / 2.0)
                .attr_f("y", 33.0)
                .attr("text-anchor", "middle")
                .style("font-size", "12px")
                .style("font-weight", "600")
                .style("fill", theme.text_primary)
                .text(truncate_label(&node.label, 48)),
        )
        .child(
            Element::new("line")
                .attr("class", "action-flow-compartment-divider")
                .attr_f("x1", 0.0)
                .attr_f("y1", crate::layout::ACTION_CONTAINER_HEADER)
                .attr_f("x2", layout.width)
                .attr_f("y2", crate::layout::ACTION_CONTAINER_HEADER)
                .style("stroke", theme.node_border)
                .style("stroke-width", "1px"),
        )
}

struct LaneExtent {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

/// Port of `renderActionFlowView`. Returns the drawn content plus `{minX,minY,maxX,maxY}`.
pub fn render_action_flow_view(
    prepared: &PreparedView,
    layout: &BehaviorLayoutResult,
    theme: &Theme,
    _width: f64,
    _height: f64,
) -> (Element, (f64, f64, f64, f64)) {
    let nodes_by_id: HashMap<&str, &PreparedNode> = prepared
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect();
    let horizontal = prepared
        .meta
        .get("layoutDirection")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_lowercase()
        == "horizontal";
    let typed_projection = prepared.meta.get("typedProjection") == Some(&Value::Bool(true));

    let mut lane_extents: Vec<(String, LaneExtent)> = Vec::new();
    for node in &prepared.nodes {
        let Some(position) = layout.positions.get(&node.id) else {
            continue;
        };
        let lane = {
            let l = attr_text(&node.attributes, "swimLane");
            if l.is_empty() {
                "default".to_string()
            } else {
                l
            }
        };
        match lane_extents
            .iter_mut()
            .find(|(candidate, _)| candidate == &lane)
        {
            Some((_, extent)) => {
                extent.min_x = extent.min_x.min(position.x);
                extent.max_x = extent.max_x.max(position.x + position.width);
                extent.min_y = extent.min_y.min(position.y);
                extent.max_y = extent.max_y.max(position.y + position.height);
            }
            None => lane_extents.push((
                lane,
                LaneExtent {
                    min_x: position.x,
                    max_x: position.x + position.width,
                    min_y: position.y,
                    max_y: position.y + position.height,
                },
            )),
        }
    }
    // Include decorations outside node rectangles before enclosing the root view.
    let mut content = crate::behavior_common::content_extent(layout);
    for node in &prepared.nodes {
        let Some(position) = layout.positions.get(&node.id) else {
            continue;
        };
        for (side, items) in [
            (
                "input",
                attr_array(&node.attributes, "inputs", "inputParameters"),
            ),
            (
                "output",
                attr_array(&node.attributes, "outputs", "outputParameters"),
            ),
        ] {
            for (index, item) in items.iter().enumerate() {
                let width = parameter_name(item).chars().count() as f64 * 8.0 + 9.0;
                if side == "input" {
                    content.0 = content.0.min(position.x - width);
                } else {
                    content.2 = content.2.max(position.x + position.width + width);
                }
                content.3 = content.3.max(position.y + 25.0 + index as f64 * 14.0);
            }
        }
        if activity_node_kind(node).contains("fork") || activity_node_kind(node).contains("join") {
            content.3 = content.3.max(position.y + position.height + 20.0);
        }
    }
    if lane_extents.len() > 1 {
        for (_, lane) in &lane_extents {
            content.0 = content.0.min(lane.min_x - 24.0);
            content.1 = content.1.min(lane.min_y - 36.0);
            content.2 = content.2.max(lane.max_x + 24.0);
            content.3 = content.3.max(lane.max_y + 20.0);
        }
    }
    // SysML v2 §§8.2.3.1, 8.2.3.26: the view frame is distinct from action containers.
    let (frame, extent) = crate::view_frame::view_frame(
        if prepared.title.is_empty() {
            "Action Flow"
        } else {
            &prepared.title
        },
        content,
        theme,
    );
    let mut root = Element::new("g").attr("class", "viz-root").child(frame);

    let mut lane_layer = Element::new("g").attr("class", "activity-swim-lanes");
    if lane_extents.len() > 1 {
        for (lane, extent) in &lane_extents {
            lane_layer = lane_layer.child(
                Element::new("rect")
                    .attr_f("x", extent.min_x - 24.0)
                    .attr_f("y", extent.min_y - 36.0)
                    .attr_f("width", extent.max_x - extent.min_x + 48.0)
                    .attr_f("height", extent.max_y - extent.min_y + 56.0)
                    .attr("rx", "8")
                    .style("fill", theme.canvas_background)
                    .style("stroke", theme.frame_stroke)
                    .style("stroke-dasharray", "6,4"),
            );
            lane_layer = lane_layer.child(
                Element::new("text")
                    .attr_f("x", extent.min_x - 12.0)
                    .attr_f("y", extent.min_y - 18.0)
                    .style("font-size", "10px")
                    .style("font-weight", "700")
                    .style("fill", theme.text_secondary)
                    .text(truncate_label(lane, 24)),
            );
        }
    }

    let mut flow_layer = Element::new("g").attr("class", "activity-flows");
    for edge in &prepared.edges {
        let (Some(source), Some(target)) = (
            layout.positions.get(&edge.source),
            layout.positions.get(&edge.target),
        ) else {
            continue;
        };
        let sections = layout
            .edge_sections_by_id
            .get(&edge.id)
            .map(|v| v.as_slice());
        let fallback = fallback_edge_path(source, target, horizontal);
        let guard = {
            let g = attr_text(&edge.attributes, "guard");
            if !g.is_empty() {
                g
            } else {
                edge.label.clone()
            }
        }
        .to_lowercase();
        let succession = matches!(edge.attributes.get("succession"), Some(Value::Bool(true)))
            || guard == "first"
            || guard == "succession"
            || guard == "succession flow";
        let streaming_flow = matches!(
            edge.attributes.get("streamingFlow"),
            Some(Value::Bool(true))
        ) || guard == "flow";
        let conditional = matches!(edge.attributes.get("conditional"), Some(Value::Bool(true)));
        let path = path_from_sections(sections).unwrap_or_else(|| fallback.path.clone());

        let class = if succession {
            if conditional {
                "activity-flow action-flow-edge aflow-succession aflow-conditional"
            } else {
                "activity-flow action-flow-edge aflow-succession"
            }
        } else if streaming_flow {
            "activity-flow action-flow-edge aflow-streaming"
        } else {
            "activity-flow action-flow-edge"
        };
        let flow_kind = if succession {
            "succession"
        } else if streaming_flow {
            "streaming"
        } else {
            "other"
        };

        let visible = Element::new("path")
            .attr("class", class)
            .attr("data-flow-kind", flow_kind)
            .attr("d", path.clone())
            .style("fill", "none")
            .style("stroke", theme.edge_default)
            .style("stroke-width", "2px")
            .style("stroke-dasharray", if succession { "7,4" } else { "none" })
            .style(
                "marker-end",
                if succession && typed_projection {
                    "url(#action-succession-arrow)"
                } else {
                    "url(#action-flow-arrow)"
                },
            );
        flow_layer = flow_layer.child(mark_visible_edge(visible, &edge.id, "2"));

        let tooltip_text = tooltip_fallback_text(&behavior_edge_tooltip_descriptor(
            "action-flow-view",
            edge,
            &nodes_by_id,
        ));
        flow_layer = flow_layer.child(path_edge_hit_target(&path, &edge.id, Some(&tooltip_text)));

        let label = edge.label.clone();
        if !label.is_empty() && !["flow", "first", "bind"].contains(&label.to_lowercase().as_str())
        {
            let elk_label = layout
                .edge_labels_by_id
                .get(&edge.id)
                .and_then(|labels| labels.first());
            let label_from_sections = edge_label_position_from_sections(sections);
            let (label_x, label_y) = match elk_label {
                Some(elk) => (elk.x + elk.width / 2.0, elk.y + elk.height / 2.0),
                None => label_from_sections.unwrap_or((fallback.label_x, fallback.label_y)),
            };
            let display_label = if label.starts_with('[') {
                label.clone()
            } else {
                format!("[{label}]")
            };
            if let Some(elk) = elk_label {
                flow_layer = flow_layer.child(
                    Element::new("rect")
                        .attr_f("x", elk.x)
                        .attr_f("y", elk.y)
                        .attr_f("width", elk.width)
                        .attr_f("height", elk.height)
                        .attr("rx", "3")
                        // Plain text beside the line; the unframed backing only keeps it
                        // legible where another line passes behind it.
                        .style("fill", theme.canvas_background)
                        .style("stroke", "none"),
                );
            }
            flow_layer = flow_layer.child(
                Element::new("text")
                    .attr_f("x", label_x)
                    .attr_f("y", label_y + if elk_label.is_some() { 3.0 } else { 0.0 })
                    .attr("text-anchor", "middle")
                    .style("font-size", "10px")
                    .style("fill", theme.text_secondary)
                    .text(display_label),
            );
        }
    }

    // Containers are drawn first, outermost first, so the flows and actions inside stay on top.
    let container_ids: std::collections::HashSet<String> = prepared
        .nodes
        .iter()
        .map(|node| attr_text(&node.attributes, "containerId"))
        .filter(|id| nodes_by_id.contains_key(id.as_str()))
        .collect();
    let depth = |node: &PreparedNode| {
        let mut depth = 0;
        let mut current = attr_text(&node.attributes, "containerId");
        while let Some(parent) = nodes_by_id.get(current.as_str()) {
            depth += 1;
            if depth > prepared.nodes.len() {
                break;
            }
            current = attr_text(&parent.attributes, "containerId");
        }
        depth
    };
    let mut containers: Vec<&PreparedNode> = prepared
        .nodes
        .iter()
        .filter(|node| container_ids.contains(&node.id))
        .collect();
    containers.sort_by_key(|node| depth(node));
    let mut container_layer = Element::new("g").attr("class", "activity-action-containers");
    for node in containers {
        if let Some(position) = layout.positions.get(&node.id) {
            container_layer = container_layer
                .child(draw_action_container(node, position, theme).style("cursor", ""));
        }
    }

    let mut node_layer = Element::new("g").attr("class", "activity-actions");
    for node in &prepared.nodes {
        let Some(position) = layout.positions.get(&node.id) else {
            continue;
        };
        if container_ids.contains(&node.id) {
            continue;
        }
        // `attachBehaviorNodeClick` runs unconditionally for every node; in headless export it
        // always resolves to `.style("cursor", "")`, kept as a literal empty declaration.
        let node_group = draw_action_node(node, position, theme).style("cursor", "");
        node_layer = node_layer.child(node_group);
    }

    // `ctx.root.insert(".activity-flows")` in the TS source puts swim lanes *before* the
    // `.activity-flows` group that was already appended -- matched via `insert_before_child`.
    if !container_ids.is_empty() {
        root = root.child(container_layer);
    }
    root = root.child(flow_layer);
    root = root.insert_before_child("activity-flows", lane_layer);
    root = root.child(node_layer);

    (root, extent)
}

/// Port of `addActionFlowMarkers`.
pub fn action_flow_marker(theme: &Theme) -> Element {
    Element::new("marker")
        .attr("id", "action-flow-arrow")
        .attr("viewBox", "0 -5 10 10")
        .attr("refX", "8")
        .attr("refY", "0")
        .attr("markerWidth", "6")
        .attr("markerHeight", "6")
        .attr("orient", "auto")
        .child(
            Element::new("path")
                .attr("d", "M0,-5L10,0L0,5")
                .style("fill", theme.edge_default),
        )
}

pub fn typed_action_flow_markers(theme: &Theme) -> Element {
    Element::new("g").child(action_flow_marker(theme)).child(
        Element::new("marker")
            .attr("id", "action-succession-arrow")
            .attr("viewBox", "0 -5 10 10")
            .attr("refX", "9")
            .attr("refY", "0")
            .attr("markerWidth", "7")
            .attr("markerHeight", "7")
            .attr("orient", "auto")
            .child(
                Element::new("path")
                    .attr("d", "M0,-5L10,0L0,5")
                    .style("fill", "none")
                    .style("stroke", theme.edge_default)
                    .style("stroke-width", "1.5"),
            ),
    )
}

#[cfg(test)]
mod signal_action_tests {
    use super::*;
    use crate::behavior_common::LaidOutRect;
    use crate::theme::LIGHT;

    fn drawn(kind: &str) -> String {
        let node = PreparedNode {
            id: "a".into(),
            label: "a".into(),
            kind: kind.into(),
            attributes: Default::default(),
        };
        let rect = LaidOutRect {
            x: 0.0,
            y: 0.0,
            width: 160.0,
            height: 60.0,
        };
        draw_action_node(&node, &rect, &LIGHT).to_string()
    }

    #[test]
    fn accept_and_send_actions_carry_their_icons() {
        assert!(drawn("accept").contains("accept-action-icon"));
        assert!(drawn("send").contains("send-action-icon"));
        let plain = drawn("action");
        assert!(!plain.contains("accept-action-icon") && !plain.contains("send-action-icon"));
    }
}
