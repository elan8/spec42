use semantic_core::{
    build_semantic_graph_from_documents, build_view_catalog, build_workspace_graph_dto_for_uris,
    evaluate_views, project_ids_for_renderer, SysmlDocument, SysmlDocumentSourceKind,
};

const INTERCONNECTION_SYSML: &str = r#"
package Pkg {
    part def System {
        part left;
        part right;
        connect left to right;
    }
    part system : System;
    view connections : InterconnectionView {
        expose Pkg::system::**;
    }
}
"#;

#[test]
fn interconnection_view_includes_connections_in_exposed_closure() {
    let doc = SysmlDocument::from_memory_path(
        "workspace",
        "model.sysml",
        INTERCONNECTION_SYSML.to_string(),
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
        .find(|view| view.name == "connections")
        .expect("connections view");

    let projected = project_ids_for_renderer(view, &graph_dto, "interconnection-view");
    assert!(
        projected.iter().any(|id| id.contains("left")),
        "interconnection view should include left part, got: {:?}",
        projected
    );
    assert!(
        projected.iter().any(|id| id.contains("right")),
        "interconnection view should include right part, got: {:?}",
        projected
    );

    let has_connection_edge = graph_dto.edges.iter().any(|edge| {
        matches!(edge.rel_type.as_str(), "connect" | "connection")
            && projected.contains(&edge.source)
            && projected.contains(&edge.target)
    });
    assert!(
        has_connection_edge,
        "interconnection projection should retain connect edges between exposed parts, edges: {:?}",
        graph_dto.edges
    );
}
