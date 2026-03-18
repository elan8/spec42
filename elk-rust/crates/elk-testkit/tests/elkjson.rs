use std::fs;

use elk_core::{EdgeRouting, LayoutDirection, LayoutEngine, LayoutOptions, PortConstraint, PortSide};
use elk_graph_json::import_str;
use elk_layered::LayeredLayoutEngine;

fn read_fixture(name: &str) -> String {
    let path = format!("{}/fixtures/elkjson/{}", env!("CARGO_MANIFEST_DIR"), name);
    fs::read_to_string(path).expect("fixture should be readable")
}

#[test]
fn json_layout_options_set_graph_level_defaults() {
    let json = read_fixture("direction_down.json");
    let imported = import_str(&json).expect("import should succeed");

    // Graph-level options are stored on `graph.layout` and then applied by LayeredLayoutEngine.
    assert_eq!(
        imported.graph.layout.direction,
        Some(LayoutDirection::TopToBottom)
    );
}

#[test]
fn json_port_constraints_and_port_sides_are_applied() {
    let json = read_fixture("ports_and_constraints.json");
    let imported = import_str(&json).expect("import should succeed");
    let graph = imported.graph;

    assert_eq!(
        graph.layout.port_constraint,
        Some(PortConstraint::FixedOrder),
        "root `elk.portConstraints` should map to graph default port constraints"
    );

    let p_out = graph.ports.iter().find(|p| p.id.index() == 0).unwrap();
    let p_in = graph.ports.iter().find(|p| p.id.index() == 1).unwrap();
    assert_eq!(p_out.side, PortSide::East);
    assert_eq!(p_in.side, PortSide::West);

    // Edge-level edge routing override should be parsed.
    assert_eq!(
        graph.edges[0].layout.edge_routing,
        Some(EdgeRouting::Straight),
        "POLYLINE is approximated as Straight routing in elk-core"
    );
}

#[test]
fn imported_fixtures_can_be_laid_out() {
    for fixture in ["direction_down.json", "ports_and_constraints.json"] {
        let json = read_fixture(fixture);
        let mut imported = import_str(&json).expect("import should succeed").graph;
        let report = LayeredLayoutEngine::new()
            .layout(&mut imported, &LayoutOptions::default())
            .expect("layered layout should succeed");
        assert!(report.stats.layers >= 1);
        assert!(imported.bounds.size.width.is_finite());
    }
}

