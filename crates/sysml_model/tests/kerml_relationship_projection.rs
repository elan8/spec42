use sysml_model::{
    build_semantic_graph_from_documents, patch_graph_for_document, ElementKind, NodeId,
    RelationshipKind, SemanticGraph, SysmlDocument, SysmlDocumentSourceKind,
};
use sysml_v2_parser::ast::{PackageBody, PackageBodyElement, RootElement};
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

fn assert_cross_document_edge(
    graph: &SemanticGraph,
    source_uri: &Url,
    source: &str,
    target_uri: &Url,
    target: &str,
    kind: RelationshipKind,
) {
    let source = NodeId::new(source_uri, source);
    let target = NodeId::new(target_uri, target);
    let source_node = graph.get_node(&source).expect("source node");
    assert!(
        graph
            .outgoing_relationships(source_node)
            .iter()
            .any(|(actual_target, actual_kind)| actual_target.id == target && *actual_kind == kind),
        "expected {kind:?} edge from {source:?} to {target:?}"
    );
}

/// The parser's recovery result is the current syntax boundary for relationship declarations
/// without an AST variant. Keep this assertion narrow so a parser addition forces this test to
/// become a semantic-projection test.
fn assert_unmodeled_package_relationship_declaration(source: &str, declaration_keyword: &str) {
    let parsed = sysml_v2_parser::parse_for_editor(source);
    assert_eq!(
        parsed.errors.len(),
        1,
        "the parser boundary must remain a single recovery diagnostic: {source}"
    );
    let error = &parsed.errors[0];
    assert_eq!(
        error.code.as_deref(),
        Some("unrecognized_declaration_in_scope"),
        "unexpected parser boundary for {source}"
    );
    assert!(
        error
            .found
            .as_deref()
            .is_some_and(|found| found.starts_with(declaration_keyword)),
        "the recovery diagnostic must identify the unsupported declaration: {error:#?}"
    );
    assert!(
        sysml_v2_parser::parse(source).is_err(),
        "strict parsing must reject an unmodeled standalone relationship declaration"
    );
}

/// Some KerML declarations are preserved by the parser only as source-fidelity fallback nodes.
/// Their public AST contract has no typed relationship endpoint, span, or authored identity.
fn assert_raw_package_declaration(
    source: &str,
    expected_keyword: &str,
    expected_text_fragment: &str,
    expected_variant: fn(&PackageBodyElement) -> Option<(&str, &str)>,
) {
    let root = sysml_v2_parser::parse(source).expect("raw declaration remains strictly parseable");
    let RootElement::Package(package) = &root.elements[0].value else {
        panic!("expected a package root");
    };
    let PackageBody::Brace { elements } = &package.value.body else {
        panic!("expected package members");
    };
    assert_eq!(elements.len(), 1, "expected exactly one raw declaration");
    let (keyword, text) = expected_variant(&elements[0].value)
        .expect("expected the parser's raw declaration fallback variant");
    assert_eq!(keyword, expected_keyword);
    assert!(
        text.contains(expected_text_fragment),
        "raw source fidelity must retain the authored spelling"
    );
}

fn raw_feature_declaration(element: &PackageBodyElement) -> Option<(&str, &str)> {
    let PackageBodyElement::FeatureDecl(declaration) = element else {
        return None;
    };
    Some((&declaration.value.keyword, &declaration.value.text))
}

fn raw_classifier_declaration(element: &PackageBodyElement) -> Option<(&str, &str)> {
    let PackageBodyElement::ClassifierDecl(declaration) = element else {
        return None;
    };
    Some((&declaration.value.keyword, &declaration.value.text))
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

    let narrowed = graph
        .get_node(&NodeId::new(&uri, "P::Derived::narrowed"))
        .expect("narrowed feature");
    assert_eq!(
        narrowed
            .declared_facts
            .relationships
            .subsetting
            .iter()
            .map(|target| target.reference.as_str())
            .collect::<Vec<_>>(),
        ["subsettable"],
        "the parser-authored target, not its display attribute, owns subsetting"
    );
    let replacement = graph
        .get_node(&NodeId::new(&uri, "P::Derived::replacement"))
        .expect("replacement feature");
    assert_eq!(
        replacement
            .declared_facts
            .relationships
            .redefinition
            .iter()
            .map(|target| target.reference.as_str())
            .collect::<Vec<_>>(),
        ["redefinable"],
        "the authored redefinition remains separate from its resolved edge"
    );
}

