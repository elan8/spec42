//! Reliability and determinism gates for newly-added LSP handlers.

use super::harness::TestSession;

#[test]
fn lsp_new_handlers_survive_invalid_intermediate_text() {
    let mut session = TestSession::new();
    let uri = "file:///quality-invalid.sysml";
    session.initialize_default("quality_gates_test");
    session.did_open(uri, "package P { part def Engine; part v : Engine; }\n", 1);
    session.did_change_full(uri, "package P { part def Engine part v : ; }\n", 2);
    std::thread::sleep(std::time::Duration::from_millis(60));

    let probes = vec![
        (
            "textDocument/signatureHelp",
            serde_json::json!({"textDocument":{"uri":uri},"position":{"line":0,"character":25}}),
        ),
        (
            "textDocument/inlayHint",
            serde_json::json!({"textDocument":{"uri":uri},"range":{"start":{"line":0,"character":0},"end":{"line":0,"character":40}}}),
        ),
        (
            "textDocument/documentLink",
            serde_json::json!({"textDocument":{"uri":uri}}),
        ),
        (
            "textDocument/codeLens",
            serde_json::json!({"textDocument":{"uri":uri}}),
        ),
        (
            "textDocument/linkedEditingRange",
            serde_json::json!({"textDocument":{"uri":uri},"position":{"line":0,"character":20}}),
        ),
    ];

    for (method, params) in probes {
        let resp = session.request(method, params);
        assert!(
            resp.get("error").is_none(),
            "{} should not fail on invalid intermediate text: {}",
            method,
            resp
        );
    }
}

#[test]
fn lsp_code_lens_is_deterministic_for_same_document_state() {
    let mut session = TestSession::new();
    let uri = "file:///quality-deterministic.sysml";
    let content = "package P {\n  part def Engine;\n  part vehicle : Engine;\n}\n";
    session.initialize_default("quality_gates_test");
    session.did_open(uri, content, 1);
    std::thread::sleep(std::time::Duration::from_millis(60));

    let first = session.request(
        "textDocument/codeLens",
        serde_json::json!({"textDocument":{"uri":uri}}),
    );
    let second = session.request(
        "textDocument/codeLens",
        serde_json::json!({"textDocument":{"uri":uri}}),
    );
    assert_eq!(first["result"], second["result"], "code lens should be stable across repeated requests for unchanged document");
}
