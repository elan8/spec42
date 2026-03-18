use std::fs;

use elk_core::{LayoutDirection, LayoutOptions, ViewProfile};
use elk_graph_json::import_str;
use elk_layered::layout;

fn read_fixture(name: &str) -> String {
    let path = format!("{}/fixtures/elkjson/{}", env!("CARGO_MANIFEST_DIR"), name);
    fs::read_to_string(path).expect("fixture should be readable")
}

#[test]
fn view_profile_defaults_are_applied() {
    let general = LayoutOptions::default().with_view_profile(ViewProfile::GeneralView);
    let interconnection = LayoutOptions::default().with_view_profile(ViewProfile::InterconnectionView);

    assert_eq!(general.layered.direction, LayoutDirection::TopToBottom);
    assert_eq!(interconnection.layered.direction, LayoutDirection::LeftToRight);
    assert!(general.layered.spacing.node_spacing > interconnection.layered.spacing.node_spacing);
    assert!(
        interconnection.layered.preferred_connector_lanes
            > general.layered.preferred_connector_lanes
    );
}

#[test]
fn layered_layout_runs_on_fixture_graphs() {
    for fixture in [
        "direction_down.json",
        "ports_and_constraints.json",
        "port_order_index.json",
        "layer_spacing_large.json",
    ] {
        let json = read_fixture(fixture);
        let mut g = import_str(&json).expect("import should succeed").graph;
        let report = layout(&mut g, &LayoutOptions::default()).expect("layout should succeed");
        assert!(report.stats.layers >= 1);

        // Bounds are stored on the synthetic root node geometry.
        let root = g.nodes[g.root.index()].geometry;
        assert!(root.width.is_finite());
        assert!(root.height.is_finite());
    }
}

#[test]
fn tree_layout_runs_on_fixture_graphs_via_service() {
    use elk_graph_json::import_str;

    for fixture in ["tree_simple.json", "tree_wide.json", "tree_compound.json"] {
        let json = read_fixture(fixture);
        let mut g = import_str(&json).expect("import should succeed").graph;
        let report = elk_service::LayoutService::default_registry()
            .layout(&mut g, &LayoutOptions::default())
            .expect("tree layout should succeed");

        let root = g.nodes[g.root.index()].geometry;
        assert!(root.width.is_finite());
        assert!(root.height.is_finite());

        // Ensure at least one edge was routed when edges exist.
        if !g.edges.is_empty() {
            assert!(
                g.edges.iter().any(|e| !e.sections.is_empty()),
                "expected routed edge sections"
            );
        }

        // Avoid unused warning on report.
        let _ = report;
    }
}

#[test]
fn rectpacking_layout_runs_and_is_non_overlapping() {
    use elk_graph_json::import_str;

    let json = read_fixture("rectpacking_simple.json");
    let mut g = import_str(&json).expect("import should succeed").graph;
    elk_service::LayoutService::default_registry()
        .layout(&mut g, &LayoutOptions::default())
        .expect("rectpacking layout should succeed");

    // Non-overlap for top-level nodes.
    let nodes: Vec<_> = g
        .nodes
        .iter()
        .filter(|n| n.parent == Some(g.root) && n.id != g.root)
        .map(|n| n.id)
        .collect();

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            let a = &g.nodes[nodes[i].index()].geometry;
            let b = &g.nodes[nodes[j].index()].geometry;
            let overlap_x = a.x < b.x + b.width && b.x < a.x + a.width;
            let overlap_y = a.y < b.y + b.height && b.y < a.y + a.height;
            assert!(
                !(overlap_x && overlap_y),
                "expected no overlap between {:?} and {:?}",
                nodes[i],
                nodes[j]
            );
        }
    }

    let root = g.nodes[g.root.index()].geometry;
    assert!(root.width.is_finite());
    assert!(root.height.is_finite());
}

#[test]
fn topdownpacking_layout_runs_on_compound_fixture() {
    let json = read_fixture("topdownpacking_compound.json");
    let mut g = import_str(&json).expect("import should succeed").graph;
    elk_service::LayoutService::default_registry()
        .layout(&mut g, &LayoutOptions::default())
        .expect("topdownpacking layout should succeed");

    let root = g.nodes[g.root.index()].geometry;
    assert!(root.width.is_finite());
    assert!(root.height.is_finite());
    assert!(root.width >= 1.0);
    assert!(root.height >= 1.0);

    // All nodes should have finite geometry.
    for node in &g.nodes {
        assert!(node.geometry.width.is_finite());
        assert!(node.geometry.height.is_finite());
    }
}

#[test]
fn libavoid_routes_edges_and_avoids_obstacles() {
    let json = read_fixture("libavoid_obstacles.json");
    let mut g = import_str(&json).expect("import should succeed").graph;
    elk_service::LayoutService::default_registry()
        .layout(&mut g, &LayoutOptions::default())
        .expect("libavoid layout should succeed");

    // Every edge should have at least one section with finite geometry.
    for edge in &g.edges {
        assert!(!edge.sections.is_empty(), "each edge should have routed sections");
        for &sid in &edge.sections {
            let s = &g.edge_sections[sid.index()];
            assert!(s.start.x.is_finite());
            assert!(s.start.y.is_finite());
            assert!(s.end.x.is_finite());
            assert!(s.end.y.is_finite());
        }
    }

    // Invariant: no routed segment may cross the interior of any node (except the edge endpoints).
    elk_testkit::assert_routed_paths_avoid_obstacles(&g, 1e-3);
}

#[test]
fn libavoid_narrow_corridor_avoids_obstacles() {
    let json = read_fixture("libavoid_narrow.json");
    let mut g = import_str(&json).expect("import should succeed").graph;
    elk_service::LayoutService::default_registry()
        .layout(&mut g, &LayoutOptions::default())
        .expect("libavoid layout should succeed");
    elk_testkit::assert_routed_paths_avoid_obstacles(&g, 1e-3);
}

#[test]
fn layered_with_libavoid_backend_routes_and_avoids_obstacles() {
    let json = read_fixture("layered_libavoid.json");
    let mut g = import_str(&json).expect("import should succeed").graph;
    layout(&mut g, &LayoutOptions::default()).expect("layered layout should succeed");

    assert!(
        g.edges.iter().any(|e| !e.sections.is_empty()),
        "at least one edge should have routed sections"
    );
    for edge in &g.edges {
        assert!(!edge.sections.is_empty(), "each edge should have routed sections");
    }
    elk_testkit::assert_routed_paths_avoid_obstacles(&g, 1e-3);
}

