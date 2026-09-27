//! Native diagram products, built the way the LSP and `spec42 diagram` build them: list the typed
//! view catalog, project one catalog handle, and normalize it with `diagram_product`.
//!
//! A standard view is typed by `StandardViewDefinitions::…`, so the catalog is only populated
//! with the standard library loaded -- as every host loads it. A view is selected by its
//! *catalog handle* (`h:<sha256>`), never a view-kind id such as `general-view`, and a handle is minted by
//! `GeneratorModelView::diagram_views` on the exact model view it is then projected on: it is not
//! transferable to another process or another view of the same publication. That is why this
//! smoke runs in-process against one `GeneratorModelView`, exactly as the LSP host does.
use std::path::{Path, PathBuf};
use std::sync::Arc;

use generator_api::{DiagramViewKind, GeneratorModelView, QueryLimits};
use spec42::cli::{Cli, DiagramArgs, DiagramFormat};
use spec42::host_snapshot::load_snapshot_for_paths;

use crate::common::with_isolated_data_dir;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// `diagram.json` for the catalog view `handle`, as the LSP's `spec42/diagram` builds it.
fn native_product(model: &GeneratorModelView, handle: &str) -> String {
    let projection = model
        .diagram_view(handle)
        .unwrap_or_else(|error| panic!("the catalog view projects: {error}"));
    String::from_utf8(diagram_product::diagram_product_json(&projection).expect("product"))
        .expect("diagram.json is UTF-8")
}

fn stdlib_cli() -> Cli {
    Cli {
        config_path: None,
        library_paths: vec![],
        stdlib_path: None,
        kpar_library_paths: Vec::new(),
        project_libraries: Vec::new(),
        disabled_kpar_libraries: Vec::new(),
        no_stdlib: false,
        stdio: false,
        command: None,
    }
}

