# General view element audit (Phase 2.1)

**Phase 2 status:** Done (2026-06-01) — BNF sign-off in [GENERAL-IBD-BNF-SIGNOFF.md](GENERAL-IBD-BNF-SIGNOFF.md); annotations (2.3) WONTFIX for 1.0. See [SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md](SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md#phase-2--general-view-completeness).

Maps SysML v2 **general-view** element kinds to Spec42 canvas behavior. Normative figures: `SysML-v2-Release/bnf/images/` (e.g. `part-def.svg`, `part.svg`, `part-ref.svg`).

**Pipeline:** `semantic_core` → `canonical_general_view_graph` ([`model_projection.rs`](../crates/semantic_core/src/semantic/model_projection.rs)) → `prepareGraph` ([`prepare.ts`](../shared/diagram-renderer/src/prepare.ts)) → shared `renderer.ts`.

| Element kind (examples) | Canvas node | Compartment-only | Omitted | Filter owner |
|-------------------------|-------------|------------------|---------|--------------|
| `package`, `library package` | No | — | Yes | `prepareGraph` / `isPackageElementType` |
| `import` | No | — | Yes | `isNonDiagramSemanticElementType` |
| `part def`, `port def`, `attribute def`, … | Yes | — | No | — |
| `part`, `port`, `attribute` (usage) | Yes | — | No | — |
| `ref`, `part-ref` | Yes | — | No | `isReferenceKind` chrome |
| `in out parameter`, `parameter` | No | Folded into owner attrs | Yes | `canonical_general_view_graph` |
| Anonymous redefinition stub (`name` empty + `redefines`) | No | — | Yes | `canonical_general_view_graph` |
| Nested usages under owner | Often folded | `generalViewParts` compartment | — | `fold_general_view_leaf_details_into_owners` |
| `requirement def` / `requirement` | Yes | — | No | — |
| `action def` / `action` | Yes | — | No | — |
| `state def` / `state` | Yes | — | No | — |
| Comment / annotation | No | — | Deferred (Phase 2.3 WONTFIX) | — |

## Relationship edges (general view)

Rendered via `applyEdgeMarker` in shared renderer. See [SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md](SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md) Phase 2.2.

| Edge kind | Status |
|-----------|--------|
| `typing` | Implemented (open arrow, dash) |
| `specializes` | Implemented (hollow triangle) |
| `hierarchy` / `contains` | Implemented (diamond) |
| `dependency` | Implemented (open arrow, dash) |
| `allocate` | Implemented |
| `satisfy` / `verify` | Implemented |
| `bind` | Implemented |
| `composition` | Mapped via hierarchy |
| `redefinition` | Implemented (open arrow + specializes marker, dash) |
| `usage` | Implemented (open arrow, dash) |

## Package frames

Multi-package general graphs: `prepareGraph` builds `meta.packageContainerGroups` from `qualifiedName` prefixes; `drawGeneralPackageContainers` renders dashed frames per [package-with-name-inside.svg](https://github.com/Systems-Modeling/SysML-v2-Release/blob/master/bnf/images/package-with-name-inside.svg).

## Tests

- `canonical_general_view_graph_filters_parameter_nodes` — parameters not on canvas
- `canonical_general_view_graph_filters_anonymous_redefinition_stubs`
- `canonical_general_view_graph_retains_def_usage_ref_nodes` — def/usage/ref stay visible

## Legacy-only (not general-view renderer)

- Type filter chips (`generalView.ts` UI)
- Cytoscape fallback layout
