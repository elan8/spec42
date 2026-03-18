use std::fs;

use elk_core::{LayoutOptions, PortSide};
use elk_graph::{PropertyValue};
use elk_graph_json::import_str;
use elk_graph_json::export_elk_graph_to_value;
use elk_layered::layout;

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
    for fixture in [
        "direction_down.json",
        "ports_and_constraints.json",
        "hyperedge_and_rich_props.json",
    ] {
        let json = read_fixture(fixture);
        let mut imported = import_str(&json).expect("import should succeed").graph;
        let report = layout(&mut imported, &LayoutOptions::default())
            .expect("layered layout should succeed");
        assert!(report.stats.layers >= 1);
        let root_geom = imported.nodes[imported.root.index()].geometry;
        assert!(root_geom.width.is_finite());
    }
}

#[test]
fn rich_properties_and_hyperedge_endpoints_round_trip() {
    let json = read_fixture("hyperedge_and_rich_props.json");
    let imported = import_str(&json).expect("import should succeed");
    let graph = imported.graph;

    assert!(graph.validate().is_ok(), "imported graph should validate");
    assert_eq!(graph.edges.len(), 1);
    assert_eq!(graph.edges[0].sources.len(), 2);
    assert_eq!(graph.edges[0].targets.len(), 2);

    let value = export_elk_graph_to_value(&graph);
    let json2 = serde_json::to_string(&value).expect("export should serialize");
    let imported2 = import_str(&json2).expect("re-import should succeed");
    let graph2 = imported2.graph;

    assert!(graph2.validate().is_ok(), "round-tripped graph should validate");

    let arr = graph2
        .properties
        .get(&elk_graph::PropertyKey("elk.someArray".to_string()));
    assert!(
        matches!(arr, Some(PropertyValue::Array(v)) if v.len() == 4),
        "expected elk.someArray to round-trip as array, got {arr:?}"
    );

    let obj = graph2
        .properties
        .get(&elk_graph::PropertyKey("elk.someObject".to_string()));
    assert!(
        matches!(obj, Some(PropertyValue::Object(m)) if m.contains_key("a") && m.contains_key("b") && m.contains_key("c")),
        "expected elk.someObject to round-trip as object, got {obj:?}"
    );
}

#[test]
fn direction_option_changes_layout_orientation() {
    // DOWN should primarily separate nodes vertically.
    let json = read_fixture("direction_down.json");
    let mut g = import_str(&json).expect("import should succeed").graph;
    let _report = layout(&mut g, &LayoutOptions::default()).expect("layout should succeed");
    let n1 = g.nodes[1].geometry;
    let n2 = g.nodes[2].geometry;
    assert!(
        n2.y > n1.y,
        "expected DOWN direction to place n2 below n1; got n1={n1:?} n2={n2:?}"
    );
}

#[test]
fn fixed_order_ports_respects_port_index_ordering() {
    let json = read_fixture("port_order_index.json");
    let mut g = import_str(&json).expect("import should succeed").graph;
    let _report = layout(&mut g, &LayoutOptions::default()).expect("layout should succeed");

    // Ports were created in JSON order: p2 then p0. With FIXED_ORDER and port.index,
    // we expect p0 to appear before p2 along the EAST side (smaller y for top-to-bottom ordering).
    let p2 = &g.ports[0];
    let p0 = &g.ports[1];
    assert_eq!(p2.side, PortSide::East);
    assert_eq!(p0.side, PortSide::East);
    assert!(
        p0.geometry.y <= p2.geometry.y,
        "expected port.index=0 to be placed before index=2; got p0.y={} p2.y={}",
        p0.geometry.y,
        p2.geometry.y
    );
}

#[test]
fn spacing_option_increases_layer_gaps() {
    // Baseline with default options.
    let baseline_json = r#"
    {
      "id": "root",
      "layoutOptions": { "elk.direction": "DOWN" },
      "children": [
        { "id": "n1", "width": 80, "height": 40 },
        { "id": "n2", "width": 80, "height": 40 },
        { "id": "n3", "width": 80, "height": 40 }
      ],
      "edges": [
        { "id": "e1", "sources": ["n1"], "targets": ["n2"] },
        { "id": "e2", "sources": ["n2"], "targets": ["n3"] }
      ]
    }
    "#;
    let mut base = import_str(baseline_json).expect("import should succeed").graph;
    let _ = layout(&mut base, &LayoutOptions::default()).expect("layout should succeed");
    let b1 = base.nodes[1].geometry;
    let b2 = base.nodes[2].geometry;
    let b3 = base.nodes[3].geometry;
    let base_gap_12 = (b2.y - b1.y).abs();
    let base_gap_23 = (b3.y - b2.y).abs();

    // Configured spacing.
    let json = read_fixture("layer_spacing_large.json");
    let mut g = import_str(&json).expect("import should succeed").graph;
    let _ = layout(&mut g, &LayoutOptions::default()).expect("layout should succeed");
    let n1 = g.nodes[1].geometry;
    let n2 = g.nodes[2].geometry;
    let n3 = g.nodes[3].geometry;
    let gap_12 = (n2.y - n1.y).abs();
    let gap_23 = (n3.y - n2.y).abs();

    assert!(
        gap_12 > base_gap_12 + 80.0 && gap_23 > base_gap_23 + 80.0,
        "expected larger between-layer gaps with elk.spacing.nodeNodeBetweenLayers; base=({base_gap_12:.1},{base_gap_23:.1}) new=({gap_12:.1},{gap_23:.1})"
    );
}

