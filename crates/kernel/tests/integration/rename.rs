//! Rename integration tests.

use super::harness::{
    lsp_barrier, next_id, read_message, read_response, send_message, spawn_server, TestSession,
};

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
    lsp_barrier(&mut stdin, &mut stdout);

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

fn position_for(content: &str, needle: &str) -> (u32, u32) {
    for (line_index, line) in content.lines().enumerate() {
        if let Some(character) = line.find(needle) {
            return (line_index as u32, character as u32);
        }
    }
    panic!("needle not found in content: {needle}");
}

fn rename_changes(
    session: &mut TestSession,
    uri: &str,
    line: u32,
    character: u32,
    new_name: &str,
) -> serde_json::Map<String, serde_json::Value> {
    let response = session.request(
        "textDocument/rename",
        serde_json::json!({
            "textDocument": { "uri": uri },
            "position": { "line": line, "character": character },
            "newName": new_name
        }),
    );
    response["result"]["changes"]
        .as_object()
        .cloned()
        .expect("rename changes object")
}

#[test]
fn lsp_rename_does_not_touch_same_file_homonyms() {
    let mut session = TestSession::new();
    let uri = "file:///rename/homonyms.sysml";
    let content = r#"package IT {
    part def Laptop {
        port hdmi;
    }
    part def Monitor {
        port hdmi;
    }
    part def Room {
        part laptop : Laptop;
        part monitor : Monitor;
        connect laptop.hdmi to monitor.hdmi;
    }
}"#;
    session.initialize_default("rename_homonym_test");
    session.did_open(uri, content, 1);
    session.barrier();

    let (line, character) = position_for(content, "port hdmi;");
    let changes = rename_changes(&mut session, uri, line, character + 6, "display");
    let edits = changes[uri].as_array().expect("file edits");
    assert!(
        edits.iter().any(|edit| edit["range"]["start"]["line"].as_u64() == Some(2)),
        "rename should include Laptop::hdmi declaration: {edits:?}"
    );
    assert!(
        !edits.iter().any(|edit| edit["range"]["start"]["line"].as_u64() == Some(5)),
        "rename should not include Monitor::hdmi declaration: {edits:?}"
    );
    assert_eq!(
        edits
            .iter()
            .filter(|edit| edit["range"]["start"]["line"].as_u64() == Some(10))
            .count(),
        1,
        "rename should include only laptop.hdmi on connect line: {edits:?}"
    );
}

#[test]
fn lsp_rename_ignores_comments_and_strings() {
    let mut session = TestSession::new();
    let uri = "file:///rename/comments.sysml";
    let content = r#"package Demo {
    part def Engine;
    part vehicle : Engine;
    // Engine should stay in this comment
    attribute label = "Engine should stay in this string";
}"#;
    session.initialize_default("rename_comments_test");
    session.did_open(uri, content, 1);
    session.barrier();

    let (line, character) = position_for(content, "part def Engine;");
    let changes = rename_changes(&mut session, uri, line, character + 9, "Motor");
    let edits = changes[uri].as_array().expect("file edits");
    assert!(
        edits.iter().any(|edit| edit["range"]["start"]["line"].as_u64() == Some(1)),
        "rename should include declaration: {edits:?}"
    );
    assert!(
        edits.iter().any(|edit| edit["range"]["start"]["line"].as_u64() == Some(2)),
        "rename should include typed usage: {edits:?}"
    );
    assert!(
        edits.iter().all(|edit| {
            let line = edit["range"]["start"]["line"].as_u64();
            line != Some(3) && line != Some(4)
        }),
        "rename should not edit comments or strings: {edits:?}"
    );
}

#[test]
fn lsp_prepare_rename_rejects_comments() {
    let mut session = TestSession::new();
    let uri = "file:///rename/comment-prepare.sysml";
    let content = "package Demo {\n  part def Engine;\n  // Engine comment\n}\n";
    session.initialize_default("prepare_rename_comment_test");
    session.did_open(uri, content, 1);
    session.barrier();

    let response = session.request(
        "textDocument/prepareRename",
        serde_json::json!({
            "textDocument": { "uri": uri },
            "position": { "line": 2, "character": 6 }
        }),
    );
    assert!(
        response["result"].is_null(),
        "prepareRename should reject comment text: {response}"
    );
}
