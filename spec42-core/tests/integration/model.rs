//! sysml/model integration tests.

use super::harness::{next_id, read_message, read_response, send_message, spawn_server};
use std::fs;
use std::path::PathBuf;

const FULL_DRONE_FIXTURE: &str = "surveillance_drone_full.sysml";

fn fixture_text(name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);
    fs::read_to_string(path).expect("read fixture")
}

fn write_rendered_svg(name: &str, svg: &str) {
    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("output");
    fs::create_dir_all(&output_dir).expect("create tests/output");
    let path = output_dir.join(name);
    let svg = force_dark_test_theme(svg);
    fs::write(&path, svg).unwrap_or_else(|err| {
        panic!("write rendered svg to {}: {err}", path.display());
    });
}

fn write_rendered_debug(name: &str, text: &str) {
    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("output");
    fs::create_dir_all(&output_dir).expect("create tests/output");
    let path = output_dir.join(name);
    fs::write(&path, text).unwrap_or_else(|err| {
        panic!("write rendered debug to {}: {err}", path.display());
    });
}

fn extract_edge_connection_paths(svg: &str, limit: usize) -> Vec<String> {
    // Keep this dependency-free (no regex): find edge-connection <path> elements,
    // then extract their `d="..."` attribute.
    let mut out = Vec::new();
    let mut cursor = 0usize;
    while out.len() < limit {
        let Some(hit) = svg[cursor..].find("class=\"diagram-edge edge-connection\"") else {
            break;
        };
        cursor += hit;
        let Some(d_attr) = svg[cursor..].find(" d=\"") else {
            cursor += 10;
            continue;
        };
        let quote = cursor + d_attr + 3; // points at the opening quote of d="
        let start = quote + 1;
        let Some(end_quote) = svg[start..].find('"') else {
            break;
        };
        let end = start + end_quote;
        out.push(svg[start..end].to_string());
        cursor = end;
    }
    out
}

fn extract_edge_paths(svg: &str, limit: usize) -> Vec<String> {
    // Extract `d="..."` for any diagram edge path elements.
    let mut out = Vec::new();
    let mut cursor = 0usize;
    while out.len() < limit {
        let Some(hit) = svg[cursor..].find("class=\"diagram-edge") else {
            break;
        };
        cursor += hit;
        let Some(d_attr) = svg[cursor..].find(" d=\"") else {
            cursor += 10;
            continue;
        };
        let quote = cursor + d_attr + 3;
        let start = quote + 1;
        let Some(end_quote) = svg[start..].find('"') else {
            break;
        };
        let end = start + end_quote;
        out.push(svg[start..end].to_string());
        cursor = end;
    }
    out
}

#[derive(Clone, Debug)]
struct SvgPort {
    id: String,
    x: f32,
    y: f32,
}

fn extract_svg_ports(svg: &str, limit: usize) -> Vec<SvgPort> {
    // Dependency-free parsing: find `<circle class="diagram-port" ... cx=".." cy=".." ... data-port-id="..."/>`
    let mut out = Vec::new();
    let mut cursor = 0usize;
    while out.len() < limit {
        let Some(hit) = svg[cursor..].find("class=\"diagram-port\"") else {
            break;
        };
        cursor += hit;
        // Scan a small window ahead to capture the whole element.
        let tail = &svg[cursor..svg.len().min(cursor + 400)];
        let Some(cx_idx) = tail.find("cx=\"") else {
            cursor += 16;
            continue;
        };
        let cx_start = cx_idx + 4;
        let Some(cx_end) = tail[cx_start..].find('"') else {
            break;
        };
        let cx_str = &tail[cx_start..cx_start + cx_end];
        let Some(cy_idx) = tail.find("cy=\"") else {
            cursor += 16;
            continue;
        };
        let cy_start = cy_idx + 4;
        let Some(cy_end) = tail[cy_start..].find('"') else {
            break;
        };
        let cy_str = &tail[cy_start..cy_start + cy_end];
        let Some(id_idx) = tail.find("data-port-id=\"") else {
            cursor += 16;
            continue;
        };
        let id_start = id_idx + "data-port-id=\"".len();
        let Some(id_end) = tail[id_start..].find('"') else {
            break;
        };
        let id = tail[id_start..id_start + id_end].to_string();
        let Ok(x) = cx_str.parse::<f32>() else {
            cursor += 16;
            continue;
        };
        let Ok(y) = cy_str.parse::<f32>() else {
            cursor += 16;
            continue;
        };
        out.push(SvgPort { id, x, y });
        cursor += 16;
    }
    out
}

fn parse_path_endpoints(d: &str) -> Option<((f32, f32), (f32, f32))> {
    // Supports strings like: `M x y L x y ...`
    let mut nums: Vec<f32> = Vec::new();
    let mut current = String::new();
    for ch in d.chars() {
        if ch.is_ascii_digit() || ch == '.' || ch == '-' {
            current.push(ch);
        } else if !current.is_empty() {
            if let Ok(v) = current.parse::<f32>() {
                nums.push(v);
            }
            current.clear();
        }
    }
    if !current.is_empty() {
        if let Ok(v) = current.parse::<f32>() {
            nums.push(v);
        }
    }
    if nums.len() < 4 {
        return None;
    }
    let start = (nums[0], nums[1]);
    let end = (nums[nums.len() - 2], nums[nums.len() - 1]);
    Some((start, end))
}

