//! LSP integration tests for interconnection visualization payload shape.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use super::harness::{next_id, read_message, send_message, spawn_server};
use super::perf_report::{graph_node_count, request_with_perf_capture, workspace_loaded_files};

const REF_PART_INTERCONNECTION_MODEL: &str = r#"package RefContext {
  port def LinkPort;

  part def System {
    port link : LinkPort;
  }

  part def Peer {
    port link : LinkPort;
  }

  part def Context {
    ref part system : System;
    part peer : Peer;
    connect system.link to peer.link;
  }

  part context : Context;

  view refConnections : InterconnectionView {
    expose context;
  }
}"#;

fn repo_examples_drone_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/drone")
}

#[test]
fn lsp_interconnection_visualization_returns_slim_scene_only_payload_for_drone() {
    let repo_root = repo_examples_drone_dir();
    assert!(
        repo_root.is_dir(),
        "expected drone example at {}",
        repo_root.display()
    );

    let root_uri = url::Url::from_directory_path(
        repo_root
            .canonicalize()
            .unwrap_or_else(|_| repo_root.clone()),
    )
    .expect("drone root uri");
    let views_uri =
        url::Url::from_file_path(repo_root.join("Views.sysml")).expect("Views.sysml uri");

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    send_message(
        &mut stdin,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "id": init_id,
            "method": "initialize",
            "params": {
                "processId": null,
                "rootUri": root_uri.as_str(),
                "capabilities": {},
                "initializationOptions": {
                    "workspace": { "maxFilesPerPattern": 1000 }
                },
                "clientInfo": { "name": "interconnection-slim-payload-test", "version": "0.1.0" }
            }
        })
        .to_string(),
    );
    let _ = read_message(&mut stdout).expect("init response");
    send_message(
        &mut stdin,
        &serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} }).to_string(),
    );

    let workspace_model_params = serde_json::json!({
        "textDocument": { "uri": views_uri.as_str() },
        "scope": ["graph", "stats", "workspaceVisualization"]
    });
    let _workspace_model_capture = {
        let wait_start = Instant::now();
        loop {
            let capture = request_with_perf_capture(
                &mut stdin,
                &mut stdout,
                "sysml/model",
                workspace_model_params.clone(),
            );
            let loaded_files = workspace_loaded_files(&capture.json);
            let graph_nodes = graph_node_count(&capture.json);
            if loaded_files > 0 && graph_nodes > 0 {
                break capture;
            }
            if wait_start.elapsed() >= Duration::from_secs(60) {
                panic!(
                    "workspace model did not become ready within 60s; last response: {:#?}",
                    capture.json
                );
            }
            std::thread::sleep(Duration::from_millis(250));
        }
    };

    let visualization_capture = request_with_perf_capture(
        &mut stdin,
        &mut stdout,
        "sysml/visualization",
        serde_json::json!({
            "workspaceRootUri": root_uri.as_str(),
            "view": "interconnection-view",
            "selectedView": "connections"
        }),
    );

    let result = &visualization_capture.json["result"];
    assert_eq!(result["selectedViewName"].as_str(), Some("connections"));
    assert_eq!(result["view"].as_str(), Some("interconnection-view"));

    let prepared = result
        .get("preparedView")
        .expect("preparedView should be present for interconnection LSP responses");
    assert!(
        !prepared.is_null(),
        "preparedView should not be null for connections view"
    );
    assert_eq!(prepared["view"].as_str(), Some("interconnection-view"));
    let edges = prepared["edges"]
        .as_array()
        .expect("preparedView.edges should be an array");
    assert!(
        !edges.is_empty(),
        "expected non-empty prepared interconnection view"
    );

    let scene = result.get("interconnectionScene");
    assert!(
        scene.is_none() || scene.is_some_and(|value| value.is_null()),
        "slim interconnection payload should omit interconnectionScene when preparedView is present, got: {scene:?}"
    );

    let ibd = result.get("ibd");
    assert!(
        ibd.is_none() || ibd.is_some_and(|value| value.is_null()),
        "slim interconnection payload should omit ibd when preparedView is present, got: {ibd:?}"
    );

    let graph = result.get("graph");
    assert!(
        graph.is_none() || graph.is_some_and(|value| value.is_null()),
        "slim interconnection payload should omit graph when preparedView is present, got: {graph:?}"
    );
    let general_view_graph = result.get("generalViewGraph");
    assert!(
        general_view_graph.is_none() || general_view_graph.is_some_and(|value| value.is_null()),
        "slim interconnection payload should omit generalViewGraph when preparedView is present, got: {general_view_graph:?}"
    );

    // Budget bumped from 52_000: prepared nodes/ports now carry `uri`/`range` for click-to-source
    // (previously hardcoded to `None` in `prepare_interconnection_prepared_view` — a real bug, not
    // a size-budget trade-off), which legitimately grows the slim payload by a few KB.
    const MAX_DRONE_SLIM_INTERCONNECTION_BYTES: usize = 62_000;
    let response_bytes = visualization_capture.raw.len();
    assert!(
        response_bytes <= MAX_DRONE_SLIM_INTERCONNECTION_BYTES,
        "slim interconnection payload should stay under {MAX_DRONE_SLIM_INTERCONNECTION_BYTES} bytes on drone, got {response_bytes}"
    );
    assert!(
        result.get("viewCandidates").is_some(),
        "slim interconnection payload should retain viewCandidates for the webview selector"
    );

    let _ = child.kill();
}

