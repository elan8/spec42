//! Parser-backed occurrence-usage modifiers must remain authoritative graph facts.

use sysml_model::{
    build_semantic_graph_from_documents, ElementKind, SysmlDocument, SysmlDocumentSourceKind,
};

fn workspace_doc(content: &str) -> SysmlDocument {
    SysmlDocument::from_memory_path(
        "workspace",
        "occurrence_modifiers.sysml",
        content.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("workspace document")
}

#[test]
fn occurrence_usage_projects_authored_modifiers_and_reference_ownership() {
    let document = workspace_doc(
        r#"package P {
  occurrence def Event;
  abstract constant ref occurrence stable : Event;
  ref snapshot moment : Event;
  individual selected : Event;
}"#,
    );
    let (graph, _) = build_semantic_graph_from_documents(&[document]).expect("semantic graph");

    let stable = graph
        .nodes_named("stable")
        .into_iter()
        .find(|node| node.element_kind == ElementKind::Occurrence)
        .expect("abstract constant occurrence usage");
    let stable_properties = stable
        .declared_facts
        .feature_properties
        .as_ref()
        .expect("declared occurrence properties");
    assert!(stable_properties.is_abstract);
    assert!(stable_properties.is_constant);
    assert_eq!(stable_properties.is_reference, Some(true));
    assert_eq!(stable_properties.is_composite, Some(false));
    assert!(!stable_properties.is_portion);

    let moment = graph
        .nodes_named("moment")
        .into_iter()
        .find(|node| node.element_kind == ElementKind::Occurrence)
        .expect("snapshot occurrence usage");
    let moment_properties = moment
        .declared_facts
        .feature_properties
        .as_ref()
        .expect("declared snapshot properties");
    assert_eq!(moment_properties.is_reference, Some(true));
    assert_eq!(moment_properties.is_composite, Some(false));
    assert!(moment_properties.is_portion);
    assert_eq!(moment_properties.portion_kind.as_deref(), Some("snapshot"));

    let selected = graph
        .nodes_named("selected")
        .into_iter()
        .find(|node| node.element_kind == ElementKind::Occurrence)
        .expect("individual occurrence usage");
    let selected_properties = selected
        .declared_facts
        .feature_properties
        .as_ref()
        .expect("declared individual properties");
    assert!(selected_properties.is_individual);
    assert_eq!(selected_properties.is_reference, Some(false));
    assert_eq!(selected_properties.is_composite, Some(true));
}
