use std::collections::HashMap;

use crate::semantic::text_span::TextRange;
use url::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{
    ConnectStatementDetail, ElementKind, NodeId, RelationshipKind, SemanticEdge,
};
use crate::semantic::reference_resolution::{resolve_expression_endpoint_strict, ResolveResult};
use crate::semantic::relationships::{
    add_edge_if_both_exist, add_pending_expression_relationship, add_typing_edge_if_exists,
};
use crate::semantic::relationships::{add_semantic_edge_once, AddSemanticEdgeResult};

use super::{add_node_and_recurse, qualified_name, qualified_name_for_node};

fn is_action_like_kind(kind: &crate::ElementKind) -> bool {
    matches!(
        kind,
        ElementKind::Action | ElementKind::ActionDef | ElementKind::Perform | ElementKind::Merge
    )
}

pub(super) fn add_perform_usage_node(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    action_name: &str,
    action_type: Option<&str>,
    range: TextRange,
) -> String {
    let base = qualified_name(container_prefix, action_name);
    let base_id = NodeId::new(uri, &base);
    if let Some(existing) = g.get_node(&base_id) {
        if !is_action_like_kind(&existing.element_kind) {
            return base;
        }
    }

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
    if kind == RelationshipKind::Connection {
        let left_resolved = resolve_expression_endpoint_strict(g, uri, container_prefix, &left_str);
        let right_resolved =
            resolve_expression_endpoint_strict(g, uri, container_prefix, &right_str);
        match (left_resolved, right_resolved) {
            (ResolveResult::Resolved(src_id), ResolveResult::Resolved(tgt_id)) => {
                if add_semantic_edge_once(
                    g,
                    &src_id,
                    &tgt_id,
                    SemanticEdge::connection_with_connect(ConnectStatementDetail {
                        declaring_uri: uri.clone(),
                        range: span_to_range(&left.span),
                        source_expression: left_str,
                        target_expression: right_str,
                        container_prefix: container_prefix.map(ToString::to_string),
                    }),
                ) == AddSemanticEdgeResult::DuplicateConnect
                {
                    add_diagnostic_node(
                        g,
                        uri,
                        container_prefix,
                        "duplicate_connection",
                        "Duplicate connection between the same two endpoints.".to_string(),
                        span_to_range(&left.span),
                    );
                }
                return;
            }
            (ResolveResult::Ambiguous, _) => {
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
            (_, ResolveResult::Ambiguous) => {
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
            (ResolveResult::Unresolved, _) | (_, ResolveResult::Unresolved) => {
                add_pending_expression_relationship(
                    g,
                    uri,
                    container_prefix,
                    &left_str,
                    &right_str,
                    RelationshipKind::Connection,
                    span_to_range(&left.span),
                );
                return;
            }
        }
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
            if kind == RelationshipKind::Satisfy || kind == RelationshipKind::Allocate {
                add_pending_expression_relationship(
                    g,
                    uri,
                    container_prefix,
                    &left_str,
                    &right_str,
                    kind.clone(),
                    span_to_range(&left.span),
                );
                let code = if kind == RelationshipKind::Allocate {
                    "unresolved_allocate_source"
                } else {
                    "unresolved_satisfy_source"
                };
                let relation = if kind == RelationshipKind::Allocate {
                    "allocate"
                } else {
                    "satisfy"
                };
                add_diagnostic_node(
                    g,
                    uri,
                    container_prefix,
                    code,
                    format!(
                        "Could not resolve {} source '{}'. Use a valid in-scope element name.",
                        relation, left_str
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
            if kind == RelationshipKind::Satisfy || kind == RelationshipKind::Allocate {
                add_pending_expression_relationship(
                    g,
                    uri,
                    container_prefix,
                    &left_str,
                    &right_str,
                    kind.clone(),
                    span_to_range(&left.span),
                );
                let source_is_view = if kind == RelationshipKind::Satisfy {
                    let source_id = NodeId::new(uri, &src);
                    g.get_node(&source_id).is_some_and(|source_node| {
                        source_node.element_kind == ElementKind::View
                            || source_node.element_kind == ElementKind::ViewDef
                    })
                } else {
                    false
                };
                let code = if source_is_view {
                    "unresolved_viewpoint_conformance_target"
                } else if kind == RelationshipKind::Allocate {
                    "unresolved_allocate_target"
                } else {
                    "unresolved_satisfy_target"
                };
                let relation = if kind == RelationshipKind::Allocate {
                    "allocate"
                } else if source_is_view {
                    "viewpoint conformance"
                } else {
                    "satisfy"
                };
                add_diagnostic_node_with_attrs(
                    g,
                    uri,
                    container_prefix,
                    code,
                    format!(
                        "Could not resolve {} target '{}'. Use a valid in-scope element name.",
                        relation, right_str
                    ),
                    span_to_range(&right.span),
                    [("resolvedEndpoint", serde_json::json!(src.clone()))],
                );
            }
            return;
        };
        id
    };
    if kind == RelationshipKind::Connection {
        let src_id = NodeId::new(uri, &src);
        let tgt_id = NodeId::new(uri, &tgt);
        if matches!(
            add_semantic_edge_once(
                g,
                &src_id,
                &tgt_id,
                SemanticEdge::connection_with_connect(ConnectStatementDetail {
                    declaring_uri: uri.clone(),
                    range: span_to_range(&left.span),
                    source_expression: left_str.clone(),
                    target_expression: right_str.clone(),
                    container_prefix: container_prefix.map(ToString::to_string),
                }),
            ),
            AddSemanticEdgeResult::DuplicateConnect
        ) {
            add_diagnostic_node(
                g,
                uri,
                container_prefix,
                "duplicate_connection",
                "Duplicate connection between the same two endpoints.".to_string(),
                span_to_range(&left.span),
            );
        }
    } else {
        add_edge_if_both_exist(g, uri, &src, &tgt, kind.clone());
    }
}

fn feature_ref_is_classification(s: &str) -> bool {
    s.starts_with('@')
}

/// Structured expression classification for diagnostics and graph attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExprClass {
    Boolean,
    Classification,
    TypeCheck,
    Comparison,
    Logical,
    FeatureRef,
    Literal,
    Unknown,
}

impl ExprClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Boolean => "boolean",
            Self::Classification => "classification",
            Self::TypeCheck => "typeCheck",
            Self::Comparison => "comparison",
            Self::Logical => "logical",
            Self::FeatureRef => "featureRef",
            Self::Literal => "literal",
            Self::Unknown => "unknown",
        }
    }
}

