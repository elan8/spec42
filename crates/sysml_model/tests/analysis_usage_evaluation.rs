use sysml_diagnostics::{collect_diagnostics_from_graph, DiagnosticsOptions};
use sysml_model::{
    build_semantic_graph_from_documents, evaluate_expressions, EvaluationStatus, SysmlDocument,
    SysmlDocumentSourceKind,
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

fn build_graph() -> sysml_model::SemanticGraph {
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

fn analysis_status(
    graph: &sysml_model::SemanticGraph,
    qualified: &str,
) -> Option<EvaluationStatus> {
    graph
        .node_ids_by_qualified_name
        .get(qualified)?
        .first()
        .and_then(|node_id| graph.get_node(node_id))
        .and_then(|node| graph.evaluation_facts_for(node))
        .and_then(|facts| facts.analysis.as_ref())
        .map(|analysis| analysis.expression.status)
}

fn has_analysis_diagnostic_code(graph: &sysml_model::SemanticGraph, code: &str) -> bool {
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

    assert_eq!(
        analysis_status(&graph, "AnalysisCases::powerRun"),
        Some(EvaluationStatus::Ok)
    );
}

#[test]
fn specialized_imported_analysis_usage_inherits_expression_via_typing() {
    let graph = build_graph();

    assert_eq!(
        analysis_status(&graph, "AnalysisCases::loadFlowRun"),
        Some(EvaluationStatus::Ok)
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
