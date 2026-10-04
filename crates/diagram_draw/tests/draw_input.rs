//! The JSON dispatcher must accept every dumped fixture shape and produce the same SVG the
//! per-view parity tests already check against the real TS/D3 output.

use std::path::Path;

use diagram_draw::render_svg_from_str;
use diagram_draw::xml;

fn fixture(name: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(manifest_dir.join(format!("tests/fixtures/{name}")))
        .unwrap_or_else(|err| panic!("failed to read {name}: {err}"))
}

fn assert_dispatcher_matches_rendered(input_name: &str, rendered_name: &str) {
    let input = fixture(input_name);
    let rendered = fixture(rendered_name);
    let actual = render_svg_from_str(&input, 1280.0, 900.0)
        .unwrap_or_else(|err| panic!("dispatcher failed on {input_name}: {err}"));
    assert_eq!(
        xml::parse(&actual),
        xml::parse(&rendered),
        "dispatcher output for {input_name}:\n{actual}\n\nfixture {rendered_name}:\n{rendered}"
    );
}

#[test]
fn dispatcher_renders_every_dumped_view_fixture() {
    assert_dispatcher_matches_rendered("general-view.laid-out.json", "general-view.rendered.svg");
    assert_dispatcher_matches_rendered(
        "general-view-fidelity.laid-out.json",
        "general-view-fidelity.rendered.svg",
    );
    assert_dispatcher_matches_rendered(
        "interconnection-view-fidelity.laid-out.json",
        "interconnection-view-fidelity.rendered.svg",
    );
    assert_dispatcher_matches_rendered("sequence-view.prepared.json", "sequence-view.rendered.svg");
    assert_dispatcher_matches_rendered(
        "action-flow-view.prepared.json",
        "action-flow-view.rendered.svg",
    );
    assert_dispatcher_matches_rendered(
        "state-transition-view.prepared.json",
        "state-transition-view.rendered.svg",
    );
}

#[test]
fn dispatcher_renders_catalog_views() {
    let browser = render_svg_from_str(
        r#"{"view":"browser-view","title":"Browser","nodes":[{"id":"root","label":"Root","kind":"part"}],"edges":[],"meta":{"rows":[{"id":"root","label":"Root","kind":"part","depth":0,"hasChildren":false}],"hierarchyLayout":true}}"#,
        1280.0,
        900.0,
    )
    .unwrap_or_else(|err| panic!("{err}"));
    assert!(browser.contains("browser-row"), "{browser}");
    assert!(browser.contains("Root"), "{browser}");
}
