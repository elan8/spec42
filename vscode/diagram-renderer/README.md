# Diagram renderer

Interaction layer owned by the VS Code package for Spec42 diagram-generator products.
Every shipped view is drawn in Rust (`spec42/draw`); this package mounts that SVG and
keeps zoom, tooltips, and disclosure.

| View | Layout / draw | Module |
|------|---------------|--------|
| `general-view` | native (`spec42/draw`) | `renderer.ts` mount |
| `interconnection-view` | native (`spec42/draw`) | `renderer.ts` mount |
| `action-flow-view` | native (`spec42/draw`) | `renderer.ts` mount |
| `state-transition-view` | native (`spec42/draw`) | `renderer.ts` mount |
| `sequence-view` | native (`spec42/draw`) | `renderer.ts` mount |
| `browser-view` | native (`spec42/draw`) membership tree | `renderer.ts` mount |
| `grid-view` | native (`spec42/draw`) table / relationship matrix | `renderer.ts` mount |
| `geometry-view` | native (`spec42/draw`) provisional 2D scene | `renderer.ts` mount |

Browser and Grid implement the presentation forms described by SysML v2 §9.2.20. Geometry remains provisional: Spec42 does not yet extract and render model-authored spatial coordinates, shapes, orientation, or 3D viewing parameters. Filtered standard views such as case/requirement-style views are projected through `general-view` with filters preserved by the backend.

The renderer consumes the versioned JSON artifact emitted by `generator-plugins/diagram`. Semantic
membership and relationships belong to typed generator queries; this package owns interaction on
native SVG. Browser hierarchy collapse is renderer-owned presentation state sent to `spec42/draw`
the same way General View expansion is.

## Notation-neutral theme

Diagrams use a single ink color for nodes and edges. Meaning comes from SysML notation (definition vs usage borders, edge markers, dash patterns), not per-element hues. Filter chips in the VS Code UI may still use colors for discoverability; SVG diagram content does not.

Native SVG uses the Rust `LIGHT`/`DARK` palettes. The webview maps VS Code light/dark body classes
onto those palettes before `spec42/draw`.

### Structure CSS classes (SysML v2 graphical notation)

- `viz-node--definition` — solid border, sharp corners (`rx` 0)
- `viz-node--usage` — solid border, rounded corners
- `viz-node--reference` — dotted border (`2,4`), rounded corners
- `viz-node--container` — dashed border `4,4` (IBD part usage frames)
- `viz-node--unsupported` — deliberately non-normative dashed chrome

The schema-v2 diagram product publishes the closed notation role consumed by
`src/node-notation.ts` (`resolveNodeChrome`). String decoding is confined to the legacy payload
adapter.

### Node chrome

`src/sysml-node-builder.ts` still owns General View node measurement used by prepare. Native
drawing lives in `crates/diagram_draw`. Expansion, compartment disclosure, and Browser row
collapse are renderer-owned presentation state held by `renderVisualization` and sent to
`spec42/draw` on each redraw.

### Visual review harness

```bash
node scripts/build-visual-harness.mjs      # writes visual-out/ (gitignored)
python3 -m http.server 8731 --directory visual-out
```

Open `harness.html?case=<id>&theme=light|dark[&w=&h=][&chrome=0]`; `harness.html` with no `case`
lists every id. Native views show an inert placeholder here (no language server).
