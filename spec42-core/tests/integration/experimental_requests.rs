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

    let custom_method = session.request(
        "sysml/model",
        serde_json::json!({
            "uri": uri,
            "includeGraph": true,
            "includeIbd": false
        }),
    );
    assert!(
        custom_method.get("error").is_none(),
        "core sysml/model custom method should remain available: {}",
        custom_method
    );

    let prepare_vehicle = session.request(
        "textDocument/prepareTypeHierarchy",
        serde_json::json!({"textDocument":{"uri":uri},"position":{"line":2,"character":7}}),
    );
    let vehicle_item = prepare_vehicle["result"]
        .as_array()
        .and_then(|a| a.first())
        .cloned()
        .expect("prepareTypeHierarchy should return item for vehicle");
    let supertypes = session.request(
        "typeHierarchy/supertypes",
        serde_json::json!({"item": vehicle_item}),
    );
    let super_items = supertypes["result"]
        .as_array()
        .expect("supertypes should return array");
    assert!(
        super_items
            .iter()
            .any(|it| it["name"].as_str() == Some("Engine")),
        "vehicle supertypes should include Engine: {}",
        supertypes
    );

    let prepare_engine = session.request(
        "textDocument/prepareTypeHierarchy",
        serde_json::json!({"textDocument":{"uri":uri},"position":{"line":1,"character":11}}),
    );
    let engine_item = prepare_engine["result"]
        .as_array()
        .and_then(|a| a.first())
        .cloned()
        .expect("prepareTypeHierarchy should return item for Engine");
    let subtypes = session.request("typeHierarchy/subtypes", serde_json::json!({"item": engine_item}));
    let sub_items = subtypes["result"]
        .as_array()
        .expect("subtypes should return array");
    assert!(
        sub_items
            .iter()
            .any(|it| it["name"].as_str() == Some("vehicle")),
        "Engine subtypes should include vehicle: {}",
        subtypes
    );
}
