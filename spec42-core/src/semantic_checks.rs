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

    // 1) Connection endpoints must be ports; port types must be compatible
    for (src_id, tgt_id) in graph.connection_edge_node_pairs_for_uri(uri) {
        if let (Some(src), Some(tgt)) = (graph.get_node(&src_id), graph.get_node(&tgt_id)) {
            if !is_port_like(&src.element_kind) {
                diagnostics.push(diag(
                    src.range,
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
                    tgt.range,
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
                        src.range,
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
    let connected_port_ids: HashSet<NodeId> = graph
        .connection_edge_node_pairs_for_uri(uri)
        .into_iter()
        .flat_map(|(a, b)| [a, b])
        .collect();

    for node in graph.nodes_for_uri(uri) {
        if is_port_like(&node.element_kind) && !connected_port_ids.contains(&node.id) {
            diagnostics.push(diag(
                node.range,
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
                    tgt.range,
                    DiagnosticSeverity::INFORMATION,
                    "semantic",
                    "duplicate_connection",
                    "Duplicate connection between the same two endpoints.".to_string(),
                ));
            }
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
    let (conj, base) = if t.starts_with('~') {
        (true, t[1..].trim())
    } else {
        (false, t)
    };
    (base.to_string(), conj)
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
}
