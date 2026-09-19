//! Port of the edge-tooltip half of `render/diagram-tooltip.ts`. The hover/positioning machinery
//! in that file is interaction-only and irrelevant to a static SVG string; what *does* end up in
//! the exported SVG is `aria-label` and a `<title>` child on each `[data-tooltip-kind]` element,
//! computed once by `installDiagramTooltips` and baked in before `exportSvg()` serializes. Only
//! the General-View edge branch of `edgeTooltipDescriptor` is ported -- port descriptors and the
//! other views' branches are out of scope for this spike.

use std::collections::HashMap;

use serde_json::Value;

use crate::behavior_common::{PreparedEdge, PreparedNode};
use crate::types::{attr_text, LaidOutEdge, LaidOutNode};

pub struct TooltipDescriptor {
    pub title: String,
    pub rows: Vec<(&'static str, String)>,
    pub technical_rows: Vec<(&'static str, String)>,
}

/// Port of `humanize` in `render/diagram-tooltip.ts`.
fn humanize(value: &str) -> String {
    let mut collapsed = String::new();
    let mut in_run = false;
    for ch in value.chars() {
        if ch == '_' || ch == '-' {
            if !in_run {
                collapsed.push(' ');
            }
            in_run = true;
        } else {
            collapsed.push(ch);
            in_run = false;
        }
    }
    let chars: Vec<char> = collapsed.chars().collect();
    let mut spaced = String::new();
    for (index, ch) in chars.iter().enumerate() {
        spaced.push(*ch);
        if ch.is_ascii_lowercase()
            && chars
                .get(index + 1)
                .is_some_and(|next| next.is_ascii_uppercase())
        {
            spaced.push(' ');
        }
    }
    let trimmed = spaced.trim();
    if trimmed.is_empty() {
        return "Relationship".to_string();
    }
    let mut chars = trimmed.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => "Relationship".to_string(),
    }
}

fn push_row(rows: &mut Vec<(&'static str, String)>, label: &'static str, value: &str) {
    let trimmed = value.trim();
    if !trimmed.is_empty() {
        rows.push((label, trimmed.to_string()));
    }
}

fn node_label(nodes_by_id: &HashMap<&str, &LaidOutNode>, id: &str) -> String {
    match nodes_by_id.get(id) {
        Some(node) if !node.label.is_empty() => node.label.clone(),
        _ => id.to_string(),
    }
}

const GENERIC_EDGE_LABELS: &[&str] = &[
    "",
    "bind",
    "binding",
    "connect",
    "connection",
    "flow",
    "succession",
    "transition",
    "message",
];

/// Port of the non-`interconnection`/`action-flow`/`state-transition`/`sequence` branch of
/// `edgeTooltipDescriptor` -- the only branch that applies to General View.
pub fn edge_tooltip_descriptor(
    edge: &LaidOutEdge,
    nodes_by_id: &HashMap<&str, &LaidOutNode>,
) -> TooltipDescriptor {
    let relation_type = attr_text(&edge.attributes, "relationType");
    let kind_attr = attr_text(&edge.attributes, "kind");
    let raw_kind = if !relation_type.is_empty() {
        relation_type
    } else if !kind_attr.is_empty() {
        kind_attr
    } else if let Some(edge_kind) = &edge.edge_kind {
        edge_kind.clone()
    } else {
        edge.label.clone()
    };
    let title = humanize(if raw_kind.is_empty() {
        "relationship"
    } else {
        &raw_kind
    });

    let raw_kind_lower = raw_kind.to_lowercase();
    let label_lower = edge.label.trim().to_lowercase();
    let is_generic =
        GENERIC_EDGE_LABELS.contains(&label_lower.as_str()) || label_lower == raw_kind_lower;

    let mut rows = Vec::new();
    let mut technical_rows = Vec::new();
    if !is_generic {
        push_row(&mut rows, "Name", &edge.label);
    }
    push_row(&mut rows, "From", &node_label(nodes_by_id, &edge.source));
    push_row(&mut rows, "To", &node_label(nodes_by_id, &edge.target));
    push_row(&mut technical_rows, "Source ID", &edge.source);
    push_row(&mut technical_rows, "Target ID", &edge.target);
    push_row(
        &mut technical_rows,
        "Semantic ID",
        &attr_text(&edge.attributes, "semanticId"),
    );

    TooltipDescriptor {
        title,
        rows,
        technical_rows,
    }
}

fn behavior_node_label(nodes_by_id: &HashMap<&str, &PreparedNode>, id: &str) -> String {
    match nodes_by_id.get(id) {
        Some(node) if !node.label.is_empty() => node.label.clone(),
        _ => id.to_string(),
    }
}

/// Port of the `action-flow-view`/`state-transition-view`/`sequence-view` branches of
/// `edgeTooltipDescriptor` (the `interconnection-view` and default/General-View branches are
/// covered by `edge_tooltip_descriptor` above, which operates on the unrelated `LaidOutEdge`
/// shape). Sequence never actually resolves this descriptor in practice -- see `sequence.rs`'s
/// module docs -- but the branch is ported for completeness/future reuse.
pub fn behavior_edge_tooltip_descriptor(
    view: &str,
    edge: &PreparedEdge,
    nodes_by_id: &HashMap<&str, &PreparedNode>,
) -> TooltipDescriptor {
    let attrs = &edge.attributes;
    let raw_kind = match view {
        "sequence-view" => "message".to_string(),
        "state-transition-view" => "transition".to_string(),
        "action-flow-view" => {
            let succession_flag = matches!(attrs.get("succession"), Some(Value::Bool(true)));
            let flow_kind = attr_text(attrs, "flowKind").to_lowercase();
            if succession_flag || flow_kind == "succession" {
                "succession".to_string()
            } else {
                "flow".to_string()
            }
        }
        _ => "relationship".to_string(),
    };
    let title = humanize(if raw_kind.is_empty() {
        if view == "sequence-view" {
            "message"
        } else {
            "relationship"
        }
    } else {
        &raw_kind
    });

    let raw_kind_lower = raw_kind.to_lowercase();
    let label_lower = edge.label.trim().to_lowercase();
    let is_generic =
        GENERIC_EDGE_LABELS.contains(&label_lower.as_str()) || label_lower == raw_kind_lower;

    let mut rows = Vec::new();
    let mut technical_rows = Vec::new();
    if !is_generic {
        push_row(&mut rows, "Name", &edge.label);
    }
    push_row(
        &mut rows,
        "From",
        &behavior_node_label(nodes_by_id, &edge.source),
    );
    push_row(
        &mut rows,
        "To",
        &behavior_node_label(nodes_by_id, &edge.target),
    );
    push_row(&mut technical_rows, "Source ID", &edge.source);
    push_row(&mut technical_rows, "Target ID", &edge.target);

    match view {
        "action-flow-view" => {
            let guard = attr_text(attrs, "guard");
            let guard_lower = guard.to_lowercase();
            if !guard.is_empty()
                && !["flow", "first", "succession", "succession flow"]
                    .contains(&guard_lower.as_str())
            {
                push_row(&mut rows, "Guard", &guard);
            }
            push_row(&mut rows, "Condition", &attr_text(attrs, "condition"));
        }
        "state-transition-view" => {
            push_row(&mut rows, "Trigger", &attr_text(attrs, "trigger"));
            push_row(&mut rows, "Accept", &attr_text(attrs, "accept"));
            push_row(&mut rows, "Guard", &attr_text(attrs, "guard"));
            push_row(&mut rows, "Effect", &attr_text(attrs, "effect"));
            push_row(&mut rows, "Send", &attr_text(attrs, "send"));
        }
        "sequence-view" => {
            let message_kind = attr_text(attrs, "messageKind");
            let kind = if !message_kind.is_empty() {
                message_kind
            } else {
                attr_text(attrs, "kind")
            };
            push_row(&mut rows, "Message kind", &kind);
            push_row(&mut rows, "Order", &attr_text(attrs, "order"));
        }
        _ => {}
    }
    push_row(
        &mut technical_rows,
        "Semantic ID",
        &attr_text(attrs, "semanticId"),
    );

    TooltipDescriptor {
        title,
        rows,
        technical_rows,
    }
}

/// Port of `tooltipFallbackText`.
pub fn tooltip_fallback_text(descriptor: &TooltipDescriptor) -> String {
    let mut lines = vec![descriptor.title.clone()];
    lines.extend(
        descriptor
            .rows
            .iter()
            .map(|(label, value)| format!("{label}: {value}")),
    );
    for (label, value) in &descriptor.technical_rows {
        let duplicate = descriptor
            .rows
            .iter()
            .any(|(row_label, row_value)| row_label == label && row_value == value);
        if !duplicate {
            lines.push(format!("{label}: {value}"));
        }
    }
    lines.join("\n")
}
