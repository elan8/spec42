//! Deferred implied-semantic contracts that are parser-backed but not yet
//! publishable without an authoritative graph representation for every fact.

use sysml_model::{build_semantic_graph_from_documents, SysmlDocument, SysmlDocumentSourceKind};

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
#[ignore = "SKIP: requires a canonical implied-multiplicity fact distinct from parser-authored multiplicity"]
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
        .find(|node| node.element_kind == "part")
        .expect("nested part usage");

    assert!(
        wheel.declared_facts.multiplicity.is_none(),
        "the parser-backed fact must remain absent when no multiplicity was authored"
    );
    panic!(
        "SKIP: publish a separate implied [1..1] multiplicity fact for this usage without changing the authored fact"
    );
}

#[test]
#[ignore = "SKIP: requires a canonical featuring-type closure rather than ownership-parent inference"]
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
        .find(|node| node.element_kind == "attribute")
        .expect("nested feature");

    assert_eq!(
        graph.parent_of(rpm).map(|node| node.name.as_str()),
        Some("engine"),
        "the parser-backed ownership chain is available"
    );
    panic!(
        "SKIP: publish the resolved featuring type only after inheritance and ownership closure share one semantic owner"
    );
}

#[test]
#[ignore = "SKIP: requires a canonical expression-result identity before an implied binding fact can be published"]
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
        .find(|node| node.element_kind == "attribute")
        .expect("bound attribute usage");

    assert!(
        speed.declared_facts.feature_value.is_some(),
        "the parser-backed bound feature value is available to the semantic graph"
    );
    panic!(
        "SKIP: materialize the relationship only after the value expression has a stable semantic identity"
    );
}
