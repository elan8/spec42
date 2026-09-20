//! End-to-end native pipeline: visualization payload → prepare → layout → SVG.
//! Sequence needs no ELK; general/interconnection/behavior go through `diagram_layout`.

use diagram_draw::{draw_input_from_payload, render_svg_from_payload, DrawError, PipelineError};
use serde_json::json;

fn general_golden_payload() -> serde_json::Value {
    json!({
        "version": 1,
        "view": "general-view",
        "selectedViewName": "General",
        "graph": {
            "nodes": [
                { "id": "P::Vehicle", "name": "Vehicle", "type": "part def", "attributes": { "attributes": ["mass"] } },
                { "id": "P::vehicle", "name": "vehicle", "type": "part", "attributes": { "partType": "Vehicle" } }
            ],
            "edges": [{ "id": "typed", "source": "P::vehicle", "target": "P::Vehicle", "type": "typing", "name": "typing" }]
        }
    })
}

fn interconnection_golden_payload() -> serde_json::Value {
    json!({
        "version": 1,
        "view": "interconnection-view",
        "selectedViewName": "Connections",
        "interconnectionScene": {
            "schemaVersion": 3,
            "view": { "id": "v", "name": "Connections", "type": "InterconnectionView", "rootIds": ["a", "b"] },
            "nodes": [
                { "id": "a", "name": "a", "kind": "part", "qualifiedName": "a", "semanticId": "a", "definitionId": "A", "typeName": "A" },
                { "id": "b", "name": "b", "kind": "part", "qualifiedName": "b", "semanticId": "b", "definitionId": "B", "typeName": "B" }
            ],
            "ports": [
                { "id": "a.p", "ownerNodeId": "a", "name": "p", "direction": "out", "typeName": "Power", "sideHint": "east" },
                { "id": "b.p", "ownerNodeId": "b", "name": "p", "direction": "in", "typeName": "Power", "sideHint": "west" }
            ],
            "edges": [
                { "id": "e", "sourceNodeId": "a", "targetNodeId": "b", "sourcePortId": "a.p", "targetPortId": "b.p", "kind": "flow", "label": "flow", "semanticId": "e" }
            ],
            "containers": [],
            "diagnostics": []
        }
    })
}

fn schema5_sequence_payload() -> serde_json::Value {
    json!({
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

#[test]
fn sequence_schema5_draw_input_carries_the_scene_lifelines() {
    let input = draw_input_from_payload(&schema5_sequence_payload()).unwrap();
    assert_eq!(input["view"], "sequence-view");
    assert_eq!(
        input["meta"]["sequenceDiagram"]["lifelines"][0]["name"],
        "client"
    );
    assert_eq!(
        input["meta"]["sequenceDiagram"]["messages"][0]["name"],
        "request"
    );
}

#[test]
fn sequence_schema5_svg_has_lifelines_and_message() {
    let svg = render_svg_from_payload(&schema5_sequence_payload(), 1280.0, 900.0).unwrap();
    assert!(svg.contains("sequence-lifeline"), "{svg}");
    assert!(svg.contains("client"), "{svg}");
    assert!(svg.contains("request"), "{svg}");
}

#[test]
fn general_golden_payload_lays_out_and_draws_both_nodes() {
    let svg = render_svg_from_payload(&general_golden_payload(), 1280.0, 900.0)
        .unwrap_or_else(|err| panic!("{err}"));
    assert!(svg.contains("Vehicle"), "{svg}");
    assert!(svg.contains("vehicle"), "{svg}");
    assert!(svg.contains("sysml-node"), "{svg}");
}

#[test]
fn interconnection_golden_payload_lays_out_ports_and_a_flow() {
    let svg = render_svg_from_payload(&interconnection_golden_payload(), 1280.0, 900.0)
        .unwrap_or_else(|err| panic!("{err}"));
    assert!(
        svg.contains("ibd-node") || svg.contains("sysml-node"),
        "{svg}"
    );
    assert!(svg.contains(">a<") || svg.contains("a</"), "{svg}");
}

#[test]
fn action_flow_payload_lays_out_through_elkrs_and_draws() {
    let payload = json!({
        "view": "action-flow-view",
        "selectedViewName": "Fulfillment",
        "activityDiagrams": [{
            "name": "Fulfillment",
            "nodes": [
                { "id": "start", "name": "start", "kind": "initial" },
                { "id": "pack", "name": "Pack Order", "kind": "action" }
            ],
            "flows": [{ "id": "f1", "from": "start", "to": "pack" }]
        }]
    });
    let svg =
        render_svg_from_payload(&payload, 1280.0, 900.0).unwrap_or_else(|err| panic!("{err}"));
    assert!(svg.contains("Pack Order"), "{svg}");
    assert!(
        svg.contains("action-flow") || svg.contains("viz-root"),
        "{svg}"
    );
}

#[test]
fn browser_view_is_rejected() {
    let err = render_svg_from_payload(
        &json!({ "view": "browser-view", "title": "x" }),
        1280.0,
        900.0,
    )
    .expect_err("browser-view is TS-only");
    match err {
        PipelineError::Draw(DrawError::UnsupportedView(view)) => {
            assert_eq!(view, "browser-view");
        }
        other => panic!("expected unsupported view, got {other}"),
    }
}
