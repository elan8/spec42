//! The checked-in diagram products pin every view family and recovery shape of the wire contract.

use std::fs;
use std::path::Path;

use diagram_product::wire::DiagramProduct;

const FIXTURES: &[&str] = &[
    "diagram_general_complete.md",
    "diagram_general_unresolved.md",
    "diagram_interconnection_complete.md",
    "diagram_action_flow_complete.md",
    "diagram_state_transition_complete.md",
    "diagram_sequence_complete.md",
    "diagram_browser_complete.md",
    "diagram_grid_complete.md",
    "diagram_geometry_incomplete.md",
];

#[test]
fn every_view_family_matches_the_published_wire_contract() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/snapshots/generation");
    for fixture in FIXTURES {
        let contents = fs::read_to_string(root.join(fixture)).expect(fixture);
        let mut lines = contents.lines();
        assert!(lines.any(|line| line == "## diagram.json"), "{fixture}");
        assert_eq!(lines.next(), Some("~~~json"), "{fixture}");
        let json = lines
            .take_while(|line| *line != "~~~")
            .collect::<Vec<_>>()
            .join("\n");
        let product: DiagramProduct =
            serde_json::from_str(&json).unwrap_or_else(|error| panic!("{fixture}: {error}"));
        assert_eq!(
            product.schema_version,
            diagram_product::SCHEMA_VERSION,
            "{fixture}"
        );
    }
}
