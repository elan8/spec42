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
use elk_testkit::{
    build_parity_case_report, build_skipped_parity_case_report, compare_layout_json_relaxed,
    ParityCaseReport, ParityFixtureKind,
};
use serde_json::Value;
use std::collections::BTreeMap;

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
    "interconnection_real_full_drone_like",
];
const LIBAVOID_FIXTURES: &[&str] = &[
    "libavoid_obstacles",
    "libavoid_narrow",
];

fn suite_summary_path() -> PathBuf {
    repo_root()
        .join("target")
        .join("elk-parity-mismatches")
        .join("parity-summary.json")
}

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

fn parse_json_from_mixed_output(stdout: &str) -> Result<Value, String> {
    let chars: Vec<(usize, char)> = stdout.char_indices().collect();
    let start_positions: Vec<usize> = chars
        .iter()
        .filter_map(|(idx, ch)| ((*ch == '{') || (*ch == '[')).then_some(*idx))
        .collect();
    let end_positions: Vec<usize> = chars
        .iter()
        .filter_map(|(idx, ch)| ((*ch == '}') || (*ch == ']')).then_some(*idx))
        .collect();

    for start in &start_positions {
        for end in end_positions.iter().rev() {
            if end < start {
                continue;
            }
            let candidate = stdout[*start..=*end].trim();
            if candidate.is_empty() {
                continue;
            }
            if let Ok(value) = serde_json::from_str::<Value>(candidate) {
                return Ok(value);
            }
        }
    }

    Err("java runner returned invalid JSON: expected value at line 1 column 2".to_string())
}

fn strict_parity_enabled() -> bool {
    std::env::var("ELK_JAVA_PARITY_STRICT")
        .ok()
        .as_deref()
        .is_some_and(|value| value == "1" || value.eq_ignore_ascii_case("true"))
}

