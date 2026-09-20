# elkrs layout

Issues: [#83](https://github.com/elan8/spec42/issues/83), [#118](https://github.com/elan8/spec42/issues/118), [#119](https://github.com/elan8/spec42/issues/119), [#176](https://github.com/elan8/spec42/issues/176)

## Current decision

Native `elkrs` through `crates/diagram_layout` is the layout engine for headless SVG and for
the interactive webview (`spec42/draw` prepare + layout + draw in `diagram_draw`).
`SPEC42_LAYOUT_ENGINE=legacy` declines `spec42/layout` and `spec42/draw`; there is no client
drawing fallback for the five shipped views.

Pinned `elan8/elkrs` revision: `8309be8cf614cfe277c572b28e4f79a1703f8e32` (ELK 0.11.0 compatible,
Apache-2.0; see `THIRD_PARTY_NOTICES.md`).

Do not call `elkrs::layout_json` directly: hierarchical graphs need the `diagram_layout` adapter
(root-authored, root-coordinate edges).

## Remaining work

- elkrs can still disagree with historical ELK.js goldens on multi-port interconnection node
  heights (`timer_interconnection`). Headless and the webview both follow elkrs.
