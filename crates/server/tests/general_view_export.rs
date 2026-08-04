//! Issue #21: a valid filter-less `GeneralView` expose (`expose Pkg::thing;`, no `filter`
//! members) used to export an empty SVG -- `viz-nodes`/`viz-edges` both empty even though the
//! Rust-side projection (`project_view`/`ProjedtedView`) correctly resolved the one exposed node.
//! The gap was downstream, in the vendored headless-renderer JS bundle
//! (`crates/server/assets/diagram-renderer/headless-renderer.js`) drifting out of sync with its
//! `shared/diagram-renderer/src` source -- see the `F-9` note in
//! `crates/sysml_model/src/semantic/prepared_view/from_visualization.rs`. This exercises the
//! exact `spec42 diagrams export --format svg` path end to end (Rust projection + the embedded
//! QuickJS bundle) so a future drift between the two regresses a test, not just a user's export.

use std::fs;

use spec42::cli::{Cli, DiagramExportFormat};
use spec42::diagrams::{build_diagram_payload, render_diagram};
use tempfile::TempDir;

#[test]
fn filter_less_general_view_expose_renders_the_exposed_node() {
    let temp = TempDir::new().expect("temp workspace");
    let root = temp.path().to_path_buf();
    fs::write(
        root.join("model.sysml"),
        r#"package Demo {
  part def Widget;
  part thing : Widget;

  view structure : GeneralView {
    expose Demo::thing;
  }
}"#,
    )
    .expect("write model.sysml");

    let cli = Cli {
        config_path: None,
        library_paths: vec![],
        stdlib_path: None,
        kpar_library_paths: Vec::new(),
        disabled_kpar_libraries: Vec::new(),
        no_stdlib: true,
        stdio: false,
        command: None,
    };

    let payload = build_diagram_payload(
        &cli,
        root.join("model.sysml").as_path(),
        Some(root.as_path()),
        "general-view",
        Some("structure"),
    )
    .expect("diagram payload should build");

    let general_view_graph = payload
        .general_view_graph
        .as_ref()
        .expect("general-view export should populate general_view_graph");
    assert_eq!(
        general_view_graph.nodes.len(),
        1,
        "expected exactly the exposed node in general_view_graph, got {:?}",
        general_view_graph.nodes
    );

    let (svg, _content_type) =
        render_diagram(&payload, DiagramExportFormat::Svg).expect("svg should render");
    assert!(
        svg.contains("Demo::thing") && svg.contains("viz-node"),
        "expected the exposed node to appear in the rendered SVG, got: {svg}"
    );
}
