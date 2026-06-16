use semantic_core::{
    build_semantic_graph_from_documents, build_view_candidates, build_view_catalog,
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

fn candidate_for(
    evaluated: &semantic_core::EvaluatedView,
) -> semantic_core::SysmlVisualizationViewCandidateDto {
    let candidates = build_view_candidates(
        std::slice::from_ref(evaluated),
        &Default::default(),
        &Default::default(),
    );
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
    assert_eq!(
        view.effective_view_type.as_deref(),
        Some("InterconnectionView")
    );
    let candidate = candidate_for(&view);
    assert!(candidate.supported);
    assert_eq!(
        candidate.renderer_view.as_deref(),
        Some("interconnection-view")
    );
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
    assert_eq!(
        view.effective_view_type.as_deref(),
        Some("InterconnectionView")
    );
    let candidate = candidate_for(&view);
    assert!(candidate.supported);
    assert_eq!(
        candidate.renderer_view.as_deref(),
        Some("interconnection-view")
    );
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

#[test]
fn general_view_part_usage_filter_excludes_non_part_elements() {
    let content = r#"
        package Pkg {
            part def Robot { part chassis; port p; }
            part robot : Robot;
            view structure : GeneralView {
                expose Pkg::robot;
                filter @SysML::PartUsage;
            }
        }
    "#;
    let view = evaluate_fixture(content, "structure");
    let graph_dto = {
        let doc = semantic_core::SysmlDocument::from_memory_path(
            "workspace",
            "model.sysml",
            content.to_string(),
            semantic_core::SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("document");
        let uri = doc.uri.clone();
        let (graph, parsed) =
            semantic_core::build_semantic_graph_from_documents(&[doc]).expect("graph");
        let parsed_doc = parsed.into_iter().find(|e| e.uri == uri).expect("parsed");
        let catalog = build_view_catalog(std::slice::from_ref(&uri), &[parsed_doc]);
        let _ = evaluate_views(
            &catalog,
            &graph,
            &semantic_core::build_workspace_graph_dto_for_uris(&graph, std::slice::from_ref(&uri)),
        );
        semantic_core::build_workspace_graph_dto_for_uris(&graph, std::slice::from_ref(&uri))
    };
    let projected = semantic_core::project_view(&view, &graph_dto);
    let node_kinds: Vec<_> = graph_dto
        .nodes
        .iter()
        .filter(|node| projected.node_ids.contains(&node.id))
        .map(|node| node.element_type.to_lowercase())
        .collect();
    assert!(node_kinds
        .iter()
        .any(|kind| kind.contains("part") && !kind.contains("def")));
    assert!(!node_kinds.iter().any(|kind| kind.contains("port")));
}

#[test]
fn general_view_requirement_filter_projection_follows_traceability_links() {
    let content = r#"
        package Pkg {
            requirement need;
            requirement req;
            part design;
            satisfy req by design;
            #derivation connection {
                end #original ::> need;
                end #derive ::> req;
            }
            view trace : GeneralView {
                expose Pkg::need;
                expose Pkg::design;
                filter @SysML::RequirementUsage or @SysML::PartUsage;
            }
        }
    "#;
    let view = evaluate_fixture(content, "trace");
    assert_eq!(view.effective_view_type.as_deref(), Some("GeneralView"));
    let graph_dto = {
        let doc = semantic_core::SysmlDocument::from_memory_path(
            "workspace",
            "model.sysml",
            content.to_string(),
            semantic_core::SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("document");
        let uri = doc.uri.clone();
        let (graph, _) = semantic_core::build_semantic_graph_from_documents(&[doc]).expect("graph");
        semantic_core::build_workspace_graph_dto_for_uris(&graph, std::slice::from_ref(&uri))
    };
    let projected = semantic_core::project_view(&view, &graph_dto);
    assert!(projected.node_ids.iter().any(|id| id.contains("need")));
    assert!(projected.node_ids.iter().any(|id| id.contains("req")));
    assert!(projected.node_ids.iter().any(|id| id.contains("design")));
    assert_eq!(projected.hints.grid_layout.as_deref(), Some("traceability"));
}

#[test]
fn browser_view_projection_omits_ancestors_outside_scope() {
    let content = r#"
        package Pkg {
            part def Robot { part chassis; }
            part robot : Robot;
            view tree : BrowserView {
                expose Pkg::robot;
                filter @SysML::PartUsage;
            }
        }
    "#;
    let view = evaluate_fixture(content, "tree");
    let graph_dto = {
        let doc = semantic_core::SysmlDocument::from_memory_path(
            "workspace",
            "model.sysml",
            content.to_string(),
            semantic_core::SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("document");
        let uri = doc.uri.clone();
        let (graph, _) = semantic_core::build_semantic_graph_from_documents(&[doc]).expect("graph");
        semantic_core::build_workspace_graph_dto_for_uris(&graph, std::slice::from_ref(&uri))
    };
    let projected = semantic_core::project_view(&view, &graph_dto);
    assert!(!projected.node_ids.iter().any(|id| id == "Pkg"));
    assert_eq!(projected.hints.browser_layout.as_deref(), Some("hierarchy"));
    assert!(projected
        .hints
        .tree_roots
        .iter()
        .any(|id| id.contains("robot")));
}

#[test]
fn geometry_view_without_usage_filter_gets_spatial_defaults() {
    let view = semantic_core::EvaluatedView {
        id: "Pkg::geom".to_string(),
        name: "geom".to_string(),
        effective_view_type: Some("GeometryView".to_string()),
        exposed_ids: std::collections::HashSet::from(["Pkg::robot".to_string()]),
        conforms_to: Vec::new(),
        filters: semantic_core::merge_usage_default_filters("GeometryView", &[], None),
        visible_ids: std::collections::HashSet::new(),
        issues: Vec::new(),
    };
    assert!(view
        .filters
        .iter()
        .any(|filter| matches!(filter, semantic_core::FilterExpr::Or(_, _))));
    let projected = semantic_core::project_view(
        &view,
        &semantic_core::SysmlGraphDto {
            nodes: vec![],
            edges: vec![],
        },
    );
    assert_eq!(projected.hints.geometry_mode.as_deref(), Some("2d"));
    assert_eq!(
        projected.hints.geometry_projection.as_deref(),
        Some("orthographic")
    );
}

#[test]
fn grid_view_connection_filter_selects_relationship_matrix_subtype() {
    let view = semantic_core::EvaluatedView {
        id: "Pkg::matrix".to_string(),
        name: "matrix".to_string(),
        effective_view_type: Some("GridView".to_string()),
        exposed_ids: std::collections::HashSet::from(["Pkg::a".to_string()]),
        conforms_to: Vec::new(),
        filters: vec![semantic_core::FilterExpr::Matches(
            "@SysML::ConnectionUsage".to_string(),
        )],
        visible_ids: std::collections::HashSet::new(),
        issues: Vec::new(),
    };
    let projected = semantic_core::project_view(
        &view,
        &semantic_core::SysmlGraphDto {
            nodes: vec![],
            edges: vec![],
        },
    );
    assert_eq!(
        projected.hints.grid_subtype.as_deref(),
        Some("relationship_matrix")
    );
}