pub(super) fn classify_expression(
    n: &sysml_v2_parser::Node<sysml_v2_parser::Expression>,
) -> ExprClass {
    use sysml_v2_parser::Expression;
    match &n.value {
        Expression::LiteralBoolean(_) => ExprClass::Boolean,
        Expression::Classification { .. } => ExprClass::Classification,
        Expression::MetaCast { .. } => ExprClass::FeatureRef,
        Expression::TypeCheck { .. } => ExprClass::TypeCheck,
        Expression::UnaryOp { op, operand } => {
            if op.as_str() == "not" {
                let inner = classify_expression(operand);
                if matches!(
                    inner,
                    ExprClass::Boolean
                        | ExprClass::Classification
                        | ExprClass::TypeCheck
                        | ExprClass::Comparison
                        | ExprClass::Logical
                ) {
                    return ExprClass::Boolean;
                }
            }
            ExprClass::Unknown
        }
        Expression::BinaryOp { op, left, right } => {
            if sysml_v2_parser::Expression::binary_op_is_comparison(op) {
                return ExprClass::Comparison;
            }
            if sysml_v2_parser::Expression::binary_op_is_logical(op) {
                return ExprClass::Logical;
            }
            let left_class = classify_expression(left);
            let right_class = classify_expression(right);
            if matches!(
                left_class,
                ExprClass::Boolean
                    | ExprClass::Classification
                    | ExprClass::TypeCheck
                    | ExprClass::Comparison
                    | ExprClass::Logical
            ) || matches!(
                right_class,
                ExprClass::Boolean
                    | ExprClass::Classification
                    | ExprClass::TypeCheck
                    | ExprClass::Comparison
                    | ExprClass::Logical
            ) {
                return ExprClass::Boolean;
            }
            ExprClass::Unknown
        }
        Expression::FeatureRef(s) if feature_ref_is_classification(s) => ExprClass::Classification,
        Expression::FeatureRef(_) => ExprClass::FeatureRef,
        Expression::LiteralInteger(_)
        | Expression::LiteralReal(_)
        | Expression::LiteralString(_)
        | Expression::LiteralWithUnit { .. } => ExprClass::Literal,
        Expression::Bracket(inner) => classify_expression(inner),
        Expression::MemberAccess(_, _)
        | Expression::Index { .. }
        | Expression::Invocation { .. }
        | Expression::Tuple(_)
        | Expression::Select { .. }
        | Expression::Collect { .. }
        | Expression::Null => ExprClass::Unknown,
    }
}

