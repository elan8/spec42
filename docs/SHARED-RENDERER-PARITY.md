# Shared renderer retirement parity

This file records the legacy-removal sign-off for SysML visualizer views. The old `spec42.visualization.useSharedRenderer` toggle has been removed; all SysML views render through `shared/diagram-renderer`.

## SysML Views

| View | Shared renderer status | Legacy status |
|------|------------------------|---------------|
| General | SysML compartments, def/usage/ref chrome, relationship markers, package frames | No SysML fallback |
| Interconnection | Hierarchical IBD, usage/reference parts, ports, flow/interface/bind/connectors | Deleted |
| Action Flow | Initial/final/flow-final, decision/merge, fork/join, action/perform nodes, parameter badges | Deleted |
| State Transition | Initial/final/terminate, states, composite regions, entry/do/exit compartments, self-loops | Deleted |
| Sequence | Lifelines, messages, activations, fragments, self messages, return styling | Deleted |
| Browser | Provisional standard-view renderer | N/A |
| Grid | Provisional standard-view renderer | N/A |
| Geometry | Provisional standard-view renderer | N/A |

`software-module-view` and `software-dependency-view` are not SysML standard views and continue to use the separate extension renderer path.

## Remaining Non-Blocking Gaps

| Gap | Status |
|-----|--------|
| Annotation/comment nodes | Deferred |
| N-ary hub graphics | Deferred; current projection uses binary edges |
| Full long-tail per-kind silhouettes | Partial |
| Browser/Grid/Geometry formal notation | Provisional while upstream graphical details settle |
| Extremely dense IBD routing polish | Follow-up quality work |

## Automated Coverage

- `shared/diagram-renderer` Vitest covers structural chrome, IBD connector styles, behavior notation, provisional standard views, export, and click/highlight hooks.
- Rust tests cover view-candidate mapping and graph/payload projection for release-critical views.

## Sign-Off

Shared renderer is now the sole SysML renderer path. Reintroducing legacy SysML renderers requires a new architecture decision and matching shared-renderer regression coverage.
