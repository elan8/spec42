//! Goto definition integration tests.

use super::harness::{
    lsp_barrier, next_id, read_message, read_response, send_message, spawn_server,
};

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
    lsp_barrier(&mut stdin, &mut stdout);

    // Go to definition on "A" (usage "part a : A" -> def A). Retry for CI determinism.
    let mut resolved_uri: Option<String> = None;
    for _ in 0..20 {
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

        let found_uri = if let Some(uri) = result["uri"].as_str() {
            Some(uri.to_string())
        } else {
            result
                .as_array()
                .and_then(|arr| arr.first())
                .and_then(|loc| loc["uri"].as_str())
                .map(|uri| uri.to_string())
        };
        if let Some(uri) = found_uri {
            resolved_uri = Some(uri);
            break;
        }
        lsp_barrier(&mut stdin, &mut stdout);
    }

    if let Some(u) = resolved_uri {
        assert!(u.contains("def_test.sysml"));
    } else {
        panic!("definition should return location with uri");
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
    let barrier_id = next_id();
    let barrier_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": barrier_id,
        "method": "workspace/symbol",
        "params": { "query": "" }
    });
    send_message(&mut stdin, &barrier_req.to_string());
    let _ = read_response(&mut stdout, barrier_id).expect("workspace barrier response");

    // Go to definition on "Engine" in use.sysml (position at "Engine" in "part e : Engine").
    // Retry in case indexing in CI is delayed.
    let mut resolved_uri: Option<String> = None;
    for _ in 0..20 {
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

        let found_uri = if let Some(uri) = result["uri"].as_str() {
            Some(uri.to_string())
        } else {
            result
                .as_array()
                .and_then(|arr| arr.first())
                .and_then(|loc| loc["uri"].as_str())
                .map(|uri| uri.to_string())
        };

        if let Some(uri) = found_uri {
            if uri.contains("def.sysml") {
                resolved_uri = Some(uri);
                break;
            }
            resolved_uri = Some(uri);
        }
        lsp_barrier(&mut stdin, &mut stdout);
    }

    let uri = resolved_uri.expect("definition should return location with uri");
    assert!(
        uri.contains("def.sysml"),
        "goto_definition should resolve to def.sysml, got uri: {}",
        uri
    );

    let _ = child.kill();
}

#[test]
fn lsp_goto_definition_resolves_public_reexported_type() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri_core = "file:///workspace/core.sysml";
    let uri_domain = "file:///workspace/domain.sysml";
    let uri_use = "file:///workspace/use.sysml";
    let content_core = "package Core { attribute def Name; }";
    let content_domain = "package Domain { public import Core::*; }";
    let content_use =
        "package Demo { import Domain::*; part def Consumer { attribute groupName : Name; } }";

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
    send_message(
        &mut stdin,
        &serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} }).to_string(),
    );

    for (uri, text) in [
        (uri_core, content_core),
        (uri_domain, content_domain),
        (uri_use, content_use),
    ] {
        send_message(
            &mut stdin,
            &serde_json::json!({
                "jsonrpc": "2.0",
                "method": "textDocument/didOpen",
                "params": {
                    "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": text }
                }
            })
            .to_string(),
        );
    }
    lsp_barrier(&mut stdin, &mut stdout);

    let mut resolved_uri: Option<String> = None;
    for _ in 0..20 {
        let def_id = next_id();
        let def_req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": def_id,
            "method": "textDocument/definition",
            "params": {
                "textDocument": { "uri": uri_use },
                "position": { "line": 0, "character": 75 }
            }
        });
        send_message(&mut stdin, &def_req.to_string());
        let def_resp = read_response(&mut stdout, def_id).expect("definition response");
        let def_json: serde_json::Value =
            serde_json::from_str(&def_resp).expect("parse definition response");
        let result = &def_json["result"];

        let found_uri = if let Some(uri) = result["uri"].as_str() {
            Some(uri.to_string())
        } else {
            result
                .as_array()
                .and_then(|arr| arr.first())
                .and_then(|loc| loc["uri"].as_str())
                .map(|uri| uri.to_string())
        };

        if let Some(uri) = found_uri {
            resolved_uri = Some(uri);
            break;
        }
        lsp_barrier(&mut stdin, &mut stdout);
    }

    let uri = resolved_uri.expect("definition should return location with uri");
    assert!(
        uri.contains("core.sysml"),
        "goto_definition should resolve re-exported Name to core.sysml, got uri: {}",
        uri
    );

    let _ = child.kill();
}
