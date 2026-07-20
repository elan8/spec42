//! Regression coverage for S42-LIM-013: evaluator robustness for Boolean equality expressions
//! and qualified/imported package-level constants used as feature default values.

use sysml_diagnostics::{collect_diagnostics_from_graph, DiagnosticsOptions};
use sysml_model::{build_semantic_graph_from_documents, SysmlDocument, SysmlDocumentSourceKind};

fn workspace_doc(path: &str, content: &str) -> SysmlDocument {
    SysmlDocument::from_memory_path(
        "workspace",
        path,
        content.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("workspace document")
}

fn has_code(diagnostics: &[sysml_diagnostics::SemanticDiagnostic], code: &str) -> bool {
    diagnostics.iter().any(|d| d.code == code)
}

#[test]
fn boolean_equality_constraint_evaluates_without_unresolved_diagnostic() {
    let doc = workspace_doc(
        "bool_true.sysml",
        r#"package P {
  part def Subject {
    attribute flag = true;
  }
  requirement r {
    subject s : Subject;
    require constraint { s.flag == true }
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        !has_code(&diagnostics, "analysis_evaluation_unresolved"),
        "Boolean equality constraint should not be unresolved: {diagnostics:#?}"
    );
    assert!(
        !has_code(&diagnostics, "analysis_constraint_failed"),
        "constraint should pass when flag is true: {diagnostics:#?}"
    );
}

#[test]
fn boolean_equality_constraint_still_detects_a_real_failure() {
    let doc = workspace_doc(
        "bool_false.sysml",
        r#"package P {
  part def Subject {
    attribute flag = false;
  }
  requirement r {
    subject s : Subject;
    require constraint { s.flag == true }
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "analysis_constraint_failed"),
        "constraint should fail when flag is false: {diagnostics:#?}"
    );
}

#[test]
fn qualified_and_unqualified_imported_constant_defaults_resolve_without_unresolved_diagnostic() {
    let doc = workspace_doc(
        "consts.sysml",
        r#"package Limits {
  attribute numericLimit = 10.0;
}
package Requirements {
  private import Limits::*;

  part def QualifiedSubject {
    attribute actual = Limits::numericLimit;
  }
  part def UnqualifiedSubject {
    attribute actual = numericLimit;
  }

  requirement qualifiedReq {
    subject s : QualifiedSubject;
    require constraint { s.actual <= numericLimit }
  }
  requirement unqualifiedReq {
    subject s : UnqualifiedSubject;
    require constraint { s.actual <= numericLimit }
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        !has_code(&diagnostics, "analysis_evaluation_unresolved"),
        "qualified/unqualified imported constant defaults should resolve: {diagnostics:#?}"
    );
}
