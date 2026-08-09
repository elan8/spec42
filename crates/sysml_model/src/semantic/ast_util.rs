//! Helpers for working with sysml-v2-parser AST: span/range conversion and name extraction.

use std::collections::HashMap;

use crate::semantic::expression_fold::{fold_expression, ExpressionAlgebra, FoldedChild};
use crate::semantic::model::{
    DeclaredExpression, DeclaredExpressionArgument, DeclaredExpressionKind,
    DeclaredFeatureProperties, DeclaredFeatureValue, DeclaredFeatureValueKind, DeclaredImportFacts,
    DeclaredImportTarget, DeclaredMembershipFacts, DeclaredMembershipKind, DeclaredMultiplicity,
    DeclaredRelationshipTarget, ImportOrigin, ImportShape, VisibilityKind,
};
use crate::semantic::text_span::{TextPosition, TextRange};
use sysml_v2_parser::ast::{
    Argument, ConnectionEnd, DefinitionPrefix, Identification, InOut, Membership, MembershipKind,
    Node, SubsettingRelationship, TypingRelationship,
};
use sysml_v2_parser::ast::{ExposeMember, Import, Visibility};
use sysml_v2_parser::{Expression, Span};

/// Maps a parser direction prefix to the Systems Modeling API direction token.
pub fn direction_name(direction: InOut) -> &'static str {
    match direction {
        InOut::In => "in",
        InOut::Out => "out",
        InOut::InOut => "inout",
    }
}

/// Maps an optional `abstract` / `variation` definition or usage prefix.
pub fn definition_prefix_flags(prefix: Option<&DefinitionPrefix>) -> (bool, bool) {
    match prefix {
        Some(DefinitionPrefix::Abstract) => (true, false),
        Some(DefinitionPrefix::Variation) => (false, true),
        None => (false, false),
    }
}

/// Parser-backed ownership facts for an explicit `ref` usage.
///
/// An ordinary usage does not author either half of the ownership pair. Its composite default is
/// contextual and is therefore published later as an effective graph fact, after containment is
/// complete. Keeping it out of this AST adapter preserves the distinction between what appeared
/// in the source and what SysML supplies by default.
fn usage_ownership_from_ref_flag(is_reference: bool) -> (Option<bool>, Option<bool>) {
    if is_reference {
        (Some(false), Some(true))
    } else {
        (None, None)
    }
}

fn visibility_kind(visibility: Visibility) -> VisibilityKind {
    match visibility {
        Visibility::Public => VisibilityKind::Public,
        Visibility::Private => VisibilityKind::Private,
        Visibility::Protected => VisibilityKind::Protected,
    }
}

fn membership_kind(kind: MembershipKind) -> DeclaredMembershipKind {
    match kind {
        MembershipKind::OwningMembership => DeclaredMembershipKind::Owning,
        MembershipKind::FeatureMembership => DeclaredMembershipKind::Feature,
        MembershipKind::Import => DeclaredMembershipKind::Import,
        MembershipKind::Alias => DeclaredMembershipKind::Alias,
        MembershipKind::VariantMembership => DeclaredMembershipKind::Variant,
        MembershipKind::ActorMembership => DeclaredMembershipKind::Actor,
    }
}

/// Retains exactly the membership information supplied by the parser. In particular, no absent
/// visibility is defaulted here: defaults require the owning graph context.
pub fn declared_membership_facts(membership: &Membership) -> DeclaredMembershipFacts {
    DeclaredMembershipFacts {
        kind: membership_kind(membership.kind),
        visibility: membership.visibility.map(visibility_kind),
        range: Some(span_to_range(&membership.span)),
        import: None,
    }
}

