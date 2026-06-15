use semantic_core::{
    build_semantic_graph_from_documents, build_view_catalog, build_view_candidates,
    build_workspace_graph_dto_for_uris, evaluate_views, SysmlDocument, SysmlDocumentSourceKind,
};

fn evaluate_fixture(content: &str, view_name: &str) -> semantic_core::EvaluatedView {
    let doc = SysmlDocument::from_memory_path(
        "workspace",
        "model.sysml",
        content.to_string(),
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
    evaluated
        .into_iter()
        .find(|view| view.name == view_name)
        .unwrap_or_else(|| panic!("view '{view_name}' not found"))
}

fn candidate_for(evaluated: &semantic_core::EvaluatedView) -> semantic_core::SysmlVisualizationViewCandidateDto {
    let candidates = build_view_candidates(std::slice::from_ref(evaluated), &Default::default(), &Default::default());
    candidates
        .into_iter()
        .find(|candidate| candidate.id == evaluated.id)
        .expect("candidate for evaluated view")
}

#[test]
fn rendering_only_view_resolves_to_interconnection_renderer() {
    let content = r#"
        package Pkg {
            part def System { part left; part right; connect left to right; }
            part system : System;
            view connections {
                expose Pkg::system::**;
                render asInterconnectionDiagram;
            }
        }
    "#;
    let view = evaluate_fixture(content, "connections");
    assert_eq!(view.effective_view_type.as_deref(), Some("InterconnectionView"));
    let candidate = candidate_for(&view);
    assert!(candidate.supported);
    assert_eq!(candidate.renderer_view.as_deref(), Some("interconnection-view"));
}

#[test]
fn explicit_view_type_wins_over_rendering_clause() {
    let content = r#"
        package Pkg {
            part def System { part child; }
            part system : System;
            view overview : GeneralView {
                expose Pkg::system;
                render asInterconnectionDiagram;
            }
        }
    "#;
    let view = evaluate_fixture(content, "overview");
    assert_eq!(view.effective_view_type.as_deref(), Some("GeneralView"));
    let candidate = candidate_for(&view);
    assert_eq!(candidate.renderer_view.as_deref(), Some("general-view"));
}

#[test]
fn element_table_rendering_maps_to_grid_view() {
    let content = r#"
        package Pkg {
            part def System { part child; }
            part system : System;
            view table {
                expose Pkg::system;
                render asElementTable;
            }
        }
    "#;
    let view = evaluate_fixture(content, "table");
    assert_eq!(view.effective_view_type.as_deref(), Some("GridView"));
    let candidate = candidate_for(&view);
    assert_eq!(candidate.renderer_view.as_deref(), Some("grid-view"));
}

#[test]
fn untyped_view_without_render_falls_back_to_general_view() {
    let content = r#"
        package Pkg {
            part def System { part child; }
            part system : System;
            view overview {
                expose Pkg::system;
            }
        }
    "#;
    let view = evaluate_fixture(content, "overview");
    assert_eq!(view.effective_view_type.as_deref(), Some("GeneralView"));
    let candidate = candidate_for(&view);
    assert!(candidate.supported);
    assert_eq!(candidate.renderer_view.as_deref(), Some("general-view"));
}

#[test]
fn typed_interconnection_view_regression() {
    let content = r#"
        package Pkg {
            part def System { part child; }
            part system : System;
            view connections : InterconnectionView {
                expose Pkg::system::**;
            }
        }
    "#;
    let view = evaluate_fixture(content, "connections");
    assert_eq!(view.effective_view_type.as_deref(), Some("InterconnectionView"));
    let candidate = candidate_for(&view);
    assert!(candidate.supported);
    assert_eq!(candidate.renderer_view.as_deref(), Some("interconnection-view"));
}

#[test]
fn view_def_rendering_is_inherited_when_usage_is_untyped() {
    let content = r#"
        package Pkg {
            part def System { part child; }
            part system : System;
            view def StructureView {
                filter @SysML::PartUsage;
                render asTreeDiagram;
            }
            view structure : StructureView {
                expose Pkg::system::**;
            }
        }
    "#;
    let view = evaluate_fixture(content, "structure");
    assert_eq!(view.effective_view_type.as_deref(), Some("BrowserView"));
    let candidate = candidate_for(&view);
    assert_eq!(candidate.renderer_view.as_deref(), Some("browser-view"));
}
