//! Find references integration tests.

use super::harness::{next_id, read_message, read_response, send_message, spawn_server};

/// Cross-file references: find references to a symbol defined in one file and used in another.
#[test]
fn lsp_cross_file_references() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri_def = "file:///refs/def.sysml";
    let uri_use = "file:///refs/use.sysml";
    let content_def = "package P { part def Widget; }";
    let content_use = "package Q { part w : Widget; }";

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": "file:///refs",
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    let did_open_def = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri_def, "languageId": "sysml", "version": 1, "text": content_def }
        }
    });
    send_message(&mut stdin, &did_open_def.to_string());
    let did_open_use = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri_use, "languageId": "sysml", "version": 1, "text": content_use }
        }
    });
    send_message(&mut stdin, &did_open_use.to_string());
    std::thread::sleep(std::time::Duration::from_millis(80));

    // Find references at "Widget" in use.sysml (include_declaration = true -> def + use)
    let ref_id = next_id();
    let ref_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": ref_id,
        "method": "textDocument/references",
        "params": {
            "textDocument": { "uri": uri_use },
            "position": { "line": 0, "character": 21 },
            "context": { "includeDeclaration": true }
        }
    });
    send_message(&mut stdin, &ref_req.to_string());
    let ref_resp = read_response(&mut stdout, ref_id).expect("references response");
    let ref_json: serde_json::Value =
        serde_json::from_str(&ref_resp).expect("parse references response");
    assert_eq!(ref_json["id"], ref_id);
    let locs = ref_json["result"]
        .as_array()
        .expect("references should return array");
    let uris: Vec<String> = locs
        .iter()
        .filter_map(|l| l["uri"].as_str().map(String::from))
        .collect();
    assert!(
        uris.iter().any(|u| u.contains("def.sysml")),
        "references should include def.sysml: {:?}",
        uris
    );
    assert!(
        uris.iter().any(|u| u.contains("use.sysml")),
        "references should include use.sysml: {:?}",
        uris
    );

    let _ = child.kill();
}
