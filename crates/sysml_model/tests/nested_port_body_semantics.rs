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
fn port_def_brace_body_materializes_nested_port_usages() {
    let doc = workspace_doc(
        "ports.sysml",
        r#"package P {
  port def CompositePort {
    port left;
    port right;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let port_def = graph
        .nodes_named("CompositePort")
        .into_iter()
        .find(|node| node.element_kind == "port def")
        .expect("composite port def");

    for name in ["left", "right"] {
        let nested = graph
            .nodes_named(name)
            .into_iter()
            .find(|node| node.element_kind == "port")
            .unwrap_or_else(|| panic!("expected nested port usage '{name}'"));
        assert_eq!(
            nested.parent_id.as_ref(),
            Some(&port_def.id),
            "nested port '{name}' should be child of port def"
        );
        assert!(
            nested.id.qualified_name.contains("CompositePort"),
            "nested qualified name should be under CompositePort"
        );
    }

    assert_eq!(
        graph.child_named(&port_def.id, "left").len(),
        1,
        "left should be direct child of port def"
    );
    let _ = uri;
}

#[test]
fn port_def_directed_item_inout_materializes_nested_attributes() {
    let doc = workspace_doc(
        "debris_port.sysml",
        r#"package P {
  port def DebrisPort {
    inout item debris {
      attribute vol;
      attribute mass;
    }
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let port_def = graph
        .nodes_named("DebrisPort")
        .into_iter()
        .find(|node| node.element_kind == "port def")
        .expect("debris port def");

    let item = graph
        .child_named(&port_def.id, "debris")
        .into_iter()
        .find(|node| node.element_kind == "item")
        .expect("directed item usage under port def");
    assert_eq!(
        item.attributes.get("direction").and_then(|v| v.as_str()),
        Some("inout")
    );

    for name in ["vol", "mass"] {
        let under_item = graph.children_of(item).iter().any(|node| {
            (node.element_kind == "attribute" || node.element_kind == "attribute def")
                && node.name == name
        });
        assert!(
            under_item,
            "expected nested attribute '{name}' under item; item children: {:?}",
            graph
                .children_of(item)
                .iter()
                .map(|node| (&node.name, &node.element_kind))
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn part_usage_nested_port_brace_body_materializes_child_ports() {
    let doc = workspace_doc(
        "vehicle.sysml",
        r#"package P {
  part vehicle {
    port vehicleToRoadPort {
      port leftWheelToRoadPort;
      port rightWheelToRoadPort;
    }
  }
}"#,
    );
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let outer = graph
        .nodes_named("vehicleToRoadPort")
        .into_iter()
        .find(|node| node.element_kind == "port")
        .expect("outer port usage");

    for name in ["leftWheelToRoadPort", "rightWheelToRoadPort"] {
        let nested = graph
            .nodes_named(name)
            .into_iter()
            .find(|node| node.element_kind == "port")
            .unwrap_or_else(|| panic!("expected nested port '{name}'"));
        assert_eq!(
            nested.parent_id.as_ref(),
            Some(&outer.id),
            "nested port '{name}' should be child of vehicleToRoadPort"
        );
    }
}
