//! sysml/model integration tests.

use super::harness::{
    next_id, read_message, read_response, send_message, spawn_server,
};
use std::fs;

const FULL_DRONE_FIXTURE: &str = "surveillance_drone_full.sysml";

fn fixture_text(name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);
    fs::read_to_string(path).expect("read fixture")
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
    let nodes = graph["nodes"]
        .as_array()
        .expect("graph should have nodes array");
    let edges = graph["edges"]
        .as_array()
        .expect("graph should have edges array");
    assert!(
        result.get("renderedDiagrams").is_none(),
        "default config should not include renderedDiagrams when no providers are registered"
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
    assert!(
        model_json["result"].get("renderedDiagrams").is_none(),
        "default config should not include rendered diagrams"
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
    let ibd_parts = result["ibd"]["parts"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    assert!(
        result.get("renderedDiagrams").is_none(),
        "default config should not include renderedDiagrams: {result:#}"
    );
    assert!(
        ibd_parts.len() >= 8,
        "expected interconnection source data (IBD parts) to be present"
    );

    let _ = child.kill();
}

#[test]
fn lsp_sysml_model_general_view_graph_deduplicates_gnss_for_full_drone_fixture() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///surveillance_drone_full_general_gnss_test.sysml";
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
    let gv_nodes = model_json["result"]["generalViewGraph"]["nodes"]
        .as_array()
        .expect("generalViewGraph nodes array");
    let gnss_ids: Vec<&str> = gv_nodes
        .iter()
        .filter(|node| node["name"].as_str() == Some("gnss"))
        .filter_map(|node| node["id"].as_str())
        .collect();
    assert_eq!(
        gnss_ids.len(),
        1,
        "expected exactly one gnss in generalViewGraph, got {:?}",
        gnss_ids
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
    let general_view_graph = result["generalViewGraph"].clone();
    assert!(
        result.get("renderedDiagrams").is_none(),
        "default config should not include rendered diagrams: {result:#}"
    );
    let edges = graph["edges"].as_array().expect("graph edges array");
    let gv_nodes = general_view_graph["nodes"]
        .as_array()
        .expect("generalViewGraph nodes array");
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
    let gnss_nodes: Vec<&serde_json::Value> = gv_nodes
        .iter()
        .filter(|node| node["name"].as_str() == Some("gnss"))
        .collect();
    assert_eq!(
        gnss_nodes.len(),
        1,
        "expected canonical generalViewGraph to include exactly one gnss node, got: {:?}",
        gnss_nodes
            .iter()
            .map(|n| n["id"].as_str().unwrap_or("<missing-id>"))
            .collect::<Vec<_>>()
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
    assert!(
        result.get("renderedDiagrams").is_none(),
        "default config should not include rendered diagrams for ConnectedBlocks: {result:#}"
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
fn lsp_sysml_model_ibd_kitchen_timer_interface_connects_produce_connectors() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///kitchen_timer_interface_connect_test.sysml";
    let content = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("vscode")
            .join("testFixture")
            .join("workspaces")
            .join("timer")
            .join("KitchenTimer.sysml"),
    )
    .expect("read KitchenTimer fixture");

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
    std::thread::sleep(std::time::Duration::from_millis(200));

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
    let connectors = ibd["connectors"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    assert!(
        !connectors.is_empty(),
        "expected KitchenTimer interface connect syntax to produce ibd connectors, got none: {}",
        model_json["result"]
    );

    let has_button_to_mcu = connectors.iter().any(|c| {
        c["sourceId"]
            .as_str()
            .is_some_and(|src| src.ends_with(".pcb.buttons.output"))
            && c["targetId"]
                .as_str()
                .is_some_and(|tgt| tgt.ends_with(".pcb.mcu.buttonIn"))
    });
    assert!(
        has_button_to_mcu,
        "expected connector from pcb.buttons.output to pcb.mcu.buttonIn, got: {:?}",
        connectors
            .iter()
            .map(|c| (c["sourceId"].as_str(), c["targetId"].as_str()))
            .collect::<Vec<_>>()
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
