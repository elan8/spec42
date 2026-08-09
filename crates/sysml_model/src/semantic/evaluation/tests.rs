use std::collections::HashMap;

use super::*;
use crate::semantic::model::{
    DeclaredExpression, DeclaredExpressionKind, DeclaredFeatureValue, DeclaredFeatureValueKind,
    DeclaredSemanticFacts, ElementKind, SemanticNode,
};
use crate::semantic::text_span::{TextPosition, TextRange};
use url::Url;

fn range() -> TextRange {
    TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 1))
}

fn expression(kind: DeclaredExpressionKind) -> DeclaredExpression {
    DeclaredExpression {
        kind,
        range: range(),
        literal: None,
        reference: None,
        operator: None,
        children: Vec::new(),
        arguments: Vec::new(),
    }
}

fn integer(value: i64) -> DeclaredExpression {
    let mut expression = expression(DeclaredExpressionKind::IntegerLiteral);
    expression.literal = Some(serde_json::json!(value));
    expression
}

fn reference(name: &str) -> DeclaredExpression {
    let mut expression = expression(DeclaredExpressionKind::FeatureReference);
    expression.reference = Some(name.to_string());
    expression
}

fn binary(
    operator: &str,
    left: DeclaredExpression,
    right: DeclaredExpression,
) -> DeclaredExpression {
    let mut expression = expression(DeclaredExpressionKind::Binary);
    expression.operator = Some(operator.to_string());
    expression.children = vec![left, right];
    expression
}

fn add_feature(
    graph: &mut SemanticGraph,
    uri: &Url,
    name: &str,
    value: DeclaredExpression,
) -> NodeId {
    add_feature_with_identity(graph, uri, name, name, value)
}

fn add_feature_with_identity(
    graph: &mut SemanticGraph,
    uri: &Url,
    qualified_name: &str,
    name: &str,
    value: DeclaredExpression,
) -> NodeId {
    let id = NodeId::new(uri, qualified_name);
    let node = SemanticNode {
        id: id.clone(),
        element_kind: ElementKind::Attribute,
        declared_name: Some(name.to_string()),
        name: name.to_string(),
        range: range(),
        // Deliberately conflicting projection data: the evaluator must ignore it.
        attributes: HashMap::from([("value".to_string(), serde_json::json!(99))]),
        declared_facts: DeclaredSemanticFacts {
            feature_value: Some(DeclaredFeatureValue {
                kind: DeclaredFeatureValueKind::Bound,
                expression: value,
                range: range(),
            }),
            ..Default::default()
        },
        parent_id: None,
    };
    let index = graph.graph.add_node(node);
    graph.node_index_by_id.insert(id.clone(), index);
    graph
        .nodes_by_uri
        .entry(uri.clone())
        .or_default()
        .push(id.clone());
    graph
        .node_ids_by_qualified_name
        .entry(qualified_name.to_string())
        .or_default()
        .push(id.clone());
    id
}

fn status<'a>(graph: &'a SemanticGraph, node: &NodeId) -> Option<&'a str> {
    graph
        .get_node(node)?
        .attributes
        .get(EVALUATION_STATUS_KEY)?
        .as_str()
}

#[test]
fn evaluates_declared_facts_not_projection_attributes() {
    let uri = Url::parse("file:///typed.sysml").expect("uri");
    let mut graph = SemanticGraph::new();
    let _a = add_feature(&mut graph, &uri, "a", integer(2));
    let b = add_feature(
        &mut graph,
        &uri,
        "b",
        binary("+", reference("a"), integer(3)),
    );

    evaluate_expressions(&mut graph);

    assert_eq!(status(&graph, &b), Some(STATUS_OK));
    assert_eq!(
        graph
            .get_node(&b)
            .and_then(|node| node.attributes.get(EVALUATED_VALUE_KEY)),
        Some(&serde_json::json!(5))
    );
}

#[test]
fn preserves_unresolved_cycle_and_malformed_states() {
    let uri = Url::parse("file:///states.sysml").expect("uri");
    let mut graph = SemanticGraph::new();
    let unresolved = add_feature(&mut graph, &uri, "unresolved", reference("missing"));
    let cycle_a = add_feature(&mut graph, &uri, "cycle_a", reference("cycle_b"));
    let _cycle_b = add_feature(&mut graph, &uri, "cycle_b", reference("cycle_a"));
    let malformed = add_feature(
        &mut graph,
        &uri,
        "malformed",
        expression(DeclaredExpressionKind::Binary),
    );

    evaluate_expressions(&mut graph);

    assert_eq!(status(&graph, &unresolved), Some(STATUS_UNRESOLVED));
    assert_eq!(status(&graph, &cycle_a), Some(STATUS_CYCLE));
    assert_eq!(status(&graph, &malformed), Some(STATUS_MALFORMED));
    for node in [&unresolved, &cycle_a, &malformed] {
        assert!(graph
            .get_node(node)
            .and_then(|node| node.attributes.get(EVALUATED_VALUE_KEY))
            .is_none());
    }
}

#[test]
fn preserves_ambiguous_and_unsupported_states() {
    let uri = Url::parse("file:///ambiguous.sysml").expect("uri");
    let mut graph = SemanticGraph::new();
    let _west = add_feature_with_identity(&mut graph, &uri, "west", "target", integer(1));
    let _east = add_feature_with_identity(&mut graph, &uri, "east", "target", integer(2));
    let ambiguous = add_feature(&mut graph, &uri, "ambiguous", reference("target"));
    let unsupported = add_feature(
        &mut graph,
        &uri,
        "unsupported",
        expression(DeclaredExpressionKind::Conditional),
    );

    evaluate_expressions(&mut graph);

    assert_eq!(status(&graph, &ambiguous), Some(STATUS_AMBIGUOUS));
    assert_eq!(status(&graph, &unsupported), Some(STATUS_UNSUPPORTED));
    for node in [&ambiguous, &unsupported] {
        assert!(graph
            .get_node(node)
            .and_then(|node| node.attributes.get(EVALUATED_VALUE_KEY))
            .is_none());
    }
}

#[test]
fn typed_engine_cannot_reintroduce_projection_or_text_parser_evaluation() {
    let source = include_str!("engine.rs");
    assert!(
        !source.contains(".attributes"),
        "the typed engine must not consume projection fields"
    );
    for legacy_entrypoint in [
        "QuantityParser",
        "AnalysisExprParser",
        "evaluate_expression_text",
        "evaluate_quantity_expression",
    ] {
        assert!(
            !source.contains(legacy_entrypoint),
            "the typed engine must not reintroduce {legacy_entrypoint}"
        );
    }
}
