//! Lifecycle integration tests for repeated open/edit/close flows.

use super::harness::{next_id, read_message, read_response, send_message, spawn_server};

#[test]
fn lsp_repeated_open_edit_close_keeps_server_usable() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///lifecycle.sysml";
    let original = "package P {\n  part def Engine;\n}\n";
    let edited = "package P {\n  part def EngineCore;\n}\n";

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "lifecycle_test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("initialize response");

    let initialized = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_message(&mut stdin, &initialized.to_string());

    for version in 1..=3 {
        let did_open = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": uri,
                    "languageId": "sysml",
                    "version": version,
                    "text": original
                }
            }
        });
        send_message(&mut stdin, &did_open.to_string());

        let did_change = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didChange",
            "params": {
                "textDocument": {
                    "uri": uri,
                    "version": version + 10
                },
                "contentChanges": [
                    {
                        "text": edited
                    }
                ]
            }
        });
        send_message(&mut stdin, &did_change.to_string());

        let did_close = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didClose",
            "params": {
                "textDocument": {
                    "uri": uri
                }
            }
        });
        send_message(&mut stdin, &did_close.to_string());
    }

    std::thread::sleep(std::time::Duration::from_millis(100));

    let hover_id = next_id();
    let hover_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": hover_id,
        "method": "textDocument/hover",
        "params": {
            "textDocument": { "uri": uri },
            "position": { "line": 1, "character": 12 }
        }
    });
    send_message(&mut stdin, &hover_req.to_string());
    let hover_resp = read_response(&mut stdout, hover_id)
        .expect("hover response after repeated open/edit/close");
    let hover_json: serde_json::Value =
        serde_json::from_str(&hover_resp).expect("parse hover response");
    assert_eq!(hover_json["id"], hover_id);
    let contents = hover_json["result"]["contents"]["value"]
        .as_str()
        .or_else(|| hover_json["result"]["contents"].as_str());
    assert!(
        contents.is_some(),
        "hover should return contents after repeated open/edit/close: {}",
        hover_resp
    );
    let contents = contents.unwrap();
    assert!(
        contents.contains("EngineCore"),
        "hover should reflect the last edited document state: {}",
        contents
    );

    let _ = child.kill();
}

#[test]
fn lsp_incremental_utf16_edit_after_emoji_keeps_hover_working() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///lifecycle_utf16.sysml";
    let original = "package P {\n  // ok \u{1F600} here\n  part def Engine;\n}\n";

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "lifecycle_utf16_test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("initialize response");

    let initialized = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": uri,
                "languageId": "sysml",
                "version": 1,
                "text": original
            }
        }
    });
    send_message(&mut stdin, &did_open.to_string());

    let did_change = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didChange",
        "params": {
            "textDocument": {
                "uri": uri,
                "version": 2
            },
            "contentChanges": [
                {
                    "range": {
                        "start": { "line": 1, "character": 10 },
                        "end": { "line": 1, "character": 10 }
                    },
                    "text": "still "
                }
            ]
        }
    });
    send_message(&mut stdin, &did_change.to_string());
    std::thread::sleep(std::time::Duration::from_millis(100));

    let hover_id = next_id();
    let hover_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": hover_id,
        "method": "textDocument/hover",
        "params": {
            "textDocument": { "uri": uri },
            "position": { "line": 2, "character": 12 }
        }
    });
    send_message(&mut stdin, &hover_req.to_string());
    let hover_resp = read_response(&mut stdout, hover_id).expect("hover response after utf16 edit");
    let hover_json: serde_json::Value =
        serde_json::from_str(&hover_resp).expect("parse hover response");
    assert_eq!(hover_json["id"], hover_id);
    let contents = hover_json["result"]["contents"]["value"]
        .as_str()
        .or_else(|| hover_json["result"]["contents"].as_str());
    assert!(
        contents.is_some(),
        "hover should return contents after utf16 edit: {}",
        hover_resp
    );
    let contents = contents.unwrap();
    assert!(
        contents.to_lowercase().contains("part"),
        "hover should still work after utf16 edit: {}",
        contents
    );

    let _ = child.kill();
}
