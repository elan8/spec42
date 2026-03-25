//! Experimental request-surface integration tests.

use super::harness::TestSession;

#[test]
fn lsp_remaining_feature_requests_round_trip() {
    let mut session = TestSession::new();
    let uri = "file:///remaining-lsp.sysml";
    let content = "package P {\n  part def Engine;\n  part vehicle : Engine;\n}\n";
    session.initialize_default("remaining_lsp_features_test");
    session.did_open(uri, content, 1);
    std::thread::sleep(std::time::Duration::from_millis(60));

    let requests = vec![
        (
            "textDocument/signatureHelp",
            serde_json::json!({"textDocument":{"uri":uri},"position":{"line":2,"character":16}}),
        ),
        (
            "textDocument/selectionRange",
            serde_json::json!({"textDocument":{"uri":uri},"positions":[{"line":2,"character":7}]}),
        ),
        (
            "textDocument/inlayHint",
            serde_json::json!({"textDocument":{"uri":uri},"range":{"start":{"line":0,"character":0},"end":{"line":3,"character":0}}}),
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
            serde_json::json!({"textDocument":{"uri":uri},"position":{"line":2,"character":7}}),
        ),
        (
            "textDocument/moniker",
            serde_json::json!({"textDocument":{"uri":uri},"position":{"line":2,"character":7}}),
        ),
        (
            "textDocument/prepareTypeHierarchy",
            serde_json::json!({"textDocument":{"uri":uri},"position":{"line":2,"character":7}}),
        ),
        (
            "textDocument/prepareCallHierarchy",
            serde_json::json!({"textDocument":{"uri":uri},"position":{"line":2,"character":7}}),
        ),
    ];

    for (method, params) in requests {
        let json = session.request(method, params);
        assert!(
            json.get("error").is_none(),
            "request {} returned error: {}",
            method,
            json
        );
    }
}