#[test]
fn cross_document_subsetting_family_uses_declared_facts_after_incremental_relink() {
    let definitions = workspace_doc(
        "shared.sysml",
        r#"package Shared {
  part def Scalar;
  part def Base {
    attribute member : Scalar;
  }
}"#,
    );
    let usage = workspace_doc(
        "usage.sysml",
        r#"package Usage {
  import Shared::*;
  part def Derived :> Base {
    attribute subset : Scalar :> member;
    attribute redefine : Scalar :>> member;
    attribute reference : Scalar references member;
    attribute cross : Scalar crosses member;
    attribute unresolved : Missing;
  }
}"#,
    );
    let usage_uri = usage.uri.clone();
    let definitions_uri = definitions.uri.clone();
    let (mut graph, _) =
        build_semantic_graph_from_documents(&[definitions.clone(), usage.clone()]).expect("graph");

    for (source, kind) in [
        ("Usage::Derived::subset", RelationshipKind::Subsetting),
        ("Usage::Derived::redefine", RelationshipKind::Redefinition),
        (
            "Usage::Derived::reference",
            RelationshipKind::ReferenceSubsetting,
        ),
        ("Usage::Derived::cross", RelationshipKind::CrossSubsetting),
    ] {
        assert_cross_document_edge(
            &graph,
            &usage_uri,
            source,
            &definitions_uri,
            "Shared::Base::member",
            kind,
        );
    }
    let unresolved = graph
        .get_node(&NodeId::new(&usage_uri, "Usage::Derived::unresolved"))
        .expect("unresolved node");
    assert_eq!(
        unresolved.declared_facts.relationships.typing[0].reference, "Missing",
        "unresolved parser-authored relationships remain explicit facts"
    );
    assert!(
        graph
            .outgoing_targets_by_kind(unresolved, RelationshipKind::Typing)
            .is_empty(),
        "an unresolved fact must not masquerade as a resolved edge"
    );

    let parsed = sysml_v2_parser::parse_for_editor(&usage.content).root;
    patch_graph_for_document(&mut graph, &usage_uri, Some(&parsed), true);
    for (source, kind) in [
        ("Usage::Derived::subset", RelationshipKind::Subsetting),
        ("Usage::Derived::redefine", RelationshipKind::Redefinition),
        (
            "Usage::Derived::reference",
            RelationshipKind::ReferenceSubsetting,
        ),
        ("Usage::Derived::cross", RelationshipKind::CrossSubsetting),
    ] {
        assert_cross_document_edge(
            &graph,
            &usage_uri,
            source,
            &definitions_uri,
            "Shared::Base::member",
            kind,
        );
    }
}

#[test]
fn duplicate_authored_targets_retain_order_spans_while_edges_deduplicate() {
    let doc = workspace_doc(
        "duplicate_targets.sysml",
        r#"package P {
  part def Base;
  part def Derived :> Base, Base;
}"#,
    );
    let uri = doc.uri.clone();
    let (graph, _) = build_semantic_graph_from_documents(&[doc]).expect("semantic graph");
    let derived = graph
        .get_node(&NodeId::new(&uri, "P::Derived"))
        .expect("Derived definition");
    let targets = &derived.declared_facts.relationships.specializes;
    assert_eq!(
        targets.len(),
        2,
        "every authored target must remain observable"
    );
    assert_eq!(targets[0].reference, "Base");
    assert_eq!(targets[1].reference, "Base");
    assert!(targets.iter().all(|target| target.range.is_some()));
    assert_eq!(
        graph
            .outgoing_targets_by_kind(derived, RelationshipKind::Specializes)
            .len(),
        1,
        "edge publication remains deduplicated independently of authored facts"
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
fn standalone_specialization_and_subclassification_remain_untyped_parser_boundaries() {
    for (declaration, keyword) in [
        (
            "specialization Named subtype A specializes B;",
            "specialization",
        ),
        ("subtype A specializes B;", "subtype"),
        (
            "specialization Classified subclassifier C specializes D;",
            "specialization",
        ),
    ] {
        assert_unmodeled_package_relationship_declaration(
            &format!("package P {{ {declaration} }}"),
            keyword,
        );
    }
    assert_raw_package_declaration(
        "package P { subclassifier C specializes D; }",
        "subclassifier",
        "specializes D",
        raw_classifier_declaration,
    );
}

#[test]
fn standalone_relationship_declarations_remain_untyped_parser_boundaries() {
    for (declaration, keyword) in [
        (
            "specialization Typed typing customer typed by Person;",
            "specialization",
        ),
        ("typing customer typed by Person;", "typing"),
        (
            "specialization Subset subset rearWheels subsets wheels;",
            "specialization",
        ),
        ("subset rearWheels :> wheels;", "subset"),
        (
            "specialization Redefined redefinition vin redefines identifier;",
            "specialization",
        ),
        ("redefinition vin redefines identifier;", "redefinition"),
        ("conjugation Named conjugate C conjugates O;", "conjugation"),
        ("conjugate C conjugates O;", "conjugate"),
        ("disjoining Named disjoint A from B;", "disjoining"),
        ("disjoint A from B;", "disjoint"),
    ] {
        assert_unmodeled_package_relationship_declaration(
            &format!("package P {{ {declaration} }}"),
            keyword,
        );
    }
}

#[test]
fn feature_inverting_and_type_featuring_remain_untyped_parser_boundaries() {
    assert_raw_package_declaration(
        "package P { feature inverse of inverseTarget; }",
        "feature",
        "of inverseTarget",
        raw_feature_declaration,
    );
    assert_raw_package_declaration(
        "package P { feature featured by FeaturingType; }",
        "feature",
        "by FeaturingType",
        raw_feature_declaration,
    );
    assert_unmodeled_package_relationship_declaration(
        "package P { inverting Named inverse specific of general; }",
        "inverting",
    );
    assert_unmodeled_package_relationship_declaration(
        "package P { featuring Named of featuredFeature by FeaturingType; }",
        "featuring",
    );
}
