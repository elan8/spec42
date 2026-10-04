//! Port of `drawIbdPorts` in `render/drawing.ts` plus `ibdPortLabelText` in
//! `render/ibd-port-label.ts`. Ports are returned as a flat `Vec<Element>` (icon+label pairs, in
//! draw order) rather than appended directly to the node's group: `drawInterconnectionPortOverlays`
//! always relocates them into a separate `g.viz-port-overlay` sibling once any exist (see
//! `ibd_node.rs`'s module docs), so building them separately from the start avoids simulating that
//! DOM-move step.

use serde_json::Value;

use crate::svg::{format_number as n, Element};
use crate::theme::Theme;
use crate::tooltip::{port_tooltip_descriptor, tooltip_fallback_text};
use crate::types::{InterconnectionLayoutNodeDto, InterconnectionLayoutPortAnchor, LaidOutNode};

const IBD_PORT_LABEL_FONT_SIZE: f64 = 8.0;
const IBD_PORT_LABEL_MAX_LENGTH: usize = 20;
const IBD_PORT_LABEL_CHARACTER_WIDTH: f64 = 5.0;

/// Port of `formatIbdPortLabel` + `ibdPortLabelText` (the `detail` parameter is explicitly unused
/// upstream too).
pub(crate) fn ibd_port_label_text(name: &str) -> String {
    let trimmed = name.trim();
    let units: Vec<u16> = trimmed.encode_utf16().collect();
    if units.len() > IBD_PORT_LABEL_MAX_LENGTH {
        format!(
            "{}…",
            String::from_utf16_lossy(&units[..IBD_PORT_LABEL_MAX_LENGTH - 1])
        )
    } else {
        trimmed.to_string()
    }
}

/// Port of `ibdPortLabelWidth`.
pub(crate) fn ibd_port_label_width(text: &str) -> f64 {
    (text.encode_utf16().count() as f64 * IBD_PORT_LABEL_CHARACTER_WIDTH).max(12.0)
}

fn port_detail<'a>(details: &'a [Value], name: &str) -> Option<&'a Value> {
    details
        .iter()
        .find(|detail| detail.get("name").and_then(Value::as_str) == Some(name))
}

fn port_tooltip_id(detail: &Value) -> String {
    detail
        .get("id")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .or_else(|| {
            detail
                .get("attributes")
                .and_then(|a| a.get("scenePortId"))
                .and_then(Value::as_str)
        })
        .unwrap_or("")
        .to_string()
}

