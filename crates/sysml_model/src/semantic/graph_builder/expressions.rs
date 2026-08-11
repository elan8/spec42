use std::collections::HashMap;

use crate::semantic::text_span::TextRange;
use url::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::expression_fold::{fold_expression, ExpressionAlgebra, FoldedChild};
use crate::semantic::graph::{DeclaredExpressionRelationshipRecord, SemanticGraph};
use crate::semantic::model::{
    ConnectStatementDetail, ConstructionOwner, DeclaredExpressionRelationship, ElementKind, NodeId,
    RelationshipKind, SemanticEdge,
};
use crate::semantic::reference_resolution::{resolve_expression_endpoint_strict, ResolveResult};
use crate::semantic::relationships::{
    add_edge_if_both_exist, add_pending_expression_relationship,
    add_pending_expression_relationship_with_metadata, add_typing_edge_if_exists,
    ExpressionRelationshipMetadata,
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
    add_expression_edge_with_metadata(
        g,
        uri,
        container_prefix,
        left,
        right,
        ExpressionRelationshipMetadata::plain(kind),
    );
}

pub(super) fn add_interface_edge_if_both_exist(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    left: &sysml_v2_parser::Node<sysml_v2_parser::Expression>,
    right: &sysml_v2_parser::Node<sysml_v2_parser::Expression>,
    interface_type: Option<&str>,
) {
    add_expression_edge_with_metadata(
        g,
        uri,
        container_prefix,
        left,
        right,
        ExpressionRelationshipMetadata::interface(interface_type.map(ToString::to_string)),
    );
}

pub(super) fn record_declared_expression_relationship(
    g: &mut SemanticGraph,
    owner: NodeId,
    kind: RelationshipKind,
    source_expression: String,
    target_expression: String,
    source_range: TextRange,
    target_range: Option<TextRange>,
    is_interface_usage: bool,
    interface_type: Option<String>,
) {
    let authored_ordinal = g
        .declared_expression_relationships
        .iter()
        .filter(|record| record.owner == owner)
        .count() as u32;
    g.declared_expression_relationships
        .push(DeclaredExpressionRelationshipRecord {
            owner: owner.clone(),
            authored_ordinal,
            relationship: DeclaredExpressionRelationship {
                kind,
                source_expression,
                target_expression,
                scope_owner: Some(owner),
                source_range,
                target_range,
                is_interface_usage,
                interface_type,
            },
        });
}

