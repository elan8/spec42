//! Editor notification versions and open intervals protect source updates.

use super::harness::TestSession;
use serde_json::json;
use std::time::{Duration, Instant};

fn assert_current_symbol(session: &mut TestSession, uri: &str, expected: &str) {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let response = session.request(
            "textDocument/documentSymbol",
            json!({
                "textDocument": {"uri": uri}
            }),
        );
        // A future background publication may explicitly defer a positional query. An old
        // successful answer is never accepted as a substitute for the current source.
        if response["error"]["code"] == -32801 {
            assert!(
                Instant::now() < deadline,
                "analysis did not settle: {response}"
            );
            std::thread::sleep(Duration::from_millis(2));
            continue;
        }
        assert!(response.get("error").is_none(), "{response}");
        let symbols = response["result"].as_array().expect("document symbols");
        let names: Vec<_> = symbols
            .iter()
            .map(|symbol| symbol["name"].as_str().unwrap())
            .collect();
        assert_eq!(names, [expected]);
        return;
    }
}

#[test]
fn editor_versions_ignore_stale_changes_advance_no_ops_and_reset_on_reopen() {
    let uri = "file:///c:/spec42-lsp-tests/editor-versions.sysml";
    let mut session = TestSession::new();
    session.initialize_default("editor-versions");
    session.did_open(uri, "part def Initial;", 1);
    session.did_change_full(uri, "part def Latest;", 4);
    assert_current_symbol(&mut session, uri, "Latest");

    session.did_change_full(uri, "part def Stale;", 3);
    assert_current_symbol(&mut session, uri, "Latest");
    session.did_change_full(uri, "part def Latest;", 8);
    session.did_change_full(uri, "part def StaleNoOp;", 7);
    assert_current_symbol(&mut session, uri, "Latest");

    session.notify("textDocument/didClose", json!({"textDocument":{"uri":uri}}));
    session.did_open(uri, "part def Reopened;", 1);
    assert_current_symbol(&mut session, uri, "Reopened");
    session.did_change_full(uri, "part def Current;", 2);
    assert_current_symbol(&mut session, uri, "Current");

    for (version, text) in [(5, "Edited"), (4, "Obsolete")] {
        session.notify(
            "textDocument/didChange",
            json!({
                "textDocument": {"uri": uri, "version": version},
                "contentChanges": [{
                    "range": {"start": {"line": 0, "character": 9},
                              "end": {"line": 0, "character": 16}},
                    "text": text
                }]
            }),
        );
    }
    assert_current_symbol(&mut session, uri, "Edited");
}

#[test]
fn an_incremental_burst_converges_to_its_final_versioned_diagnostics_and_current_status() {
    let uri = "file:///c:/spec42-lsp-tests/bounded-editor.sysml";
    let mut session = TestSession::new();
    session.initialize_default("bounded-editor");
    session.did_open(uri, "part def A;", 1);
    for version in 2..=80 {
        let name = if version % 2 == 0 { "B" } else { "A" };
        session.notify("textDocument/didChange", json!({
            "textDocument":{"uri":uri,"version":version},
            "contentChanges":[{"range":{"start":{"line":0,"character":9},"end":{"line":0,"character":10}},"text":name}]
        }));
    }
    // Observe the final notification itself. Older/unversioned diagnostics cannot satisfy it.
    let diagnostics = session.wait_for_diagnostics_version(uri, 80);
    assert!(diagnostics["diagnostics"].is_array());
    assert_current_symbol(&mut session, uri, "B");
    let stats = session.request("sysml/serverStats", json!({}));
    assert_eq!(stats["result"]["analysis"]["state"], "current");
    assert!(stats["result"]["analysis"]["activeJobs"].as_u64().unwrap() <= 1);
    assert!(stats["result"]["analysis"]["pendingJobs"].as_u64().unwrap() <= 1);
}
