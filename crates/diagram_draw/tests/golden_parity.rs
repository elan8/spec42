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
use serde::Deserialize;

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
    let fixture_path = manifest_dir.join("tests/fixtures/general-view.laid-out.json");
    let golden_path = manifest_dir.join(
        "../../vscode/diagram-renderer/src/test-support/golden-parity/general-view.markers.json",
    );

    let fixture_json = std::fs::read_to_string(&fixture_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", fixture_path.display()));
    let graph: GeneralViewGraph =
        serde_json::from_str(&fixture_json).expect("fixture must deserialize");

    let svg = render_general_view_svg(&graph, &LIGHT, 1280.0, 900.0);
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
