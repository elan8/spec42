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
fn webshop_action_flow_draws_actions_and_successions() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_webshop_action_flow.md"
    ));
    let input = draw_input_from_payload(&payload).expect("typed action flow lays out");
    let prepared = &input["prepared"];
    let nodes = prepared["nodes"].as_array().expect("action nodes");
    let edges = prepared["edges"].as_array().expect("action edges");
    assert_eq!(
        nodes.len(),
        6,
        "the action definition and its action usages are vertices"
    );
    assert_eq!(edges.len(), 4, "only authored successions connect actions");
    assert!(edges
        .iter()
        .all(|edge| edge["attributes"]["succession"] == true));
    let svg = render_svg_from_payload(&payload, 1280.0, 900.0).expect("draw action flow");
    assert_eq!(
        svg.matches("class=\"activity-action action-flow-node")
            .count(),
        6
    );
    // `CheckoutPipeline` is the container holding the flow of its five actions.
    assert_eq!(svg.matches("action-flow-container\"").count(), 1);
    assert_eq!(svg.matches("data-flow-kind=\"succession\"").count(), 4);
    assert_eq!(svg.matches("class=\"action-stereotype\"").count(), 6);
    assert_eq!(svg.matches("«action»").count(), 5);
    assert!(svg.contains("«action def»"));
    assert_eq!(svg.matches("stroke-dasharray: 7,4").count(), 4);
    assert_eq!(
        svg.matches("marker-end: url(#action-succession-arrow)")
            .count(),
        4
    );
    let view_box = svg
        .split_once("viewBox=\"")
        .unwrap()
        .1
        .split_once('"')
        .unwrap()
        .0
        .split_whitespace()
        .map(|number| number.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    let aspect = view_box[2] / view_box[3];
    assert!(
        (0.9..=2.2).contains(&aspect),
        "action flow aspect: {aspect}"
    );
}

