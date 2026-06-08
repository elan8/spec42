# Shared diagram renderer and SysML v2 graphical notation

This document records the current SysML visualizer renderer contract. Spec42 now renders every SysML visualizer view through `shared/diagram-renderer`; the VS Code legacy SysML fallback has been retired.

Related docs:

- [`shared/diagram-renderer/README.md`](../../shared/diagram-renderer/README.md)
- [`docs/user/SUPPORTED-WORKFLOWS.md`](../user/SUPPORTED-WORKFLOWS.md)
- [`docs/archive/SHARED-RENDERER-PARITY.md`](../archive/SHARED-RENDERER-PARITY.md)
- [`docs/archive/GENERAL-VIEW-ELEMENT-AUDIT.md`](../archive/GENERAL-VIEW-ELEMENT-AUDIT.md)
- [`docs/reference/SYSML-NOTATION-INVENTORY.md`](../reference/SYSML-NOTATION-INVENTORY.md)
- [`docs/archive/GENERAL-IBD-BNF-SIGNOFF.md`](../archive/GENERAL-IBD-BNF-SIGNOFF.md)
- [`docs/archive/LEGACY-RENDERER-SUNSET.md`](../archive/LEGACY-RENDERER-SUNSET.md)

## Current Renderer Contract

| Layer | Scope |
|-------|-------|
| Shared renderer | All `SYSML_ENABLED_VIEWS`: general, interconnection, action-flow, state-transition, sequence, browser, grid, geometry |
| SysML v2 spec target | Strict standard-view ambition for shipped views, with Browser/Grid/Geometry marked provisional while upstream graphical details settle |

The removed `spec42.visualization.useSharedRenderer` setting must not be reintroduced. New SysML notation belongs in `shared/diagram-renderer`, not in VS Code host-specific renderer branches.

## View Inventory

| View ID | SysML v2 view kind | Renderer | Status |
|---------|--------------------|----------|--------|
| `general-view` | General View | `renderer.ts` + `sysml-node-builder.ts` | Complete for shipped structural workflows |
| `interconnection-view` | Interconnection View | `renderer.ts` IBD path | Complete for shipped structural workflows |
| `action-flow-view` | Action Flow View | `views/action-flow.ts` | Shared behavior renderer with perform-action, parameter, and final-node notation |
| `state-transition-view` | State Transition View | `views/state-transition.ts` | Shared behavior renderer with composite regions, entry/do/exit actions, terminate/final, and self-loop support |
| `sequence-view` | Sequence View | `views/sequence.ts` | Shared behavior renderer with lifelines, messages, activations, fragments, return messages, and self messages |
| `browser-view` | Browser View | `views/standard-views.ts` | Provisional standard-view renderer |
| `grid-view` | Grid View | `views/standard-views.ts` | Provisional standard-view renderer |
| `geometry-view` / `geometric-view` | Geometry View | `views/standard-views.ts` | Provisional standard-view renderer |
| Case-style filtered views | Filtered standard views | mapped to `general-view` | Mapped |

## Routing

`vscode/src/visualization/webview/orchestrator.ts` routes every view in `SYSML_ENABLED_VIEWS` to `renderSharedView()`. There is no SysML legacy fallback.

Visualization payloads are built by `semantic_core` and normalized in `shared/diagram-renderer/src/prepare.ts`:

- `prepareGraph` -> General View and mapped filtered views
- `prepareInterconnection` -> Interconnection View
- `prepareActivity` -> Action Flow View
- `prepareState` -> State Transition View
- `prepareSequence` -> Sequence View
- `prepareBrowser`, `prepareGrid`, `prepareGeometry` -> provisional standard views

## Conformance Notes

| Spec element | Status | Owner |
|--------------|--------|-------|
| Def solid/sharp, usage solid/round, reference dotted/round | Implemented | `node-notation.ts` |
| General compartments and relationship markers | Implemented | `sysml-node-builder.ts`, `renderer.ts` |
| IBD usage-only projection and connectors | Implemented | `semantic_core`, `prepare.ts`, `renderer.ts` |
| Action perform nodes and parameter badges | Implemented | `views/action-flow.ts` |
| State composite regions and entry/do/exit compartments | Implemented | `views/state-transition.ts` |
| Sequence fragments, activations, self/return messages | Implemented | `views/sequence.ts` |
| Browser/Grid/Geometry top-level views | Provisional | `views/standard-views.ts` |
| Annotation/comment nodes and n-ary hub graphics | Deferred | Shared renderer + projection |
| Full per-kind long-tail silhouettes | Partial | `node-notation.ts` |

## Verification

Automated checks:

```bash
cd shared/diagram-renderer && npm test
cargo test -p semantic_core -p kernel
```

Add tests when changing:

- notation chrome or edge styles -> `node-notation.test.ts` / `renderer.test.ts`
- payload shaping -> `prepare.test.ts`
- backend view support -> `explicit_views.rs` tests or kernel visualization tests

Manual acceptance:

- General and Interconnection: validate def/usage/ref chrome, ports, and connectors.
- Action, State, Sequence: validate click-to-source plus behavior-specific notation.
- Browser/Grid/Geometry: validate provisional badge and useful nonblank rendering.

## Guardrails

1. Keep all SysML rendering in `shared/diagram-renderer`.
2. Do not claim full OMG graphical conformance until provisional views and deferred notation are closed.
3. Rebuild the webview bundle after shared renderer or webview TypeScript changes:

```bash
cd vscode && npm run build:webview
```
