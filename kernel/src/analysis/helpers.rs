use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range};

use crate::semantic::{NodeId, SemanticGraph, SemanticNode};
use crate::views::ibd;

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
    parent.element_kind == "part" && !is_synthetic(parent)
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

pub(super) fn port_compatibility_mismatch(
    graph: &SemanticGraph,
    src: &SemanticNode,
    tgt: &SemanticNode,
) -> Option<String> {
    let src_type = src.attributes.get("portType").and_then(|v| v.as_str())?;
    let tgt_type = tgt.attributes.get("portType").and_then(|v| v.as_str())?;

    let (src_base, src_conj) = parse_port_type(src_type);
    let (tgt_base, tgt_conj) = parse_port_type(tgt_type);

    let src_features = effective_port_features(graph, src, src_conj);
    let tgt_features = effective_port_features(graph, tgt, tgt_conj);
    if !src_features.is_empty() && !tgt_features.is_empty() {
        if ports_feature_compatible(&src_features, &tgt_features) {
            return None;
        }
        return Some(format!(
            "Port definitions '{}' and '{}' are not feature-compatible.",
            src_type, tgt_type
        ));
    }

    if src_base != tgt_base {
        return Some(format!(
            "Port types do not match: '{}' and '{}' (expected same base type, one conjugated).",
            src_type, tgt_type
        ));
    }
    if src_conj && tgt_conj {
        return Some(format!(
            "Both ports have the same conjugation ({}). For a connection, one should be conjugated (~) and the other not.",
            src_type
        ));
    }
    None
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PortFeature {
    direction: FeatureDirection,
    normalized_type: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FeatureDirection {
    In,
    Out,
    InOut,
}

fn effective_port_features(
    graph: &SemanticGraph,
    port: &SemanticNode,
    conjugated: bool,
) -> Vec<PortFeature> {
    let mut features = Vec::new();
    for typed in graph.outgoing_typing_or_specializes_targets(port) {
        if typed.element_kind != "port def" {
            continue;
        }
        for child in graph.children_of(typed) {
            if child.element_kind != "in out parameter" {
                continue;
            }
            let Some(direction) = child
                .attributes
                .get("direction")
                .and_then(|v| v.as_str())
                .and_then(parse_feature_direction)
            else {
                continue;
            };
            let Some(parameter_type) = child
                .attributes
                .get("parameterType")
                .and_then(|v| v.as_str())
            else {
                continue;
            };
            let mut effective_direction = direction;
            if conjugated {
                effective_direction = conjugated_direction(effective_direction);
            }
            features.push(PortFeature {
                direction: effective_direction,
                normalized_type: normalize_declared_type_ref(parameter_type),
            });
        }
    }
    features.sort_by(|a, b| {
        a.normalized_type.cmp(&b.normalized_type).then_with(|| {
            feature_direction_rank(a.direction).cmp(&feature_direction_rank(b.direction))
        })
    });
    features.dedup();
    features
}

fn parse_feature_direction(raw: &str) -> Option<FeatureDirection> {
    match raw {
        "in" => Some(FeatureDirection::In),
        "out" => Some(FeatureDirection::Out),
        "inout" => Some(FeatureDirection::InOut),
        _ => None,
    }
}

fn conjugated_direction(direction: FeatureDirection) -> FeatureDirection {
    match direction {
        FeatureDirection::In => FeatureDirection::Out,
        FeatureDirection::Out => FeatureDirection::In,
        FeatureDirection::InOut => FeatureDirection::InOut,
    }
}

fn ports_feature_compatible(src: &[PortFeature], tgt: &[PortFeature]) -> bool {
    src.iter().all(|src_feature| {
        tgt.iter()
            .any(|tgt_feature| feature_pair_compatible(src_feature, tgt_feature))
    }) && tgt.iter().all(|tgt_feature| {
        src.iter()
            .any(|src_feature| feature_pair_compatible(src_feature, tgt_feature))
    })
}

fn feature_pair_compatible(src: &PortFeature, tgt: &PortFeature) -> bool {
    src.normalized_type == tgt.normalized_type
        && matches!(
            (src.direction, tgt.direction),
            (FeatureDirection::In, FeatureDirection::Out)
                | (FeatureDirection::Out, FeatureDirection::In)
                | (FeatureDirection::InOut, FeatureDirection::InOut)
        )
}

fn feature_direction_rank(direction: FeatureDirection) -> u8 {
    match direction {
        FeatureDirection::In => 0,
        FeatureDirection::Out => 1,
        FeatureDirection::InOut => 2,
    }
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

pub(super) fn declared_specializes_refs(node: &SemanticNode) -> Vec<String> {
    let Some(raw) = node.attributes.get("specializes") else {
        return Vec::new();
    };
    match raw {
        serde_json::Value::String(value) => value
            .split(',')
            .map(str::trim)
            .filter(|item| !item.is_empty())
            .map(ToOwned::to_owned)
            .collect(),
        serde_json::Value::Array(items) => items
            .iter()
            .filter_map(|item| item.as_str())
            .flat_map(|item| {
                item.split(',')
                    .map(str::trim)
                    .filter(|entry| !entry.is_empty())
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .collect(),
        _ => Vec::new(),
    }
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
