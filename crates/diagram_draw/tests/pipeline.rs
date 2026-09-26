//! End-to-end native pipeline: visualization payload → prepare → layout → SVG.
//! Sequence needs no ELK; general/interconnection/behavior go through `diagram_layout`.

use diagram_draw::{
    draw_input_from_payload, render_svg_from_payload, render_svg_from_payload_with_options,
    PipelineError,
};
use serde_json::json;

fn generated_diagram(snapshot: &str) -> serde_json::Value {
    let json = snapshot
        .split_once("## diagram.json\n~~~json\n")
        .or_else(|| snapshot.split_once("## diagram.json\r\n~~~json\r\n"))
        .expect("snapshot contains diagram.json")
        .1;
    let json = json.split_once("\n~~~").expect("closed JSON fence").0;
    serde_json::from_str(json).expect("valid generated diagram")
}

#[test]
fn webshop_action_flow_draws_only_actions_and_successions() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_webshop_action_flow.md"
    ));
    let input = draw_input_from_payload(&payload).expect("typed action flow lays out");
    let prepared = &input["prepared"];
    let nodes = prepared["nodes"].as_array().expect("action nodes");
    let edges = prepared["edges"].as_array().expect("action edges");
    assert_eq!(nodes.len(), 5, "only nested action usages are vertices");
    assert_eq!(edges.len(), 4, "only authored successions connect actions");
    assert!(edges
        .iter()
        .all(|edge| edge["attributes"]["succession"] == true));
    let svg = render_svg_from_payload(&payload, 1280.0, 900.0).expect("draw action flow");
    assert_eq!(
        svg.matches("class=\"activity-action action-flow-node")
            .count(),
        5
    );
    assert_eq!(svg.matches("data-flow-kind=\"succession\"").count(), 4);
    assert_eq!(svg.matches("class=\"action-stereotype\"").count(), 5);
    assert!(svg.contains("«action»"));
    let background = svg.split_once("class=\"viz-bg\"").unwrap().1;
    let background_height: f64 = background
        .split_once("height=\"")
        .unwrap()
        .1
        .split_once('"')
        .unwrap()
        .0
        .parse()
        .unwrap();
    assert!(
        background_height > 1000.0,
        "background must cover the full action chain"
    );
}

#[test]
fn action_flow_rejects_out_of_range_member_index() {
    let mut payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_webshop_action_flow.md"
    ));
    payload["projection"]["metadata"]["actions"] = json!([999]);
    let error = draw_input_from_payload(&payload).expect_err("invalid member index must fail");
    assert!(matches!(error, PipelineError::InvalidPayload(_)));
}

#[test]
fn unconnected_second_monitor_preserves_first_monitor_connections() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_office_two_monitors.md"
    ));
    let input = draw_input_from_payload(&payload).expect("typed interconnection lays out");
    let edges = input["interconnectionLayout"]["edges"]
        .as_array()
        .expect("laid out connectors");
    assert_eq!(edges.len(), 3);
    assert!(edges.iter().all(|edge| edge["routePoints"]
        .as_array()
        .is_some_and(|points| points.len() >= 2)));
    for edge in edges {
        let points = edge["routePoints"].as_array().expect("connector route");
        let first = points.first().expect("source point");
        let last = points.last().expect("target point");
        let dx = (first["x"].as_f64().unwrap() - last["x"].as_f64().unwrap()).abs();
        let dy = (first["y"].as_f64().unwrap() - last["y"].as_f64().unwrap()).abs();
        assert!(
            dx + dy >= 80.0,
            "connector has no readable clearance: {edge}"
        );
    }
    let svg = render_svg_from_payload(&payload, 1280.0, 900.0).expect("draw interconnection");
    assert_eq!(svg.matches("class=\"ibd-connector").count(), 3);
    assert!(svg.contains("data-view-name=\"Workplace\""));
    assert!(!svg.contains("marker-start: url(#ibd-connection-dot)"));
    assert!(!svg.contains("viz-edge-label--connection"));
}

