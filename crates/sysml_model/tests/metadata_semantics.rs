use sysml_model::{
    build_semantic_graph_from_documents, RelationshipKind, SysmlDocument, SysmlDocumentSourceKind,
};

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
    let annotation = graph
        .children_of(requirement)
        .into_iter()
        .find(|child| child.element_kind == "metadata usage" && child.name == "reviewTag")
        .expect("expected metadata usage under requirement def body");
    assert_eq!(
        annotation.id.qualified_name,
        format!("{}::reviewTag", requirement.id.qualified_name),
        "metadata usage QN must nest under the owning requirement (not the package)"
    );
    assert!(
        graph
            .outgoing_targets_by_kind(annotation, RelationshipKind::Annotation)
            .iter()
            .any(|target| target.id == requirement.id),
        "expected annotation edge from metadata usage to owning requirement"
    );
}

#[test]
fn part_def_metadata_annotation_brace_body_projects_attribute_children() {
    let doc = SysmlDocument::from_memory_path(
        "metadata-part-annotation-body",
        "part_metadata.sysml",
        r#"package P {
  metadata def ApprovalAnnotation {
    attribute approved;
    attribute approver;
  }
  part def Design {
    @ApprovalAnnotation : ApprovalAnnotation {
      approved = true;
      approver = "John";
    }
  }
}"#
        .to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let annotation = graph
        .nodes_named("ApprovalAnnotation")
        .into_iter()
        .find(|node| node.element_kind == "metadata usage")
        .expect("metadata usage");
    let design = graph
        .nodes_named("Design")
        .into_iter()
        .find(|node| node.element_kind == "part def")
        .expect("part def");
    assert_eq!(
        annotation.id.qualified_name,
        format!("{}::ApprovalAnnotation", design.id.qualified_name),
        "part-body metadata usage QN must nest under the owning part def"
    );
    let bindings: Vec<_> = graph
        .children_of(annotation)
        .into_iter()
        .filter(|child| child.element_kind == "attribute")
        .map(|child| child.name.as_str())
        .collect();
    assert!(bindings.contains(&"approved"));
    assert!(bindings.contains(&"approver"));
}

#[test]
fn calc_def_metadata_annotation_qn_nests_under_calc() {
    let doc = SysmlDocument::from_memory_path(
        "metadata-calc",
        "calc_metadata.sysml",
        r#"package P {
  metadata def Tag;
  calc def Score {
    @tag : Tag;
  }
}"#
        .to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let calc = graph
        .nodes_named("Score")
        .into_iter()
        .find(|node| node.element_kind == "calc def")
        .expect("calc def");
    let annotation = graph
        .children_of(calc)
        .into_iter()
        .find(|child| child.element_kind == "metadata usage" && child.name == "tag")
        .expect("metadata usage under calc def");
    assert_eq!(
        annotation.id.qualified_name,
        format!("{}::tag", calc.id.qualified_name),
        "metadata on a calc def must nest under the calc, not the enclosing package"
    );
}

#[test]
fn metadata_usage_about_clause_wires_annotation_edges() {
    let doc = SysmlDocument::from_memory_path(
        "metadata-about",
        "about.sysml",
        r#"package P {
  metadata def Tag;
  part def Target;
  metadata note : Tag about Target;
}"#
        .to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let usage = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.element_kind == "metadata usage" && node.name == "note")
        .expect("metadata usage");
    let target = graph
        .nodes_named("Target")
        .into_iter()
        .find(|node| node.element_kind == "part def")
        .expect("target part def");
    assert!(
        graph
            .outgoing_targets_by_kind(usage, RelationshipKind::Annotation)
            .iter()
            .any(|annotated| annotated.id == target.id),
        "expected annotation edge to about target"
    );
}

#[test]
fn implicit_metadata_annotation_wires_annotation_edge_to_owner() {
    let doc = SysmlDocument::from_memory_path(
        "metadata-implicit-owner",
        "implicit.sysml",
        r#"package P {
  metadata def Tag;
  part def Design {
    @Tag : Tag;
  }
}"#
        .to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let part_def = graph
        .nodes_named("Design")
        .into_iter()
        .find(|node| node.element_kind == "part def")
        .expect("part def");
    let annotation = graph
        .children_of(part_def)
        .into_iter()
        .find(|child| child.element_kind == "metadata usage")
        .expect("metadata usage");
    assert!(
        graph
            .outgoing_targets_by_kind(annotation, RelationshipKind::Annotation)
            .iter()
            .any(|annotated| annotated.id == part_def.id),
        "implicit annotatedElement should be the owning part def"
    );
}
