//! Experimental capability-surface integration tests.

use super::harness::TestSession;

#[test]
fn lsp_initialize_advertises_remaining_feature_capabilities() {
    let mut session = TestSession::new();
    let init_json = session.request(
        "initialize",
        serde_json::json!({"processId": null, "rootUri": null, "capabilities": {}}),
    );
    let caps = &init_json["result"]["capabilities"];
    assert!(caps["signatureHelpProvider"].is_object());
    assert!(caps["selectionRangeProvider"].as_bool().unwrap_or(false));
    assert!(caps["documentLinkProvider"].is_object());
    assert!(caps["codeLensProvider"].is_object());
    assert!(caps["inlayHintProvider"].as_bool().unwrap_or(false));
    assert!(caps["linkedEditingRangeProvider"]
        .as_bool()
        .unwrap_or(false));
    assert!(caps["monikerProvider"].as_bool().unwrap_or(false));
    assert!(caps["callHierarchyProvider"].as_bool().unwrap_or(false));
    assert_eq!(
        caps["experimental"]["typeHierarchyProvider"].as_bool(),
        Some(true),
        "type hierarchy capability is advertised through experimental surface for current tower-lsp compatibility"
    );
}
