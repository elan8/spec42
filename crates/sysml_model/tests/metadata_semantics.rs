use sysml_diagnostics::{collect_diagnostics_from_graph, DiagnosticsOptions};
use sysml_model::{
    build_semantic_graph_from_documents, RelationshipKind, SysmlDocument, SysmlDocumentSourceKind,
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
        .filter(|child| child.element_kind == "attribute")
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
            .children_of(requirement)
            .iter()
            .any(|child| child.element_kind == "metadata usage" && child.name == "reviewTag"),
        "expected metadata usage under requirement def body"
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
fn metadata_keyword_usage_resolves_with_typing_edge() {
    let doc = SysmlDocument::from_memory_path(
        "metadata-keyword-typed",
        "keyword.sysml",
        include_str!("fixtures/parser_wave/metadata-keyword-usage.sysml").to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let part_def = graph
        .nodes_named("Widget")
        .into_iter()
        .find(|node| node.element_kind == "part def")
        .expect("part def");
    let keyword = graph
        .children_of(part_def)
        .into_iter()
        .find(|child| child.element_kind == "metadata keyword")
        .expect("metadata keyword");
    assert_eq!(
        keyword.attributes.get("keyword").and_then(|v| v.as_str()),
        Some("Tag")
    );
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        !diagnostics
            .iter()
            .any(|diag| diag.code == "metadata_keyword_unresolved"),
        "resolved keyword should not be unresolved"
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

const REQUIREMENT_METADATA_SYSML: &str = r#"
package RequirementMetadata {
  enum def RequirementRoleKind {
    enum user;
    enum system;
  }

  metadata def RequirementRole {
    :> annotatedElement : SysML::RequirementUsage;
    attribute role : RequirementRoleKind;
  }

  metadata def RequirementIdentity {
    :> annotatedElement : SysML::RequirementUsage;
    attribute requirementId;
  }

  metadata def <user> UserRequirementRole :> SemanticMetadata {
    :> annotatedElement : SysML::RequirementUsage;
    :>> baseType = requirementChecks meta SysML::Usage;
  }

  metadata def <system> SystemRequirementRole :> SemanticMetadata {
    :> annotatedElement : SysML::RequirementUsage;
    :>> baseType = requirementChecks meta SysML::Usage;
  }
}
"#;

#[test]
fn requirement_metadata_def_shorthand_projects_restriction_attributes() {
    let doc = SysmlDocument::from_memory_path(
        "requirement-metadata-def",
        "RequirementMetadata.sysml",
        REQUIREMENT_METADATA_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    let uri = doc.uri.clone();
    let (graph, _parsed) =
        build_semantic_graph_from_documents(&[doc]).expect("semantic graph should build");

    let requirement_role = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.element_kind == "metadata def" && node.name == "RequirementRole")
        .expect("RequirementRole metadata def");
    let annotated_element = graph
        .children_of(requirement_role)
        .into_iter()
        .find(|child| child.element_kind == "attribute" && child.name == "annotatedElement")
        .expect("annotatedElement restriction attribute");
    assert_eq!(
        annotated_element
            .attributes
            .get("subsetsFeature")
            .and_then(|v| v.as_str()),
        Some("annotatedElement")
    );
    assert!(
        annotated_element
            .attributes
            .get("attributeType")
            .and_then(|v| v.as_str())
            .is_some_and(|t| t.contains("RequirementUsage")),
        "expected RequirementUsage typing on annotatedElement restriction"
    );

    let user_role = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.element_kind == "metadata def" && node.name == "UserRequirementRole")
        .expect("UserRequirementRole metadata def");
    let base_type = graph
        .children_of(user_role)
        .into_iter()
        .find(|child| child.element_kind == "attribute" && child.name == "baseType")
        .expect("baseType restriction attribute");
    assert_eq!(
        base_type
            .attributes
            .get("redefines")
            .and_then(|v| v.as_str()),
        Some("baseType")
    );
    assert!(
        base_type
            .attributes
            .get("value")
            .and_then(|v| v.as_str())
            .is_some_and(|v| v.contains("meta SysML::Usage")),
        "expected meta cast value on baseType restriction"
    );

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    let parser_errors: Vec<_> = diagnostics
        .iter()
        .filter(|diag| diag.source == "parser" && diag.code.contains("recovered"))
        .collect();
    assert!(
        parser_errors.is_empty(),
        "unexpected parser recovery diagnostics: {parser_errors:?}"
    );
}

#[test]
fn semantic_metadata_restriction_allows_compatible_requirement_annotation() {
    let doc = SysmlDocument::from_memory_path(
        "semantic-metadata-requirement-annotation",
        "semantic_metadata.sysml",
        r#"package P {
  metadata def SemanticMetadata {
    attribute annotatedElement;
    attribute baseType;
  }

  metadata def UserRequirementRole :> SemanticMetadata {
    :> annotatedElement : SysML::RequirementUsage;
    :>> baseType = requirementChecks meta SysML::Usage;
  }

  requirement def R1 {
    @UserRequirementRole : UserRequirementRole;
  }
}"#
        .to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    let uri = doc.uri.clone();
    let (graph, _parsed) =
        build_semantic_graph_from_documents(&[doc]).expect("semantic graph should build");

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        !diagnostics
            .iter()
            .any(|diag| diag.code == "metadata_annotated_element_incompatible"),
        "UserRequirementRole on requirement def should satisfy annotatedElement restriction"
    );
}

#[test]
fn metadata_redefine_shorthand_projects_subsets_feature_for_annotated_element() {
    let doc = SysmlDocument::from_memory_path(
        "metadata-redefine-shorthand",
        "redefine.sysml",
        r#"package P {
  metadata def Role {
    :>> annotatedElement : SysML::RequirementUsage;
  }
}"#
        .to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    let uri = doc.uri.clone();
    let sysml = SysmlDocument::from_memory_path(
        "metadata-redefine-shorthand-lib",
        "SysML.sysml",
        r#"standard library package SysML {
  public import Systems::*;
  package Systems {
    metadata def RequirementUsage;
  }
}"#
        .to_string(),
        SysmlDocumentSourceKind::Library,
        None,
        None,
    )
    .expect("library doc");
    let (graph, _parsed) =
        build_semantic_graph_from_documents(&[doc, sysml]).expect("semantic graph should build");

    let role = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.element_kind == "metadata def" && node.name == "Role")
        .expect("Role metadata def");
    let annotated = graph
        .children_of(role)
        .into_iter()
        .find(|child| child.name == "annotatedElement")
        .expect("annotatedElement");
    assert_eq!(
        annotated
            .attributes
            .get("subsetsFeature")
            .and_then(|value| value.as_str()),
        Some("annotatedElement")
    );
    assert_eq!(
        annotated
            .attributes
            .get("redefines")
            .and_then(|value| value.as_str()),
        Some("annotatedElement")
    );

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert!(
        !diagnostics
            .iter()
            .any(|diag| diag.code == "incompatible_type_kind"),
        "unexpected incompatible_type_kind: {diagnostics:?}"
    );
}
