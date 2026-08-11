use std::collections::HashMap;

use super::*;
use crate::semantic::model::{
    DeclaredBinaryOperator, DeclaredExpression, DeclaredExpressionKind, DeclaredExpressionOperator,
    DeclaredFeatureValue, DeclaredFeatureValueKind, DeclaredLiteral, DeclaredSemanticFacts,
    DeclaredUnaryOperator, ElementKind, EvaluatedValue, EvaluationStatus, SemanticNode,
};
use crate::semantic::pipeline::{build_and_link_graph, patch_graph_for_document};
use crate::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};
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
    expression.literal = Some(DeclaredLiteral::Integer(value));
    expression
}

fn reference(name: &str) -> DeclaredExpression {
    let mut expression = expression(DeclaredExpressionKind::FeatureReference);
    expression.reference = Some(name.to_string());
    expression
}

fn binary(
    operator: DeclaredBinaryOperator,
    left: DeclaredExpression,
    right: DeclaredExpression,
) -> DeclaredExpression {
    let mut expression = expression(DeclaredExpressionKind::Binary);
    expression.operator = Some(DeclaredExpressionOperator::Binary(operator));
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
        source_text: Default::default(),
        expression_text: Default::default(),
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

fn status(graph: &SemanticGraph, node: &NodeId) -> Option<EvaluationStatus> {
    graph
        .evaluation_facts_for(graph.get_node(node)?)?
        .expression
        .as_ref()
        .map(|outcome| outcome.status)
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
        binary(DeclaredBinaryOperator::Add, reference("a"), integer(3)),
    );

    evaluate_expressions(&mut graph);

    assert_eq!(status(&graph, &b), Some(EvaluationStatus::Ok));
    assert_eq!(
        graph
            .get_node(&b)
            .and_then(|node| graph.evaluation_facts_for(node))
            .and_then(|facts| facts.expression.as_ref())
            .and_then(|outcome| outcome.value.as_ref()),
        Some(&EvaluatedValue::Integer(5))
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

    assert_eq!(
        status(&graph, &unresolved),
        Some(EvaluationStatus::Unresolved)
    );
    assert_eq!(status(&graph, &cycle_a), Some(EvaluationStatus::Cycle));
    assert_eq!(
        status(&graph, &malformed),
        Some(EvaluationStatus::Malformed)
    );
    for node in [&unresolved, &cycle_a, &malformed] {
        assert!(graph
            .get_node(node)
            .and_then(|node| graph.evaluation_facts_for(node))
            .and_then(|facts| facts.expression.as_ref())
            .and_then(|outcome| outcome.value.as_ref())
            .is_none());
    }
}

#[test]
fn rejects_declared_kind_literal_and_operator_family_mismatches() {
    let uri = Url::parse("file:///typed-expression-mismatches.sysml").expect("uri");
    let mut graph = SemanticGraph::new();

    let mut literal_mismatch = expression(DeclaredExpressionKind::IntegerLiteral);
    literal_mismatch.literal = Some(DeclaredLiteral::Boolean(true));
    let literal_mismatch = add_feature(&mut graph, &uri, "literal_mismatch", literal_mismatch);

    let mut unary_operator_mismatch = expression(DeclaredExpressionKind::Unary);
    unary_operator_mismatch.operator = Some(DeclaredExpressionOperator::Binary(
        DeclaredBinaryOperator::Add,
    ));
    unary_operator_mismatch.children = vec![integer(1)];
    let unary_operator_mismatch = add_feature(
        &mut graph,
        &uri,
        "unary_operator_mismatch",
        unary_operator_mismatch,
    );

    let mut binary_operator_mismatch = expression(DeclaredExpressionKind::Binary);
    binary_operator_mismatch.operator = Some(DeclaredExpressionOperator::Unary(
        DeclaredUnaryOperator::Minus,
    ));
    binary_operator_mismatch.children = vec![integer(1), integer(2)];
    let binary_operator_mismatch = add_feature(
        &mut graph,
        &uri,
        "binary_operator_mismatch",
        binary_operator_mismatch,
    );

    evaluate_expressions(&mut graph);

    for node in [
        &literal_mismatch,
        &unary_operator_mismatch,
        &binary_operator_mismatch,
    ] {
        assert_eq!(status(&graph, node), Some(EvaluationStatus::Malformed));
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

    assert_eq!(
        status(&graph, &ambiguous),
        Some(EvaluationStatus::Ambiguous)
    );
    assert_eq!(
        status(&graph, &unsupported),
        Some(EvaluationStatus::Unsupported)
    );
    for node in [&ambiguous, &unsupported] {
        assert!(graph
            .get_node(node)
            .and_then(|node| graph.evaluation_facts_for(node))
            .and_then(|facts| facts.expression.as_ref())
            .and_then(|outcome| outcome.value.as_ref())
            .is_none());
    }
}

#[test]
fn publishes_analysis_for_a_constraint_usage_with_an_own_expression() {
    let uri = Url::parse("file:///constraint-usage.sysml").expect("uri");
    let mut graph = SemanticGraph::new();
    let id = NodeId::new(&uri, "P::check");
    let node = SemanticNode {
        id: id.clone(),
        element_kind: ElementKind::Constraint,
        declared_name: Some("check".to_string()),
        name: "check".to_string(),
        range: range(),
        attributes: HashMap::new(),
        declared_facts: DeclaredSemanticFacts {
            own_expression: Some(reference("missing")),
            ..Default::default()
        },
        source_text: Default::default(),
        expression_text: Default::default(),
        parent_id: None,
    };
    let index = graph.graph.add_node(node);
    graph.node_index_by_id.insert(id.clone(), index);
    graph.nodes_by_uri.entry(uri).or_default().push(id.clone());
    graph
        .node_ids_by_qualified_name
        .entry("P::check".to_string())
        .or_default()
        .push(id.clone());

    evaluate_expressions(&mut graph);

    assert_eq!(status(&graph, &id), Some(EvaluationStatus::Unresolved));
    assert_eq!(
        graph
            .get_node(&id)
            .and_then(|node| graph.evaluation_facts_for(node))
            .and_then(|facts| facts.analysis.as_ref())
            .map(|analysis| analysis.expression.status),
        Some(EvaluationStatus::Unresolved)
    );
}

#[test]
fn typed_engine_cannot_reintroduce_projection_or_text_parser_evaluation() {
    let source = include_str!("engine.rs");
    let module = include_str!("mod.rs");
    assert!(
        !source.contains(".attributes"),
        "the typed engine must not consume projection fields"
    );
    assert!(
        !module.contains("UnitRegistry::from_graph"),
        "evaluation must not ingest unit data from projection attributes"
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

#[test]
fn production_consumers_cannot_reintroduce_retired_evaluation_attributes() {
    // Keep the semantic/host consumer closure on the canonical graph facts. Test fixtures may
    // mention historical keys while asserting migration behavior; production code may not.
    let production_sources = [
        include_str!("engine.rs"),
        include_str!("mod.rs"),
        include_str!("../../../../sysml_diagnostics/src/engine_impl.rs"),
        include_str!("../../../../generator_api/src/model.rs"),
        include_str!("../../../../language_service/src/presentation_hover.rs"),
        include_str!("../../../../lsp_server/src/lsp_runtime/symbols.rs"),
    ];
    for source in production_sources {
        for retired_key in [
            "evaluatedValue",
            "evaluatedUnit",
            "evaluationStatus",
            "evaluationError",
            "analysisEvaluationStatus",
            "analysisEvaluationValue",
            "analysisEvaluationError",
            "analysisConstraintPassed",
            "analysisComputedValue",
            "analysisComputedUnit",
        ] {
            assert!(
                !source.contains(retired_key),
                "production semantic consumers must read typed evaluation facts, not {retired_key}"
            );
        }
    }
}

#[test]
fn parser_backed_resolution_prefers_nearest_lexical_scope() {
    let uri = Url::parse("memory://typed-evaluation/shadowing.sysml").expect("uri");
    let document = SysmlDocument {
        uri: uri.clone(),
        content: r#"
            package Scope {
                attribute value = 1;
                part def Inner {
                    attribute value = 2;
                    attribute result = value + 1;
                }
            }
        "#
        .to_string(),
        path_hint: None,
        source_kind: SysmlDocumentSourceKind::Workspace,
        sha256: None,
        byte_size: None,
    };
    let (graph, _) = build_and_link_graph(&[document]).expect("graph");
    let result = graph
        .node_ids_by_qualified_name
        .get("Scope::Inner::result")
        .and_then(|ids| ids.first())
        .and_then(|id| graph.get_node(id))
        .expect("nested result");
    assert_eq!(status(&graph, &result.id), Some(EvaluationStatus::Ok));
    assert_eq!(
        graph
            .evaluation_facts_for(result)
            .and_then(|facts| facts.expression.as_ref())
            .and_then(|evaluation| evaluation.value.as_ref()),
        Some(&EvaluatedValue::Integer(3))
    );
}

#[test]
fn parser_backed_same_scope_cross_source_ambiguity_matches_incremental() {
    let docs = [("left.sysml", 1), ("right.sysml", 2)]
        .into_iter()
        .map(|(name, value)| {
            SysmlDocument::from_memory_path(
                "typed-evaluation-ambiguity",
                name,
                format!("package Scope {{ attribute value = {value}; }}"),
                SysmlDocumentSourceKind::Workspace,
                None,
                None,
            )
            .expect("document")
        })
        .chain(std::iter::once(
            SysmlDocument::from_memory_path(
                "typed-evaluation-ambiguity",
                "use.sysml",
                "package Scope { attribute result = value; }".to_string(),
                SysmlDocumentSourceKind::Workspace,
                None,
                None,
            )
            .expect("document"),
        ))
        .collect::<Vec<_>>();
    let (full, _) = build_and_link_graph(&docs).expect("full graph");
    let mut incremental = SemanticGraph::new();
    for document in &docs {
        let parsed = sysml_v2_parser::parse(&document.content).expect("parse");
        patch_graph_for_document(&mut incremental, &document.uri, Some(&parsed), true);
    }
    let status_for = |graph: &SemanticGraph| {
        graph
            .node_ids_by_qualified_name
            .get("Scope::result")
            .and_then(|ids| ids.first())
            .and_then(|id| graph.get_node(id))
            .and_then(|node| graph.evaluation_facts_for(node))
            .and_then(|facts| facts.expression.as_ref())
            .map(|evaluation| evaluation.status)
    };
    assert_eq!(status_for(&full), Some(EvaluationStatus::Ambiguous));
    assert_eq!(status_for(&incremental), status_for(&full));
}
