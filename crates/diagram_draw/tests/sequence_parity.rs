//! spec42 #176 phase 2 acceptance test for sequence-view, mirroring `golden_parity.rs`'s
//! full-tree byte-comparison approach: parse both the Rust output and a real TS/D3-rendered SVG of
//! the identical fixture into trees and assert full structural equality.

use std::path::Path;

use diagram_draw::behavior_common::PreparedView;
use diagram_draw::render_sequence_view_svg;
use diagram_draw::theme::LIGHT;
use diagram_draw::xml;
use serde_json::json;

#[test]
fn a_single_lifeline_export_keeps_long_titles_inside_the_view_box() {
    // The normal title is about 246 px in a 14 px bold serif font; the wide-glyph title
    // needs over 500 px. Neither fits in the 164 px available in the old export.
    for (title, minimum_width) in [
        ("MotorControllerInitializationSequence", 246.0),
        ("WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW", 520.0),
    ] {
        let prepared: PreparedView = serde_json::from_value(json!({
            "title": title, "view": "sequence-view", "nodes": [], "edges": [],
            "meta": {"sequenceDiagram": {
                "lifelines": [{"id": "controller", "name": "Controller"}],
                "messages": [{"id": "init", "name": "initialize", "source": "controller",
                    "target": "controller", "order": 1}]
            }}
        }))
        .unwrap();
        for canvas_width in [0.0, 1280.0] {
            let svg = render_sequence_view_svg(&prepared, &LIGHT, canvas_width, 900.0);
            let tree = xml::parse(&svg);
            let view_box: Vec<f64> = tree.attrs["viewBox"]
                .split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect();
            let content = tree
                .children
                .iter()
                .find(|node| node.attrs.get("class").map(String::as_str) == Some("viz-root"))
                .unwrap();
            let frame = &content.children[0];
            assert_eq!(frame.attrs["class"], "view-frame");
            let drawn_title = frame
                .children
                .iter()
                .find(|node| node.attrs.get("class").map(String::as_str) == Some("view-frame-name"))
                .unwrap();
            assert_eq!(drawn_title.text, format!("«view» {title}"));
            let title_x: f64 = drawn_title.attrs["x"].parse().unwrap();
            assert!(view_box[0] <= title_x);
            assert!(
                view_box[0] + view_box[2] >= title_x + minimum_width,
                "title {title} is cropped in viewBox {}",
                tree.attrs["viewBox"]
            );
        }
    }
}

#[test]
fn sequence_view_rendered_svg_matches_the_native_golden() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let prepared_path = manifest_dir.join("tests/fixtures/sequence-view.prepared.json");
    let rendered_path = manifest_dir.join("tests/fixtures/sequence-view.rendered.svg");

    let prepared_json = std::fs::read_to_string(&prepared_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", prepared_path.display()));
    let prepared: PreparedView =
        serde_json::from_str(&prepared_json).expect("fixture must deserialize");
    let rendered_svg = std::fs::read_to_string(&rendered_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", rendered_path.display()));

    let actual_svg = render_sequence_view_svg(&prepared, &LIGHT, 1280.0, 900.0);

    let expected_tree = xml::parse(&rendered_svg);
    let actual_tree = xml::parse(&actual_svg);
    assert_eq!(
        actual_tree, expected_tree,
        "Rust output:\n{actual_svg}\n\nNative golden output:\n{rendered_svg}"
    );
}
