use std::path::PathBuf;

use semantic_core::{
    build_semantic_graph_from_documents, build_view_catalog, build_workspace_graph_dto_for_uris,
    evaluate_views, project_ids_for_renderer, SysmlDocument, SysmlDocumentSourceKind,
};

fn optional_external_grid_fixture_sysml_root() -> Option<PathBuf> {
    let repo_root = PathBuf::from(std::env::var_os("SYSML_POWERSYSTEMS_DIR")?);
    let nested = repo_root.join("sysml");
    if nested.is_dir() {
        Some(nested)
    } else if repo_root.is_dir() {
        Some(repo_root)
    } else {
        None
    }
}

#[test]
#[ignore = "optional local drill-down; set SYSML_POWERSYSTEMS_DIR to an external grid fixture checkout"]
fn powersystems_grid_structure_general_view_is_not_empty() {
    let Some(workspace_root) = optional_external_grid_fixture_sysml_root() else {
        return;
    };

    let mut documents = Vec::new();
    let mut uris = Vec::new();
    for entry in walkdir::WalkDir::new(workspace_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "sysml"))
    {
        let path = entry.path();
        let content = std::fs::read_to_string(path).expect("read power systems model");
        let doc = SysmlDocument::from_memory_path(
            "powersystems",
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("model.sysml"),
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
