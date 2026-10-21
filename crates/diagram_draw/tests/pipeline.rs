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
fn a_filtered_empty_view_explains_the_selection_in_the_svg() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_filtered_empty.md"
    ));
    let svg = render_svg_from_payload(&payload, 800.0, 600.0).expect("valid empty view");
    assert!(svg.contains("No elements to display."));
    assert!(
        svg.contains("Check the view&apos;s exposes and filters.")
            || svg.contains("Check the view's exposes and filters.")
    );
    assert!(svg.contains("viz-empty-state"));
    // Empty-state presentation must not hide validation errors.
    assert!(diagram_draw::render_svg_from_json(
        &json!({ "view": "unknown-view", "nodes": [], "edges": [] }),
        800.0,
        600.0,
    )
    .is_err());
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
fn connectors_are_labelled_by_authored_name_else_type() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_connector_labels.md"
    ));
    let svg = render_svg_from_payload(&payload, 1280.0, 900.0).expect("draw interconnection");
    let mut labels: Vec<&str> = svg
        .split("<text class=\"viz-edge-label ")
        .skip(1)
        .filter_map(|rest| {
            rest.split_once('>')
                .and_then(|(_, text)| text.split_once('<'))
        })
        .map(|(text, _)| text)
        .collect();
    labels.sort_unstable();
    // `connect a.p to b.p` is anonymous and untyped, so it has no label (not "connector").
    assert_eq!(
        labels,
        [": DataLink", ": PowerLink", "mainPower", "namedUntyped"]
    );
    assert!(svg.matches("data-connector-id=").count() >= 5);
    assert!(
        !svg.contains("data-label-placement=\"tooltip-only\""),
        "every label found a clear place inside the `system` container"
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
    assert!(svg.contains(&format!(
        "data-view-name=\"{}\"",
        payload["selectedView"]["name"].as_str().unwrap()
    )));
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
    // Expand the scene explicitly: a collapsed root does not exercise a wide canvas.
    let expanded_node_ids = input["nodes"]
        .as_array()
        .expect("laid-out timer nodes")
        .iter()
        .map(|node| node["id"].as_str().expect("node identity").to_owned())
        .collect();
    let svg = render_svg_from_payload_with_options(
        &payload,
        &diagram_draw::theme::LIGHT,
        1280.0,
        900.0,
        Some(&diagram_draw::DisclosureState {
            expanded_node_ids,
            section_states: vec![],
        }),
    )
    .expect("draw expanded timer");
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
    assert!(
        background_width > 1280.0,
        "wide diagram needs full canvas: {background_width}"
    );
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

#[test]
fn explicit_expansion_keeps_usage_and_definition_independent_of_depth_default() {
    let payload = json!({
        "version": 1, "view": "general-view", "selectedViewName": "Power",
        "graph": {
            "nodes": [
                {"id": "usage", "name": "power", "type": "part", "attributes": {}},
                {"id": "definition", "name": "Power", "type": "part def", "attributes": {}},
                {"id": "usage-member", "name": "battery", "type": "part", "attributes": {"owner": "usage"}},
                {"id": "definition-member", "name": "battery", "type": "part", "attributes": {"owner": "definition"}}
            ],
            "edges": [
                {"id": "usage-owns", "source": "usage", "target": "usage-member", "type": "hierarchy"},
                {"id": "definition-owns", "source": "definition", "target": "definition-member", "type": "hierarchy"},
                {"id": "typing", "source": "usage", "target": "definition", "type": "typing"}
            ]
        }
    });
    for (expanded, visible_member, hidden_member) in [
        ("usage", "usage-member", "definition-member"),
        ("definition", "definition-member", "usage-member"),
    ] {
        let state = diagram_draw::DisclosureState {
            expanded_node_ids: vec![expanded.into()],
            section_states: vec![],
        };
        // Neither a fully closed nor a fully open depth default may replace a
        // reader's per-occurrence choices. The input publication stays intact.
        for depth in [diagram_draw::Nesting::Levels(0), diagram_draw::Nesting::All] {
            let input = diagram_draw::pipeline::draw_input_from_payload_with_presentation(
                &payload,
                Some(&state),
                depth,
            )
            .expect("independent expansion");
            let ids: Vec<&str> = input["nodes"]
                .as_array()
                .unwrap()
                .iter()
                .map(|node| node["id"].as_str().unwrap())
                .collect();
            assert_eq!(ids.len(), 3);
            assert!(ids.contains(&visible_member));
            assert!(!ids.contains(&hidden_member));
            assert!(ids.contains(&"usage") && ids.contains(&"definition"));
            assert_eq!(input["edges"].as_array().unwrap().len(), 1);
        }
    }
    assert_eq!(payload["graph"]["nodes"].as_array().unwrap().len(), 4);
}

#[test]
fn multiple_exposed_modules_use_the_view_title_for_the_frame() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_interconnection_incident_connection.md"
    ));
    let input = diagram_draw::pipeline::draw_input_from_payload_with_presentation(
        &payload,
        None,
        diagram_draw::Nesting::Levels(0),
    )
    .expect("closed modules");
    let svg = diagram_draw::render_svg_from_json(&input, 1280.0, 900.0).expect("module frame");
    let title = input["title"].as_str().expect("view title");
    assert!(svg.contains(&format!("data-view-name=\"{title}\"")));
    let xml = diagram_draw::xml::parse(&svg);
    fn frame(node: &diagram_draw::xml::XmlNode) -> Option<&diagram_draw::xml::XmlNode> {
        if node.attrs.get("class").map(String::as_str) == Some("view-frame") {
            node.children.iter().find(|child| child.tag == "rect")
        } else {
            node.children.iter().find_map(frame)
        }
    }
    let rect = frame(&xml).expect("view frame rectangle");
    let view_box: Vec<f64> = xml.attrs["viewBox"]
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let number = |name: &str| rect.attrs[name].parse::<f64>().unwrap();
    assert!(number("x") >= view_box[0] && number("y") >= view_box[1]);
    assert!(number("x") + number("width") <= view_box[0] + view_box[2]);
    assert!(number("y") + number("height") <= view_box[1] + view_box[3]);
}

