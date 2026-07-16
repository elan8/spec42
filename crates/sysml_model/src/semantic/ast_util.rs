//! Helpers for working with sysml-v2-parser AST: span/range conversion and name extraction.

use std::collections::HashMap;

use crate::semantic::model::{
    DeclaredExpression, DeclaredExpressionArgument, DeclaredFeatureProperties, DeclaredFeatureValue,
    DeclaredFeatureValueKind, DeclaredMultiplicity,
};
use crate::semantic::text_span::{TextPosition, TextRange};
use sysml_v2_parser::ast::{
    Argument, ConnectionEnd, DefinitionPrefix, Identification, InOut, Node,
    SubsettingRelationship, TypingRelationship,
};
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

/// Builds declared feature properties for a part usage.
pub fn part_usage_feature_properties(
    usage: &sysml_v2_parser::ast::PartUsage,
) -> DeclaredFeatureProperties {
    let (is_abstract, is_variation) = definition_prefix_flags(usage.usage_prefix.as_ref());
    DeclaredFeatureProperties {
        direction: usage.direction.map(direction_name).map(str::to_owned),
        is_abstract,
        is_variation,
        is_individual: usage.is_individual,
        is_derived: usage.is_derived,
        is_constant: usage.is_constant,
        is_end: false,
        is_ordered: Some(usage.ordered),
        is_unique: None,
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
        is_ordered: Some(usage.ordered),
        is_unique: Some(!usage.nonunique),
    }
}

/// Builds declared feature properties for a port usage.
pub fn port_usage_feature_properties(
    usage: &sysml_v2_parser::ast::PortUsage,
) -> DeclaredFeatureProperties {
    DeclaredFeatureProperties {
        direction: usage.direction.map(direction_name).map(str::to_owned),
        is_abstract: false,
        is_variation: false,
        is_individual: false,
        is_derived: usage.is_derived,
        is_constant: usage.is_constant,
        is_end: false,
        is_ordered: None,
        is_unique: None,
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
        is_ordered: None,
        is_unique: None,
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
        is_ordered: None,
        is_unique: None,
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

/// Returns the complete source-level feature chain of a subsetting-family
/// relationship. Keep this distinct from [`subsetting_target`], whose local
/// name remains useful for display and effective-name rules.
pub fn subsetting_target_display(
    relationship: Option<&SubsettingRelationship>,
) -> Option<String> {
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

/// Normalize the parser expression AST into typed semantic facts. This never
/// uses the debug renderer; structural children and named arguments remain
/// explicit for later addressable projection.
pub fn declared_expression(node: &Node<Expression>) -> DeclaredExpression {
    use sysml_v2_parser::ast::Expression as Expr;
    let mut expression = DeclaredExpression {
        kind: String::new(),
        range: span_to_range(&node.span),
        literal: None,
        reference: None,
        operator: None,
        children: Vec::new(),
        arguments: Vec::new(),
    };
    match &node.value {
        Expr::LiteralInteger(value) => {
            expression.kind = "integerLiteral".into();
            expression.literal = Some(serde_json::json!(value));
        }
        Expr::LiteralReal(value) => {
            expression.kind = "realLiteral".into();
            expression.literal = Some(serde_json::json!(value));
        }
        Expr::LiteralString(value) => {
            expression.kind = "stringLiteral".into();
            expression.literal = Some(serde_json::json!(value));
        }
        Expr::LiteralBoolean(value) => {
            expression.kind = "booleanLiteral".into();
            expression.literal = Some(serde_json::json!(value));
        }
        Expr::Null => expression.kind = "null".into(),
        Expr::FeatureRef(value) => {
            expression.kind = "featureReference".into();
            expression.reference = Some(value.clone());
        }
        Expr::FeatureChainRef(value) => {
            expression.kind = "featureChain".into();
            expression.reference = Some(value.segments.join("."));
        }
        Expr::Classification { metaclass } => {
            expression.kind = "classification".into();
            expression.reference = Some(metaclass.clone());
        }
        Expr::MemberAccess(base, member) => {
            expression.kind = "memberAccess".into();
            expression.reference = Some(member.clone());
            expression.children.push(declared_expression(base));
        }
        Expr::Select { base, selector } => {
            expression.kind = "select".into();
            expression.reference = Some(selector.clone());
            expression.children.push(declared_expression(base));
        }
        Expr::Collect { base, selector } => {
            expression.kind = "collect".into();
            expression.reference = Some(selector.clone());
            expression.children.push(declared_expression(base));
        }
        Expr::MetadataAccess(base) => {
            expression.kind = "metadataAccess".into();
            expression.children.push(declared_expression(base));
        }
        Expr::Parenthesized(inner) => {
            expression.kind = "parenthesized".into();
            expression.children.push(declared_expression(inner));
        }
        Expr::Bracket(inner) => {
            expression.kind = "bracket".into();
            expression.children.push(declared_expression(inner));
        }
        Expr::UnaryOp { op, operand } => {
            expression.kind = "unary".into();
            expression.operator = Some(op.as_str().into());
            expression.children.push(declared_expression(operand));
        }
        Expr::BinaryOp { op, left, right } => {
            expression.kind = "binary".into();
            expression.operator = Some(op.as_str().into());
            expression.children = vec![declared_expression(left), declared_expression(right)];
        }
        Expr::Index { base, index } => {
            expression.kind = "index".into();
            expression.children = vec![declared_expression(base), declared_expression(index)];
        }
        Expr::LiteralWithUnit { value, unit } => {
            expression.kind = "literalWithUnit".into();
            expression.children = vec![declared_expression(value), declared_expression(unit)];
        }
        Expr::Tuple(values) => {
            expression.kind = "tuple".into();
            expression.children = values.iter().map(declared_expression).collect();
        }
        Expr::Invocation { callee, args } => {
            expression.kind = "invocation".into();
            expression.children.push(declared_expression(callee));
            expression.arguments = args
                .iter()
                .map(|arg| DeclaredExpressionArgument {
                    name: arg.name.clone(),
                    value: declared_expression(&arg.value),
                })
                .collect();
        }
        Expr::Constructor { type_name, args } => {
            expression.kind = "constructor".into();
            expression.reference = Some(type_name.clone());
            expression.arguments = args
                .iter()
                .map(|arg| DeclaredExpressionArgument {
                    name: arg.name.clone(),
                    value: declared_expression(&arg.value),
                })
                .collect();
        }
        Expr::CollectionOp { op, base, args } => {
            expression.kind = "collectionOperation".into();
            expression.operator = Some(op.as_str().into());
            expression.children.push(declared_expression(base));
            expression.arguments = args
                .iter()
                .map(|arg| DeclaredExpressionArgument {
                    name: arg.name.clone(),
                    value: declared_expression(&arg.value),
                })
                .collect();
        }
        Expr::MetaCast { base, metaclass } => {
            expression.kind = "metaCast".into();
            expression.reference = Some(metaclass.clone());
            expression.children.push(declared_expression(base));
        }
        Expr::TypeCheck {
            kind,
            operand,
            type_name,
        } => {
            expression.kind = "typeCheck".into();
            expression.operator = Some(
                match kind {
                    sysml_v2_parser::ast::TypeCheckKind::Istype => "istype",
                    sysml_v2_parser::ast::TypeCheckKind::Hastype => "hastype",
                    sysml_v2_parser::ast::TypeCheckKind::As => "as",
                }
                .into(),
            );
            expression.reference = Some(type_name.clone());
            if let Some(value) = operand {
                expression.children.push(declared_expression(value));
            }
        }
    }
    expression
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
}
