use std::collections::HashSet;

use url::Url;

use crate::semantic::diagnostics::helpers::{diag, diagnostic_range, is_synthetic};
use crate::semantic::diagnostics::types::DiagnosticSeverity;
use crate::semantic::model::RelationshipKind;
use crate::semantic::reference_resolution::resolve_expression_endpoint_strict;
use crate::{ResolveResult, SemanticDiagnostic, SemanticGraph, SemanticNode};

fn is_action_like(kind: &str) -> bool {
    matches!(kind, "action" | "action def" | "perform" | "merge")
}

fn is_state_like(kind: &str) -> bool {
    matches!(kind, "state" | "state def")
}

fn state_def_ancestor(graph: &SemanticGraph, node: &SemanticNode) -> Option<String> {
    let mut current = node.parent_id.as_ref()?;
    loop {
        let Some(parent) = graph.get_node(current) else {
            return None;
        };
        if parent.element_kind == "state def" {
            return Some(parent.id.qualified_name.clone());
        }
        current = parent.parent_id.as_ref()?;
    }
}

fn resolve_qualified_endpoint(
    graph: &SemanticGraph,
    uri: &Url,
    reference: &str,
) -> Option<SemanticNode> {
    let normalized = reference.replace('.', "::");
    match resolve_expression_endpoint_strict(graph, uri, None, &normalized) {
        ResolveResult::Resolved(id) => graph.get_node(&id).cloned(),
        ResolveResult::Ambiguous | ResolveResult::Unresolved => {
            let container_prefix = normalized
                .rsplit_once("::")
                .map(|(prefix, _)| prefix)
                .filter(|prefix| !prefix.is_empty());
            let local_name = normalized.rsplit("::").next().unwrap_or(&normalized);
            match resolve_expression_endpoint_strict(graph, uri, container_prefix, local_name) {
                ResolveResult::Resolved(id) => graph.get_node(&id).cloned(),
                ResolveResult::Ambiguous | ResolveResult::Unresolved => None,
            }
        }
    }
}

pub(in crate::semantic::diagnostics) fn collect_behavior_conformance_diagnostics(
    graph: &SemanticGraph,
    uri: &Url,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = Vec::new();
    let mut seen = HashSet::new();

    for (source_qn, target_qn, kind, _) in graph.edges_for_uri_as_strings(uri) {
        if kind != RelationshipKind::Perform {
            continue;
        }
        let target_id = crate::NodeId::new(uri, target_qn.clone());
        let Some(target_node) = graph.get_node(&target_id) else {
            continue;
        };
        if is_action_like(&target_node.element_kind) {
            continue;
        }
        let key = format!("perform|{source_qn}|{target_qn}");
        if !seen.insert(key) {
            continue;
        }
        let source_id = crate::NodeId::new(uri, source_qn.clone());
        let source_node = graph.get_node(&source_id);
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, target_node, source_node),
            DiagnosticSeverity::Warning,
            "semantic",
            "perform_target_invalid_kind",
            format!(
                "Perform target '{}' must resolve to an action definition or usage (got '{}').",
                target_node.name, target_node.element_kind
            ),
        ));
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "transition" || is_synthetic(node) {
            continue;
        }
        let Some(source_ref) = node.attributes.get("source").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some(target_ref) = node.attributes.get("target").and_then(|v| v.as_str()) else {
            continue;
        };
        let source_node = resolve_qualified_endpoint(graph, uri, source_ref);
        let target_node = resolve_qualified_endpoint(graph, uri, target_ref);
        let (Some(source_node), Some(target_node)) = (source_node, target_node) else {
            continue;
        };
        if !is_state_like(&source_node.element_kind) || !is_state_like(&target_node.element_kind) {
            let key = format!("transition|{}|state_kind", node.id.qualified_name);
            if seen.insert(key) {
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "transition_endpoint_invalid_state",
                    format!(
                        "Transition '{}' endpoints must resolve to state usages (got '{}' -> '{}').",
                        node.name, source_node.element_kind, target_node.element_kind
                    ),
                ));
            }
            continue;
        }
        let source_context = state_def_ancestor(graph, &source_node);
        let target_context = state_def_ancestor(graph, &target_node);
        if source_context.is_some()
            && target_context.is_some()
            && source_context != target_context
        {
            let key = format!("transition|{}|context", node.id.qualified_name);
            if seen.insert(key) {
                diagnostics.push(diag(
                    uri,
                    diagnostic_range(graph, node, None),
                    DiagnosticSeverity::Warning,
                    "semantic",
                    "transition_endpoint_invalid_context",
                    format!(
                        "Transition '{}' source and target must belong to the same state definition context.",
                        node.name
                    ),
                ));
            }
        }
    }

    for (source_qn, target_qn, kind, _) in graph.edges_for_uri_as_strings(uri) {
        if kind != RelationshipKind::InitialState {
            continue;
        }
        let target_id = crate::NodeId::new(uri, target_qn.clone());
        let Some(target_node) = graph.get_node(&target_id) else {
            continue;
        };
        if is_state_like(&target_node.element_kind) {
            continue;
        }
        let key = format!("initial|{source_qn}|{target_qn}");
        if !seen.insert(key) {
            continue;
        }
        let source_id = crate::NodeId::new(uri, source_qn.clone());
        let source_node = graph.get_node(&source_id);
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, target_node, source_node),
            DiagnosticSeverity::Warning,
            "semantic",
            "initial_state_invalid_target",
            format!(
                "Initial transition target '{}' must resolve to a state usage (got '{}').",
                target_node.name, target_node.element_kind
            ),
        ));
    }

    for (source_qn, target_qn, kind, _) in graph.edges_for_uri_as_strings(uri) {
        if kind != RelationshipKind::Flow {
            continue;
        }
        let source_id = crate::NodeId::new(uri, source_qn.clone());
        let target_id = crate::NodeId::new(uri, target_qn.clone());
        let (Some(source_node), Some(target_node)) =
            (graph.get_node(&source_id), graph.get_node(&target_id))
        else {
            continue;
        };
        let in_action_context = matches!(
            source_node.element_kind.as_str(),
            "action def" | "action" | "perform" | "merge"
        ) || source_node
            .parent_id
            .as_ref()
            .and_then(|id| graph.get_node(id))
            .is_some_and(|parent| {
                matches!(
                    parent.element_kind.as_str(),
                    "action def" | "action" | "verification def" | "verification"
                )
            });
        if !in_action_context {
            continue;
        }
        if is_action_like(&source_node.element_kind) && is_action_like(&target_node.element_kind) {
            continue;
        }
        let key = format!("flow|{source_qn}|{target_qn}");
        if !seen.insert(key) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, source_node, Some(target_node)),
            DiagnosticSeverity::Warning,
            "semantic",
            "succession_endpoint_invalid",
            format!(
                "Succession flow '{}' -> '{}' must connect action-like endpoints (got '{}' -> '{}').",
                source_node.name,
                target_node.name,
                source_node.element_kind,
                target_node.element_kind
            ),
        ));
    }

    diagnostics
}
