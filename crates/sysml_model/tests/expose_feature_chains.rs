use sysml_model::{
    build_semantic_graph_from_documents, build_view_catalog, build_workspace_graph_dto_for_uris,
    evaluate_views, project_ids_for_renderer, SysmlDocument, SysmlDocumentSourceKind,
};

const EXPOSE_FEATURE_CHAIN_SYSML: &str = r#"
package RegionalGridExpansion {
    part def Architecture;

    part regionalExpansionProject {
        part architecture : Architecture;
    }

    view def GridStructureView;

    view expansionStructure : GridStructureView {
        expose RegionalGridExpansion::regionalExpansionProject.architecture;
    }
}
"#;

#[test]
fn expose_feature_chain_resolves_nested_usage_in_view_projection() {
    let doc = SysmlDocument::from_memory_path(
        "grid-expansion",
        "grid.sysml",
        EXPOSE_FEATURE_CHAIN_SYSML.to_string(),
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
        .find(|view| view.name == "expansionStructure")
        .expect("evaluated view usage");

    assert!(
        view.exposed_ids
            .contains("RegionalGridExpansion::regionalExpansionProject::architecture"),
        "feature-chain expose should resolve nested architecture usage, got: {:?}",
        view.exposed_ids
    );

    let projected = project_ids_for_renderer(view, &graph_dto, "general-view");
    assert!(
        !projected.is_empty(),
        "general-view projection should include exposed architecture, got: {:?}",
        projected
    );
}
