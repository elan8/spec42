//! spec42 #176 phase 2 acceptance test for interconnection-view, mirroring `golden_parity.rs`'s
//! full-tree byte-comparison approach.

use std::path::Path;

use diagram_draw::render_interconnection_view_svg;
use diagram_draw::theme::LIGHT;
use diagram_draw::types::InterconnectionViewGraph;
use diagram_draw::xml;

#[test]
fn interconnection_view_fidelity_rendered_svg_matches_the_native_golden() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixture_path =
        manifest_dir.join("tests/fixtures/interconnection-view-fidelity.laid-out.json");
    let rendered_path =
        manifest_dir.join("tests/fixtures/interconnection-view-fidelity.rendered.svg");

    let fixture_json = std::fs::read_to_string(&fixture_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", fixture_path.display()));
    let graph: InterconnectionViewGraph =
        serde_json::from_str(&fixture_json).expect("fixture must deserialize");
    let rendered_svg = std::fs::read_to_string(&rendered_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", rendered_path.display()));

    let actual_svg = render_interconnection_view_svg(&graph, &LIGHT, 1280.0, 900.0);

    let expected_tree = xml::parse(&rendered_svg);
    let actual_tree = xml::parse(&actual_svg);
    assert_eq!(
        actual_tree, expected_tree,
        "Rust output:\n{actual_svg}\n\nNative golden output:\n{rendered_svg}"
    );
}

#[test]
fn interconnection_structural_markers_match_the_native_golden() {
    let input: InterconnectionViewGraph = serde_json::from_str(include_str!(
        "fixtures/interconnection-view-fidelity.laid-out.json"
    ))
    .unwrap();
    let svg = render_interconnection_view_svg(&input, &LIGHT, 1280.0, 900.0);
    let summary = diagram_draw::svg_markers::summarize_svg_markers(&svg);
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../../vscode/diagram-renderer/src/test-support/golden-parity/interconnection-view.markers.json")).unwrap();
    assert_eq!(
        serde_json::json!({"classCounts":summary.class_counts, "markerIds":summary.marker_ids}),
        expected
    );
}
