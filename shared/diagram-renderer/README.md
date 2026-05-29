# Diagram renderer

Shared D3 + ELK renderer for Spec42 General and Interconnection views, used by the VS Code extension and Babel42.

## Notation-neutral theme

Diagrams use a single ink color for nodes and edges. Meaning comes from SysML notation (definition vs usage borders, edge markers, dash patterns), not per-element hues. Filter chips in the VS Code UI may still use colors for discoverability; SVG diagram content does not.

### `colorScheme`

Pass via `renderVisualization(..., { theme: { colorScheme } })`:

| Value | Use |
|-------|-----|
| `vscode` | VS Code webview (default): `var(--vscode-*)` follows editor light/dark |
| `light` / `dark` | Static hex tokens for tests, export, standalone hosts |
| `auto` | `prefers-color-scheme` when `window` exists; else light (Babel42) |

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
