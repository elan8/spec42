use sysml_model::{
    build_semantic_graph_from_documents, collect_diagnostics_from_graph, evaluate_expressions,
    DiagnosticsOptions, RelationshipKind, SysmlDocument, SysmlDocumentSourceKind,
};

const ANALYSIS_ROLLUP_SYSML: &str = r#"
package Demo {
  part def MobilitySubsystem {
    attribute drivePowerW : Real = 28;
  }

  part def Robot {
    part mobility : MobilitySubsystem;
  }

  analysis def PowerAnalysis {
    attribute powerBudgetW : Real = 55;
    subject robot : Robot;
    return ref withinBudget {
      return sum(robot.mobility.drivePowerW) <= powerBudgetW;
    }
  }
}
"#;

const ARCHITECTURE_SYSML: &str = r#"
package Architecture {
  part def MobilitySubsystem {
    attribute drivePowerW : Real = 28;
  }
  part def CleaningSubsystem {
    attribute brushPowerW : Real = 12;
    attribute vacuumPowerW : Real = 6;
  }
  part def PowerSubsystem {
    attribute electronicsPowerW : Real = 8;
  }
  part def AutonomousFloorCleaningRobot {
    part mobility : MobilitySubsystem;
    part cleaning : CleaningSubsystem;
    part power : PowerSubsystem;
  }
}
"#;

const ANALYSIS_CASES_SYSML: &str = r#"
package AnalysisCases {
  private import Architecture::*;

  analysis def TotalPowerConsumptionAnalysis {
    attribute powerBudgetW : Real = 55;
    subject robot : AutonomousFloorCleaningRobot;
    return ref powerWithinBudget {
      return sum(
        robot.mobility.drivePowerW,
        robot.cleaning.brushPowerW,
        robot.cleaning.vacuumPowerW,
        robot.power.electronicsPowerW
      ) <= powerBudgetW;
    }
  }
}
"#;

fn build_graph() -> sysml_model::SemanticGraph {
    let doc = SysmlDocument::from_memory_path(
        "analysis-rollup",
        "demo.sysml",
        ANALYSIS_ROLLUP_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    graph
}

fn build_two_document_graph() -> sysml_model::SemanticGraph {
    let architecture = SysmlDocument::from_memory_path(
        "analysis-rollup",
        "Architecture.sysml",
        ARCHITECTURE_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("architecture uri");
    let analysis = SysmlDocument::from_memory_path(
        "analysis-rollup",
        "AnalysisCases.sysml",
        ANALYSIS_CASES_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("analysis uri");
    let (mut graph, _parsed) =
        build_semantic_graph_from_documents(&[architecture, analysis]).expect("semantic graph");
    evaluate_expressions(&mut graph);
    graph
}

fn node_attr(graph: &sysml_model::SemanticGraph, qualified: &str, key: &str) -> Option<String> {
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

#[test]
fn graph_builder_materializes_analysis_attributes_and_subject_typing() {
    let graph = build_graph();

    assert!(
        graph
            .node_ids_by_qualified_name
            .contains_key("Demo::PowerAnalysis::powerBudgetW"),
        "analysis-local attribute should be graph materialized"
    );
    assert!(
        graph
            .node_ids_by_qualified_name
            .contains_key("Demo::PowerAnalysis::robot"),
        "analysis subject should be graph materialized"
    );
    let subject_id = graph
        .node_ids_by_qualified_name
        .get("Demo::PowerAnalysis::robot")
        .and_then(|ids| ids.first())
        .expect("analysis subject node");
    let targets = graph
        .outgoing_typing_or_specializes_targets(graph.get_node(subject_id).expect("subject node"));
    assert!(
        targets
            .iter()
            .any(|target| target.id.qualified_name == "Demo::Robot"),
        "subject should type to Demo::Robot"
    );
}

#[test]
fn cross_document_analysis_subject_links_to_architecture_type() {
    let graph = build_two_document_graph();
    let subject_qualified = "AnalysisCases::TotalPowerConsumptionAnalysis::robot";
    let subject_id = graph
        .node_ids_by_qualified_name
        .get(subject_qualified)
        .and_then(|ids| ids.first())
        .expect("analysis subject node");
    let targets = graph
        .outgoing_typing_or_specializes_targets(graph.get_node(subject_id).expect("subject node"));
    assert!(
        targets.iter().any(|target| {
            target.id.qualified_name == "Architecture::AutonomousFloorCleaningRobot"
        }),
        "subject should type to architecture robot definition, got {:?}",
        targets
            .iter()
            .map(|node| node.id.qualified_name.as_str())
            .collect::<Vec<_>>()
    );
}

#[test]
fn cross_document_analysis_subject_relationship_resolves_without_pending_diagnostic() {
    let graph = build_two_document_graph();
    let analysis_qualified = "AnalysisCases::TotalPowerConsumptionAnalysis";
    let analysis_id = graph
        .node_ids_by_qualified_name
        .get(analysis_qualified)
        .and_then(|ids| ids.first())
        .expect("analysis case node");
    let subject_targets = graph.outgoing_targets_by_kind(
        graph.get_node(analysis_id).expect("analysis case"),
        RelationshipKind::Subject,
    );
    assert!(
        subject_targets.iter().any(|target| {
            target.id.qualified_name == "Architecture::AutonomousFloorCleaningRobot"
        }),
        "analysis case should have subject edge to imported part def, got {:?}",
        subject_targets
            .iter()
            .map(|node| node.id.qualified_name.as_str())
            .collect::<Vec<_>>()
    );
    assert!(
        graph.pending_relationships.is_empty(),
        "unexpected pending relationships: {:?}",
        graph.pending_relationships
    );
    let analysis_uri = analysis_id.uri.clone();
    let diagnostics =
        collect_diagnostics_from_graph(&graph, &analysis_uri, DiagnosticsOptions::default());
    assert!(
        !diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "unresolved_pending_relationship"),
        "unexpected unresolved subject pending diagnostic: {diagnostics:?}"
    );
}

#[test]
fn cross_document_analysis_evaluates_power_rollup() {
    let graph = build_two_document_graph();
    let analysis_qualified = "AnalysisCases::TotalPowerConsumptionAnalysis";
    assert_eq!(
        node_attr(&graph, analysis_qualified, "analysisEvaluationStatus").as_deref(),
        Some("ok"),
        "analysis should evaluate successfully"
    );
    assert_eq!(
        node_attr(&graph, analysis_qualified, "analysisComputedValue").as_deref(),
        Some("54"),
        "expected 28+12+6+8 power roll-up"
    );
    assert_eq!(
        node_attr(&graph, analysis_qualified, "analysisEvaluationValue").as_deref(),
        Some("true"),
        "comparison constraint should pass"
    );
    assert_eq!(
        node_attr(&graph, analysis_qualified, "analysisLimitDisplay").as_deref(),
        Some("55"),
        "budget attribute on analysis def should project as limit"
    );
}
