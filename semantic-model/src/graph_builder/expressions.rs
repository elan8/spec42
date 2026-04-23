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
    left: &sysml_v2_parser::Node<sysml_v2_parser::Expression>,
    right: &sysml_v2_parser::Node<sysml_v2_parser::Expression>,
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

/// Best-effort display of an expression for attributes and diagnostics (not a full SysML text serializer).
pub(super) fn expression_to_debug_string(
    n: &sysml_v2_parser::Node<sysml_v2_parser::Expression>,
) -> String {
    use sysml_v2_parser::Expression;
    match &n.value {
        Expression::LiteralInteger(i) => i.to_string(),
        Expression::LiteralReal(s) => s.clone(),
        Expression::LiteralString(s) => format!("{s:?}"),
        Expression::LiteralBoolean(b) => b.to_string(),
        Expression::FeatureRef(s) => s.clone(),
        Expression::MemberAccess(box_base, member) => {
            format!("{}.{}", expression_to_debug_string(box_base), member)
        }
        Expression::Index { base, index } => {
            format!(
                "{}#({})",
                expression_to_debug_string(base),
                expression_to_debug_string(index)
            )
        }
        Expression::Bracket(inner) => {
            format!("[{}]", expression_to_debug_string(inner))
        }
        Expression::LiteralWithUnit { value, unit } => {
            format!(
                "{} [{}]",
                expression_to_debug_string(value),
                expression_to_unit_debug_string(unit)
            )
        }
        Expression::BinaryOp { op, left, right } => {
            format!(
                "({} {} {})",
                expression_to_debug_string(left),
                op,
                expression_to_debug_string(right)
            )
        }
        Expression::UnaryOp { op, operand } => {
            format!("({}{})", op, expression_to_debug_string(operand))
        }
        Expression::Tuple(items) => {
            let rendered = items
                .iter()
                .map(expression_to_debug_string)
                .collect::<Vec<_>>()
                .join(", ");
            format!("({rendered})")
        }
        Expression::Null => "()".to_string(),
    }
}

fn expression_to_unit_debug_string(
    n: &sysml_v2_parser::Node<sysml_v2_parser::Expression>,
) -> String {
    use sysml_v2_parser::Expression;
    match &n.value {
        // Unit expressions are already bracket-delimited in source syntax,
        // so unwrap here to avoid serializing as double brackets ("[[m]]").
        Expression::Bracket(inner) => expression_to_unit_debug_string(inner),
        _ => expression_to_debug_string(n),
    }
}

/// Path-like string for resolving connection/satisfy/transition endpoints where possible.
/// Literals and general expressions return empty so callers skip edge creation.
pub(super) fn expr_node_to_qualified_string(
    n: &sysml_v2_parser::Node<sysml_v2_parser::Expression>,
) -> String {
    use sysml_v2_parser::Expression;
    match &n.value {
        Expression::FeatureRef(s) => s.clone(),
        Expression::MemberAccess(box_base, member) => {
            let base = expr_node_to_qualified_string(box_base);
            if base.is_empty() {
                return String::new();
            }
            format!("{}::{}", base, member)
        }
        Expression::Index { base, index } => {
            let b = expr_node_to_qualified_string(base);
            if b.is_empty() {
                return String::new();
            }
            let i = expr_node_to_qualified_string(index);
            if i.is_empty() {
                let d = expression_to_debug_string(index);
                if d.is_empty() {
                    return String::new();
                }
                format!("{}#({})", b, d)
            } else {
                format!("{}#({})", b, i)
            }
        }
        Expression::Bracket(inner) => expr_node_to_qualified_string(inner),
        Expression::LiteralWithUnit { value, .. } => expr_node_to_qualified_string(value),
        Expression::LiteralInteger(_)
        | Expression::LiteralReal(_)
        | Expression::LiteralString(_)
        | Expression::LiteralBoolean(_)
        | Expression::BinaryOp { .. }
        | Expression::UnaryOp { .. }
        | Expression::Tuple(_)
        | Expression::Null => String::new(),
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

#[cfg(test)]
mod expr_string_tests {
    use super::{expr_node_to_qualified_string, expression_to_debug_string};
    use sysml_v2_parser::ast::{Expression, Node};
    use sysml_v2_parser::Span;

    fn node(expr: Expression) -> Node<Expression> {
        Node::new(Span::dummy(), expr)
    }

    #[test]
    fn qualified_string_member_chain() {
        let e = node(Expression::MemberAccess(
            Box::new(node(Expression::FeatureRef("a".into()))),
            "b".into(),
        ));
        assert_eq!(expr_node_to_qualified_string(&e), "a::b");
    }

    #[test]
    fn qualified_string_index_appends_slot() {
        let e = node(Expression::Index {
            base: Box::new(node(Expression::FeatureRef("w".into()))),
            index: Box::new(node(Expression::LiteralInteger(1))),
        });
        assert_eq!(expr_node_to_qualified_string(&e), "w#(1)");
    }

    #[test]
    fn qualified_string_bracket_unwraps() {
        let inner = node(Expression::FeatureRef("u".into()));
        let e = node(Expression::Bracket(Box::new(inner)));
        assert_eq!(expr_node_to_qualified_string(&e), "u");
    }

    #[test]
    fn debug_string_covers_binary_op() {
        let e = node(Expression::BinaryOp {
            op: "+".into(),
            left: Box::new(node(Expression::LiteralInteger(1))),
            right: Box::new(node(Expression::LiteralInteger(2))),
        });
        assert!(expression_to_debug_string(&e).contains('+'));
    }

    #[test]
    fn debug_string_literal_with_unit_avoids_double_brackets() {
        let e = node(Expression::LiteralWithUnit {
            value: Box::new(node(Expression::LiteralInteger(1))),
            unit: Box::new(node(Expression::Bracket(Box::new(node(
                Expression::FeatureRef("m".into()),
            ))))),
        });
        assert_eq!(expression_to_debug_string(&e), "1 [m]");
    }
}