#[test]
fn native_general_edges_keep_their_presentation_contract_through_layout() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_general_incident_allocation.md"
    ));
    let input = draw_input_from_payload(&payload).expect("native allocation layout");
    let edges = input["edges"].as_array().unwrap();
    assert!(!edges.is_empty());
    assert!(edges
        .iter()
        .all(|edge| edge["attributes"]["typedProjection"] == true));
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

fn schema6_sequence_payload() -> serde_json::Value {
    json!({
        "schemaVersion": 6,
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
fn sequence_schema6_draw_input_carries_the_scene_lifelines() {
    let input = draw_input_from_payload(&schema6_sequence_payload()).unwrap();
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
fn sequence_schema6_svg_has_lifelines_and_message() {
    let svg = render_svg_from_payload(&schema6_sequence_payload(), 1280.0, 900.0).unwrap();
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

fn schema6_state_transition_payload(
    vertices: Vec<serde_json::Value>,
    transitions: Vec<serde_json::Value>,
) -> serde_json::Value {
    json!({
        "schemaVersion": 6,
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
fn schema6_state_transition_out_of_range_endpoint_is_an_error() {
    let err = render_svg_from_payload(
        &schema6_state_transition_payload(
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
fn schema6_state_transition_missing_endpoint_is_an_error() {
    let err = render_svg_from_payload(
        &schema6_state_transition_payload(
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
fn schema6_state_transition_non_numeric_endpoint_is_an_error() {
    let err = render_svg_from_payload(
        &schema6_state_transition_payload(
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
fn an_action_flow_written_with_then_draws_its_whole_flow() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_action_flow_then_successions.md"
    ));
    let input = draw_input_from_payload(&payload).expect("action flow lays out");
    let nodes = input["prepared"]["nodes"].as_array().expect("nodes");
    let id_of = |label: &str, kind: &str| {
        nodes
            .iter()
            .find(|node| node["label"] == label && node["kind"] == kind)
            .unwrap_or_else(|| panic!("no {kind} node labelled `{label}`"))["id"]
            .as_str()
            .unwrap()
            .to_owned()
    };
    // The library's `start` and `done` are the initial and final nodes; they carry no name.
    let (start, done) = (id_of("", "initial"), id_of("", "final"));
    let (prepare, left, right, finish) = (
        id_of("prepare", "ActionUsage"),
        id_of("left", "ActionUsage"),
        id_of("right", "ActionUsage"),
        id_of("finish", "ActionUsage"),
    );
    let (split, joined, choice) = (
        id_of("split", "ForkNode"),
        id_of("joined", "JoinNode"),
        id_of("choice", "DecisionNode"),
    );
    let accept = id_of("waitForPayment", "AcceptActionUsage");
    // `then accept ...;` declares an accept action without a name of its own.
    let then_accept = id_of("accept", "AcceptActionUsage");
    let edges = input["prepared"]["edges"].as_array().expect("edges");
    let has = |source: &str, target: &str, flow: bool| {
        edges.iter().any(|edge| {
            edge["source"] == source
                && edge["target"] == target
                && edge["attributes"]["streamingFlow"] == flow
        })
    };
    // SysML 7.17.4: a `then` continues from the nearest occurrence written before it, so
    // `then prepare;` and `then fork split;` both start at `start`, and both `then`s after
    // the fork start at the fork.
    for (source, target) in [
        (&start, &prepare),
        (&start, &split),
        (&split, &left),
        (&split, &right),
        (&left, &joined),
        (&right, &joined),
        (&joined, &choice),
        (&choice, &accept),
        (&accept, &then_accept),
        (&then_accept, &finish),
        (&finish, &done),
    ] {
        assert!(
            has(source, target, false),
            "no succession {source} -> {target}"
        );
    }
    assert_eq!(
        edges
            .iter()
            .filter(|edge| edge["attributes"]["succession"] == true)
            .count(),
        11
    );
    // The flow between two parameters is drawn between the actions that own them.
    assert!(has(&prepare, &left, true));
    let node = |id: &str| nodes.iter().find(|node| node["id"] == id).unwrap();
    assert_eq!(node(&prepare)["attributes"]["outputs"][0]["name"], "order");
    assert_eq!(node(&left)["attributes"]["inputs"][0]["name"], "order");

    let svg = render_svg_from_payload(&payload, 1280.0, 900.0).expect("draw action flow");
    assert!(svg.contains("accept-action-icon"), "{svg}");
    assert_eq!(svg.matches("action-parameter-badge").count(), 2);
}

#[test]
fn a_port_takes_its_direction_from_the_features_of_its_definition() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_interconnection_feature_direction.md"
    ));
    let input = draw_input_from_payload(&payload).expect("interconnection lays out");
    let direction = |part: &str, port: &str| {
        input["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .find(|node| node["label"] == part)
            .unwrap_or_else(|| panic!("no part `{part}`"))["attributes"]["portDetails"]
            .as_array()
            .expect("port details")
            .iter()
            .find(|detail| detail["name"] == port)
            .unwrap_or_else(|| panic!("no port `{port}` on `{part}`"))["direction"]
            .clone()
    };
    // `Supply` has one `out` feature: the port sends, and its conjugate receives.
    assert_eq!(direction("source", "feed"), "out");
    assert_eq!(direction("load", "feed"), "in");
    // Features both ways make the port bidirectional; none leave it without a direction.
    assert_eq!(direction("source", "link"), "inout");
    assert!(direction("source", "tag").is_null());
    // A direction written on the port itself is what is drawn.
    assert_eq!(direction("load", "forced"), "in");
}

#[test]
fn a_flow_between_port_features_runs_between_the_ports() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_interconnection_flows.md"
    ));
    let input = draw_input_from_payload(&payload).expect("interconnection lays out");
    let part = |id: &serde_json::Value| {
        input["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .find(|node| &node["id"] == id)
            .map(|node| node["label"].as_str().unwrap().to_owned())
            .unwrap()
    };
    let flows: Vec<(String, String, String)> = input["edges"]
        .as_array()
        .expect("edges")
        .iter()
        .filter(|edge| edge["edgeKind"] == "flow")
        .map(|edge| {
            (
                part(&edge["source"]),
                part(&edge["target"]),
                edge["attributes"]["itemType"]
                    .as_str()
                    .unwrap_or("")
                    .to_owned(),
            )
        })
        .collect();
    // `tank.outlet.fuel` is a feature of the port, which the view does not draw: the flow
    // attaches to the port. It is labelled with its item, after its name when it has one.
    assert!(
        flows.contains(&("tank".into(), "engine".into(), "Fuel".into())),
        "{flows:?}"
    );
    assert!(
        flows.contains(&("spare".into(), "engine".into(), "topUp".into())),
        "{flows:?}"
    );
    assert_eq!(flows.len(), 2);
    let routed = input["interconnectionLayout"]["edges"]
        .as_array()
        .expect("routes");
    assert_eq!(routed.len(), 2);
}

#[test]
fn conditional_successions_carry_their_guards() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_action_flow_conditional_successions.md"
    ));
    let input = draw_input_from_payload(&payload).expect("action flow lays out");
    let nodes = input["prepared"]["nodes"].as_array().expect("nodes");
    let id_of = |label: &str, kind: &str| {
        nodes
            .iter()
            .find(|node| node["label"] == label && node["kind"] == kind)
            .unwrap_or_else(|| panic!("no {kind} node labelled `{label}`"))["id"]
            .as_str()
            .unwrap()
            .to_owned()
    };
    let label = |source: &str, target: &str| {
        input["prepared"]["edges"]
            .as_array()
            .expect("edges")
            .iter()
            .find(|edge| edge["source"] == source && edge["target"] == target)
            .unwrap_or_else(|| panic!("no succession {source} -> {target}"))["label"]
            .as_str()
            .unwrap()
            .to_owned()
    };
    let outcome = id_of("outcome", "DecisionNode");
    let (ship, repair, expedite, check) = (
        id_of("ship", "ActionUsage"),
        id_of("repair", "ActionUsage"),
        id_of("expedite", "ActionUsage"),
        id_of("check", "ActionUsage"),
    );
    let done = id_of("", "final");
    // A brace-less `if g then x; else y;` after a `decide` is two conditional successions
    // from the decision, not an if action.
    assert_eq!(label(&outcome, &ship), "[ready]");
    assert_eq!(label(&outcome, &repair), "[else]");
    assert!(nodes.iter().all(|node| node["kind"] != "IfActionUsage"));
    // Written in full, the guard is kept as it is authored.
    assert_eq!(label(&ship, &expedite), "[urgent]");
    assert_eq!(label(&ship, &done), "[not urgent]");
    assert_eq!(label(&repair, &check), "");

    let svg = render_svg_from_payload(&payload, 1280.0, 900.0).expect("draw action flow");
    for guard in ["[ready]", "[else]", "[urgent]", "[not urgent]"] {
        assert!(svg.contains(guard), "`{guard}` is not drawn");
    }
}

#[test]
fn a_state_lists_its_entry_do_and_exit_actions() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_state_actions.md"
    ));
    let input = draw_input_from_payload(&payload).expect("state machine lays out");
    let state = |label: &str| {
        input["prepared"]["nodes"]
            .as_array()
            .expect("nodes")
            .iter()
            .find(|node| node["label"] == label)
            .unwrap_or_else(|| panic!("no state `{label}`"))["attributes"]
            .clone()
    };
    let idle = state("idle");
    assert_eq!(idle["entry"], "warmUp");
    assert_eq!(idle["do"], "hold");
    assert_eq!(idle["exit"], "coolDown");
    let baking = state("baking");
    assert_eq!(baking["do"], "bake");
    assert!(baking["entry"].is_null() && baking["exit"].is_null());

    let svg = render_svg_from_payload(&payload, 1280.0, 900.0).expect("draw state machine");
    for line in ["entry warmUp", "do hold", "exit coolDown", "do bake"] {
        assert!(svg.contains(line), "`{line}` is not drawn: {svg}");
    }
    assert_eq!(svg.matches("state actions").count(), 2);
}

#[test]
fn a_state_that_owns_states_shows_them_inside_it() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_composite_state.md"
    ));
    let input = draw_input_from_payload(&payload).expect("state machine lays out");
    let nodes = input["prepared"]["nodes"].as_array().expect("nodes");
    let node = |label: &str| {
        nodes
            .iter()
            .find(|node| node["label"] == label && node["kind"] == "state")
            .unwrap_or_else(|| panic!("no state `{label}`"))
    };
    let on = node("on")["id"].as_str().unwrap();
    assert!(node("off")["attributes"]["containerId"].is_null());
    for nested in ["warming", "ready"] {
        assert_eq!(node(nested)["attributes"]["containerId"], on);
    }
    // The initial node of the nested machine is inside it as well.
    assert_eq!(
        nodes
            .iter()
            .filter(|node| node["kind"] == "initial" && node["attributes"]["containerId"] == on)
            .count(),
        1
    );
    // Each nested state lies within the box of the state that owns it.
    let rect = |id: &str| {
        let position = &input["behaviorLayout"]["positions"][id];
        let value = |key: &str| position[key].as_f64().unwrap();
        (
            value("x"),
            value("y"),
            value("x") + value("width"),
            value("y") + value("height"),
        )
    };
    let outer = rect(on);
    for nested in ["warming", "ready"] {
        let inner = rect(node(nested)["id"].as_str().unwrap());
        assert!(
            outer.0 < inner.0 && outer.1 < inner.1 && inner.2 < outer.2 && inner.3 < outer.3,
            "`{nested}` {inner:?} is not inside `on` {outer:?}"
        );
    }
    // `first on.ready ... then fault` leaves the nested state for one outside its owner. It is
    // routed, around the states in its way, and not left to a straight fallback line.
    let fault = node("fault")["id"].as_str().unwrap();
    let ready = node("ready")["id"].as_str().unwrap();
    let crossing = input["prepared"]["edges"]
        .as_array()
        .expect("edges")
        .iter()
        .find(|edge| edge["source"] == ready && edge["target"] == fault)
        .expect("the transition out of the nested state");
    let sections = &input["behaviorLayout"]["edgeSectionsById"][crossing["id"].as_str().unwrap()];
    assert!(sections[0]["startPoint"].is_object(), "{sections}");
    // The containing state is drawn below the transitions, the others above them.
    let svg = render_svg_from_payload(&payload, 1280.0, 900.0).expect("draw state machine");
    let layer = |class: &str| svg.find(&format!("class=\"{class}\"")).unwrap();
    assert!(layer("state-containers") < layer("state-transitions"));
    assert!(layer("state-transitions") < layer("state-nodes"));
}

#[test]
fn interconnection_view_opens_a_part_when_its_control_is_toggled() {
    use diagram_draw::theme::LIGHT;
    use diagram_draw::{render_svg_from_payload_with_options, DisclosureState};

    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_timer_interconnection.md"
    ));
    let draw = |expanded: &[&str]| {
        render_svg_from_payload_with_options(
            &payload,
            &LIGHT,
            1280.0,
            900.0,
            Some(&DisclosureState {
                expanded_node_ids: expanded.iter().map(|id| (*id).to_owned()).collect(),
                section_states: vec![],
            }),
        )
        .expect("draw timer")
    };
    let parts = |svg: &str| svg.matches("class=\"ibd-part").count();

    // As the editor first draws it: the exposed part with its direct parts, closed.
    let closed = draw(&[]);
    assert!(closed.contains("aria-label=\"Expand pcb\""), "{closed}");
    assert!(!closed.contains("aria-label=\"Collapse"), "{closed}");
    // The exposed part is always open, so it has no control.
    assert!(!closed.contains("aria-label=\"Expand timerInstance\""));

    // Toggle the id the rendered control actually carries; projection indexes can
    // change when another inherited member becomes visible.
    fn pcb_id(node: &diagram_draw::xml::XmlNode) -> Option<&str> {
        if node.attrs.get("data-element-name").map(String::as_str) == Some("pcb") {
            if let Some(id) = node.attrs.get("data-node-id") {
                return Some(id);
            }
        }
        node.children.iter().find_map(pcb_id)
    }
    let rendered = diagram_draw::xml::parse(&closed);
    let pcb = pcb_id(&rendered).expect("rendered pcb control");
    let opened = draw(&[pcb]);
    assert!(opened.contains("aria-label=\"Collapse pcb\""), "{opened}");
    assert!(parts(&opened) > parts(&closed));

    // A file export has no reader, so it has no controls.
    let exported = render_svg_from_payload(&payload, 1280.0, 900.0).expect("draw timer");
    assert!(!exported.contains("general-node-toggle"));
    assert!(parts(&exported) >= parts(&opened));
}

