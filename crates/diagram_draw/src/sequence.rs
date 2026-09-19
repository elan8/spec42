//! Port of `views/sequence.ts`'s `renderSequenceView`. Sequence diagrams read from
//! `prepared.meta.sequenceDiagram` (`{lifelines, messages, activations, fragments}`), which is
//! entirely free-form JSON -- there is no typed contract on the TS side either (`asRecord`/
//! `asArray`/`asString` coercers are defined locally in `sequence.ts` for exactly this reason).
//! No ELK dependency at all: positions are five constants plus index arithmetic, so unlike
//! action-flow/state-transition this view needs no separate layout-result fixture.

use serde_json::Value;

use crate::behavior_common::{truncate_label, PreparedNode, PreparedView};
use crate::hit_target::{line_edge_hit_target, mark_visible_edge, path_edge_hit_target};
use crate::svg::{format_number as n, Element};
use crate::theme::Theme;

const HEADER_Y: f64 = 64.0;
const LIFELINE_TOP: f64 = 118.0;
const LIFELINE_GAP: f64 = 220.0;
const MESSAGE_GAP: f64 = 78.0;
const LIFELINE_BOX_WIDTH: f64 = 132.0;
const LIFELINE_BOX_HEIGHT: f64 = 38.0;

fn as_object(value: &Value) -> Option<&serde_json::Map<String, Value>> {
    value.as_object()
}

fn get<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    as_object(value)?.get(key)
}

fn as_array(value: Option<&Value>) -> &[Value] {
    match value {
        Some(Value::Array(items)) => items.as_slice(),
        _ => &[],
    }
}

fn as_str(value: Option<&Value>, fallback: &str) -> String {
    match value {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Bool(b)) => b.to_string(),
        _ => fallback.to_string(),
    }
}

fn as_f64(value: Option<&Value>, fallback: f64) -> f64 {
    match value {
        Some(Value::Number(n)) => n.as_f64().unwrap_or(fallback),
        Some(Value::String(s)) => s.parse().unwrap_or(fallback),
        _ => fallback,
    }
}

/// Port of `messageRow`.
fn message_row(order: f64) -> f64 {
    LIFELINE_TOP + 58.0 + (order.max(1.0) - 1.0) * MESSAGE_GAP
}

/// Port of `messageRef`.
fn message_ref(message: &Value) -> String {
    as_str(
        get(message, "id")
            .or_else(|| get(message, "name"))
            .or_else(|| get(message, "label")),
        "",
    )
}

/// Port of `findPreparedLifeline`: maps a lifeline record back to a `PreparedNode`, if any exists
/// whose id/qualifiedName/label matches. `None` means the raw lifeline id/name is used as-is and
/// -- critically -- `attachBehaviorNodeClick` is never called for this lifeline (see its one call
/// site being `if (preparedNode) { attachBehaviorNodeClick(...) }`, unlike action-flow/
/// state-transition which call it unconditionally for every node).
fn find_prepared_lifeline<'a>(
    prepared: &'a PreparedView,
    lifeline: &Value,
) -> Option<&'a PreparedNode> {
    let id = as_str(get(lifeline, "id").or_else(|| get(lifeline, "name")), "");
    let name = as_str(get(lifeline, "name").or_else(|| get(lifeline, "label")), "");
    prepared.nodes.iter().find(|node| {
        let qualified_name = match node.attributes.get("qualifiedName") {
            Some(Value::String(s)) => s.as_str(),
            _ => "",
        };
        (!id.is_empty() && (node.id == id || qualified_name == id))
            || (!name.is_empty() && (node.label == name || qualified_name == name))
    })
}

struct MessagePosition {
    source_x: f64,
    target_x: f64,
    y: f64,
}

