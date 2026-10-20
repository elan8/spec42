use diagram_draw::{draw_input_from_payload, render_svg_from_payload};
use serde_json::{json, Value};

fn payload(direction: Value, conjugated: bool, typed: bool) -> Value {
    let typing = if typed {
        json!({ "status": "resolved", "types": [{ "reference": 0, "label": "P" }] })
    } else {
        json!({ "status": "absent", "types": [] })
    };
    json!({
        "schemaVersion": 6,
        "documents": [{ "uri": "file:///notation.sysml" }],
        "sources": [{ "document": 0, "range": [0, 0, 0, 1] }],
        "references": [{ "kind": "qualified-name", "qualifiedName": "P" }],
        "selectedView": { "reference": 0, "kind": "general-view", "name": "Features", "source": 0 },
        "completeness": { "status": "complete", "reasons": [] },
        "projection": {
            "kind": "general-view", "exposedRoots": [0],
            "nodes": [
                { "reference": 0, "metaclass": "InterfaceDefinition", "notationRole": "definition",
                  "name": "I", "typing": { "status": "absent" }, "owner": null, "source": 0,
                  "compartments": [{ "kind": "ports", "provenance": "direct", "members": [1] }] },
                { "reference": 0, "metaclass": "PortUsage", "notationRole": "usage",
                  "name": "y", "typing": typing, "direction": direction,
                  "featureDirection": "out", "conjugated": conjugated,
                  "owner": 0, "source": 0, "compartments": [] }
            ],
            "relationships": [], "edges": [], "metadata": {}, "scene": { "kind": "general" }
        }
    })
}

#[test]
fn general_feature_headers_and_compartments_preserve_supplied_notation() {
    for (direction, conjugated, typed, name, type_name, row) in [
        (json!("in"), true, true, "in y", json!("~P"), "in y : ~P"),
        (json!("out"), false, true, "out y", json!("P"), "out y : P"),
        (
            json!("inout"),
            true,
            true,
            "inout y",
            json!("~P"),
            "inout y : ~P",
        ),
        (Value::Null, false, true, "y", json!("P"), "y : P"),
        (Value::Null, true, false, "y", Value::Null, "y"),
    ] {
        let payload = payload(direction, conjugated, typed);
        let input = draw_input_from_payload(&payload).expect("feature notation lays out");
        let nodes = input["nodes"].as_array().unwrap();
        let feature = nodes.iter().find(|node| node["id"] == "n:1").unwrap();
        assert_eq!(feature["compartments"]["header"]["name"], name);
        assert_eq!(feature["compartments"]["typedByName"], type_name);
        let owner = nodes.iter().find(|node| node["id"] == "n:0").unwrap();
        assert_eq!(
            owner["compartments"]["collapsibleSections"][0]["items"][0]["displayText"],
            row
        );
        // The source label remains suitable for navigation; formatting is presentation only.
        assert_eq!(feature["label"], "y");
        let svg = render_svg_from_payload(&payload, 800.0, 600.0).expect("draw feature notation");
        let xml = diagram_draw::xml::parse(&svg);
        fn texts<'a>(node: &'a diagram_draw::xml::XmlNode, result: &mut Vec<&'a str>) {
            if node.tag == "text" || node.tag == "tspan" {
                result.push(&node.text);
            }
            for child in &node.children {
                texts(child, result);
            }
        }
        let mut rendered = Vec::new();
        texts(&xml, &mut rendered);
        assert!(rendered.contains(&name), "header text: {rendered:?}");
        assert!(rendered.contains(&row), "compartment text: {rendered:?}");
        if let Some(type_name) = type_name.as_str() {
            assert!(rendered.contains(&format!(": {type_name}").as_str()));
        }
    }
}
