//! The repository diagram generator, driven the way a host drives it: list the typed view
//! catalog, hand the guest one catalog handle, and read back a complete scene.
//!
//! A standard view is typed by `StandardViewDefinitions::…`, so the catalog is only populated
//! with the standard library loaded -- as every host loads it. The guest takes a *catalog
//! handle* (`h:<sha256>`), never a view-kind id such as `general-view`, and a handle is minted by
//! `GeneratorModelView::diagram_views` on the exact model view the guest then runs against: it is
//! not transferable to another process or another view of the same publication. That is why this
//! smoke runs in-process against one `GeneratorModelView`, exactly as the LSP host does.
use std::path::{Path, PathBuf};
use std::sync::Arc;

use generator_api::{ArtifactLimits, DiagramViewKind, GeneratorModelView, QueryLimits};
use generator_host::{CancellationHandle, GeneratorRuntime, RuntimeLimits};
use spec42::cli::Cli;
use spec42::host_snapshot::load_snapshot_for_paths;

use crate::common::with_isolated_data_dir;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The plugin as `scripts/build-repository-generator-plugins.sh` leaves it; absent, the test
/// skips itself the way the conformance CLI tests do, so a bare `cargo test` stays green.
fn diagram_plugin() -> Option<Vec<u8>> {
    let path = repo_root().join(
        "generator-plugins/target/wasm32-unknown-unknown/release/spec42_diagram_generator.wasm",
    );
    std::fs::read(path).ok()
}

fn stdlib_cli() -> Cli {
    Cli {
        config_path: None,
        library_paths: vec![],
        stdlib_path: None,
        kpar_library_paths: Vec::new(),
        disabled_kpar_libraries: Vec::new(),
        no_stdlib: false,
        stdio: false,
        command: None,
    }
}

#[test]
fn an_authored_standard_view_generates_a_complete_scene_from_its_catalog_handle() {
    let Some(module) = diagram_plugin() else {
        eprintln!("skipping: run scripts/build-repository-generator-plugins.sh first");
        return;
    };
    with_isolated_data_dir(|| {
        let workspace = repo_root().join("vscode/testFixture/workspaces/state-view");
        let views_document = workspace.join("Views.sysml");

        let cli = stdlib_cli();
        let snapshot = load_snapshot_for_paths(&cli, &views_document, Some(&workspace), false)
            .expect("the state-view fixture publishes");
        let publication = snapshot.published_model_arc();
        let model = Arc::new(
            GeneratorModelView::new(
                Arc::clone(&publication),
                publication.publication().model_digest().to_string(),
                env!("CARGO_PKG_VERSION"),
                QueryLimits::default(),
            )
            .expect("complete generator model"),
        );

        // The handle is minted by this catalog listing, on this model view.
        let views = model.diagram_views().expect("the view catalog lists");
        let view = views
            .iter()
            .find(|view| view.kind == DiagramViewKind::StateTransitionView)
            .unwrap_or_else(|| {
                panic!("the fixture authors a StateTransitionView; catalog: {views:?}")
            });
        assert!(
            view.handle.starts_with("h:"),
            "a catalog handle is opaque, got {}",
            view.handle
        );

        let runtime = GeneratorRuntime::new().expect("generator runtime");
        let prepared = runtime
            .prepare(&module)
            .expect("the diagram plugin is a valid module");
        let execution = runtime
            .execute_prepared(
                &prepared,
                Arc::clone(&model),
                std::slice::from_ref(&view.handle),
                RuntimeLimits::default(),
                ArtifactLimits::default(),
                CancellationHandle::new(),
            )
            .unwrap_or_else(|error| panic!("the diagram guest generates: {error}"));

        let diagram = execution
            .artifacts
            .entries()
            .find(|(path, _)| path.as_str() == "diagram.json")
            .map(|(_, bytes)| String::from_utf8(bytes.to_vec()).expect("diagram.json is UTF-8"))
            .expect("the guest writes diagram.json");
        let product: serde_json::Value =
            serde_json::from_str(&diagram).expect("diagram.json is JSON");
        assert_eq!(product["selectedView"]["kind"], "state-transition-view");
        assert_eq!(product["projection"]["kind"], "state-transition-view");
        assert_eq!(
            product["completeness"]["status"], "complete",
            "the authored view of the fixture projects completely: {}",
            product["completeness"]
        );
        assert!(
            !product["projection"]["nodes"]
                .as_array()
                .expect("nodes")
                .is_empty(),
            "the exposed state machine yields nodes"
        );
    });
}