/// Whether an expression is intended to evaluate to Boolean (conservative).
pub(super) fn expression_is_boolean_valued(
    n: &sysml_v2_parser::Node<sysml_v2_parser::Expression>,
) -> bool {
    use sysml_v2_parser::Expression;
    match classify_expression(n) {
        ExprClass::Boolean | ExprClass::Classification | ExprClass::TypeCheck => true,
        ExprClass::Comparison | ExprClass::Logical => true,
        ExprClass::FeatureRef => matches!(
            &n.value,
            Expression::FeatureRef(s) if feature_ref_is_classification(s)
        ),
        ExprClass::Literal | ExprClass::Unknown => false,
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
        Expression::Classification { metaclass } => format!("@{metaclass}"),
        Expression::MetaCast { base, metaclass } => {
            format!("{} meta {metaclass}", expression_to_debug_string(base))
        }
        Expression::TypeCheck {
            kind,
            operand,
            type_name,
        } => {
            let op = match kind {
                sysml_v2_parser::TypeCheckKind::Istype => "istype",
                sysml_v2_parser::TypeCheckKind::Hastype => "hastype",
                sysml_v2_parser::TypeCheckKind::As => "as",
            };
            match operand {
                Some(operand) => {
                    format!("{} {op} {type_name}", expression_to_debug_string(operand))
                }
                None => format!("{op} {type_name}"),
            }
        }
        Expression::Select { base, selector } => {
            format!("{}.?{selector}", expression_to_debug_string(base))
        }
        Expression::Collect { base, selector } => {
            format!("{}.**{selector}", expression_to_debug_string(base))
        }
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
                op.as_str(),
                expression_to_debug_string(right)
            )
        }
        Expression::UnaryOp { op, operand } => {
            format!("({}{})", op.as_str(), expression_to_debug_string(operand))
        }
        Expression::Invocation { callee, args } => {
            let rendered = args
                .iter()
                .map(expression_to_debug_string)
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({rendered})", expression_to_debug_string(callee))
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
        | Expression::Invocation { .. }
        | Expression::Tuple(_)
        | Expression::Classification { .. }
        | Expression::MetaCast { .. }
        | Expression::TypeCheck { .. }
        | Expression::Select { .. }
        | Expression::Collect { .. }
        | Expression::Null => String::new(),
    }
}

pub(super) fn resolve_expression_endpoint_legacy(
    g: &SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    expression: &str,
) -> Option<String> {
    crate::semantic::resolution::resolve_expression_endpoint_qualified(
        g,
        uri,
        container_prefix,
        expression,
    )
}

pub(super) fn add_diagnostic_node(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    code: &str,
    message: String,
    range: TextRange,
) {
    add_diagnostic_node_with_attrs(
        g,
        uri,
        container_prefix,
        code,
        message,
        range,
        std::iter::empty::<(&str, serde_json::Value)>(),
    );
}

