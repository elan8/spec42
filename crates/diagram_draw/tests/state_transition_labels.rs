//! Transition labels: SysML 8.2.3.18 notation, and each label on its own transition.
use diagram_draw::pipeline::{draw_input_from_payload, render_svg_from_payload};
use serde_json::Value;

fn fixture(name: &str) -> Value {
    let path = format!(
        "{}/../../tests/snapshots/generation/{name}.md",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(path).unwrap().replace("\r\n", "\n");
    let json = text
        .split_once("## diagram.json\n~~~json\n")
        .unwrap()
        .1
        .split_once("\n~~~")
        .unwrap()
        .0;
    serde_json::from_str(json).unwrap()
}

fn points(sections: &Value) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    for section in sections.as_array().into_iter().flatten() {
        let point = |value: &Value| (value["x"].as_f64().unwrap(), value["y"].as_f64().unwrap());
        points.push(point(&section["startPoint"]));
        for bend in section["bendPoints"].as_array().into_iter().flatten() {
            points.push(point(bend));
        }
        points.push(point(&section["endPoint"]));
    }
    points
}

fn distance_to_polyline((x, y): (f64, f64), polyline: &[(f64, f64)]) -> f64 {
    polyline
        .windows(2)
        .map(|segment| {
            let ((x1, y1), (x2, y2)) = (segment[0], segment[1]);
            let (dx, dy) = (x2 - x1, y2 - y1);
            let length = dx * dx + dy * dy;
            let t = if length == 0.0 {
                0.0
            } else {
                (((x - x1) * dx + (y - y1) * dy) / length).clamp(0.0, 1.0)
            };
            ((x - (x1 + t * dx)).powi(2) + (y - (y1 + t * dy)).powi(2)).sqrt()
        })
        .fold(f64::INFINITY, f64::min)
}

#[test]
fn transition_labels_show_trigger_guard_and_effect() {
    let svg = render_svg_from_payload(
        &fixture("diagram_state_transition_guard_effect"),
        1280.0,
        900.0,
    )
    .expect("the state machine renders");
    assert!(
        svg.contains(">Interrupted [attempts &lt; limit] / count<"),
        "{svg}"
    );
    assert!(
        svg.contains(">Interrupted [attempts &gt;= limit]<"),
        "{svg}"
    );
    assert!(svg.contains(">Closed<"), "{svg}");
}

#[test]
fn each_transition_label_lies_on_its_own_transition() {
    let input = draw_input_from_payload(&fixture("diagram_state_transition_guard_effect"))
        .expect("the state machine lays out");
    let layout = &input["behaviorLayout"];
    let sections = layout["edgeSectionsById"]
        .as_object()
        .expect("routed edges");
    let labels = layout["edgeLabelsById"].as_object().expect("label boxes");
    assert!(!labels.is_empty(), "transitions carry labels");
    for (edge_id, boxes) in labels {
        let label = &boxes[0];
        let centre = (
            label["x"].as_f64().unwrap() + label["width"].as_f64().unwrap() / 2.0,
            label["y"].as_f64().unwrap() + label["height"].as_f64().unwrap() / 2.0,
        );
        let own = sections
            .get(edge_id)
            .unwrap_or_else(|| panic!("label {edge_id} has a routed edge"));
        let distance = distance_to_polyline(centre, &points(own));
        assert!(
            distance <= 1.0,
            "label of {edge_id} is {distance:.1} px off its own transition"
        );
    }
}
