use sysml_model::{
    build_graph_from_doc, build_semantic_graph_from_documents, ElementKind, NodeId,
    RelationshipKind, SysmlDocument, SysmlDocumentSourceKind,
};
use sysml_v2_parser::parse;
use url::Url;

#[test]
fn dependency_materializes_client_to_supplier_semantic_edges() {
    let source = r#"
package Selection {
    part def RequiredSensor;
    part def CatalogSensor;
    dependency selectedSensor from RequiredSensor to CatalogSensor;
}
"#;
    let parsed = parse(source).expect("parse dependency model");
    let uri = Url::parse("file:///dependency-selection.sysml").expect("fixture URI");
    let graph = build_graph_from_doc(&parsed, &uri);
    let client_id = NodeId::new(&uri, "Selection::RequiredSensor");
    let client = graph.get_node(&client_id).expect("dependency client");

    let targets = graph.outgoing_targets_by_kind(client, RelationshipKind::Dependency);
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].id.qualified_name, "Selection::CatalogSensor");

    let dependency_id = NodeId::new(&uri, "Selection::selectedSensor");
    let dependency = graph
        .get_node(&dependency_id)
        .expect("dependency relationship node");
    assert_eq!(dependency.element_kind, ElementKind::Dependency);
    assert_eq!(
        dependency.attributes["clients"],
        serde_json::json!(["RequiredSensor"])
    );
    assert_eq!(
        dependency.attributes["suppliers"],
        serde_json::json!(["CatalogSensor"])
    );
}

#[test]
fn dependency_resolves_an_imported_supplier_across_documents() {
    let catalog = SysmlDocument::from_memory_path(
        "workspace",
        "catalog.sysml",
        "package Catalog { part def SensorSku; }".to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("catalog document");
    let design = SysmlDocument::from_memory_path(
        "workspace",
        "design.sysml",
        concat!(
            "package Design {\n",
            "  private import Catalog::*;\n",
            "  part def RequiredSensor;\n",
            "  dependency selection from RequiredSensor to SensorSku;\n",
            "}\n"
        )
        .to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("design document");
    let design_uri = design.uri.clone();

    let (graph, _) =
        build_semantic_graph_from_documents(&[catalog, design]).expect("workspace graph");
    let client_id = NodeId::new(&design_uri, "Design::RequiredSensor");
    let client = graph.get_node(&client_id).expect("dependency client");
    let targets = graph.outgoing_targets_by_kind(client, RelationshipKind::Dependency);

    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].id.qualified_name, "Catalog::SensorSku");
}

#[test]
fn dependency_owned_by_part_definition_has_nested_identity_and_semantics() {
    let source = r#"
package Selection {
    part def CatalogSensor;
    part def RequiredSensor {
        dependency selectedImplementation
            from RequiredSensor to CatalogSensor;
    }
}
"#;
    let parsed = parse(source).expect("parse nested dependency model");
    let uri = Url::parse("file:///nested-dependency-selection.sysml").expect("fixture URI");
    let graph = build_graph_from_doc(&parsed, &uri);

    let dependency_id = NodeId::new(&uri, "Selection::RequiredSensor::selectedImplementation");
    let dependency = graph
        .get_node(&dependency_id)
        .expect("nested dependency relationship node");
    assert_eq!(dependency.element_kind, ElementKind::Dependency);

    let client_id = NodeId::new(&uri, "Selection::RequiredSensor");
    let client = graph.get_node(&client_id).expect("dependency client");
    let targets = graph.outgoing_targets_by_kind(client, RelationshipKind::Dependency);
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].id.qualified_name, "Selection::CatalogSensor");
}
