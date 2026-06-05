use semantic_core::{
    build_semantic_graph_from_documents, collect_diagnostics_from_graph, DiagnosticsOptions,
    SysmlDocument, SysmlDocumentSourceKind,
};

const INHERITED_RETURN_REF_SYSML: &str = r#"
package PowerAnalysis {
    part def PowerSystem;

    analysis def LoadFlowAnalysis {
        subject powerSystem : PowerSystem;
        return ref loadFlowComplete {
            return true;
        }
    }

    analysis def VoltageDropAnalysis :> LoadFlowAnalysis {
        objective voltageDropObjective {
            doc /* Evaluate voltage deviations. */
        }
    }
}
"#;

#[test]
fn specialized_analysis_def_inherits_parent_return_ref_for_objective_binding() {
    let doc = SysmlDocument::from_memory_path(
        "analysis-inheritance",
        "PowerAnalysis.sysml",
        INHERITED_RETURN_REF_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let objective = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.element_kind == "objective" && node.name == "voltageDropObjective")
        .expect("voltageDropObjective node");
    let bound_to = objective
        .attributes
        .get("objectiveBoundTo")
        .and_then(|value| value.as_str())
        .unwrap_or_default();
    assert!(
        bound_to.ends_with("loadFlowComplete"),
        "expected objective bound to inherited analysis result, got {bound_to:?}"
    );

    let specialized = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.element_kind == "analysis def" && node.name == "VoltageDropAnalysis")
        .expect("VoltageDropAnalysis node");
    assert_eq!(
        specialized
            .attributes
            .get("analysisExpression")
            .and_then(|value| value.as_str()),
        Some("true"),
        "expected inherited analysis expression on specialized analysis def"
    );

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        !diagnostics
            .iter()
            .any(|diag| diag.code == "objective_binding_unresolved"),
        "unexpected objective_binding_unresolved: {diagnostics:?}"
    );
}