fn nearest_port<'a>(ports: &'a [SvgPort], x: f32, y: f32) -> Option<(&'a SvgPort, f32)> {
    ports
        .iter()
        .map(|p| {
            let dx = p.x - x;
            let dy = p.y - y;
            (p, (dx * dx + dy * dy).sqrt())
        })
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
}

fn force_dark_test_theme(svg: &str) -> String {
    let injection = "<style>.diagram-root{--diagram-paper:#1e1e1e;--diagram-ink:#d4d4d4;--diagram-muted:#a0a0a0;--diagram-faint:#6b6b6b;}</style>";
    svg.replacen('>', &format!(">{injection}"), 1)
}

/// sysml/model with scope ["graph"] returns nodes and edges after didOpen.
/// Validates that the semantic graph is built and serialized correctly.
#[test]
fn lsp_sysml_model_graph() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///model_test.sysml";
    let content = "package P { part def X; part a : X; }";

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": content }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(80));

    let model_id = next_id();
    let model_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": model_id,
        "method": "sysml/model",
        "params": {
            "textDocument": { "uri": uri },
            "scope": ["graph", "stats"]
        }
    });
    send_message(&mut stdin, &model_req.to_string());
    let model_resp = read_response(&mut stdout, model_id).expect("sysml/model response");
    let model_json: serde_json::Value =
        serde_json::from_str(&model_resp).expect("parse sysml/model response");
    assert_eq!(model_json["id"], model_id);
    let result = &model_json["result"];
    let graph = result
        .get("graph")
        .expect("sysml/model with scope graph should return graph");
    let rendered_diagrams = result
        .get("renderedDiagrams")
        .expect("sysml/model should return renderedDiagrams");
    let nodes = graph["nodes"]
        .as_array()
        .expect("graph should have nodes array");
    let edges = graph["edges"]
        .as_array()
        .expect("graph should have edges array");
    assert!(
        rendered_diagrams.get("generalView").is_some(),
        "generalView render should be present"
    );

    assert!(
        !nodes.is_empty(),
        "graph.nodes should not be empty for package P with part def X and part a"
    );
    assert!(
        nodes.len() >= 2,
        "expect at least 2 nodes (package P, part def X, part a): got {}",
        nodes.len()
    );

    let node_ids: Vec<String> = nodes
        .iter()
        .filter_map(|n| n["id"].as_str().map(String::from))
        .collect();
    assert!(
        node_ids.iter().any(|id| id.contains("P")),
        "nodes should include package P: {:?}",
        node_ids
    );

    let contains_edges: usize = edges
        .iter()
        .filter(|e| e["type"].as_str() == Some("contains"))
        .count();
    assert!(
        contains_edges >= 1,
        "graph should have contains edges for hierarchy"
    );

    let typing_edges: Vec<_> = edges
        .iter()
        .filter(|e| e["type"].as_str() == Some("typing"))
        .collect();
    assert!(
        !typing_edges.is_empty(),
        "graph should have typing edges from part a to part def X: {:?}",
        edges
    );

    let _ = child.kill();
}

/// sysml/model with scope ["graph"] returns state machine nodes and transition edges.
/// Validates semantic graph for state-transition-view: state def container, state usages (type "state"),
/// contains edges, and transition edges.
#[test]
#[ignore] // sysml-parser does not expose state def / transition; graph has no state nodes yet
fn lsp_sysml_model_state_transition_view() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///state_test.sysml";
    let content = r#"
        package P {
            state def A;
            state def B;
            state def M {
                state a : A;
                state b : B;
                transition t first a then b;
            }
        }
    "#;

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": content }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(80));

    let model_id = next_id();
    let model_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": model_id,
        "method": "sysml/model",
        "params": {
            "textDocument": { "uri": uri },
            "scope": ["graph"]
        }
    });
    send_message(&mut stdin, &model_req.to_string());
    let model_resp = read_response(&mut stdout, model_id).expect("sysml/model response");
    let model_json: serde_json::Value =
        serde_json::from_str(&model_resp).expect("parse sysml/model response");
    assert_eq!(model_json["id"], model_id);
    let result = &model_json["result"];
    let graph = result
        .get("graph")
        .expect("sysml/model with scope graph should return graph");
    let nodes = graph["nodes"]
        .as_array()
        .expect("graph should have nodes array");
    let edges = graph["edges"]
        .as_array()
        .expect("graph should have edges array");

    // State machine container M (state def) and state usages a, b (type "state")
    let state_def_nodes: Vec<_> = nodes
        .iter()
        .filter(|n| n["type"].as_str() == Some("state def"))
        .collect();
    let state_usage_nodes: Vec<_> = nodes
        .iter()
        .filter(|n| n["type"].as_str() == Some("state"))
        .collect();

    assert!(
        state_def_nodes
            .iter()
            .any(|n| n["name"].as_str() == Some("M")),
        "graph should have state def M (state machine container), nodes: {:?}",
        nodes
            .iter()
            .map(|n| (n["name"].as_str(), n["type"].as_str()))
            .collect::<Vec<_>>()
    );
    assert!(
        state_usage_nodes.len() >= 2,
        "graph should have state usages a and b (type 'state'), got: {:?}",
        state_usage_nodes
            .iter()
            .map(|n| n["name"].as_str())
            .collect::<Vec<_>>()
    );

    // Contains edges: M -> a, M -> b
    let contains_edges: Vec<_> = edges
        .iter()
        .filter(|e| e["type"].as_str() == Some("contains"))
        .collect();
    let contains_targets: Vec<&str> = contains_edges
        .iter()
        .filter_map(|e| e["target"].as_str())
        .collect();
    assert!(
        contains_targets.iter().any(|t| t.ends_with("::a")),
        "contains edges should link M to state a, got: {:?}",
        contains_targets
    );
    assert!(
        contains_targets.iter().any(|t| t.ends_with("::b")),
        "contains edges should link M to state b, got: {:?}",
        contains_targets
    );

    // Transition edges: a -> b
    let transition_edges: Vec<_> = edges
        .iter()
        .filter(|e| e["type"].as_str() == Some("transition"))
        .collect();
    assert!(
        !transition_edges.is_empty(),
        "graph should have transition edges, got: {:?}",
        edges
            .iter()
            .map(|e| (
                e["type"].as_str(),
                e["source"].as_str(),
                e["target"].as_str()
            ))
            .collect::<Vec<_>>()
    );

    let _ = child.kill();
}

