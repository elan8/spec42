//! Semantic validation beyond syntax: port connectivity, type compatibility, etc.
//!
//! These checks use the semantic graph (parts, ports, connections) to report
//! diagnostics such as: unconnected ports, connection to non-port, port type mismatch.

use std::collections::HashSet;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Url};

use crate::analysis::helpers::*;
use crate::semantic_model::{NodeId, SemanticGraph};

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
    let mut unresolved_seen: HashSet<String> = HashSet::new();
    for node in graph.nodes_for_uri(uri) {
        if is_synthetic(node) {
            continue;
        }
        let Some(type_ref) = declared_type_ref(node) else {
            continue;
        };
        let normalized_type_ref = normalize_declared_type_ref(type_ref);
        if is_builtin_type_ref(&normalized_type_ref) {
            continue;
        }
        let has_resolved_type = !graph
            .outgoing_typing_or_specializes_targets(node)
            .is_empty();
        if has_resolved_type {
            continue;
        }
        let Some(range) = unresolved_type_diagnostic_range(node) else {
            continue;
        };
        let key = format!(
            "{}|{}|{}:{}:{}:{}:{}",
            node.id.qualified_name,
            normalized_type_ref,
            range.start.line,
            range.start.character,
            range.end.line,
            range.end.character,
            node.name
        );
        if !unresolved_seen.insert(key) {
            continue;
        }
        diagnostics.push(diag(
            range,
            DiagnosticSeverity::WARNING,
            "semantic",
            "unresolved_type_reference",
            format!(
                "Type reference '{}' for '{}' could not be resolved in the semantic graph.",
                type_ref, node.name
            ),
        ));
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

/// Default semantic checks (port connectivity, type compatibility, unconnected ports, duplicate connections).
/// Implements [crate::host::config::SemanticCheckProvider] for use in [crate::host::config::Spec42Config].
#[derive(Debug, Default)]
pub struct DefaultSemanticChecks;

impl crate::host::config::SemanticCheckProvider for DefaultSemanticChecks {
    fn compute_diagnostics(&self, graph: &SemanticGraph, uri: &Url) -> Vec<Diagnostic> {
        compute_semantic_diagnostics(graph, uri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic_model::{build_graph_from_doc, SemanticNode};
    use tower_lsp::lsp_types::{Position, Range};

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
