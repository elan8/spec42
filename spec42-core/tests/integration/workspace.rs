//! Workspace scan integration tests.

use std::path::PathBuf;

use super::harness::{next_id, read_message, read_response, send_message, spawn_server};

/// Workspace scan: definition file exists only on disk; we never didOpen it.
/// Proves the server indexes files from the workspace root and goto_definition resolves across them.
#[test]
fn lsp_workspace_scan_goto_definition() {
    let temp = tempfile::tempdir().expect("temp dir");
    let root: PathBuf = temp.path().canonicalize().expect("canonical root");

    std::fs::write(root.join("def.sysml"), "package P { part def Engine; }").expect("write def");
    std::fs::write(root.join("use.sysml"), "package Q { part e : Engine; }").expect("write use");

    let root_uri = url::Url::from_file_path(&root).expect("root uri");
    let uri_use = url::Url::from_file_path(root.join("use.sysml")).expect("use uri");

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": root_uri.as_str(),
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    // Wait for workspace scan to index def.sysml and use.sysml from disk
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Open only the file that contains the usage; def.sysml is only in the index from the scan
    let did_open_use = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": uri_use.as_str(),
                "languageId": "sysml",
                "version": 1,
                "text": "package Q { part e : Engine; }"
            }
        }
    });
    send_message(&mut stdin, &did_open_use.to_string());
    std::thread::sleep(std::time::Duration::from_millis(50));

    let def_id = next_id();
    let def_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": def_id,
        "method": "textDocument/definition",
        "params": {
            "textDocument": { "uri": uri_use.as_str() },
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
        "goto_definition must resolve to def.sysml (loaded by workspace scan), got uri: {}",
        uri
    );

    let _ = child.kill();
}

/// When SYSML_V2_RELEASE_DIR is set, index that folder and assert workspace/symbol finds symbols.
/// Validates workspace awareness against the official OMG SysML v2 repo.
const SYSML_V2_RELEASE_DIR_ENV: &str = "SYSML_V2_RELEASE_DIR";

#[test]
fn lsp_workspace_scan_sysml_release() {
    let release_root = match std::env::var_os(SYSML_V2_RELEASE_DIR_ENV) {
        Some(v) => PathBuf::from(v),
        None => {
            eprintln!(
                "Skipping lsp_workspace_scan_sysml_release: set {} to the SysML-v2-Release clone root",
                SYSML_V2_RELEASE_DIR_ENV
            );
            return;
        }
    };
    if !release_root.is_dir() {
        eprintln!("Skipping: {} is not a directory", release_root.display());
        return;
    }

    let root_uri = match url::Url::from_file_path(&release_root) {
        Ok(u) => u,
        Err(_) => {
            eprintln!(
                "Skipping: cannot build file URL for {}",
                release_root.display()
            );
            return;
        }
    };

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": root_uri.as_str(),
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    // Allow time for scanning a large repo
    std::thread::sleep(std::time::Duration::from_secs(3));

    let sym_id = next_id();
    let sym_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": sym_id,
        "method": "workspace/symbol",
        "params": { "query": "Part" }
    });
    send_message(&mut stdin, &sym_req.to_string());
    let sym_resp = read_response(&mut stdout, sym_id).expect("workspace/symbol response");
    let sym_json: serde_json::Value =
        serde_json::from_str(&sym_resp).expect("parse workspace/symbol response");
    assert_eq!(sym_json["id"], sym_id);
    let results = sym_json["result"]
        .as_array()
        .expect("workspace/symbol returns array");
    assert!(
        !results.is_empty(),
        "workspace/symbol over SysML-v2-Release should return at least one symbol for query 'Part'"
    );

    let _ = child.kill();
}
