//! Scoped vs full-workspace IBD parity for general views.
//!
//! Ensures `IbdBuildScope::ViewExposedPackages` + scoped merge produces the same
//! filtered IBD as full-workspace IBD + `filter_ibd_by_visible_ids`.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use sysml_model::{
    build_merged_workspace_ibd, build_semantic_graph_with_provider, build_view_catalog,
    build_workspace_graph_dto_for_uris, evaluate_views, filter_ibd_by_visible_ids,
    project_ids_for_renderer, workspace_uris_for_ibd_scope, FileSystemDocumentProvider,
    IbdBuildScope, IbdDataDto,
};
use url::Url;

fn repo_examples_dir(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples")
        .join(relative)
}

fn optional_robot_vacuum_fixture_root() -> Option<PathBuf> {
    let repo_root = PathBuf::from(std::env::var_os("SYSML_ROBOT_VACUUM_DIR")?);
    let model = repo_root.join("model");
    if model.is_dir() || repo_root.is_dir() {
        Some(repo_root)
    } else {
        None
    }
}

fn robot_vacuum_model_dir(repo_root: &Path) -> PathBuf {
    let nested = repo_root.join("model");
    if nested.is_dir() {
        nested
    } else {
        repo_root.to_path_buf()
    }
}

struct GeneralViewWorkspace {
    workspace_uris: Vec<Url>,
    graph: sysml_model::SemanticGraph,
    parsed: Vec<sysml_model::WorkspaceParsedDocument>,
}

fn load_filesystem_workspace(scan_root: PathBuf, workspace_root: PathBuf) -> GeneralViewWorkspace {
    let provider = FileSystemDocumentProvider::new(
        scan_root.clone(),
        Some(workspace_root.clone()),
        Vec::new(),
    );
    let (graph, parsed) = build_semantic_graph_with_provider(&provider).expect("semantic graph");
    let workspace_root_uri =
        Url::from_directory_path(workspace_root.canonicalize().unwrap()).unwrap();
    let workspace_uris = sysml_model::workspace_uris_for_root(&graph, &[], &workspace_root_uri);
    GeneralViewWorkspace {
        workspace_uris,
        graph,
        parsed,
    }
}

fn ibd_signatures(ibd: &IbdDataDto) -> (usize, usize, Vec<String>) {
    let mut part_ids: Vec<String> = ibd.parts.iter().map(|part| part.id.clone()).collect();
    part_ids.sort();
    let mut connector_ids: Vec<String> = ibd
        .connectors
        .iter()
        .map(|connector| format!("{}->{}", connector.source, connector.target))
        .collect();
    connector_ids.sort();
    (part_ids.len(), connector_ids.len(), part_ids)
}

fn assert_filtered_ibd_equivalent(
    full_path: &IbdDataDto,
    scoped_path: &IbdDataDto,
    view_name: &str,
) {
    let (full_parts, full_connectors, full_part_ids) = ibd_signatures(full_path);
    let (scoped_parts, scoped_connectors, scoped_part_ids) = ibd_signatures(scoped_path);
    assert_eq!(
        full_parts, scoped_parts,
        "{view_name}: filtered IBD part count mismatch"
    );
    assert_eq!(
        full_connectors, scoped_connectors,
        "{view_name}: filtered IBD connector count mismatch"
    );
    assert_eq!(
        full_part_ids, scoped_part_ids,
        "{view_name}: filtered IBD part ids mismatch"
    );
}

fn assert_scoped_ibd_parity_for_general_views(workspace: &GeneralViewWorkspace) {
    let full_ibd = build_merged_workspace_ibd(&workspace.graph, &workspace.workspace_uris);
    let graph_dto = build_workspace_graph_dto_for_uris(&workspace.graph, &workspace.workspace_uris);
    let catalog = build_view_catalog(&workspace.workspace_uris, &workspace.parsed);
    let evaluated = evaluate_views(&catalog, &workspace.graph, &graph_dto);

    let general_views: Vec<_> = evaluated
        .iter()
        .filter(|view| {
            view.effective_view_type.as_deref() == Some("GeneralView")
                || sysml_model::renderer_view_for_view_type(view.effective_view_type.as_deref())
                    == Some("general-view")
        })
        .collect();

    assert!(
        !general_views.is_empty(),
        "expected at least one general view in workspace"
    );

    for view in general_views {
        let selected_ids: HashSet<String> =
            project_ids_for_renderer(view, &graph_dto, "general-view");
        let ibd_from_full = filter_ibd_by_visible_ids(&full_ibd, &selected_ids);

        let scoped_uris = workspace_uris_for_ibd_scope(
            &workspace.workspace_uris,
            &workspace.graph,
            IbdBuildScope::ViewExposedPackages,
            &selected_ids,
        );
        assert!(
            !scoped_uris.is_empty(),
            "{}: scoped URI closure should not be empty",
            view.name
        );
        assert!(
            scoped_uris.len() <= workspace.workspace_uris.len(),
            "{}: scoped URIs ({}) should not exceed workspace URIs ({})",
            view.name,
            scoped_uris.len(),
            workspace.workspace_uris.len()
        );

        let scoped_merged = build_merged_workspace_ibd(&workspace.graph, &scoped_uris);
        let ibd_from_scoped = filter_ibd_by_visible_ids(&scoped_merged, &selected_ids);

        assert_filtered_ibd_equivalent(&ibd_from_full, &ibd_from_scoped, &view.name);
    }
}

#[test]
fn drone_general_view_scoped_ibd_matches_full_workspace_filter() {
    let drone_dir = repo_examples_dir("drone");
    assert!(
        drone_dir.is_dir(),
        "expected drone example at {}",
        drone_dir.display()
    );

    let workspace = load_filesystem_workspace(drone_dir.clone(), drone_dir);
    assert_scoped_ibd_parity_for_general_views(&workspace);
}

#[test]
#[ignore = "optional local drill-down; set SYSML_ROBOT_VACUUM_DIR to a robot-vacuum checkout"]
fn robot_vacuum_product_structure_scoped_ibd_matches_full_workspace_filter() {
    let Some(repo_root) = optional_robot_vacuum_fixture_root() else {
        return;
    };
    let model_dir = robot_vacuum_model_dir(&repo_root);
    let workspace = load_filesystem_workspace(model_dir.clone(), repo_root);
    assert_scoped_ibd_parity_for_general_views(&workspace);
}
