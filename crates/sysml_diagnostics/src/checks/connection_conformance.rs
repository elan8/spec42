use std::collections::HashSet;

use url::Url;

use crate::helpers::{
    diag, diagnostic_range, is_part_like, is_port_like, parse_port_type,
    port_compatibility_mismatch,
};
use crate::types::DiagnosticSeverity;
use sysml_model::semantic::model::RelationshipKind;
use sysml_model::semantic::reference_resolution::resolve_expression_endpoint_strict;
use crate::SemanticDiagnostic;
use sysml_model::{ResolveResult, SemanticGraph};

fn first_unresolved_connection_segment(
    graph: &SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    expression: &str,
) -> Option<String> {
    let normalized = expression.replace('.', "::");
    let segments: Vec<&str> = normalized
        .split("::")
        .filter(|segment| !segment.is_empty())
        .collect();
    if segments.is_empty() {
        return None;
    }
    if segments.len() == 1 {
        return match resolve_expression_endpoint_strict(graph, uri, container_prefix, expression) {
            ResolveResult::Unresolved => Some(segments[0].to_string()),
            ResolveResult::Ambiguous => None,
            ResolveResult::Resolved(_) => None,
        };
    }
    match resolve_expression_endpoint_strict(graph, uri, container_prefix, segments[0]) {
        ResolveResult::Unresolved => Some(segments[0].to_string()),
        ResolveResult::Ambiguous => None,
        ResolveResult::Resolved(mut current_id) => {
            for segment in segments.iter().skip(1) {
                let Some(owner) = graph.get_node(&current_id) else {
                    return Some((*segment).to_string());
                };
                match sysml_model::resolve_member_via_type(graph, owner, segment) {
                    ResolveResult::Resolved(next) => current_id = next,
                    ResolveResult::Unresolved => return Some((*segment).to_string()),
                    ResolveResult::Ambiguous => return None,
                }
            }
            None
        }
    }
}

fn port_mismatch_code(message: &str) -> Option<&'static str> {
    if message.contains("same conjugation") || message.contains("conjugated") {
        Some("conjugated_port_inconsistent")
    } else if message.contains("feature-compatible") || message.contains("direction") {
        Some("flow_direction_incompatible")
    } else if message.contains("incompatible definitions") {
        Some("flow_item_type_incompatible")
    } else {
        None
    }
}

pub(crate) fn collect_connection_conformance_diagnostics(
    graph: &SemanticGraph,
    uri: &Url,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = Vec::new();
    let mut seen = HashSet::new();

    for pending in &graph.pending_expression_relationships {
        if pending.uri != *uri || pending.kind != RelationshipKind::Connection {
            continue;
        }
        for expression in [&pending.source_expression, &pending.target_expression] {
            if let Some(segment) = first_unresolved_connection_segment(
                graph,
                uri,
                pending.container_prefix.as_deref(),
                expression,
            ) {
                let key = format!("segment|{}|{}", expression, segment);
                if seen.insert(key) {
                    diagnostics.push(diag(
                        uri,
                        pending.source_range,
                        DiagnosticSeverity::Warning,
                        "semantic",
                        "unresolved_connection_segment",
                        format!(
                            "Connection endpoint '{}' could not resolve segment '{}'.",
                            expression, segment
                        ),
                    ));
                }
            }
        }
    }

    for (left_id, right_id) in graph.connection_edge_node_pairs_for_uri(uri) {
        let (Some(left), Some(right)) = (graph.get_node(&left_id), graph.get_node(&right_id))
        else {
            continue;
        };
        if is_port_like(&left.element_kind) && is_port_like(&right.element_kind) {
            if let Some(message) = port_compatibility_mismatch(graph, left, right) {
                if let Some(code) = port_mismatch_code(&message) {
                    let key = format!(
                        "{}|{}|{}",
                        code, left.id.qualified_name, right.id.qualified_name
                    );
                    if seen.insert(key) {
                        diagnostics.push(diag(
                            uri,
                            diagnostic_range(graph, left, Some(right)),
                            DiagnosticSeverity::Warning,
                            "semantic",
                            code,
                            message,
                        ));
                    }
                }
            }
            continue;
        }
        if !(is_port_like(&left.element_kind)
            || is_port_like(&right.element_kind)
            || is_part_like(&left.element_kind) && is_part_like(&right.element_kind))
        {
            let key = format!(
                "context|{}|{}",
                left.id.qualified_name, right.id.qualified_name
            );
            if seen.insert(key) {
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, left, Some(right)),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "connection_context_invalid",
                    format!(
                        "Connection between '{}' ({}) and '{}' ({}) is not in a connectable structural context.",
                        left.name,
                        left.element_kind,
                        right.name,
                        right.element_kind
                    ),
                ));
            }
        }
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != sysml_model::ElementKind::InterfaceEnd {
            continue;
        }
        let Some(port_type) = node
            .attributes
            .get("portType")
            .and_then(|value| value.as_str())
        else {
            let key = format!("iface|{}", node.id.qualified_name);
            if seen.insert(key) {
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "interface_end_invalid",
                    format!(
                        "Interface end '{}' does not declare a port type.",
                        node.name
                    ),
                ));
            }
            continue;
        };
        let (base, _) = parse_port_type(port_type);
        if base.is_empty() {
            let key = format!("iface-empty|{}", node.id.qualified_name);
            if seen.insert(key) {
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "interface_end_invalid",
                    format!("Interface end '{}' has an empty port type.", node.name),
                ));
            }
        }
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != sysml_model::ElementKind::Binding {
            continue;
        }
        let endpoints: Vec<String> = graph
            .edges_for_uri_as_strings(uri)
            .into_iter()
            .filter(|(source, _, kind, _)| {
                *kind == RelationshipKind::Connection && source == &node.id.qualified_name
            })
            .map(|(_, target, _, _)| target)
            .collect();
        if endpoints.len() == 2 {
            let left_id = sysml_model::NodeId::new(uri, &endpoints[0]);
            let right_id = sysml_model::NodeId::new(uri, &endpoints[1]);
            if let (Some(left), Some(right)) = (graph.get_node(&left_id), graph.get_node(&right_id))
            {
                if left.element_kind == sysml_model::ElementKind::Attribute
                    && right.element_kind == sysml_model::ElementKind::Attribute
                {
                    let left_type = left
                        .attributes
                        .get("valueType")
                        .or_else(|| left.attributes.get("typeRef"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let right_type = right
                        .attributes
                        .get("valueType")
                        .or_else(|| right.attributes.get("typeRef"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if !left_type.is_empty() && !right_type.is_empty() && left_type != right_type {
                        let key = format!(
                            "bind|{}|{}",
                            left.id.qualified_name, right.id.qualified_name
                        );
                        if seen.insert(key) {
                            diagnostics.push(diag(
                                uri,
                                diagnostic_range(graph, left, Some(right)),
                                DiagnosticSeverity::Warning,
                                "semantic",
                                "binding_connector_incompatible",
                                format!(
                                    "Binding connector binds '{}' ({left_type}) to '{}' ({right_type}) with incompatible value types.",
                                    left.name, right.name
                                ),
                            ));
                        }
                    }
                }
            }
        }
    }

    diagnostics
}
