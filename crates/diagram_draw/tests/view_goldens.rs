//! Every view kind, from SysML source to SVG: the generated diagram products in
//! `tests/snapshots/generation` go through preparation, layout and drawing, and the result must
//! match the SVG checked in under `tests/fixtures/views`. The hand-written fixtures in
//! `draw_input.rs` start after layout; these catch a change to layout or preparation in any view.

use std::path::Path;

use diagram_draw::{render_svg_from_payload, xml};

/// At least one realistic case per view kind, and the cases each view handles differently.
const VIEWS: &[&str] = &[
    "diagram_general_complete",
    "diagram_general_multiplicity",
    "diagram_occurrence_presentation",
    "diagram_interconnection_complete",
    "diagram_port_families",
    "diagram_connector_labels",
    "diagram_action_flow_complete",
    "diagram_action_control_structures",
    "diagram_composite_action_flow",
    "diagram_action_flow_nested_start_done",
    "diagram_state_transition_complete",
    "diagram_state_transition_guard_effect",
    "diagram_composite_state",
    "diagram_state_accepting_features",
    "diagram_sequence_complete",
    "diagram_sequence_unsupported_details",
    "diagram_browser_complete",
    "diagram_grid_relationship_matrix",
    "diagram_timer_geometry",
    "diagram_filtered_empty",
];

fn generated_diagram(name: &str) -> serde_json::Value {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/snapshots/generation");
    let snapshot = std::fs::read_to_string(root.join(format!("{name}.md")))
        .unwrap_or_else(|err| panic!("failed to read {name}: {err}"));
    let json = snapshot
        .replace("\r\n", "\n")
        .split_once("## diagram.json\n~~~json\n")
        .unwrap_or_else(|| panic!("{name} contains no diagram.json"))
        .1
        .split_once("\n~~~")
        .expect("closed JSON fence")
        .0
        .to_owned();
    serde_json::from_str(&json).unwrap_or_else(|err| panic!("{name}: {err}"))
}

#[test]
fn every_view_kind_renders_its_golden_from_source() {
    let goldens = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/views");
    // After an intended change, `DIAGRAM_DRAW_UPDATE_FIXTURES=1 cargo test -p diagram_draw --test
    // view_goldens` rewrites the goldens. Rasterize the changed ones and look at them.
    let update = std::env::var_os("DIAGRAM_DRAW_UPDATE_FIXTURES").is_some();
    let mut changed = Vec::new();
    for name in VIEWS {
        let svg = render_svg_from_payload(&generated_diagram(name), 1280.0, 900.0)
            .unwrap_or_else(|err| panic!("{name} did not render: {err}"));
        let path = goldens.join(format!("{name}.svg"));
        if update {
            std::fs::create_dir_all(&goldens).unwrap();
            std::fs::write(&path, &svg).unwrap();
        }
        let golden = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("no golden for {name}: {err}"));
        if xml::parse(&svg) != xml::parse(&golden) {
            changed.push(*name);
        }
    }
    assert!(changed.is_empty(), "rendering changed for {changed:?}");
}

#[test]
fn rendering_from_source_is_deterministic() {
    for name in VIEWS {
        let payload = generated_diagram(name);
        let first = render_svg_from_payload(&payload, 1280.0, 900.0).unwrap();
        let second = render_svg_from_payload(&payload, 1280.0, 900.0).unwrap();
        assert_eq!(first, second, "{name} rendered differently twice");
    }
}
