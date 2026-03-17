# SysML Language Server Status and TODO

This document tracks the current state of the project on the path to `1.0`.

## Current Status

Overall state: usable beta for day-to-day VS Code testing, but not yet a `1.0` stable release.

What is already solid:

- Core LSP flows exist: diagnostics, hover, completion, definition, references, rename, formatting, semantic tokens, folding ranges.
- Crash-on-edit behavior has been hardened in the server.
- VS Code startup, restart, and failure messaging are much more explicit.
- VS Code integration tests now fail on real startup/readiness regressions instead of silently passing.
- Test fixtures now support multiple workspace layouts under `vscode/testFixture/workspaces/`.

What still keeps this from `1.0`:

- Some server runtime paths still need defensive cleanup and guard audits.
- Large-workspace behavior needs stronger test coverage and clearer product guarantees.
- Troubleshooting and release-readiness docs are still incomplete.
- Performance behavior under repeated edits and larger workspaces is not yet well bounded.
- Hover and symbol-resolution coverage are still incomplete for many valid SysML symbols in real files.
- Source/diagram synchronization is only one-way in practice today: diagram-to-source works better than source-to-diagram.
- Diagram support is not yet broad or stable enough: some views remain disabled for release and some expected SysML views are still missing.

## Done

- [x] Fix crash-prone incremental edit handling for UTF-16/LSP position updates.
- [x] Add regression coverage for invalid intermediate edits.
- [x] Remove major runtime `unwrap` and panic points from key production paths.
- [x] Improve extension startup, crash, and restart UX.
- [x] Make VS Code tests strict about language server readiness.
- [x] Add restart-recovery regression tests.
- [x] Fix CI webview asset build drift.
- [x] Introduce workspace fixture subfolders for multiple test workspaces.
- [x] Surface workspace indexing limits in extension configuration and warnings.
- [x] Add regression tests for repeated open/edit/close cycles.
- [x] Add regression coverage for semantic tokens after edits.
- [x] Harden multi-file visualization model fetch against partial request failures.
- [x] Add VS Code end-to-end coverage for multi-file references and rename flows.
- [x] Add malformed-range and UTF-16/emoji edit regressions beyond the initial crash fix.
- [x] Harden explorer/model request paths against cancellations and client-not-running failures.
- [x] Add automated packaging layout verification for staged release assets and VSIX contents.
- [x] Surface skipped files and invalid roots during workspace and library indexing instead of silently dropping them.
- [x] Harden visualization restore and update flows against invalid saved URIs and silent refresh failures.
- [x] Improve hover resolution for typed usages and nested symbols, with regression coverage.
- [x] Extend hover coverage to port and attribute type references with semantic typing resolution.
- [x] Add a first source-to-diagram synchronization path by highlighting the best matching graph node for the active editor selection.
- [x] Improve source-to-diagram navigation so editor selections can also center/reveal the corresponding diagram element when available.
- [x] Expose experimental diagram views as an explicit opt-in in the visualizer UI instead of silently hiding them.
- [x] Improve diagram UX with clearer experimental-view messaging in the visualizer itself.
- [x] Tighten interconnection-view root scoping so focused connectors and ports stay inside the selected IBD root.
- [x] Restore clean local VS Code extension-host test runs with cross-platform test workspace server paths.
- [x] Promote `interconnection-view` from experimental opt-in to a release-enabled default view.
- [x] Add General View presets so users can start from readable overview modes before fine-tuning categories.
- [x] Simplify General View controls to presets only, and render packages as visual containers instead of regular nodes.
- [x] Extend the semantic graph so General View can include requirements, use cases, actors, concerns, and state-machine nodes.

## In Progress

- [ ] Finish runtime guard audit across remaining production code paths.
- [ ] Strengthen large-workspace behavior with broader smoke and performance coverage.
- [ ] Validate packaged extension startup against bundled binaries in a clean install flow.
- [ ] Expand hover resolution so common SysML symbol kinds consistently return useful hover content.
- [ ] Improve bidirectional source/diagram synchronization so source selection can focus/highlight the corresponding diagram element.
- [ ] Bring diagram support to release quality: fix disabled views, clarify missing views, and close the biggest layout/routing gaps.
- [x] Improve interconnection-view readability with one-sided ports, compact side labels, stronger port-driven node sizing, and a real-drone release-gate.
- [ ] Audit missing or weak diagram semantics such as connector labels/item types, source-to-diagram focus, and view-specific empty/degraded states.
- [x] Add basic CI-backed export coverage for `interconnection-view` so regressions are caught before release.
- [x] Add a dedicated VS Code `interconnection-view` regression lane with a connector-bearing fixture that asserts exported connector paths and prevents duplicate port body rendering.
- [x] Strengthen `interconnection-view` routing to keep connectors orthogonal, approach side ports horizontally, and verify those invariants in VS Code export tests.
- [x] Add a dedicated real-drone VS Code `interconnection-view` regression lane that gates connector count, side labels, node sizing, and node-avoidance on a representative model.

## Next

### Reliability

- [ ] Audit remaining runtime assumptions in server document, model, and visualization flows.
- [ ] Add regression coverage for hover on real-world symbol varieties (parts, ports, attributes, actions, packages, typed usages).

### Workspace and Performance

- [x] Add a bounded large-workspace smoke test fixture and CI coverage.
- [x] Revisit repeated recomputation in `didChange`.
- [x] Add more explicit indexing progress and partial-result behavior in the explorer UX.
- [ ] Measure and bound end-to-end latency for hover/definition/model refresh in medium and large workspaces.

### UX and Diagrams

- [ ] Add source-to-diagram reveal/highlight for the active symbol or cursor position.
- [x] Audit which SysML view types are expected for `1.0` and classify each as shipped, experimental, or missing.
- [ ] Re-enable release-blocked diagram views only behind passing regression coverage and acceptable layout quality.
- [x] Decide which experimental views can graduate from opt-in to release-enabled defaults.
- [ ] Improve diagram interaction polish: navigation feedback, richer empty/error states, and graduation criteria per view.
- [x] Improve interconnection-view port readability, routing separation, and label clarity enough to use it as a default-quality view.
- [ ] Further improve dense-cluster routing so large real-world IBDs use fewer long detours and clearer connector bundling.
- [ ] Verify that expected SysML element kinds appear in General and diagram views, including requirements, use cases, states, and other missing semantic nodes.

### Docs and Release

- [x] Write a concise troubleshooting guide.
- [x] Add a `1.0` release checklist.
- [x] Document supported workflows and experimental areas clearly.
- [x] Verify packaging expectations for bundled binaries and release artifacts.

## Working Rule Until 1.0

- Every crash gets a regression test.
- Every user-visible failure gets a visible message or state.
- No major new feature outranks a known reliability issue in the edit loop.
