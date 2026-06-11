use std::path::Path;

use semantic_core::{
    build_semantic_graph_from_documents, build_view_catalog, build_workspace_graph_dto_for_uris,
    evaluate_views, project_ids_for_renderer, SysmlDocument, SysmlDocumentSourceKind,
};

#[test]
fn stedin_grid_structure_general_view_is_not_empty() {
    let workspace_root = Path::new(r"C:\Git\sysml-powersystems\sysml");
    if !workspace_root.is_dir() {
        return;
    }

    let mut documents = Vec::new();
    let mut uris = Vec::new();
    for entry in walkdir::WalkDir::new(workspace_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "sysml"))
    {
        let path = entry.path();
        let content = std::fs::read_to_string(path).expect("read stedin model");
        let doc = SysmlDocument::from_memory_path(
            "stedin",
            path.file_name().and_then(|n| n.to_str()).unwrap_or("model.sysml"),
            content,
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("document uri");
        uris.push(doc.uri.clone());
        documents.push(doc);
    }

    let (graph, parsed) =
        build_semantic_graph_from_documents(&documents).expect("semantic graph should build");
    let catalog = build_view_catalog(&uris, &parsed);
    let graph_dto = build_workspace_graph_dto_for_uris(&graph, &uris);
    let evaluated = evaluate_views(&catalog, &graph, &graph_dto);
    let view = evaluated
        .iter()
        .find(|view| view.name == "gridStructure")
        .expect("gridStructure view");

    assert!(
        !view.exposed_ids.is_empty(),
        "gridStructure expose should resolve, issues: {:?}, exposed: {:?}",
        view.issues,
        view.exposed_ids
    );

    let projected = project_ids_for_renderer(view, &graph_dto, "general-view");
    assert!(
        !projected.is_empty(),
        "gridStructure general-view should project nodes, exposed: {:?}, projected: {:?}",
        view.exposed_ids,
        projected
    );
}
