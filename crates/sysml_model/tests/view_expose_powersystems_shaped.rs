use sysml_model::{
    build_semantic_graph_from_documents, build_view_catalog, build_workspace_graph_dto_for_uris,
    evaluate_views, project_ids_for_renderer, SysmlDocument, SysmlDocumentSourceKind,
};

const PROJECT_SYSML: &str = r#"
package RegionalGridExpansion {
    public import RegionalGridExpansion::Architecture::*;
    part regionalExpansionProject {
        part architecture : RegionalGridArchitecture;
    }
}
"#;

const ARCHITECTURE_SYSML: &str = r#"
package RegionalGridExpansion::Architecture {
    part def RegionalGridArchitecture {
        part feederNorth;
        part feederSouth;
    }
}
"#;

const VIEWS_SYSML: &str = r#"
package RegionalGridExpansion::Views {
    public import RegionalGridExpansion::*;
    view gridStructure : GeneralView {
        expose RegionalGridExpansion::regionalExpansionProject.architecture;
        filter @SysML::PartUsage or @SysML::PartDefinition;
    }
}
"#;

#[test]
fn powersystems_shaped_feature_chain_expose_projects_grid_topology() {
    let project = SysmlDocument::from_memory_path(
        "powersystems",
        "Project.sysml",
        PROJECT_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("project uri");
    let architecture = SysmlDocument::from_memory_path(
        "powersystems",
        "Architecture.sysml",
        ARCHITECTURE_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("architecture uri");
    let views = SysmlDocument::from_memory_path(
        "powersystems",
        "Views.sysml",
        VIEWS_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("views uri");

    let docs = vec![project, architecture, views];
    let uris: Vec<_> = docs.iter().map(|doc| doc.uri.clone()).collect();
    let (graph, parsed) =
        build_semantic_graph_from_documents(&docs).expect("semantic graph should build");

    let catalog = build_view_catalog(&uris, &parsed);
    let graph_dto = build_workspace_graph_dto_for_uris(&graph, &uris);
    let evaluated = evaluate_views(&catalog, &graph, &graph_dto);
    let view = evaluated
        .iter()
        .find(|view| view.name == "gridStructure")
        .expect("gridStructure view");

    assert!(
        !view.exposed_ids.is_empty(),
        "architecture expose should resolve, issues: {:?}, exposed: {:?}",
        view.issues,
        view.exposed_ids
    );

    let projected = project_ids_for_renderer(view, &graph_dto, "general-view");
    assert!(
        projected.iter().any(|id| id.contains("feederNorth")),
        "general-view should include feederNorth from typed architecture definition, got: {:?}",
        projected
    );
    assert!(
        projected.iter().any(|id| id.contains("feederSouth")),
        "general-view should include feederSouth from typed architecture definition, got: {:?}",
        projected
    );
}
