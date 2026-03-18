//! Diagnostics integration tests.

use super::harness::{next_id, read_message, send_message, spawn_server};

#[test]
fn lsp_diagnostics_on_invalid_sysml() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///bad.sysml";
    // Use invalid input that sysml-parser's parse_with_diagnostics reports (e.g. extra "}" or invalid keyword).
    // "package P { part def X " does NOT produce diagnostics - parser recovers without error.
    let content = "package P { } }"; // extra closing brace -> "expected end of input"

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

    // Server sends publishDiagnostics (notification); allow time for async processing
    std::thread::sleep(std::time::Duration::from_millis(500));
    // Drain notifications (no id); we expect at least one diagnostics notification
    let mut got_diagnostics = false;
    for _ in 0..20 {
        if let Some(msg) = read_message(&mut stdout) {
            let json: serde_json::Value = serde_json::from_str(&msg).ok().unwrap_or_default();
            if json["method"].as_str() == Some("textDocument/publishDiagnostics") {
                let diags = json["params"]["diagnostics"].as_array();
                if diags.map(|a| !a.is_empty()).unwrap_or(false) {
                    got_diagnostics = true;
                    break;
                }
            }
        } else {
            break;
        }
    }
    assert!(
        got_diagnostics,
        "invalid SysML should produce at least one diagnostic"
    );

    let _ = child.kill();
}