#[test]
fn lsp_sysml_model_graph_includes_requirement_usecase_and_state_nodes() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///rich_model_test.sysml";
    let content = r#"
        package P {
            requirement def EnduranceReq;
            use case def PatrolMission {
                actor operator : HumanOperator;
            }
            state def DroneMode {
                state idle;
                state active;
                transition activate first idle then active;
            }
        }
    "#;

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": content }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(120));

    let model_id = next_id();
    let model_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": model_id,
        "method": "sysml/model",
        "params": {
            "textDocument": { "uri": uri },
            "scope": ["graph"]
        }
    });
    send_message(&mut stdin, &model_req.to_string());
    let model_resp = read_response(&mut stdout, model_id).expect("sysml/model response");
    let model_json: serde_json::Value =
        serde_json::from_str(&model_resp).expect("parse sysml/model response");
    let graph = &model_json["result"]["graph"];
    let rendered_general = &model_json["result"]["renderedDiagrams"]["generalView"]["svg"];
    let nodes = graph["nodes"]
        .as_array()
        .expect("graph should have nodes array");
    let edges = graph["edges"]
        .as_array()
        .expect("graph should have edges array");

    let has_requirement = nodes.iter().any(|n| {
        n["type"].as_str() == Some("requirement def") && n["name"].as_str() == Some("EnduranceReq")
    });
    assert!(
        has_requirement,
        "graph should include requirement def EnduranceReq"
    );

    let has_use_case = nodes.iter().any(|n| {
        n["type"].as_str() == Some("use case def") && n["name"].as_str() == Some("PatrolMission")
    });
    assert!(
        has_use_case,
        "graph should include use case def PatrolMission"
    );

    let has_actor = nodes
        .iter()
        .any(|n| n["type"].as_str() == Some("actor") && n["name"].as_str() == Some("operator"));
    assert!(has_actor, "graph should include actor usage operator");

    let has_state_def = nodes.iter().any(|n| {
        n["type"].as_str() == Some("state def") && n["name"].as_str() == Some("DroneMode")
    });
    assert!(has_state_def, "graph should include state def DroneMode");

    let state_names: Vec<_> = nodes
        .iter()
        .filter(|n| n["type"].as_str() == Some("state"))
        .filter_map(|n| n["name"].as_str())
        .collect();
    assert!(
        state_names.contains(&"idle") && state_names.contains(&"active"),
        "graph should include state usages idle and active, got {:?}",
        state_names
    );

    let has_transition = edges.iter().any(|e| {
        e["type"].as_str() == Some("transition")
            && e["source"].as_str().is_some_and(|s| s.ends_with("::idle"))
            && e["target"]
                .as_str()
                .is_some_and(|t| t.ends_with("::active"))
    });
    assert!(
        has_transition,
        "graph should include transition edge idle -> active"
    );
    let general_svg = rendered_general.as_str().unwrap_or_default();
    assert!(
        general_svg.contains("EnduranceReq"),
        "general view svg should include requirement def EnduranceReq"
    );
    assert!(
        general_svg.contains("PatrolMission"),
        "general view svg should include use case def PatrolMission"
    );
    assert!(
        general_svg.contains("operator"),
        "general view svg should include actor usage operator"
    );
    assert!(
        general_svg.contains("DroneMode"),
        "general view svg should include state def DroneMode"
    );
    assert!(
        general_svg.contains("idle") && general_svg.contains("active"),
        "general view svg should include state usages idle and active"
    );

    let _ = child.kill();
}

