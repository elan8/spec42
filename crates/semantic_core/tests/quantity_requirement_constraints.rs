use semantic_core::{
    build_semantic_graph_from_documents, collect_diagnostics_from_graph, evaluate_expressions,
    DiagnosticsOptions, SysmlDocument, SysmlDocumentSourceKind,
};

const PASSING_REQUIREMENT_SYSML: &str = r#"
package GridRequirements {
    requirement def GridCapacity {
        attribute basePeakLoad = 12.3;
        attribute winterMultiplier = 1.30;
        attribute requiredCapacity = 16.0;
        require constraint {
            basePeakLoad * winterMultiplier <= requiredCapacity
        }
    }
}
"#;

const FAILING_REQUIREMENT_SYSML: &str = r#"
package GridRequirements {
    requirement def GridCapacity {
        attribute basePeakLoad = 12.3;
        attribute winterMultiplier = 1.30;
        attribute requiredCapacity = 15.0;
        require constraint {
            basePeakLoad * winterMultiplier <= requiredCapacity
        }
    }
}
"#;

const INHERITED_DEF_CONSTRAINT_SYSML: &str = r#"
package GridRequirements {
    requirement def CapacityRequirement {
        attribute basePeakLoad;
        attribute winterMultiplier = 1.30;
        attribute requiredCapacity;
        require constraint {
            basePeakLoad * winterMultiplier <= requiredCapacity
        }
    }

    requirement gridCapacity2035 : CapacityRequirement {
        attribute basePeakLoad = 12.3;
        attribute requiredCapacity = 16.0;
    }
}
"#;

const QUANTITY_UNITS_REQUIREMENT_SYSML: &str = r#"
package GridRequirements {
    requirement def GridCapacity {
        attribute basePeakLoad = 12.3 [MW];
        attribute winterMultiplier = 1.30;
        attribute requiredCapacity = 16 [MW];
        require constraint {
            basePeakLoad * winterMultiplier <= requiredCapacity
        }
    }
}
"#;

fn build_graph(source: &str) -> semantic_core::SemanticGraph {
    let doc = SysmlDocument::from_memory_path(
        "quantity-requirement-constraints",
        "GridRequirements.sysml",
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

fn has_code(graph: &semantic_core::SemanticGraph, code: &str) -> bool {
    let uri = graph
        .node_ids_by_qualified_name
        .get("GridRequirements::GridCapacity")
        .and_then(|ids| ids.first())
        .map(|id| id.uri.clone())
        .expect("requirement uri");
    collect_diagnostics_from_graph(graph, &uri, DiagnosticsOptions::default())
        .into_iter()
        .any(|diag| diag.code == code)
}

#[test]
fn requirement_usage_quantity_constraint_passes_when_within_capacity() {
    let graph = build_graph(PASSING_REQUIREMENT_SYSML);
    assert_eq!(
        node_attr(
            &graph,
            "GridRequirements::GridCapacity",
            "analysisEvaluationStatus"
        ),
        Some("ok".to_string())
    );
    assert!(
        !has_code(&graph, "analysis_evaluation_unresolved"),
        "quantity require constraint should evaluate without unresolved status"
    );
}

#[test]
fn requirement_usage_quantity_constraint_fails_when_over_capacity() {
    let graph = build_graph(FAILING_REQUIREMENT_SYSML);
    assert_eq!(
        node_attr(
            &graph,
            "GridRequirements::GridCapacity",
            "analysisEvaluationStatus"
        ),
        Some("failed_constraint".to_string())
    );
    assert!(has_code(&graph, "analysis_constraint_failed"));
}

#[test]
fn requirement_usage_inherits_require_constraint_from_typed_definition() {
    let graph = build_graph(INHERITED_DEF_CONSTRAINT_SYSML);
    assert_eq!(
        node_attr(
            &graph,
            "GridRequirements::gridCapacity2035",
            "analysisEvaluationStatus"
        ),
        Some("ok".to_string())
    );
}

#[test]
fn requirement_def_quantity_units_evaluate_in_constraint() {
    let graph = build_graph(QUANTITY_UNITS_REQUIREMENT_SYSML);
    assert_eq!(
        node_attr(
            &graph,
            "GridRequirements::GridCapacity",
            "analysisEvaluationStatus"
        ),
        Some("ok".to_string())
    );
}
