use std::collections::{HashMap, HashSet};

use url::Url;

use crate::semantic::diagnostics::helpers::{
    condition_expression_is_boolean, diag, diagnostic_range, is_synthetic,
    normalize_declared_type_ref,
};
use crate::semantic::diagnostics::kind_rules::{allowed_typing_target_kinds, is_compatible_kind};
use crate::semantic::diagnostics::types::DiagnosticSeverity;
use crate::semantic::model::RelationshipKind;
use crate::semantic::reference_resolution::resolve_expression_endpoint_strict;
use crate::semantic::relationships::{resolve_type_target_in_workspace, TYPING_TARGET_KINDS};
use crate::{ResolveResult, SemanticDiagnostic, SemanticGraph, SemanticNode};

fn is_action_like(kind: &str) -> bool {
    matches!(
        kind,
        "action" | "action def" | "perform" | "merge" | "verdict"
    )
}

fn is_state_like(kind: &str) -> bool {
    matches!(kind, "state" | "state def")
}

fn state_def_contains_node(graph: &SemanticGraph, state_def_qn: &str, node: &SemanticNode) -> bool {
    if node.id.qualified_name == state_def_qn {
        return true;
    }
    let mut current = node.parent_id.as_ref();
    while let Some(parent_id) = current {
        if parent_id.qualified_name == state_def_qn {
            return true;
        }
        current = graph
            .get_node(parent_id)
            .and_then(|parent| parent.parent_id.as_ref());
    }
    false
}

fn state_def_has_initial_transition(
    graph: &SemanticGraph,
    uri: &Url,
    state_def: &SemanticNode,
) -> bool {
    let state_def_qn = state_def.id.qualified_name.as_str();
    if graph
        .edges_for_uri_as_strings(uri)
        .iter()
        .any(|(source, _, kind, _)| {
            *kind == RelationshipKind::InitialState
                && (source == state_def_qn
                    || graph
                        .get_node(&crate::NodeId::new(uri, source.clone()))
                        .and_then(|source_node| state_def_ancestor(graph, source_node))
                        .as_ref()
                        == Some(&state_def.id.qualified_name))
        })
    {
        return true;
    }
    graph.nodes_for_uri(uri).into_iter().any(|node| {
        node.element_kind == "transition"
            && !is_synthetic(node)
            && node
                .attributes
                .get("isInitial")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            && state_def_contains_node(graph, state_def_qn, node)
    })
}

fn state_def_has_final_indicator(graph: &SemanticGraph, state_def: &SemanticNode) -> bool {
    let final_state_count = state_def
        .attributes
        .get("finalStateCount")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    if final_state_count > 0 {
        return true;
    }
    if state_def
        .attributes
        .get("doneTransitionCount")
        .and_then(|v| v.as_u64())
        .unwrap_or(0)
        > 0
    {
        return true;
    }
    graph
        .children_of(state_def)
        .into_iter()
        .any(|child| child.element_kind == "final state")
}

fn state_def_is_cyclic(graph: &SemanticGraph, state_def: &SemanticNode) -> bool {
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    for child in graph.children_of(state_def) {
        if child.element_kind != "transition" || is_synthetic(child) {
            continue;
        }
        let Some(source) = child
            .attributes
            .get("source")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
        else {
            continue;
        };
        let Some(target) = child
            .attributes
            .get("target")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
        else {
            continue;
        };
        adjacency
            .entry(source.to_string())
            .or_default()
            .push(target.to_string());
    }
    let nodes: HashSet<String> = adjacency
        .keys()
        .cloned()
        .chain(adjacency.values().flatten().cloned())
        .collect();
    for start in nodes {
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();
        if state_graph_has_cycle(&start, &adjacency, &mut visiting, &mut visited) {
            return true;
        }
    }
    false
}

