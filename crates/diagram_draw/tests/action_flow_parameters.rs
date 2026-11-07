//! Flows between parameters run from the box of one parameter to the box of the other (SysML v2
//! 8.2.3.17), including the parameters of the action that holds the flow. A parameter's box stands
//! against its action's boundary on the outside and holds its direction mark. In a top-to-bottom
//! flow an action's inputs stand on its top edge and its outputs on its bottom edge
//! (`param-t`, `param-b`); an action that holds its own flow keeps them on its sides.

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
        "schemaVersion": diagram_product::SCHEMA_VERSION,
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
    // The box of an input of `b` or `c` stands on the top edge and that of an output on the
    // bottom edge; a flow meets its outer edge, 12 px out. The holding action keeps its parameters
    // on its sides, and a flow inside it meets the side itself.
    const BOX: f64 = 12.0;
    let across = |x: f64, rect: Rect| x > rect.0 && x < rect.0 + rect.2;
    let within = |y: f64, rect: Rect| y > rect.1 && y < rect.1 + rect.3;
    let on_top = |(x, y): (f64, f64), rect: Rect| y == rect.1 - BOX && across(x, rect);
    let on_bottom = |(x, y): (f64, f64), rect: Rect| y == rect.1 + rect.3 + BOX && across(x, rect);
    let inside_left = |(x, y): (f64, f64), rect: Rect| x == rect.0 && within(y, rect);
    let inside_right = |(x, y): (f64, f64), rect: Rect| x == rect.0 + rect.2 && within(y, rect);

    // From the input of the holding action, on its left side, inwards to the input of `b`, which
    // it enters from above.
    let x_to_p = route("e:0");
    assert!(inside_left(x_to_p[0], a), "{x_to_p:?}");
    // Below the name compartment of the holding action.
    assert!(x_to_p[0].1 > a.1 + 44.0, "{x_to_p:?}");
    assert!(x_to_p[1].0 > a.0, "{x_to_p:?}");
    assert!(on_top(*x_to_p.last().unwrap(), b), "{x_to_p:?}");
    assert!(x_to_p[x_to_p.len() - 2].1 < b.1 - BOX, "{x_to_p:?}");

    // From the output of `b`, leaving its bottom edge downwards, to the input of `c` on its top.
    let q_to_r = route("e:1");
    assert!(on_bottom(q_to_r[0], b), "{q_to_r:?}");
    assert!(q_to_r[1].1 > b.1 + b.3 + BOX, "{q_to_r:?}");
    assert!(on_top(*q_to_r.last().unwrap(), c), "{q_to_r:?}");
    assert!(q_to_r[q_to_r.len() - 2].1 < c.1 - BOX, "{q_to_r:?}");

    // From the output of `c`, on its bottom edge, to the output of the holding action, on its
    // right side, from inside it.
    let s_to_y = route("e:2");
    assert!(on_bottom(s_to_y[0], c), "{s_to_y:?}");
    assert!(inside_right(*s_to_y.last().unwrap(), a), "{s_to_y:?}");
    assert!(s_to_y[s_to_y.len() - 2].0 < a.0 + a.2, "{s_to_y:?}");

    // Every route is orthogonal.
    for points in [&x_to_p, &q_to_r, &s_to_y] {
        assert!(points
            .windows(2)
            .all(|pair| pair[0].0 == pair[1].0 || pair[0].1 == pair[1].1));
    }

    // The holding action draws its own parameters as boxes on its boundary too.
    let svg = render_svg_from_payload(&payload(), 800.0, 600.0).expect("draw action flow");
    assert_eq!(svg.matches("class=\"action-parameter-badge").count(), 6);
}

#[test]
fn a_parameter_box_holds_its_direction_and_an_action_flow_draws_no_ports() {
    // `b` gets an `inout` parameter, a parameter without a direction, and a port.
    let mut payload = payload();
    let nodes = payload["projection"]["nodes"].as_array_mut().unwrap();
    nodes.push(node("ReferenceUsage", "both", json!(3), json!("inout")));
    nodes.push(node("ReferenceUsage", "plain", json!(3), Value::Null));
    nodes.push(node("PortUsage", "socket", json!(3), json!("in")));
    let svg = render_svg_from_payload(&payload, 800.0, 600.0).expect("draw action flow");

    // in: x, p, r; out: y, q, s; inout: both. The undirected feature and the port get no box.
    assert_eq!(svg.matches("class=\"action-parameter-badge").count(), 7);
    assert_eq!(svg.matches("action-parameter-direction-in\"").count(), 3);
    assert_eq!(svg.matches("action-parameter-direction-out\"").count(), 3);
    assert_eq!(svg.matches("action-parameter-direction-inout\"").count(), 1);
    assert!(
        !svg.contains(">plain<"),
        "an undirected feature is not a parameter"
    );
    assert!(
        !svg.contains(">socket<"),
        "an action flow view draws no ports"
    );
    for port_class in [
        "port-icon",
        "port-label",
        "port-direction",
        "viz-port-overlay",
    ] {
        assert!(
            !svg.contains(&format!("class=\"{port_class}")),
            "no port notation in an action flow view: {port_class}"
        );
    }

    // The box of an input stands above its action's top edge, touching it; that of an output
    // below its bottom edge.
    let input = draw_input_from_payload(&payload).expect("action flow lays out");
    let b = &input["behaviorLayout"]["positions"]["n:3"];
    let (by, bh) = (b["y"].as_f64().unwrap(), b["height"].as_f64().unwrap());
    let badges = svg
        .split("<g class=\"activity-action")
        .find(|group| group.contains(">b</text>"))
        .expect("the group of b");
    let tree = diagram_draw::xml::parse(&format!("<g>{badges}"));
    let mut pending = vec![&tree];
    let mut boxes = Vec::new();
    while let Some(element) = pending.pop() {
        if element.tag == "rect" {
            if let Some(class) = element.attrs.get("class") {
                if class.starts_with("action-parameter-badge") {
                    boxes.push((class.clone(), element.attrs["y"].parse::<f64>().unwrap()));
                }
            }
        }
        pending.extend(&element.children);
    }
    assert!(boxes
        .iter()
        .any(|(class, y)| class.ends_with("input") && *y == -12.0));
    assert!(boxes
        .iter()
        .any(|(class, y)| class.ends_with("output") && *y == bh));
    assert!(by.is_finite());
}

#[test]
fn a_flow_goes_around_the_names_of_other_parameters() {
    // `b` gets a second output beside `q`, with a long name. The flow from `q` to `c.r` must not
    // run through that name, which stands beside its box under `b`.
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
    let (bx, by, bw, bh) = (
        b["x"].as_f64().unwrap(),
        b["y"].as_f64().unwrap(),
        b["width"].as_f64().unwrap(),
        b["height"].as_f64().unwrap(),
    );
    // The two outputs share the bottom edge, side by side from 16 px in, each in a slot of its
    // box, 16 px and its name; the second's name is 8 px text starting 3 px beyond its 12 px box,
    // level with the box.
    let first_slot = 5.4 + 12.0 + 16.0; // `q`
    let centre = bx + 16.0 + first_slot + 6.0;
    let name = (centre + 9.0, by + bh + 1.0, 22.0 * 5.4, 10.0);
    assert!(bw > 0.0);
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
