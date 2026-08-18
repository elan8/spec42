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

Logic lives in `src/node-notation.ts` (`resolveNodeChrome`).

### Rebuild VS Code webview

After changing renderer sources, run from `vscode/`:

```bash
npm run build:diagram-webview
```

`media/diagram-viewer.js` is generated for extension packaging and is gitignored.