#[test]
fn general_view_disclosure_hides_nested_nodes_until_expanded() {
    use diagram_draw::theme::LIGHT;
    use diagram_draw::{render_svg_from_payload_with_options, DisclosureState};

    let payload = json!({
        "schemaVersion": 6,
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

/// A line in a nested general view that the router cannot place is still drawn, so the view is
/// not lost: here because one end contains the other (a recursive structure), and because the
/// diagram has more boxes than the router takes on.
#[test]
fn general_view_lines_are_drawn_when_they_cannot_be_routed() {
    let recursive = json!({
        "version": 1, "view": "general-view", "selectedViewName": "Recursive",
        "graph": {
            "nodes": [
                {"id": "Node", "name": "Node", "type": "part def"},
                {"id": "children", "name": "children", "type": "part", "attributes": {"partType": "Node"}}
            ],
            "edges": [
                {"id": "owns", "source": "Node", "target": "children", "type": "hierarchy"},
                {"id": "typed", "source": "children", "target": "Node", "type": "typing"}
            ]
        }
    });
    let mut nodes = vec![json!({"id": "system", "name": "system", "type": "part"})];
    let mut edges = Vec::new();
    for index in 0..90 {
        nodes.push(json!({"id": format!("p{index}"), "name": format!("p{index}"), "type": "part"}));
        nodes.push(
            json!({"id": format!("D{index}"), "name": format!("D{index}"), "type": "part def"}),
        );
        edges.push(json!({"id": format!("owns{index}"), "source": "system", "target": format!("p{index}"), "type": "hierarchy"}));
        edges.push(json!({"id": format!("typed{index}"), "source": format!("p{index}"), "target": format!("D{index}"), "type": "typing"}));
    }
    let large = json!({
        "version": 1, "view": "general-view", "selectedViewName": "Large",
        "graph": { "nodes": nodes, "edges": edges }
    });
    for (name, payload, lines) in [("recursive", recursive, 1), ("large", large, 90)] {
        let input = diagram_draw::pipeline::draw_input_from_payload_with_presentation(
            &payload,
            None,
            diagram_draw::Nesting::All,
        )
        .unwrap_or_else(|error| panic!("{name} general view lays out: {error:?}"));
        let drawn = input["edges"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|edge| edge["layout"]["sections"][0]["endPoint"].is_object())
            .count();
        assert_eq!(drawn, lines, "{name}: every typing line is drawn");
    }
}

/// `«ref»` and the dashed outline mark a usage declared `ref`, not every referential usage: a
/// port is referential by its kind and is drawn as a plain port (SysML 8.2.3).
#[test]
fn only_a_usage_declared_ref_is_drawn_as_a_reference() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_general_reference_membership.md"
    ));
    let svg = render_svg_from_payload_with_options(
        &payload,
        &diagram_draw::theme::LIGHT,
        1280.0,
        900.0,
        None,
    )
    .expect("reference membership view");
    assert_eq!(svg.matches("«ref part»").count(), 2, "{svg}");
    assert!(!svg.contains("«ref port»"), "{svg}");
    assert!(svg.contains("«port»"), "{svg}");
}

