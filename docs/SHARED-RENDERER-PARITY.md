# Shared vs legacy renderer parity

Side-by-side checklist for `spec42.visualization.useSharedRenderer` (all **`SYSML_ENABLED_VIEWS`** when the flag is on).  
Normative notation targets: [SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md](SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md).

**How to compare:** Open the same workspace in VS Code, toggle **Spec42 › Visualization: Use Shared Renderer**, reload the visualizer panel for each view. Set the flag to `false` to exercise legacy `renderers/*.ts` fallbacks.

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

## Action-flow view

| Check | Shared (`views/action-flow.ts`) | Legacy (`activity.ts`) | Severity |
|-------|--------------------------------|------------------------|----------|
| ELK layered layout | Yes | Yes | cosmetic |
| Initial / final / decision / fork nodes | Yes | Yes | — |
| Control-flow edges | Yes | Yes | cosmetic |
| Perform-action / I/O badges | No | Yes | legacy-only (for now) |
| Rich action compartment text | Partial | Yes | cosmetic |

**Automated:** `action-flow-view.test.ts` in `shared/diagram-renderer`

---

## State-transition view

| Check | Shared (`views/state-transition.ts`) | Legacy (`state.ts`) | Severity |
|-------|--------------------------------------|---------------------|----------|
| States + transitions | Yes | Yes | — |
| Initial / final pseudostates | Yes | Yes | — |
| ELK layout | Yes | Yes (force optional in legacy) | cosmetic |
| Composite state regions | Limited | Yes | cosmetic |
| Self-loop / transition labels | Partial | Yes | cosmetic |

**Automated:** `state-transition-view.test.ts`

---

## Sequence view

| Check | Shared (`views/sequence.ts`) | Legacy (`sequence.ts`) | Severity |
|-------|------------------------------|------------------------|----------|
| Lifelines + sync messages | Yes | Yes | — |
| D3 column layout (not ELK) | Yes | Yes | — |
| Fragments (alt/opt/loop) | No | Partial | legacy-only |
| Self-messages / return arrows | Partial | Yes | cosmetic |

**Note:** Sequence is **experimental** per [SUPPORTED-WORKFLOWS.md](SUPPORTED-WORKFLOWS.md) (Spec42 `SequenceView` payloads).

**Automated:** `sequence-view.test.ts`

---

## Sign-off summary (2026-05-29)

### Phase 0 — General + interconnection

- **Blockers:** None identified after IBD `kind` ReferenceError fix, connector `style()` stroke, and `nodeBodyChromeStyle` centralization.
- **Decision:** Safe to set `spec42.visualization.useSharedRenderer` default to `true` for structural views.

### Phase 1 — IBD projection

- IBD payloads exclude `part def` via `semantic_core`; scoped roots collapse redundant package/view frames in shared renderer.

### Phases 2–3 — General + behavior (baseline)

- **Blockers:** None for routing all `SYSML_ENABLED_VIEWS` through the shared package.
- **Not signed off:** Full legacy parity for behavior notation, general-view BNF checklist, or Phase 3.6 SVG snapshots.
- **Workaround:** Set `useSharedRenderer` to `false` for behavior views if shared output is insufficient.
