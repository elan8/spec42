# Diagram renderer

Interaction layer owned by the VS Code package for Spec42 diagram-generator products.
The five shipped views are drawn in Rust (`spec42/draw`); this package mounts that SVG and
keeps zoom, tooltips, and disclosure. Browser / grid / geometry still draw locally with D3.

| View | Layout / draw | Module |
|------|---------------|--------|
| `general-view` | native (`spec42/draw`) | `renderer.ts` mount |
| `interconnection-view` | native (`spec42/draw`) | `renderer.ts` mount |
| `action-flow-view` | native (`spec42/draw`) | `renderer.ts` mount |
| `state-transition-view` | native (`spec42/draw`) | `renderer.ts` mount |
| `sequence-view` | native (`spec42/draw`) | `renderer.ts` mount |
| `browser-view` | D3 collapsible membership tree | `views/standard-views-render.ts` |
| `grid-view` | D3 element table / relationship matrix | `views/standard-views-render.ts` |
| `geometry-view` | D3 provisional 2D scene | `views/standard-views-render.ts` |

Browser and Grid implement the presentation forms described by SysML v2 §9.2.20. Geometry remains provisional: Spec42 does not yet extract and render model-authored spatial coordinates, shapes, orientation, or 3D viewing parameters. Filtered standard views such as case/requirement-style views are projected through `general-view` with filters preserved by the backend.

The renderer consumes the versioned JSON artifact emitted by `generator-plugins/diagram`. Semantic
membership and relationships belong to typed generator queries; this package owns preparation for
catalog views plus interaction on native SVG.

## Notation-neutral theme

Diagrams use a single ink color for nodes and edges. Meaning comes from SysML notation (definition vs usage borders, edge markers, dash patterns), not per-element hues. Filter chips in the VS Code UI may still use colors for discoverability; SVG diagram content does not.

Native SVG uses the Rust `LIGHT`/`DARK` palettes. The webview maps VS Code light/dark body classes
onto those palettes before `spec42/draw`. Catalog views still accept `theme.colorScheme` as below.

### `colorScheme` (catalog views)

Pass via `renderVisualization(..., { theme: { colorScheme } })`:

| Value | Use |
|-------|-----|
| `vscode` | VS Code CSS variables |
| `light` / `dark` | Static hex tokens for tests and export |
| `auto` | `prefers-color-scheme` when `window` exists; else light |

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
drawing lives in `crates/diagram_draw`. Expansion and compartment disclosure are renderer-owned
presentation state held by `renderVisualization` and sent to `spec42/draw` on each redraw.

### Visual review harness

```bash
node scripts/build-visual-harness.mjs      # writes visual-out/ (gitignored)
python3 -m http.server 8731 --directory visual-out
```

Open `harness.html?case=<id>&theme=light|dark[&w=&h=][&chrome=0]`; `harness.html` with no `case`
lists every id. Native views show an inert placeholder here (no language server). Catalog views
still render locally.