fn nested_ownership_payload() -> serde_json::Value {
    generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_nested_item_and_ports.md"
    ))
}

#[test]
fn nested_parts_stay_inside_their_item_containers() {
    let payload = nested_ownership_payload();
    let input = draw_input_from_payload(&payload).expect("nested ownership lays out");
    let nodes = input["nodes"].as_array().unwrap();
    let label_of = |id: &serde_json::Value| {
        nodes
            .iter()
            .find(|node| &node["id"] == id)
            .map(|node| node["label"].as_str().unwrap())
            .unwrap_or("-")
    };
    let mut ownership = nodes
        .iter()
        .map(|node| {
            (
                node["label"].as_str().unwrap(),
                label_of(&node["attributes"]["containerId"]),
            )
        })
        .collect::<Vec<_>>();
    ownership.sort_unstable();
    assert_eq!(
        ownership,
        [
            ("Assembly", "-"),
            ("carried", "destination"),
            ("destination", "Assembly"),
            ("inBox.box", "destination"),
            ("slot", "carried"),
            ("slot", "inBox.box"),
            ("src", "Assembly"),
        ]
    );
    for slot in nodes.iter().filter(|node| node["label"] == "slot") {
        let owner = nodes
            .iter()
            .find(|node| node["id"] == slot["attributes"]["containerId"])
            .unwrap();
        for axis in ["x", "y"] {
            assert!(slot[axis].as_f64().unwrap() >= owner[axis].as_f64().unwrap());
        }
        for (axis, extent) in [("x", "width"), ("y", "height")] {
            assert!(
                slot[axis].as_f64().unwrap() + slot[extent].as_f64().unwrap()
                    <= owner[axis].as_f64().unwrap() + owner[extent].as_f64().unwrap()
            );
        }
    }
    let svg = render_svg_from_payload(&payload, 1000.0, 800.0).expect("draw nested items");
    assert!(svg.contains("«inout item» inBox.box"));
    assert!(svg.contains("«ref item» carried"));
    // Closing their containing part hides the complete item subtree, never loose child boxes.
    let closed = diagram_draw::pipeline::draw_input_from_payload_with_presentation(
        &payload,
        None,
        diagram_draw::Nesting::Levels(1),
    )
    .expect("closed item subtree");
    let mut labels = closed["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|node| node["label"].as_str().unwrap())
        .collect::<Vec<_>>();
    labels.sort_unstable();
    assert_eq!(labels, ["Assembly", "destination", "src"]);
}

#[test]
fn bindings_to_nested_ports_keep_their_exact_boundary_anchors() {
    let payload = nested_ownership_payload();
    let input = draw_input_from_payload(&payload).expect("nested port binding lays out");
    let root = input["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|node| node["label"] == "Assembly")
        .unwrap();
    let ports = root["attributes"]["portDetails"].as_array().unwrap();
    let mut names = ports
        .iter()
        .map(|port| port["name"].as_str().unwrap())
        .collect::<Vec<_>>();
    names.sort_unstable();
    assert_eq!(
        names,
        ["bundle", "bundle.a", "bundle.b", "second", "second.a", "second.b"]
    );
    let nested = ports
        .iter()
        .find(|port| port["name"] == "bundle.a")
        .unwrap();
    let other = ports
        .iter()
        .find(|port| port["name"] == "second.a")
        .unwrap();
    assert_ne!(
        nested["id"], other["id"],
        "reused port definitions keep occurrence identities"
    );
    let edges = input["edges"].as_array().unwrap();
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0]["source"], root["id"]);
    assert_eq!(edges[0]["attributes"]["sourcePortId"], nested["id"]);
    let routes = input["interconnectionLayout"]["edges"].as_array().unwrap();
    assert_eq!(routes.len(), 1);
    assert!(routes[0]["routePoints"].as_array().unwrap().len() >= 2);
    let svg = render_svg_from_payload(&payload, 1000.0, 800.0).expect("draw nested binding");
    assert!(svg.contains("data-port-name=\"bundle.a\""));
    assert!(svg.contains("data-port-name=\"bundle.b\""));
    assert_eq!(svg.matches("class=\"ibd-connector ").count(), 1);
}