/// Parser-backed import membership facts. Filter-package imports remain a separate shape so
/// downstream resolution cannot accidentally treat an unsupported filter as `::*`.
pub fn declared_import_membership_facts(import: &Node<Import>) -> DeclaredMembershipFacts {
    let value = &import.value;
    let shape = if value.filter_members.is_some() {
        ImportShape::FilteredNamespace
    } else if value.is_import_all {
        ImportShape::Namespace
    } else {
        ImportShape::Membership
    };
    DeclaredMembershipFacts {
        kind: membership_kind(value.membership.kind),
        visibility: value.membership.visibility.map(visibility_kind),
        range: Some(span_to_range(&value.membership.span)),
        import: Some(DeclaredImportFacts {
            target: DeclaredImportTarget {
                reference: value.target.clone(),
                range: Some(span_to_range(&value.target_span)),
            },
            origin: ImportOrigin::Import,
            shape,
            recursive: value.is_recursive,
        }),
    }
}

/// `ExposeMember` currently has no target sub-span or membership wrapper in the parser AST.
/// Retain that absence explicitly rather than inventing source precision or visibility.
pub fn declared_expose_membership_facts(expose: &Node<ExposeMember>) -> DeclaredMembershipFacts {
    let value = &expose.value;
    DeclaredMembershipFacts {
        kind: DeclaredMembershipKind::Import,
        visibility: None,
        range: Some(span_to_range(&expose.span)),
        import: Some(DeclaredImportFacts {
            target: DeclaredImportTarget {
                reference: value.target.clone(),
                range: None,
            },
            origin: ImportOrigin::Expose,
            shape: if value.is_import_all {
                ImportShape::Namespace
            } else {
                ImportShape::Membership
            },
            recursive: value.is_recursive,
        }),
    }
}

/// Builds declared feature properties for a part usage.
pub fn part_usage_feature_properties(
    usage: &sysml_v2_parser::ast::PartUsage,
) -> DeclaredFeatureProperties {
    let (is_abstract, is_variation) = definition_prefix_flags(usage.usage_prefix.as_ref());
    let (is_composite, is_reference) = usage_ownership_from_ref_flag(usage.is_reference);
    DeclaredFeatureProperties {
        direction: usage.direction.map(direction_name).map(str::to_owned),
        is_abstract,
        is_variation,
        is_individual: usage.is_individual,
        is_derived: usage.is_derived,
        is_constant: usage.is_constant,
        is_end: false,
        is_composite,
        is_reference,
        is_conjugated: false,
        is_ordered: Some(usage.ordered),
        is_unique: None,
        is_portion: false,
        portion_kind: None,
    }
}

/// Builds declared feature properties for an attribute usage.
pub fn attribute_usage_feature_properties(
    usage: &sysml_v2_parser::ast::AttributeUsage,
) -> DeclaredFeatureProperties {
    DeclaredFeatureProperties {
        direction: usage.direction.map(direction_name).map(str::to_owned),
        is_abstract: false,
        is_variation: false,
        is_individual: false,
        is_derived: usage.is_derived,
        is_constant: usage.is_constant,
        is_end: usage.is_end,
        is_composite: None,
        is_reference: None,
        is_conjugated: false,
        is_ordered: Some(usage.ordered),
        is_unique: Some(!usage.nonunique),
        is_portion: false,
        portion_kind: None,
    }
}

/// Builds declared feature properties for a port usage.
pub fn port_usage_feature_properties(
    usage: &sysml_v2_parser::ast::PortUsage,
) -> DeclaredFeatureProperties {
    let conjugated = usage
        .type_name
        .as_deref()
        .is_some_and(|type_name| type_name.trim_start().starts_with('~'));
    DeclaredFeatureProperties {
        direction: usage.direction.map(direction_name).map(str::to_owned),
        is_abstract: usage.is_abstract,
        is_variation: false,
        is_individual: false,
        is_derived: usage.is_derived,
        is_constant: usage.is_constant,
        is_end: false,
        is_composite: None,
        is_reference: None,
        is_conjugated: conjugated,
        is_ordered: None,
        is_unique: None,
        is_portion: false,
        portion_kind: None,
    }
}

