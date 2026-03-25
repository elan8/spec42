//! Integration tests for remaining roadmap LSP features.

use super::harness::{next_id, read_response, send_message, spawn_server};

fn initialize_and_open(
    stdin: &mut std::process::ChildStdin,
    stdout: &mut std::process::ChildStdout,
    uri: &str,
    content: &str,
) {
    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "remaining_lsp_features_test", "version": "0.1.0" }
        }
    });
    send_message(stdin, &init_req.to_string());
    let _ = read_response(stdout, init_id).expect("initialize response");
    send_message(
        stdin,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "method": "initialized",
            "params": {}
        })
        .to_string(),
    );
    send_message(
        stdin,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": content }
            }
        })
        .to_string(),
    );
    std::thread::sleep(std::time::Duration::from_millis(60));
}

#[test]
fn lsp_initialize_advertises_remaining_feature_capabilities() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": { "processId": null, "rootUri": null, "capabilities": {} }
    });
    send_message(&mut stdin, &init_req.to_string());
    let init_resp = read_response(&mut stdout, init_id).expect("init response");
    let init_json: serde_json::Value = serde_json::from_str(&init_resp).expect("parse init");
    let caps = &init_json["result"]["capabilities"];
    assert!(caps["signatureHelpProvider"].is_object());
    assert!(caps["selectionRangeProvider"].as_bool().unwrap_or(false));
    assert!(caps["documentLinkProvider"].is_object());
    assert!(caps["codeLensProvider"].is_object());
    assert!(caps["inlayHintProvider"].as_bool().unwrap_or(false));
    assert!(caps["linkedEditingRangeProvider"]
        .as_bool()
        .unwrap_or(false));
    assert!(caps["monikerProvider"].as_bool().unwrap_or(false));
    assert!(caps["callHierarchyProvider"].as_bool().unwrap_or(false));

    let _ = child.kill();
}

#[test]
fn lsp_remaining_feature_requests_round_trip() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");
    let uri = "file:///remaining-lsp.sysml";
    let content = "package P {\n  part def Engine;\n  part vehicle : Engine;\n}\n";
    initialize_and_open(&mut stdin, &mut stdout, uri, content);

    let requests = vec![
        (
            "textDocument/signatureHelp",
            serde_json::json!({"textDocument":{"uri":uri},"position":{"line":2,"character":16}}),
        ),
        (
            "textDocument/selectionRange",
            serde_json::json!({"textDocument":{"uri":uri},"positions":[{"line":2,"character":7}]}),
        ),
        (
            "textDocument/inlayHint",
            serde_json::json!({"textDocument":{"uri":uri},"range":{"start":{"line":0,"character":0},"end":{"line":3,"character":0}}}),
        ),
        (
            "textDocument/documentLink",
            serde_json::json!({"textDocument":{"uri":uri}}),
        ),
        (
            "textDocument/codeLens",
            serde_json::json!({"textDocument":{"uri":uri}}),
        ),
        (
            "textDocument/linkedEditingRange",
            serde_json::json!({"textDocument":{"uri":uri},"position":{"line":2,"character":7}}),
        ),
        (
            "textDocument/moniker",
            serde_json::json!({"textDocument":{"uri":uri},"position":{"line":2,"character":7}}),
        ),
        (
            "textDocument/prepareTypeHierarchy",
            serde_json::json!({"textDocument":{"uri":uri},"position":{"line":2,"character":7}}),
        ),
        (
            "textDocument/prepareCallHierarchy",
            serde_json::json!({"textDocument":{"uri":uri},"position":{"line":2,"character":7}}),
        ),
    ];

    for (method, params) in requests {
        let id = next_id();
        let req = serde_json::json!({
            "jsonrpc":"2.0",
            "id": id,
            "method": method,
            "params": params
        });
        send_message(&mut stdin, &req.to_string());
        let resp = read_response(&mut stdout, id).expect("feature response");
        let json: serde_json::Value = serde_json::from_str(&resp).expect("parse response");
        assert!(
            json.get("error").is_none(),
            "request {} returned error: {}",
            method,
            resp
        );
    }

    let _ = child.kill();
}