#[test]
fn lsp_sysml_model_includes_rendered_interconnection_diagram() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///surveillance_drone_full_render_test.sysml";
    let content = fixture_text(FULL_DRONE_FIXTURE);

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": content }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(120));

    let model_id = next_id();
    let model_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": model_id,
        "method": "sysml/model",
        "params": {
            "textDocument": { "uri": uri },
            "scope": ["graph"]
        }
    });
    send_message(&mut stdin, &model_req.to_string());
    let model_resp = read_response(&mut stdout, model_id).expect("sysml/model response");
    let model_json: serde_json::Value =
        serde_json::from_str(&model_resp).expect("parse sysml/model response");
    let result = &model_json["result"];
    let rendered = result["renderedDiagrams"]["interconnectionView"].clone();
    let ibd_parts = result["ibd"]["parts"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    assert!(
        !rendered.is_null(),
        "renderedDiagrams.interconnectionView should be present: {result:#}"
    );
    let svg = rendered["svg"].as_str().unwrap_or_default();
    let metrics = &rendered["metrics"];
    let aspect_ratio = metrics["aspectRatio"].as_f64().unwrap_or(0.0);
    let crossings = metrics["edgeCrossingCount"].as_u64().unwrap_or(u64::MAX);
    let intrusions = metrics["edgeNodeIntrusionCount"].as_u64().unwrap_or(u64::MAX);
    let bends = metrics["bendCount"].as_u64().unwrap_or(u64::MAX);
    let orthogonal_violations = metrics["orthogonalViolationCount"].as_u64().unwrap_or(u64::MAX);
    let warnings = rendered["warnings"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect::<Vec<_>>();
    write_rendered_svg("interconnection-view-full-drone.svg", svg);
    let mut debug = String::new();
    debug.push_str("interconnection-view-full-drone debug\n");
    debug.push_str(&format!("aspect_ratio={aspect_ratio}\n"));
    debug.push_str(&format!("edge_crossings={crossings}\n"));
    debug.push_str(&format!("edge_node_intrusions={intrusions}\n"));
    debug.push_str(&format!("bend_count={bends}\n"));
    debug.push_str(&format!("orthogonal_violations={orthogonal_violations}\n"));
    debug.push_str(&format!("warnings_count={}\n", warnings.len()));
    let router_warnings = warnings
        .iter()
        .filter(|warning| {
            warning.contains("router active")
                || warning.contains("routing failed")
                || warning.contains("libavoid")
        })
        .count();
    debug.push_str(&format!("router_warning_count={router_warnings}\n"));
    if !warnings.is_empty() {
        debug.push_str("\nwarnings:\n");
        for (idx, warning) in warnings.iter().enumerate() {
            debug.push_str(&format!("{idx}: {warning}\n"));
        }
    }
    debug.push_str("\nedge_connection_paths:\n");
    let ports = extract_svg_ports(svg, 300);
    let paths = extract_edge_connection_paths(svg, 60);
    for (idx, path) in paths.iter().cloned().enumerate() {
        debug.push_str(&format!("{idx}: {path}\n"));
    }
    debug.push_str("\nibd_parts_sample:\n");
    for part in ibd_parts.iter().take(40) {
        let qn = part["qualifiedName"].as_str().unwrap_or("<missing-qn>");
        let container = part["containerId"].as_str().unwrap_or("<none>");
        debug.push_str(&format!("{qn} | container={container}\n"));
    }
    debug.push_str("\nedge_endpoint_nearest_ports:\n");
    let mut max_start_drift = 0.0f32;
    let mut max_end_drift = 0.0f32;
    let mut parsed_endpoint_paths = 0usize;
    for (idx, path) in paths.into_iter().enumerate() {
        if let Some(((sx, sy), (ex, ey))) = parse_path_endpoints(&path) {
            let start_near = nearest_port(&ports, sx, sy);
            let end_near = nearest_port(&ports, ex, ey);
            let start_dist = start_near.map(|(_, d)| d).unwrap_or(f32::INFINITY);
            let end_dist = end_near.map(|(_, d)| d).unwrap_or(f32::INFINITY);
            max_start_drift = max_start_drift.max(start_dist);
            max_end_drift = max_end_drift.max(end_dist);
            parsed_endpoint_paths += 1;
            debug.push_str(&format!(
                "{idx}: start=({sx:.1},{sy:.1}) nearest_start_port={} dist={:.2} end=({ex:.1},{ey:.1}) nearest_end_port={} dist={:.2}\n",
                start_near.map(|(p, _)| p.id.as_str()).unwrap_or("<none>"),
                start_dist,
                end_near.map(|(p, _)| p.id.as_str()).unwrap_or("<none>"),
                end_dist,
            ));
        } else {
            debug.push_str(&format!("{idx}: (failed to parse endpoints) {path}\n"));
        }
    }
    write_rendered_debug("interconnection-view-full-drone.debug.txt", &debug);
    assert!(
        svg.contains("diagram-root interconnection-view"),
        "expected backend interconnection svg, got: {}",
        &svg[..svg.len().min(200)]
    );
    assert!(
        svg.contains("SurveillanceQuadrotorDrone"),
        "expected interconnection view root to be present"
    );
    assert!(
        svg.contains("flightController")
            && svg.contains("cameraPayload")
            && svg.contains("battery"),
        "expected interconnection view to include key drone parts"
    );
    assert!(
        svg.contains("diagram-port-label") && svg.contains("motorCmd") && svg.contains("videoOut"),
        "expected interconnection view to include readable port labels"
    );
    assert!(
        svg.matches("class=\"diagram-port\"").count() >= 12,
        "expected interconnection view to include multiple ports"
    );
    assert!(
        svg.matches("edge-connection").count() >= 8,
        "expected interconnection view to include multiple routed connections"
    );
    assert!(
        orthogonal_violations == 0,
        "expected orthogonal routing (no violations), got {orthogonal_violations} (aspect_ratio={aspect_ratio})"
    );
    assert!(
        intrusions <= 60,
        "expected edge-node intrusions to stay below quality gate, got {intrusions} (aspect_ratio={aspect_ratio}, crossings={crossings}, bends={bends})"
    );
    assert!(
        crossings <= 14,
        "expected edge crossings to stay below quality gate, got {crossings} (aspect_ratio={aspect_ratio}, intrusions={intrusions}, bends={bends})"
    );
    assert!(
        bends <= 100,
        "expected bend count to stay bounded, got {bends} (aspect_ratio={aspect_ratio}, intrusions={intrusions}, crossings={crossings})"
    );
    assert!(
        warnings.iter().any(|warning| {
            warning.contains("router active")
                || warning.contains("routing failed")
                || warning.contains("libavoid")
        }),
        "expected interconnection warnings to include routing backend diagnostics, got: {warnings:?}"
    );
    assert!(
        !warnings
            .iter()
            .any(|warning| warning.contains("endpoint out-of-bounds")),
        "expected no libavoid endpoint out-of-bounds warnings, got: {warnings:?}"
    );
    assert!(
        !warnings
            .iter()
            .any(|warning| warning.contains("canonicalization_skipped_large_delta")),
        "expected no large-delta canonicalization skip warnings, got: {warnings:?}"
    );
    assert!(
        !warnings
            .iter()
            .any(|warning| warning.contains("libavoid terminal canonicalization adjusted edge")),
        "expected no mixed-frame terminal canonicalization adjustments, got: {warnings:?}"
    );
    assert!(
        parsed_endpoint_paths >= 8,
        "expected enough parsed edge paths for drift checks, got {parsed_endpoint_paths}"
    );
    assert!(
        max_start_drift <= 1200.0 && max_end_drift <= 1200.0,
        "expected edge endpoints to stay within a bounded distance of declared ports (<=1200px drift), got start_max={max_start_drift:.2}, end_max={max_end_drift:.2}"
    );
    assert!(
        aspect_ratio > 0.0 && aspect_ratio < 9.5,
        "expected interconnection view to stay within a sanity aspect ratio bound (<9.5) to prevent runaway canvas growth, got {aspect_ratio}"
    );

    let _ = child.kill();
}

#[test]
fn lsp_sysml_model_includes_rendered_general_diagram_for_full_drone_fixture() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///surveillance_drone_full_general_test.sysml";
    let content = fixture_text(FULL_DRONE_FIXTURE);

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": content }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(180));

    let model_id = next_id();
    let model_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": model_id,
        "method": "sysml/model",
        "params": {
            "textDocument": { "uri": uri },
            "scope": ["graph"]
        }
    });
    send_message(&mut stdin, &model_req.to_string());
    let model_resp = read_response(&mut stdout, model_id).expect("sysml/model response");
    let model_json: serde_json::Value =
        serde_json::from_str(&model_resp).expect("parse sysml/model response");
    let result = &model_json["result"];
    let graph = result["graph"].clone();
    let rendered = result["renderedDiagrams"]["generalView"].clone();
    assert!(
        !rendered.is_null(),
        "renderedDiagrams.generalView should be present for full drone fixture: {result:#}"
    );
    let edges = graph["edges"].as_array().expect("graph edges array");
    assert!(
        edges.iter().any(|edge| {
            edge["type"].as_str() == Some("satisfy")
                && edge["source"]
                    .as_str()
                    .is_some_and(|source| source.ends_with("EnduranceReq"))
                && edge["target"]
                    .as_str()
                    .is_some_and(|target| target.ends_with("droneInstance"))
        }),
        "expected graph to include satisfy edge EnduranceReq -> droneInstance, edges: {edges:#?}"
    );
    assert!(
        edges.iter().any(|edge| {
            edge["type"].as_str() == Some("perform")
                && edge["source"]
                    .as_str()
                    .is_some_and(|source| source.ends_with("SurveillanceQuadrotorDroneWithBehavior"))
                && edge["target"]
                    .as_str()
                    .is_some_and(|target| target.ends_with("executePatrol"))
        }),
        "expected graph to include perform edge from SurveillanceQuadrotorDroneWithBehavior to executePatrol"
    );
    assert!(
        edges.iter().any(|edge| {
            edge["type"].as_str() == Some("allocate")
                && edge["source"]
                    .as_str()
                    .is_some_and(|source| source.ends_with("executePatrol"))
                && edge["target"]
                    .as_str()
                    .is_some_and(|target| target.ends_with("flightControl"))
        }),
        "expected graph to include allocate edge executePatrol -> flightControl"
    );
    let svg = rendered["svg"].as_str().unwrap_or_default();
    write_rendered_svg("general-view-full-drone.svg", svg);
    let mut debug = String::new();
    debug.push_str("general-view-full-drone debug\n");
    debug.push_str(&format!(
        "edge_paths_count_sampled={}\n",
        extract_edge_paths(svg, 40).len()
    ));
    debug.push_str("\nedge_paths:\n");
    for (idx, path) in extract_edge_paths(svg, 40).into_iter().enumerate() {
        debug.push_str(&format!("{idx}: {path}\n"));
    }
    write_rendered_debug("general-view-full-drone.debug.txt", &debug);
    assert!(
        svg.contains("diagram-root general-view"),
        "expected backend general svg, got: {}",
        &svg[..svg.len().min(200)]
    );
    assert!(
        svg.contains("SurveillanceQuadrotorDrone"),
        "expected full drone general view to include SurveillanceQuadrotorDrone"
    );
    assert!(
        svg.contains("droneInstance"),
        "expected full drone general view to include droneInstance"
    );
    assert!(
        svg.contains("SurveillanceQuadrotorDroneWithBehavior"),
        "expected full drone general view to include SurveillanceQuadrotorDroneWithBehavior"
    );
    assert!(
        !svg.contains("executePatrol"),
        "expected structural general view to exclude action nodes like executePatrol"
    );
    assert!(
        !svg.contains("requirement def") && !svg.contains("use case") && !svg.contains("state def"),
        "expected structural general view to exclude non-structural node kinds"
    );
    assert!(
        svg.matches("class=\"diagram-node part").count() >= 12,
        "expected structural general view to include multiple part and part def nodes"
    );
    assert!(
        svg.matches("edge-contains").count() >= 8,
        "expected structural general view to include containment edges"
    );
    assert!(
        svg.matches("edge-typing").count() >= 8,
        "expected structural general view to include typing edges"
    );

    let _ = child.kill();
}

