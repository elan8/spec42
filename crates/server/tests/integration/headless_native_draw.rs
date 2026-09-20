//! spec42 #176: native headless SVG is prepare + elkrs layout + diagram_draw, with no QuickJS.

use spec42::headless_renderer::render_native_svg;

fn schema5_sequence_payload() -> String {
    serde_json::json!({
        "schemaVersion": 5,
        "documents": [{ "uri": "file:///model.sysml" }],
        "sources": [{ "document": 0, "range": [0, 0, 0, 1] }],
        "references": [{ "kind": "qualified-name" }],
        "selectedView": { "reference": 0, "kind": "sequence-view", "name": "Interaction", "source": 0 },
        "completeness": { "status": "complete", "reasons": [] },
        "projection": {
            "kind": "sequence-view",
            "exposedRoots": [0],
            "nodes": [
                {
                    "reference": 0,
                    "metaclass": "OccurrenceDefinition",
                    "notationRole": "usage",
                    "name": "Interaction",
                    "typing": { "status": "absent" },
                    "owner": null,
                    "source": 0,
                    "compartments": []
                },
                {
                    "reference": 0,
                    "metaclass": "PartUsage",
                    "notationRole": "usage",
                    "name": "client",
                    "typing": { "status": "absent" },
                    "owner": 0,
                    "source": 0,
                    "compartments": []
                },
                {
                    "reference": 0,
                    "metaclass": "PartUsage",
                    "notationRole": "usage",
                    "name": "server",
                    "typing": { "status": "absent" },
                    "owner": 0,
                    "source": 0,
                    "compartments": []
                },
                {
                    "reference": 0,
                    "metaclass": "FlowUsage",
                    "notationRole": "usage",
                    "name": "request",
                    "typing": { "status": "absent" },
                    "owner": 0,
                    "source": 0,
                    "compartments": []
                }
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
    .to_string()
}

#[test]
fn native_sequence_svg_has_lifelines_and_the_authored_message() {
    let svg = render_native_svg(&schema5_sequence_payload())
        .unwrap_or_else(|err| panic!("native sequence export failed: {err}"));
    assert!(svg.contains("sequence-lifeline"), "sequence SVG:\n{svg}");
    assert!(svg.contains("client"), "sequence SVG:\n{svg}");
    assert!(svg.contains("server"), "sequence SVG:\n{svg}");
    assert!(svg.contains("request"), "sequence SVG:\n{svg}");
}

#[test]
fn unsupported_views_fail_without_falling_back_to_quickjs() {
    let err = render_native_svg(r#"{"view":"browser-view","title":"x"}"#)
        .expect_err("browser-view is still TS-only");
    assert!(
        err.contains("browser-view"),
        "unsupported-view error should name the view: {err}"
    );
}