/// Every element of `node`'s subtree whose `class` attribute contains `class`.
fn with_class<'a>(
    node: &'a diagram_draw::xml::XmlNode,
    class: &str,
    found: &mut Vec<&'a diagram_draw::xml::XmlNode>,
) {
    if node
        .attrs
        .get("class")
        .is_some_and(|value| value.split_whitespace().any(|item| item == class))
    {
        found.push(node);
    }
    for child in &node.children {
        with_class(child, class, found);
    }
}

#[test]
fn ports_stand_on_the_boundary_and_connectors_end_on_their_squares() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_office_two_monitors.md"
    ));
    let input = draw_input_from_payload(&payload).expect("typed interconnection lays out");
    let layout = &input["interconnectionLayout"];
    let mut ends = Vec::new();
    for node in layout["nodes"].as_array().unwrap() {
        let width = node["width"].as_f64().unwrap();
        for anchor in node["portAnchors"].as_object().unwrap().values() {
            // SysML 8.2.3.12: a port is a small square on the boundary of its part.
            let x = anchor["x"].as_f64().unwrap();
            match anchor["side"].as_str().unwrap() {
                "WEST" => assert_eq!(x, 0.0, "{anchor}"),
                "EAST" => assert_eq!(x, width, "{anchor}"),
                _ => {}
            }
            ends.push((
                node["x"].as_f64().unwrap() + x,
                node["y"].as_f64().unwrap() + anchor["y"].as_f64().unwrap(),
            ));
        }
    }
    // A connector ends on the side of a port's 10 px square, not inside it or beyond it.
    for edge in layout["edges"].as_array().unwrap() {
        let points = edge["routePoints"].as_array().unwrap();
        for point in [points.first().unwrap(), points.last().unwrap()] {
            let (x, y) = (point["x"].as_f64().unwrap(), point["y"].as_f64().unwrap());
            assert!(
                ends.iter()
                    .any(|(px, py)| (x - px).abs() == 5.0 && y == *py),
                "connector end {point} is not on a port square"
            );
        }
    }
}