fn add_diagnostic_node_with_attrs(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    code: &str,
    message: String,
    range: TextRange,
    extra_attrs: impl IntoIterator<Item = (&'static str, serde_json::Value)>,
) {
    let qualified = qualified_name_for_node(g, uri, container_prefix, code, "diagnostic");
    let mut attrs = HashMap::new();
    attrs.insert("code".to_string(), serde_json::json!(code));
    attrs.insert("message".to_string(), serde_json::json!(message));
    for (key, value) in extra_attrs {
        attrs.insert(key.to_string(), value);
    }
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
    use super::{
        expr_node_to_qualified_string, expression_to_debug_string,
        resolve_expression_endpoint_legacy, resolve_expression_endpoint_strict,
    };
    use crate::semantic::relationships::add_cross_document_edges_for_uri;
    use crate::{build_graph_from_doc, ResolveResult};
    use sysml_v2_parser::ast::{BinaryOperator, Expression, Node};
    use sysml_v2_parser::Span;
    use url::Url;

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
            op: BinaryOperator::from_token("+"),
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

    #[test]
    fn legacy_endpoint_resolution_follows_typed_member_chain_across_documents() {
        let architecture = r#"
            package WebShopArchitecture {
                part def CheckoutService {}
                part def WebShopSystem {
                    part checkoutService : CheckoutService;
                }
            }
        "#;
        let instance = r#"
            package WebShopExample {
                import WebShopArchitecture::*;
                part webshopSystem : WebShopSystem;
            }
        "#;

        let architecture_uri = Url::parse("file:///WebShopArchitecture.sysml").expect("arch uri");
        let instance_uri = Url::parse("file:///webshop.sysml").expect("instance uri");
        let architecture_root = sysml_v2_parser::parse(architecture).expect("parse architecture");
        let instance_root = sysml_v2_parser::parse(instance).expect("parse instance");

        let mut graph = build_graph_from_doc(&architecture_root, &architecture_uri);
        graph.merge(build_graph_from_doc(&instance_root, &instance_uri));
        add_cross_document_edges_for_uri(&mut graph, &instance_uri);

        let resolved = resolve_expression_endpoint_legacy(
            &graph,
            &instance_uri,
            Some("WebShopExample"),
            "webshopSystem::checkoutService",
        );

        assert_eq!(
            resolved.as_deref(),
            Some("WebShopArchitecture::WebShopSystem::checkoutService")
        );
    }

    #[test]
    fn legacy_endpoint_resolution_follows_member_imported_instance_name() {
        let architecture = r#"
            package WebShopArchitecture {
                part def CheckoutService {}
                part def WebShopSystem {
                    part checkoutService : CheckoutService;
                }
                part webshopSystem : WebShopSystem;
            }
        "#;
        let usage = r#"
            package WebShopExample {
                import WebShopArchitecture::webshopSystem;
            }
        "#;

        let architecture_uri = Url::parse("file:///WebShopArchitecture.sysml").expect("arch uri");
        let usage_uri = Url::parse("file:///webshop.sysml").expect("usage uri");
        let architecture_root = sysml_v2_parser::parse(architecture).expect("parse architecture");
        let usage_root = sysml_v2_parser::parse(usage).expect("parse usage");

        let mut graph = build_graph_from_doc(&architecture_root, &architecture_uri);
        graph.merge(build_graph_from_doc(&usage_root, &usage_uri));
        add_cross_document_edges_for_uri(&mut graph, &usage_uri);

        let owner = resolve_expression_endpoint_strict(
            &graph,
            &usage_uri,
            Some("WebShopExample"),
            "webshopSystem",
        );
        assert!(matches!(owner, ResolveResult::Resolved(_)));

        let resolved = resolve_expression_endpoint_legacy(
            &graph,
            &usage_uri,
            Some("WebShopExample"),
            "webshopSystem::checkoutService",
        );

        assert_eq!(
            resolved.as_deref(),
            Some("WebShopArchitecture::WebShopSystem::checkoutService")
        );
    }

    #[test]
    fn legacy_endpoint_resolution_supports_qualified_package_path_across_documents() {
        let architecture = r#"
            package WebShopArchitecture {
                part def WebShopSystem {}
                part webshopSystem : WebShopSystem;
            }
        "#;
        let usage = r#"
            package WebShopExample {
                import WebShopArchitecture::*;
            }
        "#;

        let architecture_uri = Url::parse("file:///WebShopArchitecture.sysml").expect("arch uri");
        let usage_uri = Url::parse("file:///webshop.sysml").expect("usage uri");
        let architecture_root = sysml_v2_parser::parse(architecture).expect("parse architecture");
        let usage_root = sysml_v2_parser::parse(usage).expect("parse usage");

        let mut graph = build_graph_from_doc(&architecture_root, &architecture_uri);
        graph.merge(build_graph_from_doc(&usage_root, &usage_uri));
        add_cross_document_edges_for_uri(&mut graph, &usage_uri);

        let resolved = resolve_expression_endpoint_legacy(
            &graph,
            &usage_uri,
            Some("WebShopExample"),
            "WebShopArchitecture::webshopSystem",
        );

        assert_eq!(
            resolved.as_deref(),
            Some("WebShopArchitecture::webshopSystem")
        );
    }
}
