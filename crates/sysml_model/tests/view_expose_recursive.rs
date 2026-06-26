use sysml_model::{
    build_semantic_graph_from_documents, build_view_catalog, build_workspace_graph_dto_for_uris,
    evaluate_views, SysmlDocument, SysmlDocumentSourceKind,
};

const RECURSIVE_EXPOSE_SYSML: &str = r#"
package Pkg {
    part def System {
        part subsystem;
    }
    part system : System;
    view v : GeneralView {
        expose Pkg::system::**;
    }
}
"#;

#[test]
fn recursive_expose_includes_transitive_owned_members() {
    let doc = SysmlDocument::from_memory_path(
        "workspace",
        "model.sysml",
        RECURSIVE_EXPOSE_SYSML.to_string(),
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
        view.exposed_ids.contains("Pkg::system"),
        "recursive expose should include root usage, got: {:?}",
        view.exposed_ids
    );
    assert!(
        view.exposed_ids.contains("Pkg::System::subsystem"),
        "recursive expose should include inherited subsystem from typed definition, got: {:?}",
        view.exposed_ids
    );
}