/// Port of `renderSequenceView`. Returns the drawn content plus its `{minX,minY,maxX,maxY}`
/// extents (`contentBoundsFromExtents` turns these into the `viewBox`-feeding bounds elsewhere).
pub fn render_sequence_view(
    prepared: &PreparedView,
    theme: &Theme,
    width: f64,
    height: f64,
) -> (Element, (f64, f64, f64, f64)) {
    let mut root = Element::new("g").attr("class", "viz-root");
    let diagram = prepared
        .meta
        .get("sequenceDiagram")
        .cloned()
        .unwrap_or(Value::Null);
    let lifelines = as_array(get(&diagram, "lifelines")).to_vec();
    let mut messages = as_array(get(&diagram, "messages")).to_vec();
    messages.sort_by(|a, b| as_f64(get(a, "order"), 0.0).total_cmp(&as_f64(get(b, "order"), 0.0)));
    let activations = as_array(get(&diagram, "activations")).to_vec();
    let fragments = as_array(get(&diagram, "fragments")).to_vec();

    root = root.child(
        Element::new("text")
            .attr_f("x", width / 2.0)
            .attr_f("y", 32.0)
            .attr("text-anchor", "middle")
            .style("font-size", "14px")
            .style("font-weight", "700")
            .style("fill", theme.text_primary)
            .text(if prepared.title.is_empty() {
                "Sequence".to_string()
            } else {
                prepared.title.clone()
            }),
    );

    if lifelines.is_empty() || messages.is_empty() {
        root = root.child(
            Element::new("text")
                .attr_f("x", width / 2.0)
                .attr_f("y", height / 2.0)
                .attr("text-anchor", "middle")
                .style("fill", theme.text_secondary)
                .text("No sequence lifelines or messages in payload"),
        );
        return (root, (0.0, 0.0, width, height));
    }

    let x_offset = (80.0_f64).max(
        (width - (((lifelines.len() as f64 - 1.0).max(0.0)) * LIFELINE_GAP + LIFELINE_BOX_WIDTH))
            / 2.0,
    );
    let lifeline_id =
        |lifeline: &Value| as_str(get(lifeline, "id").or_else(|| get(lifeline, "name")), "");
    let lifeline_x: Vec<(String, f64)> = lifelines
        .iter()
        .enumerate()
        .map(|(index, lifeline)| {
            (
                lifeline_id(lifeline),
                x_offset + index as f64 * LIFELINE_GAP,
            )
        })
        .collect();
    let x_for = |id: &str| -> Option<f64> {
        lifeline_x
            .iter()
            .find(|(candidate, _)| candidate == id)
            .map(|(_, x)| *x)
    };

    let last_message_y = messages
        .last()
        .map(|m| message_row(as_f64(get(m, "order"), messages.len() as f64)))
        .unwrap_or(LIFELINE_TOP + 100.0);
    let lifeline_bottom = last_message_y + 140.0;

    let mut lifeline_layer = Element::new("g").attr("class", "sequence-lifelines");
    for lifeline in &lifelines {
        let id = lifeline_id(lifeline);
        let x = x_for(&id).unwrap_or(x_offset);
        let label = truncate_label(
            &as_str(
                get(lifeline, "name")
                    .or_else(|| get(lifeline, "label"))
                    .or(Some(&Value::String(id.clone()))),
                &id,
            ),
            18,
        );
        let prepared_node: Option<&PreparedNode> = find_prepared_lifeline(prepared, lifeline);
        let mut group = Element::new("g").attr("class", "sequence-lifeline").attr(
            "data-node-id",
            prepared_node.map(|node| node.id.as_str()).unwrap_or(&id),
        );
        // `attachBehaviorNodeClick` runs only `if (preparedNode)`; in headless export (no
        // `onNodeClick`) it just sets `.style("cursor", "")`, which -- unlike an omitted/`null`
        // style -- the `VirtualStyle` map keeps as a literal empty declaration, serializing
        // `style="cursor: ;"`. Confirmed against a real rendered fixture, not assumed.
        if prepared_node.is_some() {
            group = group.style("cursor", "");
        }
        let group = group
            .child(
                Element::new("rect")
                    .attr("class", "node-background")
                    .attr("data-original-stroke", theme.node_border)
                    .attr("data-original-width", "1.5px")
                    .attr_f("x", x - LIFELINE_BOX_WIDTH / 2.0)
                    .attr_f("y", HEADER_Y)
                    .attr_f("width", LIFELINE_BOX_WIDTH)
                    .attr_f("height", LIFELINE_BOX_HEIGHT)
                    .attr("rx", "6")
                    .style("fill", theme.node_fill)
                    .style("stroke", theme.node_border)
                    .style("stroke-width", "1.5px"),
            )
            .child(
                Element::new("text")
                    .attr_f("x", x)
                    .attr_f("y", HEADER_Y + 24.0)
                    .attr("text-anchor", "middle")
                    .style("font-size", "11px")
                    .style("fill", theme.text_primary)
                    .text(label),
            )
            .child(
                Element::new("line")
                    .attr_f("x1", x)
                    .attr_f("y1", LIFELINE_TOP)
                    .attr_f("x2", x)
                    .attr_f("y2", lifeline_bottom)
                    .style("stroke", theme.node_border)
                    .style("stroke-dasharray", "6,4"),
            );
        lifeline_layer = lifeline_layer.child(group);
    }
    root = root.child(lifeline_layer);

    let mut message_layer = Element::new("g").attr("class", "sequence-messages");
    let mut message_positions: Vec<(String, MessagePosition)> = Vec::new();
    for message in &messages {
        let source_id = as_str(get(message, "source").or_else(|| get(message, "from")), "");
        let target_id = as_str(get(message, "target").or_else(|| get(message, "to")), "");
        let (Some(source_x), Some(target_x)) = (x_for(&source_id), x_for(&target_id)) else {
            continue;
        };
        let y = message_row(as_f64(get(message, "order"), 1.0));
        let message_id = message_ref(message);
        message_positions.push((
            message_id.clone(),
            MessagePosition {
                source_x,
                target_x,
                y,
            },
        ));
        let kind = as_str(get(message, "kind").or_else(|| get(message, "type")), "").to_lowercase();
        let edge_id = as_str(get(message, "id"), &message_id);
        let is_return = kind.contains("return") || kind.contains("reply");
        let is_self = source_id == target_id;

        // Sequence messages are never `PreparedEdge`s (`prepared.edges` stays empty for this
        // view -- messages live only in `meta.sequenceDiagram`), so `installDiagramTooltips`'
        // descriptor map, keyed off `prepared.edges`, never has an entry for a message id: no
        // `aria-label`/`<title>` is ever baked into a sequence hit target. Verified against a real
        // rendered fixture, not assumed.
        if is_self {
            let path = format!(
                "M{},{} C{},{} {},{} {},{}",
                n(source_x),
                n(y),
                n(source_x + 84.0),
                n(y - 18.0),
                n(source_x + 84.0),
                n(y + 34.0),
                n(source_x),
                n(y + 28.0)
            );
            let visible = Element::new("path")
                .attr(
                    "class",
                    format!(
                        "sequence-message sequence-message-self{}",
                        if is_return {
                            " sequence-message-return"
                        } else {
                            ""
                        }
                    ),
                )
                .attr("d", path.clone())
                .style("fill", "none")
                .style("stroke", theme.edge_default)
                .style("stroke-width", "1.8px")
                .style("stroke-dasharray", if is_return { "6,4" } else { "none" })
                .style("marker-end", "url(#sequence-arrow-sync)");
            message_layer = message_layer.child(mark_visible_edge(visible, &edge_id, "1.8"));
            message_layer = message_layer.child(path_edge_hit_target(&path, &edge_id, None));
        } else {
            let visible = Element::new("line")
                .attr(
                    "class",
                    format!(
                        "sequence-message{}",
                        if is_return {
                            " sequence-message-return"
                        } else {
                            ""
                        }
                    ),
                )
                .attr_f("x1", source_x)
                .attr_f("y1", y)
                .attr_f("x2", target_x)
                .attr_f("y2", y)
                .style("stroke", theme.edge_default)
                .style("stroke-width", "1.8px")
                .style("stroke-dasharray", if is_return { "6,4" } else { "none" })
                .style("marker-end", "url(#sequence-arrow-sync)");
            message_layer = message_layer.child(mark_visible_edge(visible, &edge_id, "1.8"));
            message_layer = message_layer.child(line_edge_hit_target(
                source_x, y, target_x, y, &edge_id, None,
            ));
        }
        let label = truncate_label(
            &as_str(get(message, "name").or_else(|| get(message, "label")), ""),
            28,
        );
        if !label.is_empty() {
            message_layer = message_layer.child(
                Element::new("text")
                    .attr_f("x", (source_x + target_x) / 2.0)
                    .attr_f("y", y - 8.0)
                    .attr("text-anchor", "middle")
                    .style("font-size", "10px")
                    .style("fill", theme.text_secondary)
                    .text(label),
            );
        }
    }
    root = root.child(message_layer);

    let position_for = |id: &str| -> Option<&MessagePosition> {
        message_positions
            .iter()
            .find(|(candidate, _)| candidate == id)
            .map(|(_, position)| position)
    };

    let mut activation_layer = Element::new("g").attr("class", "sequence-activations");
    for activation in &activations {
        let lifeline_ref = as_str(
            get(activation, "on_lifeline")
                .or_else(|| get(activation, "onLifeline"))
                .or_else(|| get(activation, "lifeline"))
                .or_else(|| get(activation, "on")),
            "",
        );
        let Some(x) = x_for(&lifeline_ref) else {
            continue;
        };
        let start_ref = as_str(
            get(activation, "start_message")
                .or_else(|| get(activation, "startMessage"))
                .or_else(|| get(activation, "start")),
            "",
        );
        let finish_ref = as_str(
            get(activation, "finish_message")
                .or_else(|| get(activation, "finishMessage"))
                .or_else(|| get(activation, "finish")),
            "",
        );
        let start_y = position_for(&start_ref)
            .map(|p| p.y)
            .unwrap_or(LIFELINE_TOP + 36.0);
        let finish_y = position_for(&finish_ref)
            .map(|p| p.y)
            .unwrap_or(start_y + MESSAGE_GAP);
        activation_layer = activation_layer.child(
            Element::new("rect")
                .attr("class", "sequence-activation")
                .attr_f("x", x - 7.0)
                .attr_f("y", start_y + 6.0)
                .attr_f("width", 14.0)
                .attr_f("height", (finish_y - start_y + 18.0).max(34.0))
                .attr("rx", "3")
                .style("fill", theme.node_fill)
                .style("stroke", theme.node_border)
                .style("stroke-width", "1px"),
        );
    }
    root = root.child(activation_layer);

    // `ctx.root.insert("g", ".sequence-messages")` in the TS source puts fragments *before*
    // messages in document order -- matched here by inserting this child ahead of the
    // already-appended `.sequence-messages` group.
    let mut fragment_layer = Element::new("g").attr("class", "sequence-fragments");
    for fragment in &fragments {
        let kind = as_str(
            get(fragment, "kind").or_else(|| get(fragment, "type")),
            "fragment",
        );
        let operands = as_array(get(fragment, "operands")).to_vec();
        let mut referenced_messages: Vec<String> = Vec::new();
        for operand in &operands {
            for id in as_array(
                get(operand, "message_ids")
                    .or_else(|| get(operand, "messageIds"))
                    .or_else(|| get(operand, "messages")),
            ) {
                let id = as_str(Some(id), "");
                if !referenced_messages.contains(&id) {
                    referenced_messages.push(id);
                }
            }
        }
        let mut matching: Vec<&MessagePosition> = referenced_messages
            .iter()
            .filter_map(|id| position_for(id))
            .collect();
        if matching.is_empty() && !messages.is_empty() {
            matching = message_positions
                .iter()
                .map(|(_, position)| position)
                .collect();
        }
        if matching.is_empty() {
            continue;
        }
        let min_x = matching
            .iter()
            .map(|p| p.source_x.min(p.target_x))
            .fold(f64::INFINITY, f64::min)
            - 58.0;
        let max_x = matching
            .iter()
            .map(|p| p.source_x.max(p.target_x))
            .fold(f64::NEG_INFINITY, f64::max)
            + 58.0;
        let min_y = matching.iter().map(|p| p.y).fold(f64::INFINITY, f64::min) - 34.0;
        let max_y = matching
            .iter()
            .map(|p| p.y)
            .fold(f64::NEG_INFINITY, f64::max)
            + 34.0
            + (operands.len() as f64 - 1.0).max(0.0) * 28.0;
        let mut fragment_box = Element::new("g").attr(
            "class",
            format!(
                "sequence-fragment sequence-fragment-{}",
                kind.to_lowercase()
            ),
        );
        fragment_box = fragment_box.child(
            Element::new("rect")
                .attr_f("x", min_x)
                .attr_f("y", min_y)
                .attr_f("width", max_x - min_x)
                .attr_f("height", max_y - min_y)
                .attr("rx", "4")
                .style("fill", "none")
                .style("stroke", theme.node_border)
                .style("stroke-dasharray", "7,4"),
        );
        fragment_box = fragment_box.child(
            Element::new("path")
                .attr(
                    "d",
                    format!(
                        "M{},{} L{},{} L{},{} L{},{}",
                        n(min_x),
                        n(min_y + 24.0),
                        n(min_x + 72.0),
                        n(min_y + 24.0),
                        n(min_x + 90.0),
                        n(min_y),
                        n(min_x),
                        n(min_y)
                    ),
                )
                .style("fill", theme.canvas_background)
                .style("stroke", theme.node_border),
        );
        fragment_box = fragment_box.child(
            Element::new("text")
                .attr_f("x", min_x + 10.0)
                .attr_f("y", min_y + 16.0)
                .style("font-size", "10px")
                .style("font-weight", "700")
                .style("fill", theme.text_primary)
                .text(kind.clone()),
        );
        for (index, operand) in operands.iter().enumerate() {
            let guard = as_str(
                get(operand, "guard").or_else(|| get(operand, "condition")),
                "",
            );
            if index > 0 {
                let y = min_y + 28.0 + index as f64 * 28.0;
                fragment_box = fragment_box.child(
                    Element::new("line")
                        .attr_f("x1", min_x)
                        .attr_f("x2", max_x)
                        .attr_f("y1", y)
                        .attr_f("y2", y)
                        .style("stroke", theme.node_border)
                        .style("stroke-dasharray", "4,3"),
                );
            }
            if !guard.is_empty() {
                fragment_box = fragment_box.child(
                    Element::new("text")
                        .attr("class", "sequence-fragment-guard")
                        .attr_f("x", min_x + 12.0)
                        .attr_f("y", min_y + 44.0 + index as f64 * 28.0)
                        .style("font-size", "9px")
                        .style("fill", theme.text_secondary)
                        .text(format!("[{}]", truncate_label(&guard, 28))),
                );
            }
        }
        fragment_layer = fragment_layer.child(fragment_box);
    }
    root = root.insert_before_child("sequence-messages", fragment_layer);

    let extents = (
        0.0,
        0.0,
        x_offset + lifelines.len() as f64 * LIFELINE_GAP + 80.0,
        lifeline_bottom + 60.0,
    );
    (root, extents)
}

/// Port of `addSequenceMarkers`.
pub fn sequence_marker(theme: &Theme) -> Element {
    Element::new("marker")
        .attr("id", "sequence-arrow-sync")
        .attr("viewBox", "0 -5 10 10")
        .attr("refX", "9")
        .attr("refY", "0")
        .attr("markerWidth", "8")
        .attr("markerHeight", "8")
        .attr("orient", "auto")
        .child(
            Element::new("path")
                .attr("d", "M0,-5L10,0L0,5")
                .style("fill", theme.edge_default),
        )
}
