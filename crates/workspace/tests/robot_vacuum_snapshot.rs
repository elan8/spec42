#[path = "../../../tests/fixtures/robot_vacuum_fixture.rs"]
mod robot_vacuum_fixture;

use robot_vacuum_fixture::require_robot_vacuum_fixture;
use tempfile::tempdir;
use workspace::{EngineBuilder, HostContext, HostFilesystemProvider, WorkspaceLoadRequest};

/// Lean view-catalog smoke for the pinned showcase. Diagnostic zero-warning is gated via
/// `spec42 check` in CI (same path as the product CLI), not the host snapshot builder.
#[test]
#[ignore = "CI fetches the pin; locally: bash scripts/fetch-robot-vacuum-cleaner.sh then cargo test -p workspace --test robot_vacuum_snapshot -- --ignored"]
fn robot_vacuum_snapshot_validates_and_prepares_product_decomposition() {
    let (root, model_dir) = require_robot_vacuum_fixture();

    let cache = tempdir().expect("cache");
    let engine = EngineBuilder::default()
        .cache_dir(cache.path().to_path_buf())
        .embed_domain_libraries()
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
    let expected_views = [
        "productDecomposition",
        "interconnections",
        "firmwareRuntime",
        "requirementsTraceability",
        "cliffSafeStopGoldenThread",
        "selectedParts",
    ];
    let model_views: Vec<_> = probe
        .view_candidates
        .iter()
        .filter(|candidate| {
            candidate.id.starts_with("ModelViews::")
                && expected_views.contains(&candidate.name.as_str())
        })
        .collect();
    assert!(
        model_views.len() == 6,
        "expected the 6 public lean ModelViews catalog views, got {}",
        model_views.len()
    );

    let product_decomposition = snapshot
        .prepare_view("general-view", Some("productDecomposition"))
        .expect("productDecomposition view");
    assert!(
        product_decomposition.empty_state_message.is_none(),
        "productDecomposition should render: {:?}",
        product_decomposition.empty_state_message
    );
}
