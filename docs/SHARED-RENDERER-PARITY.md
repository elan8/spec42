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
| Heuristic connector paths (non-ELK) | No (ELK sections required) | No | — |
| Heuristic node grid when ELK fails | No (empty layout) | No (legacy general also unified ELK) | — |
| BNF connector kinds (`binding-connection`, `interface-connection`) | Yes (`normalizeEdgeKind`) | Yes | — |
| Render only when model ready | Yes (`modelReady` + loading gate) | Partial (legacy skips connectors without ELK) | — |
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
| Relationship edge markers (typing, specializes, hierarchy, redefinition, …) | Yes | Yes | — |
| Package namespace nodes omitted from canvas | Yes | Yes | — |
| Package container frames (multi-package models) | Yes (`drawGeneralPackageContainers`) | No (unified single layout) | cosmetic |
| Heuristic node grid when ELK fails | No (empty layout) | No | — |
| Type filter chips (UI) | No | Yes | legacy-only |
| Cytoscape fallback layout | No | Yes | legacy-only |

**Fixture:** External `sysml-v2-release/.../01-Parts Tree/1d-Parts Tree with Reference.sysml` (def + `ref hitchBall`)  
**Automated:** `shared/diagram-renderer` — def/usage/ref chrome, relationship markers, package frames, `prepareViewData` package groups

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
| Node click highlight + jump to source | Yes | Yes | — |
| Perform-action / I/O badges | No | Yes | legacy-only (for now) |
| Rich action compartment text | Partial | Yes | cosmetic |

**Automated:** `action-flow-view.test.ts` in `shared/diagram-renderer`

---

## State-transition view

| Check | Shared (`views/state-transition.ts`) | Legacy (`state.ts`) | Severity |
|-------|--------------------------------------|---------------------|----------|
| States + transitions | Yes | Yes | — |
| Initial / final pseudostates | Yes | Yes | — |
| ELK layout + edge labels | Yes | Yes | cosmetic |
| Node click highlight + jump to source | Yes | Yes | — |
| Composite state regions | Limited | Yes | cosmetic |
| Self-loop transitions | Yes | Yes | cosmetic |

**Automated:** `state-transition-view.test.ts`

---

## Sequence view

| Check | Shared (`views/sequence.ts`) | Legacy (`sequence.ts`) | Severity |
|-------|------------------------------|------------------------|----------|
| Lifelines + sync messages | Yes | Yes | — |
| Lifeline click + jump to source | Yes | Yes | — |
| D3 column layout (not ELK) | Yes | Yes | — |
| Fragments (alt/opt/loop) | No | Partial | legacy-only |
| Self-messages / return arrows | Partial | Yes | cosmetic |

**Note:** Sequence targets Spec42 `SequenceView` payloads; fragment overlays remain legacy-only for now.

**Automated:** `sequence-view.test.ts`

**Manual (webshop):** Open `orderEventFanout` — click a lifeline header → gold highlight + editor jumps to the lifeline declaration.

---

## Behavior view gap classification (1.0)

| Gap | Severity | 1.0 action |
|-----|----------|------------|
| Action-flow perform / I/O badges | legacy-only | Keep in legacy `activity.ts`; shared path acceptable without badges |
| Action-flow rich compartment text | cosmetic | Shared shows nodes/edges; polish deferred |
| State composite regions | cosmetic | Shared renders states/transitions; nested region chrome deferred |
| Sequence fragments (alt/opt/loop) | legacy-only | Spec42 `SequenceView` payloads use shared lifelines/messages; UML fragments not release-gating |
| Sequence self-messages / returns | cosmetic | Partial in shared; full parity deferred |
| ELK routing differences (all views) | cosmetic | Documented; not a blocker |

**Blockers for default shared renderer:** none identified for any `SYSML_ENABLED_VIEWS` view as of 2026-06-03.

---

## Sign-off summary (2026-06-03)

### Product defaults

- `spec42.visualization.useSharedRenderer` default is **`true`** in [`vscode/package.json`](../vscode/package.json).
- Webview init in [`vscode/src/visualization/htmlBuilder.ts`](../vscode/src/visualization/htmlBuilder.ts) uses the same default when the setting is unset (aligned 2026-06-03).

### Phase 0 — General + interconnection

- **Blockers:** None. ELK-only routing; model-ready gate; BNF connector/relationship normalization.
- **Decision:** Safe to set `spec42.visualization.useSharedRenderer` default to `true` for structural views.

### Phase 1 — IBD projection

- IBD payloads exclude `part def` via `semantic_core`; scoped roots collapse redundant package/view frames in shared renderer.

### Phase 2 — General view (BNF)

- **Done (except 2.3):** Full inventory + [GENERAL-IBD-BNF-SIGNOFF.md](GENERAL-IBD-BNF-SIGNOFF.md); `redefinition` edge marker; multi-package frames; no `fallbackLayout` grid on ELK failure.
- **WONTFIX 1.0:** Annotation nodes (2.3); n-ary hub graphics (binary edges from projection); package tab variant SVG.

### Phase 4 — Notation catalog

- **284** BNF SVGs indexed in [SYSML-NOTATION-INVENTORY.md](SYSML-NOTATION-INVENTORY.md) (regenerate with `SYSML_V2_RELEASE_DIR`).

### Phases 3 — Behavior

- **Blockers:** None for routing all `SYSML_ENABLED_VIEWS` through the shared package.
- **Signed off for 1.0 default path:** Structural + behavior views render via shared package; gaps above are legacy-only or cosmetic.
- **Not required for 1.0:** Full legacy parity (I/O badges, composite regions, sequence fragments) or Phase 3.6 SVG snapshots.
- **Manual (webshop):** `CheckoutPipeline` (action flow), `OrderLifecycleStateMachine` (state), `orderEventFanout` (sequence) — click node/lifeline → gold highlight + editor jumps to correct line.
- **Automated regression:** `shared/diagram-renderer` Vitest per view; `crates/kernel/tests/integration/model.rs` for IBD/general payloads.
