//! spec42 #176 phase 2 acceptance test for sequence-view, mirroring `golden_parity.rs`'s
//! full-tree byte-comparison approach: parse both the Rust output and a real TS/D3-rendered SVG of
//! the identical fixture into trees and assert full structural equality.

use std::path::Path;

use diagram_draw::behavior_common::PreparedView;
use diagram_draw::render_sequence_view_svg;
use diagram_draw::theme::LIGHT;
use diagram_draw::xml;

#[test]
fn sequence_view_rendered_svg_matches_the_real_typescript_output() {
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
        "Rust output:\n{actual_svg}\n\nReal TS/D3 output:\n{rendered_svg}"
    );
}
