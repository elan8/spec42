# Diagram renderer

Shared D3 + ELK renderer for Spec42 **SysML visualizer views** (`SYSML_ENABLED_VIEWS`), used by the VS Code extension. Embedding hosts may consume a subset of views (for example **general** and **interconnection** only).

| View | Layout | Module |
|------|--------|--------|
| `general-view` | ELK | `renderer.ts` |
| `interconnection-view` | ELK hierarchical | `renderer.ts` |
| `action-flow-view` | ELK layered | `views/action-flow.ts` |
| `state-transition-view` | ELK | `views/state-transition.ts` |
| `sequence-view` | D3 columns | `views/sequence.ts` |
| `browser-view` | D3 rows | `views/standard-views.ts` |
| `grid-view` | D3 grid | `views/standard-views.ts` |
| `geometry-view` | D3 provisional scene | `views/standard-views.ts` |

Browser, Grid, and Geometry are exposed as provisional standard-view renderers until the upstream graphical notation gaps are closed. Filtered standard views such as case/requirement-style views are projected through `general-view` with filters preserved by the backend.

For the full picture (legacy vs shared vs SysML v2 spec) and the conformance roadmap, see [`docs/architecture/SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md`](../../docs/architecture/SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md).

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
npm run build:webview
```

`media/webview/visualizer.js` is gitignored; the bundle must be rebuilt locally.
