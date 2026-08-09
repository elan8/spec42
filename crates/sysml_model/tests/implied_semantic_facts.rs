//! Implied semantic contracts published separately from their authored source facts.

use sysml_model::{
    build_semantic_graph_from_documents, ElementKind, ExpressionResultRole, SysmlDocument,
    SysmlDocumentSourceKind,
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
