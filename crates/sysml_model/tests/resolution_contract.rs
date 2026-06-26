use semantic_core::{
    build_semantic_graph_from_documents, collect_diagnostics_from_graph, DiagnosticsOptions,
    RelationshipKind, SysmlDocument, SysmlDocumentSourceKind,
};

fn workspace_doc(path: &str, content: &str) -> SysmlDocument {
    SysmlDocument::from_memory_path(
        "resolution-contract",
        path,
        content.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("workspace document")
}

fn library_doc(path: &str, content: &str) -> SysmlDocument {
    SysmlDocument::from_memory_path(
        "resolution-contract-lib",
        path,
        content.to_string(),
        SysmlDocumentSourceKind::Library,
        None,
        None,
    )
    .expect("library document")
}

fn semantic_codes(diagnostics: &[semantic_core::SemanticDiagnostic]) -> Vec<&str> {
    diagnostics
        .iter()
        .filter(|diag| diag.source == "semantic")
        .map(|diag| diag.code.as_str())
        .collect()
}

fn assert_no_semantic_codes(
    diagnostics: &[semantic_core::SemanticDiagnostic],
    forbidden: &[&str],
    context: &str,
) {
    let codes = semantic_codes(diagnostics);
    for code in forbidden {
        assert!(
            !codes.contains(code),
            "{context}: unexpected {code}, got {codes:?}"
        );
    }
}

#[test]
fn contract_sysml_qualified_metadata_restrictions_resolve_without_warnings() {
    let sysml = library_doc(
        "SysML.sysml",
        r#"standard library package SysML {
  public import Systems::*;
  package Systems {
    metadata def RequirementUsage;
    metadata def Usage;
  }
}"#,
    );
    let metaobjects = library_doc(
        "Metaobjects.kerml",
        r#"standard library package Metaobjects {
  abstract metaclass SemanticMetadata {
    feature baseType;
  }
}"#,
    );
    let workspace = workspace_doc(
        "RequirementMetadata.sysml",
        r#"package RequirementMetadata {
  private import Metaobjects::SemanticMetadata;

  metadata def <user> UserRequirementRole :> SemanticMetadata {
    :> annotatedElement : SysML::RequirementUsage;
    :>> baseType = requirementChecks meta SysML::Usage;
  }
}"#,
    );
    let uri = workspace.uri.clone();
    let (graph, _parsed) =
        build_semantic_graph_from_documents(&[workspace, sysml, metaobjects]).expect("graph");

    let user_role = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.element_kind == "metadata def" && node.name == "UserRequirementRole")
        .expect("UserRequirementRole");

    assert!(
        !graph
            .outgoing_typing_or_specializes_targets(user_role)
            .is_empty(),
        "expected specializes edge on UserRequirementRole"
    );

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert_no_semantic_codes(
        &diagnostics,
        &[
            "invalid_qualified_name_segment",
            "unresolved_type_reference",
            "incompatible_specializes_kind",
            "incompatible_type_kind",
            "unresolved_redefines_target",
        ],
        "RequirementMetadata contract",
    );
}

#[test]
fn contract_derivation_connection_has_derivation_edge_not_connection_context_invalid() {
    let doc = workspace_doc(
        "derivation.sysml",
        r#"package Demo {
  requirement def OriginalReq;
  requirement def DerivedReq;
  requirement original : OriginalReq;
  requirement derived : DerivedReq;
  #derivation connection {
    end #original ::> original;
    end #derive ::> derived;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");

    let has_derivation = graph.graph.edge_indices().any(|edge_index| {
        graph
            .graph
            .edge_weight(edge_index)
            .is_some_and(|edge| edge.kind == RelationshipKind::Derivation)
    });
    assert!(has_derivation, "expected Derivation edge");

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert_no_semantic_codes(
        &diagnostics,
        &["connection_context_invalid"],
        "derivation connection",
    );
}

#[test]
fn contract_satisfy_requirement_by_part_is_valid() {
    let doc = workspace_doc(
        "satisfy.sysml",
        r#"package Demo {
  requirement def ReqA;
  part def System;
  requirement r1 : ReqA;
  part system : System;
  satisfy r1 by system;
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert_no_semantic_codes(
        &diagnostics,
        &["satisfy_invalid_endpoint_kind"],
        "satisfy by part",
    );
}

#[test]
fn contract_sibling_import_resolves_sysml_requirement_usage() {
    let sysml = library_doc(
        "SysML.sysml",
        r#"standard library package SysML {
  public import Systems::*;
  package Systems {
    metadata def RequirementUsage;
  }
}"#,
    );
    let workspace = workspace_doc(
        "UsesSysML.sysml",
        r#"package UsesSysML {
  metadata def Role {
    :> annotatedElement : SysML::RequirementUsage;
  }
}"#,
    );
    let uri = workspace.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[workspace, sysml]).expect("graph");
    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert_no_semantic_codes(
        &diagnostics,
        &[
            "invalid_qualified_name_segment",
            "unresolved_type_reference",
        ],
        "SysML sibling import",
    );
}