#[test]
fn composite_action_is_a_container_holding_its_own_flow() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_composite_action_flow.md"
    ));
    let input = draw_input_from_payload(&payload).expect("typed action flow lays out");
    let prepared = &input["prepared"];
    let label_of = |id: &serde_json::Value| {
        prepared["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|node| &node["id"] == id)
            .map(|node| node["label"].as_str().unwrap().to_string())
            .unwrap()
    };
    let mut containment: Vec<(String, String)> = prepared["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|node| {
            let container = &node["attributes"]["containerId"];
            let container = if container.is_null() {
                "-".to_string()
            } else {
                label_of(container)
            };
            (node["label"].as_str().unwrap().to_string(), container)
        })
        .collect();
    containment.sort();
    assert_eq!(
        containment,
        [
            ("Fulfil", "-"),
            ("order", "Fulfil"),
            ("pack", "order"),
            ("pick", "order"),
            ("ship", "Fulfil"),
        ]
        .map(|(node, container)| (node.to_string(), container.to_string()))
    );
    let mut successions: Vec<(String, String)> = prepared["edges"]
        .as_array()
        .unwrap()
        .iter()
        .map(|edge| (label_of(&edge["source"]), label_of(&edge["target"])))
        .collect();
    successions.sort();
    assert_eq!(
        successions,
        [("order", "ship"), ("pick", "pack")]
            .map(|(source, target)| (source.to_string(), target.to_string()))
    );

    // Every action lies inside the action that owns it, below that action's name compartment.
    let positions = &input["behaviorLayout"]["positions"];
    let id_of = |label: &str| {
        prepared["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|node| node["label"] == label)
            .map(|node| node["id"].as_str().unwrap().to_string())
            .unwrap()
    };
    let rect = |label: &str| {
        let rect = &positions[id_of(label)];
        let [x, y, w, h] = ["x", "y", "width", "height"].map(|key| rect[key].as_f64().unwrap());
        (x, y, x + w, y + h)
    };
    for (inner, outer) in [
        ("order", "Fulfil"),
        ("ship", "Fulfil"),
        ("pick", "order"),
        ("pack", "order"),
    ] {
        let (inner, outer) = (rect(inner), rect(outer));
        assert!(
            inner.0 > outer.0 && inner.1 > outer.1 + 40.0 && inner.2 < outer.2 && inner.3 < outer.3,
            "{inner:?} is not inside {outer:?}"
        );
    }
    let (pick, pack, ship) = (rect("pick"), rect("pack"), rect("ship"));
    assert!(pick.2 <= pack.0 || pack.2 <= pick.0 || pick.3 <= pack.1 || pack.3 <= pick.1);
    let order = rect("order");
    assert!(order.2 <= ship.0 || ship.2 <= order.0 || order.3 <= ship.1 || ship.3 <= order.1);

    let svg = render_svg_from_payload(&payload, 1280.0, 900.0).expect("draw action flow");
    assert_eq!(svg.matches("action-flow-container\"").count(), 2);
    assert_eq!(svg.matches("data-flow-kind=\"succession\"").count(), 2);
    assert!(svg.contains("«action def»"));
    assert_eq!(svg.matches("«action»").count(), 4);
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
    let port_types = input["edges"]
        .as_array()
        .expect("prepared connectors")
        .iter()
        .map(|edge| edge["attributes"]["portTypeIdentity"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(port_types.len(), 3);
    assert_eq!(
        port_types
            .iter()
            .filter(|key| key.ends_with("PowerPort"))
            .count(),
        2
    );
    assert_eq!(
        port_types
            .iter()
            .filter(|key| key.ends_with("VideoPort"))
            .count(),
        1
    );
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
    assert_eq!(svg.matches("data-port-type=").count(), 3);
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

fn shared_port_interconnection_payload() -> serde_json::Value {
    json!({
        "version": 1,
        "view": "interconnection-view",
        "selectedViewName": "Shared ports",
        "interconnectionScene": {
            "schemaVersion": 3,
            "view": { "id": "v", "name": "Shared ports", "type": "InterconnectionView", "rootIds": ["a", "b", "c"] },
            "nodes": [
                { "id": "a", "name": "a", "kind": "part", "qualifiedName": "a", "semanticId": "a", "definitionId": "Box", "typeName": "Box" },
                { "id": "b", "name": "b", "kind": "part", "qualifiedName": "b", "semanticId": "b", "definitionId": "Box", "typeName": "Box" },
                { "id": "c", "name": "c", "kind": "part", "qualifiedName": "c", "semanticId": "c", "definitionId": "Box", "typeName": "Box" }
            ],
            "ports": [
                { "id": "a.p", "ownerNodeId": "a", "name": "p", "direction": "inout", "typeName": "Data", "sideHint": "east" },
                { "id": "b.p", "ownerNodeId": "b", "name": "p", "direction": "inout", "typeName": "Data", "sideHint": "west" },
                { "id": "c.p", "ownerNodeId": "c", "name": "p", "direction": "inout", "typeName": "Data", "sideHint": "west" }
            ],
            "edges": [
                { "id": "a-b", "sourceNodeId": "a", "targetNodeId": "b", "sourcePortId": "a.p", "targetPortId": "b.p", "kind": "connection", "label": "", "semanticId": "a-b" },
                { "id": "a-c", "sourceNodeId": "a", "targetNodeId": "c", "sourcePortId": "a.p", "targetPortId": "c.p", "kind": "connection", "label": "namedUntyped", "semanticId": "a-c" },
                { "id": "b-c", "sourceNodeId": "b", "targetNodeId": "c", "sourcePortId": "b.p", "targetPortId": "c.p", "kind": "interface", "label": "DataLink", "semanticId": "b-c" }
            ],
            "containers": [],
            "diagnostics": []
        }
    })
}

fn collinear_overlap(
    a: (&serde_json::Value, &serde_json::Value),
    b: (&serde_json::Value, &serde_json::Value),
) -> f64 {
    let point =
        |value: &serde_json::Value| (value["x"].as_f64().unwrap(), value["y"].as_f64().unwrap());
    let (a0, a1, b0, b1) = (point(a.0), point(a.1), point(b.0), point(b.1));
    if a0.1 == a1.1 && b0.1 == b1.1 && a0.1 == b0.1 {
        return a0.0.max(a1.0).min(b0.0.max(b1.0)) - a0.0.min(a1.0).max(b0.0.min(b1.0));
    }
    if a0.0 == a1.0 && b0.0 == b1.0 && a0.0 == b0.0 {
        return a0.1.max(a1.1).min(b0.1.max(b1.1)) - a0.1.min(a1.1).max(b0.1.min(b1.1));
    }
    0.0
}

#[test]
fn coincident_hyperedge_routes_render_as_individual_lanes() {
    let input = draw_input_from_payload(&shared_port_interconnection_payload())
        .expect("shared-port interconnection lays out");
    let edges = input["interconnectionLayout"]["edges"]
        .as_array()
        .expect("laid out connectors");
    assert_eq!(edges.len(), 3);

    for (left_index, left) in edges.iter().enumerate() {
        let left_points = left["routePoints"].as_array().expect("left route");
        for right in edges.iter().skip(left_index + 1) {
            let right_points = right["routePoints"].as_array().expect("right route");
            for left_pair in left_points.windows(2) {
                for right_pair in right_points.windows(2) {
                    let overlap = collinear_overlap(
                        (&left_pair[0], &left_pair[1]),
                        (&right_pair[0], &right_pair[1]),
                    );
                    assert!(
                        overlap <= 0.001,
                        "connectors {} and {} share a visible route segment ({overlap}): {left_points:?} / {right_points:?}",
                        left["id"],
                        right["id"]
                    );
                }
            }
        }
    }
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
