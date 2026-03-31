use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range};

use crate::views::ibd;
use crate::semantic_model::{NodeId, SemanticGraph, SemanticNode};

pub(super) fn is_port_like(kind: &str) -> bool {
    ibd::is_port_like(kind)
}

pub(super) fn diag(
    range: Range,
    severity: DiagnosticSeverity,
    source: &str,
    code: &str,
    message: String,
) -> Diagnostic {
    Diagnostic {
        range,
        severity: Some(severity),
        code: Some(NumberOrString::String(code.to_string())),
        code_description: None,
        source: Some(source.to_string()),
        message,
        related_information: None,
        tags: None,
        data: None,
    }
}

pub(super) fn is_unknown_range(range: Range) -> bool {
    range.start.line == 0
        && range.start.character == 0
        && range.end.line == 0
        && range.end.character == 0
}

pub(super) fn is_declaration_port(graph: &SemanticGraph, node: &SemanticNode) -> bool {
    let Some(parent_id) = &node.parent_id else {
        return false;
    };
    let Some(parent) = graph.get_node(parent_id) else {
        return false;
    };
    parent.element_kind == "part def" || parent.element_kind == "part"
}

pub(super) fn is_synthetic(node: &SemanticNode) -> bool {
    node.attributes
        .get("synthetic")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

pub(super) fn parse_origin_range(node: &SemanticNode) -> Option<Range> {
    let origin = node.attributes.get("originRange")?;
    let start = origin.get("start")?;
    let end = origin.get("end")?;
    Some(Range {
        start: Position {
            line: start.get("line")?.as_u64()? as u32,
            character: start.get("character")?.as_u64()? as u32,
        },
        end: Position {
            line: end.get("line")?.as_u64()? as u32,
            character: end.get("character")?.as_u64()? as u32,
        },
    })
}

fn preferred_port_anchor_range(node: &SemanticNode) -> Option<Range> {
    if is_synthetic(node) {
        if let Some(origin) = parse_origin_range(node) {
            if !is_unknown_range(origin) {
                return Some(origin);
            }
        }
    }
    if !is_unknown_range(node.range) {
        return Some(node.range);
    }
    if let Some(origin) = parse_origin_range(node) {
        if !is_unknown_range(origin) {
            return Some(origin);
        }
    }
    None
}

pub(super) fn port_anchor_key(node: &SemanticNode) -> Option<String> {
    let r = preferred_port_anchor_range(node)?;
    Some(format!(
        "{}:{}:{}:{}:{}",
        r.start.line, r.start.character, r.end.line, r.end.character, node.name
    ))
}

pub(super) fn diagnostic_range(
    graph: &SemanticGraph,
    node: &SemanticNode,
    peer: Option<&SemanticNode>,
) -> Range {
    if node.element_kind == "port" {
        if let Some(range) = preferred_port_anchor_range(node) {
            return range;
        }
    }
    if !is_unknown_range(node.range) {
        return node.range;
    }
    if let Some(parent_id) = &node.parent_id {
        if let Some(parent) = graph.get_node(parent_id) {
            if !is_unknown_range(parent.range) {
                return parent.range;
            }
        }
    }
    if let Some(peer) = peer {
        if !is_unknown_range(peer.range) {
            return peer.range;
        }
    }
    node.range
}

pub(super) fn normalize_edge_pair(a: &NodeId, b: &NodeId) -> (NodeId, NodeId) {
    if a.qualified_name <= b.qualified_name {
        (a.clone(), b.clone())
    } else {
        (b.clone(), a.clone())
    }
}

pub(super) fn port_type_mismatch(src: &SemanticNode, tgt: &SemanticNode) -> Option<String> {
    let src_type = src.attributes.get("portType").and_then(|v| v.as_str())?;
    let tgt_type = tgt.attributes.get("portType").and_then(|v| v.as_str())?;

    let (src_base, src_conj) = parse_port_type(src_type);
    let (tgt_base, tgt_conj) = parse_port_type(tgt_type);

    if src_base != tgt_base {
        return Some(format!(
            "Port types do not match: '{}' and '{}' (expected same base type, one conjugated).",
            src_type, tgt_type
        ));
    }
    if src_conj == tgt_conj {
        return Some(format!(
            "Both ports have the same conjugation ({}). For a connection, one should be conjugated (~) and the other not.",
            src_type
        ));
    }
    None
}

pub(super) fn parse_port_type(s: &str) -> (String, bool) {
    let t = s.trim();
    let (conj, base) = if let Some(stripped) = t.strip_prefix('~') {
        (true, stripped.trim())
    } else {
        (false, t)
    };
    (base.to_string(), conj)
}

pub(super) fn normalize_declared_type_ref(type_ref: &str) -> String {
    type_ref
        .trim()
        .strip_prefix('~')
        .map(str::trim)
        .unwrap_or(type_ref.trim())
        .to_string()
}

pub(super) fn is_builtin_type_ref(type_ref: &str) -> bool {
    matches!(type_ref, "String")
}

pub(super) fn unresolved_type_diagnostic_range(node: &SemanticNode) -> Option<Range> {
    if !is_unknown_range(node.range) {
        return Some(node.range);
    }
    None
}

pub(super) fn declared_type_ref(node: &SemanticNode) -> Option<&str> {
    [
        "partType",
        "attributeType",
        "portType",
        "actionType",
        "actorType",
        "itemType",
        "occurrenceType",
        "flowType",
        "allocationType",
        "stateType",
        "requirementType",
        "useCaseType",
        "concernType",
    ]
    .iter()
    .find_map(|k| {
        node.attributes
            .get(*k)
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
    })
}

pub(super) fn multiplicity_issue_message(multiplicity: &str) -> Option<String> {
    let normalized = multiplicity
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']');
    if normalized.is_empty() {
        return Some("empty multiplicity".to_string());
    }
    let Some((lower_raw, upper_raw)) = normalized.split_once("..") else {
        return validate_single_multiplicity_value(normalized);
    };
    let lower = match parse_non_negative_bound(lower_raw.trim()) {
        Ok(value) => value,
        Err(error) => return Some(error),
    };
    let upper = if upper_raw.trim() == "*" {
        None
    } else {
        match parse_non_negative_bound(upper_raw.trim()) {
            Ok(value) => Some(value),
            Err(error) => return Some(error),
        }
    };
    if let Some(upper) = upper {
        if lower > upper {
            return Some(format!(
                "lower bound {lower} is greater than upper bound {upper}"
            ));
        }
    }
    None
}

pub(super) fn validate_single_multiplicity_value(raw: &str) -> Option<String> {
    if raw == "*" {
        return None;
    }
    parse_non_negative_bound(raw).err()
}

pub(super) fn parse_non_negative_bound(raw: &str) -> Result<i64, String> {
    match raw.parse::<i64>() {
        Ok(value) if value >= 0 => Ok(value),
        Ok(value) => Err(format!("bound {value} must be non-negative")),
        Err(_) => Err(format!("bound '{raw}' is not an integer or '*'")),
    }
}