fn sanitize_port_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-') {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Port of `drawIbdPorts`. Returns the drawn `rect.port-icon`/`text.port-label` pairs in draw
/// order (west-then-east when `portDrawOrder` is present, matching the original).
pub fn draw_ibd_ports(
    node: &LaidOutNode,
    width: f64,
    fallback_start_y: f64,
    theme: &Theme,
    layout_node: Option<&InterconnectionLayoutNodeDto>,
) -> Vec<Element> {
    let details: Vec<Value> = match node.attributes.get("portDetails") {
        Some(Value::Array(items)) => items.clone(),
        _ => Vec::new(),
    };
    let draw_order = layout_node.and_then(|l| l.port_draw_order.as_ref());
    let port_names: Vec<String> = if let Some(order) = draw_order {
        order
            .west
            .iter()
            .chain(order.east.iter())
            .cloned()
            .collect()
    } else if !details.is_empty() {
        details
            .iter()
            .filter_map(|d| d.get("name").and_then(Value::as_str))
            .map(str::to_string)
            .collect()
    } else {
        match node.attributes.get("ports") {
            Some(Value::Array(items)) => items
                .iter()
                .map(|v| match v {
                    Value::String(s) => s.clone(),
                    other => other.to_string(),
                })
                .collect(),
            _ => Vec::new(),
        }
    };
    let empty_anchors = std::collections::BTreeMap::new();
    let anchors = layout_node
        .map(|l| &l.port_anchors)
        .unwrap_or(&empty_anchors);
    let port_size = 10.0;
    let fallback_spacing = 26.0;

    let anchor_for = |name: &str| -> Option<&InterconnectionLayoutPortAnchor> {
        anchors
            .get(&sanitize_port_name(name))
            .or_else(|| anchors.get(name))
    };

    let draw_port = |name: &str, side_index: usize, side: &str, elements: &mut Vec<Element>| {
        let detail = port_detail(&details, name);
        let tooltip_id = detail.map(port_tooltip_id).unwrap_or_default();
        let tooltip_text = if !tooltip_id.is_empty() {
            detail.map(|d| tooltip_fallback_text(&port_tooltip_descriptor(d)))
        } else {
            None
        };
        let anchor = anchor_for(name);
        let resolved_side = anchor
            .map(|a| a.side.as_str())
            .filter(|s| *s == "WEST" || *s == "EAST")
            .unwrap_or(side);
        let x = anchor
            .map(|a| a.x)
            .unwrap_or(if resolved_side == "WEST" { 0.0 } else { width });
        let y = anchor
            .map(|a| a.y)
            .unwrap_or(fallback_start_y + side_index as f64 * fallback_spacing);
        let color = theme.node_border;
        let label_layout = anchor.and_then(|a| a.label.as_ref());
        let label_text = label_layout
            .map(|l| l.text.clone())
            .filter(|t| !t.is_empty())
            .unwrap_or_else(|| ibd_port_label_text(name));

        let mut icon_el = Element::new("rect")
            .attr("class", "port-icon")
            .attr("data-port-name", name)
            .attr("data-port-side", resolved_side)
            .maybe_attr(
                "data-tooltip-kind",
                (!tooltip_id.is_empty()).then_some("port"),
            )
            .maybe_attr(
                "data-tooltip-id",
                (!tooltip_id.is_empty()).then(|| tooltip_id.clone()),
            )
            .attr_f("x", x - port_size / 2.0)
            .attr_f("y", y - port_size / 2.0)
            .attr_f("width", port_size)
            .attr_f("height", port_size)
            .style("fill", "none")
            .style("stroke", color)
            .style("stroke-width", "1.8px")
            .style(
                "pointer-events",
                if !tooltip_id.is_empty() {
                    "all"
                } else {
                    "none"
                },
            )
            .style(
                "cursor",
                if !tooltip_id.is_empty() {
                    "help"
                } else {
                    "default"
                },
            );
        if let Some(text) = &tooltip_text {
            icon_el = icon_el
                .attr("aria-label", text.replace('\n', "; "))
                .child(Element::new("title").text(text.clone()));
        }
        elements.push(icon_el);

        let label_x = label_layout
            .map(|l| l.x)
            .unwrap_or(if resolved_side == "WEST" {
                (width - 10.0).min(x + 16.0)
            } else {
                (x - 16.0).max(10.0)
            });
        let label_y = label_layout
            .map(|l| l.y + l.height - 1.0)
            .unwrap_or(y - 6.0);
        let text_anchor = if label_layout.is_some() || resolved_side == "WEST" {
            "start"
        } else {
            "end"
        };

        let mut label_el = Element::new("text")
            .attr("class", "port-label")
            .attr("data-port-name", name)
            .attr("data-port-side", resolved_side)
            .maybe_attr(
                "data-tooltip-kind",
                (!tooltip_id.is_empty()).then_some("port"),
            )
            .maybe_attr(
                "data-tooltip-id",
                (!tooltip_id.is_empty()).then(|| tooltip_id.clone()),
            )
            .attr_f("x", label_x)
            .attr_f("y", label_y)
            .attr("text-anchor", text_anchor);
        if let Some(label) = label_layout {
            label_el = label_el
                .attr_f("textLength", label.width)
                .attr("lengthAdjust", "spacingAndGlyphs");
        }
        label_el = label_el
            .attr("paint-order", "stroke fill")
            .attr("stroke", theme.canvas_background)
            .attr_f("stroke-width", 3.0)
            .attr("stroke-linejoin", "round")
            .text(label_text)
            .style("font-family", "monospace")
            .style("font-size", format!("{}px", n(IBD_PORT_LABEL_FONT_SIZE)))
            .style("font-weight", "500")
            .style("fill", color)
            .style(
                "pointer-events",
                if !tooltip_id.is_empty() {
                    "all"
                } else {
                    "none"
                },
            )
            .style(
                "cursor",
                if !tooltip_id.is_empty() {
                    "help"
                } else {
                    "default"
                },
            );
        if let Some(text) = &tooltip_text {
            label_el = label_el
                .attr("aria-label", text.replace('\n', "; "))
                .child(Element::new("title").text(text.clone()));
        }
        elements.push(label_el);
    };

    let mut elements = Vec::new();
    if let Some(order) = draw_order {
        for (index, name) in order.west.iter().enumerate() {
            draw_port(name, index, "WEST", &mut elements);
        }
        for (index, name) in order.east.iter().enumerate() {
            draw_port(name, index, "EAST", &mut elements);
        }
        return elements;
    }

    for (index, name) in port_names.iter().enumerate() {
        let side = match anchor_for(name).map(|a| a.side.as_str()) {
            Some("WEST") => "WEST",
            Some("EAST") => "EAST",
            _ => {
                if name.to_lowercase().starts_with("in") {
                    "WEST"
                } else {
                    "EAST"
                }
            }
        };
        draw_port(name, index, side, &mut elements);
    }
    elements
}