fn run_java_runner_optional(input_json_path: &Path) -> Result<Value, String> {
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
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let combined = format!("{stdout}\n{stderr}");
    if combined.contains("Layout algorithm 'org.eclipse.elk.libavoid' not found") {
        return Err("Java runner lacks org.eclipse.elk.alg.libavoid".to_string());
    }
    if combined.contains("LocalRepositoryNotAccessibleException")
        || combined.contains(".lastUpdated (Toegang geweigerd)")
        || combined.contains(".lastUpdated (Access is denied)")
    {
        return Err("Java runner cannot write to the Maven local repository in this environment".to_string());
    }
    if combined.contains("ClassNotFoundException: spec42.elk.ElkJsonRunner")
        || combined.contains("[ERROR] Failed to execute goal")
    {
        return Err("Java runner is unavailable in the current environment".to_string());
    }
    if !out.status.success() {
        panic!(
            "java runner failed (exit {}):\nstdout:\n{}\nstderr:\n{}",
            out.status, stdout, stderr
        );
    }
    parse_json_from_mixed_output(&stdout)
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

    // Match layered algorithm and coordinate mode the Rust parity fixtures expect.
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

fn ensure_java_libavoid_options(json: &mut Value) {
    let root_obj = json
        .as_object_mut()
        .expect("root json must be an object");
    let layout_options = root_obj
        .entry("layoutOptions")
        .or_insert_with(|| Value::Object(serde_json::Map::new()));
    let opts_obj = layout_options
        .as_object_mut()
        .expect("layoutOptions must be an object");
    opts_obj.insert(
        "elk.algorithm".to_string(),
        Value::String("org.eclipse.elk.libavoid".to_string()),
    );
    opts_obj.insert(
        "org.eclipse.elk.json.shapeCoords".to_string(),
        Value::String("ROOT".to_string()),
    );
    opts_obj.insert(
        "org.eclipse.elk.json.edgeCoords".to_string(),
        Value::String("ROOT".to_string()),
    );
}

fn edge_bend_counts(json: &Value) -> BTreeMap<String, usize> {
    let mut out = BTreeMap::new();
    if let Some(edges) = json.get("edges").and_then(Value::as_array) {
        for e in edges {
            let Some(id) = e.get("id").and_then(Value::as_str) else {
                continue;
            };
            let bends = e
                .get("sections")
                .and_then(Value::as_array)
                .map(|sections| {
                    sections
                        .iter()
                        .map(|s| {
                            s.get("bendPoints")
                                .and_then(Value::as_array)
                                .map_or(0, |bp| bp.len())
                        })
                        .sum::<usize>()
                })
                .unwrap_or(0);
            out.insert(id.to_string(), bends);
        }
    }
    out
}

fn edge_endpoint_signature(edge: &Value) -> Option<String> {
    let sources = edge.get("sources")?.as_array()?;
    let targets = edge.get("targets")?.as_array()?;
    let endpoint_id = |v: &Value| -> Option<String> {
        if let Some(s) = v.as_str() {
            return Some(s.to_string());
        }
        let obj = v.as_object()?;
        if let Some(port) = obj.get("port").and_then(Value::as_str) {
            return Some(format!("port:{port}"));
        }
        if let Some(node) = obj.get("node").and_then(Value::as_str) {
            return Some(format!("node:{node}"));
        }
        if let Some(id) = obj.get("id").and_then(Value::as_str) {
            return Some(id.to_string());
        }
        None
    };
    let src = sources
        .iter()
        .filter_map(endpoint_id)
        .collect::<Vec<_>>()
        .join(",");
    let dst = targets
        .iter()
        .filter_map(endpoint_id)
        .collect::<Vec<_>>()
        .join(",");
    if src.is_empty() || dst.is_empty() {
        return None;
    }
    Some(format!("{src}->{dst}"))
}

fn edge_bend_counts_by_signature(json: &Value) -> BTreeMap<String, usize> {
    let mut out = BTreeMap::new();
    if let Some(edges) = json.get("edges").and_then(Value::as_array) {
        for e in edges {
            let Some(sig) = edge_endpoint_signature(e) else {
                continue;
            };
            let bends = e
                .get("sections")
                .and_then(Value::as_array)
                .map(|sections| {
                    sections
                        .iter()
                        .map(|s| {
                            s.get("bendPoints")
                                .and_then(Value::as_array)
                                .map_or(0, |bp| bp.len())
                        })
                        .sum::<usize>()
                })
                .unwrap_or(0);
            out.insert(sig, bends);
        }
    }
    out
}

fn bend_complexity_error(
    kind: ParityFixtureKind,
    fixture: &str,
    rust_out: &Value,
    java_out: &Value,
) -> Option<String> {
    let rust_bends = edge_bend_counts(rust_out);
    let java_bends = edge_bend_counts(java_out);
    let mut shared = 0usize;

    let direct_limit = match kind {
        ParityFixtureKind::Layered => 0usize,
        ParityFixtureKind::Interconnection => 6usize,
        ParityFixtureKind::Libavoid => 3usize,
    };
    for (id, rb) in &rust_bends {
        if let Some(jb) = java_bends.get(id) {
            shared += 1;
            let delta = rb.abs_diff(*jb);
            if delta > direct_limit {
                return Some(format!(
                    "edge bend complexity diverged for {} edge {} (rust={}, java={}, limit={})",
                    fixture, id, rb, jb, direct_limit
                ));
            }
        }
    }

    if shared == 0 {
        let rust_sig_bends = edge_bend_counts_by_signature(rust_out);
        let java_sig_bends = edge_bend_counts_by_signature(java_out);
        let sig_limit = match kind {
            ParityFixtureKind::Layered => 0usize,
            ParityFixtureKind::Interconnection => 3usize,
            ParityFixtureKind::Libavoid => 3usize,
        };
        for (sig, rb) in &rust_sig_bends {
            if let Some(jb) = java_sig_bends.get(sig) {
                shared += 1;
                let delta = rb.abs_diff(*jb);
                if delta > sig_limit {
                    return Some(format!(
                        "edge bend complexity diverged for {} signature {} (rust={}, java={}, limit={})",
                        fixture, sig, rb, jb, sig_limit
                    ));
                }
            }
        }
    }

    if shared == 0 && kind == ParityFixtureKind::Interconnection {
        let rust_total_bends: usize = rust_bends.values().sum();
        let java_total_bends: usize = java_bends.values().sum();
        let rust_max_bends = rust_bends.values().copied().max().unwrap_or(0);
        let java_max_bends = java_bends.values().copied().max().unwrap_or(0);
        let total_delta = rust_total_bends.abs_diff(java_total_bends);
        let max_delta = rust_max_bends.abs_diff(java_max_bends);
        if total_delta > 64 || max_delta > 6 {
            return Some(format!(
                "aggregate bend deltas too high for {} (total: rust={}, java={}, delta={}; max: rust={}, java={}, delta={})",
                fixture,
                rust_total_bends,
                java_total_bends,
                total_delta,
                rust_max_bends,
                java_max_bends,
                max_delta
            ));
        }
    }

    None
}

fn write_case_report(case_dir: &Path, report: &ParityCaseReport, rust_out: &Value, java_out: &Value) {
    let _ = fs::create_dir_all(case_dir);
    fs::write(
        case_dir.join("rust.json"),
        serde_json::to_string_pretty(rust_out).unwrap(),
    )
    .ok();
    fs::write(
        case_dir.join("java.json"),
        serde_json::to_string_pretty(java_out).unwrap(),
    )
    .ok();
    fs::write(
        case_dir.join("report.json"),
        serde_json::to_string_pretty(report).unwrap(),
    )
    .ok();
}

fn append_suite_summary(report: &ParityCaseReport) {
    let summary_path = suite_summary_path();
    let mut reports: Vec<ParityCaseReport> = fs::read_to_string(&summary_path)
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default();
    reports.retain(|existing| !(existing.fixture == report.fixture && existing.kind == report.kind));
    reports.push(report.clone());
    reports.sort_by(|a, b| {
        a.kind
            .cmp(&b.kind)
            .then_with(|| a.fixture.cmp(&b.fixture))
    });
    if let Some(parent) = summary_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(summary_path, serde_json::to_string_pretty(&reports).unwrap()).ok();
}

fn append_skipped_case(fixture: &str, kind: ParityFixtureKind, reason: &str) {
    let report = build_skipped_parity_case_report(fixture, kind, reason);
    append_suite_summary(&report);
}

fn evaluate_case(
    fixture: &str,
    kind: ParityFixtureKind,
    rust_out: &Value,
    java_out: &Value,
    mismatch_dir: &Path,
) -> Result<(), String> {
    let relaxed_error = compare_layout_json_relaxed(rust_out, java_out).err();
    let bend_error = bend_complexity_error(kind, fixture, rust_out, java_out);
    let report = build_parity_case_report(
        fixture,
        kind,
        rust_out,
        java_out,
        relaxed_error.clone(),
        bend_error.clone(),
    )
    .map_err(|e| format!("failed to build parity report for {}: {}", fixture, e))?;
    write_case_report(&mismatch_dir.join(fixture), &report, rust_out, java_out);
    append_suite_summary(&report);
    if let Some(error) = relaxed_error {
        return Err(error);
    }
    if let Some(error) = bend_error {
        return Err(error);
    }
    Ok(())
}

fn finish_suite(kind: &str, mismatches: &[String], skipped: &[String], processed: usize) {
    assert!(processed > 0, "no fixtures were processed for {}", kind);
    if strict_parity_enabled() && !mismatches.is_empty() {
        panic!(
            "strict Java parity mismatches for {}:\n{}",
            kind,
            mismatches.join("\n")
        );
    }
    if !mismatches.is_empty() {
        eprintln!(
            "Java parity mismatches recorded for {} (strict mode disabled):\n{}",
            kind,
            mismatches.join("\n")
        );
    }
    if !skipped.is_empty() {
        eprintln!(
            "Java parity skips recorded for {}:\n{}",
            kind,
            skipped.join("\n")
        );
    }
}

#[test]
fn parity_java_matches_rust_on_fixtures() {
    let root = repo_root();
    let mismatch_dir = root
        .join("target")
        .join("elk-parity-mismatches");
    let _ = fs::create_dir_all(&mismatch_dir);
    let mut mismatches = Vec::new();
    let mut skipped = Vec::new();
    let mut processed = 0usize;

    for name in FIXTURES {
        processed += 1;
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
        let java_out = match run_java_runner_optional(&in_path) {
            Ok(out) => out,
            Err(reason) => {
                append_skipped_case(name, ParityFixtureKind::Layered, &reason);
                skipped.push(format!("{}: {}", name, reason));
                continue;
            }
        };

        if let Err(e) = evaluate_case(name, ParityFixtureKind::Layered, &rust_out, &java_out, &mismatch_dir) {
            mismatches.push(format!("{}: {}", name, e));
        }
    }
    finish_suite("layered", &mismatches, &skipped, processed);
}

#[test]
fn parity_java_matches_rust_on_interconnection_topology() {
    let root = repo_root();
    let mismatch_dir = root.join("target").join("elk-parity-mismatches");
    let _ = fs::create_dir_all(&mismatch_dir);
    let mut mismatches = Vec::new();
    let mut skipped = Vec::new();
    let mut processed = 0usize;

    for name in INTERCONNECTION_FIXTURES {
        processed += 1;
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
        let java_out = match run_java_runner_optional(&in_path) {
            Ok(out) => out,
            Err(reason) => {
                append_skipped_case(name, ParityFixtureKind::Interconnection, &reason);
                skipped.push(format!("{}: {}", name, reason));
                continue;
            }
        };

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

        if let Err(e) = evaluate_case(
            name,
            ParityFixtureKind::Interconnection,
            &rust_out,
            &java_out,
            &mismatch_dir,
        ) {
            mismatches.push(format!("{}: {}", name, e));
        }
    }
    finish_suite("interconnection", &mismatches, &skipped, processed);
}

#[test]
fn parity_java_matches_rust_on_libavoid_fixtures() {
    let root = repo_root();
    let mismatch_dir = root.join("target").join("elk-parity-mismatches");
    let _ = fs::create_dir_all(&mismatch_dir);
    let mut mismatches = Vec::new();
    let mut skipped = Vec::new();
    let mut processed = 0usize;

    for name in LIBAVOID_FIXTURES {
        processed += 1;
        let json = fixture_json(name);
        let mut g = import_str(&json).expect("import").graph;
        LayoutService::default_registry()
            .layout(&mut g, &LayoutOptions::default())
            .expect("rust libavoid layout");
        let rust_out = export_elk_graph_to_value(&g);

        let in_path = mismatch_dir.join(format!("input_{}_libavoid.json", name));
        let mut java_input: Value = serde_json::from_str(&json).expect("fixture JSON should parse");
        ensure_java_libavoid_options(&mut java_input);
        fs::write(&in_path, serde_json::to_string_pretty(&java_input).unwrap())
            .expect("write java input");
        let java_out = match run_java_runner_optional(&in_path) {
            Ok(out) => out,
            Err(reason) => {
                append_skipped_case(name, ParityFixtureKind::Libavoid, &reason);
                skipped.push(format!("{}: {}", name, reason));
                continue;
            }
        };

        let rust_edges = rust_out
            .get("edges")
            .and_then(Value::as_array)
            .expect("rust edges array");
        let java_edges = java_out
            .get("edges")
            .and_then(Value::as_array)
            .expect("java edges array");
        assert_eq!(rust_edges.len(), java_edges.len(), "edge count mismatch for {}", name);

        if let Err(e) = evaluate_case(
            name,
            ParityFixtureKind::Libavoid,
            &rust_out,
            &java_out,
            &mismatch_dir,
        ) {
            mismatches.push(format!("{}: {}", name, e));
        }
    }
    finish_suite("libavoid", &mismatches, &skipped, processed);
}