#[test]
fn lsp_interconnection_visualization_projects_connections_to_typed_ref_parts() {
    let workspace_dir = tempfile::tempdir().expect("temporary workspace");
    let model_path = workspace_dir.path().join("Model.sysml");
    std::fs::write(&model_path, REF_PART_INTERCONNECTION_MODEL).expect("write model fixture");

    let root_uri = url::Url::from_directory_path(workspace_dir.path()).expect("workspace root uri");
    let model_uri = url::Url::from_file_path(&model_path).expect("model uri");

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    send_message(
        &mut stdin,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "id": init_id,
            "method": "initialize",
            "params": {
                "processId": null,
                "rootUri": root_uri.as_str(),
                "capabilities": {},
                "initializationOptions": {
                    "workspace": { "maxFilesPerPattern": 100 }
                },
                "clientInfo": { "name": "ref-part-interconnection-test", "version": "0.1.0" }
            }
        })
        .to_string(),
    );
    let _ = read_message(&mut stdout).expect("init response");
    send_message(
        &mut stdin,
        &serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} }).to_string(),
    );

    let workspace_model_params = serde_json::json!({
        "textDocument": { "uri": model_uri.as_str() },
        "scope": ["graph", "stats", "workspaceVisualization"]
    });
    let wait_start = Instant::now();
    loop {
        let capture = request_with_perf_capture(
            &mut stdin,
            &mut stdout,
            "sysml/model",
            workspace_model_params.clone(),
        );
        if workspace_loaded_files(&capture.json) > 0 && graph_node_count(&capture.json) > 0 {
            break;
        }
        if wait_start.elapsed() >= Duration::from_secs(60) {
            panic!(
                "workspace model did not become ready within 60s; last response: {:#?}",
                capture.json
            );
        }
        std::thread::sleep(Duration::from_millis(250));
    }

    let visualization = request_with_perf_capture(
        &mut stdin,
        &mut stdout,
        "sysml/visualization",
        serde_json::json!({
            "workspaceRootUri": root_uri.as_str(),
            "view": "interconnection-view",
            "selectedView": "refConnections"
        }),
    );
    let result = &visualization.json["result"];
    assert_eq!(result["selectedViewName"].as_str(), Some("refConnections"));

    let prepared = result["preparedView"]
        .as_object()
        .expect("preparedView should be present");
    let nodes = prepared["nodes"]
        .as_array()
        .expect("preparedView.nodes should be an array");
    let reference_part = nodes
        .iter()
        .find(|node| {
            node["attributes"]["qualifiedName"]
                .as_str()
                .is_some_and(|name| name.ends_with(".context.system"))
        })
        .expect("preparedView should contain the typed reference part");
    assert_eq!(
        reference_part["attributes"]["isReference"].as_bool(),
        Some(true)
    );
    let reference_ports = reference_part["attributes"]["portDetails"]
        .as_array()
        .expect("reference part should expose inherited port details");
    assert!(reference_ports.iter().any(|port| port["name"] == "link"));

    let peer_part = nodes
        .iter()
        .find(|node| {
            node["attributes"]["qualifiedName"]
                .as_str()
                .is_some_and(|name| name.ends_with(".context.peer"))
        })
        .expect("preparedView should contain the peer part");
    let peer_ports = peer_part["attributes"]["portDetails"]
        .as_array()
        .expect("peer part should expose port details");
    assert!(peer_ports.iter().any(|port| port["name"] == "link"));

    let edges = prepared["edges"]
        .as_array()
        .expect("preparedView.edges should be an array");
    assert_eq!(edges.len(), 1, "expected exactly one prepared connection");
    assert!(edges[0]["attributes"]["sourcePortId"].is_string());
    assert!(edges[0]["attributes"]["targetPortId"].is_string());
    assert!(
        result.get("emptyStateMessage").is_none()
            || result["emptyStateMessage"].is_null()
            || !result["emptyStateMessage"]
                .as_str()
                .is_some_and(|message| message.contains("no internal connectors")),
        "a connected ref-part view must not report the no-connectors empty state"
    );

    let _ = child.kill();
}
