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
    let (graph, _) =
        build_semantic_graph_from_documents(std::slice::from_ref(&document)).expect("graph");

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
        source_text: Default::default(),
        expression_text: Default::default(),
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
        source_text: Default::default(),
        expression_text: Default::default(),
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
fn ownership_kind_predicates_are_typed_and_exclude_non_feature_contexts() {
    for usage_kind in [
        ElementKind::Interface,
        ElementKind::Enumeration,
        ElementKind::Perform,
        ElementKind::ViewRendering,
        ElementKind::Objective,
        ElementKind::AssertConstraint,
        ElementKind::RequireConstraint,
    ] {
        assert!(
            usage_kind.is_composite_by_default_usage(),
            "{usage_kind} must retain its explicit composite-by-default classification"
        );
    }

    for non_feature_context in [
        ElementKind::Package,
        ElementKind::InterfaceEnd,
        ElementKind::InOutParameter,
    ] {
        assert!(
            !non_feature_context.is_composite_by_default_usage(),
            "{non_feature_context} must not receive the composite ownership default"
        );
        assert!(
            !non_feature_context.is_type_context(),
            "{non_feature_context} must not make its members eligible for the default"
        );
    }
}

#[test]
#[ignore = "SKIP: interface usage is not yet materialized and enumeration usage does not yet publish parser-backed DeclaredFeatureProperties, so ownership cannot safely distinguish authored ref/end/direction facts"]
fn interface_and_enumeration_ownership_defaults_require_typed_builder_facts() {
    let document = workspace_doc(
        "future_interface_enumeration_ownership.sysml",
        r#"package P {
  part def Container {
    interface nestedInterface;
    enum nestedEnumeration;
  }
}"#,
    );
    let (graph, _) = build_semantic_graph_from_documents(&[document]).expect("graph");

    let enumeration = graph
        .nodes_named("nestedEnumeration")
        .into_iter()
        .find(|node| node.element_kind == ElementKind::Enumeration)
        .expect("parser-backed enumeration usage");
    assert_future_implied_ownership(&graph, enumeration, "enumeration usage");

    let interface = graph
        .nodes_named("nestedInterface")
        .into_iter()
        .find(|node| node.element_kind == ElementKind::Interface)
        .expect("parser-backed interface usage");
    assert_future_implied_ownership(&graph, interface, "interface usage");
}

#[test]
#[ignore = "SKIP: objective, assert-constraint, and require-constraint builders retain only legacy attributes; typed declared feature properties are required before ownership defaults can be resolved"]
fn objective_and_constraint_ownership_defaults_require_typed_builder_facts() {
    let document = workspace_doc(
        "future_objective_constraint_ownership.sysml",
        r#"package P {
  requirement def Required {
    require constraint { true; }
  }
  occurrence def Checked {
    assert constraint { true; }
  }
  verification def Verification {
    objective verificationObjective {
      verify requirement Required;
    }
  }
}"#,
    );
    let (graph, _) = build_semantic_graph_from_documents(&[document]).expect("graph");

    for (name, element_kind, description) in [
        ("verificationObjective", ElementKind::Objective, "objective"),
        (
            "_assertConstraint_0",
            ElementKind::AssertConstraint,
            "assert constraint",
        ),
        (
            "_requireConstraint_0",
            ElementKind::RequireConstraint,
            "require constraint",
        ),
    ] {
        let node = graph
            .nodes_named(name)
            .into_iter()
            .find(|node| node.element_kind == element_kind)
            .unwrap_or_else(|| panic!("parser-backed {description} usage"));
        assert_future_implied_ownership(&graph, node, description);
    }
}

#[test]
#[ignore = "SKIP: performed-action and view-rendering builders do not yet publish parser-backed DeclaredFeatureProperties, so ownership cannot safely distinguish authored ref/end/direction facts"]
fn perform_and_view_rendering_ownership_defaults_require_typed_builder_facts() {
    let document = workspace_doc(
        "future_perform_rendering_ownership.sysml",
        r#"package P {
  part def Container {
    perform action nestedPerformedAction;
  }
  action def Workflow {
    perform action nestedPerformStep;
  }
  view def Dashboard {
    render nestedRendering;
  }
}"#,
    );
    let (graph, _) = build_semantic_graph_from_documents(&[document]).expect("graph");

    let perform_step = graph
        .nodes_named("nestedPerformStep")
        .into_iter()
        .find(|node| node.element_kind == ElementKind::Perform)
        .expect("parser-backed perform step");
    assert_future_implied_ownership(&graph, perform_step, "perform step");

    let performed_action = graph
        .nodes_named("nestedPerformedAction")
        .into_iter()
        .find(|node| node.element_kind == ElementKind::Action)
        .expect("parser-backed performed action usage");
    assert_future_implied_ownership(&graph, performed_action, "performed action usage");

    let view_rendering = graph
        .nodes_named("nestedRendering")
        .into_iter()
        .find(|node| node.element_kind == ElementKind::ViewRendering)
        .expect("parser-backed view rendering usage");
    assert_future_implied_ownership(&graph, view_rendering, "view rendering usage");
}

fn assert_future_implied_ownership(graph: &SemanticGraph, node: &SemanticNode, description: &str) {
    assert!(
        node.declared_facts.feature_properties.is_some(),
        "{description} builder must publish parser-backed DeclaredFeatureProperties before ownership can be resolved"
    );
    assert_eq!(
        graph
            .effective_facts_for(node)
            .and_then(|facts| facts.implied_feature_ownership),
        Some(ImpliedFeatureOwnership {
            is_composite: true,
            is_reference: false,
        }),
        "{description} should receive the contextual composite ownership default"
    );
    assert_eq!(
        graph
            .effective_feature_ownership_for(node)
            .map(|ownership| ownership.provenance),
        Some(FeatureOwnershipProvenance::Implied),
        "{description} should expose implied ownership through the canonical query"
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
