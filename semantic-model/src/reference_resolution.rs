use std::collections::HashSet;

use tower_lsp::lsp_types::Url;

use crate::graph::SemanticGraph;
use crate::model::{NodeId, SemanticNode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveResult<T> {
    Resolved(T),
    Ambiguous,
    Unresolved,
}

/// Resolve an endpoint expression (e.g. `a.b`, `A::B`) to a node id.
pub fn resolve_expression_endpoint_strict(
    g: &SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    expression: &str,
) -> ResolveResult<NodeId> {
    let expr_normalized = expression.replace('.', "::");
    let mut expression_forms = Vec::new();
    expression_forms.push(expression.to_string());
    if expr_normalized != expression {
        expression_forms.push(expr_normalized.clone());
    }

    let mut candidates = Vec::new();
    if let Some(prefix) = container_prefix {
        for form in &expression_forms {
            candidates.push(format!("{}::{}", prefix, form));
        }
    }
    candidates.extend(expression_forms.clone());

    for candidate in &candidates {
        let node_id = NodeId::new(uri, candidate);
        if g.node_index_by_id.contains_key(&node_id) {
            return ResolveResult::Resolved(node_id);
        }
    }

    let suffixes: Vec<String> = expression_forms
        .iter()
        .map(|form| format!("::{}", form))
        .collect();
    let mut matches: Vec<&NodeId> = g
        .nodes_by_uri
        .get(uri)
        .into_iter()
        .flatten()
        .filter(|node_id| {
            expression_forms.contains(&node_id.qualified_name)
                || suffixes
                    .iter()
                    .any(|suffix| node_id.qualified_name.ends_with(suffix))
        })
        .collect();
    matches.sort_by_key(|node_id| node_id.qualified_name.len());
    matches.dedup_by(|a, b| a.qualified_name == b.qualified_name);
    if matches.len() == 1 {
        ResolveResult::Resolved(matches[0].clone())
    } else if matches.len() > 1 {
        ResolveResult::Ambiguous
    } else {
        ResolveResult::Unresolved
    }
}

/// Resolve `member` through typing/specialization starting from `owner`.
pub fn resolve_member_via_type(
    g: &SemanticGraph,
    owner: &SemanticNode,
    member: &str,
) -> ResolveResult<NodeId> {
    let mut matches: Vec<NodeId> = Vec::new();
    let mut visited: HashSet<NodeId> = HashSet::new();
    let mut stack: Vec<NodeId> = g
        .outgoing_typing_or_specializes_targets(owner)
        .into_iter()
        .map(|n| n.id.clone())
        .collect();

    while let Some(type_id) = stack.pop() {
        if !visited.insert(type_id.clone()) {
            continue;
        }
        for child in g.child_named(&type_id, member) {
            matches.push(child.id.clone());
        }
        if let Some(type_node) = g.get_node(&type_id) {
            for base in g.outgoing_typing_or_specializes_targets(type_node) {
                stack.push(base.id.clone());
            }
        }
    }

    matches.sort_by_key(|id| id.qualified_name.len());
    matches.dedup_by(|a, b| a == b);
    if matches.len() == 1 {
        ResolveResult::Resolved(matches.remove(0))
    } else if matches.len() > 1 {
        ResolveResult::Ambiguous
    } else {
        ResolveResult::Unresolved
    }
}
