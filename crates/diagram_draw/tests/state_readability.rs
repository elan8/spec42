//! Geometry and text regressions for nested state presentation.
use diagram_draw::{draw_input_from_payload, render_svg_from_payload};
use serde_json::{json, Value};

#[test]
fn short_named_states_are_substantial_and_close_without_touching() {
    let payload = json!({"preparedView": {
        "view":"state-transition-view", "title":"Compact states", "meta":{"layoutDirection":"horizontal"},
        "nodes":[
            {"id":"a", "kind":"state", "label":"Idle", "attributes":{}},
            {"id":"b", "kind":"state", "label":"Running", "attributes":{}}
        ],
        "edges":[{"id":"next", "source":"a", "target":"b", "label":"ready", "attributes":{}}]
    }});
    let input = draw_input_from_payload(&payload).unwrap();
    let positions = &input["behaviorLayout"]["positions"];
    let a = &positions["a"];
    let b = &positions["b"];
    for state in [a, b] {
        assert!(state["width"].as_f64().unwrap() >= 120.0);
        assert!(state["height"].as_f64().unwrap() >= 56.0);
    }
    let gap = b["x"].as_f64().unwrap() - a["x"].as_f64().unwrap() - a["width"].as_f64().unwrap();
    assert!(gap >= 28.0, "transition channel is preserved: {gap}");
    assert!(
        gap < a["width"].as_f64().unwrap(),
        "the gap should not dominate a short transition: {gap}"
    );
}

fn payload(direction: &str) -> Value {
    json!({"preparedView": {
        "view": "state-transition-view", "title": "Nested states",
        "meta": {"layoutDirection": direction},
        "nodes": [
            {"id": "outer", "kind": "state", "label": "Controller", "attributes": {}},
            {"id": "a", "kind": "state", "label": "WaitingForTheExternalControllerToBecomeReady",
             "attributes": {"containerId": "outer", "entry": "prepareTheControllerAndVerifyEverySafetyInterlock"}},
            {"id": "b", "kind": "state", "label": "Running", "attributes": {"containerId": "outer"}}
        ],
        "edges": [{"id": "next", "source": "a", "target": "b", "label": "ready", "attributes": {}}]
    }})
}

#[test]
fn nested_state_layout_respects_the_requested_direction() {
    for direction in ["horizontal", "vertical"] {
        let input = draw_input_from_payload(&payload(direction)).expect("nested layout");
        assert_eq!(input["prepared"]["meta"]["layoutDirection"], direction);
        let positions = &input["behaviorLayout"]["positions"];
        let a = &positions["a"];
        let b = &positions["b"];
        let axis = if direction == "horizontal" { "x" } else { "y" };
        let size = if direction == "horizontal" {
            "width"
        } else {
            "height"
        };
        assert!(b[axis].as_f64().unwrap() >= a[axis].as_f64().unwrap() + a[size].as_f64().unwrap());
        let outer = &positions["outer"];
        for child in [a, b] {
            for (axis, size) in [("x", "width"), ("y", "height")] {
                assert!(child[axis].as_f64().unwrap() > outer[axis].as_f64().unwrap());
                assert!(
                    child[axis].as_f64().unwrap() + child[size].as_f64().unwrap()
                        < outer[axis].as_f64().unwrap() + outer[size].as_f64().unwrap()
                );
            }
        }
    }
}

#[test]
fn state_names_and_action_lines_are_complete_and_fit_their_boxes() {
    let payload = payload("horizontal");
    let input = draw_input_from_payload(&payload).expect("nested layout");
    let node = &payload["preparedView"]["nodes"][1];
    let name = node["label"].as_str().unwrap();
    let action = format!("entry {}", node["attributes"]["entry"].as_str().unwrap());
    let width = input["behaviorLayout"]["positions"]["a"]["width"]
        .as_f64()
        .unwrap();
    assert!(width >= name.chars().count() as f64 * 7.4 + 32.0);
    assert!(width >= action.chars().count() as f64 * 5.6 + 24.0);
    let svg = render_svg_from_payload(&payload, 1280.0, 900.0).expect("draw nested machine");
    assert!(svg.contains(name));
    assert!(svg.contains(&action));
}