/// Builds declared feature properties for an action usage (including `ref action`).
pub fn action_usage_feature_properties(
    usage: &sysml_v2_parser::ast::ActionUsage,
) -> DeclaredFeatureProperties {
    let (is_composite, is_reference) = usage_ownership_from_ref_flag(usage.is_reference);
    DeclaredFeatureProperties {
        direction: None,
        is_abstract: usage.is_abstract,
        is_variation: false,
        is_individual: false,
        is_derived: false,
        is_constant: false,
        is_end: false,
        is_composite,
        is_reference,
        is_conjugated: false,
        is_ordered: None,
        is_unique: None,
        is_portion: false,
        portion_kind: None,
    }
}

/// Builds declared feature properties for a state usage (including `ref state`).
pub fn state_usage_feature_properties(
    usage: &sysml_v2_parser::ast::StateUsage,
) -> DeclaredFeatureProperties {
    let (is_composite, is_reference) = usage_ownership_from_ref_flag(usage.is_reference);
    DeclaredFeatureProperties {
        direction: None,
        is_abstract: usage.is_abstract,
        is_variation: false,
        is_individual: false,
        is_derived: false,
        is_constant: false,
        is_end: false,
        is_composite,
        is_reference,
        is_conjugated: false,
        is_ordered: None,
        is_unique: None,
        is_portion: false,
        portion_kind: None,
    }
}

/// Builds declared feature properties for an item usage.
pub fn item_usage_feature_properties(
    usage: &sysml_v2_parser::ast::ItemUsage,
) -> DeclaredFeatureProperties {
    DeclaredFeatureProperties {
        direction: usage.direction.map(direction_name).map(str::to_owned),
        is_abstract: false,
        is_variation: false,
        is_individual: false,
        is_derived: false,
        is_constant: false,
        is_end: false,
        is_composite: None,
        is_reference: None,
        is_conjugated: false,
        is_ordered: None,
        is_unique: None,
        is_portion: false,
        portion_kind: None,
    }
}

/// Builds declared feature properties for an occurrence usage.
pub fn occurrence_usage_feature_properties(
    usage: &sysml_v2_parser::ast::OccurrenceUsage,
) -> DeclaredFeatureProperties {
    let (is_composite, is_reference) = usage_ownership_from_ref_flag(usage.is_reference);
    DeclaredFeatureProperties {
        direction: None,
        is_abstract: usage.is_abstract,
        is_variation: false,
        is_individual: usage.is_individual,
        is_derived: false,
        is_constant: usage.is_constant,
        is_end: false,
        is_composite,
        is_reference,
        is_conjugated: false,
        is_ordered: None,
        is_unique: None,
        is_portion: usage.portion_kind.is_some(),
        portion_kind: usage.portion_kind.clone(),
    }
}

/// Builds declared properties for a `ref` declaration (`RefDecl`).
pub fn ref_decl_feature_properties() -> DeclaredFeatureProperties {
    DeclaredFeatureProperties {
        is_composite: Some(false),
        is_reference: Some(true),
        ..DeclaredFeatureProperties::default()
    }
}

/// Builds declared properties for a part/item-style definition with prefix and individual flags.
pub fn definition_feature_properties(
    prefix: Option<&DefinitionPrefix>,
    is_individual: bool,
) -> DeclaredFeatureProperties {
    let (is_abstract, is_variation) = definition_prefix_flags(prefix);
    DeclaredFeatureProperties {
        direction: None,
        is_abstract,
        is_variation,
        is_individual,
        is_derived: false,
        is_constant: false,
        is_end: false,
        is_composite: None,
        is_reference: None,
        is_conjugated: false,
        is_ordered: None,
        is_unique: None,
        is_portion: false,
        portion_kind: None,
    }
}

fn typing_relationship_target(relationship: &TypingRelationship) -> Option<&str> {
    relationship
        .target
        .first()
        .and_then(|target| target.value.local_name())
}

fn subsetting_relationship_target(relationship: &SubsettingRelationship) -> Option<&str> {
    relationship
        .target
        .first()
        .and_then(|target| target.value.local_name())
}

