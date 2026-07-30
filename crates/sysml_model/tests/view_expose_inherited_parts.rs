use sysml_model::{
    build_semantic_graph_from_documents, build_view_catalog, build_workspace_graph_dto_for_uris,
    evaluate_views, project_ids_for_renderer, SysmlDocument, SysmlDocumentSourceKind,
};

const EXACT_EXPOSE_SYSML: &str = r#"
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

const RECURSIVE_EXPOSE_SYSML: &str = r#"
package P {
    part def Vehicle {
        part engine;
        part cabin;
    }
    part vehicle : Vehicle;
    view v : GeneralView {
        expose P::vehicle::**;
        filter @SysML::PartUsage;
    }
}
"#;

fn evaluate_named_view(sysml: &str) -> (sysml_model::EvaluatedView, sysml_model::SysmlGraphDto) {
    let doc = SysmlDocument::from_memory_path(
        "workspace",
        "model.sysml",
        sysml.to_string(),
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
        .into_iter()
        .find(|view| view.name == "v")
        .expect("evaluated view usage");
    (view, graph_dto)
}

#[test]
fn general_view_exact_expose_does_not_infer_typed_definition_parts() {
    let (view, graph_dto) = evaluate_named_view(EXACT_EXPOSE_SYSML);

    assert!(
        view.exposed_ids.contains("P::vehicle"),
        "expose should resolve vehicle usage, got: {:?}",
        view.exposed_ids
    );
    assert!(
        !view.exposed_ids.iter().any(|id| id.contains("engine")),
        "exact expose must not invent typed-definition members, got: {:?}",
        view.exposed_ids
    );

    let projected = project_ids_for_renderer(&view, &graph_dto, "general-view");
    assert_eq!(
        projected, view.exposed_ids,
        "GeneralView projection follows exact exposed scope"
    );
}

#[test]
fn recursive_expose_of_typed_usage_includes_inherited_definition_parts() {
    let (view, graph_dto) = evaluate_named_view(RECURSIVE_EXPOSE_SYSML);

    assert!(
        view.exposed_ids.contains("P::vehicle"),
        "recursive expose should include root usage, got: {:?}",
        view.exposed_ids
    );
    assert!(
        view.exposed_ids.iter().any(|id| id.contains("engine")),
        "recursive expose should include inherited engine from typed definition, got: {:?}",
        view.exposed_ids
    );
    assert!(
        view.exposed_ids.iter().any(|id| id.contains("cabin")),
        "recursive expose should include inherited cabin from typed definition, got: {:?}",
        view.exposed_ids
    );

    let projected = project_ids_for_renderer(&view, &graph_dto, "general-view");
    assert!(
        projected.iter().any(|id| id.contains("engine")),
        "general-view should project recursively exposed engine, got: {:?}",
        projected
    );
    assert!(
        projected.iter().any(|id| id.contains("cabin")),
        "general-view should project recursively exposed cabin, got: {:?}",
        projected
    );
}
