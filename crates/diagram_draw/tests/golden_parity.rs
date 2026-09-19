//! spec42 #176 phase 1 acceptance test: a Rust-rendered General View SVG must produce the same
//! structural-marker summary as the golden fixture the TS golden-parity suites
//! (`headless-export.golden-parity.test.ts`, `renderer.golden-parity.test.ts`) already check
//! against -- one golden file, checked from both languages, so this proves the Rust drawing port
//! draws the same thing as the existing D3 pipeline for this fixture, not just something
//! self-consistent.

use std::collections::BTreeMap;
use std::path::Path;

use diagram_draw::render_general_view_svg;
use diagram_draw::svg_markers::{summarize_svg_markers, SvgMarkerSummary};
use diagram_draw::theme::LIGHT;
use diagram_draw::types::GeneralViewGraph;
use diagram_draw::xml;
use serde::Deserialize;

fn render_fixture() -> (GeneralViewGraph, String) {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = manifest_dir.join("tests/fixtures/general-view.laid-out.json");
    let fixture_json = std::fs::read_to_string(&fixture_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", fixture_path.display()));
    let graph: GeneralViewGraph =
        serde_json::from_str(&fixture_json).expect("fixture must deserialize");
    let svg = render_general_view_svg(&graph, &LIGHT, 1280.0, 900.0);
    (graph, svg)
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

    let (_graph, svg) = render_fixture();
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

/// The class/marker-count check above only proves the two pipelines agree on which classes and
/// markers exist and how many of each -- it cannot see geometry, text, styles, data attributes, or
/// ARIA content drifting apart (the exact class of bug a prior review round found: a wrong corner
/// radius, a dropped compartment-disclosure chrome, a missing edge-label attribute -- none of which
/// move a class count). This test instead parses both the Rust output and a real TS/D3-rendered
/// SVG of the identical fixture (`general-view.rendered.svg`, dumped by
/// `dump-general-view-fixture.test.ts` via the same `exportHeadlessSvg` call
/// `headless-export.golden-parity.test.ts` exercises) into trees and asserts full structural
/// equality: same tags in the same order, same attributes (order-independent, since D3 and this
/// crate build attributes in different but equally arbitrary sequences), same text.
#[test]
fn general_view_rendered_svg_matches_the_real_typescript_output() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let rendered_path = manifest_dir.join("tests/fixtures/general-view.rendered.svg");
    let rendered_svg = std::fs::read_to_string(&rendered_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", rendered_path.display()));

    let (_graph, actual_svg) = render_fixture();

    let expected_tree = xml::parse(&rendered_svg);
    let actual_tree = xml::parse(&actual_svg);
    assert_eq!(
        actual_tree, expected_tree,
        "Rust output:\n{actual_svg}\n\nReal TS/D3 output:\n{rendered_svg}"
    );
}
