use semantic_core::{
    build_semantic_graph_from_documents, build_view_catalog, build_workspace_graph_dto_for_uris,
    evaluate_views, SysmlDocument, SysmlDocumentSourceKind,
};

const EXPOSE_FEATURE_CHAIN_SYSML: &str = r#"
package StedinRijnmondGridExpansion {
    part def Architecture;

    part rijnmondExpansionProject {
        part architecture : Architecture;
    }

    view def GridStructureView;

    view expansionStructure : GridStructureView {
        expose StedinRijnmondGridExpansion::rijnmondExpansionProject.architecture;
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
    let evaluated = evaluate_views(&catalog, &graph_dto);
    let view = evaluated
        .iter()
        .find(|view| view.name == "expansionStructure")
        .expect("evaluated view usage");

    assert!(
        view.exposed_ids
            .contains("StedinRijnmondGridExpansion::rijnmondExpansionProject::architecture"),
        "feature-chain expose should resolve nested architecture usage, got: {:?}",
        view.exposed_ids
    );
}
