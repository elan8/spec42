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

/// Same-file homonyms: references for one declaration should not include another declaration with same short name.
#[test]
fn lsp_same_file_homonym_references_are_disambiguated_by_position() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///refs/laptop.sysml";
    let content = r#"package IT {
    part def Laptop {
        port hdmi;
    }
    part def Monitor {
        port hdmi;
    }
}"#;

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

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": content }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(80));

    // Query references for Laptop::hdmi declaration (line 2).
    let ref_id = next_id();
    let ref_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": ref_id,
        "method": "textDocument/references",
        "params": {
            "textDocument": { "uri": uri },
            "position": { "line": 2, "character": 13 },
            "context": { "includeDeclaration": true }
        }
    });
    send_message(&mut stdin, &ref_req.to_string());
    let ref_resp = read_response(&mut stdout, ref_id).expect("references response");
    let ref_json: serde_json::Value =
        serde_json::from_str(&ref_resp).expect("parse references response");
    let locs = ref_json["result"]
        .as_array()
        .expect("references should return array");

    // Must include Laptop declaration line.
    assert!(
        locs.iter().any(|l| l["range"]["start"]["line"].as_u64() == Some(2)),
        "references should include Laptop::hdmi declaration: {:?}",
        locs
    );
    // Must not include Monitor declaration line.
    assert!(
        !locs.iter().any(|l| l["range"]["start"]["line"].as_u64() == Some(5)),
        "references should not include Monitor::hdmi declaration: {:?}",
        locs
    );

    let _ = child.kill();
}

#[test]
fn lsp_dotted_usage_disambiguates_same_name_members() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///refs/dotted.sysml";
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

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": content }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(80));

    let ref_id = next_id();
    let ref_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": ref_id,
        "method": "textDocument/references",
        "params": {
            "textDocument": { "uri": uri },
            "position": { "line": 2, "character": 13 },
            "context": { "includeDeclaration": true }
        }
    });
    send_message(&mut stdin, &ref_req.to_string());
    let ref_resp = read_response(&mut stdout, ref_id).expect("references response");
    let ref_json: serde_json::Value =
        serde_json::from_str(&ref_resp).expect("parse references response");
    let locs = ref_json["result"]
        .as_array()
        .expect("references should return array");

    assert!(
        locs.iter().any(|l| {
            l["range"]["start"]["line"].as_u64() == Some(2)
                && l["range"]["start"]["character"].as_u64() == Some(13)
        }),
        "references should include Laptop::hdmi declaration: {:?}",
        locs
    );
    let usage_refs_on_connect_line = locs
        .iter()
        .filter(|l| l["range"]["start"]["line"].as_u64() == Some(10))
        .count();
    assert_eq!(
        usage_refs_on_connect_line, 1,
        "references should include exactly one hdmi endpoint on connect line: {:?}",
        locs
    );

    let _ = child.kill();
}

#[test]
fn lsp_same_short_name_in_library_is_not_counted_without_semantic_match() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri_workspace = "file:///refs/workspace.sysml";
    let uri_library = "file:///stdlib/lib.sysml";
    let workspace_content = r#"package W {
    part def Laptop {
        port power;
    }
}"#;
    let library_content = r#"package L {
    part def Generator {
        port power;
    }
}"#;

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

    let did_open_workspace = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri_workspace, "languageId": "sysml", "version": 1, "text": workspace_content }
        }
    });
    send_message(&mut stdin, &did_open_workspace.to_string());
    let did_open_library = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri_library, "languageId": "sysml", "version": 1, "text": library_content }
        }
    });
    send_message(&mut stdin, &did_open_library.to_string());
    std::thread::sleep(std::time::Duration::from_millis(80));

    let ref_id = next_id();
    let ref_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": ref_id,
        "method": "textDocument/references",
        "params": {
            "textDocument": { "uri": uri_workspace },
            "position": { "line": 2, "character": 13 },
            "context": { "includeDeclaration": true }
        }
    });
    send_message(&mut stdin, &ref_req.to_string());
    let ref_resp = read_response(&mut stdout, ref_id).expect("references response");
    let ref_json: serde_json::Value =
        serde_json::from_str(&ref_resp).expect("parse references response");
    let locs = ref_json["result"]
        .as_array()
        .expect("references should return array");
    assert_eq!(
        locs.len(),
        1,
        "workspace power should only resolve to its own declaration: {:?}",
        locs
    );
    assert!(
        locs.iter()
            .all(|l| l["uri"].as_str().is_some_and(|u| u == uri_workspace)),
        "references should not include library declaration: {:?}",
        locs
    );

    let _ = child.kill();
}

