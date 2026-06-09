use semantic_core::{
    build_semantic_graph_from_documents, collect_diagnostics_from_graph, DiagnosticsOptions,
    SysmlDocument, SysmlDocumentSourceKind,
};

const METADATA_DESIGN_DECISION_SYSML: &str = r#"
package DesignDecisions {
    metadata def DesignDecision {
        attribute id;
        attribute status;
    }

    metadata decision001 : DesignDecision {
        attribute id = "DD-001";
        attribute status = "approved";
    }
}
"#;

#[test]
fn metadata_def_and_usage_with_attribute_bindings_have_no_semantic_diagnostics() {
    let doc = SysmlDocument::from_memory_path(
        "metadata-design-decisions",
        "design_decisions.sysml",
        METADATA_DESIGN_DECISION_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    let uri = doc.uri.clone();
    let (graph, _parsed) =
        build_semantic_graph_from_documents(&[doc]).expect("semantic graph should build");

    let metadata_def = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.element_kind == "metadata def" && node.name == "DesignDecision")
        .expect("metadata def node");
    let metadata_usage = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.element_kind == "metadata usage" && node.name == "decision001")
        .expect("metadata usage node");

    let def_attributes: Vec<_> = graph
        .children_of(metadata_def)
        .into_iter()
        .filter(|child| child.element_kind == "attribute def")
        .map(|child| child.name.as_str())
        .collect();
    assert!(def_attributes.contains(&"id"));
    assert!(def_attributes.contains(&"status"));

    let usage_attributes: Vec<_> = graph
        .children_of(metadata_usage)
        .into_iter()
        .filter(|child| child.element_kind == "attribute")
        .map(|child| child.name.as_str())
        .collect();
    assert!(usage_attributes.contains(&"id"));
    assert!(usage_attributes.contains(&"status"));

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    let semantic_codes: Vec<_> = diagnostics
        .iter()
        .filter(|diag| diag.source == "semantic")
        .map(|diag| diag.code.as_str())
        .collect();
    assert!(
        semantic_codes.is_empty(),
        "unexpected semantic diagnostics: {semantic_codes:?}"
    );
}

#[test]
fn requirement_body_metadata_annotation_materializes_on_graph() {
    let doc = SysmlDocument::from_memory_path(
        "metadata-requirement",
        "requirement_metadata.sysml",
        r#"package P {
  metadata def ReviewTag;
  requirement def R1 {
    @reviewTag : ReviewTag;
    doc /* tagged requirement */
  }
}"#
        .to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let requirement = graph
        .nodes_named("R1")
        .into_iter()
        .find(|node| node.element_kind == "requirement def")
        .expect("requirement def");
    assert!(
        graph
            .children_of(&requirement)
            .iter()
            .any(|child| child.element_kind == "metadata usage" && child.name == "reviewTag"),
        "expected metadata usage under requirement def body"
    );
}
