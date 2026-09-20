# elkrs layout

Issues: [#83](https://github.com/elan8/spec42/issues/83), [#118](https://github.com/elan8/spec42/issues/118), [#119](https://github.com/elan8/spec42/issues/119), [#176](https://github.com/elan8/spec42/issues/176)

## Current decision

Native `elkrs` through `crates/diagram_layout` is the layout engine for headless SVG
(`diagram_draw` prepare + layout + draw) and for `spec42/layout`. `SPEC42_LAYOUT_ENGINE=legacy`
declines that request so the VS Code webview can keep its in-webview `elkjs` path.

Pinned `elan8/elkrs` revision: `8309be8cf614cfe277c572b28e4f79a1703f8e32` (ELK 0.11.0 compatible,
Apache-2.0; see `THIRD_PARTY_NOTICES.md`).

Do not call `elkrs::layout_json` directly: hierarchical graphs need the `diagram_layout` adapter
(root-authored, root-coordinate edges).

## Remaining work

- [#176](https://github.com/elan8/spec42/issues/176): serve native SVG to the webview over LSP,
  shrink `vscode/diagram-renderer` to interaction, then delete TS/D3 drawing and the webview
  `elkjs` dependency. Until then, `SPEC42_LAYOUT_ENGINE=legacy` remains the decline switch.
- elkrs and ELK.js can still disagree on multi-port interconnection node heights
  (`timer_interconnection`). Headless and `spec42/layout` follow elkrs; the in-webview fallback
  follows elk.js until the webview is on the native path.
