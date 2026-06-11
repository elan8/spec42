use semantic_core::{
    build_semantic_graph_from_documents, collect_diagnostics_from_graph, evaluate_expressions,
    DiagnosticsOptions, SysmlDocument, SysmlDocumentSourceKind,
};

const ARCHITECTURE_SYSML: &str = r#"
package Architecture {
    part def MobilitySubsystem {
        attribute drivePowerW : Real = 28;
    }

    part def Robot {
        part mobility : MobilitySubsystem;
    }
}
"#;

const LIBRARY_SYSML: &str = r#"
package GridAnalysis {
    private import Architecture::*;

    analysis def PowerCheck {
        attribute powerBudgetW : Real = 55;
        subject robot : Robot;
        return ref withinBudget {
            return sum(robot.mobility.drivePowerW) <= powerBudgetW;
        }
    }

    analysis def LoadFlowAnalysis :> PowerCheck {
        return ref loadFlowComplete {
            return sum(robot.mobility.drivePowerW) <= powerBudgetW;
        }
    }
}
"#;

const USAGE_SYSML: &str = r#"
package AnalysisCases {
    private import GridAnalysis::*;
    private import Architecture::*;

    analysis powerRun : PowerCheck {
        subject robot : Robot;
    }

    analysis loadFlowRun : LoadFlowAnalysis {
        subject robot : Robot;
    }
}
"#;

fn build_graph() -> semantic_core::SemanticGraph {
    let architecture = SysmlDocument::from_memory_path(
        "analysis-usage-eval",
        "Architecture.sysml",
        ARCHITECTURE_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("architecture uri");
    let library = SysmlDocument::from_memory_path(
        "analysis-usage-eval",
        "GridAnalysis.sysml",
        LIBRARY_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("library uri");
    let consumer = SysmlDocument::from_memory_path(
        "analysis-usage-eval",
        "AnalysisCases.sysml",
        USAGE_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("consumer uri");
    let (mut graph, _) = build_semantic_graph_from_documents(&[architecture, library, consumer])
        .expect("semantic graph");
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

fn has_analysis_diagnostic_code(graph: &semantic_core::SemanticGraph, code: &str) -> bool {
    let uri = graph
        .node_ids_by_qualified_name
        .get("AnalysisCases::powerRun")
        .and_then(|ids| ids.first())
        .map(|id| id.uri.clone())
        .expect("analysis usage uri");
    collect_diagnostics_from_graph(graph, &uri, DiagnosticsOptions::default())
        .into_iter()
        .any(|diag| diag.code == code)
}

#[test]
fn typed_analysis_usage_inherits_expression_and_evaluates_successfully() {
    let graph = build_graph();

    assert!(
        node_attr(&graph, "AnalysisCases::powerRun", "analysisExpression")
            .is_some_and(|expr| expr.contains("sum(robot.mobility.drivePowerW)")),
        "expected propagated analysis expression on usage"
    );
    assert_eq!(
        node_attr(
            &graph,
            "AnalysisCases::powerRun",
            "analysisEvaluationStatus"
        ),
        Some("ok".to_string())
    );
    assert_eq!(
        node_attr(
            &graph,
            "AnalysisCases::powerRun",
            "analysisConstraintPassed"
        ),
        Some("true".to_string())
    );
}

#[test]
fn specialized_imported_analysis_usage_inherits_expression_via_typing() {
    let graph = build_graph();

    assert!(
        node_attr(&graph, "AnalysisCases::loadFlowRun", "analysisExpression").is_some(),
        "expected propagated expression on specialized analysis usage"
    );
    assert_eq!(
        node_attr(
            &graph,
            "AnalysisCases::loadFlowRun",
            "analysisEvaluationStatus"
        ),
        Some("ok".to_string())
    );
}

#[test]
fn typed_analysis_usage_does_not_emit_analysis_evaluation_unresolved() {
    let graph = build_graph();
    assert!(
        !has_analysis_diagnostic_code(&graph, "analysis_evaluation_unresolved"),
        "unexpected analysis_evaluation_unresolved after typed usage evaluation"
    );
}