/// Returns the source-level target of a typed typing or specialization relationship.
/// Consumers use this adapter rather than treating parser relationship nodes as strings.
pub fn typing_target(relationship: Option<&TypingRelationship>) -> Option<&str> {
    relationship.and_then(typing_relationship_target)
}

/// Returns every source-level target of a typed typing or specialization relationship, not just
/// the first. SysML v2 allows a comma-separated multi-target clause (e.g. `item x : A, B;` is
/// equivalent to `item x defined by A defined by B;` -- each target is its own independent
/// `FeatureTyping`/`Subclassification` relationship). [`typing_target`] intentionally keeps its
/// single-target contract for callers that only ever see one target in practice; use this form
/// wherever every declared target must become its own edge.
pub fn typing_targets(relationship: Option<&TypingRelationship>) -> Vec<&str> {
    relationship.map_or_else(Vec::new, |relationship| {
        relationship
            .target
            .iter()
            .filter_map(|target| target.value.local_name())
            .collect()
    })
}

/// Parser-owned typing/specialization targets with exact target spans.
pub fn declared_typing_targets(
    relationship: Option<&TypingRelationship>,
) -> Vec<DeclaredRelationshipTarget> {
    relationship.map_or_else(Vec::new, |relationship| {
        relationship
            .target
            .iter()
            .map(|target| DeclaredRelationshipTarget {
                reference: target.value.to_display_string(),
                range: Some(span_to_range(&target.span)),
            })
            .collect()
    })
}

/// Returns the complete source-level feature chain of a typed typing or
/// specialization relationship. Resolution and dependency-closure consumers
/// must use this form: reducing `OtherPkg::Base` to `Base` loses the package
/// that owns the referenced definition.
pub fn typing_target_display(relationship: Option<&TypingRelationship>) -> Option<String> {
    relationship.and_then(|relationship| {
        relationship
            .target
            .first()
            .map(|target| target.value.to_display_string())
    })
}

/// Returns the source-level target of a typed subsetting-family relationship.
pub fn subsetting_target(relationship: Option<&SubsettingRelationship>) -> Option<&str> {
    relationship.and_then(subsetting_relationship_target)
}

/// Returns every source-level target of a subsetting-family relationship (multi-target `:>`).
pub fn subsetting_targets(relationship: Option<&SubsettingRelationship>) -> Vec<&str> {
    relationship.map_or_else(Vec::new, |relationship| {
        relationship
            .target
            .iter()
            .filter_map(|target| target.value.local_name())
            .collect()
    })
}

/// Parser-owned subsetting-family targets with exact target spans.
pub fn declared_subsetting_targets(
    relationship: Option<&SubsettingRelationship>,
) -> Vec<DeclaredRelationshipTarget> {
    relationship.map_or_else(Vec::new, |relationship| {
        relationship
            .target
            .iter()
            .map(|target| DeclaredRelationshipTarget {
                reference: target.value.to_display_string(),
                range: Some(span_to_range(&target.span)),
            })
            .collect()
    })
}

/// Returns the complete source-level feature chain of a subsetting-family
/// relationship. Keep this distinct from [`subsetting_target`], whose local
/// name remains useful for display and effective-name rules.
pub fn subsetting_target_display(relationship: Option<&SubsettingRelationship>) -> Option<String> {
    relationship.and_then(|relationship| {
        relationship
            .target
            .first()
            .map(|target| target.value.to_display_string())
    })
}

/// Returns the expression carried by a typed connection/interface endpoint.
pub fn connection_end_expression(endpoint: &Node<ConnectionEnd>) -> &Node<Expression> {
    &endpoint.value.expression
}

/// Returns the value expression of either a positional or named invocation argument.
pub fn argument_expression(argument: &Argument) -> &Node<Expression> {
    &argument.value
}

