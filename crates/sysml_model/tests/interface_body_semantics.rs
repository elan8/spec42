use semantic_core::{
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
fn interface_def_body_materializes_ends_with_port_type() {
    let doc = workspace_doc(
        "iface.sysml",
        r#"package P {
  port def PowerPort;
  interface def PowerLink {
    end source : PowerPort;
    end sink : PowerPort;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let iface = graph
        .nodes_named("PowerLink")
        .into_iter()
        .find(|node| node.element_kind == "interface")
        .expect("interface def");

    let ends: Vec<_> = graph
        .children_of(iface)
        .into_iter()
        .filter(|child| child.element_kind == "interface end")
        .collect();
    assert_eq!(ends.len(), 2, "expected two interface ends");
    for end in &ends {
        assert_eq!(
            end.attributes.get("portType").and_then(|v| v.as_str()),
            Some("PowerPort"),
            "interface end should expose portType for conformance"
        );
    }
    let _ = uri;
}

#[test]
fn interface_def_with_distinct_end_types_wires_connection_edges() {
    let doc = workspace_doc(
        "iface_connect.sysml",
        r#"package P {
  port def SourcePort;
  port def SinkPort;
  interface def PowerLink {
    end source : SourcePort;
    end sink : SinkPort;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    let connection_edges: Vec<_> = graph
        .edges_for_uri_as_strings(&uri)
        .into_iter()
        .filter(|(_, _, kind, _)| *kind == RelationshipKind::Connection)
        .collect();
    assert!(
        !connection_edges.is_empty(),
        "expected auto-wired connection edges between matching interface ends, got: {:?}",
        connection_edges
    );
}
