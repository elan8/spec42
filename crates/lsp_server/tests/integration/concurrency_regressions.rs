//! Regression tests for the `ServerState` -> `SessionActor`/`SnapshotHandle` migration.
//!
//! These specifically guard the property that motivated the whole migration: a slow
//! background rebuild (relink, or the startup workspace scan) must never block an unrelated,
//! cheap read request (hover) behind it. Before the migration, `sysml_model_result` held the
//! server's single `RwLock` read guard for the duration of a full workspace visualization
//! rebuild, so *any* concurrent request queued behind it for as long as that rebuild took
//! (20-30+ seconds on a real workspace). Under the actor, reads only ever look at the latest
//! published snapshot and never wait on the actor's mailbox, so these requests should return
//! promptly regardless of what the actor is doing in the background.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use super::harness::{next_id, read_message, read_response, send_message, spawn_server};

/// Generous but still meaningful bound: a hover/goto-definition response for a handful of
/// lines of SysML should return in well under a second once the actor has anything published
/// at all. 5 seconds gives slow CI machines headroom while still catching a real regression
/// back to the old 20-30s-class stall.
const NOT_BLOCKED_BOUND: Duration = Duration::from_secs(5);

#[test]
fn hover_request_not_blocked_by_concurrent_relink() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///concurrent_relink.sysml";
    let content = "package P {\n  part def Engine;\n  part motor : Engine;\n}\n";

    let init_id = next_id();
    send_message(
        &mut stdin,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "id": init_id,
            "method": "initialize",
            "params": {
                "processId": null,
                "rootUri": null,
                "capabilities": {},
                "clientInfo": { "name": "concurrent_relink_test", "version": "0.1.0" }
            }
        })
        .to_string(),
    );
    let _ = read_message(&mut stdout).expect("initialize response");
    send_message(
        &mut stdin,
        &serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} }).to_string(),
    );

    // Open the document — this schedules an async relink (90ms debounced) that runs in a
    // detached task, entirely independent of the request below.
    send_message(
        &mut stdin,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": content }
            }
        })
        .to_string(),
    );

    // Fire the hover request immediately — no barrier, no sleep — while the relink triggered
    // by didOpen may still be in flight in the background.
    let hover_id = next_id();
    let started = Instant::now();
    send_message(
        &mut stdin,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "id": hover_id,
            "method": "textDocument/hover",
            "params": {
                "textDocument": { "uri": uri },
                "position": { "line": 2, "character": 8 }
            }
        })
        .to_string(),
    );
    let hover_resp = read_response(&mut stdout, hover_id).expect("hover response");
    let elapsed = started.elapsed();

    assert!(
        elapsed < NOT_BLOCKED_BOUND,
        "hover must not block behind a concurrent relink; took {elapsed:?}: {hover_resp}"
    );

    let _ = child.kill();
}

#[test]
fn hover_request_not_blocked_by_concurrent_startup_scan() {
    // Give the startup scan real work: several files under a real workspace root, so
    // `initialized`'s scan+relink pipeline isn't a no-op.
    let temp = tempfile::tempdir().expect("temp dir");
    let root: PathBuf = temp.path().canonicalize().expect("canonical root");
    for i in 0..8 {
        std::fs::write(
            root.join(format!("scan_fixture_{i}.sysml")),
            format!("package Scan{i} {{ part def Widget{i}; part w : Widget{i}; }}\n"),
        )
        .expect("write scan fixture");
    }
    let uri = url::Url::from_file_path(root.join("scan_fixture_0.sysml")).expect("fixture uri");
    let root_uri = url::Url::from_file_path(&root).expect("root uri");

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    send_message(
        &mut stdin,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "id": init_id,
            "method": "initialize",
            "params": {
                "processId": null,
                "rootUri": root_uri.as_str(),
                "capabilities": {},
                "clientInfo": { "name": "concurrent_startup_scan_test", "version": "0.1.0" }
            }
        })
        .to_string(),
    );
    let _ = read_message(&mut stdout).expect("initialize response");
    send_message(
        &mut stdin,
        &serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} }).to_string(),
    );

    // Fire a hover request immediately after `initialized` — no sleep, no barrier — while the
    // startup scan (discover -> parse -> ingest -> relink, all off the actor's mailbox) is
    // presumably still running in the background.
    let hover_id = next_id();
    let started = Instant::now();
    send_message(
        &mut stdin,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "id": hover_id,
            "method": "textDocument/hover",
            "params": {
                "textDocument": { "uri": uri.as_str() },
                "position": { "line": 0, "character": 8 }
            }
        })
        .to_string(),
    );
    let hover_resp = read_response(&mut stdout, hover_id).expect("hover response");
    let elapsed = started.elapsed();

    assert!(
        elapsed < NOT_BLOCKED_BOUND,
        "hover must not block behind a concurrent startup scan; took {elapsed:?}: {hover_resp}"
    );

    let _ = child.kill();
}

