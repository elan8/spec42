use sysml_model::{
    build_semantic_graph_from_documents, evaluate_expressions, SysmlDocument,
    SysmlDocumentSourceKind,
};

const LIBRARY_SYSML: &str = r#"
package VerificationLib {
    part def Device;

    verification def PowerVerification {
        subject device : Device;
        return ref withinSpec {
            return true;
        }
    }
}
"#;

const USAGE_SYSML: &str = r#"
package VerificationCases {
    private import VerificationLib::*;

    part unit : Device;

    verification powerCheck : PowerVerification {
        subject device : unit;
    }
}
"#;

#[test]
fn typed_verification_usage_inherits_return_expression() {
    let library = SysmlDocument::from_memory_path(
        "verification-usage-eval",
        "VerificationLib.sysml",
        LIBRARY_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("library uri");
    let consumer = SysmlDocument::from_memory_path(
        "verification-usage-eval",
        "VerificationCases.sysml",
        USAGE_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("consumer uri");
    let (mut graph, _) =
        build_semantic_graph_from_documents(&[library, consumer]).expect("semantic graph");
    evaluate_expressions(&mut graph);

    let expression = graph
        .node_ids_by_qualified_name
        .get("VerificationCases::powerCheck")
        .and_then(|ids| ids.first())
        .and_then(|id| graph.get_node(id))
        .and_then(|node| node.attributes.get("analysisExpression"))
        .and_then(|value| value.as_str())
        .unwrap_or_default();
    assert!(
        !expression.is_empty(),
        "expected propagated verification return expression"
    );

    let status = graph
        .node_ids_by_qualified_name
        .get("VerificationCases::powerCheck")
        .and_then(|ids| ids.first())
        .and_then(|id| graph.get_node(id))
        .and_then(|node| node.attributes.get("analysisEvaluationStatus"))
        .and_then(|value| value.as_str())
        .unwrap_or_default();
    assert_eq!(status, "ok");
}

#[test]
fn verification_return_verdict_kind_pass_evaluates_to_ok() {
    let doc = SysmlDocument::from_memory_path(
        "verification-verdict-eval",
        "Verification.sysml",
        r#"
            package Verification {
                verification verifyCleaningCoverage {
                    return ref verdictResult { return VerdictKind::pass; }
                }
            }
        "#
        .to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("verification doc");
    let (mut graph, _) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    evaluate_expressions(&mut graph);

    let status = graph
        .node_ids_by_qualified_name
        .get("Verification::verifyCleaningCoverage")
        .and_then(|ids| ids.first())
        .and_then(|id| graph.get_node(id))
        .and_then(|node| node.attributes.get("analysisEvaluationStatus"))
        .and_then(|value| value.as_str())
        .unwrap_or_default();
    assert_eq!(
        status, "ok",
        "VerdictKind::pass should evaluate without analysis_evaluation_unresolved"
    );
}
