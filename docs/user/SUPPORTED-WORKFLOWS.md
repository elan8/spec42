# Features and support

What Spec42 supports well, what works with caveats, and what it does not support yet.

## Supported

These workflows are actively hardened and treated as release-gating:

- Editing `.sysml` and `.kerml` files in VS Code without crashing the server
- Diagnostics for invalid intermediate text while typing
- Hover on keywords and indexed symbols
- Go to definition within a file and across indexed workspace files
- Find references across indexed workspace files
- Rename for indexed symbols
- Document symbols and workspace symbol search
- Formatting
- Semantic tokens
- Folding ranges
- Deterministic CLI validation reports for CI (`text`, `json`, `sarif`, `junit`)
- Optional Sysand package-manager detection and dependency-root ingestion
- Generator plugins over the immutable model query API
- Diagram views: the General, Interconnection, Action Flow, State Transition, Sequence, Browser
  and Grid views render through the packaged generator and the VS Code viewer (see
  [Diagram views](DIAGRAM-VIEW.md) for what each shows)

Coverage for the language, validation, views, CLI and Sysand integration is generated from the
project's metadata in the [conformance matrix](../reference/CONFORMANCE-MATRIX.md).

## Works with caveats

These workflows are useful today but have known limits:

- **Workspace indexing in larger repositories.** The extension can truncate discovery per workspace
  folder and file type based on `spec42.workspace.maxFilesPerPattern` (the legacy
  `sysml-language-server.workspace.maxFilesPerPattern` is still supported).
- **Library path indexing.** Useful for hover, definition and completion, but dependent on parser
  coverage and the files available.
- **Geometry views.** Only 2D shapes with authored coordinates are drawn; other exposed elements
  are reported as incomplete.
- **Diagram rendering outside VS Code.** SVG produced by `spec42 diagram` does not yet match the
  VS Code viewer's rendering in every detail.

## Not supported yet

- Full OMG SysML v2 specification coverage; Spec42 supports a well-tested subset
- Deep semantic validation beyond the published typed rules
- Editable table and matrix views
- Python automation APIs and ReqIF, DOORS or Polarion bridges
- Production-grade performance for very large repositories without tuning

Spec42 does not replace Sysand: Sysand is treated as optional package infrastructure.
