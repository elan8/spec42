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
fn part_def_enum_usage_materializes_inner_attribute() {
    let doc = workspace_doc(
        "enum.sysml",
        r#"package P {
  enum def Status;
  part def Vehicle {
    enum status : Status {
      attribute code : String;
    }
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let vehicle = graph
        .nodes_named("Vehicle")
        .into_iter()
        .find(|node| node.element_kind == "part def")
        .expect("part def");
    let enum_usage = graph
        .children_of(&vehicle)
        .into_iter()
        .find(|child| child.element_kind == "enumeration" && child.name == "status")
        .expect("enumeration usage");
    assert!(
        graph
            .children_of(enum_usage)
            .iter()
            .any(|child| {
                (child.element_kind == "attribute" || child.element_kind == "attribute def")
                    && child.name == "code"
            }),
        "expected attribute under enumeration usage"
    );
}

#[test]
fn part_def_item_usage_materializes_inner_attribute() {
    let doc = workspace_doc(
        "item.sysml",
        r#"package P {
  item def Payload;
  part def Vehicle {
    item cargo : Payload {
      attribute weight : Real;
    }
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let vehicle = graph
        .nodes_named("Vehicle")
        .into_iter()
        .find(|node| node.element_kind == "part def")
        .expect("part def");
    let item_usage = graph
        .children_of(&vehicle)
        .into_iter()
        .find(|child| child.element_kind == "item" && child.name == "cargo")
        .expect("item usage");
    assert!(
        graph
            .children_of(item_usage)
            .iter()
            .any(|child| {
                (child.element_kind == "attribute" || child.element_kind == "attribute def")
                    && child.name == "weight"
            }),
        "expected attribute under item usage"
    );
}

#[test]
fn part_def_occurrence_usage_brace_body_materializes_attribute() {
    let doc = workspace_doc(
        "occurrence_part.sysml",
        r#"package P {
  occurrence def Step;
  part def Process {
    occurrence step : Step {
      attribute label : String;
    }
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let process = graph
        .nodes_named("Process")
        .into_iter()
        .find(|node| node.element_kind == "part def")
        .expect("part def");
    let occurrence_usage = graph
        .children_of(&process)
        .into_iter()
        .find(|child| child.element_kind == "occurrence" && child.name == "step")
        .expect("occurrence usage");
    assert!(
        graph
            .children_of(occurrence_usage)
            .iter()
            .any(|child| child.element_kind == "attribute" && child.name == "label"),
        "expected attribute under occurrence usage brace body"
    );
}
