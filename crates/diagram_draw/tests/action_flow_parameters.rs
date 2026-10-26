//! Flows between parameters run from the square of one parameter to the square of the other
//! (SysML v2 8.2.3.17), including the parameters of the action that holds the flow.

use diagram_draw::{draw_input_from_payload, render_svg_from_payload};
use serde_json::{json, Value};

fn node(metaclass: &str, name: &str, owner: Value, direction: Value) -> Value {
    json!({
        "reference": 0, "metaclass": metaclass,
        "notationRole": if metaclass == "ActionDefinition" { "definition" } else { "usage" },
        "name": name, "typing": { "status": "absent" }, "owner": owner, "source": 0,
        "compartments": [], "direction": direction, "conjugated": false,
    })
}

fn edge(kind: &str, source: usize, target: usize) -> Value {
    json!({ "reference": 0, "source": source, "target": target, "origin": source,
            "kind": kind, "provenance": "authored", "navigation": 0 })
}

/// `action def A { in x; out y; action b { in p; out q; } action c { in r; out s; }
/// flow x to b.p; flow b.q to c.r; flow c.s to y; first b then c; }`
fn payload() -> Value {
    json!({
        "schemaVersion": 8,
        "documents": [{ "uri": "file:///flow.sysml" }],
        "sources": [{ "document": 0, "range": [0, 0, 0, 1] }],
        "references": [{ "kind": "qualified-name", "qualifiedName": "A" }],
        "selectedView": { "reference": 0, "kind": "action-flow-view", "name": "Flow", "source": 0 },
        "completeness": { "status": "complete", "reasons": [] },
        "projection": {
            "kind": "action-flow-view", "exposedRoots": [0],
            "nodes": [
                node("ActionDefinition", "A", Value::Null, Value::Null),
                node("ReferenceUsage", "x", json!(0), json!("in")),
                node("ReferenceUsage", "y", json!(0), json!("out")),
                node("ActionUsage", "b", json!(0), Value::Null),
                node("ReferenceUsage", "p", json!(3), json!("in")),
                node("ReferenceUsage", "q", json!(3), json!("out")),
                node("ActionUsage", "c", json!(0), Value::Null),
                node("ReferenceUsage", "r", json!(6), json!("in")),
                node("ReferenceUsage", "s", json!(6), json!("out")),
            ],
            "relationships": [],
            "edges": [
                edge("flow", 1, 4),
                edge("flow", 5, 7),
                edge("flow", 8, 2),
                edge("succession", 3, 6),
            ],
            "metadata": { "actions": [0, 3, 6], "controlNodes": [] },
            "scene": { "kind": "action-flow" }
        }
    })
}

fn point(value: &Value) -> (f64, f64) {
    (value["x"].as_f64().unwrap(), value["y"].as_f64().unwrap())
}

#[test]
fn flows_between_parameters_start_and_end_on_their_squares() {
    let input = draw_input_from_payload(&payload()).expect("action flow lays out");
    let layout = &input["behaviorLayout"];
    let rect = |id: &str| {
        let rect = &layout["positions"][id];
        (
            rect["x"].as_f64().unwrap(),
            rect["y"].as_f64().unwrap(),
            rect["width"].as_f64().unwrap(),
            rect["height"].as_f64().unwrap(),
        )
    };
    let route = |id: &str| {
        let section = &layout["edgeSectionsById"][id][0];
        let mut points = vec![point(&section["startPoint"])];
        points.extend(section["bendPoints"].as_array().unwrap().iter().map(point));
        points.push(point(&section["endPoint"]));
        points
    };
    let (a, b, c) = (rect("n:0"), rect("n:3"), rect("n:6"));
    type Rect = (f64, f64, f64, f64);
    // A parameter square stands on the left or right side of its action, below its top.
    let on_left = |(x, y): (f64, f64), rect: Rect| x == rect.0 && y > rect.1 && y < rect.1 + rect.3;
    let on_right =
        |(x, y): (f64, f64), rect: Rect| x == rect.0 + rect.2 && y > rect.1 && y < rect.1 + rect.3;

    // From the input of the holding action, on its left side, inwards to the input of `b`.
    let x_to_p = route("e:0");
    assert!(on_left(x_to_p[0], a), "{x_to_p:?}");
    // Below the name compartment of the holding action.
    assert!(x_to_p[0].1 > a.1 + 44.0, "{x_to_p:?}");
    assert!(x_to_p[1].0 > a.0, "{x_to_p:?}");
    assert!(on_left(*x_to_p.last().unwrap(), b), "{x_to_p:?}");

    // From the output of `b`, on its right side, to the input of `c`, on its left side.
    let q_to_r = route("e:1");
    assert!(on_right(q_to_r[0], b), "{q_to_r:?}");
    assert!(q_to_r[1].0 > b.0 + b.2, "{q_to_r:?}");
    assert!(on_left(*q_to_r.last().unwrap(), c), "{q_to_r:?}");
    assert!(q_to_r[q_to_r.len() - 2].0 < c.0, "{q_to_r:?}");

    // To the output of the holding action, on its right side, from inside it.
    let s_to_y = route("e:2");
    assert!(on_right(s_to_y[0], c), "{s_to_y:?}");
    assert!(on_right(*s_to_y.last().unwrap(), a), "{s_to_y:?}");
    assert!(s_to_y[s_to_y.len() - 2].0 < a.0 + a.2, "{s_to_y:?}");

    // Every route is orthogonal.
    for points in [&x_to_p, &q_to_r, &s_to_y] {
        assert!(points
            .windows(2)
            .all(|pair| pair[0].0 == pair[1].0 || pair[0].1 == pair[1].1));
    }

    // The holding action draws its own parameters as squares on its boundary.
    let svg = render_svg_from_payload(&payload(), 800.0, 600.0).expect("draw action flow");
    assert_eq!(svg.matches("class=\"action-parameter-badge").count(), 6);
}

#[test]
fn a_flow_goes_around_the_names_of_other_parameters() {
    // `b` gets a second output below `q`, with a long name. The flow from `q` to `c.r` must not
    // run through that name, which stands beside `b` above its square.
    let mut payload = payload();
    let nodes = payload["projection"]["nodes"].as_array_mut().unwrap();
    nodes.push(node(
        "ReferenceUsage",
        "aVeryLongParameterName",
        json!(3),
        json!("out"),
    ));
    let input = draw_input_from_payload(&payload).expect("action flow lays out");
    let layout = &input["behaviorLayout"];
    let b = &layout["positions"]["n:3"];
    let (bx, by, bw) = (
        b["x"].as_f64().unwrap(),
        b["y"].as_f64().unwrap(),
        b["width"].as_f64().unwrap(),
    );
    // The second output's square is 38 px below the top of `b`; its name is 8 px text whose
    // baseline is 6 px above the square, starting 9 px out from the side.
    let name = (bx + bw + 9.0, by + 38.0 - 13.0, 22.0 * 5.0, 10.0);
    let section = &layout["edgeSectionsById"]["e:1"][0];
    let mut points = vec![point(&section["startPoint"])];
    points.extend(section["bendPoints"].as_array().unwrap().iter().map(point));
    points.push(point(&section["endPoint"]));
    let crosses = points.windows(2).any(|pair| {
        let (a, b) = (pair[0], pair[1]);
        let (left, right) = (a.0.min(b.0), a.0.max(b.0));
        let (top, bottom) = (a.1.min(b.1), a.1.max(b.1));
        left < name.0 + name.2 && right > name.0 && top < name.1 + name.3 && bottom > name.1
    });
    assert!(
        !crosses,
        "flow {points:?} runs through the name at {name:?}"
    );
}
