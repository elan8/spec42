use semantic_core::{
    build_semantic_graph_from_documents, collect_diagnostics_from_graph, evaluate_expressions,
    DiagnosticsOptions, SysmlDocument, SysmlDocumentSourceKind,
};

const PASSING_ASSERT_SYSML: &str = r#"
package Grid {
    occurrence def Feeder {
        attribute load : Real = 8;
        attribute thermalLimit : Real = 10;
        assert constraint {
            load <= thermalLimit;
        }
    }
}
"#;

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

const FAILING_ASSERT_SYSML: &str = r#"
package Grid {
    occurrence def OverloadedFeeder {
        attribute load : Real = 12;
        attribute thermalLimit : Real = 10;
        assert constraint {
            load <= thermalLimit;
        }
    }
}
"#;

fn build_graph(source: &str) -> semantic_core::SemanticGraph {
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

fn node_attr(graph: &semantic_core::SemanticGraph, qualified: &str, key: &str) -> Option<String> {
    graph
        .node_ids_by_qualified_name
        .get(qualified)?
        .first()
        .and_then(|node_id| graph.get_node(node_id))
        .and_then(|node| node.attributes.get(key))
        .and_then(|value| match value {
            serde_json::Value::String(text) => Some(text.clone()),
            serde_json::Value::Number(number) => Some(number.to_string()),
            serde_json::Value::Bool(flag) => Some(flag.to_string()),
            _ => None,
        })
}

fn diagnostics_for(graph: &semantic_core::SemanticGraph) -> Vec<semantic_core::SemanticDiagnostic> {
    let uri = graph
        .node_ids_by_qualified_name
        .keys()
        .next()
        .and_then(|qualified| graph.node_ids_by_qualified_name.get(qualified))
        .and_then(|ids| ids.first())
        .map(|id| id.uri.clone())
        .expect("document uri");
    collect_diagnostics_from_graph(graph, &uri, DiagnosticsOptions::default())
}

#[test]
fn assert_constraint_projects_to_analysis_constraints_and_passes_when_true() {
    let graph = build_graph(PASSING_ASSERT_SYSML);
    let constraints = graph
        .node_ids_by_qualified_name
        .get("Grid::Feeder")
        .and_then(|ids| ids.first())
        .and_then(|id| graph.get_node(id))
        .and_then(|node| node.attributes.get("analysisConstraints"))
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();
    assert_eq!(
        constraints.len(),
        1,
        "expected one aggregated assert constraint"
    );
    assert_eq!(
        node_attr(&graph, "Grid::Feeder", "analysisEvaluationStatus"),
        Some("ok".to_string())
    );
    assert!(
        !diagnostics_for(&graph)
            .iter()
            .any(|diag| diag.code == "analysis_constraint_failed"),
        "passing assert constraint should not emit analysis_constraint_failed"
    );
}

#[test]
fn assert_constraint_sums_nested_part_siblings_for_collection_projection() {
    let graph = build_graph(MULTI_PART_SUM_SYSML);
    assert_eq!(
        node_attr(&graph, "Grid::System", "analysisEvaluationStatus"),
        Some("ok".to_string())
    );
}

#[test]
fn assert_constraint_emits_failed_analysis_diagnostic_when_false() {
    let graph = build_graph(FAILING_ASSERT_SYSML);
    assert_eq!(
        node_attr(&graph, "Grid::OverloadedFeeder", "analysisEvaluationStatus"),
        Some("failed_constraint".to_string())
    );
    assert!(
        diagnostics_for(&graph)
            .iter()
            .any(|diag| diag.code == "analysis_constraint_failed"),
        "expected analysis_constraint_failed for violated assert constraint"
    );
}
