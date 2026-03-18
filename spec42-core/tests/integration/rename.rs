//! Rename integration tests.

use super::harness::{next_id, read_message, read_response, send_message, spawn_server};

/// Rename: prepareRename returns range; rename returns WorkspaceEdit updating all references.
#[test]
fn lsp_rename() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri_def = "file:///rename/def.sysml";
    let uri_use = "file:///rename/use.sysml";
    let content_def = "package P { part def Foo; }";
    let content_use = "package Q { part f : Foo; }";

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": "file:///rename",
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

    // prepareRename at "Foo" in def.sysml ("package P { part def Foo; }" -> Foo at line 0, char 21)
    let prep_id = next_id();
    let prep_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": prep_id,
        "method": "textDocument/prepareRename",
        "params": {
            "textDocument": { "uri": uri_def },
            "position": { "line": 0, "character": 21 }
        }
    });
    send_message(&mut stdin, &prep_req.to_string());
    let prep_resp = read_response(&mut stdout, prep_id).expect("prepareRename response");
    let prep_json: serde_json::Value =
        serde_json::from_str(&prep_resp).expect("parse prepareRename response");
    assert_eq!(prep_json["id"], prep_id);
    let prep_result = &prep_json["result"];
    assert!(
        !prep_result.is_null(),
        "prepareRename should return range or result"
    );
    if prep_result.get("range").is_some() {
        assert!(
            prep_result["range"]["start"].is_object() && prep_result["range"]["end"].is_object()
        );
    } else {
        assert!(
            prep_result.get("start").is_some() && prep_result.get("end").is_some(),
            "prepareRename result should have range"
        );
    }

    // rename Foo -> Bar (same position as prepareRename)
    let ren_id = next_id();
    let ren_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": ren_id,
        "method": "textDocument/rename",
        "params": {
            "textDocument": { "uri": uri_def },
            "position": { "line": 0, "character": 21 },
            "newName": "Bar"
        }
    });
    send_message(&mut stdin, &ren_req.to_string());
    let ren_resp = read_response(&mut stdout, ren_id).expect("rename response");
    let ren_json: serde_json::Value =
        serde_json::from_str(&ren_resp).expect("parse rename response");
    assert_eq!(ren_json["id"], ren_id);
    let changes = ren_json["result"]["changes"]
        .as_object()
        .expect("rename should return WorkspaceEdit with changes");
    assert!(
        !changes.is_empty(),
        "rename should have at least one file in changes"
    );
    let uris: Vec<&str> = changes.keys().map(|k| k.as_str()).collect();
    assert!(
        uris.iter().any(|u| u.contains("def.sysml")),
        "changes should include def.sysml: {:?}",
        uris
    );
    assert!(
        uris.iter().any(|u| u.contains("use.sysml")),
        "changes should include use.sysml: {:?}",
        uris
    );
    for (_uri, edits) in changes {
        let edits_arr = edits.as_array().expect("edits per file should be array");
        for edit in edits_arr {
            assert_eq!(
                edit["newText"].as_str(),
                Some("Bar"),
                "each edit should replace with Bar"
            );
        }
    }

    let _ = child.kill();
}
