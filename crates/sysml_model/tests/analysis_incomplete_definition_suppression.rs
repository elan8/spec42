//! Regression coverage for S42-LIM-012: `analysis_evaluation_incomplete` should not be emitted
//! for reusable, parametric `requirement def`/`analysis def`/`verification def`/`use case def`
//! templates that intentionally declare but do not assign subject/constraint values — a real
//! constraint failure on such a definition should still surface, only the "incomplete" info
//! diagnostic is suppressed.

use sysml_model::{
    build_semantic_graph_from_documents, collect_diagnostics_from_graph, DiagnosticsOptions,
    SysmlDocument, SysmlDocumentSourceKind,
};

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

#[test]
fn requirement_def_with_unassigned_values_does_not_emit_incomplete_info() {
    let doc = workspace_doc(
        "capacity.sysml",
        r#"package GridRequirements {
  requirement def CapacityRequirement {
    attribute basePeakLoad;
    attribute winterMultiplier = 1.30;
    attribute requiredCapacity;
    require constraint {
      basePeakLoad * winterMultiplier <= requiredCapacity
    }
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    let incomplete: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.code == "analysis_evaluation_incomplete")
        .collect();
    assert!(
        incomplete.is_empty(),
        "requirement def templates should not emit analysis_evaluation_incomplete: {incomplete:#?}"
    );
}

#[test]
fn requirement_usage_with_unassigned_values_still_emits_incomplete_info() {
    let doc = workspace_doc(
        "capacity_usage.sysml",
        r#"package GridRequirements {
  requirement gridCapacity2035 {
    attribute basePeakLoad;
    attribute winterMultiplier = 1.30;
    attribute requiredCapacity;
    require constraint {
      basePeakLoad * winterMultiplier <= requiredCapacity
    }
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    let incomplete: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.code == "analysis_evaluation_incomplete")
        .collect();
    assert!(
        !incomplete.is_empty(),
        "requirement usages with unassigned values should still surface \
         analysis_evaluation_incomplete (unchanged behavior for non-definition kinds)"
    );
}

#[test]
fn abstract_requirement_usage_with_unassigned_values_does_not_emit_incomplete_info() {
    // An explicitly `abstract` usage is the spec's precise signal for "intentionally waiting
    // for a concrete binding" (SysML v2 8.3.6.4/8.3.6.5), so it should be suppressed the same
    // way a `requirement def` template is, even though it isn't itself a definition.
    let doc = workspace_doc(
        "abstract_capacity_usage.sysml",
        r#"package GridRequirements {
  abstract requirement gridCapacity2035 {
    attribute basePeakLoad;
    attribute winterMultiplier = 1.30;
    attribute requiredCapacity;
    require constraint {
      basePeakLoad * winterMultiplier <= requiredCapacity
    }
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    let incomplete: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.code == "analysis_evaluation_incomplete")
        .collect();
    assert!(
        incomplete.is_empty(),
        "abstract requirement usages should not emit analysis_evaluation_incomplete: {incomplete:#?}"
    );
}
