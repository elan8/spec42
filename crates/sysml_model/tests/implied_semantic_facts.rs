//! Implied semantic contracts published separately from their authored source facts.

use sysml_model::{
    build_semantic_graph_from_documents, DeclaredFeatureProperties, DeclaredSemanticFacts,
    ElementKind, ExpressionResultRole, FeatureOwnershipProvenance, ImpliedFeatureOwnership, NodeId,
    SemanticGraph, SemanticNode, SysmlDocument, SysmlDocumentSourceKind, TextPosition, TextRange,
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

#[test]
fn ownership_defaults_are_implied_only_for_supported_usages_in_type_bodies() {
    let document = workspace_doc(
        "implied_ownership_scope.sysml",
        r#"package P {
  part packageMember;
  part def Container {
    part ordinaryPart;
    part outer {
      part nestedPart;
    }
    attribute ordinaryAttribute;
    port ordinaryPort;
    item ordinaryItem;
    action ordinaryAction;
    state ordinaryState;
    occurrence ordinaryOccurrence;
    ref part explicitReference;
    end attribute endFeature;
  }
}"#,
    );
    let (graph, _) = build_semantic_graph_from_documents(&[document.clone()]).expect("graph");

    for name in [
        "ordinaryPart",
        "nestedPart",
        "ordinaryAttribute",
        "ordinaryPort",
        "ordinaryItem",
        "ordinaryAction",
        "ordinaryState",
        "ordinaryOccurrence",
    ] {
        let node = graph
            .nodes_named(name)
            .into_iter()
            .next()
            .unwrap_or_else(|| panic!("missing ordinary usage {name}"));
        let declared = node
            .declared_facts
            .feature_properties
            .as_ref()
            .expect("parser-backed feature properties");
        assert!(
            declared.is_composite.is_none() && declared.is_reference.is_none(),
            "{name} must not turn an implied ownership default into an authored fact"
        );
        assert_eq!(
            graph
                .effective_facts_for(node)
                .and_then(|facts| facts.implied_feature_ownership),
            Some(ImpliedFeatureOwnership {
                is_composite: true,
                is_reference: false,
            }),
            "{name} should receive the supported contextual default"
        );
        assert_eq!(
            graph
                .effective_feature_ownership_for(node)
                .map(|ownership| ownership.provenance),
            Some(FeatureOwnershipProvenance::Implied),
            "{name} must expose the default through the canonical ownership query"
        );
    }

    let explicit_reference = graph
        .nodes_named("explicitReference")
        .into_iter()
        .next()
        .expect("explicit reference usage");
    let declared_reference = explicit_reference
        .declared_facts
        .feature_properties
        .as_ref()
        .expect("parser-backed reference properties");
    assert_eq!(declared_reference.is_composite, Some(false));
    assert_eq!(declared_reference.is_reference, Some(true));
    assert!(
        graph
            .effective_facts_for(explicit_reference)
            .and_then(|facts| facts.implied_feature_ownership)
            .is_none(),
        "an explicit reference must not also receive an implied ownership default"
    );
    assert_eq!(
        graph
            .effective_feature_ownership_for(explicit_reference)
            .map(|ownership| ownership.provenance),
        Some(FeatureOwnershipProvenance::Authored)
    );

    for name in ["packageMember", "endFeature"] {
        let node = graph
            .nodes_named(name)
            .into_iter()
            .next()
            .unwrap_or_else(|| panic!("missing excluded usage {name}"));
        assert!(
            graph.effective_feature_ownership_for(node).is_none(),
            "{name} must not receive the ownership default outside its typed context"
        );
    }
}

#[test]
fn directed_feature_does_not_receive_the_ownership_default() {
    let uri = Url::parse("memory://ownership/directed.sysml").expect("URI");
    let owner_id = NodeId::new(&uri, "P::Container");
    let feature_id = NodeId::new(&uri, "P::Container::input");
    let range = TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 0));
    let mut graph = SemanticGraph::new();
    graph.insert_workspace_node(SemanticNode {
        id: owner_id.clone(),
        element_kind: ElementKind::PartDef,
        declared_name: Some("Container".into()),
        name: "Container".into(),
        range,
        attributes: Default::default(),
        declared_facts: Default::default(),
        parent_id: None,
    });
    graph.insert_workspace_node(SemanticNode {
        id: feature_id.clone(),
        element_kind: ElementKind::Attribute,
        declared_name: Some("input".into()),
        name: "input".into(),
        range,
        attributes: Default::default(),
        declared_facts: DeclaredSemanticFacts {
            feature_properties: Some(DeclaredFeatureProperties {
                direction: Some("in".into()),
                ..Default::default()
            }),
            ..Default::default()
        },
        parent_id: Some(owner_id),
    });
    graph.refresh_effective_facts();

    let feature = graph.get_node(&feature_id).expect("directed feature");
    assert!(
        graph.effective_feature_ownership_for(feature).is_none(),
        "directed parameters are outside the composite ownership default"
    );
}

