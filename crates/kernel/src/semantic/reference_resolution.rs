use std::collections::HashSet;

use tower_lsp::lsp_types::Url;

use crate::semantic::graph::SemanticGraph;
use crate::semantic::import_resolution::resolve_imported_node_ids_for_simple_name;
use crate::semantic::model::{NodeId, SemanticNode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveResult<T> {
    Resolved(T),
    Ambiguous,
    Unresolved,
}

fn is_namespace_kind(kind: &str) -> bool {
    matches!(
        kind,
        "package"
            | "requirement def"
            | "requirement"
            | "use case def"
            | "use case"
            | "analysis def"
            | "analysis"
            | "verification def"
            | "verification"
            | "concern def"
            | "concern"
    )
}

fn resolve_context_node_for_prefix<'a>(
    g: &'a SemanticGraph,
    uri: &Url,
    prefix: &str,
) -> Option<&'a SemanticNode> {
    let owner_id = NodeId::new(uri, prefix);
    if let Some(owner) = g.get_node(&owner_id) {
        return Some(owner);
    }

    let suffix = format!("::{}", prefix);
    g.nodes_by_uri
        .get(uri)
        .into_iter()
        .flatten()
        .filter_map(|node_id| g.get_node(node_id))
        .filter(|node| {
            is_namespace_kind(&node.element_kind)
                && (node.id.qualified_name == prefix || node.id.qualified_name.ends_with(&suffix))
        })
        .min_by_key(|node| node.id.qualified_name.len())
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
        if let Some(node) = g.get_node(&node_id) {
            if node.element_kind != "import" {
                return ResolveResult::Resolved(node_id);
            }
        }
    }
    if let Some(prefix) = container_prefix {
        if !expression.contains("::") && !expression.contains('.') {
            if let Some(owner) = resolve_context_node_for_prefix(g, uri, prefix) {
                if let ResolveResult::Resolved(member_id) =
                    resolve_member_via_type(g, owner, expression)
                {
                    return ResolveResult::Resolved(member_id);
                }
                let imported_matches =
                    resolve_imported_node_ids_for_simple_name(g, owner, expression);
                if imported_matches.len() == 1 {
                    return ResolveResult::Resolved(imported_matches[0].clone());
                }
                if imported_matches.len() > 1 {
                    return ResolveResult::Ambiguous;
                }
            }
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
            (expression_forms.contains(&node_id.qualified_name)
                || suffixes
                    .iter()
                    .any(|suffix| node_id.qualified_name.ends_with(suffix)))
                && g.get_node(node_id).is_some_and(|node| node.element_kind != "import")
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
