# Shared vs legacy renderer parity

Side-by-side checklist for `spec42.visualization.useSharedRenderer` (General + Interconnection only).  
Normative notation targets: [SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md](SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md).

**How to compare:** Open the same workspace in VS Code, toggle **Spec42 › Visualization: Use Shared Renderer**, reload the visualizer panel for each view.

| Severity | Meaning |
|----------|---------|
| **blocker** | Wrong or missing semantics; must fix before defaulting shared renderer |
| **cosmetic** | Acceptable visual difference |
| **legacy-only** | Feature only in legacy path; not required for Phase 0 exit |

---

## Interconnection view

| Check | Shared | Legacy (`ibd.ts`) | Severity |
|-------|--------|-------------------|----------|
| Nested part containers (dashed frame) | Yes | Yes | — |
| Leaf part compartments (stereo, typed-by, children text) | Yes | Yes | — |
| Port squares on boundaries | Yes | Yes | — |
| Connectors visible (flow / connection / interface / bind) | Yes (after `kind` fix + edge `style()`) | Yes | — |
| ELK hierarchical layout | Yes | Yes | cosmetic (routing may differ) |
| Single outer boundary for scoped instance root | Yes (package wrapper + view frame collapsed) | Partial | — |
| Degraded routing console diagnostics | No | Yes | legacy-only |
| Port-side heuristics maturity | Good | More tuned in legacy | cosmetic |

**Fixture:** [vscode/testFixture/workspaces/timer/KitchenTimer.sysml](../vscode/testFixture/workspaces/timer/KitchenTimer.sysml)  
**Automated:** `lsp_sysml_model_ibd_kitchen_timer_interface_connects_produce_connectors` in `crates/kernel/tests/integration/model.rs`

---

## General view

| Check | Shared | Legacy (`generalView.ts`) | Severity |
|-------|--------|---------------------------|----------|
| SysML compartments (header / attrs / parts / ports) | Yes | Yes | — |
| Def solid sharp / usage round / ref dotted | Yes | Partial (defs were dashed before shared chrome) | — |
| Relationship edge markers (typing, specializes, hierarchy, …) | Yes | Yes | cosmetic |
| Package namespace nodes omitted from canvas | Yes | Yes | — |
| Package container frames | Partial | Yes | cosmetic |
| Type filter chips (UI) | No | Yes | legacy-only |
| Cytoscape fallback layout | No | Yes | legacy-only |

**Fixture:** External `sysml-v2-release/.../01-Parts Tree/1d-Parts Tree with Reference.sysml` (def + `ref hitchBall`)

---

## Theme / hosts

| Check | Shared | Notes |
|-------|--------|-------|
| Light theme strokes | Yes | `colorScheme: light` |
| Dark theme strokes (Babel42) | Yes | Inline edge `style()` + `app.css` dark rules |
| VS Code theme vars | Yes | `colorScheme: vscode` |

---

## Phase 0 sign-off (2026-05-29)

- **Blockers:** None identified after IBD `kind` ReferenceError fix, connector `style()` stroke, and `nodeBodyChromeStyle` centralization.
- **Decision:** Safe to set `spec42.visualization.useSharedRenderer` default to `true`.
- **Follow-up:** Phase 1 removes `part def` from IBD payloads (semantic_core), not a renderer-only gap.
