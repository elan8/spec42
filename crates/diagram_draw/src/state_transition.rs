//! Port of `views/state-transition.ts`.

use crate::behavior_common::{
    build_self_loop_path, edge_label_position_from_sections, fallback_edge_path, node_kind,
    truncate_label, BehaviorLayoutResult, LaidOutRect, PreparedNode, PreparedView,
};
use crate::hit_target::{mark_visible_edge, path_edge_hit_target};
use crate::svg::Element;
use crate::theme::Theme;
use crate::tooltip::{behavior_edge_tooltip_descriptor, tooltip_fallback_text};
use crate::types::attr_bool;
use std::collections::HashMap;

/// Port of `transitionDisplayLabel`.
fn transition_display_label(label: &str) -> String {
    let trimmed = label.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("entry") {
        String::new()
    } else {
        trimmed.to_string()
    }
}

/// Port of e.g. `String(attrs.entry ?? attrs.entryAction ?? "").trim()` followed by a truthiness
/// check: the first key with a non-empty value, or `None` if every alias is empty/absent.
fn first_non_empty(node: &PreparedNode, keys: &[&str]) -> Option<String> {
    keys.iter()
        .map(|key| crate::types::attr_text(&node.attributes, key))
        .find(|value| !value.is_empty())
}

fn drawn_regions(node: &PreparedNode) -> Vec<String> {
    let raw = node
        .attributes
        .get("regions")
        .or_else(|| node.attributes.get("children"));
    match raw {
        Some(serde_json::Value::Array(items)) => items
            .iter()
            .map(|item| match item {
                serde_json::Value::Object(map) => map
                    .get("name")
                    .or_else(|| map.get("label"))
                    .and_then(|v| v.as_str())
                    .map(str::to_string)
                    .unwrap_or_default(),
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn draw_state_node(node: &PreparedNode, layout: &LaidOutRect, theme: &Theme) -> Element {
    let kind = node_kind(node);
    let entry = first_non_empty(node, &["entry", "entryAction"]);
    let do_action = first_non_empty(node, &["do", "doAction"]);
    let exit = first_non_empty(node, &["exit", "exitAction"]);
    let regions = drawn_regions(node);
    let is_composite = kind.contains("composite") || !regions.is_empty();
    let is_terminate = kind.contains("terminate");

    let mut g = Element::new("g")
        .attr("class", "state-node state-transition-node")
        .attr("data-node-id", node.id.clone())
        .attr(
            "transform",
            format!(
                "translate({},{})",
                crate::svg::format_number(layout.x),
                crate::svg::format_number(layout.y)
            ),
        );

    if kind.contains("initial") {
        g = g.child(
            Element::new("circle")
                .attr("class", "node-background")
                .attr("data-original-stroke", theme.node_border)
                .attr("data-original-width", "2px")
                .attr_f("cx", layout.width / 2.0)
                .attr_f("cy", layout.height / 2.0)
                .attr_f("r", layout.width / 2.0 - 2.0)
                .style("fill", theme.edge_default)
                .style("stroke", theme.node_border)
                .style("stroke-width", "2px"),
        );
    } else if kind.contains("final") || is_terminate {
        g = g.child(
            Element::new("circle")
                .attr("class", "node-background")
                .attr("data-original-stroke", theme.node_border)
                .attr("data-original-width", "2px")
                .attr_f("cx", layout.width / 2.0)
                .attr_f("cy", layout.height / 2.0)
                .attr_f("r", layout.width / 2.0 - 2.0)
                .style("fill", theme.canvas_background)
                .style("stroke", theme.node_border)
                .style("stroke-width", "2px"),
        );
        if is_terminate {
            let cx = layout.width / 2.0;
            let cy = layout.height / 2.0;
            let n = crate::svg::format_number;
            g = g.child(
                Element::new("path")
                    .attr("class", "terminate-state-x")
                    .attr(
                        "d",
                        format!(
                            "M{},{} L{},{} M{},{} L{},{}",
                            n(cx - 9.0),
                            n(cy - 9.0),
                            n(cx + 9.0),
                            n(cy + 9.0),
                            n(cx + 9.0),
                            n(cy - 9.0),
                            n(cx - 9.0),
                            n(cy + 9.0)
                        ),
                    )
                    .style("stroke", theme.edge_default)
                    .style("stroke-width", "2px"),
            );
        } else {
            g = g.child(
                Element::new("circle")
                    .attr_f("cx", layout.width / 2.0)
                    .attr_f("cy", layout.height / 2.0)
                    .attr_f("r", 10.0)
                    .style("fill", theme.edge_default)
                    .style("stroke", "none"),
            );
        }
    } else {
        g = g.child(
            Element::new("rect")
                .attr("class", "node-background")
                .attr("data-original-stroke", theme.node_border)
                .attr("data-original-width", "2px")
                .attr_f("width", layout.width)
                .attr_f("height", layout.height)
                .attr_f("rx", if is_composite { 10.0 } else { 14.0 })
                .style("fill", theme.node_fill)
                .style("stroke", theme.node_border)
                .style("stroke-width", "2px"),
        );
        g = g.child(
            Element::new("text")
                .attr_f("x", layout.width / 2.0)
                .attr_f("y", 22.0)
                .attr("text-anchor", "middle")
                .style("font-size", "12px")
                .style("font-weight", "700")
                .style("fill", theme.text_primary)
                .text(truncate_label(&node.label, 28)),
        );
        let mut action_lines = Vec::new();
        if let Some(entry) = &entry {
            action_lines.push(format!("entry / {entry}"));
        }
        if let Some(do_action) = &do_action {
            action_lines.push(format!("do / {do_action}"));
        }
        if let Some(exit) = &exit {
            action_lines.push(format!("exit / {exit}"));
        }
        if !action_lines.is_empty() || is_composite {
            g = g.child(
                Element::new("line")
                    .attr("class", "state-compartment-divider")
                    .attr_f("x1", 0.0)
                    .attr_f("x2", layout.width)
                    .attr_f("y1", 34.0)
                    .attr_f("y2", 34.0)
                    .style("stroke", theme.node_border)
                    .style("stroke-width", "1px"),
            );
        }
        for (index, line) in action_lines.iter().enumerate() {
            g = g.child(
                Element::new("text")
                    .attr("class", "state-action-compartment")
                    .attr_f("x", 12.0)
                    .attr_f("y", 54.0 + index as f64 * 16.0)
                    .style("font-size", "10px")
                    .style("fill", theme.text_secondary)
                    .text(truncate_label(line, 34)),
            );
        }
        if is_composite {
            let action_count = action_lines.len() as f64;
            let region_top = (52.0 + action_count * 16.0).max(80.0);
            let region_count = if regions.is_empty() {
                1.0
            } else {
                regions.len() as f64
            };
            let region_height = ((layout.height - region_top - 14.0) / region_count).max(32.0);
            let region_list: Vec<String> = if regions.is_empty() {
                vec!["region".to_string()]
            } else {
                regions.clone()
            };
            for (index, region_name) in region_list.iter().take(4).enumerate() {
                let y = region_top + index as f64 * region_height;
                g = g.child(
                    Element::new("rect")
                        .attr("class", "state-region")
                        .attr_f("x", 12.0)
                        .attr_f("y", y)
                        .attr_f("width", layout.width - 24.0)
                        .attr_f("height", (region_height - 8.0).max(24.0))
                        .attr("rx", "5")
                        .style("fill", "none")
                        .style("stroke", theme.node_border)
                        .style("stroke-dasharray", "4,3"),
                );
                let label = if region_name.is_empty() {
                    format!("region {}", index + 1)
                } else {
                    region_name.clone()
                };
                g = g.child(
                    Element::new("text")
                        .attr("class", "state-region-label")
                        .attr_f("x", 20.0)
                        .attr_f("y", y + 17.0)
                        .style("font-size", "9px")
                        .style("fill", theme.text_secondary)
                        .text(truncate_label(&label, 28)),
                );
            }
        }
    }

    g
}

/// Port of `renderStateTransitionView`. Returns the drawn content plus `{minX,minY,maxX,maxY}`.
pub fn render_state_transition_view(
    prepared: &PreparedView,
    layout: &BehaviorLayoutResult,
    theme: &Theme,
    width: f64,
    height: f64,
) -> (Element, (f64, f64, f64, f64)) {
    let nodes_by_id: HashMap<&str, &PreparedNode> = prepared
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect();

    let mut root = Element::new("g").attr("class", "viz-root").child(
        Element::new("text")
            .attr_f("x", 24.0)
            .attr_f("y", 28.0)
            .style("font-size", "14px")
            .style("font-weight", "700")
            .style("fill", theme.text_primary)
            .text(if prepared.title.is_empty() {
                "State Transition".to_string()
            } else {
                prepared.title.clone()
            }),
    );

    let horizontal = {
        let mode = prepared
            .meta
            .get("layoutDirection")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("horizontal")
            .to_lowercase();
        mode != "vertical" && mode != "force"
    };

    let mut edge_layer = Element::new("g").attr("class", "state-transitions");
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
        let self_loop = attr_bool(&edge.attributes, "selfLoop") || edge.source == edge.target;
        let fallback = if self_loop {
            build_self_loop_path(source)
        } else {
            fallback_edge_path(source, target, horizontal)
        };
        let path = if self_loop {
            fallback.path.clone()
        } else {
            crate::behavior_common::path_from_sections(sections)
                .unwrap_or_else(|| fallback.path.clone())
        };
        let guard = crate::types::attr_text(&edge.attributes, "guard");
        let effect = crate::types::attr_text(&edge.attributes, "effect");
        let accept = crate::types::attr_text(&edge.attributes, "accept");
        let send = crate::types::attr_text(&edge.attributes, "send");

        let visible = Element::new("path")
            .attr("class", "state-transition-edge")
            .maybe_attr("data-guard", (!guard.is_empty()).then_some(guard))
            .maybe_attr("data-effect", (!effect.is_empty()).then_some(effect))
            .maybe_attr("data-accept", (!accept.is_empty()).then_some(accept))
            .maybe_attr("data-send", (!send.is_empty()).then_some(send))
            .attr("d", path.clone())
            .style("fill", "none")
            .style("stroke", theme.edge_default)
            .style("stroke-width", "2px")
            .style("marker-end", "url(#state-transition-arrow)");
        edge_layer = edge_layer.child(mark_visible_edge(visible, &edge.id, "2"));

        let tooltip_text = tooltip_fallback_text(&behavior_edge_tooltip_descriptor(
            "state-transition-view",
            edge,
            &nodes_by_id,
        ));
        edge_layer = edge_layer.child(path_edge_hit_target(&path, &edge.id, Some(&tooltip_text)));

        let label = transition_display_label(&edge.label);
        if !label.is_empty() {
            let elk_label = layout
                .edge_labels_by_id
                .get(&edge.id)
                .and_then(|labels| labels.first());
            let label_from_sections = edge_label_position_from_sections(sections);
            let (label_x, label_y) = match elk_label {
                Some(elk) => (elk.x + elk.width / 2.0, elk.y + elk.height / 2.0),
                None => label_from_sections.unwrap_or((fallback.label_x, fallback.label_y)),
            };
            let label_width = elk_label
                .map(|elk| elk.width)
                .unwrap_or_else(|| (label.encode_utf16().count() as f64 * 6.0 + 10.0).max(42.0));
            let label_height = elk_label.map(|elk| elk.height).unwrap_or(18.0);
            let (rect_x, rect_y) = match elk_label {
                Some(elk) => (elk.x, elk.y),
                None => (label_x - label_width / 2.0, label_y - 10.0),
            };
            edge_layer = edge_layer.child(
                Element::new("rect")
                    .attr_f("x", rect_x)
                    .attr_f("y", rect_y)
                    .attr_f("width", label_width)
                    .attr_f("height", label_height)
                    .attr("rx", "4")
                    .style("fill", theme.canvas_background)
                    .style("stroke", theme.edge_default)
                    .style("stroke-width", "1px"),
            );
            edge_layer = edge_layer.child(
                Element::new("text")
                    .attr_f("x", label_x)
                    .attr_f("y", label_y + 3.0)
                    .attr("text-anchor", "middle")
                    .style("font-size", "10px")
                    .style("font-weight", "500")
                    .style("fill", theme.edge_default)
                    .text(label),
            );
        }
    }
    root = root.child(edge_layer);

    let mut node_layer = Element::new("g").attr("class", "state-nodes");
    for node in &prepared.nodes {
        let Some(position) = layout.positions.get(&node.id) else {
            continue;
        };
        // `attachBehaviorNodeClick` runs unconditionally for every node here (unlike
        // sequence-view's lifelines); in headless export it always resolves to
        // `.style("cursor", "")`, which `VirtualStyle` keeps as a literal empty declaration.
        let node_group = draw_state_node(node, position, theme).style("cursor", "");
        node_layer = node_layer.child(node_group);
    }
    root = root.child(node_layer);

    let mut min_x = 0.0_f64;
    let mut min_y = 0.0_f64;
    let mut max_x = width;
    let mut max_y = height;
    for rect in layout.positions.values() {
        min_x = min_x.min(rect.x);
        min_y = min_y.min(rect.y);
        max_x = max_x.max(rect.x + rect.width);
        max_y = max_y.max(rect.y + rect.height + 20.0);
    }
    (
        root,
        (min_x - 40.0, min_y - 40.0, max_x + 40.0, max_y + 40.0),
    )
}

/// Port of `addStateTransitionMarkers`.
pub fn state_transition_marker(theme: &Theme) -> Element {
    Element::new("marker")
        .attr("id", "state-transition-arrow")
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
