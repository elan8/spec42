//! Scoped vs full-workspace IBD parity for interconnection views.
//!
//! Ensures `IbdBuildScope::ViewExposedPackages` + scoped merge produces the same
//! interconnection scene as full-workspace IBD + `select_interconnection_ibd_scope`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use sysml_model::{
    build_interconnection_scene, build_merged_workspace_ibd, build_semantic_graph_with_provider,
    build_view_catalog, build_workspace_graph_dto_for_uris, evaluate_views,
    project_ids_for_renderer, select_interconnection_ibd_scope, workspace_uris_for_ibd_scope,
    workspace_uris_for_root, EvaluatedView, FileSystemDocumentProvider, IbdBuildScope,
    InterconnectionSceneDto,
};
use url::Url;

fn repo_examples_dir(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples")
        .join(relative)
}

fn optional_powersystems_fixture_root() -> Option<PathBuf> {
    let repo_root = PathBuf::from(std::env::var_os("SYSML_POWERSYSTEMS_DIR")?);
    if repo_root.is_dir() {
        Some(repo_root)
    } else {
        None
    }
}

fn powersystems_sysml_root(repo_root: &Path) -> PathBuf {
    let nested = repo_root.join("sysml");
    if nested.is_dir() {
        nested
    } else {
        repo_root.to_path_buf()
    }
}

struct InterconnectionWorkspace {
    workspace_uris: Vec<Url>,
    graph: sysml_model::SemanticGraph,
    parsed: Vec<sysml_model::WorkspaceParsedDocument>,
}

fn load_filesystem_workspace(scan_root: PathBuf, workspace_root: PathBuf) -> InterconnectionWorkspace {
    let provider = FileSystemDocumentProvider::new(
        scan_root.clone(),
        Some(workspace_root.clone()),
        Vec::new(),
    );
    let (graph, parsed) =
        build_semantic_graph_with_provider(&provider).expect("semantic graph should build");
    let workspace_root_uri = Url::from_directory_path(
        workspace_root
            .canonicalize()
            .unwrap_or(workspace_root),
    )
    .expect("workspace root uri");
    let workspace_uris =
        workspace_uris_for_root(&graph, &[], &workspace_root_uri);
    InterconnectionWorkspace {
        workspace_uris,
        graph,
        parsed,
    }
}

fn root_ids_for_view(view: &EvaluatedView) -> Vec<String> {
    view.exposed_ids
        .iter()
        .map(|id| id.replace("::", "."))
        .collect()
}

fn build_scene_for_view(
    workspace: &InterconnectionWorkspace,
    view: &EvaluatedView,
    ibd_source: &sysml_model::IbdDataDto,
) -> InterconnectionSceneDto {
    let graph_dto =
        build_workspace_graph_dto_for_uris(&workspace.graph, &workspace.workspace_uris);
    let projected = project_ids_for_renderer(view, &graph_dto, "interconnection-view");
    let scoped_ibd = select_interconnection_ibd_scope(
        ibd_source,
        &projected,
        Some(&view.exposed_ids),
    );
    build_interconnection_scene(
        &scoped_ibd,
        &view.id,
        &view.name,
        &root_ids_for_view(view),
        None,
    )
}

fn edge_signatures(scene: &InterconnectionSceneDto) -> BTreeSet<(String, String, String)> {
    scene
        .edges
        .iter()
        .map(|edge| {
            (
                edge.source_node_id.clone(),
                edge.target_node_id.clone(),
                edge.semantic_id.clone().unwrap_or_default(),
            )
        })
        .collect()
}

fn assert_scenes_equivalent(
    full_path: &InterconnectionSceneDto,
    scoped_path: &InterconnectionSceneDto,
    view_name: &str,
) {
    assert_eq!(
        full_path.view.root_ids, scoped_path.view.root_ids,
        "{view_name}: root_ids mismatch"
    );
    assert_eq!(
        full_path.nodes.len(),
        scoped_path.nodes.len(),
        "{view_name}: node count mismatch (full={}, scoped={})",
        full_path.nodes.len(),
        scoped_path.nodes.len()
    );
    assert_eq!(
        full_path.ports.len(),
        scoped_path.ports.len(),
        "{view_name}: port count mismatch"
    );
    assert_eq!(
        full_path.edges.len(),
        scoped_path.edges.len(),
        "{view_name}: edge count mismatch"
    );
    assert_eq!(
        edge_signatures(full_path),
        edge_signatures(scoped_path),
        "{view_name}: edge signatures mismatch"
    );
}

fn assert_scoped_ibd_parity_for_interconnection_views(workspace: &InterconnectionWorkspace) {
    let full_ibd = build_merged_workspace_ibd(&workspace.graph, &workspace.workspace_uris);
    let graph_dto =
        build_workspace_graph_dto_for_uris(&workspace.graph, &workspace.workspace_uris);
    let catalog = build_view_catalog(&workspace.workspace_uris, &workspace.parsed);
    let evaluated = evaluate_views(&catalog, &workspace.graph, &graph_dto);

    let interconnection_views: Vec<_> = evaluated
        .iter()
        .filter(|view| {
            view.effective_view_type.as_deref() == Some("InterconnectionView")
                || sysml_model::renderer_view_for_view_type(view.effective_view_type.as_deref())
                    == Some("interconnection-view")
        })
        .collect();

    assert!(
        !interconnection_views.is_empty(),
        "expected at least one interconnection view in workspace"
    );

    for view in interconnection_views {
        let scene_full = build_scene_for_view(workspace, view, &full_ibd);

        let scoped_uris = workspace_uris_for_ibd_scope(
            &workspace.workspace_uris,
            &workspace.graph,
            IbdBuildScope::ViewExposedPackages,
            &view.exposed_ids,
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
        let scene_scoped = build_scene_for_view(workspace, view, &scoped_merged);

        assert_scenes_equivalent(&scene_full, &scene_scoped, &view.name);

        let projected = project_ids_for_renderer(view, &graph_dto, "interconnection-view");
        let ibd_from_full =
            select_interconnection_ibd_scope(&full_ibd, &projected, Some(&view.exposed_ids));
        let ibd_from_scoped = select_interconnection_ibd_scope(
            &scoped_merged,
            &projected,
            Some(&view.exposed_ids),
        );
        assert_eq!(
            ibd_from_full.parts.len(),
            ibd_from_scoped.parts.len(),
            "{}: scoped IBD part count mismatch",
            view.name
        );
        assert_eq!(
            ibd_from_full.connectors.len(),
            ibd_from_scoped.connectors.len(),
            "{}: scoped IBD connector count mismatch",
            view.name
        );
    }
}

#[test]
fn drone_connections_scoped_ibd_matches_full_workspace_filter() {
    let drone_dir = repo_examples_dir("drone");
    assert!(
        drone_dir.is_dir(),
        "expected drone example at {}",
        drone_dir.display()
    );

    let workspace = load_filesystem_workspace(drone_dir.clone(), drone_dir);
    assert_scoped_ibd_parity_for_interconnection_views(&workspace);
}

#[test]
#[ignore = "optional local drill-down; set SYSML_POWERSYSTEMS_DIR to an external grid fixture checkout"]
fn powersystems_system_context_scoped_ibd_matches_full_workspace_filter() {
    let Some(repo_root) = optional_powersystems_fixture_root() else {
        return;
    };
    let scan_root = powersystems_sysml_root(&repo_root);
    let workspace = load_filesystem_workspace(scan_root, repo_root);
    assert_scoped_ibd_parity_for_interconnection_views(&workspace);
}
