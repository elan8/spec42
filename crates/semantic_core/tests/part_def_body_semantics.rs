use semantic_core::{build_semantic_graph_from_documents, SysmlDocument, SysmlDocumentSourceKind};

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
        graph.children_of(enum_usage).iter().any(|child| {
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
        graph.children_of(item_usage).iter().any(|child| {
            (child.element_kind == "attribute" || child.element_kind == "attribute def")
                && child.name == "weight"
        }),
        "expected attribute under item usage"
    );
}

#[test]
fn part_def_nested_part_def_materializes_as_child_of_part_def() {
    let doc = workspace_doc(
        "accumulator_part.sysml",
        r#"package P {
  part def Accumulator {
    part def Cell {
      attribute capacity : Real;
    }
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let accumulator = graph
        .nodes_named("Accumulator")
        .into_iter()
        .find(|node| node.element_kind == "part def")
        .expect("part def");
    let cell = graph
        .children_of(&accumulator)
        .into_iter()
        .find(|child| child.element_kind == "part def" && child.name == "Cell")
        .expect("nested part def");
    assert_eq!(cell.parent_id.as_ref(), Some(&accumulator.id));
    assert!(
        graph.children_of(&cell).iter().any(|child| {
            (child.element_kind == "attribute" || child.element_kind == "attribute def")
                && child.name == "capacity"
        }),
        "expected capacity attribute under nested part def"
    );
}

#[test]
fn part_def_nested_item_def_materializes_as_child_of_part_def() {
    let doc = workspace_doc(
        "accumulator.sysml",
        r#"package P {
  part def Accumulator {
    item def Energy;
    attribute mass : Real;
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let accumulator = graph
        .nodes_named("Accumulator")
        .into_iter()
        .find(|node| node.element_kind == "part def")
        .expect("part def");
    let energy = graph
        .children_of(&accumulator)
        .into_iter()
        .find(|child| child.element_kind == "item def" && child.name == "Energy")
        .expect("nested item def");
    assert_eq!(energy.parent_id.as_ref(), Some(&accumulator.id));
    assert!(
        graph.children_of(&accumulator).iter().any(|child| {
            (child.element_kind == "attribute" || child.element_kind == "attribute def")
                && child.name == "mass"
        }),
        "expected mass attribute as sibling of nested item def"
    );
}

#[test]
fn part_def_nested_item_def_body_materializes_inner_attribute() {
    let doc = workspace_doc(
        "energy.sysml",
        r#"package P {
  part def Accumulator {
    item def Energy {
      attribute density : Real;
    }
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let accumulator = graph
        .nodes_named("Accumulator")
        .into_iter()
        .find(|node| node.element_kind == "part def")
        .expect("part def");
    let energy = graph
        .children_of(&accumulator)
        .into_iter()
        .find(|child| child.element_kind == "item def" && child.name == "Energy")
        .expect("nested item def");
    assert!(
        graph.children_of(&energy).iter().any(|child| {
            (child.element_kind == "attribute" || child.element_kind == "attribute def")
                && child.name == "density"
        }),
        "expected attribute under nested item def body"
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
