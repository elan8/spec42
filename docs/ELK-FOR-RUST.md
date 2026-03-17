# ELK for Rust

## Current Direction

`sysml-language-server` integrates [`elan8/elk-rust`](https://github.com/elan8/elk-rust) directly in the backend layout path for release-enabled diagram views.

The current backend split is:

- `server`
  - builds semantic and view-specific DTOs
- `sysml-diagrams`
  - builds SysML-specific `DiagramGraph` scenes and view policies
- `sysml-diagrams` (includes layout + SVG rendering)
  - adapts `DiagramGraph` to `elk-rust`
  - maps the laid out graph back to `DiagramLayout`
  - computes metrics
  - renders SVG and hit regions

This keeps the public Rust API and the LSP payload shape stable while replacing the hand-written layout engine with a layered engine specialized for graph layout.

## Why This Approach

The project already had the right architectural boundary for backend-rendered diagrams. The missing piece was a stronger generic layout engine for:

- compound graphs
- orthogonal routing
- port constraints and port ordering
- repeatable layout quality on real SysML fixtures

`elk-rust` gives us those building blocks without requiring a JVM runtime or moving rendering back into the client.

## Scope of the Integration

Phase 1 targets:

- `general-view`
- `interconnection-view`

The integration currently focuses on:

- node placement
- compound layout
- orthogonal edge routing
- port-side and port-order constraints
- report and warning propagation into rendered diagrams

Known limitations we still surface as warnings when relevant:

- cross-hierarchy routing is still maturing
- engine-driven edge label placement is partial

## Adapter Responsibilities

`sysml-diagrams` is the stable crate boundary for the rest of the workspace. Its responsibilities are now:

1. normalize the SysML scene graph
2. translate nodes, ports, edges, and config into `elk-rust`
3. run `elk-layered`
4. map bounds and routed edge sections back to `DiagramLayout`
5. compute metrics and phase summaries
6. render SVG and hit regions

That keeps `sysml-diagrams` focused on SysML semantics instead of layout internals.

## Operational Notes

- `elk-rust` now lives in this repository as the `vendor/elk-rust` Git submodule.
- Workspace dependencies point at the local submodule paths:
  - `vendor/elk-rust/crates/elk-core`
  - `vendor/elk-rust/crates/elk-layered`
- The submodule is intentionally pinned to a specific commit; update it deliberately when taking in upstream changes.
- `LayoutPipelineReport` remains the main debug artifact in tests and CI.
- SVG generation lives in `sysml-diagrams`, so the webview and export contracts stay stable.

## Updating `elk-rust`

To pull a newer upstream revision into this workspace:

```bash
git -c safe.directory=$PWD/vendor/elk-rust -C vendor/elk-rust fetch origin
git -c safe.directory=$PWD/vendor/elk-rust -C vendor/elk-rust checkout <commit>
git add vendor/elk-rust
```

If we need to fix `elk-rust` locally first, make the change in `vendor/elk-rust`, commit there, and then update the submodule pointer in the parent repository.

## SVG Regression Testing

The server integration tests write backend-rendered SVG for manual inspection under:

- `server/tests/output/general-view-full-drone.svg`
- `server/tests/output/interconnection-view-full-drone.svg`

Rust integration tests now focus on semantic checks instead of exact SVG string comparison:

- required key nodes are present
- non-structural nodes are excluded from structural General View
- expected ports and routed connections are present in Interconnection View
- minimum counts for structural edges and ports remain intact

## Near-Term Follow-up

- tighten regression gates for the existing drone and interconnection fixtures
- reduce legacy synthetic layout-hint usage where ELK ordering is sufficient
- extend fixture coverage for nested compounds and dense connector sets
- revisit edge label placement as `elk-rust` matures further
