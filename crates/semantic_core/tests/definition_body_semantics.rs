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

fn child_with_kind<'a>(
    graph: &'a semantic_core::SemanticGraph,
    parent: &semantic_core::SemanticNode,
    kind: &str,
    name: &str,
) -> Option<&'a semantic_core::SemanticNode> {
    graph
        .children_of(parent)
        .into_iter()
        .find(|child| child.element_kind == kind && child.name == name)
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
        child_with_kind(&graph, occurrence_def, "attribute", "id").is_some(),
        "expected attribute child under occurrence def"
    );
}

#[test]
fn occurrence_def_body_materializes_nested_part_usage() {
    let doc = workspace_doc(
        "occurrence_part.sysml",
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
        child_with_kind(&graph, occurrence_def, "part", "wheel").is_some(),
        "expected nested part usage under occurrence def"
    );
}

#[test]
fn flow_def_body_materializes_inner_attribute() {
    let doc = workspace_doc(
        "flow.sysml",
        r#"package P {
  flow def Power {
    attribute rate : Real;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let flow_def = graph
        .nodes_named("Power")
        .into_iter()
        .find(|node| node.element_kind == "flow def")
        .expect("flow def");
    assert!(
        graph
            .children_of(&flow_def)
            .iter()
            .any(|child| {
                (child.element_kind == "attribute" || child.element_kind == "attribute def")
                    && child.name == "rate"
            }),
        "expected attribute child under flow def"
    );
}

#[test]
fn flow_def_body_materializes_nested_part_usage() {
    let doc = workspace_doc(
        "flow_part.sysml",
        r#"package P {
  part def Wheel;
  flow def PowerFlow {
    part wheel : Wheel;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let flow_def = graph
        .nodes_named("PowerFlow")
        .into_iter()
        .find(|node| node.element_kind == "flow def")
        .expect("flow def");
    assert!(
        child_with_kind(&graph, flow_def, "part", "wheel").is_some(),
        "expected nested part usage under flow def"
    );
}

#[test]
fn allocation_def_body_materializes_inner_attribute() {
    let doc = workspace_doc(
        "allocation.sysml",
        r#"package P {
  allocation def Map {
    attribute id : String;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let allocation_def = graph
        .nodes_named("Map")
        .into_iter()
        .find(|node| node.element_kind == "allocation def")
        .expect("allocation def");
    assert!(
        graph
            .children_of(&allocation_def)
            .iter()
            .any(|child| {
                (child.element_kind == "attribute" || child.element_kind == "attribute def")
                    && child.name == "id"
            }),
        "expected attribute child under allocation def"
    );
}

#[test]
fn flow_usage_brace_body_materializes_inner_attribute() {
    let doc = workspace_doc(
        "flow_usage.sysml",
        r#"package P {
  item def Payload;
  flow cargo : Payload {
    attribute weight : Real;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let flow_usage = graph
        .nodes_named("cargo")
        .into_iter()
        .find(|node| node.element_kind == "flow")
        .expect("flow usage");
    assert!(
        graph
            .children_of(&flow_usage)
            .iter()
            .any(|child| {
                (child.element_kind == "attribute" || child.element_kind == "attribute def")
                    && child.name == "weight"
            }),
        "expected attribute child under flow usage brace body"
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
