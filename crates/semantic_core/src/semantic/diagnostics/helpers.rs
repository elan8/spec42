use crate::semantic::diagnostics::types::{DiagnosticSeverity, SemanticDiagnostic};

use crate::{NodeId, SemanticGraph, SemanticNode, TextPosition, TextRange};

use crate::semantic::ibd;

pub(super) fn is_port_like(kind: &str) -> bool {
    ibd::is_port_like(kind)
}

pub(super) fn is_part_like(kind: &str) -> bool {
    ibd::is_part_like(kind)
}

pub(super) fn diag(
    uri: &url::Url,
    range: TextRange,
    severity: DiagnosticSeverity,
    source: &str,
    code: &str,
    message: String,
) -> SemanticDiagnostic {
    SemanticDiagnostic {
        uri: uri.clone(),
        range,
        severity,
        source: source.to_string(),
        code: code.to_string(),
        message,
        related_information: Vec::new(),
    }
}

pub(super) fn is_unknown_range(range: TextRange) -> bool {
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

pub(super) fn parse_attribute_text_range(
    node: &SemanticNode,
    key: &str,
) -> Option<TextRange> {
    let entry = node.attributes.get(key)?;
    let start = entry.get("start")?;
    let end = entry.get("end")?;
    Some(TextRange {
        start: TextPosition {
            line: start.get("line")?.as_u64()? as u32,
            character: start.get("character")?.as_u64()? as u32,
        },
        end: TextPosition {
            line: end.get("line")?.as_u64()? as u32,
            character: end.get("character")?.as_u64()? as u32,
        },
    })
}

pub(super) fn parse_origin_range(node: &SemanticNode) -> Option<TextRange> {
    let origin = node.attributes.get("originRange")?;
    let start = origin.get("start")?;
    let end = origin.get("end")?;
    Some(TextRange {
        start: TextPosition {
            line: start.get("line")?.as_u64()? as u32,
            character: start.get("character")?.as_u64()? as u32,
        },
        end: TextPosition {
            line: end.get("line")?.as_u64()? as u32,
            character: end.get("character")?.as_u64()? as u32,
        },
    })
}

fn preferred_port_anchor_range(node: &SemanticNode) -> Option<TextRange> {
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
) -> TextRange {
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

pub(super) fn reference_token_range(node: &SemanticNode, raw_reference: &str) -> Option<TextRange> {
    let reference = raw_reference.trim();
    if reference.is_empty() || is_unknown_range(node.range) {
        return None;
    }
    let source = source_text_for_node(node)?;
    find_reference_in_source_range(&source, node.range, reference)
}

fn source_text_for_node(node: &SemanticNode) -> Option<String> {
    let path = node.id.uri.to_file_path().ok()?;
    std::fs::read_to_string(path).ok()
}

fn find_reference_in_source_range(
    source: &str,
    range: TextRange,
    reference: &str,
) -> Option<TextRange> {
    let lines: Vec<&str> = source.lines().collect();
    if range.start.line as usize >= lines.len() || range.end.line as usize >= lines.len() {
        return None;
    }
    let normalized_reference = reference.trim();
    let simple_reference = normalized_reference
        .rsplit("::")
        .next()
        .unwrap_or(normalized_reference)
        .trim_end_matches("::*")
        .trim_end_matches("::")
        .trim();
    let candidates = if simple_reference != normalized_reference && !simple_reference.is_empty() {
        vec![normalized_reference, simple_reference]
    } else {
        vec![normalized_reference]
    };

    for line_no in range.start.line..=range.end.line {
        let line = lines.get(line_no as usize)?;
        let start_char = if line_no == range.start.line {
            range.start.character
        } else {
            0
        };
        let end_char = if line_no == range.end.line {
            range.end.character
        } else {
            line.chars().count() as u32
        };
        for candidate in &candidates {
            if let Some((start, end)) = find_token_in_line(line, start_char, end_char, candidate) {
                return Some(TextRange::new(
                    TextPosition::new(line_no, start),
                    TextPosition::new(line_no, end),
                ));
            }
        }
    }
    None
}

fn find_token_in_line(
    line: &str,
    start_char: u32,
    end_char: u32,
    needle: &str,
) -> Option<(u32, u32)> {
    if needle.is_empty() {
        return None;
    }
    let line_len = line.chars().count() as u32;
    let bounded_end = end_char.min(line_len);
    if start_char >= bounded_end {
        return None;
    }
    let start_byte = char_to_byte_index(line, start_char);
    let end_byte = char_to_byte_index(line, bounded_end);
    let search = &line[start_byte..end_byte];
    let mut offset = 0usize;
    while let Some(found) = search[offset..].find(needle) {
        let byte_start = offset + found;
        let byte_end = byte_start + needle.len();
        let before = search[..byte_start].chars().next_back();
        let after = search[byte_end..].chars().next();
        if !before.is_some_and(is_reference_char) && !after.is_some_and(is_reference_char) {
            let char_start = start_char + search[..byte_start].chars().count() as u32;
            let char_end = char_start + needle.chars().count() as u32;
            return Some((char_start, char_end));
        }
        offset = byte_end;
    }
    None
}

fn char_to_byte_index(text: &str, char_index: u32) -> usize {
    text.char_indices()
        .nth(char_index as usize)
        .map(|(idx, _)| idx)
        .unwrap_or(text.len())
}

fn is_reference_char(ch: char) -> bool {
    ch.is_alphanumeric() || matches!(ch, '_' | '-' | ':' | '.')
}

pub(super) fn normalize_edge_pair(a: &NodeId, b: &NodeId) -> (NodeId, NodeId) {
    if a.qualified_name <= b.qualified_name {
        (a.clone(), b.clone())
    } else {
        (b.clone(), a.clone())
    }
}

/// Key for duplicate-connection detection. Prefer textual connect endpoints so
/// distinct usage paths (e.g. `propulsionUnit1.cmd` vs `propulsionUnit2.cmd`) are
/// not collapsed to the same typed port feature on a part def.
pub(super) fn connection_duplicate_key(
    source_endpoint: Option<&str>,
    target_endpoint: Option<&str>,
    source_id: &NodeId,
    target_id: &NodeId,
) -> String {
    if let (Some(source), Some(target)) = (source_endpoint, target_endpoint) {
        let (left, right) = normalize_edge_pair(source_id, target_id);
        let mut endpoints = [source.replace('.', "::"), target.replace('.', "::")];
        endpoints.sort();
        return format!(
            "expr:{}|{}|node:{}|{}",
            endpoints[0], endpoints[1], left.qualified_name, right.qualified_name
        );
    }
    let (left, right) = normalize_edge_pair(source_id, target_id);
    format!("node:{}|{}", left.qualified_name, right.qualified_name)
}

fn port_definition_qualified_name(graph: &SemanticGraph, port: &SemanticNode) -> Option<String> {
    graph
        .outgoing_typing_or_specializes_targets(port)
        .iter()
        .find(|node| node.element_kind == "port def")
        .map(|node| node.id.qualified_name.clone())
}

fn port_type_display_label(port_type: &str) -> &str {
    port_type.rsplit("::").next().unwrap_or(port_type)
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
        let src_qn =
            port_definition_qualified_name(graph, src).unwrap_or_else(|| src_type.to_string());
        let tgt_qn =
            port_definition_qualified_name(graph, tgt).unwrap_or_else(|| tgt_type.to_string());
        let label = port_type_display_label(src_type);
        if src_qn == tgt_qn {
            return Some(format!(
                "Port definitions '{}' are not feature-compatible (check port features and directions).",
                src_qn
            ));
        }
        return Some(format!(
            "Port types look alike ('{label}') but refer to incompatible definitions: '{src_qn}' vs '{tgt_qn}'. Align imports or use the same port def."
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
            if let Some(mut feature) = port_feature_from_child(child) {
                if conjugated {
                    feature.direction = conjugated_direction(feature.direction);
                }
                features.push(feature);
            }
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

fn port_feature_from_child(child: &SemanticNode) -> Option<PortFeature> {
    let direction = child
        .attributes
        .get("direction")
        .and_then(|v| v.as_str())
        .and_then(parse_feature_direction)?;
    let type_ref = match child.element_kind.as_str() {
        "in out parameter" => child.attributes.get("parameterType")?.as_str()?,
        "item" => child.attributes.get("itemType")?.as_str()?,
        _ => return None,
    };
    Some(PortFeature {
        direction,
        normalized_type: normalize_declared_type_ref(type_ref),
    })
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
    if matches!(type_ref, "String") {
        return true;
    }
    let normalized = normalize_declared_type_ref(type_ref);
    crate::semantic::explicit_views::renderer_view_for_view_type(Some(normalized.as_str()))
        .is_some()
}

pub(super) fn attribute_value_is_string_literal(value: &str) -> bool {
    let trimmed = value.trim();
    trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2
}

pub(super) fn resolves_to_enum_def(
    graph: &crate::SemanticGraph,
    context_node: &crate::SemanticNode,
    type_ref: &str,
) -> bool {
    use crate::resolve_type_reference_targets;
    !resolve_type_reference_targets(graph, context_node, type_ref, &["enum def"]).is_empty()
}

pub(super) fn unresolved_type_diagnostic_range(
    node: &SemanticNode,
    raw_reference: &str,
) -> Option<TextRange> {
    if let Some(range) = reference_token_range(node, raw_reference) {
        return Some(range);
    }
    if !is_unknown_range(node.range) {
        return Some(node.range);
    }
    None
}

pub(super) fn declared_type_ref(node: &SemanticNode) -> Option<&str> {
    [
        "partType",
        "refType",
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
        "viewType",
        "viewpointType",
        "renderingType",
        "subjectType",
        "analysisType",
        "verificationType",
        "metadataType",
        "objectiveType",
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

pub(super) fn condition_expression_is_boolean(node: &SemanticNode, condition: &str) -> bool {
    if let Some(is_boolean) = node
        .attributes
        .get("conditionIsBoolean")
        .and_then(|v| v.as_bool())
    {
        return is_boolean;
    }
    is_booleanish_filter_expression(condition)
}

pub(super) fn is_booleanish_filter_expression(condition: &str) -> bool {
    let trimmed = condition.trim();
    if trimmed.is_empty() {
        return false;
    }
    matches!(
        trimmed.to_ascii_lowercase().as_str(),
        "true" | "false" | "not" | "and" | "or" | "xor"
    ) || trimmed.contains("==")
        || trimmed.contains("!=")
        || trimmed.contains(">")
        || trimmed.contains("<")
        || trimmed.contains("not ")
        || trimmed.contains(" and ")
        || trimmed.contains(" or ")
}

pub(super) fn is_boolean_literal_value(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "true" | "false"
    )
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

/// User-facing text for [`objective_binding_unresolved`](super::engine_impl) diagnostics.
pub(super) fn objective_binding_unresolved_message(
    objective_name: &str,
    binding_kind: &str,
) -> String {
    match binding_kind {
        "verification_subject" => format!(
            "Verification objective '{objective_name}' is not bound to a case subject. \
Add a `subject` clause to the verification def before this objective \
(for example: `subject systemUnderTest : MyPartDef;`)."
        ),
        "analysis_result" => format!(
            "Analysis objective '{objective_name}' is not bound to a case result. \
Add a `return ref` clause to the analysis def before this objective \
(for example: `return ref analysisResult {{ return true; }}`)."
        ),
        other => format!(
            "Objective '{objective_name}' could not be bound (expected binding: {other}). \
Check the case definition body: required clauses must appear before the objective."
        ),
    }
}

#[cfg(test)]
mod connection_duplicate_key_tests {
    use url::Url;

    use crate::semantic::model::NodeId;

    use super::connection_duplicate_key;

    fn node(qn: &str) -> NodeId {
        NodeId::new(&Url::parse("file:///test.sysml").expect("url"), qn)
    }

    #[test]
    fn connection_duplicate_key_distinguishes_usage_scoped_endpoints() {
        let motor = node("Pkg.drone.propulsion.propulsionUnit1.cmd");
        let shared = node("Pkg.PropulsionUnit.cmd");
        let key_one = connection_duplicate_key(
            Some("propulsion.propulsionUnit1.cmd"),
            Some("flightControl.flightController.motorCmd"),
            &motor,
            &shared,
        );
        let key_two = connection_duplicate_key(
            Some("propulsion.propulsionUnit2.cmd"),
            Some("flightControl.flightController.motorCmd"),
            &motor,
            &shared,
        );
        assert_ne!(key_one, key_two);
    }

    #[test]
    fn connection_duplicate_key_flags_repeated_textual_endpoints() {
        let left = node("Pkg.a");
        let right = node("Pkg.b");
        let key = connection_duplicate_key(Some("a.out"), Some("b.in"), &left, &right);
        let repeated = connection_duplicate_key(Some("a.out"), Some("b.in"), &left, &right);
        assert_eq!(key, repeated);
    }
}

#[cfg(test)]
mod objective_binding_message_tests {
    use super::objective_binding_unresolved_message;

    #[test]
    fn verification_subject_message_mentions_subject_clause() {
        let message =
            objective_binding_unresolved_message("coverageObjective", "verification_subject");
        assert!(message.contains("subject"));
        assert!(message.contains("verification"));
        assert!(!message.contains("verification_subject"));
    }

    #[test]
    fn analysis_result_message_mentions_return_ref() {
        let message = objective_binding_unresolved_message("runtimeObjective", "analysis_result");
        assert!(message.contains("return ref"));
        assert!(!message.contains("analysis_result"));
    }
}
