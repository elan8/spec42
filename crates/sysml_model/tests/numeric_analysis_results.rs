use sysml_diagnostics::{collect_diagnostics_from_graph, DiagnosticsOptions};
use sysml_model::{
    build_semantic_graph_from_documents, evaluate_expressions, ElementKind, SysmlDocument,
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

const FAILING_NUMERIC_ANALYSIS_SYSML: &str = r#"
package PowerBudget {
    attribute def OpticalPowerLevelDbm :> Real;

    requirement def StrictMinimumTransmitOutputPower {
        subject outputPowerDbm : OpticalPowerLevelDbm;
        attribute minimumOutputPowerDbm : OpticalPowerLevelDbm = -4.0;
        require constraint { outputPowerDbm >= minimumOutputPowerDbm }
    }

    analysis insufficientTransmitPowerBudget {
        attribute carrierInputPowerDbm : OpticalPowerLevelDbm = 3.0;
        attribute totalInsertionLossDb : Real = 7.5;
        return attribute fibreOutputPowerDbm : OpticalPowerLevelDbm =
            carrierInputPowerDbm - totalInsertionLossDb;
        objective outputPowerObjective : StrictMinimumTransmitOutputPower;
    }
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

fn number_attr(node: &sysml_model::SemanticNode, key: &str) -> Option<f64> {
    node.attributes.get(key).and_then(|value| value.as_f64())
}

#[test]
#[ignore = "SKIP: typed objective/requirement comparison facts are not modeled yet; sign-based legacy analysis verdicts are intentionally removed"]
fn numeric_return_attribute_materializes_and_satisfies_objective_requirement() {
    let graph = build_graph();
    let analysis = node(&graph, "PowerBudget::nominalTransmitPowerBudget");
    assert_eq!(
        analysis
            .attributes
            .get("analysisResultMode")
            .and_then(|value| value.as_str()),
        Some("value")
    );
    assert_eq!(number_attr(analysis, "analysisComputedValue"), Some(-4.5));
    assert_eq!(
        analysis
            .attributes
            .get("analysisEvaluationStatus")
            .and_then(|value| value.as_str()),
        Some("ok")
    );
    assert_eq!(
        analysis
            .attributes
            .get("analysisConstraintPassed")
            .and_then(|value| value.as_bool()),
        Some(true)
    );
    assert_eq!(number_attr(analysis, "analysisLimitValue"), Some(-6.0));
    assert_eq!(
        number_attr(analysis, "analysisComputedValue").unwrap()
            - number_attr(analysis, "analysisLimitValue").unwrap(),
        1.5
    );

    let result = node(
        &graph,
        "PowerBudget::nominalTransmitPowerBudget::fibreOutputPowerDbm",
    );
    assert_eq!(result.element_kind, ElementKind::AnalysisResult);
    assert_eq!(
        result
            .attributes
            .get("returnType")
            .and_then(|value| value.as_str()),
        Some("OpticalPowerLevelDbm")
    );
    assert_eq!(number_attr(result, "evaluatedValue"), Some(-4.5));

    let objective = node(
        &graph,
        "PowerBudget::nominalTransmitPowerBudget::outputPowerObjective",
    );
    assert_eq!(
        objective
            .attributes
            .get("objectiveBoundTo")
            .and_then(|value| value.as_str()),
        Some("PowerBudget::nominalTransmitPowerBudget::fibreOutputPowerDbm")
    );

    let diagnostics =
        collect_diagnostics_from_graph(&graph, &analysis.id.uri, DiagnosticsOptions::default());
    for forbidden in [
        "objective_binding_unresolved",
        "case_objective_binding_cardinality",
        "analysis_constraint_failed",
    ] {
        assert!(
            diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != forbidden),
            "unexpected {forbidden}: {diagnostics:#?}"
        );
    }
}

#[test]
#[ignore = "SKIP: typed objective/requirement comparison facts are not modeled yet; sign-based legacy analysis verdicts are intentionally removed"]
fn numeric_result_fails_when_bound_objective_requirement_is_not_met() {
    let graph = build_graph_from_source(FAILING_NUMERIC_ANALYSIS_SYSML);
    let analysis = node(&graph, "PowerBudget::insufficientTransmitPowerBudget");
    assert_eq!(number_attr(analysis, "analysisComputedValue"), Some(-4.5));
    assert_eq!(number_attr(analysis, "analysisLimitValue"), Some(-4.0));
    assert_eq!(
        analysis
            .attributes
            .get("analysisConstraintPassed")
            .and_then(|value| value.as_bool()),
        Some(false)
    );
    assert_eq!(
        analysis
            .attributes
            .get("analysisEvaluationStatus")
            .and_then(|value| value.as_str()),
        Some("failed_constraint")
    );

    let diagnostics =
        collect_diagnostics_from_graph(&graph, &analysis.id.uri, DiagnosticsOptions::default());
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "analysis_constraint_failed"),
        "expected failed objective diagnostic: {diagnostics:#?}"
    );
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
    assert_eq!(number_attr(usage, "analysisComputedValue"), Some(-4.5));
    assert_eq!(
        usage
            .attributes
            .get("analysisEvaluationStatus")
            .and_then(|value| value.as_str()),
        Some("ok")
    );
}
