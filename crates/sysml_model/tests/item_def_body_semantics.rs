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

#[test]
fn item_def_body_materializes_inner_attributes() {
    let doc = workspace_doc(
        "item.sysml",
        r#"package P {
  item def Event {
    attribute def timestamp : Real;
    attribute id : String;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let item_def = graph
        .nodes_named("Event")
        .into_iter()
        .find(|node| node.element_kind == "item def")
        .expect("item def");

    for name in ["timestamp", "id"] {
        let child = graph
            .children_of(item_def)
            .into_iter()
            .find(|node| node.name == name)
            .unwrap_or_else(|| panic!("expected attribute member '{name}'"));
        assert_eq!(
            child.parent_id.as_ref(),
            Some(&item_def.id),
            "attribute '{name}' should be child of item def"
        );
    }

    let timestamp = graph
        .children_of(item_def)
        .into_iter()
        .find(|node| node.name == "timestamp")
        .expect("timestamp attribute def");
    assert_eq!(timestamp.element_kind, "attribute def");
}

#[test]
fn individual_def_body_materializes_attribute_usage() {
    let doc = workspace_doc(
        "individual.sysml",
        r#"package P {
  individual def Person {
    attribute name : String;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let individual = graph
        .nodes_named("Person")
        .into_iter()
        .find(|node| node.element_kind == "individual def")
        .expect("individual def");

    assert!(
        graph.children_of(individual).iter().any(|child| {
            (child.element_kind == "attribute" || child.element_kind == "attribute def")
                && child.name == "name"
        }),
        "expected attribute member child on individual def"
    );
}