#[test]
fn lsp_sysml_model_includes_rendered_interconnection_diagram_for_connected_blocks_fixture() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///connected_blocks_fixture_test.sysml";
    let content = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("vscode")
            .join("testFixture")
            .join("workspaces")
            .join("interconnection")
            .join("ConnectedBlocks.sysml"),
    )
    .expect("read connected blocks fixture");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": content }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(120));

    let model_id = next_id();
    let model_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": model_id,
        "method": "sysml/model",
        "params": {
            "textDocument": { "uri": uri },
            "scope": ["graph"]
        }
    });
    send_message(&mut stdin, &model_req.to_string());
    let model_resp = read_response(&mut stdout, model_id).expect("sysml/model response");
    let model_json: serde_json::Value =
        serde_json::from_str(&model_resp).expect("parse sysml/model response");
    let result = &model_json["result"];
    let rendered = result["renderedDiagrams"]["interconnectionView"].clone();
    assert!(
        !rendered.is_null(),
        "renderedDiagrams.interconnectionView should be present for ConnectedBlocks: {result:#}"
    );

    let _ = child.kill();
}

#[test]
fn lsp_sysml_model_ibd_includes_connectors_for_part_def_connect_statements() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///ibd_connectors_test.sysml";
    let content = r#"
        package P {
            port def SignalPort;

            part def Controller {
                port commandOut : SignalPort;
            }

            part def Sensor {
                port readingOut : SignalPort;
            }

            part def Processor {
                port commandIn : SignalPort;
                port readingIn : SignalPort;
            }

            part def System {
                part controller : Controller;
                part sensor : Sensor;
                part processor : Processor;

                connect controller.commandOut to processor.commandIn;
                connect sensor.readingOut to processor.readingIn;
            }
        }
    "#;

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": content }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(120));

    let model_id = next_id();
    let model_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": model_id,
        "method": "sysml/model",
        "params": {
            "textDocument": { "uri": uri },
            "scope": ["graph"]
        }
    });
    send_message(&mut stdin, &model_req.to_string());
    let model_resp = read_response(&mut stdout, model_id).expect("sysml/model response");
    let model_json: serde_json::Value =
        serde_json::from_str(&model_resp).expect("parse sysml/model response");

    let ibd = &model_json["result"]["ibd"];
    let connectors = ibd["connectors"].as_array().expect("ibd connectors array");
    let parts = ibd["parts"].as_array().expect("ibd parts array");
    let ports = ibd["ports"].as_array().expect("ibd ports array");

    assert!(
        connectors.len() >= 2,
        "expected IBD connectors for part-def connect statements, got {:?}",
        connectors
    );
    assert!(
        connectors.iter().any(
            |c| c["sourceId"].as_str() == Some("P.System.controller.commandOut")
                && c["targetId"].as_str() == Some("P.System.processor.commandIn")
        ),
        "expected controller -> processor connector, got {:?}",
        connectors
    );
    assert!(
        connectors.iter().any(
            |c| c["sourceId"].as_str() == Some("P.System.sensor.readingOut")
                && c["targetId"].as_str() == Some("P.System.processor.readingIn")
        ),
        "expected sensor -> processor connector, got {:?}",
        connectors
    );

    assert!(
        parts
            .iter()
            .any(|p| p["qualifiedName"].as_str() == Some("P.System.controller")),
        "expected expanded IBD part for controller, got {:?}",
        parts
    );
    assert!(
        ports
            .iter()
            .any(|p| p["parentId"].as_str() == Some("P.System.processor")
                && p["name"].as_str() == Some("commandIn")),
        "expected expanded IBD port for processor.commandIn, got {:?}",
        ports
    );

    let _ = child.kill();
}

