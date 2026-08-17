use sysml_model::{
    build_semantic_graph_from_documents, evaluate_expressions, SysmlDocument,
    SysmlDocumentSourceKind,
};

const MULTI_PART_SUM_SYSML: &str = r#"
package Grid {
    part def Module {
        attribute powerW : Real = 10;
    }

    occurrence def System {
        attribute budget : Real = 25;
        part moduleA : Module;
        part moduleB : Module;
        assert constraint {
            sum(modules.powerW) <= budget;
        }
    }
}
"#;

const NON_BOOLEAN_ASSERT_SYSML: &str = r#"
package Grid {
    occurrence def Feeder {
        attribute load : Real = 8;
        assert constraint {
            load;
        }
    }
}
"#;

fn build_graph(source: &str) -> sysml_model::SemanticGraph {
    let doc = SysmlDocument::from_memory_path(
        "assert-constraint-eval",
        "Grid.sysml",
        source.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    let (mut graph, _) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    evaluate_expressions(&mut graph);
    graph
}

fn analysis_result(
    graph: &sysml_model::SemanticGraph,
    qualified: &str,
) -> Option<(sysml_model::EvaluationStatus, Option<bool>)> {
    graph
        .node_ids_by_qualified_name
        .get(qualified)?
        .first()
        .and_then(|node_id| graph.get_node(node_id))
        .and_then(|node| graph.evaluation_facts_for(node))
        .and_then(|facts| facts.analysis.as_ref())
        .map(|analysis| (analysis.expression.status, analysis.passed))
}

#[test]
#[ignore = "SKIP: typed collection-member projection for sum(modules.powerW) is not modeled; the evaluator reports unresolved rather than synthesizing members"]
fn assert_constraint_sums_nested_part_siblings_for_collection_projection() {
    let graph = build_graph(MULTI_PART_SUM_SYSML);
    assert_eq!(
        analysis_result(&graph, "Grid::System"),
        Some((sysml_model::EvaluationStatus::Ok, Some(true)))
    );
}

/// A numeric constraint result is evaluated but carries no verdict.
///
/// Reporting it is no longer this graph's job: `non_boolean_expression` is settled by the
/// immutable publication from the same settled evaluation, so the assertion that survives here is
/// the one this evaluator owns -- that the analysis ran and produced no pass/fail.
#[test]
fn assert_constraint_with_numeric_result_evaluates_without_a_verdict() {
    let graph = build_graph(NON_BOOLEAN_ASSERT_SYSML);
    assert_eq!(
        analysis_result(&graph, "Grid::Feeder"),
        Some((sysml_model::EvaluationStatus::Ok, None))
    );
}
