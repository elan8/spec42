use sysml_model::{
    build_semantic_graph_from_documents, evaluate_expressions, SysmlDocument,
    SysmlDocumentSourceKind,
};

const NUMERIC_ANALYSIS_SYSML: &str = r#"
package PowerBudget {
    attribute def OpticalPowerLevelDbm :> Real;

    requirement def MinimumTransmitOutputPower {
        subject outputPowerDbm : OpticalPowerLevelDbm;
        attribute minimumOutputPowerDbm : OpticalPowerLevelDbm = -6.0;
        require constraint {
            outputPowerDbm >= minimumOutputPowerDbm
        }
    }

    analysis nominalTransmitPowerBudget {
        attribute carrierInputPowerDbm : OpticalPowerLevelDbm = 3.0;
        attribute totalInsertionLossDb : Real = 7.5;

        return attribute fibreOutputPowerDbm : OpticalPowerLevelDbm =
            carrierInputPowerDbm - totalInsertionLossDb;

        objective outputPowerObjective : MinimumTransmitOutputPower;
    }

    analysis def ReusablePowerBudget {
        attribute carrierInputPowerDbm : OpticalPowerLevelDbm = 3.0;
        attribute totalInsertionLossDb : Real = 7.5;
        return attribute fibreOutputPowerDbm : OpticalPowerLevelDbm =
            carrierInputPowerDbm - totalInsertionLossDb;
    }

    analysis inheritedPowerBudget : ReusablePowerBudget;
}
"#;

fn build_graph() -> sysml_model::SemanticGraph {
    build_graph_from_source(NUMERIC_ANALYSIS_SYSML)
}

fn build_graph_from_source(source: &str) -> sysml_model::SemanticGraph {
    let document = SysmlDocument::from_memory_path(
        "numeric-analysis-results",
        "PowerBudget.sysml",
        source.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document URI");
    let (mut graph, _) = build_semantic_graph_from_documents(&[document]).expect("semantic graph");
    evaluate_expressions(&mut graph);
    graph
}

fn node<'a>(
    graph: &'a sysml_model::SemanticGraph,
    qualified_name: &str,
) -> &'a sysml_model::SemanticNode {
    graph
        .node_ids_by_qualified_name
        .get(qualified_name)
        .and_then(|ids| ids.first())
        .and_then(|id| graph.get_node(id))
        .unwrap_or_else(|| panic!("missing node {qualified_name}"))
}

fn analysis_result<'a>(
    graph: &'a sysml_model::SemanticGraph,
    node: &sysml_model::SemanticNode,
) -> Option<&'a sysml_model::AnalysisEvaluation> {
    graph.evaluation_facts_for(node)?.analysis.as_ref()
}

fn value_number(value: Option<&sysml_model::EvaluatedValue>) -> Option<f64> {
    match value? {
        sysml_model::EvaluatedValue::Integer(value) => Some(*value as f64),
        sysml_model::EvaluatedValue::Real(value) => Some(*value),
        _ => None,
    }
}

#[test]
fn typed_analysis_usage_inherits_numeric_result_metadata_and_value() {
    let graph = build_graph();
    let usage = node(&graph, "PowerBudget::inheritedPowerBudget");
    assert_eq!(
        usage
            .attributes
            .get("analysisResultMode")
            .and_then(|value| value.as_str()),
        Some("value")
    );
    assert_eq!(
        usage
            .attributes
            .get("analysisResultType")
            .and_then(|value| value.as_str()),
        Some("OpticalPowerLevelDbm")
    );
    assert_eq!(
        value_number(
            analysis_result(&graph, usage).and_then(|result| result.computed_value.as_ref())
        ),
        Some(-4.5)
    );
    assert_eq!(
        analysis_result(&graph, usage).map(|result| result.expression.status),
        Some(sysml_model::EvaluationStatus::Ok)
    );
}
