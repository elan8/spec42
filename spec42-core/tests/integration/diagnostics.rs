//! Diagnostics integration tests.

use super::harness::{next_id, read_message, send_message, spawn_server};
use std::fs;

#[test]
fn lsp_diagnostics_on_invalid_sysml() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///bad.sysml";
    // Use invalid input that sysml-parser's parse_with_diagnostics reports (e.g. extra "}" or invalid keyword).
    // "package P { part def X " does NOT produce diagnostics - parser recovers without error.
    let content = "package P { } }"; // extra closing brace -> "expected end of input"

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

    // Server sends publishDiagnostics (notification); allow time for async processing
    std::thread::sleep(std::time::Duration::from_millis(500));
    // Drain notifications (no id); we expect at least one diagnostics notification
    let mut got_diagnostics = false;
    for _ in 0..20 {
        if let Some(msg) = read_message(&mut stdout) {
            let json: serde_json::Value = serde_json::from_str(&msg).ok().unwrap_or_default();
            if json["method"].as_str() == Some("textDocument/publishDiagnostics") {
                let diags = json["params"]["diagnostics"].as_array();
                if diags.map(|a| !a.is_empty()).unwrap_or(false) {
                    got_diagnostics = true;
                    break;
                }
            }
        } else {
            break;
        }
    }
    assert!(
        got_diagnostics,
        "invalid SysML should produce at least one diagnostic"
    );

    let _ = child.kill();
}

#[test]
fn surveillance_drone_semantic_diagnostics_have_meaningful_ranges() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///surveillance_drone_diag_test.sysml";
    let fixture_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("surveillance_drone_full.sysml");
    let content = fs::read_to_string(&fixture_path).expect("read drone fixture");

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
    std::thread::sleep(std::time::Duration::from_millis(600));

    let mut semantic_diags: Vec<serde_json::Value> = Vec::new();
    for _ in 0..30 {
        let Some(msg) = read_message(&mut stdout) else {
            break;
        };
        let json: serde_json::Value = serde_json::from_str(&msg).unwrap_or_default();
        if json["method"].as_str() != Some("textDocument/publishDiagnostics") {
            continue;
        }
        let diags = json["params"]["diagnostics"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        semantic_diags.extend(
            diags.into_iter()
                .filter(|d| d["source"].as_str() == Some("semantic")),
        );
        // Stop after the first diagnostics publication to avoid blocking on
        // additional messages that may never arrive.
        break;
    }

    assert!(
        !semantic_diags.is_empty(),
        "expected semantic diagnostics for drone fixture"
    );
    let at_1_1 = semantic_diags
        .iter()
        .filter(|d| {
            d["range"]["start"]["line"].as_u64() == Some(0)
                && d["range"]["start"]["character"].as_u64() == Some(0)
                && d["range"]["end"]["line"].as_u64() == Some(0)
                && d["range"]["end"]["character"].as_u64() == Some(0)
        })
        .count();
    assert_eq!(
        at_1_1, 0,
        "expected semantic diagnostics to avoid line1/col1 sentinel ranges"
    );

    let unconnected_count = semantic_diags
        .iter()
        .filter(|d| d["code"].as_str() == Some("unconnected_port"))
        .count();
    assert!(
        unconnected_count <= 25,
        "expected reduced unconnected_port noise, got {unconnected_count}"
    );

    let _ = child.kill();
}