#[test]
fn superseded_relink_result_is_dropped_and_does_not_regress_diagnostics() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///superseded_relink.sysml";
    let broken = "package P {\n  part def Vehicle {\n    part engine : MissingEngineType;\n  }\n}\n";
    let fixed = "package P {\n  part def MissingEngineType;\n  part def Vehicle {\n    part engine : MissingEngineType;\n  }\n}\n";

    let init_id = next_id();
    send_message(
        &mut stdin,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "id": init_id,
            "method": "initialize",
            "params": {
                "processId": null,
                "rootUri": null,
                "capabilities": {},
                "clientInfo": { "name": "superseded_relink_test", "version": "0.1.0" }
            }
        })
        .to_string(),
    );
    let _ = read_message(&mut stdout).expect("initialize response");
    send_message(
        &mut stdin,
        &serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} }).to_string(),
    );

    // Open with the broken (unresolved-reference) content, then immediately edit to the fixed
    // content — both land before the first edit's 90ms relink debounce can fire, so the first
    // relink (still reflecting the broken content) must be superseded and its result dropped
    // when it eventually completes; only the second (fixed) edit's relink should ever publish.
    send_message(
        &mut stdin,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": broken }
            }
        })
        .to_string(),
    );
    send_message(
        &mut stdin,
        &serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didChange",
            "params": {
                "textDocument": { "uri": uri, "version": 2 },
                "contentChanges": [{ "text": fixed }]
            }
        })
        .to_string(),
    );

    // Barrier: drain messages until a response to this request arrives, collecting the most
    // recent publishDiagnostics for our URI along the way. Poll a few times with a short sleep
    // between attempts so the (now-superseded) first relink and the second (winning) relink
    // both have a chance to settle before we assert.
    let mut published: Vec<serde_json::Value> = Vec::new();
    for _ in 0..20 {
        std::thread::sleep(Duration::from_millis(50));
        let barrier_id = next_id();
        send_message(
            &mut stdin,
            &serde_json::json!({
                "jsonrpc": "2.0",
                "id": barrier_id,
                "method": "textDocument/hover",
                "params": {
                    "textDocument": { "uri": uri },
                    "position": { "line": 0, "character": 0 }
                }
            })
            .to_string(),
        );
        loop {
            let msg = read_message(&mut stdout).expect("expected message while waiting for barrier");
            let json: serde_json::Value = serde_json::from_str(&msg).unwrap_or_default();
            if json["method"].as_str() == Some("textDocument/publishDiagnostics")
                && json["params"]["uri"].as_str() == Some(uri)
            {
                published = json["params"]["diagnostics"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default();
            }
            if json["id"].as_i64() == Some(barrier_id) {
                break;
            }
        }
    }

    let unresolved: Vec<&serde_json::Value> = published
        .iter()
        .filter(|d| d["code"].as_str() == Some("unresolved_type_reference"))
        .collect();
    assert!(
        unresolved.is_empty(),
        "final diagnostics must reflect the fixed (superseding) edit, not a stale/superseded \
         relink result; got unresolved_type_reference diagnostics: {unresolved:#?}\nall diagnostics: {published:#?}"
    );

    let _ = child.kill();
}
