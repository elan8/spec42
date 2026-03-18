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