#[test]
fn an_authored_standard_view_projects_a_complete_scene_from_its_catalog_handle() {
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

        let diagram = native_product(&model, &view.handle);
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

#[test]
fn a_sequence_view_projects_lifelines_messages_and_their_order() {
    with_isolated_data_dir(|| {
        let workspace = repo_root().join("vscode/testFixture/workspaces/sequence-view");
        let model_document = workspace.join("Model.sysml");

        let cli = stdlib_cli();
        let snapshot = load_snapshot_for_paths(&cli, &model_document, Some(&workspace), false)
            .expect("the sequence-view fixture publishes");
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

        let views = model.diagram_views().expect("the view catalog lists");
        let view = views
            .iter()
            .find(|view| view.kind == DiagramViewKind::SequenceView)
            .unwrap_or_else(|| panic!("the fixture authors a SequenceView; catalog: {views:?}"));

        let diagram = native_product(&model, &view.handle);
        let product: serde_json::Value =
            serde_json::from_str(&diagram).expect("diagram.json is JSON");

        assert_eq!(product["selectedView"]["kind"], "sequence-view");
        assert_eq!(
            product["completeness"]["status"], "complete",
            "every message end and succession resolves: {}",
            product["completeness"]
        );

        let metadata = &product["projection"]["metadata"];
        assert_eq!(
            metadata["participants"].as_array().map(Vec::len),
            Some(3),
            "only the three lifelines are participants -- not the ports or nested parts of \
             their types: {metadata}"
        );
        assert_eq!(
            metadata["messages"].as_array().map(Vec::len),
            Some(4),
            "each `message` usage is classified as a message, not left unrecognised: {metadata}"
        );

        let edge_kinds: std::collections::BTreeSet<&str> = product["projection"]["edges"]
            .as_array()
            .expect("edges")
            .iter()
            .filter_map(|edge| edge["kind"].as_str())
            .collect();
        assert!(
            edge_kinds.contains("flow"),
            "message send/receive ends project as flow edges: {edge_kinds:?}"
        );
        assert!(
            edge_kinds.contains("succession"),
            "authored message order projects as succession edges: {edge_kinds:?}"
        );

        let scene = &product["projection"]["scene"];
        assert_eq!(scene["kind"], "sequence");
        assert_eq!(scene["lifelines"].as_array().map(Vec::len), Some(3));
        let messages = scene["messages"].as_array().expect("sequence messages");
        assert_eq!(messages.len(), 4);
        let mut orders = Vec::new();
        for message in messages {
            assert_eq!(message["source"]["status"], "resolved");
            assert!(message["source"]["lifeline"].is_number());
            assert_eq!(message["target"]["status"], "resolved");
            assert!(message["target"]["lifeline"].is_number());
            assert_eq!(message["order"]["status"], "resolved");
            orders.push(message["order"]["value"].as_u64().expect("numeric order"));
        }
        orders.sort_unstable();
        assert_eq!(orders, vec![1, 2, 3, 4]);

        let svg = spec42::headless_renderer::render_shared_svg(&diagram)
            .unwrap_or_else(|error| panic!("the generated SequenceView renders as SVG: {error}"));
        assert_eq!(svg.matches("class=\"sequence-lifeline\"").count(), 3);
        assert_eq!(
            svg.matches("<line class=\"sequence-message").count()
                + svg.matches("<path class=\"sequence-message").count(),
            4
        );
        for label in [
            "apiGateway",
            "storefront",
            "checkoutService",
            "submitCheckout",
            "forwardCheckout",
            "checkoutOutcome",
            "apiResponse",
        ] {
            assert!(svg.contains(label), "the SVG contains {label}: {svg}");
        }
        assert!(!svg.contains("NaN"), "the SVG has finite geometry: {svg}");
    });
}

#[test]
fn the_diagram_command_lists_views_and_renders_one_by_qualified_name() {
    with_isolated_data_dir(|| {
        let workspace = repo_root().join("vscode/testFixture/workspaces/state-view");
        let output = tempfile::tempdir().expect("output directory");
        let run = |view: Option<&str>, format: DiagramFormat, file: &str| {
            let path = output.path().join(file);
            spec42::diagram::run_diagram(
                &stdlib_cli(),
                &DiagramArgs {
                    path: workspace.clone(),
                    workspace_root: Some(workspace.clone()),
                    view: view.map(str::to_owned),
                    format,
                    output: Some(path.clone()),
                },
            )
            .unwrap_or_else(|error| panic!("spec42 diagram: {error}"));
            std::fs::read_to_string(path).expect("command output")
        };

        let listing: serde_json::Value =
            serde_json::from_str(&run(None, DiagramFormat::Json, "views.json")).expect("JSON");
        let view = listing
            .as_array()
            .expect("view list")
            .iter()
            .find(|view| view["kind"] == "state-transition-view")
            .unwrap_or_else(|| panic!("the fixture lists its StateTransitionView: {listing}"));
        let name = view["qualifiedName"].as_str().expect("qualified name");
        assert!(run(None, DiagramFormat::Svg, "views.txt").contains(name));

        let product: serde_json::Value =
            serde_json::from_str(&run(Some(name), DiagramFormat::Json, "diagram.json"))
                .expect("product JSON");
        assert_eq!(product["schemaVersion"], 5);
        assert_eq!(product["selectedView"]["kind"], "state-transition-view");

        let svg = run(Some(name), DiagramFormat::Svg, "diagram.svg");
        assert!(svg.starts_with("<svg"), "an SVG document: {svg}");
        assert!(!svg.contains("NaN"), "the SVG has finite geometry");

        let error = spec42::diagram::run_diagram(
            &stdlib_cli(),
            &DiagramArgs {
                path: workspace.clone(),
                workspace_root: Some(workspace.clone()),
                view: Some("No::such".to_owned()),
                format: DiagramFormat::Svg,
                output: None,
            },
        )
        .expect_err("an unknown view is an error");
        assert!(
            error.contains(name),
            "the error lists the available views: {error}"
        );
    });
}