fn add_expression_edge_with_metadata(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    left: &sysml_v2_parser::Node<sysml_v2_parser::Expression>,
    right: &sysml_v2_parser::Node<sysml_v2_parser::Expression>,
    metadata: ExpressionRelationshipMetadata,
) {
    let kind = metadata.kind.clone();
    let is_interface_usage = metadata.is_interface_usage;
    let interface_type = metadata.interface_type.clone();
    let left_str = expr_node_to_qualified_string(left);
    let right_str = expr_node_to_qualified_string(right);
    if g.structural_input_only {
        let owner = container_prefix
            .map(|prefix| NodeId::new(uri, prefix))
            .filter(|id| g.get_node(id).is_some())
            .or_else(|| g.root_scope_id(uri))
            .expect("structural documents always materialize a root semantic scope");
        record_declared_expression_relationship(
            g,
            owner,
            kind.clone(),
            left_str,
            right_str,
            span_to_range(&left.span),
            Some(span_to_range(&right.span)),
            is_interface_usage,
            interface_type,
        );
        return;
    }
    if matches!(kind, RelationshipKind::Connection | RelationshipKind::Bind) {
        let left_resolved = resolve_expression_endpoint_strict(g, uri, container_prefix, &left_str);
        let right_resolved =
            resolve_expression_endpoint_strict(g, uri, container_prefix, &right_str);
        match (left_resolved, right_resolved) {
            (ResolveResult::Resolved(src_id), ResolveResult::Resolved(tgt_id)) => {
                if kind == RelationshipKind::Bind {
                    add_semantic_edge_once(
                        g,
                        &src_id,
                        &tgt_id,
                        SemanticEdge::interconnection_with_detail(
                            kind,
                            ConnectStatementDetail {
                                declaring_uri: uri.clone(),
                                range: span_to_range(&left.span),
                                source_expression: left_str,
                                target_expression: right_str,
                                container_prefix: container_prefix.map(ToString::to_string),
                                is_interface_usage,
                                interface_type,
                            },
                            ConstructionOwner::DocumentConstruction,
                        ),
                    );
                    return;
                }
                if add_semantic_edge_once(
                    g,
                    &src_id,
                    &tgt_id,
                    SemanticEdge::connection_with_connect(
                        ConnectStatementDetail {
                            declaring_uri: uri.clone(),
                            range: span_to_range(&left.span),
                            source_expression: left_str,
                            target_expression: right_str,
                            container_prefix: container_prefix.map(ToString::to_string),
                            is_interface_usage,
                            interface_type: interface_type.clone(),
                        },
                        ConstructionOwner::DocumentConstruction,
                    ),
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
                add_pending_expression_relationship_with_metadata(
                    g,
                    uri,
                    container_prefix,
                    &left_str,
                    &right_str,
                    span_to_range(&left.span),
                    metadata.clone(),
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
                SemanticEdge::connection_with_connect(
                    ConnectStatementDetail {
                        declaring_uri: uri.clone(),
                        range: span_to_range(&left.span),
                        source_expression: left_str.clone(),
                        target_expression: right_str.clone(),
                        container_prefix: container_prefix.map(ToString::to_string),
                        is_interface_usage,
                        interface_type: interface_type.clone(),
                    },
                    ConstructionOwner::DocumentConstruction,
                ),
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

fn is_booleanish(class: ExprClass) -> bool {
    matches!(
        class,
        ExprClass::Boolean
            | ExprClass::Classification
            | ExprClass::TypeCheck
            | ExprClass::Comparison
            | ExprClass::Logical
    )
}

/// Iterative, not recursive -- but unlike `expression_to_debug_string`/`declared_expression`, this
/// only ever follows `UnaryOp`'s operand (and only when `op == "not"`), `BinaryOp`'s `left`/`right`
/// (and only when the operator is neither comparison nor logical), and `Bracket`/`Parenthesized`'s
/// inner value; every other variant is a leaf for this function's purposes. So it gets its own
/// small dedicated stack machine instead of `expression_fold`'s shared "visit everything" engine
/// (same reasoning as `expr_node_to_qualified_string` above) -- reusing that engine would make it
/// walk subtrees (e.g. a large `Invocation`/`Tuple`) it currently never looks at.
pub(super) fn classify_expression(
    n: &sysml_v2_parser::Node<sysml_v2_parser::Expression>,
) -> ExprClass {
    use sysml_v2_parser::Expression;
    use sysml_v2_parser::Node;

    enum Frame<'a> {
        Enter(&'a Node<Expression>),
        AfterUnaryOperand,
        AfterBinaryLeft { right: &'a Node<Expression> },
        AfterBinaryBoth,
    }

    let mut work = vec![Frame::Enter(n)];
    let mut results: Vec<ExprClass> = Vec::new();

    while let Some(frame) = work.pop() {
        match frame {
            Frame::Enter(node) => match &node.value {
                Expression::LiteralBoolean(_) => results.push(ExprClass::Boolean),
                Expression::Classification { .. } => results.push(ExprClass::Classification),
                Expression::MetaCast { .. } => results.push(ExprClass::FeatureRef),
                Expression::TypeCheck { .. } => results.push(ExprClass::TypeCheck),
                Expression::UnaryOp { op, operand } if op.as_str() == "not" => {
                    work.push(Frame::AfterUnaryOperand);
                    work.push(Frame::Enter(operand));
                }
                Expression::UnaryOp { .. } => results.push(ExprClass::Unknown),
                Expression::BinaryOp { op, .. }
                    if sysml_v2_parser::Expression::binary_op_is_comparison(op) =>
                {
                    results.push(ExprClass::Comparison);
                }
                Expression::BinaryOp { op, .. }
                    if sysml_v2_parser::Expression::binary_op_is_logical(op) =>
                {
                    results.push(ExprClass::Logical);
                }
                Expression::BinaryOp { left, right, .. } => {
                    work.push(Frame::AfterBinaryLeft { right });
                    work.push(Frame::Enter(left));
                }
                Expression::FeatureRef(s) if feature_ref_is_classification(s) => {
                    results.push(ExprClass::Classification)
                }
                Expression::FeatureRef(_) => results.push(ExprClass::FeatureRef),
                Expression::LiteralInteger(_)
                | Expression::LiteralReal(_)
                | Expression::LiteralString(_)
                | Expression::LiteralWithUnit { .. } => results.push(ExprClass::Literal),
                Expression::Bracket(inner) | Expression::Parenthesized(inner) => {
                    work.push(Frame::Enter(inner));
                }
                Expression::MetadataAccess(_) | Expression::FeatureChainRef(_) => {
                    results.push(ExprClass::FeatureRef)
                }
                Expression::MemberAccess(_, _)
                | Expression::Index { .. }
                | Expression::Invocation { .. }
                | Expression::Tuple(_)
                | Expression::Select { .. }
                | Expression::Collect { .. }
                | Expression::Constructor { .. }
                | Expression::CollectionOp { .. }
                | Expression::Conditional { .. }
                | Expression::Extent { .. }
                | Expression::Null => results.push(ExprClass::Unknown),
            },
            Frame::AfterUnaryOperand => {
                let inner = results.pop().unwrap_or(ExprClass::Unknown);
                results.push(if is_booleanish(inner) {
                    ExprClass::Boolean
                } else {
                    ExprClass::Unknown
                });
            }
            Frame::AfterBinaryLeft { right } => {
                let left_class = results.pop().unwrap_or(ExprClass::Unknown);
                results.push(left_class);
                work.push(Frame::AfterBinaryBoth);
                work.push(Frame::Enter(right));
            }
            Frame::AfterBinaryBoth => {
                let right_class = results.pop().unwrap_or(ExprClass::Unknown);
                let left_class = results.pop().unwrap_or(ExprClass::Unknown);
                results.push(if is_booleanish(left_class) || is_booleanish(right_class) {
                    ExprClass::Boolean
                } else {
                    ExprClass::Unknown
                });
            }
        }
    }

    results.pop().unwrap_or(ExprClass::Unknown)
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

fn render_argument(name: &Option<String>, value: &str) -> String {
    match name {
        Some(name) => format!("{name} = {value}"),
        None => value.to_string(),
    }
}

struct DebugStringAlgebra;

impl ExpressionAlgebra for DebugStringAlgebra {
    type Output = String;

    /// Mirrors the pre-0.47.0 recursive `expression_to_debug_string` exactly, with each recursive
    /// call replaced by that child's already-folded string from `subs`/`arguments` -- see the
    /// module doc on [`crate::semantic::expression_fold`] for why. The one exception is
    /// `LiteralWithUnit`'s `unit` child: the original deliberately renders it through
    /// `expression_to_unit_debug_string` (which strips one `Bracket` layer to avoid "[[m]]"), not
    /// through this algebra's own generic `Bracket` handling -- so that arm ignores its folded
    /// `unit` child and calls `expression_to_unit_debug_string` on the original AST node instead
    /// (cheap and no longer recursion-risk, see that function).
    fn build(
        &mut self,
        node: &sysml_v2_parser::Node<sysml_v2_parser::Expression>,
        children: Vec<FoldedChild<String>>,
    ) -> String {
        use sysml_v2_parser::Expression;
        let mut subs = Vec::new();
        let mut arguments: Vec<(Option<String>, String)> = Vec::new();
        for child in children {
            match child {
                FoldedChild::Sub(value) => subs.push(value),
                FoldedChild::Argument { name, value } => arguments.push((name, value)),
            }
        }
        let rendered_args = || {
            arguments
                .iter()
                .map(|(name, value)| render_argument(name, value))
                .collect::<Vec<_>>()
                .join(", ")
        };
        match &node.value {
            Expression::LiteralInteger(i) => i.to_string(),
            Expression::LiteralReal(s) => s.clone(),
            Expression::LiteralString(s) => format!("{s:?}"),
            Expression::LiteralBoolean(b) => b.to_string(),
            Expression::Classification { metaclass } => format!("@{metaclass}"),
            Expression::MetaCast { metaclass, .. } => format!("{} meta {metaclass}", subs[0]),
            Expression::TypeCheck {
                kind, type_name, ..
            } => {
                let op = match kind {
                    sysml_v2_parser::TypeCheckKind::Istype => "istype",
                    sysml_v2_parser::TypeCheckKind::Hastype => "hastype",
                    sysml_v2_parser::TypeCheckKind::As => "as",
                };
                match subs.first() {
                    Some(operand) => format!("{operand} {op} {type_name}"),
                    None => format!("{op} {type_name}"),
                }
            }
            Expression::Select { selector, .. } => format!("{}.?{selector}", subs[0]),
            Expression::Collect { selector, .. } => format!("{}.**{selector}", subs[0]),
            Expression::FeatureRef(s) => s.clone(),
            Expression::MemberAccess(_, member) => format!("{}.{}", subs[0], member),
            Expression::Index { .. } => format!("{}#({})", subs[0], subs[1]),
            Expression::Bracket(_) => format!("[{}]", subs[0]),
            Expression::LiteralWithUnit { unit, .. } => {
                format!("{} [{}]", subs[0], expression_to_unit_debug_string(unit))
            }
            Expression::BinaryOp { op, .. } => format!("({} {} {})", subs[0], op.as_str(), subs[1]),
            Expression::UnaryOp { op, .. } => format!("({}{})", op.as_str(), subs[0]),
            Expression::Invocation { .. } => format!("{}({})", subs[0], rendered_args()),
            Expression::Tuple(_) => format!("({})", subs.join(", ")),
            Expression::Parenthesized(_) => format!("({})", subs[0]),
            Expression::Constructor { type_name, .. } => {
                format!("new {type_name}({})", rendered_args())
            }
            Expression::FeatureChainRef(chain) => chain.segments.join("."),
            Expression::CollectionOp { op, .. } => {
                format!("{}->{}({})", subs[0], op.as_str(), rendered_args())
            }
            Expression::MetadataAccess(_) => format!("{}.metadata", subs[0]),
            Expression::Null => "()".to_string(),
            Expression::Conditional { .. } => {
                format!("if {} ? {} else {}", subs[0], subs[1], subs[2])
            }
            Expression::Extent { target } => format!("all {target}"),
        }
    }
}

/// Best-effort display of an expression for attributes and diagnostics (not a full SysML text
/// serializer). Iterative, not recursive: see [`crate::semantic::expression_fold`] for why.
pub(crate) fn expression_to_debug_string(
    n: &sysml_v2_parser::Node<sysml_v2_parser::Expression>,
) -> String {
    fold_expression(n, &mut DebugStringAlgebra)
}

/// Unit expressions are already bracket-delimited in source syntax, so unwrap here to avoid
/// serializing as double brackets ("[[m]]"). A `while` loop, not recursion: the parser only ever
/// constructs a `LiteralWithUnit.unit` as exactly one `Bracket` around a `FeatureRef`, but nothing
/// stops this field from statically holding a deeper `Expression`, so this stays non-recursive on
/// principle rather than relying on that always being true.
fn expression_to_unit_debug_string(
    n: &sysml_v2_parser::Node<sysml_v2_parser::Expression>,
) -> String {
    use sysml_v2_parser::Expression;
    let mut current = n;
    while let Expression::Bracket(inner) = &current.value {
        current = inner;
    }
    expression_to_debug_string(current)
}

/// Path-like string for resolving connection/satisfy/transition endpoints where possible.
/// Literals and general expressions return empty so callers skip edge creation.
///
/// Iterative, not recursive -- but unlike `expression_to_debug_string`/`declared_expression`, this
/// function only ever follows a narrow subset of children (`MemberAccess`'s base, `Index`'s
/// base/index, `Bracket`/`Parenthesized`/`MetadataAccess`'s inner, `LiteralWithUnit`'s value) and
/// short-circuits on an empty base without even visiting `index`, so it gets its own small
/// dedicated stack machine instead of `expression_fold`'s shared "visit everything" engine
/// (matching `classify_expression`'s reasoning below) -- reusing that engine would make it walk
/// subtrees (e.g. a large `Invocation`/`Tuple`) it currently skips entirely.
pub(super) fn expr_node_to_qualified_string(
    n: &sysml_v2_parser::Node<sysml_v2_parser::Expression>,
) -> String {
    use sysml_v2_parser::Expression;
    use sysml_v2_parser::Node;

    enum Frame<'a> {
        Enter(&'a Node<Expression>),
        AfterMemberAccessBase { member: &'a str },
        AfterIndexBase { index: &'a Node<Expression> },
        AfterIndexBoth { index: &'a Node<Expression> },
    }

    let mut work = vec![Frame::Enter(n)];
    let mut results: Vec<String> = Vec::new();

    while let Some(frame) = work.pop() {
        match frame {
            Frame::Enter(node) => match &node.value {
                Expression::FeatureRef(s) => results.push(s.clone()),
                Expression::FeatureChainRef(chain) => results.push(chain.segments.join("::")),
                Expression::MemberAccess(base, member) => {
                    work.push(Frame::AfterMemberAccessBase { member });
                    work.push(Frame::Enter(base));
                }
                Expression::Index { base, index } => {
                    work.push(Frame::AfterIndexBase { index });
                    work.push(Frame::Enter(base));
                }
                Expression::Bracket(inner)
                | Expression::Parenthesized(inner)
                | Expression::MetadataAccess(inner) => {
                    work.push(Frame::Enter(inner));
                }
                Expression::LiteralWithUnit { value, .. } => {
                    work.push(Frame::Enter(value));
                }
                _ => results.push(String::new()),
            },
            Frame::AfterMemberAccessBase { member } => {
                let base = results.pop().unwrap_or_default();
                results.push(if base.is_empty() {
                    String::new()
                } else {
                    format!("{base}::{member}")
                });
            }
            Frame::AfterIndexBase { index } => {
                let base = results.pop().unwrap_or_default();
                if base.is_empty() {
                    results.push(String::new());
                } else {
                    results.push(base);
                    work.push(Frame::AfterIndexBoth { index });
                    work.push(Frame::Enter(index));
                }
            }
            Frame::AfterIndexBoth { index } => {
                let index_str = results.pop().unwrap_or_default();
                let base = results.pop().unwrap_or_default();
                results.push(if !index_str.is_empty() {
                    format!("{base}#({index_str})")
                } else {
                    let debug = expression_to_debug_string(index);
                    if debug.is_empty() {
                        String::new()
                    } else {
                        format!("{base}#({debug})")
                    }
                });
            }
        }
    }

    results.pop().unwrap_or_default()
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
        classify_expression, expr_node_to_qualified_string, expression_to_debug_string,
        resolve_expression_endpoint_legacy, ExprClass,
    };
    use crate::build_graph_from_doc;
    use crate::semantic::relationships::add_cross_document_edges_for_uri;
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
    fn canonical_publication_resolves_imported_instance_type() {
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

        let architecture = crate::semantic::source::SysmlDocument::from_uri(
            "file:///WebShopArchitecture.sysml",
            architecture.to_string(),
            None,
            crate::semantic::source::SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("architecture document");
        let usage = crate::semantic::source::SysmlDocument::from_uri(
            "file:///webshop.sysml",
            usage.to_string(),
            None,
            crate::semantic::source::SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("usage document");
        let snapshot = crate::ImmutableSourceSnapshot::new(vec![architecture, usage])
            .expect("source snapshot");
        let model = crate::build_semantic_model(crate::SemanticBuildRequest {
            sources: snapshot,
            construction: crate::ConstructionStrategy::Sequential,
            evaluation: crate::EvaluationPolicy::ResolvedOnly,
            configuration: crate::SemanticConfiguration::default(),
        })
        .expect("canonical semantic model");
        assert!(model.resolution().facts().iter().any(|fact| {
            fact.reference.source.qualified_name == "WebShopExample::webshopSystem"
                && fact.reference.kind == crate::ReferenceKind::MembershipImport
                && matches!(
                    &fact.outcome,
                    crate::ResolutionOutcome::Resolved { target }
                        if target.qualified_name == "WebShopArchitecture::webshopSystem"
                )
        }));
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

    fn deep_parenthesized_chain(depth: usize) -> Node<Expression> {
        let mut tree = node(Expression::LiteralInteger(1));
        for _ in 0..depth {
            tree = node(Expression::Parenthesized(Box::new(tree)));
        }
        tree
    }

    #[test]
    fn expression_to_debug_string_handles_deep_nesting_without_overflowing_the_stack() {
        const DEPTH: usize = 200_000;
        let tree = deep_parenthesized_chain(DEPTH);
        let rendered = expression_to_debug_string(&tree);
        let expected = format!("{}1{}", "(".repeat(DEPTH), ")".repeat(DEPTH));
        assert_eq!(rendered, expected);
    }

    #[test]
    fn qualified_string_handles_deep_bracket_nesting_without_overflowing_the_stack() {
        // `expr_node_to_qualified_string` unwraps `Bracket`/`Parenthesized` transparently, so a
        // deep chain of either collapses down to the innermost feature reference.
        let mut tree = node(Expression::FeatureRef("x".into()));
        for _ in 0..200_000 {
            tree = node(Expression::Bracket(Box::new(tree)));
        }
        assert_eq!(expr_node_to_qualified_string(&tree), "x");
    }

    #[test]
    fn classify_expression_handles_deep_unary_not_chain_without_overflowing_the_stack() {
        use sysml_v2_parser::ast::UnaryOperator;

        let mut tree = node(Expression::LiteralBoolean(true));
        for _ in 0..200_000 {
            tree = node(Expression::UnaryOp {
                op: UnaryOperator::from_token("not"),
                operand: Box::new(tree),
            });
        }
        assert_eq!(classify_expression(&tree), ExprClass::Boolean);
    }
}
