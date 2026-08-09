//! Parser-backed relationship projection contracts.
//!
//! Explicit relationships are published as edges between resolved semantic nodes. These tests
//! deliberately assert the relationship kind and resolved identities, rather than source-text
//! labels, so authored spelling does not become a competing semantic representation.

use sysml_model::{
    build_semantic_graph_from_documents, ElementKind, NodeId, RelationshipKind, SemanticGraph,
    SysmlDocument, SysmlDocumentSourceKind,
};
use url::Url;

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

fn assert_edge(
    graph: &SemanticGraph,
    uri: &Url,
    source: &str,
    target: &str,
    kind: RelationshipKind,
) {
    let source = NodeId::new(uri, source);
    let target = NodeId::new(uri, target);
    let source_node = graph.get_node(&source).expect("source node");
    let outgoing = graph.outgoing_relationships(source_node);
    assert!(
        outgoing
            .iter()
            .any(|(actual_target, actual_kind)| actual_target.id == target && *actual_kind == kind),
        "expected {kind:?} edge from {source:?} to {target:?}; actual edges: {outgoing:#?}"
    );
}

#[test]
fn definition_subclassification_preserves_named_and_anonymous_authored_sources() {
    let doc = workspace_doc(
        "subclassification.sysml",
        r#"package P {
  part def Base;
  part def Named :> Base;
  part def :> Base;
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    assert_edge(
        &graph,
        &uri,
        "P::Named",
        "P::Base",
        RelationshipKind::Specializes,
    );
    let anonymous = graph
        .nodes_for_uri(&uri)
        .into_iter()
        .find(|node| {
            node.element_kind == ElementKind::PartDef
                && node
                    .attributes
                    .get("isAnonymous")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
        })
        .expect("anonymous definition");
    assert_edge(
        &graph,
        &uri,
        &anonymous.id.qualified_name,
        "P::Base",
        RelationshipKind::Specializes,
    );
}

#[test]
fn feature_typing_subsetting_and_redefinition_share_resolved_edge_projection() {
    let doc = workspace_doc(
        "feature_relationships.sysml",
        r#"package P {
  part def Scalar;
  part def Base {
    attribute subsettable : Scalar;
    attribute redefinable : Scalar;
  }
  part def Derived :> Base {
    attribute narrowed : Scalar :> subsettable;
    attribute replacement : Scalar :>> redefinable;
  }
  part instance : Base;
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    assert_edge(
        &graph,
        &uri,
        "P::instance",
        "P::Base",
        RelationshipKind::Typing,
    );
    assert_edge(
        &graph,
        &uri,
        "P::Derived::narrowed",
        "P::Base::subsettable",
        RelationshipKind::Subsetting,
    );
    assert_edge(
        &graph,
        &uri,
        "P::Derived::replacement",
        "P::Base::redefinable",
        RelationshipKind::Redefinition,
    );
}

#[test]
fn port_conjugation_has_its_own_relationship_kind_and_conjugated_typing_target() {
    let doc = workspace_doc(
        "port_conjugation.sysml",
        r#"package P {
  port def Signal;
  part def Holder {
    port endpoint : ~Signal;
  }
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");

    assert_edge(
        &graph,
        &uri,
        "P::Signal::~Signal",
        "P::Signal",
        RelationshipKind::PortConjugation,
    );
    assert_edge(
        &graph,
        &uri,
        "P::Holder::endpoint",
        "P::Signal::~Signal",
        RelationshipKind::Typing,
    );
}

#[test]
#[ignore = "SKIP: publish standalone specialization and subclassification only after the parser exposes their endpoints as structured relationship facts"]
fn standalone_named_and_anonymous_specialization_and_subclassification_are_projected() {
    let doc = workspace_doc(
        "standalone_specialization.sysml",
        r#"package P {
  specialization Named subtype A specializes B;
  subtype A specializes B;
  specialization Classified subclassifier C specializes D;
  subclassifier C specializes D;
}"#,
    );
    let (graph, _) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    assert!(graph.graph.node_count() > 0, "parser-backed graph");
    panic!(
        "SKIP: preserve each standalone relationship's authored identity and resolved endpoints"
    );
}

#[test]
#[ignore = "SKIP: publish standalone typing, subsetting, redefinition, conjugation, and disjoining only after the parser exposes their endpoints as structured relationship facts"]
fn standalone_named_and_anonymous_relationship_declarations_are_projected() {
    let doc = workspace_doc(
        "standalone_relationships.sysml",
        r#"package P {
  specialization Typed typing customer typed by Person;
  typing customer typed by Person;
  specialization Subset subset rearWheels subsets wheels;
  subset rearWheels :> wheels;
  specialization Redefined redefinition vin redefines identifier;
  redefinition vin redefines identifier;
  conjugation Named conjugate C conjugates O;
  conjugate C conjugates O;
  disjoining Named disjoint A from B;
  disjoint A from B;
}"#,
    );
    let (graph, _) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    assert!(graph.graph.node_count() > 0, "parser-backed graph");
    panic!(
        "SKIP: preserve each standalone relationship's authored identity and resolved endpoints"
    );
}

#[test]
#[ignore = "SKIP: publish feature-inverting and explicit type-featuring only after the parser exposes their endpoints as structured relationship facts"]
fn named_and_anonymous_feature_inverting_and_type_featuring_are_projected() {
    let doc = workspace_doc(
        "feature_inverting_and_featuring.sysml",
        r#"package P {
  feature inverse of inverseTarget;
  inverting Named inverse specific of general;
  feature featured by FeaturingType;
  featuring Named of featuredFeature by FeaturingType;
}"#,
    );
    let (graph, _) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    assert!(graph.graph.node_count() > 0, "parser-backed graph");
    panic!("SKIP: retain authored inversion and featuring facts without inferring a featuring-type closure");
}
