#[path = "../../../tests/fixtures/robot_vacuum_fixture.rs"]
mod robot_vacuum_fixture;

use robot_vacuum_fixture::require_robot_vacuum_fixture;
use workspace::{
    EngineBuilder, HostContext, HostFilesystemProvider, WorkspaceLoadRequest,
};
use tempfile::tempdir;

#[test]
#[ignore = "local showcase: bash scripts/fetch-robot-vacuum-cleaner.sh then cargo test -- --ignored"]
fn robot_vacuum_snapshot_validates_and_prepares_product_structure() {
    let (root, model_dir) = require_robot_vacuum_fixture();

    let cache = tempdir().expect("cache");
    let engine = EngineBuilder::default()
        .cache_dir(cache.path().to_path_buf())
        .no_stdlib(true)
        .build()
        .expect("engine");

    let provider = HostFilesystemProvider::from_paths(&model_dir, Some(root.as_path()), &[]);
    let snapshot = engine
        .load_workspace(
            provider,
            WorkspaceLoadRequest::single_target(model_dir.clone())
                .with_workspace_root(Some(root.clone())),
            HostContext::default(),
        )
        .expect("snapshot");

    assert!(
        snapshot.validation().summary.document_count > 0,
        "expected target validation documents"
    );

    let probe = snapshot
        .prepare_view("general-view", None)
        .expect("general-view probe");
    let model_views: Vec<_> = probe
        .view_candidates
        .iter()
        .filter(|candidate| candidate.id.starts_with("ModelViews::"))
        .collect();
    assert_eq!(
        model_views.len(),
        3,
        "expected exactly 3 ModelViews catalog views"
    );

    let product_structure = snapshot
        .prepare_view("general-view", Some("productStructure"))
        .expect("productStructure view");
    assert!(
        product_structure.empty_state_message.is_none(),
        "productStructure should render: {:?}",
        product_structure.empty_state_message
    );
}
