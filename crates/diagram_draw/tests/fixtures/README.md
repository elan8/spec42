# Native rendering goldens

Prepared and laid-out JSON files are renderer inputs. Their SVG goldens are owned by
`diagram_draw`; historical TypeScript capture names do not require reproducing old notation.
Regenerate a changed SVG through the native renderer, for example:

```sh
cargo run --offline -p diagram_draw --example render_draw_input -- crates/diagram_draw/tests/fixtures/action-flow-view.prepared.json crates/diagram_draw/tests/fixtures/action-flow-view.rendered.svg
```

Review the SVG diff and rasterize it with `resvg` for visual inspection before accepting it.
Tests compare parsed XML trees, including geometry, text, hit targets and accessibility attributes.
Source-to-product fixtures remain owned by `cargo snapshot`; never hand-edit their generated sections.

An optional third path writes the structural marker summary using the same renderer:

```sh
cargo run --offline -p diagram_draw --example render_draw_input -- crates/diagram_draw/tests/fixtures/general-view.laid-out.json crates/diagram_draw/tests/fixtures/general-view.rendered.svg vscode/diagram-renderer/src/test-support/golden-parity/general-view.markers.json
```

Regenerate both shared General/Interconnection summaries together with
`node vscode/diagram-renderer/scripts/update-golden-markers.mjs`. The Interconnection summary
uses `interconnection-view-fidelity.laid-out.json`; both summaries have native regression tests.
