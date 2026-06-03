# Legacy SysML renderer retirement

The SysML visualizer no longer exposes or routes through legacy SysML renderers. All `SYSML_ENABLED_VIEWS` render through `shared/diagram-renderer`.

## Retired SysML modules

Deleted from `vscode/src/visualization/webview/renderers/`:

| Former file | View | Shared replacement |
|------|------|-------------------|
| `ibd.ts` | Interconnection | `shared/diagram-renderer` interconnection |
| `activity.ts` | Action flow | `shared/diagram-renderer/views/action-flow.ts` |
| `state.ts` | State transition | `shared/diagram-renderer/views/state-transition.ts` |
| `sequence.ts` | Sequence | `shared/diagram-renderer/views/sequence.ts` |

`generalView.ts` remains only for Spec42 extension views: `software-module-view` and `software-dependency-view`. It is not a SysML fallback.

## Product behavior

- `spec42.visualization.useSharedRenderer` has been removed.
- The webview initializes without a shared-renderer feature flag.
- `orchestrator.ts` routes every SysML view in `SYSML_ENABLED_VIEWS` to `renderSharedView()`.
- Browser, Grid, and Geometry views are standard-view placeholders with provisional shared renderers while graphical notation details remain unsettled upstream.

## Guardrails

1. Do not reintroduce SysML renderer branches under `vscode/src/visualization/webview/renderers/`.
2. Keep notation rules in `shared/diagram-renderer`.
3. Keep software-specific visualizations outside `SYSML_ENABLED_VIEWS`.
4. Add shared renderer Vitest coverage for every new SysML notation rule.
