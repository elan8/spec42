//! Root-view framing: OMG SysML v2 formal/2026-03-02 §§8.2.3.1, 8.2.3.26.
use diagram_draw::{
    pipeline,
    theme::{DARK, LIGHT},
    xml::{self, XmlNode},
};
use serde_json::{json, Value};

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

fn with_class<'a>(node: &'a XmlNode, class: &str) -> Vec<&'a XmlNode> {
    let mut result = Vec::new();
    if node
        .attrs
        .get("class")
        .is_some_and(|s| s.split_whitespace().any(|c| c == class))
    {
        result.push(node);
    }
    for child in &node.children {
        result.extend(with_class(child, class));
    }
    result
}

fn assert_frame(svg: &str, title: &str) {
    let tree = xml::parse(svg);
    assert_eq!(with_class(&tree, "view-frame").len(), 1);
    let names = with_class(&tree, "view-frame-name");
    assert_eq!(names.len(), 1);
    assert_eq!(names[0].text, format!("«view» {title}"));
    let border = with_class(&tree, "view-frame-border")[0];
    let number = |node: &XmlNode, key: &str| node.attrs[key].parse::<f64>().unwrap();
    let view_box: Vec<f64> = tree.attrs["viewBox"]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    for (axis, size, i) in [("x", "width", 0), ("y", "height", 1)] {
        assert!(number(border, axis) >= view_box[i]);
        assert!(number(border, axis) + number(border, size) <= view_box[i] + view_box[i + 2]);
    }
}

#[test]
fn configuration_change_preserves_the_action_boundary_and_view_identity() {
    let original = fixture("diagram_configuration_change");
    for title in [
        "configurationChange",
        "ApprovalScenario",
        "視圖::承認と変更_Élévateur_WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW",
    ] {
        let mut product = original.clone();
        product["selectedView"]["name"] = json!(title);
        for theme in [&LIGHT, &DARK] {
            let svg = pipeline::render_svg_from_payload_with_theme(&product, theme, 800.0, 600.0)
                .unwrap();
            assert_frame(&svg, title);
            let tree = xml::parse(&svg);
            assert_eq!(with_class(&tree, "action-flow-container").len(), 1);
            assert_eq!(with_class(&tree, "action-flow-node").len(), 5);
            assert_eq!(with_class(&tree, "action-flow-edge").len(), 3);
            assert!(svg.contains("«action»"));
            assert!(svg.contains("configurationChange"));
        }
    }
}

#[test]
fn state_view_title_is_independent_of_the_exposed_machine() {
    let mut product = fixture("diagram_state_transition_complete");
    product["selectedView"]["name"] = json!("LifecycleReview");
    let input = pipeline::draw_input_from_payload(&product).unwrap();
    assert_eq!(input["prepared"]["title"], "LifecycleReview");
    for theme in [&LIGHT, &DARK] {
        assert_frame(
            &diagram_draw::render_svg_from_json_with_theme(&input, theme, 800.0, 600.0).unwrap(),
            "LifecycleReview",
        );
    }
}

#[test]
fn empty_and_incomplete_behavior_views_keep_their_view_boundary() {
    for fixture_name in [
        "diagram_action_flow_complete",
        "diagram_state_transition_complete",
    ] {
        let mut product = fixture(fixture_name);
        product["projection"]["nodes"] = json!([]);
        product["projection"]["edges"] = json!([]);
        product["projection"]["metadata"] = json!({"actions":[],"controlNodes":[]});
        if product["projection"]["kind"] == "state-transition-view" {
            product["projection"]["scene"]["vertices"] = json!([]);
            product["projection"]["scene"]["transitions"] = json!([]);
        }
        for status in ["complete", "incomplete"] {
            product["completeness"]["status"] = json!(status);
            let svg = pipeline::render_svg_from_payload(&product, 800.0, 600.0).unwrap();
            assert_frame(&svg, product["selectedView"]["name"].as_str().unwrap());
        }
    }
}

