use semantic_core::{build_semantic_graph_from_documents, SysmlDocument, SysmlDocumentSourceKind};

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

fn build_graph() -> semantic_core::SemanticGraph {
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

#[test]
fn graph_builder_materializes_analysis_attributes_and_subject_members() {
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
            .contains_key("Demo::PowerAnalysis::robot::mobility"),
        "subject typed member path should be graph materialized"
    );
    assert!(
        graph
            .node_ids_by_qualified_name
            .contains_key("Demo::PowerAnalysis::robot::mobility::drivePowerW"),
        "subject typed nested attribute should be graph materialized"
    );
}