/// Splits a fold step's already-built children back into plain sub-expressions (`children`) and
/// named/positional invocation-style arguments (`arguments`), preserving each group's relative
/// order -- the inverse of how [`crate::semantic::expression_fold::expression_children`] tagged
/// them going in.
fn split_children(
    children: Vec<FoldedChild<DeclaredExpression>>,
) -> (Vec<DeclaredExpression>, Vec<DeclaredExpressionArgument>) {
    let mut subs = Vec::new();
    let mut arguments = Vec::new();
    for child in children {
        match child {
            FoldedChild::Sub(value) => subs.push(value),
            FoldedChild::Argument { name, value } => {
                arguments.push(DeclaredExpressionArgument { name, value })
            }
        }
    }
    (subs, arguments)
}

struct DeclaredExpressionAlgebra;

impl ExpressionAlgebra for DeclaredExpressionAlgebra {
    type Output = DeclaredExpression;

    /// Mirrors the pre-0.47.0 recursive `declared_expression` exactly, one match arm per variant,
    /// with each `declared_expression(child)` recursive call replaced by that child's
    /// already-folded result from `subs`/`arguments` -- see the module doc on
    /// [`crate::semantic::expression_fold`] for why.
    fn build(
        &mut self,
        node: &Node<Expression>,
        children: Vec<FoldedChild<DeclaredExpression>>,
    ) -> DeclaredExpression {
        use sysml_v2_parser::ast::Expression as Expr;
        let (subs, arguments) = split_children(children);
        let mut expression = DeclaredExpression {
            kind: DeclaredExpressionKind::Null,
            range: span_to_range(&node.span),
            literal: None,
            reference: None,
            operator: None,
            children: Vec::new(),
            arguments: Vec::new(),
        };
        match &node.value {
            Expr::LiteralInteger(value) => {
                expression.kind = DeclaredExpressionKind::IntegerLiteral;
                expression.literal = Some(serde_json::json!(value));
            }
            Expr::LiteralReal(value) => {
                expression.kind = DeclaredExpressionKind::RealLiteral;
                expression.literal = Some(serde_json::json!(value));
            }
            Expr::LiteralString(value) => {
                expression.kind = DeclaredExpressionKind::StringLiteral;
                expression.literal = Some(serde_json::json!(value));
            }
            Expr::LiteralBoolean(value) => {
                expression.kind = DeclaredExpressionKind::BooleanLiteral;
                expression.literal = Some(serde_json::json!(value));
            }
            Expr::Null => expression.kind = DeclaredExpressionKind::Null,
            Expr::FeatureRef(value) => {
                expression.kind = DeclaredExpressionKind::FeatureReference;
                expression.reference = Some(value.clone());
            }
            Expr::FeatureChainRef(value) => {
                expression.kind = DeclaredExpressionKind::FeatureChain;
                expression.reference = Some(value.segments.join("."));
            }
            Expr::Classification { metaclass } => {
                expression.kind = DeclaredExpressionKind::Classification;
                expression.reference = Some(metaclass.clone());
            }
            Expr::MemberAccess(_, member) => {
                expression.kind = DeclaredExpressionKind::MemberAccess;
                expression.reference = Some(member.clone());
                expression.children = subs;
            }
            Expr::Select { selector, .. } => {
                expression.kind = DeclaredExpressionKind::Select;
                expression.reference = Some(selector.clone());
                expression.children = subs;
            }
            Expr::Collect { selector, .. } => {
                expression.kind = DeclaredExpressionKind::Collect;
                expression.reference = Some(selector.clone());
                expression.children = subs;
            }
            Expr::MetadataAccess(_) => {
                expression.kind = DeclaredExpressionKind::MetadataAccess;
                expression.children = subs;
            }
            Expr::Parenthesized(_) => {
                expression.kind = DeclaredExpressionKind::Parenthesized;
                expression.children = subs;
            }
            Expr::Bracket(_) => {
                expression.kind = DeclaredExpressionKind::Bracket;
                expression.children = subs;
            }
            Expr::UnaryOp { op, .. } => {
                expression.kind = DeclaredExpressionKind::Unary;
                expression.operator = Some(op.as_str().into());
                expression.children = subs;
            }
            Expr::BinaryOp { op, .. } => {
                expression.kind = DeclaredExpressionKind::Binary;
                expression.operator = Some(op.as_str().into());
                expression.children = subs;
            }
            Expr::Index { .. } => {
                expression.kind = DeclaredExpressionKind::Index;
                expression.children = subs;
            }
            Expr::LiteralWithUnit { .. } => {
                expression.kind = DeclaredExpressionKind::LiteralWithUnit;
                expression.children = subs;
            }
            Expr::Tuple(_) => {
                expression.kind = DeclaredExpressionKind::Tuple;
                expression.children = subs;
            }
            Expr::Invocation { .. } => {
                expression.kind = DeclaredExpressionKind::Invocation;
                expression.children = subs;
                expression.arguments = arguments;
            }
            Expr::Constructor { type_name, .. } => {
                expression.kind = DeclaredExpressionKind::Constructor;
                expression.reference = Some(type_name.clone());
                expression.arguments = arguments;
            }
            Expr::CollectionOp { op, .. } => {
                expression.kind = DeclaredExpressionKind::CollectionOperation;
                expression.operator = Some(op.as_str().into());
                expression.children = subs;
                expression.arguments = arguments;
            }
            Expr::MetaCast { metaclass, .. } => {
                expression.kind = DeclaredExpressionKind::MetaCast;
                expression.reference = Some(metaclass.clone());
                expression.children = subs;
            }
            Expr::TypeCheck {
                kind, type_name, ..
            } => {
                expression.kind = DeclaredExpressionKind::TypeCheck;
                expression.operator = Some(
                    match kind {
                        sysml_v2_parser::ast::TypeCheckKind::Istype => "istype",
                        sysml_v2_parser::ast::TypeCheckKind::Hastype => "hastype",
                        sysml_v2_parser::ast::TypeCheckKind::As => "as",
                    }
                    .into(),
                );
                expression.reference = Some(type_name.clone());
                expression.children = subs;
            }
            Expr::Conditional { .. } => {
                expression.kind = DeclaredExpressionKind::Conditional;
                expression.children = subs;
            }
            Expr::Extent { target } => {
                expression.kind = DeclaredExpressionKind::Extent;
                expression.reference = Some(target.clone());
            }
        }
        expression
    }
}

