use std::collections::HashMap;

use tower_lsp::lsp_types::{Range, Url};

use crate::ast_util::span_to_range;
use crate::graph::SemanticGraph;
use crate::model::{NodeId, RelationshipKind};
use crate::reference_resolution::{resolve_expression_endpoint_strict, ResolveResult};
use crate::relationships::{add_edge_if_both_exist, add_typing_edge_if_exists};

use super::{add_node_and_recurse, qualified_name_for_node};

pub(super) fn add_perform_usage_node(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    action_name: &str,
    action_type: Option<&str>,
    range: Range,
) -> String {
    let qualified = qualified_name_for_node(g, uri, container_prefix, action_name, "action");
    if !g
        .node_index_by_id
        .contains_key(&NodeId::new(uri, &qualified))
    {
        let mut attrs = HashMap::new();
        if let Some(action_type) = action_type {
            attrs.insert("actionType".to_string(), serde_json::json!(action_type));
        }
        add_node_and_recurse(
            g,
            uri,
            &qualified,
            "action",
            action_name.to_string(),
            range,
            attrs,
            Some(parent_id),
        );
    }
    if let Some(action_type) = action_type {
        add_typing_edge_if_exists(g, uri, &qualified, action_type, container_prefix);
    }
    qualified
}

pub(super) fn add_expression_edge_if_both_exist(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    left: &sysml_parser::Node<sysml_parser::Expression>,
    right: &sysml_parser::Node<sysml_parser::Expression>,
    kind: RelationshipKind,
) {
    let left_str = expr_node_to_qualified_string(left);
    let right_str = expr_node_to_qualified_string(right);
    if left_str.is_empty() || right_str.is_empty() {
        return;
    }
    let src = if kind == RelationshipKind::Connection {
        match resolve_expression_endpoint_strict(g, uri, container_prefix, &left_str) {
            ResolveResult::Resolved(id) => id.qualified_name,
            ResolveResult::Ambiguous => {
                add_diagnostic_node(
                    g,
                    uri,
                    container_prefix,
                    "ambiguous_connection_endpoint",
                    format!(
                        "Ambiguous connection endpoint '{}'. Use a fully qualified endpoint path.",
                        left_str
                    ),
                    span_to_range(&left.span),
                );
                return;
            }
            ResolveResult::Unresolved => return,
        }
    } else {
        let Some(id) = resolve_expression_endpoint_legacy(g, uri, container_prefix, &left_str)
        else {
            if kind == RelationshipKind::Satisfy {
                add_diagnostic_node(
                    g,
                    uri,
                    container_prefix,
                    "unresolved_satisfy_source",
                    format!(
                        "Could not resolve satisfy source '{}'. Use a valid in-scope element name.",
                        left_str
                    ),
                    span_to_range(&left.span),
                );
            }
            return;
        };
        id
    };
    let tgt = if kind == RelationshipKind::Connection {
        match resolve_expression_endpoint_strict(g, uri, container_prefix, &right_str) {
            ResolveResult::Resolved(id) => id.qualified_name,
            ResolveResult::Ambiguous => {
                add_diagnostic_node(
                    g,
                    uri,
                    container_prefix,
                    "ambiguous_connection_endpoint",
                    format!(
                        "Ambiguous connection endpoint '{}'. Use a fully qualified endpoint path.",
                        right_str
                    ),
                    span_to_range(&right.span),
                );
                return;
            }
            ResolveResult::Unresolved => return,
        }
    } else {
        let Some(id) = resolve_expression_endpoint_legacy(g, uri, container_prefix, &right_str)
        else {
            if kind == RelationshipKind::Satisfy {
                add_diagnostic_node(
                    g,
                    uri,
                    container_prefix,
                    "unresolved_satisfy_target",
                    format!(
                        "Could not resolve satisfy target '{}'. Use a valid in-scope element name.",
                        right_str
                    ),
                    span_to_range(&right.span),
                );
            }
            return;
        };
        id
    };
    add_edge_if_both_exist(g, uri, &src, &tgt, kind.clone());
    if kind == RelationshipKind::Connection {
        g.record_connection_occurrence(
            uri,
            NodeId::new(uri, &src),
            NodeId::new(uri, &tgt),
            span_to_range(&left.span),
        );
    }
}

pub(super) fn expr_node_to_qualified_string(
    n: &sysml_parser::Node<sysml_parser::Expression>,
) -> String {
    use sysml_parser::Expression;
    match &n.value {
        Expression::FeatureRef(s) => s.clone(),
        Expression::MemberAccess(box_base, member) => {
            format!("{}::{}", expr_node_to_qualified_string(box_base), member)
        }
        _ => "".to_string(),
    }
}

pub(super) fn resolve_expression_endpoint_legacy(
    g: &SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    expression: &str,
) -> Option<String> {
    let mut candidates = Vec::new();
    if let Some(prefix) = container_prefix {
        candidates.push(format!("{}::{}", prefix, expression));
    }
    candidates.push(expression.to_string());

    for candidate in &candidates {
        let node_id = NodeId::new(uri, candidate);
        if g.node_index_by_id.contains_key(&node_id) {
            return Some(candidate.clone());
        }
    }

    let suffix = format!("::{}", expression);
    g.nodes_by_uri
        .get(uri)
        .into_iter()
        .flatten()
        .filter(|node_id| {
            node_id.qualified_name == expression || node_id.qualified_name.ends_with(&suffix)
        })
        .min_by_key(|node_id| node_id.qualified_name.len())
        .map(|node_id| node_id.qualified_name.clone())
}

pub(super) fn add_diagnostic_node(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    code: &str,
    message: String,
    range: Range,
) {
    let qualified = qualified_name_for_node(g, uri, container_prefix, code, "diagnostic");
    let mut attrs = HashMap::new();
    attrs.insert("code".to_string(), serde_json::json!(code));
    attrs.insert("message".to_string(), serde_json::json!(message));
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "diagnostic",
        code.to_string(),
        range,
        attrs,
        None,
    );
}
