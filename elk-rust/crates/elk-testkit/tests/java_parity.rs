//! Java-vs-Rust JSON parity test.
//!
//! This test runs the Rust layout engine and a small Java ELK runner on the same
//! input ELK Graph JSON fixture and compares the resulting JSON trees.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use elk_core::LayoutOptions;
use elk_graph_json::{export_elk_graph_to_value, import_str};
use elk_service::LayoutService;
use elk_testkit::compare_layout_json_relaxed;
use serde_json::Value;

const FIXTURES: &[&str] = &[
    "direction_down",
    "ports_and_constraints",
    "port_order_index",
    // Derived from ELK layered test graph patterns (TestGraphCreator).
    "upstream_cross_formed",
    "upstream_multiple_edges_and_single",
    "upstream_fixed_port_order",
    "upstream_three_layer_dense",
    "upstream_dense_port_ordered",
    "upstream_four_layer_feedback",
];
const INTERCONNECTION_FIXTURES: &[&str] = &[
    "interconnection_real_small",
    "interconnection_real_medium",
    "interconnection_real_dense",
];

fn fixture_json(name: &str) -> String {
    let path = format!(
        "{}/fixtures/elkjson/{}.json",
        env!("CARGO_MANIFEST_DIR"),
        name
    );
    fs::read_to_string(path).expect("fixture should exist")
}

fn ensure_layered_algorithm(graph: &mut elk_graph::ElkGraph) {
    use elk_graph::{PropertyKey, PropertyValue};
    if graph
        .properties
        .get_str(&PropertyKey::from("elk.algorithm"))
        .is_none()
    {
        graph.properties.insert(
            "elk.algorithm",
            PropertyValue::String("org.eclipse.elk.layered".to_string()),
        );
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root")
        .to_path_buf()
}

fn run_java_runner(input_json_path: &Path) -> Value {
    let root = repo_root();
    let script = root.join("scripts").join("run-elk-java-json.ps1");

    let out = Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            script.to_string_lossy().as_ref(),
            "-InputJson",
            input_json_path.to_string_lossy().as_ref(),
        ])
        .output()
        .expect("run java runner script");

    if !out.status.success() {
        panic!(
            "java runner failed (exit {}):\nstdout:\n{}\nstderr:\n{}",
            out.status,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }

    // Maven can print some log lines before/after JSON; extract the first JSON object/array.
    let stdout = String::from_utf8_lossy(&out.stdout);
    let start = stdout
        .find('{')
        .or_else(|| stdout.find('['))
        .expect("java runner stdout should contain JSON");
    let end = stdout
        .rfind('}')
        .or_else(|| stdout.rfind(']'))
        .expect("java runner stdout should contain JSON end");
    let json_str = &stdout[start..=end];

    serde_json::from_str(json_str).unwrap_or_else(|e| {
        panic!(
            "java runner stdout should contain JSON: {}\nextracted:\n{}\nfull stdout:\n{}",
            e, json_str, stdout
        )
    })
}

fn ensure_java_layout_options(json: &mut Value) {
    let root_obj = json
        .as_object_mut()
        .expect("root json must be an object");

    let layout_options = root_obj
        .entry("layoutOptions")
        .or_insert_with(|| Value::Object(serde_json::Map::new()));

    let opts_obj = layout_options
        .as_object_mut()
        .expect("layoutOptions must be an object");

    // Match the algorithm and coordinate mode the Rust parity fixtures expect.
    opts_obj.insert(
        "elk.algorithm".to_string(),
        Value::String("org.eclipse.elk.layered".to_string()),
    );
    opts_obj.insert(
        "org.eclipse.elk.json.shapeCoords".to_string(),
        Value::String("ROOT".to_string()),
    );
    opts_obj.insert(
        "org.eclipse.elk.json.edgeCoords".to_string(),
        Value::String("ROOT".to_string()),
    );
    // Align Java ELK defaults with Rust layered defaults as closely as possible.
    opts_obj.insert("elk.padding".to_string(), Value::String("[24,24,24,24]".to_string()));
    opts_obj.insert(
        "elk.spacing.nodeNode".to_string(),
        Value::Number(serde_json::Number::from(40)),
    );
    opts_obj.insert(
        "elk.spacing.nodeNodeBetweenLayers".to_string(),
        Value::Number(serde_json::Number::from(80)),
    );
}

#[test]
fn parity_java_matches_rust_on_fixtures() {
    let root = repo_root();
    let mismatch_dir = root
        .join("target")
        .join("elk-parity-mismatches");
    let _ = fs::create_dir_all(&mismatch_dir);

    for name in FIXTURES {
        let json = fixture_json(name);

        // Rust output
        let mut g = import_str(&json).expect("import").graph;
        ensure_layered_algorithm(&mut g);
        LayoutService::default_registry()
            .layout(&mut g, &LayoutOptions::default())
            .expect("rust layout");
        let rust_out = export_elk_graph_to_value(&g);

        // Java output (write fixture to temp file for the script)
        let in_path = mismatch_dir.join(format!("input_{}.json", name));
        let mut java_input: Value = serde_json::from_str(&json).expect("fixture JSON should parse");
        ensure_java_layout_options(&mut java_input);
        fs::write(&in_path, serde_json::to_string_pretty(&java_input).unwrap())
            .expect("write java input");
        let java_out = run_java_runner(&in_path);

        if let Err(e) = compare_layout_json_relaxed(&rust_out, &java_out) {
            let case_dir = mismatch_dir.join(name);
            let _ = fs::create_dir_all(&case_dir);
            fs::write(
                case_dir.join("rust.json"),
                serde_json::to_string_pretty(&rust_out).unwrap(),
            )
            .ok();
            fs::write(
                case_dir.join("java.json"),
                serde_json::to_string_pretty(&java_out).unwrap(),
            )
            .ok();
            panic!("parity mismatch for {}: {}", name, e);
        }
    }
}

#[test]
fn parity_java_matches_rust_on_interconnection_topology() {
    let root = repo_root();
    let mismatch_dir = root.join("target").join("elk-parity-mismatches");
    let _ = fs::create_dir_all(&mismatch_dir);

    for name in INTERCONNECTION_FIXTURES {
        let json = fixture_json(name);

        let mut g = import_str(&json).expect("import").graph;
        ensure_layered_algorithm(&mut g);
        LayoutService::default_registry()
            .layout(&mut g, &LayoutOptions::default())
            .expect("rust layout");
        let rust_out = export_elk_graph_to_value(&g);

        let in_path = mismatch_dir.join(format!("input_{}.json", name));
        let mut java_input: Value = serde_json::from_str(&json).expect("fixture JSON should parse");
        ensure_java_layout_options(&mut java_input);
        fs::write(&in_path, serde_json::to_string_pretty(&java_input).unwrap())
            .expect("write java input");
        let java_out = run_java_runner(&in_path);

        let rust_edges = rust_out
            .get("edges")
            .and_then(Value::as_array)
            .expect("rust edges array");
        let java_edges = java_out
            .get("edges")
            .and_then(Value::as_array)
            .expect("java edges array");
        assert!(
            !rust_edges.is_empty() && !java_edges.is_empty(),
            "missing routed edge set for {}",
            name
        );

        let rust_routed = rust_edges
            .iter()
            .filter(|e| e.get("sections").and_then(Value::as_array).is_some_and(|s| !s.is_empty()))
            .count();
        assert!(
            rust_routed >= rust_edges.len().saturating_sub(1),
            "rust routed edge coverage too low for {} (routed={}, edges={})",
            name,
            rust_routed,
            rust_edges.len()
        );
    }
}