fn state_graph_has_cycle(
    node: &str,
    adjacency: &HashMap<String, Vec<String>>,
    visiting: &mut HashSet<String>,
    visited: &mut HashSet<String>,
) -> bool {
    if !visiting.insert(node.to_string()) {
        return true;
    }
    if visited.contains(node) {
        visiting.remove(node);
        return false;
    }
    let mut found = false;
    if let Some(neighbors) = adjacency.get(node) {
        for neighbor in neighbors {
            if state_graph_has_cycle(neighbor, adjacency, visiting, visited) {
                found = true;
                break;
            }
        }
    }
    visiting.remove(node);
    visited.insert(node.to_string());
    found
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
        if source_context.is_some() && target_context.is_some() && source_context != target_context
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

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "transition" || is_synthetic(node) {
            continue;
        }
        let Some(guard) = node
            .attributes
            .get("guardExpression")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
        else {
            continue;
        };
        if condition_expression_is_boolean(node, guard) {
            continue;
        }
        let key = format!("guard|{}", node.id.qualified_name);
        if !seen.insert(key) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::Warning,
            "semantic",
            "transition_guard_non_boolean",
            format!(
                "Transition '{}' guard expression must be Boolean-valued.",
                node.name
            ),
        ));
    }

    let mut unguarded_initial_by_container: HashMap<String, usize> = HashMap::new();
    for (source_qn, _target_qn, kind, _) in graph.edges_for_uri_as_strings(uri) {
        if kind != RelationshipKind::InitialState {
            continue;
        }
        let source_id = crate::NodeId::new(uri, source_qn.clone());
        let container = graph
            .get_node(&source_id)
            .and_then(|node| state_def_ancestor(graph, node))
            .unwrap_or(source_qn);
        *unguarded_initial_by_container.entry(container).or_insert(0) += 1;
    }
    for (container, count) in unguarded_initial_by_container {
        if count <= 1 {
            continue;
        }
        let key = format!("initial_multi|{container}");
        if !seen.insert(key) {
            continue;
        }
        let Some(container_node) = graph.get_node(&crate::NodeId::new(uri, container.clone()))
        else {
            continue;
        };
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, container_node, None),
            DiagnosticSeverity::Warning,
            "semantic",
            "multiple_initial_states",
            format!(
                "State definition '{}' declares {count} unguarded initial transitions; only one is expected (guarded conditionals per SysML 7.18.2 are allowed).",
                container.rsplit("::").next().unwrap_or(container.as_str())
            ),
        ));
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "state def" || is_synthetic(node) {
            continue;
        }
        let has_state_children = graph
            .children_of(node)
            .into_iter()
            .any(|child| child.element_kind == "state");
        if !has_state_children {
            continue;
        }
        if state_def_has_initial_transition(graph, uri, node) {
            continue;
        }
        let key = format!("missing_initial|{}", node.id.qualified_name);
        if !seen.insert(key) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::Information,
            "semantic",
            "missing_initial_state",
            format!(
                "State definition '{}' has state usages but no initial transition.",
                node.name
            ),
        ));
    }

    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "state def" || is_synthetic(node) {
            continue;
        }
        let has_state_children = graph
            .children_of(node)
            .into_iter()
            .any(|child| child.element_kind == "state");
        if !has_state_children {
            continue;
        }
        if state_def_has_final_indicator(graph, node) {
            continue;
        }
        if state_def_is_cyclic(graph, node) {
            continue;
        }
        let key = format!("missing_final|{}", node.id.qualified_name);
        if !seen.insert(key) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, node, None),
            DiagnosticSeverity::Information,
            "semantic",
            "missing_final_state",
            format!(
                "State definition '{}' has state usages but no finality indicator (`final`/`final state` or a transition to `done` per SysML 7.18.3).",
                node.name
            ),
        ));
    }

    for node in graph.nodes_for_uri(uri) {
        if is_synthetic(node) {
            continue;
        }
        let is_action = node.element_kind == "action";
        let is_transition = node.element_kind == "transition";
        if !is_action && !is_transition {
            continue;
        }
        let payload_type = node
            .attributes
            .get("payloadType")
            .or_else(|| node.attributes.get("acceptType"))
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty());
        let Some(payload_type) = payload_type else {
            continue;
        };
        let action_kind = node
            .attributes
            .get("actionKind")
            .and_then(|v| v.as_str())
            .unwrap_or("accept");
        if is_transition && action_kind != "accept" {
            continue;
        }
        let code = if action_kind == "send" {
            "send_payload_incompatible"
        } else {
            "accept_payload_incompatible"
        };
        let allowed = allowed_typing_target_kinds("action");
        let Some(target_id) =
            resolve_type_target_in_workspace(graph, node, payload_type, TYPING_TARGET_KINDS)
        else {
            continue;
        };
        let Some(target_node) = graph.get_node(&target_id) else {
            continue;
        };
        if is_compatible_kind(&target_node.element_kind, allowed) {
            continue;
        }
        let key = format!("{code}|{}", node.id.qualified_name);
        if !seen.insert(key) {
            continue;
        }
        let subject_label = if is_transition {
            "Transition"
        } else if action_kind == "send" {
            "Send"
        } else {
            "Accept"
        };
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, node, Some(target_node)),
            DiagnosticSeverity::Warning,
            "semantic",
            code,
            format!(
                "{} payload type '{}' on '{}' resolves to incompatible kind '{}'.",
                subject_label,
                normalize_declared_type_ref(payload_type),
                node.name,
                target_node.element_kind
            ),
        ));
    }

    let mut final_states_by_container: HashMap<String, usize> = HashMap::new();
    for node in graph.nodes_for_uri(uri) {
        if node.element_kind != "final state" || is_synthetic(node) {
            continue;
        }
        let container = node
            .parent_id
            .as_ref()
            .and_then(|id| graph.get_node(id))
            .and_then(|parent| state_def_ancestor(graph, parent))
            .or_else(|| node.parent_id.as_ref().map(|id| id.qualified_name.clone()));
        let Some(container) = container else {
            continue;
        };
        *final_states_by_container.entry(container).or_insert(0) += 1;
    }
    for (container, count) in final_states_by_container {
        if count <= 1 {
            continue;
        }
        let key = format!("final_multi|{container}");
        if !seen.insert(key) {
            continue;
        }
        let Some(container_node) = graph.get_node(&crate::NodeId::new(uri, container.clone()))
        else {
            continue;
        };
        diagnostics.push(diag(
            uri,
            diagnostic_range(graph, container_node, None),
            DiagnosticSeverity::Warning,
            "semantic",
            "multiple_final_states",
            format!(
                "State definition '{}' declares {count} `final`/`final state` markers; only one explicit marker is expected (transitions to `done` are separate per SysML 7.18.3).",
                container.rsplit("::").next().unwrap_or(container.as_str())
            ),
        ));
    }

    diagnostics
}
