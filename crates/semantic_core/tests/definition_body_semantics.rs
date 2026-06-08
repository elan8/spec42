use semantic_core::{
    build_semantic_graph_from_documents, SysmlDocument, SysmlDocumentSourceKind,
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
fn occurrence_def_body_materializes_inner_attribute() {
    let doc = workspace_doc(
        "occurrence.sysml",
        r#"package P {
  occurrence def Event {
    attribute id : String;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let occurrence_def = graph
        .nodes_named("Event")
        .into_iter()
        .find(|node| node.element_kind == "occurrence def")
        .expect("occurrence def");
    assert!(
        graph
            .children_of(&occurrence_def)
            .iter()
            .any(|child| child.element_kind == "attribute" && child.name == "id"),
        "expected attribute child under occurrence def"
    );
}

#[test]
fn occurrence_def_body_materializes_nested_part_usage() {
    let doc = workspace_doc(
        "flow.sysml",
        r#"package P {
  part def Wheel;
  occurrence def Event {
    part wheel : Wheel;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let occurrence_def = graph
        .nodes_named("Event")
        .into_iter()
        .find(|node| node.element_kind == "occurrence def")
        .expect("occurrence def");
    assert!(
        graph
            .children_of(&occurrence_def)
            .iter()
            .any(|child| child.element_kind == "part" && child.name == "wheel"),
        "expected nested part usage under occurrence def"
    );
}

#[test]
fn flow_def_shell_still_materializes_with_doc_only_body() {
    let doc = workspace_doc(
        "flow_doc.sysml",
        r#"package P {
  flow def PowerFlow {
    doc /* flow body is parser doc-only today */
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    assert!(
        graph
            .nodes_named("PowerFlow")
            .into_iter()
            .any(|node| node.element_kind == "flow def"),
        "flow def shell should still materialize"
    );
}

#[test]
fn occurrence_def_assert_constraint_materializes_child_node() {
    let doc = workspace_doc(
        "assert.sysml",
        r#"package P {
  occurrence def Checked {
    assert constraint {
      true;
    }
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let occurrence_def = graph
        .nodes_named("Checked")
        .into_iter()
        .find(|node| node.element_kind == "occurrence def")
        .expect("occurrence def");
    assert!(
        graph
            .children_of(&occurrence_def)
            .iter()
            .any(|child| child.element_kind == "assert constraint"),
        "expected assert constraint child under occurrence def"
    );
}
