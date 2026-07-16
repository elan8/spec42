//! Unified name resolution helpers and role-based entry points.

use url::Url;

use crate::semantic::graph::SemanticGraph;
use crate::semantic::import_resolution::resolve_type_reference_targets;
use crate::semantic::kinds::{allowed_for_role, ResolutionRole};
use crate::semantic::model::{ElementKind, NodeId};
use crate::semantic::reference_resolution::{
    resolve_expression_endpoint_strict, resolve_member_via_type, ResolveResult,
};
use crate::SemanticNode;

pub mod naming;

pub use naming::{
    normalize_declared_type_ref, normalize_for_lookup, type_ref_candidates,
    type_ref_candidates_with_kind,
};

/// Outcome of a unified name resolution request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveNameOutcome {
    Resolved(Vec<NodeId>),
    Ambiguous,
    Unresolved,
}

fn ids_from_strict(endpoint: ResolveResult<NodeId>) -> ResolveNameOutcome {
    match endpoint {
        ResolveResult::Resolved(id) => ResolveNameOutcome::Resolved(vec![id]),
        ResolveResult::Ambiguous => ResolveNameOutcome::Ambiguous,
        ResolveResult::Unresolved => ResolveNameOutcome::Unresolved,
    }
}

/// Resolve a name for typing, specializes, or other graph linking roles.
pub fn resolve_name(
    graph: &SemanticGraph,
    context: &SemanticNode,
    name: &str,
    role: ResolutionRole,
) -> ResolveNameOutcome {
    let allowed = allowed_for_role(role);
    let targets = resolve_type_reference_targets(graph, context, name, allowed);
    if targets.is_empty() {
        ResolveNameOutcome::Unresolved
    } else if targets.len() == 1 {
        ResolveNameOutcome::Resolved(targets)
    } else {
        ResolveNameOutcome::Ambiguous
    }
}

/// Resolve an expression endpoint using strict graph rules.
pub fn resolve_expression_endpoint(
    graph: &SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    expression: &str,
) -> ResolveNameOutcome {
    ids_from_strict(resolve_expression_endpoint_strict(
        graph,
        uri,
        container_prefix,
        expression,
    ))
}

/// Resolve an expression endpoint to a qualified name (graph build + connection wiring).
pub fn resolve_expression_endpoint_qualified(
    graph: &SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    expression: &str,
) -> Option<String> {
    if let ResolveResult::Resolved(resolved) =
        resolve_expression_endpoint_strict(graph, uri, container_prefix, expression)
    {
        return Some(resolved.qualified_name);
    }

    let normalized = normalize_for_lookup(expression);
    if expression.contains("::") || expression.contains('.') {
        if let Some(node_ids) = graph.node_ids_for_qualified_name(&normalized) {
            if let Some(best_match) = node_ids
                .iter()
                .filter_map(|node_id| {
                    graph.get_node(node_id).and_then(|node| {
                        (node.element_kind != ElementKind::Import)
                            .then_some(node_id.qualified_name.clone())
                    })
                })
                .min_by_key(|qualified_name| qualified_name.len())
            {
                return Some(best_match);
            }
        }
    }

    let segments: Vec<&str> = normalized
        .split("::")
        .filter(|segment| !segment.is_empty())
        .collect();
    if segments.len() > 1 {
        if let ResolveResult::Resolved(mut current_id) =
            resolve_expression_endpoint_strict(graph, uri, container_prefix, segments[0])
        {
            let mut resolved_all = true;
            for member in segments.iter().skip(1) {
                let Some(owner) = graph.get_node(&current_id) else {
                    resolved_all = false;
                    break;
                };
                match resolve_member_via_type(graph, owner, member) {
                    ResolveResult::Resolved(next_id) => current_id = next_id,
                    ResolveResult::Ambiguous | ResolveResult::Unresolved => {
                        resolved_all = false;
                        break;
                    }
                }
            }
            if resolved_all {
                return Some(current_id.qualified_name);
            }
        }
    }

    let mut candidates = Vec::new();
    if let Some(prefix) = container_prefix {
        candidates.push(format!("{}::{}", prefix, expression));
    }
    candidates.push(expression.to_string());
    for candidate in &candidates {
        let node_id = NodeId::new(uri, candidate);
        if graph.node_index_by_id.contains_key(&node_id) {
            return Some(candidate.clone());
        }
    }

    let suffix = format!("::{}", expression);
    graph
        .nodes_by_uri
        .get(uri)
        .into_iter()
        .flatten()
        .filter(|node_id| {
            node_id.qualified_name == expression || node_id.qualified_name.ends_with(&suffix)
        })
        .min_by_key(|node_id| node_id.qualified_name.len())
        .map(|node_id| node_id.qualified_name.clone())
}

/// Resolve an inherited member name from a feature owner.
pub fn resolve_member(
    graph: &SemanticGraph,
    owner: &SemanticNode,
    member: &str,
) -> ResolveNameOutcome {
    ids_from_strict(resolve_member_via_type(graph, owner, member))
}
