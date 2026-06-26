use sysml_model::{
    build_semantic_graph_from_documents, build_view_catalog, build_workspace_graph_dto_for_uris,
    evaluate_views, project_ids_for_renderer, SysmlDocument, SysmlDocumentSourceKind,
};

const INHERITED_PARTS_SYSML: &str = r#"
package P {
    part def Vehicle {
        part engine;
        part cabin;
    }
    part vehicle : Vehicle;
    view v : GeneralView {
        expose P::vehicle;
        filter @SysML::PartUsage;
    }
}
"#;

#[test]
fn expose_typed_usage_projects_inherited_definition_parts() {
    let doc = SysmlDocument::from_memory_path(
        "workspace",
        "model.sysml",
        INHERITED_PARTS_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    let uri = doc.uri.clone();
    let (graph, parsed) =
        build_semantic_graph_from_documents(&[doc]).expect("semantic graph should build");
    let parsed_doc = parsed
        .into_iter()
        .find(|entry| entry.uri == uri)
        .expect("parsed workspace document");

    let catalog = build_view_catalog(std::slice::from_ref(&uri), &[parsed_doc]);
    let graph_dto = build_workspace_graph_dto_for_uris(&graph, std::slice::from_ref(&uri));
    let evaluated = evaluate_views(&catalog, &graph, &graph_dto);
    let view = evaluated
        .iter()
        .find(|view| view.name == "v")
        .expect("evaluated view usage");

    assert!(
        view.exposed_ids.contains("P::vehicle"),
        "expose should resolve vehicle usage, got: {:?}",
        view.exposed_ids
    );

    let projected = project_ids_for_renderer(view, &graph_dto, "general-view");
    assert!(
        projected.iter().any(|id| id.contains("engine")),
        "general-view should include inherited engine, got: {:?}",
        projected
    );
    assert!(
        projected.iter().any(|id| id.contains("cabin")),
        "general-view should include inherited cabin, got: {:?}",
        projected
    );
}
