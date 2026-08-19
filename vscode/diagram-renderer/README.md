# Diagram renderer

D3 + ELK renderer owned by the VS Code package for Spec42 diagram-generator products.

| View | Layout | Module |
|------|--------|--------|
| `general-view` | ELK | `renderer.ts` |
| `interconnection-view` | ELK hierarchical | `renderer.ts` |
| `action-flow-view` | ELK layered | `views/action-flow.ts` |
| `state-transition-view` | ELK | `views/state-transition.ts` |
| `sequence-view` | D3 columns | `views/sequence.ts` |
| `browser-view` | D3 collapsible membership tree | `views/standard-views-render.ts` |
| `grid-view` | D3 element table / relationship matrix | `views/standard-views-render.ts` |
| `geometry-view` | D3 provisional 2D scene | `views/standard-views-render.ts` |

Browser and Grid implement the presentation forms described by SysML v2 §9.2.20. Geometry remains provisional: Spec42 does not yet extract and render model-authored spatial coordinates, shapes, orientation, or 3D viewing parameters. Filtered standard views such as case/requirement-style views are projected through `general-view` with filters preserved by the backend.

The renderer consumes the versioned JSON artifact emitted by `generator-plugins/diagram`. Semantic
membership and relationships belong to typed generator queries; this package owns only preparation,
layout, interaction, and drawing.

## Notation-neutral theme

Diagrams use a single ink color for nodes and edges. Meaning comes from SysML notation (definition vs usage borders, edge markers, dash patterns), not per-element hues. Filter chips in the VS Code UI may still use colors for discoverability; SVG diagram content does not.

### `colorScheme`

Pass via `renderVisualization(..., { theme: { colorScheme } })`:

| Value | Use |
|-------|-----|
| `vscode` | VS Code webview (default): `var(--vscode-*)` follows editor light/dark |
| `light` / `dark` | Static hex tokens for tests, export, standalone hosts |
| `auto` | `prefers-color-scheme` when `window` exists; else light (headless hosts) |

Hosts that embed exported SVG outside VS Code should use `light` or `dark`, not `vscode`, so strokes are real colors.

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

`src/sysml-node-builder.ts` owns every coordinate inside a General View node. `layoutSysMLNode`
computes the header regions and compartment blocks once; `computeNodeHeight`/`computeNodeWidth`
(ELK sizing, in `render/layout.ts`) and `renderSysMLNode` (drawing) both read that same layout, so
a node cannot be measured at one size and painted at another. Header height and node width are
derived from header and compartment content, not from fixed coordinates.

The header fill is a path from `headerFillPath` (`src/node-notation.ts`) that follows the body's
own rounded corners inset by half the border stroke, so the outer border stays visually continuous
and the header never introduces corners of its own.

Expansion and compartment disclosure are renderer-owned presentation state held by
`renderVisualization` for the lifetime of a controller. It reaches layout and drawing only through
the projected node attributes `disclosure`, `hiddenRelationshipCount` and
`compartmentSectionState`; `RenderOptions.disclosure` carries toggles back. Nothing in the node
chrome infers model semantics.

### Visual review harness

```bash
node scripts/build-visual-harness.mjs      # writes visual-out/ (gitignored)
python3 -m http.server 8731 --directory visual-out
```

Open `harness.html?case=<id>&theme=light|dark[&w=&h=][&chrome=0]`; `harness.html` with no `case`
lists every id. The corpus is every checked-in repository diagram product
(`tests/snapshots/generation/diagram_*.md`) plus the authored node-chrome stress cases in
`visual/synthetic-cases.ts`. The page sets `data-visual-ready="1"` once layout, drawing, fitting
and any scripted disclosure activation have settled, so a screenshot driver never captures a
partial frame.

`src/visual-corpus.test.ts` renders the same corpus headlessly and asserts the geometry invariants
the screenshots are reviewed for (header fill inside the border, dividers inside the boundary,
non-overlapping header regions, minimum pointer target, no non-finite coordinates).

### Golden marker fixtures

`src/test-support/golden-parity/*.markers.json` are derived from renderer output. Rebuild them with
`node scripts/update-golden-markers.mjs` when a chrome or marker change is intentional; do not edit
them by hand.

### Rebuild VS Code webview

After changing renderer sources, run from `vscode/`:

```bash
npm run build:diagram-webview
```

`media/diagram-viewer.js` is generated for extension packaging and is gitignored.