#[test]
fn flows_between_connected_ports_are_drawn_on_their_connector() {
    let payload = generated_diagram(include_str!(
        "../../../tests/snapshots/generation/diagram_interconnection_flows_on_connector.md"
    ));
    let svg = render_svg_from_payload(&payload, 1280.0, 900.0).expect("draw interconnection");
    let tree = diagram_draw::xml::parse(&svg);
    let mut connectors = Vec::new();
    with_class(&tree, "ibd-connector", &mut connectors);
    // SysML 8.2.3.12 `flow-on-connection`: the connector is the only line between the ports,
    // with one arrow for each flow on it.
    assert_eq!(connectors.len(), 1, "{svg}");
    assert_eq!(connectors[0].attrs["data-type"], "connector");
    let mut flows = Vec::new();
    with_class(&tree, "ibd-flow-on-connection", &mut flows);
    assert_eq!(flows.len(), 2, "{svg}");
    let (source, target) = (
        &connectors[0].attrs["data-source"],
        &connectors[0].attrs["data-target"],
    );
    let mut directions: Vec<(&String, &String)> = flows
        .iter()
        .map(|flow| (&flow.attrs["data-source"], &flow.attrs["data-target"]))
        .collect();
    directions.sort();
    let mut expected = vec![(source, target), (target, source)];
    expected.sort();
    assert_eq!(directions, expected);
}
