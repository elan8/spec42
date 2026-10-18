//! Indexed project diagnostics do not belong to editor tabs.
use super::harness::{next_id, read_message, read_response, send_message, spawn_server};
use serde_json::{json, Value};

#[test]
fn project_diagnostics_survive_tabs_and_closure_until_file_removal() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().canonicalize().unwrap();
    let broken = root.join("broken.sysml");
    let other = root.join("other.kerml");
    let broken_source = "package Broken { } }";
    let other_source = "package Other { class Kept; }";
    std::fs::write(&broken, broken_source).unwrap();
    std::fs::write(&other, other_source).unwrap();
    let uri = |path| {
        sysml_query::source::normalize_uri(&url::Url::from_file_path(path).unwrap()).to_string()
    };
    let broken_uri = uri(&broken);
    let other_uri = uri(&other);
    let mut server = spawn_server();
    let mut stdin = server.stdin.take().unwrap();
    let mut stdout = server.stdout.take().unwrap();
    let id = next_id();
    send_message(&mut stdin, &json!({"jsonrpc":"2.0", "id":id, "method":"initialize", "params":{
        "processId":null, "rootUri":url::Url::from_directory_path(&root).unwrap(), "capabilities":{}
    }}).to_string());
    read_response(&mut stdout, id).unwrap();
    send_message(
        &mut stdin,
        &json!({"jsonrpc":"2.0", "method":"initialized", "params":{}}).to_string(),
    );
    let initial = loop {
        let message: Value =
            serde_json::from_str(&read_message(&mut stdout).expect("initial diagnostics")).unwrap();
        if message["method"] == "textDocument/publishDiagnostics"
            && message["params"]["uri"] == broken_uri
        {
            let diagnostics = message["params"]["diagnostics"].as_array().unwrap().clone();
            assert!(
                !diagnostics.is_empty(),
                "indexed unopened sources must be diagnosed"
            );
            break diagnostics;
        }
    };
    for (uri, text, language) in [
        (&broken_uri, broken_source, "sysml"),
        (&other_uri, other_source, "kerml"),
    ] {
        send_message(
            &mut stdin,
            &json!({"jsonrpc":"2.0", "method":"textDocument/didOpen", "params":{
                "textDocument":{"uri":uri, "languageId":language, "version":1, "text":text}
            }})
            .to_string(),
        );
    }
    send_message(
        &mut stdin,
        &json!({"jsonrpc":"2.0", "method":"textDocument/didClose", "params":{
            "textDocument":{"uri":broken_uri}
        }})
        .to_string(),
    );
    let id = next_id();
    send_message(
        &mut stdin,
        &json!({"jsonrpc":"2.0", "id":id, "method":"workspace/symbol", "params":{"query":"Kept"}})
            .to_string(),
    );
    loop {
        let message: Value =
            serde_json::from_str(&read_message(&mut stdout).expect("tab-change barrier")).unwrap();
        if message["method"] == "textDocument/publishDiagnostics"
            && message["params"]["uri"] == broken_uri
        {
            assert_eq!(
                message["params"]["diagnostics"],
                json!(initial),
                "tabs must not change project diagnostics"
            );
        }
        if message["id"] == id {
            assert!(message["result"]
                .as_array()
                .unwrap()
                .iter()
                .any(|symbol| symbol["name"] == "Kept"));
            break;
        }
    }
    std::fs::remove_file(&broken).unwrap();
    send_message(
        &mut stdin,
        &json!({"jsonrpc":"2.0", "method":"workspace/didChangeWatchedFiles", "params":{
            "changes":[{"uri":broken_uri,"type":3}]
        }})
        .to_string(),
    );
    loop {
        let message: Value = serde_json::from_str(
            &read_message(&mut stdout).expect("deleted-source diagnostic clear"),
        )
        .unwrap();
        if message["method"] == "textDocument/publishDiagnostics"
            && message["params"]["uri"] == broken_uri
        {
            assert_eq!(message["params"]["diagnostics"], json!([]));
            break;
        }
    }
}