#[test]
fn nested_usage_without_multiplicity_publishes_implied_exactly_one() {
    let document = workspace_doc(
        "default_multiplicity.sysml",
        r#"package P {
  part def Vehicle {
    part wheel;
  }
}"#,
    );
    let (graph, _) = build_semantic_graph_from_documents(&[document]).expect("graph");
    let wheel = graph
        .nodes_named("wheel")
        .into_iter()
        .find(|node| node.element_kind == ElementKind::Part)
        .expect("nested part usage");

    assert!(
        wheel.declared_facts.multiplicity.is_none(),
        "the parser-backed fact must remain absent when no multiplicity was authored"
    );
    let multiplicity = graph
        .effective_facts_for(wheel)
        .and_then(|facts| facts.implied_multiplicity)
        .expect("the effective multiplicity fact");
    assert_eq!(multiplicity.lower, 1);
    assert_eq!(multiplicity.upper, Some(1));
    assert!(!multiplicity.is_ordered);
}

#[test]
fn only_nested_ordinary_part_attribute_and_port_usages_receive_implied_multiplicity() {
    let document = workspace_doc(
        "implied_multiplicity_scope.sysml",
        r#"package P {
  part packagePart;
  attribute packageAttribute;
  port packagePort;
  connection packageConnection;

  part def Container {
    part nestedPart;
    attribute nestedAttribute;
    port nestedPort;
    part base;
    part subsetPart subsets base;
    part authoredPart[0..*];
  }
}"#,
    );
    let (graph, _) = build_semantic_graph_from_documents(&[document]).expect("graph");

    for name in ["nestedPart", "nestedAttribute", "nestedPort"] {
        let node = graph
            .nodes_named(name)
            .into_iter()
            .next()
            .unwrap_or_else(|| panic!("missing nested usage {name}"));
        assert_eq!(
            graph
                .effective_facts_for(node)
                .and_then(|facts| facts.implied_multiplicity),
            Some(sysml_model::ImpliedMultiplicity {
                lower: 1,
                upper: Some(1),
                is_ordered: false,
                is_unique: None,
            }),
            "{name} should receive the owned-usage default"
        );
    }

    for name in [
        "packagePart",
        "packageAttribute",
        "packagePort",
        "packageConnection",
        "subsetPart",
        "authoredPart",
    ] {
        let node = graph
            .nodes_named(name)
            .into_iter()
            .next()
            .unwrap_or_else(|| panic!("missing excluded usage {name}"));
        assert!(
            graph
                .effective_facts_for(node)
                .and_then(|facts| facts.implied_multiplicity)
                .is_none(),
            "{name} must not receive the ordinary owned-usage default"
        );
    }
}

#[test]
fn feature_nested_in_a_usage_publishes_its_nearest_featuring_type() {
    let document = workspace_doc(
        "nested_featuring.sysml",
        r#"package P {
  part def Vehicle {
    part engine {
      attribute rpm;
    }
  }
}"#,
    );
    let (graph, _) = build_semantic_graph_from_documents(&[document]).expect("graph");
    let rpm = graph
        .nodes_named("rpm")
        .into_iter()
        .find(|node| node.element_kind == ElementKind::Attribute)
        .expect("nested feature");

    let featuring_type = graph
        .effective_facts_for(rpm)
        .and_then(|facts| facts.featuring_type.as_ref())
        .expect("resolved featuring type");
    assert_eq!(featuring_type.qualified_name, "P::Vehicle");
    assert_eq!(
        graph
            .get_node(featuring_type)
            .map(|node| &node.element_kind),
        Some(&ElementKind::PartDef)
    );
}

#[test]
fn bound_feature_value_publishes_an_implied_binding_to_its_expression_result() {
    let document = workspace_doc(
        "bound_feature_value.sysml",
        r#"package P {
  part def Vehicle {
    attribute speed = 5;
  }
}"#,
    );
    let (graph, _) = build_semantic_graph_from_documents(&[document]).expect("graph");
    let speed = graph
        .nodes_named("speed")
        .into_iter()
        .find(|node| node.element_kind == ElementKind::Attribute)
        .expect("bound attribute usage");

    assert!(
        speed.declared_facts.feature_value.is_some(),
        "the parser-backed bound feature value is available to the semantic graph"
    );
    let binding = graph
        .effective_facts_for(speed)
        .and_then(|facts| facts.implied_feature_value_binding.as_ref())
        .expect("implied feature-value binding");
    assert_eq!(binding.expression_result.owner_id, speed.id);
    assert_eq!(
        binding.expression_result.role,
        ExpressionResultRole::FeatureValue
    );
}