#[test]
fn lsp_sysml_model_ibd_surveillance_drone_is_complete_enough_for_interconnection_view() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///surveillance_drone_full.sysml";
    let content = fixture_text(FULL_DRONE_FIXTURE);

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": content }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(180));

    let model_id = next_id();
    let model_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": model_id,
        "method": "sysml/model",
        "params": {
            "textDocument": { "uri": uri },
            "scope": ["graph"]
        }
    });
    send_message(&mut stdin, &model_req.to_string());
    let model_resp = read_response(&mut stdout, model_id).expect("sysml/model response");
    let model_json: serde_json::Value =
        serde_json::from_str(&model_resp).expect("parse sysml/model response");

    let ibd = &model_json["result"]["ibd"];
    let connectors = ibd["connectors"].as_array().expect("ibd connectors array");
    let parts = ibd["parts"].as_array().expect("ibd parts array");
    let ports = ibd["ports"].as_array().expect("ibd ports array");
    let default_root = ibd["defaultRoot"].as_str().expect("default root");

    assert_eq!(
        default_root, "SurveillanceQuadrotorDrone",
        "expected drone root to be selected by default"
    );
    assert!(
        connectors.len() >= 17,
        "expected real drone IBD to expose at least the 17 top-level connectors, got {:?}",
        connectors
    );
    assert!(
        connectors.iter().any(|c|
            c["sourceId"].as_str() == Some("SurveillanceDrone.SurveillanceQuadrotorDrone.flightControl.flightController.motorCmd")
                && c["targetId"].as_str() == Some("SurveillanceDrone.SurveillanceQuadrotorDrone.propulsion.propulsionUnit1.cmd")
        ),
        "expected propulsion command connector in IBD, got {:?}",
        connectors
    );
    assert!(
        connectors.iter().any(|c| c["sourceId"].as_str()
            == Some("SurveillanceDrone.SurveillanceQuadrotorDrone.power.distribution.regulated5V")
            && c["targetId"].as_str()
                == Some("SurveillanceDrone.SurveillanceQuadrotorDrone.communication.pwr")),
        "expected regulated power connector in IBD, got {:?}",
        connectors
    );
    assert!(
        connectors.iter().any(|c| c["sourceId"].as_str()
            == Some("SurveillanceDrone.SurveillanceQuadrotorDrone.cameraPayload.videoOut")
            && c["targetId"].as_str()
                == Some("SurveillanceDrone.SurveillanceQuadrotorDrone.communication.videoIn")),
        "expected video link connector in IBD, got {:?}",
        connectors
    );

    assert!(
        parts.iter().any(|p| p["qualifiedName"].as_str()
            == Some("SurveillanceDrone.SurveillanceQuadrotorDrone.propulsion.propulsionUnit4")),
        "expected expanded propulsion unit part in IBD, got {:?}",
        parts
    );
    assert!(
        parts.iter().any(|p| p["qualifiedName"].as_str()
            == Some("SurveillanceDrone.SurveillanceQuadrotorDrone.power.distribution")),
        "expected expanded power distribution part in IBD, got {:?}",
        parts
    );

    let propulsion_ports: Vec<_> = ports
        .iter()
        .filter(|p| {
            p["parentId"]
                .as_str()
                .is_some_and(|id| id.contains(".propulsion.propulsionUnit"))
        })
        .collect();
    assert!(
        propulsion_ports.len() >= 8,
        "expected typed port expansion for all propulsion units, got {:?}",
        propulsion_ports
    );
    assert!(
        ports.iter().any(|p| p["parentId"].as_str()
            == Some("SurveillanceDrone.SurveillanceQuadrotorDrone.flightControl.flightController")
            && p["name"].as_str() == Some("sensorIn")),
        "expected nested flight controller port in IBD, got {:?}",
        ports
    );
    assert!(
        ports.iter().any(|p| p["parentId"].as_str()
            == Some("SurveillanceDrone.SurveillanceQuadrotorDrone.flightControl.flightController")
            && p["name"].as_str() == Some("telemetryOut")
            && p["portSide"].as_str() == Some("right")),
        "expected telemetryOut to resolve to right-side port, got {:?}",
        ports
    );
    assert!(
        ports.iter().any(|p| p["parentId"].as_str()
            == Some("SurveillanceDrone.SurveillanceQuadrotorDrone.communication")
            && p["name"].as_str() == Some("videoIn")
            && p["portSide"].as_str() == Some("left")),
        "expected videoIn to resolve to left-side port, got {:?}",
        ports
    );
    assert!(
        ports.iter().any(|p| p["parentId"].as_str()
            == Some("SurveillanceDrone.SurveillanceQuadrotorDrone.power.distribution")
            && p["name"].as_str() == Some("regulated5V")
            && p["portSide"].as_str() == Some("right")),
        "expected regulated5V to resolve to right-side port, got {:?}",
        ports
    );

    let _ = child.kill();
}

