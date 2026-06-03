# Legacy webview renderer sunset plan

The shared diagram renderer (`shared/diagram-renderer`) is the default for all `SYSML_ENABLED_VIEWS` when `spec42.visualization.useSharedRenderer` is true (product default as of 2026-06-03). Legacy modules under `vscode/src/visualization/webview/renderers/` remain as fallbacks only.

## Current legacy modules

| File | View | Shared replacement |
|------|------|-------------------|
| `generalView.ts` | General | `shared/diagram-renderer` general + `prepareGraph` |
| `ibd.ts` | Interconnection | `shared/diagram-renderer` interconnection + `prepareInterconnection` |
| `activity.ts` | Action flow | `shared/diagram-renderer/views/action-flow.ts` |
| `state.ts` | State transition | `shared/diagram-renderer/views/state-transition.ts` |
| `sequence.ts` | Sequence | `shared/diagram-renderer/views/sequence.ts` |

Orchestrator: [`vscode/src/visualization/webview/orchestrator.ts`](../vscode/src/visualization/webview/orchestrator.ts) branches on `useSharedRenderer` and `SYSML_ENABLED_VIEWS`.

## Removal criteria (all must be true)

1. [SHARED-RENDERER-PARITY.md](SHARED-RENDERER-PARITY.md) sign-off holds with no **blocker** rows for shipped views.
2. No open regression in `shared/diagram-renderer` Vitest or `crates/kernel/tests/integration/model.rs` visualization tests.
3. Manual webshop fixtures pass click-to-source on the **shared** path (documented in parity doc).
4. At least one release cycle with default `useSharedRenderer: true` and no user-facing rollback request for structural views.

## Removal steps

1. Delete `renderers/generalView.ts`, `ibd.ts`, `activity.ts`, `state.ts`, `sequence.ts`.
2. Remove legacy branches from `orchestrator.ts` (keep ELK worker + shared `renderVisualization` only).
3. Remove `spec42.visualization.useSharedRenderer` setting from `package.json` (always shared) or keep as hidden escape hatch for one release with deprecation notice.
4. Update [vscode/README.md](../vscode/README.md) and [SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md](SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md) to drop legacy references.

## Explicitly retained in legacy until reimplemented in shared (optional post-1.0)

- Action-flow perform / I/O badges (legacy-only today).
- Sequence interaction fragments (alt/opt/loop).
- General view type-filter chips and Cytoscape fallback (host UI concerns, not SVG core).

These are **not** blockers for deleting legacy renderers if the shared path is the only shipped path and the gaps remain documented as WONTFIX or follow-up.