#[test]
fn timer_interconnection_example_retains_its_connectors() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_timer_interconnection.md"
    ));
    let input = draw_input_from_payload(&payload).expect("timer interconnection lays out");
    let projected_connectors = payload["projection"]["edges"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|edge| edge["kind"] == "connector")
        .count();
    let routed = input["interconnectionLayout"]["edges"]
        .as_array()
        .expect("routed connectors");
    assert_eq!(routed.len(), projected_connectors);
    assert!(routed.iter().all(|edge| edge["routePoints"]
        .as_array()
        .is_some_and(|points| points.len() >= 2)));
    let svg = render_svg_from_payload(&payload, 1280.0, 900.0).expect("draw timer");
    let background = svg.split_once("class=\"viz-bg\"").unwrap().1;
    let background_width: f64 = background
        .split_once("width=\"")
        .unwrap()
        .1
        .split_once('"')
        .unwrap()
        .0
        .parse()
        .unwrap();
    assert!(background_width > 1280.0, "wide diagram needs full canvas");
}

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
fn browser_view_renders_hierarchy_and_honours_collapse() {
    let payload = json!({
        "view": "browser-view",
        "selectedViewName": "Structure",
        "projectionHints": { "browserLayout": "hierarchy", "treeRoots": ["root"] },
        "generalViewGraph": {
            "nodes": [
                { "id": "root", "name": "Root", "type": "part def", "parentId": "" },
                { "id": "child", "name": "Child", "type": "part", "parentId": "root" }
            ],
            "edges": []
        }
    });
    let svg =
        render_svg_from_payload(&payload, 1280.0, 900.0).unwrap_or_else(|err| panic!("{err}"));
    assert!(svg.contains("browser-row"), "{svg}");
    assert!(svg.contains("Root"), "{svg}");
    assert!(svg.contains("Child"), "{svg}");
    assert!(svg.contains("▾"), "{svg}");

    let collapsed = render_svg_from_payload_with_options(
        &payload,
        &diagram_draw::theme::LIGHT,
        1280.0,
        900.0,
        Some(&diagram_draw::DisclosureState {
            expanded_node_ids: vec!["root".into()],
            section_states: vec![],
        }),
    )
    .unwrap_or_else(|err| panic!("{err}"));
    assert!(collapsed.contains("Root"), "{collapsed}");
    assert!(
        !collapsed.contains(">Child<"),
        "collapsed parent should hide the child row: {collapsed}"
    );
    assert!(collapsed.contains("▸"), "{collapsed}");
}

#[test]
fn grid_and_geometry_views_render() {
    let grid = render_svg_from_payload(
        &json!({
            "view": "grid-view",
            "selectedViewName": "Parts",
            "generalViewGraph": {
                "nodes": [{ "id": "robot", "name": "robot", "type": "part", "attributes": { "parts": ["arm"] } }],
                "edges": []
            }
        }),
        1280.0,
        900.0,
    )
    .unwrap_or_else(|err| panic!("{err}"));
    assert!(grid.contains("grid-cell"), "{grid}");
    assert!(grid.contains("robot"), "{grid}");

    let geometry = render_svg_from_payload(
        &json!({
            "view": "geometry-view",
            "selectedViewName": "Shape",
            "generalViewGraph": {
                "nodes": [{ "id": "body", "name": "body", "type": "part" }],
                "edges": []
            }
        }),
        1280.0,
        900.0,
    )
    .unwrap_or_else(|err| panic!("{err}"));
    assert!(geometry.contains("geometry-object"), "{geometry}");
    assert!(geometry.contains("provisional-view-badge"), "{geometry}");
    assert!(geometry.contains("2d orthographic preview"), "{geometry}");
}

fn schema5_state_transition_payload(
    vertices: Vec<serde_json::Value>,
    transitions: Vec<serde_json::Value>,
) -> serde_json::Value {
    json!({
        "schemaVersion": 5,
        "documents": [{ "uri": "file:///model.sysml" }],
        "sources": [{ "document": 0, "range": [0, 0, 0, 1] }],
        "references": [{ "kind": "qualified-name" }],
        "selectedView": { "reference": 0, "kind": "state-transition-view", "name": "States", "source": 0 },
        "completeness": { "status": "complete", "reasons": [] },
        "projection": {
            "kind": "state-transition-view",
            "exposedRoots": [],
            "nodes": [],
            "edges": [],
            "scene": {
                "kind": "state-transition",
                "frame": { "label": "States" },
                "vertices": vertices,
                "transitions": transitions
            }
        }
    })
}