/// Normalize the parser expression AST into typed semantic facts. This never
/// uses the debug renderer; structural children and named arguments remain
/// explicit for later addressable projection.
///
/// Iterative, not recursive: see the `semantic::expression_fold` module doc for why.
pub fn declared_expression(node: &Node<Expression>) -> DeclaredExpression {
    fold_expression(node, &mut DeclaredExpressionAlgebra)
}

pub fn declared_multiplicity(
    node: &Node<sysml_v2_parser::ast::Multiplicity>,
    ordered: bool,
) -> DeclaredMultiplicity {
    DeclaredMultiplicity {
        lower: node.value.lower.as_deref().map(declared_expression),
        upper: node.value.upper.as_deref().map(declared_expression),
        range: span_to_range(&node.span),
        is_implied: false,
        is_ordered: ordered,
        is_unique: None,
    }
}

/// Normalizes the parser's typed `FeatureValue` without conflating its
/// operator with display text. `=` binds a value, `:=` establishes an initial
/// value, and `default` keeps its distinct default-value semantics.
pub fn declared_feature_value(
    node: &Node<sysml_v2_parser::ast::FeatureValue>,
) -> DeclaredFeatureValue {
    use sysml_v2_parser::ast::FeatureValueKind;

    let kind = if node.value.is_default {
        DeclaredFeatureValueKind::Default
    } else {
        match node.value.kind {
            FeatureValueKind::Bind => DeclaredFeatureValueKind::Bound,
            FeatureValueKind::Assign => DeclaredFeatureValueKind::Initial,
        }
    };
    DeclaredFeatureValue {
        kind,
        expression: declared_expression(&node.value.expression),
        range: span_to_range(&node.value.span),
    }
}

