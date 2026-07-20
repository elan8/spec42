use sysml_diagnostics::{collect_diagnostics_from_graph, DiagnosticsOptions};
use sysml_model::{
    build_semantic_graph_from_documents, RelationshipKind, SysmlDocument, SysmlDocumentSourceKind,
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
fn action_def_brace_body_materializes_nested_action_usages() {
    let doc = workspace_doc(
        "actions.sysml",
        r#"package P {
  action def Step;
  action def Pipeline {
    action step1 : Step;
    action step2 : Step;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let pipeline = graph
        .nodes_named("Pipeline")
        .into_iter()
        .find(|node| node.element_kind == "action def")
        .expect("pipeline action def");

    for name in ["step1", "step2"] {
        let nested = graph
            .nodes_named(name)
            .into_iter()
            .find(|node| node.element_kind == "action")
            .unwrap_or_else(|| panic!("expected nested action usage '{name}'"));
        assert_eq!(
            nested.parent_id.as_ref(),
            Some(&pipeline.id),
            "nested action '{name}' should be child of pipeline"
        );
    }
    let _ = uri;
}

#[test]
fn action_def_then_action_chain_emits_flow_edges() {
    let doc = workspace_doc(
        "then_chain.sysml",
        r#"package P {
  action def A;
  action def B;
  action def Pipeline {
    then action step1 : A;
    then action step2 : B;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let has_edge = |source_suffix: &str, target_suffix: &str, kind: RelationshipKind| {
        graph
            .edges_for_uri_as_strings(&uri)
            .iter()
            .any(|(src, tgt, k, _)| {
                *k == kind && src.ends_with(source_suffix) && tgt.ends_with(target_suffix)
            })
    };

    assert!(
        has_edge("::Pipeline", "::step1", RelationshipKind::Perform),
        "expected Perform from pipeline to first step"
    );
    assert!(
        has_edge("::step1", "::step2", RelationshipKind::Flow),
        "expected Flow between then-action steps"
    );
}

#[test]
fn action_def_body_materializes_metadata_annotation() {
    let doc = workspace_doc(
        "action_metadata.sysml",
        r#"package P {
  metadata def SafetyTag;
  action def Pipeline {
    @safetyTag : SafetyTag;
    action step1 :> Pipeline;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let pipeline = graph
        .nodes_named("Pipeline")
        .into_iter()
        .find(|node| node.element_kind == "action def")
        .expect("pipeline action def");
    assert!(
        graph
            .children_of(pipeline)
            .iter()
            .any(|child| { child.element_kind == "metadata usage" && child.name == "safetyTag" }),
        "expected metadata usage child under action def body"
    );
}

#[test]
fn action_def_flow_still_emits_succession_invalid_for_bad_target() {
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
        diagnostics
            .iter()
            .any(|d| d.code == "succession_endpoint_invalid"),
        "expected succession_endpoint_invalid, got: {:?}",
        diagnostics
            .iter()
            .map(|d| (&d.code, &d.message))
            .collect::<Vec<_>>()
    );
}