/// sysml/model with scope ["sequenceDiagrams"] returns diagrams with correct action def names.
/// Regression test for action def name parsing (was "(anonymous)" due to Pest silent terminals).
#[test]
#[ignore] // extract_sequence_diagrams returns empty (sysml-parser ActionDef body has no Call/Perform)
fn lsp_sysml_model_sequence_diagrams() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///seq_test.sysml";
    let content = r#"
        package P {
            action def ExecutePatrol { perform action ControlGimbal; }
            action def ControlGimbal { }
        }
    "#;

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": content }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(80));

    let model_id = next_id();
    let model_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": model_id,
        "method": "sysml/model",
        "params": {
            "textDocument": { "uri": uri },
            "scope": ["sequenceDiagrams"]
        }
    });
    send_message(&mut stdin, &model_req.to_string());
    let model_resp = read_response(&mut stdout, model_id).expect("sysml/model response");
    let model_json: serde_json::Value =
        serde_json::from_str(&model_resp).expect("parse sysml/model response");
    let result = &model_json["result"];
    let diagrams = result["sequenceDiagrams"]
        .as_array()
        .expect("sequenceDiagrams array");

    assert_eq!(diagrams.len(), 2, "expected 2 sequence diagrams");
    let names: Vec<&str> = diagrams.iter().filter_map(|d| d["name"].as_str()).collect();
    assert!(
        names.contains(&"ExecutePatrol"),
        "diagrams should include ExecutePatrol, got: {:?}",
        names
    );
    assert!(
        names.contains(&"ControlGimbal"),
        "diagrams should include ControlGimbal, got: {:?}",
        names
    );
    assert!(
        !names
            .iter()
            .any(|n| *n == "(anonymous)" || n.to_lowercase().contains("anonymous")),
        "no diagram should have anonymous name, got: {:?}",
        names
    );

    let _ = child.kill();
}