#[test]
fn schema5_state_transition_out_of_range_endpoint_is_an_error() {
    let err = render_svg_from_payload(
        &schema5_state_transition_payload(
            vec![],
            vec![json!({ "source": 0, "target": 0, "label": "go" })],
        ),
        1280.0,
        900.0,
    )
    .expect_err("out-of-range scene indices must not panic");
    match err {
        PipelineError::InvalidPayload(message) => {
            assert!(
                message.contains("out of range"),
                "expected an index error, got {message}"
            );
        }
        other => panic!("expected invalid payload, got {other}"),
    }
}

#[test]
fn schema5_state_transition_missing_endpoint_is_an_error() {
    let err = render_svg_from_payload(
        &schema5_state_transition_payload(
            vec![json!({ "id": "idle", "label": "idle", "kind": "state" })],
            vec![json!({ "target": 0, "label": "go" })],
        ),
        1280.0,
        900.0,
    )
    .expect_err("missing source must not default to vertex 0");
    match err {
        PipelineError::InvalidPayload(message) => {
            assert!(
                message.contains("source") && message.contains("vertex index"),
                "expected a missing-source error, got {message}"
            );
        }
        other => panic!("expected invalid payload, got {other}"),
    }
}

#[test]
fn schema5_state_transition_non_numeric_endpoint_is_an_error() {
    let err = render_svg_from_payload(
        &schema5_state_transition_payload(
            vec![json!({ "id": "idle", "label": "idle", "kind": "state" })],
            vec![json!({ "source": "idle", "target": 0, "label": "go" })],
        ),
        1280.0,
        900.0,
    )
    .expect_err("non-numeric source must not default to vertex 0");
    match err {
        PipelineError::InvalidPayload(message) => {
            assert!(
                message.contains("source") && message.contains("vertex index"),
                "expected a malformed-source error, got {message}"
            );
        }
        other => panic!("expected invalid payload, got {other}"),
    }
}

#[test]
fn general_view_disclosure_hides_nested_nodes_until_expanded() {
    use diagram_draw::theme::LIGHT;
    use diagram_draw::{render_svg_from_payload_with_options, DisclosureState};

    let payload = json!({
        "schemaVersion": 5,
        "documents": [{ "uri": "file:///model.sysml" }],
        "sources": [{ "document": 0, "range": [0, 0, 0, 1] }],
        "references": [{ "kind": "qualified-name" }],
        "selectedView": { "reference": 0, "kind": "general-view", "name": "Nested", "source": 0 },
        "completeness": { "status": "complete", "reasons": [] },
        "projection": {
            "kind": "general-view",
            "exposedRoots": [0],
            "nodes": [
                { "reference": 0, "metaclass": "PartUsage", "notationRole": "usage", "name": "root", "typing": { "status": "absent" }, "owner": null, "source": 0, "compartments": [] },
                { "reference": 0, "metaclass": "PartUsage", "notationRole": "usage", "name": "child", "typing": { "status": "absent" }, "owner": 0, "source": 0, "compartments": [] }
            ],
            "relationships": [],
            "edges": [],
            "metadata": {},
            "scene": { "kind": "general" }
        }
    });
    let collapsed = render_svg_from_payload_with_options(
        &payload,
        &LIGHT,
        800.0,
        600.0,
        Some(&DisclosureState::default()),
    )
    .expect("collapsed general view");
    assert!(collapsed.contains("data-node-id=\"n:0\""), "{collapsed}");
    assert!(
        !collapsed.contains("data-node-id=\"n:1\""),
        "nested child must stay hidden until expanded: {collapsed}"
    );

    let expanded = render_svg_from_payload_with_options(
        &payload,
        &LIGHT,
        800.0,
        600.0,
        Some(&DisclosureState {
            expanded_node_ids: vec!["n:0".into()],
            section_states: vec![],
        }),
    )
    .expect("expanded general view");
    assert!(expanded.contains("data-node-id=\"n:0\""), "{expanded}");
    assert!(
        expanded.contains("data-node-id=\"n:1\""),
        "expanded owner must reveal the nested child: {expanded}"
    );
}
