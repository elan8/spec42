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
        !graph.outgoing_typing_or_specializes_targets(user_role).is_empty(),
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
        graph.graph.edge_weight(edge_index).is_some_and(|edge| {
            edge.kind == RelationshipKind::Derivation
        })
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
        &["invalid_qualified_name_segment", "unresolved_type_reference"],
        "SysML sibling import",
    );
}
