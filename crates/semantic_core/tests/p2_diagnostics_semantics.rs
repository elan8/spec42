use semantic_core::{
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

fn has_code(diagnostics: &[semantic_core::SemanticDiagnostic], code: &str) -> bool {
    diagnostics.iter().any(|diagnostic| diagnostic.code == code)
}

#[test]
fn action_flow_to_part_def_emits_succession_invalid() {
    let doc = workspace_doc(
        "flow_invalid.sysml",
        r#"package Demo {
  part def WrongPart;
  action def Step1;
  action def Pipeline {
    action step1 : Step1;
    flow step1 to WrongPart;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "succession_endpoint_invalid"),
        "expected succession_endpoint_invalid, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn requirement_satisfy_wrong_target_emits_kind_diagnostic() {
    let doc = workspace_doc(
        "satisfy_invalid.sysml",
        r#"package Demo {
  requirement def ReqA;
  part def System;
  requirement r1 : ReqA;
  satisfy r1 by System;
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "satisfy_invalid_endpoint_kind"),
        "expected satisfy_invalid_endpoint_kind, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn view_without_expose_emits_empty_expose_diagnostic() {
    let doc = workspace_doc(
        "view_empty.sysml",
        r#"package Demo {
  view def StructuralView;
  view structure : StructuralView {
    filter @Type;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "view_expose_empty"),
        "expected view_expose_empty, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn untyped_metadata_annotation_emits_unresolved_diagnostic() {
    let doc = workspace_doc(
        "metadata_unresolved.sysml",
        r#"package Demo {
  metadata orphan;
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "metadata_annotation_unresolved"),
        "expected metadata_annotation_unresolved, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn include_missing_use_case_emits_invalid_target() {
    let doc = workspace_doc(
        "include_invalid.sysml",
        r#"package Demo {
  use case def Main {
    include MissingCase;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        has_code(&diagnostics, "use_case_include_invalid_target"),
        "expected use_case_include_invalid_target, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}

#[test]
fn metadata_usage_with_valid_type_has_no_annotation_diagnostic() {
    let doc = workspace_doc(
        "metadata_ok.sysml",
        r#"package Demo {
  metadata def Tag;
  metadata tag1 : Tag;
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        !has_code(&diagnostics, "metadata_annotation_unresolved"),
        "unexpected metadata_annotation_unresolved: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}
