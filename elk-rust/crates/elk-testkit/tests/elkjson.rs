use std::fs;

use elk_core::{LayoutEngine, LayoutOptions, PortSide};
use elk_graph::{PropertyValue};
use elk_graph_json::{import_str, import_str_core};
use elk_layered::LayeredLayoutEngine;

fn read_fixture(name: &str) -> String {
    let path = format!("{}/fixtures/elkjson/{}", env!("CARGO_MANIFEST_DIR"), name);
    fs::read_to_string(path).expect("fixture should be readable")
}

#[test]
fn json_layout_options_set_graph_level_defaults() {
    let json = read_fixture("direction_down.json");
    let imported = import_str(&json).expect("import should succeed");

    let dir = imported
        .graph
        .properties
        .get(&elk_graph::PropertyKey("elk.direction".to_string()));
    assert!(
        matches!(dir, Some(PropertyValue::String(s)) if s == "DOWN"),
        "expected root layoutOptions elk.direction=DOWN, got {dir:?}"
    );
}

#[test]
fn json_port_constraints_and_port_sides_are_applied() {
    let json = read_fixture("ports_and_constraints.json");
    let imported = import_str(&json).expect("import should succeed");
    let graph = imported.graph;

    let pc = graph
        .properties
        .get(&elk_graph::PropertyKey("elk.portConstraints".to_string()));
    assert!(
        matches!(pc, Some(PropertyValue::String(s)) if s == "FIXED_ORDER"),
        "expected root elk.portConstraints=FIXED_ORDER, got {pc:?}"
    );

    let p_out = graph.ports.iter().find(|p| p.id.index() == 0).unwrap();
    let p_in = graph.ports.iter().find(|p| p.id.index() == 1).unwrap();
    assert_eq!(p_out.side, PortSide::East);
    assert_eq!(p_in.side, PortSide::West);
    let er = graph.edges[0]
        .properties
        .get(&elk_graph::PropertyKey("elk.edgeRouting".to_string()));
    assert!(
        matches!(er, Some(PropertyValue::String(s)) if s == "POLYLINE"),
        "expected edge elk.edgeRouting=POLYLINE, got {er:?}"
    );
}

#[test]
fn imported_fixtures_can_be_laid_out() {
    for fixture in ["direction_down.json", "ports_and_constraints.json"] {
        let json = read_fixture(fixture);
        let (mut imported, _warnings) = import_str_core(&json).expect("import should succeed");
        let report = LayeredLayoutEngine::new()
            .layout(&mut imported, &LayoutOptions::default())
            .expect("layered layout should succeed");
        assert!(report.stats.layers >= 1);
        assert!(imported.bounds.size.width.is_finite());
    }
}

