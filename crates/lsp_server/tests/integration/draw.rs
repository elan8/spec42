use super::harness::TestSession;

fn sequence_product() -> serde_json::Value {
    serde_json::json!({
        "schemaVersion": 5,
        "modelDigest": "blake3:test-digest",
        "documents": [{ "uri": "file:///model.sysml" }],
        "sources": [{ "document": 0, "range": [0, 0, 0, 1] }],
        "references": [{ "kind": "qualified-name" }],
        "selectedView": { "reference": 0, "kind": "sequence-view", "name": "Interaction", "source": 0 },
        "completeness": { "status": "complete", "reasons": [] },
        "projection": {
            "kind": "sequence-view",
            "exposedRoots": [0],
            "nodes": [
                { "reference": 0, "metaclass": "OccurrenceDefinition", "notationRole": "usage", "name": "Interaction", "typing": { "status": "absent" }, "owner": null, "source": 0, "compartments": [] },
                { "reference": 0, "metaclass": "PartUsage", "notationRole": "usage", "name": "client", "typing": { "status": "absent" }, "owner": 0, "source": 0, "compartments": [] },
                { "reference": 0, "metaclass": "PartUsage", "notationRole": "usage", "name": "server", "typing": { "status": "absent" }, "owner": 0, "source": 0, "compartments": [] },
                { "reference": 0, "metaclass": "FlowUsage", "notationRole": "usage", "name": "request", "typing": { "status": "absent" }, "owner": 0, "source": 0, "compartments": [] }
            ],
            "relationships": [],
            "edges": [],
            "metadata": { "participants": [1, 2], "messages": [3] },
            "scene": {
                "kind": "sequence",
                "lifelines": [1, 2],
                "messages": [{
                    "node": 3,
                    "label": "request",
                    "source": { "status": "resolved", "lifeline": 1 },
                    "target": { "status": "resolved", "lifeline": 2 },
                    "order": { "status": "resolved", "value": 1 },
                    "provenance": "authored",
                    "navigation": 0
                }]
            }
        }
    })
}

fn draw_params(product: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "modelDigest": "blake3:test-digest",
        "viewHandle": "sequence/root",
        "presentationRevision": 3,
        "product": product,
        "width": 800.0,
        "height": 600.0,
        "colorScheme": "light",
        "disclosure": { "expandedNodeIds": [], "sectionStates": [] },
    })
}

#[test]
fn spec42_draw_returns_native_svg_and_echoes_identity() {
    let mut session = TestSession::new();
    session.initialize_default("draw-test");

    let response = session.request("spec42/draw", draw_params(sequence_product()));
    assert!(response.get("error").is_none(), "LSP response: {response}");
    let result = &response["result"];
    assert_eq!(result["modelDigest"], "blake3:test-digest");
    assert_eq!(result["viewHandle"], "sequence/root");
    assert_eq!(result["presentationRevision"], 3);
    assert_eq!(result["engine"], "native");
    let svg = result["svg"].as_str().unwrap_or_default();
    assert!(svg.contains("<svg"), "expected SVG markup: {svg}");
    assert!(svg.contains("sequence-lifeline"), "{svg}");
    assert!(svg.contains("client"), "{svg}");
}

#[test]
fn spec42_draw_declines_when_legacy_engine_is_requested() {
    let mut session =
        TestSession::new_with_env(&[("SPEC42_LAYOUT_ENGINE", std::path::Path::new("legacy"))]);
    session.initialize_default("draw-legacy-test");

    let response = session.request("spec42/draw", draw_params(sequence_product()));
    let error = response.get("error").unwrap_or_else(|| {
        panic!("SPEC42_LAYOUT_ENGINE=legacy must decline spec42/draw: {response}")
    });
    let message = error["message"].as_str().unwrap_or_default();
    assert!(
        message.contains("legacy") && message.contains("no client drawing fallback"),
        "decline message should explain why: {response}"
    );
}

#[test]
fn spec42_draw_rejects_catalog_views() {
    let mut session = TestSession::new();
    session.initialize_default("draw-catalog-test");
    let mut product = sequence_product();
    product["selectedView"]["kind"] = serde_json::json!("browser-view");
    product["projection"]["kind"] = serde_json::json!("browser-view");

    let response = session.request("spec42/draw", draw_params(product));
    let error = response
        .get("error")
        .unwrap_or_else(|| panic!("browser-view must fail spec42/draw: {response}"));
    let message = error["message"].as_str().unwrap_or_default();
    assert!(
        message.contains("browser-view") || message.contains("Unsupported"),
        "error should name the unsupported catalog view: {response}"
    );
}