/// Converts sysml-v2-parser Span (1-based line/column) to 0-based TextRange.
pub fn span_to_range(span: &Span) -> TextRange {
    let (start_line, start_char, end_line, end_char) = span.to_lsp_range();
    TextRange::new(
        TextPosition::new(start_line, start_char),
        TextPosition::new(end_line, end_char),
    )
}

pub fn text_range_to_json(range: TextRange) -> serde_json::Value {
    serde_json::json!({
        "start": {
            "line": range.start.line,
            "character": range.start.character,
        },
        "end": {
            "line": range.end.line,
            "character": range.end.character,
        },
    })
}

/// Returns the display name from Identification (name, or short_name, or empty string).
pub fn identification_name(ident: &Identification) -> String {
    ident
        .name
        .as_deref()
        .or(ident.short_name.as_deref())
        .unwrap_or("")
        .to_string()
}

/// Stashes `identification.short_name` as a `"shortName"` attribute when both a short name
/// and a regular name are present. When short_name is the *only* name, `identification_name`
/// already uses it as `SemanticNode.name`, so there's nothing extra to capture — without this,
/// a short name declared alongside a regular name (e.g. `part def <'CB'> ControlBoard;`) was
/// silently dropped: nothing outside the raw source text ever knew `CB` refers to
/// `ControlBoard`, so references to `CB` failed to resolve entirely.
pub fn attach_short_name_attribute(
    attrs: &mut HashMap<String, serde_json::Value>,
    identification: &Identification,
) {
    if identification.name.is_none() {
        return;
    }
    if let Some(short) = identification
        .short_name
        .as_deref()
        .filter(|s| !s.is_empty())
    {
        attrs.insert("shortName".to_string(), serde_json::json!(short));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identification(name: Option<&str>, short_name: Option<&str>) -> Identification {
        Identification {
            name: name.map(str::to_string),
            short_name: short_name.map(str::to_string),
        }
    }

    #[test]
    fn attaches_short_name_when_both_name_and_short_name_present() {
        let ident = identification(Some("ControlBoard"), Some("CB"));
        let mut attrs = HashMap::new();
        attach_short_name_attribute(&mut attrs, &ident);
        assert_eq!(attrs.get("shortName").and_then(|v| v.as_str()), Some("CB"));
    }

    #[test]
    fn does_not_attach_short_name_when_only_short_name_present() {
        // identification_name already uses short_name as the node's primary name in this case,
        // so there is nothing extra to capture.
        let ident = identification(None, Some("CB"));
        let mut attrs = HashMap::new();
        attach_short_name_attribute(&mut attrs, &ident);
        assert!(!attrs.contains_key("shortName"));
    }

    #[test]
    fn does_not_attach_short_name_when_absent() {
        let ident = identification(Some("ControlBoard"), None);
        let mut attrs = HashMap::new();
        attach_short_name_attribute(&mut attrs, &ident);
        assert!(!attrs.contains_key("shortName"));
    }

    #[test]
    fn declared_expression_handles_deeply_nested_parentheses_without_overflowing_the_stack() {
        use sysml_v2_parser::ast::{Node, Span};

        const DEPTH: usize = 200_000;
        let mut tree = Node::new(Span::dummy(), Expression::LiteralInteger(1));
        for _ in 0..DEPTH {
            tree = Node::new(Span::dummy(), Expression::Parenthesized(Box::new(tree)));
        }

        let declared = declared_expression(&tree);
        let mut depth = 0usize;
        let mut current = &declared;
        loop {
            match current.kind {
                DeclaredExpressionKind::Parenthesized => {
                    depth += 1;
                    current = &current.children[0];
                }
                DeclaredExpressionKind::IntegerLiteral => break,
                other => panic!("unexpected kind at depth {depth}: {other:?}"),
            }
        }
        assert_eq!(depth, DEPTH);
    }
}
