//! Native General View goldens. The marker summary and parsed SVG tree pin the public
//! drawing contract, including geometry, text, styles, hit targets and accessibility attributes.
//! Regenerate through render_draw_input, then review SVG and PNG output (fixtures/README.md).

use std::collections::BTreeMap;
use std::path::Path;

use diagram_draw::render_general_view_svg;
use diagram_draw::svg_markers::{summarize_svg_markers, SvgMarkerSummary};
use diagram_draw::theme::LIGHT;
use diagram_draw::types::GeneralViewGraph;
use diagram_draw::xml;
use serde::Deserialize;

fn render_fixture(name: &str) -> (GeneralViewGraph, String) {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = manifest_dir.join(format!("tests/fixtures/{name}.laid-out.json"));
    let fixture_json = std::fs::read_to_string(&fixture_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", fixture_path.display()));
    let graph: GeneralViewGraph =
        serde_json::from_str(&fixture_json).expect("fixture must deserialize");
    let svg = render_general_view_svg(&graph, &LIGHT, 1280.0, 900.0);
    (graph, svg)
}

fn assert_matches_native_golden(name: &str) {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let rendered_path = manifest_dir.join(format!("tests/fixtures/{name}.rendered.svg"));
    let rendered_svg = std::fs::read_to_string(&rendered_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", rendered_path.display()));

    let (_graph, actual_svg) = render_fixture(name);

    let expected_tree = xml::parse(&rendered_svg);
    let actual_tree = xml::parse(&actual_svg);
    assert_eq!(
        actual_tree, expected_tree,
        "Rust output:\n{actual_svg}\n\nNative golden output:\n{rendered_svg}"
    );
}

#[derive(Deserialize)]
struct GoldenSummary {
    #[serde(rename = "classCounts")]
    class_counts: BTreeMap<String, u32>,
    #[serde(rename = "markerIds")]
    marker_ids: Vec<String>,
}

#[test]
fn general_view_structural_markers_match_the_golden_fixture() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let golden_path = manifest_dir.join(
        "../../vscode/diagram-renderer/src/test-support/golden-parity/general-view.markers.json",
    );

    let (_graph, svg) = render_fixture("general-view");
    let actual = summarize_svg_markers(&svg);

    let golden_json = std::fs::read_to_string(&golden_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", golden_path.display()));
    let golden: GoldenSummary =
        serde_json::from_str(&golden_json).expect("golden fixture must deserialize");

    assert_eq!(
        actual,
        SvgMarkerSummary {
            class_counts: golden.class_counts,
            marker_ids: golden.marker_ids
        },
        "Rust-rendered General View SVG:\n{svg}"
    );
}

/// Marker counts cannot detect geometry, text or ARIA changes. Compare the complete native
/// output with its reviewed SVG golden as a parsed XML tree as well.
#[test]
fn general_view_rendered_svg_matches_the_native_golden() {
    assert_matches_native_golden("general-view");
}

/// `general-view` (above) only has a plain two-node/one-edge `typing` graph -- it never exercises
/// a collapsed compartment's disclosure chrome, a visible non-generic edge label, a `specializes`
/// edge (whose marker branch also re-sets `stroke-width`, so it separately guards
/// `Element`'s set-vs-append semantics), or a header fill on a non-zero-radius node. This fixture
/// (the checked-in laid-out renderer input) exists specifically to
/// exercise those paths.
#[test]
fn general_view_fidelity_rendered_svg_matches_the_native_golden() {
    assert_matches_native_golden("general-view-fidelity");
}