#[test]
fn action_frame_encloses_parameters_swimlanes_and_detoured_edge_labels() {
    let input = json!({
        "prepared": {"title":"Procedure", "view":"action-flow-view",
            "nodes":[
                {"id":"a", "label":"a", "kind":"action", "attributes":{"swimLane":"Operator", "inputs":[{"name":"veryLongInputParameter"}]}},
                {"id":"b", "label":"b", "kind":"action", "attributes":{"swimLane":"Controller"}}
            ], "edges":[]},
        "behaviorLayout": {"positions": {
            "a":{"x":0,"y":0,"width":200,"height":70},
            "b":{"x":300,"y":0,"width":200,"height":70}},
            "edgeLabelsById":{"detour":[{"id":"label", "text":"guard", "x":550,"y":-90,"width":120,"height":20}]}}
    });
    let svg = diagram_draw::render_svg_from_json(&input, 800.0, 600.0).unwrap();
    assert_frame(&svg, "Procedure");
    let tree = xml::parse(&svg);
    let frame = with_class(&tree, "view-frame-border")[0];
    let x: f64 = frame.attrs["x"].parse().unwrap();
    let y: f64 = frame.attrs["y"].parse().unwrap();
    let width: f64 = frame.attrs["width"].parse().unwrap();
    assert!(x < -177.0, "input label must be inside the frame");
    assert!(y < -90.0, "detoured label must be inside the frame");
    assert!(x + width > 670.0);
}

#[test]
fn nested_behavior_containers_do_not_acquire_extra_view_frames() {
    for name in ["diagram_composite_action_flow", "diagram_composite_state"] {
        let product = fixture(name);
        let svg = pipeline::render_svg_from_payload(&product, 800.0, 600.0).unwrap();
        assert_frame(&svg, product["selectedView"]["name"].as_str().unwrap());
    }
}

#[test]
fn all_five_standard_views_share_one_selected_view_frame() {
    for name in [
        "diagram_general_complete",
        "diagram_interconnection_complete",
        "diagram_action_flow_complete",
        "diagram_state_transition_complete",
        "diagram_sequence_complete",
    ] {
        let mut product = fixture(name);
        product["selectedView"]["name"] =
            json!("視圖::ArchitectureReview_WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW");
        for nesting in [diagram_draw::Nesting::All, diagram_draw::Nesting::Levels(0)] {
            let input =
                pipeline::draw_input_from_payload_with_presentation(&product, None, nesting)
                    .unwrap();
            for theme in [&LIGHT, &DARK] {
                let svg =
                    diagram_draw::render_svg_from_json_with_theme(&input, theme, 0.0, 0.0).unwrap();
                assert_frame(&svg, product["selectedView"]["name"].as_str().unwrap());
                assert_eq!(with_class(&xml::parse(&svg), "ibd-view-frame").len(), 0);
            }
        }
    }
}

#[test]
fn multiple_roots_and_incomplete_projections_keep_a_single_view_frame() {
    for name in [
        "diagram_interconnection_incident_connection",
        "diagram_interconnection_connector_incomplete",
        "diagram_action_flow_unresolved",
        "diagram_state_transition_unresolved",
    ] {
        let product = fixture(name);
        let svg = pipeline::render_svg_from_payload(&product, 800.0, 600.0).unwrap();
        assert_frame(&svg, product["selectedView"]["name"].as_str().unwrap());
    }
}

#[test]
fn empty_standard_views_keep_the_selected_view_name() {
    for kind in ["general-view", "interconnection-view", "sequence-view"] {
        let input =
            json!({"view":kind, "title":"EmptySelection", "nodes":[], "edges":[], "meta":{}});
        let svg = diagram_draw::render_svg_from_json(&input, 800.0, 600.0).unwrap();
        assert_frame(&svg, "EmptySelection");
    }
}

#[test]
fn interconnection_frame_encloses_external_port_labels_and_container_boxes() {
    let mut input: Value = serde_json::from_str(include_str!(
        "fixtures/interconnection-view-fidelity.laid-out.json"
    ))
    .unwrap();
    // Place one port label well outside its owning part. This is renderer geometry, not a model edit.
    let nodes = input["interconnectionLayout"]["nodes"]
        .as_array_mut()
        .unwrap();
    let node = nodes
        .iter_mut()
        .find(|n| !n["portAnchors"].as_object().unwrap().is_empty())
        .unwrap();
    let x = node["x"].as_f64().unwrap();
    let y = node["y"].as_f64().unwrap();
    let anchor = node["portAnchors"]
        .as_object_mut()
        .unwrap()
        .values_mut()
        .next()
        .unwrap();
    anchor["label"] = json!({"x":-550,"y":-90,"width":250,"height":14,"text":"external label"});
    let svg = diagram_draw::render_svg_from_json(&input, 800.0, 600.0).unwrap();
    assert_frame(&svg, input["title"].as_str().unwrap());
    let tree = xml::parse(&svg);
    let border = with_class(&tree, "view-frame-border")[0];
    assert!(border.attrs["x"].parse::<f64>().unwrap() < x - 550.0);
    assert!(border.attrs["y"].parse::<f64>().unwrap() < y - 90.0);
}
