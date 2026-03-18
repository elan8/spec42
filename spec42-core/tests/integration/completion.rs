//! Completion integration tests.

use super::harness::{next_id, read_message, read_response, send_message, spawn_server};

#[test]
fn lsp_completion() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///test2.sysml";
    let content = "package P { part def X; }";

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
    std::thread::sleep(std::time::Duration::from_millis(50));

    let compl_id = next_id();
    let compl_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": compl_id,
        "method": "textDocument/completion",
        "params": {
            "textDocument": { "uri": uri },
            "position": { "line": 0, "character": 2 }
        }
    });
    send_message(&mut stdin, &compl_req.to_string());
    let compl_resp = read_response(&mut stdout, compl_id).expect("completion response");
    let compl_json: serde_json::Value =
        serde_json::from_str(&compl_resp).expect("parse completion response");
    assert_eq!(compl_json["id"], compl_id);
    let items = compl_json["result"]
        .as_array()
        .or_else(|| compl_json["result"]["items"].as_array());
    assert!(
        items.is_some(),
        "completion should return array: {}",
        compl_resp
    );
    let items = items.unwrap();
    assert!(
        !items.is_empty(),
        "completion should have at least one item"
    );
    let labels: Vec<String> = items
        .iter()
        .filter_map(|i| i["label"].as_str().map(String::from))
        .collect();
    assert!(
        labels.iter().any(|l| l == "part" || l == "package"),
        "completion should include keywords: {:?}",
        labels
    );

    let _ = child.kill();
}