/// sysml/model with scope ["graph"] returns ibd with defaultRoot = SurveillanceQuadrotorDrone
/// (largest top-level part tree), not Propulsion. Validates IBD backend for interconnection-view.
#[test]
#[ignore] // ibd defaultRoot depends on graph/content that may differ with sysml-parser
fn lsp_sysml_model_ibd_default_root() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///ibd_test.sysml";
    let content = r#"
package SurveillanceDrone {
    port def MotorCommandPort { }
    port def PowerPort { }
    part def PropulsionUnit {
        port cmd : ~MotorCommandPort;
        port pwr : ~PowerPort;
    }
    part def Propulsion {
        part propulsionUnit1 : PropulsionUnit;
        part propulsionUnit2 : PropulsionUnit;
        part propulsionUnit3 : PropulsionUnit;
        part propulsionUnit4 : PropulsionUnit;
    }
    part def FlightController {
        port motorCmd : ~MotorCommandPort;
        port pwr : ~PowerPort;
    }
    part def FlightControlAndSensing {
        part flightController : FlightController;
    }
    part def SurveillanceQuadrotorDrone {
        part propulsion : Propulsion;
        part flightControl : FlightControlAndSensing;
        connect flightControl.flightController.motorCmd to propulsion.propulsionUnit1.cmd;
    }
}
"#;

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": content }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(120));

    let model_id = next_id();
    let model_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": model_id,
        "method": "sysml/model",
        "params": {
            "textDocument": { "uri": uri },
            "scope": ["graph"]
        }
    });
    send_message(&mut stdin, &model_req.to_string());
    let model_resp = read_response(&mut stdout, model_id).expect("sysml/model response");
    let model_json: serde_json::Value =
        serde_json::from_str(&model_resp).expect("parse sysml/model response");
    assert_eq!(model_json["id"], model_id);
    let result = &model_json["result"];
    let ibd = result
        .get("ibd")
        .expect("sysml/model with scope graph should return ibd");
    let default_root = ibd["defaultRoot"]
        .as_str()
        .expect("ibd should have defaultRoot");
    assert_eq!(
        default_root, "SurveillanceQuadrotorDrone",
        "defaultRoot must be SurveillanceQuadrotorDrone (largest tree), got: {}",
        default_root
    );

    let root_candidates = ibd["rootCandidates"]
        .as_array()
        .expect("ibd should have rootCandidates");
    assert!(
        root_candidates
            .iter()
            .any(|c| c.as_str() == Some("SurveillanceQuadrotorDrone")),
        "rootCandidates should include SurveillanceQuadrotorDrone: {:?}",
        root_candidates
    );
    assert!(
        root_candidates
            .iter()
            .any(|c| c.as_str() == Some("Propulsion")),
        "rootCandidates should include Propulsion: {:?}",
        root_candidates
    );

    let parts = ibd["parts"].as_array().expect("ibd should have parts");
    let sqd_parts: Vec<_> = parts
        .iter()
        .filter(|p| {
            let qn = p["qualifiedName"].as_str().unwrap_or("");
            qn == "SurveillanceDrone.SurveillanceQuadrotorDrone"
                || qn.starts_with("SurveillanceDrone.SurveillanceQuadrotorDrone.")
        })
        .collect();

    assert!(
        sqd_parts.len() >= 8,
        "IBD must include complete part tree: root + propulsion + flightControl + 4 propulsionUnit + flightController; got {}: {:?}",
        sqd_parts.len(),
        sqd_parts.iter().map(|p| p["qualifiedName"].as_str()).collect::<Vec<_>>()
    );

    let has_propulsion_units = sqd_parts.iter().any(|p| {
        let qn = p["qualifiedName"].as_str().unwrap_or("");
        qn.contains(".propulsion.propulsionUnit")
    });
    assert!(
        has_propulsion_units,
        "IBD must include nested parts under propulsion (propulsionUnit1..4); got: {:?}",
        sqd_parts
            .iter()
            .map(|p| p["qualifiedName"].as_str())
            .collect::<Vec<_>>()
    );

    let has_flight_controller = sqd_parts.iter().any(|p| {
        let qn = p["qualifiedName"].as_str().unwrap_or("");
        qn.contains(".flightControl.flightController")
    });
    assert!(
        has_flight_controller,
        "IBD must include nested part under flightControl (flightController); got: {:?}",
        sqd_parts
            .iter()
            .map(|p| p["qualifiedName"].as_str())
            .collect::<Vec<_>>()
    );

    let _connectors = ibd["connectors"]
        .as_array()
        .expect("ibd should have connectors array");

    let _ = child.kill();
}
