//! Java-vs-Rust JSON parity tests.
//!
//! Compares Rust layout output to checked-in expected JSON (produced by Rust or Java ELK).
//! To regenerate expected files: set env REGENERATE_PARITY=1 and run this test.

use std::fs;
use std::path::Path;

use elk_core::LayoutOptions;
use elk_graph_json::{export_elk_graph_to_value, import_str};
use elk_service::LayoutService;
use elk_testkit::{compare_layout_json, compare_layout_json_relaxed};
use serde_json::Value;

const PARITY_FIXTURES: &[&str] = &["direction_down", "ports_and_constraints", "port_order_index"];
const COORD_EPS: f32 = 1e-2;

fn fixture_path(name: &str, suffix: &str) -> String {
    format!(
        "{}/fixtures/elkjson/{}{}",
        env!("CARGO_MANIFEST_DIR"),
        suffix,
        name
    )
}

fn load_input(name: &str) -> (elk_graph::ElkGraph, String) {
    let path = fixture_path(name, "");
    let path = format!("{}.json", path);
    let json = fs::read_to_string(&path).expect("input fixture should exist");
    let imp = import_str(&json).expect("import should succeed");
    (imp.graph, json)
}

fn ensure_layered_algorithm(graph: &mut elk_graph::ElkGraph) {
    use elk_graph::{PropertyKey, PropertyValue};
    if graph.properties.get_str(&PropertyKey::from("elk.algorithm")).is_none() {
        graph.properties.insert(
            "elk.algorithm",
            PropertyValue::String("org.eclipse.elk.layered".to_string()),
        );
    }
}

#[test]
fn parity_rust_output_matches_expected() {
    let regenerate = std::env::var("REGENERATE_PARITY").as_deref() == Ok("1");
    let parity_dir = format!("{}/fixtures/elkjson/parity", env!("CARGO_MANIFEST_DIR"));

    for name in PARITY_FIXTURES {
        let (mut graph, _) = load_input(name);
        ensure_layered_algorithm(&mut graph);

        LayoutService::default_registry()
            .layout(&mut graph, &LayoutOptions::default())
            .expect("layout should succeed");

        let actual = export_elk_graph_to_value(&graph);
        let expected_path = Path::new(&parity_dir).join(format!("expected_{}.json", name));

        if regenerate {
            fs::create_dir_all(&parity_dir).ok();
            let pretty =
                serde_json::to_string_pretty(&actual).expect("serialize");
            fs::write(&expected_path, pretty).expect("write expected file");
            continue;
        }

        let expected_str = fs::read_to_string(&expected_path)
            .unwrap_or_else(|_| panic!("expected file missing: {:?}. Run with REGENERATE_PARITY=1 to create.", expected_path));
        let expected: Value =
            serde_json::from_str(&expected_str).expect("expected JSON should be valid");

        let cmp = if *name == "port_order_index" {
            compare_layout_json_relaxed(&actual, &expected)
        } else {
            compare_layout_json(&actual, &expected, COORD_EPS)
        };
        cmp.unwrap_or_else(|e| panic!("parity mismatch for {}: {}", name, e));
    }
}
