//! Goto definition integration tests.

use super::harness::{next_id, read_message, read_response, send_message, spawn_server};

#[test]
fn lsp_goto_definition() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///def_test.sysml";
    let content = "package P { part def A; part a : A; }";

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

    // Go to definition on "A" (usage "part a : A" -> def A). Line 0, character after "part a : "
    let def_id = next_id();
    let def_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": def_id,
        "method": "textDocument/definition",
        "params": {
            "textDocument": { "uri": uri },
            "position": { "line": 0, "character": 22 }
        }
    });
    send_message(&mut stdin, &def_req.to_string());
    let def_resp = read_response(&mut stdout, def_id).expect("definition response");
    let def_json: serde_json::Value =
        serde_json::from_str(&def_resp).expect("parse definition response");
    assert_eq!(def_json["id"], def_id);
    let result = &def_json["result"];
    assert!(
        result.is_object() || result["uri"].is_string(),
        "definition should return location: {}",
        def_resp
    );
    if let Some(u) = result["uri"].as_str() {
        assert!(u.contains("def_test.sysml"));
    }

    let _ = child.kill();
}

/// Cross-file goto definition: symbol defined in file_def.sysml, used in file_use.sysml.
#[test]
fn lsp_cross_file_goto_definition() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri_def = "file:///workspace/def.sysml";
    let uri_use = "file:///workspace/use.sysml";
    let content_def = "package P { part def Engine; }";
    let content_use = "package Q { part e : Engine; }";

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": "file:///workspace",
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    // Open both documents so the index has both (definition in def, usage in use)
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

    // Go to definition on "Engine" in use.sysml (position at "Engine" in "part e : Engine")
    let def_id = next_id();
    let def_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": def_id,
        "method": "textDocument/definition",
        "params": {
            "textDocument": { "uri": uri_use },
            "position": { "line": 0, "character": 22 }
        }
    });
    send_message(&mut stdin, &def_req.to_string());
    let def_resp = read_response(&mut stdout, def_id).expect("definition response");
    let def_json: serde_json::Value =
        serde_json::from_str(&def_resp).expect("parse definition response");
    assert_eq!(def_json["id"], def_id);
    let result = &def_json["result"];
    let uri = result["uri"]
        .as_str()
        .expect("definition should return location with uri");
    assert!(
        uri.contains("def.sysml"),
        "goto_definition should resolve to def.sysml, got uri: {}",
        uri
    );

    let _ = child.kill();
}
