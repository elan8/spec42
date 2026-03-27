//! Semantic validation beyond syntax: port connectivity, type compatibility, etc.
//!
//! These checks use the semantic graph (parts, ports, connections) to report
//! diagnostics such as: unconnected ports, connection to non-port, port type mismatch.

use std::collections::HashSet;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Range, Url};

use crate::ibd;
use crate::semantic_model::{NodeId, SemanticGraph, SemanticNode};

/// Returns LSP diagnostics for semantic rules in the given document.
/// Only runs when the document has been parsed and merged into the graph.
pub fn compute_semantic_diagnostics(graph: &SemanticGraph, uri: &Url) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // 0) Explicit builder diagnostics (e.g. ambiguous endpoint resolution).
    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "diagnostic" {
            continue;
        }
        let code = node
            .attributes
            .get("code")
            .and_then(|v| v.as_str())
            .unwrap_or("semantic_diagnostic");
        let message = node
            .attributes
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("semantic diagnostic")
            .to_string();
        diagnostics.push(diag(
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::WARNING,
            "semantic",
            code,
            message,
        ));
    }

    // 1) Connection endpoints must be ports; port types must be compatible
    let connection_occurrences = graph.connection_edge_occurrences_for_uri(uri);
    for (src_id, tgt_id, connection_range) in connection_occurrences {
        if let (Some(src), Some(tgt)) = (graph.get_node(&src_id), graph.get_node(&tgt_id)) {
            if !is_port_like(&src.element_kind) {
                diagnostics.push(diag(
                    diagnostic_range(graph, src, Some(tgt)),
                    DiagnosticSeverity::WARNING,
                    "semantic",
                    "connection_endpoint_not_port",
                    format!(
                        "Connection source '{}' is not a port (element kind: {}).",
                        src.name, src.element_kind
                    ),
                ));
            }
            if !is_port_like(&tgt.element_kind) {
                diagnostics.push(diag(
                    diagnostic_range(graph, tgt, Some(src)),
                    DiagnosticSeverity::WARNING,
                    "semantic",
                    "connection_endpoint_not_port",
                    format!(
                        "Connection target '{}' is not a port (element kind: {}).",
                        tgt.name, tgt.element_kind
                    ),
                ));
            }
            if is_port_like(&src.element_kind) && is_port_like(&tgt.element_kind) {
                if let Some(msg) = port_type_mismatch(src, tgt) {
                    diagnostics.push(diag(
                        connection_range,
                        DiagnosticSeverity::WARNING,
                        "semantic",
                        "port_type_mismatch",
                        msg,
                    ));
                }
            }
        }
    }

    // 2) Unconnected ports (ports in this URI that are not an endpoint of any connection)
    let connected_port_keys: HashSet<String> = graph
        .connection_edge_node_pairs_for_uri(uri)
        .into_iter()
        .flat_map(|(a, b)| [a, b])
        .filter_map(|id| graph.get_node(&id))
        .filter_map(port_anchor_key)
        .collect();

    for node in graph.nodes_for_uri(uri) {
        if is_port_like(&node.element_kind)
            && node.element_kind == "port"
            && !is_synthetic(node)
            && is_declaration_port(graph, node)
            && port_anchor_key(node)
                .as_ref()
                .is_some_and(|key| !connected_port_keys.contains(key))
        {
            diagnostics.push(diag(
                diagnostic_range(graph, node, None),
                DiagnosticSeverity::INFORMATION,
                "semantic",
                "unconnected_port",
                format!("Port '{}' is not connected to any other port.", node.name),
            ));
        }
    }

    // 3) Duplicate connections (same pair of endpoints connected more than once)
    let mut seen_pairs: HashSet<(NodeId, NodeId)> = HashSet::new();
    for (src_id, tgt_id) in graph.connection_edge_node_pairs_for_uri(uri) {
        let pair = normalize_edge_pair(&src_id, &tgt_id);
        if !seen_pairs.insert(pair) {
            if let Some(tgt) = graph.get_node(&tgt_id) {
                diagnostics.push(diag(
                    diagnostic_range(graph, tgt, None),
                    DiagnosticSeverity::INFORMATION,
                    "semantic",
                    "duplicate_connection",
                    "Duplicate connection between the same two endpoints.".to_string(),
                ));
            }
        }
    }

    // 4) Multiplicity validation (syntax and interval sanity)
    for node in graph.nodes_for_uri(uri) {
        if let Some(multiplicity) = node.attributes.get("multiplicity").and_then(|v| v.as_str()) {
            if let Some(message) = multiplicity_issue_message(multiplicity) {
                diagnostics.push(diag(
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::WARNING,
                    "semantic",
                    "invalid_multiplicity",
                    format!("Invalid multiplicity on '{}': {message}", node.name),
                ));
            }
        }
    }

    // 5) Stronger typing checks: declarations that name a type should resolve via typing/specializes.
    for node in graph.nodes_for_uri(uri) {
        if let Some(type_ref) = declared_type_ref(node) {
            let has_resolved_type = !graph.outgoing_typing_or_specializes_targets(node).is_empty();
            if !has_resolved_type {
                diagnostics.push(diag(
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::WARNING,
                    "semantic",
                    "unresolved_type_reference",
                    format!(
                        "Type reference '{}' for '{}' could not be resolved in the semantic graph.",
                        type_ref, node.name
                    ),
                ));
            }
        }
    }

    // 6) Redefines consistency, when the parser/graph captures a `redefines` attribute.
    for node in graph.nodes_for_uri(uri) {
        let Some(redefines_raw) = node.attributes.get("redefines").and_then(|v| v.as_str()) else {
            continue;
        };
        if redefines_raw.trim().is_empty() {
            diagnostics.push(diag(
                diagnostic_range(graph, node, None),
                DiagnosticSeverity::WARNING,
                "semantic",
                "invalid_redefines_reference",
                format!("Element '{}' has an empty redefines target.", node.name),
            ));
            continue;
        }
        if redefines_raw.trim() == node.name || redefines_raw.trim() == node.id.qualified_name {
            diagnostics.push(diag(
                diagnostic_range(graph, node, None),
                DiagnosticSeverity::WARNING,
                "semantic",
                "invalid_redefines_reference",
                format!("Element '{}' cannot redefine itself.", node.name),
            ));
        }
    }

    diagnostics
}