#[test]
fn contract_metadata_redefine_shorthand_annotated_element_no_incompatible_type_kind() {
    let sysml = library_doc(
        "SysML.sysml",
        r#"standard library package SysML {
  public import Systems::*;
  package Systems {
    metadata def RequirementUsage;
  }
}"#,
    );
    let workspace = workspace_doc(
        "RedefineAnnotatedElement.sysml",
        r#"package Demo {
  metadata def Role {
    :>> annotatedElement : SysML::RequirementUsage;
  }
}"#,
    );
    let uri = workspace.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[workspace, sysml]).expect("graph");

    let role = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.element_kind == "metadata def" && node.name == "Role")
        .expect("Role metadata def");
    let annotated = graph
        .children_of(role)
        .into_iter()
        .find(|child| child.name == "annotatedElement")
        .expect("annotatedElement attribute");
    assert!(
        annotated
            .attributes
            .get("subsetsFeature")
            .and_then(|value| value.as_str())
            == Some("annotatedElement"),
        "expected subsetsFeature projection for :>> annotatedElement"
    );

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert_no_semantic_codes(
        &diagnostics,
        &["incompatible_type_kind", "unresolved_type_reference"],
        ":>> annotatedElement",
    );
}

#[test]
fn contract_omg_style_fmea_metadata_block_no_metadata_typing_warnings() {
    let sysml = library_doc(
        "SysML.sysml",
        r#"standard library package SysML {
  public import Systems::*;
  package Systems {
    metadata def RequirementUsage;
    metadata def Usage;
  }
}"#,
    );
    let metaobjects = library_doc(
        "Metaobjects.kerml",
        r#"standard library package Metaobjects {
  abstract metaclass SemanticMetadata {
    feature baseType;
    feature annotatedElement;
  }
}"#,
    );
    let workspace = workspace_doc(
        "FMEAMetadata.sysml",
        include_str!("fixtures/stdlib/omg_14c_metadata_slice.sysml"),
    );
    let uri = workspace.uri.clone();
    let (graph, _parsed) =
        build_semantic_graph_from_documents(&[workspace, sysml, metaobjects]).expect("graph");

    let semantic_metadata = graph.nodes_by_uri.values().flatten().find_map(|id| {
        graph.get_node(id).filter(|node| {
            node.name == "SemanticMetadata"
                && node.element_kind == "metadata def"
                && node
                    .attributes
                    .get("metaclassRole")
                    .and_then(|value| value.as_str())
                    == Some("SemanticMetadata")
        })
    });
    assert!(
        semantic_metadata.is_some(),
        "expected SemanticMetadata as metadata def with metaclassRole"
    );

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert_no_semantic_codes(
        &diagnostics,
        &[
            "incompatible_type_kind",
            "incompatible_specializes_kind",
            "unresolved_redefines_target",
        ],
        "OMG-style FMEA metadata slice",
    );
}

#[test]
fn contract_imported_semantic_metadata_specializes_without_warnings() {
    let sysml = library_doc(
        "SysML.sysml",
        r#"standard library package SysML {
  public import Systems::*;
  package Systems {
    metadata def Usage;
  }
}"#,
    );
    let metaobjects = library_doc(
        "Metaobjects.kerml",
        r#"standard library package Metaobjects {
  abstract metaclass SemanticMetadata {
    feature baseType;
  }
}"#,
    );
    let workspace = workspace_doc(
        "Profile.sysml",
        r#"package Profile {
  private import Metaobjects::SemanticMetadata;

  metadata def UserRole :> SemanticMetadata {
    :>> baseType = requirementChecks meta SysML::Usage;
  }
}"#,
    );
    let uri = workspace.uri.clone();
    let (graph, _parsed) =
        build_semantic_graph_from_documents(&[workspace, sysml, metaobjects]).expect("graph");

    let user_role = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| node.element_kind == "metadata def" && node.name == "UserRole")
        .expect("UserRole");

    assert!(
        !graph
            .outgoing_typing_or_specializes_targets(user_role)
            .is_empty(),
        "expected specializes edge on UserRole"
    );

    let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
    assert_no_semantic_codes(
        &diagnostics,
        &[
            "incompatible_specializes_kind",
            "incompatible_type_kind",
            "unresolved_redefines_target",
        ],
        "imported SemanticMetadata profile",
    );
}