fn is_port_like(kind: &str) -> bool {
    ibd::is_port_like(kind)
}

fn diag(
    range: Range,
    severity: DiagnosticSeverity,
    source: &str,
    code: &str,
    message: String,
) -> Diagnostic {
    Diagnostic {
        range,
        severity: Some(severity),
        code: Some(tower_lsp::lsp_types::NumberOrString::String(code.to_string())),
        code_description: None,
        source: Some(source.to_string()),
        message,
        related_information: None,
        tags: None,
        data: None,
    }
}

fn is_unknown_range(range: Range) -> bool {
    range.start.line == 0
        && range.start.character == 0
        && range.end.line == 0
        && range.end.character == 0
}

fn is_declaration_port(graph: &SemanticGraph, node: &SemanticNode) -> bool {
    let Some(parent_id) = &node.parent_id else {
        return false;
    };
    let Some(parent) = graph.get_node(parent_id) else {
        return false;
    };
    parent.element_kind == "part def" || parent.element_kind == "part"
}

fn is_synthetic(node: &SemanticNode) -> bool {
    node.attributes
        .get("synthetic")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

fn parse_origin_range(node: &SemanticNode) -> Option<Range> {
    let origin = node.attributes.get("originRange")?;
    let start = origin.get("start")?;
    let end = origin.get("end")?;
    Some(Range {
        start: tower_lsp::lsp_types::Position {
            line: start.get("line")?.as_u64()? as u32,
            character: start.get("character")?.as_u64()? as u32,
        },
        end: tower_lsp::lsp_types::Position {
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

fn port_anchor_key(node: &SemanticNode) -> Option<String> {
    let r = preferred_port_anchor_range(node)?;
    Some(format!(
        "{}:{}:{}:{}:{}",
        r.start.line, r.start.character, r.end.line, r.end.character, node.name
    ))
}

fn diagnostic_range(graph: &SemanticGraph, node: &SemanticNode, peer: Option<&SemanticNode>) -> Range {
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

/// Normalize (a,b) and (b,a) to the same key for duplicate detection.
fn normalize_edge_pair(a: &NodeId, b: &NodeId) -> (NodeId, NodeId) {
    if a.qualified_name <= b.qualified_name {
        (a.clone(), b.clone())
    } else {
        (b.clone(), a.clone())
    }
}

/// Returns an error message if the two ports are not compatible for connection.
/// In SysML, compatible means: same base port type, with one conjugated (~T) and one not (T).
fn port_type_mismatch(src: &SemanticNode, tgt: &SemanticNode) -> Option<String> {
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

/// Returns (base_type_name, is_conjugated). E.g. "~PowerPort" -> ("PowerPort", true).
fn parse_port_type(s: &str) -> (String, bool) {
    let t = s.trim();
    let (conj, base) = if let Some(stripped) = t.strip_prefix('~') {
        (true, stripped.trim())
    } else {
        (false, t)
    };
    (base.to_string(), conj)
}

fn declared_type_ref(node: &SemanticNode) -> Option<&str> {
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

fn multiplicity_issue_message(multiplicity: &str) -> Option<String> {
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

fn validate_single_multiplicity_value(raw: &str) -> Option<String> {
    if raw == "*" {
        return None;
    }
    parse_non_negative_bound(raw).err()
}

fn parse_non_negative_bound(raw: &str) -> Result<i64, String> {
    match raw.parse::<i64>() {
        Ok(value) if value >= 0 => Ok(value),
        Ok(value) => Err(format!("bound {value} must be non-negative")),
        Err(_) => Err(format!("bound '{raw}' is not an integer or '*'")),
    }
}

/// Default semantic checks (port connectivity, type compatibility, unconnected ports, duplicate connections).
/// Implements [crate::config::SemanticCheckProvider] for use in [crate::config::Spec42Config].
#[derive(Debug, Default)]
pub struct DefaultSemanticChecks;

impl crate::config::SemanticCheckProvider for DefaultSemanticChecks {
    fn compute_diagnostics(&self, graph: &SemanticGraph, uri: &Url) -> Vec<Diagnostic> {
        compute_semantic_diagnostics(graph, uri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic_model::build_graph_from_doc;
    use tower_lsp::lsp_types::Position;

    #[test]
    fn parse_port_type_conjugated() {
        let (base, conj) = parse_port_type("~PowerPort");
        assert_eq!(base, "PowerPort");
        assert!(conj);
    }

    #[test]
    fn parse_port_type_plain() {
        let (base, conj) = parse_port_type("PowerPort");
        assert_eq!(base, "PowerPort");
        assert!(!conj);
    }

    #[test]
    fn port_type_mismatch_different_base() {
        let src = node_with_port_type("~PowerPort");
        let tgt = node_with_port_type("~TelemetryPort");
        let msg = port_type_mismatch(&src, &tgt);
        assert!(msg.is_some());
        assert!(msg.unwrap().contains("do not match"));
    }

    #[test]
    fn port_type_compatible() {
        let src = node_with_port_type("~PowerPort");
        let tgt = node_with_port_type("PowerPort");
        assert!(port_type_mismatch(&src, &tgt).is_none());
    }

    #[test]
    fn port_type_both_conjugated() {
        let src = node_with_port_type("~PowerPort");
        let tgt = node_with_port_type("~PowerPort");
        let msg = port_type_mismatch(&src, &tgt);
        assert!(msg.is_some());
        assert!(msg.unwrap().contains("same conjugation"));
    }

    fn node_with_port_type(port_type: &str) -> SemanticNode {
        use std::collections::HashMap;
        let mut attrs = HashMap::new();
        attrs.insert("portType".to_string(), serde_json::json!(port_type));
        SemanticNode {
            id: NodeId {
                uri: Url::parse("file:///test.sysml").unwrap(),
                qualified_name: "Test::p".to_string(),
            },
            element_kind: "port".to_string(),
            name: "p".to_string(),
            range: Range::new(Position::new(0, 0), Position::new(0, 0)),
            attributes: attrs,
            parent_id: None,
        }
    }

    #[test]
    fn declared_type_ref_ignores_empty_values() {
        use std::collections::HashMap;
        let mut attrs = HashMap::new();
        attrs.insert("partType".to_string(), serde_json::json!(""));
        let node = SemanticNode {
            id: NodeId {
                uri: Url::parse("file:///test.sysml").unwrap(),
                qualified_name: "Test::display".to_string(),
            },
            element_kind: "part".to_string(),
            name: "display".to_string(),
            range: Range::new(Position::new(0, 0), Position::new(0, 0)),
            attributes: attrs,
            parent_id: None,
        };
        assert!(declared_type_ref(&node).is_none());
    }

    #[test]
    fn port_type_mismatch_is_anchored_to_connection_statement() {
        let input = r#"
            package P {
                part def Left {
                    port p : ~PowerPort;
                }
                part def Right {
                    port p : ~PowerPort;
                }
                part def Top {
                    part l : Left;
                    part r : Right;
                    connect l.p to r.p;
                }
            }
        "#;
        let root = sysml_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///test.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let diags = compute_semantic_diagnostics(&graph, &uri);
        let mismatch = diags
            .iter()
            .find(|d| {
                d.code.as_ref()
                    == Some(&tower_lsp::lsp_types::NumberOrString::String(
                        "port_type_mismatch".to_string(),
                    ))
            })
            .expect("expected mismatch diagnostic");

        // "connect l.p to r.p;" line (0-based, accounting for leading newline/indentation in input string)
        assert_eq!(mismatch.range.start.line, 11);
        assert_eq!(mismatch.range.end.line, 11);
    }

    #[test]
    fn multiplicity_validator_rejects_negative_and_reversed_ranges() {
        let negative = multiplicity_issue_message("[-1]");
        assert!(negative.is_some());
        assert!(negative.expect("negative issue").contains("non-negative"));

        let reversed = multiplicity_issue_message("[5..2]");
        assert!(reversed.is_some());
        assert!(reversed.expect("reversed issue").contains("greater than"));
    }

    #[test]
    fn multiplicity_validator_accepts_common_valid_forms() {
        assert!(multiplicity_issue_message("[0..1]").is_none());
        assert!(multiplicity_issue_message("[1..*]").is_none());
        assert!(multiplicity_issue_message("[3]").is_none());
    }
}
