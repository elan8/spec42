# View Rendering Issues Tracker

Living tracker for bugs, gaps, and improvement ideas in diagram/view rendering
(Interconnection View, general/definition-usage diagrams, node-body notation).
Not a replacement for [`ibd-interconnection-pipeline-analysis.md`](ibd-interconnection-pipeline-analysis.md)
(architecture/CI-contract reference) or [`reference/SYSML-NOTATION-INVENTORY.md`](reference/SYSML-NOTATION-INVENTORY.md)
(per-diagram-kind notation coverage audit) — this doc is the working bug/issue list that
points at those for detail.

Convention: newest entries at the top of each section. Mark an item `Fixed` in place
(with commit/date) rather than deleting it, so the history of what broke stays visible.

---

## Refactor plan — remaining work (not yet scheduled)

Phases 0–8 of the view-projection/rendering refactor plan are done (see F-11 through F-18 below).
The following were explicitly deferred, not forgotten — keeping them here so they don't get lost.

### Phase 9 (not started): TS — converge the 3 rendering strategies
- **Scope:** eliminate the "shadow" behavior-family rendering stack
  (`views/behavior-common.ts` + `action-flow.ts`/`state-transition.ts`/`sequence.ts`, ~1150 lines) —
  its own ELK instance, its own node-sizing logic, its own draw-call convention, its own color/theme
  resolution — running parallel to `render/layout.ts` + `render/drawing.ts` (~1200 lines) as a
  second complete layout+draw pipeline.
- **Size/risk:** large, multi-week — the single highest-value item in the whole audit, but must be
  sub-phased (unify node sizing → unify draw-call convention → route through the shared theme
  resolver → only then retire the second ELK instance), keeping both stacks live and switchable
  until each kind is CLI-verified. **Needs its own dedicated plan when picked up** — don't try to
  fold it into a quick session.

### Phase 10 (not started): Rust — DTO shape convergence
- **Scope:** replace `SysmlVisualizationResultDto`'s 18 `Option<...>` fields (`dto.rs:228-293`)
  with a per-kind sub-DTO or enum variant, and wire TS's hand-declared `prepare/types.ts` interfaces
  to the `ts-rs`-generated bindings instead of maintaining a parallel, drift-prone copy.
- **Size/risk:** large, multi-week, breaking change to the golden-file `ts-rs` bindings and every
  DTO consumer (webview, CLI `--format json`, any external tooling). **Do not schedule concrete
  work without a separate design pass on the enum-variant shape first.**

### Follow-up on Phase 7 (done, but with a known verification gap)
- Phase 7 gated `general_view_graph` computation per view kind ([F-17](#f-17-phase-7-of-the-view-pipeline-refactor-plan--skip-general_view_graph-computation-for-kinds-that-dont-use-it-partial-ibd-skipping-deferred)).
  It was verified via CLI export diffing and by tracing every TS consumer directly, but the plan
  also called for a manual VS Code webview spot-check, which wasn't possible in this (headless)
  session. **Action:** before shipping, open general-view, browser-view, and interconnection-view
  on a real workspace in the actual VS Code extension and confirm they render correctly.
- [O-7](#o-7-ibdfiltered_ibd-computed-unconditionally-for-every-view-kind-despite-zero-direct-ts-consumers-found)
  is the natural next step after Phase 7 — investigate whether `ibd` computation can also be
  skipped for the 6 kinds that don't use it, once the unexplained `general-view` IBD-scoping
  special case is understood.

---

## Open

### O-7: `ibd`/`filtered_ibd` computed unconditionally for every view kind despite zero direct TS consumers found
- **Where:** `crates/sysml_model/src/semantic/visualization/response.rs`
  (`build_sysml_visualization_from_artifacts`, `ibd_source`/`filtered_ibd` construction).
- **Symptom:** every non-slim response computes and attaches `ibd: Some(filtered_ibd)` regardless
  of view kind, but grepping every `shared/diagram-renderer/src/prepare/*.ts` file and the VS Code
  extension host found **no direct consumer** of `visualization.ibd` for *any* kind — even
  interconnection-view reads the separately-computed `interconnectionScene` field instead (`ibd` is
  only an upstream input used to *build* `interconnection_scene`/`package_container_groups`
  server-side, not something the client re-reads).
- **Why not fixed in [F-17](#f-17-phase-7-of-the-view-pipeline-refactor-plan--skip-general_view_graph-computation-for-kinds-that-dont-use-it-partial-ibd-skipping-deferred):**
  `ibd_source` selection has a `resolved_view == "general-view"` special case that triggers a scoped
  IBD rebuild (`build_merged_workspace_ibd` over `workspace_uris_for_ibd_scope`) whose purpose isn't
  fully understood — it's suspicious given how tricky
  [O-1](#o-1-scopedincremental-ibd-build-can-pick-a-different-but-valid-root-than-the-full-workspace-build)'s
  scoped-vs-full parity gap already proved to be this session. Skipping `ibd` computation for
  non-interconnection kinds without first understanding why general-view gets its own IBD-scoping
  branch risks silently breaking whatever that branch exists for.
- **Suggested next step:** before touching this, find out what (if anything) actually depends on
  `ibd`/`filtered_ibd` being computed for general-view specifically — check git blame/history on
  that branch, or instrument it locally to see whether removing it changes any test outcome. If it
  turns out to be dead (as the TS consumer grep suggests), skipping IBD computation for
  browser/grid/geometry/sequence/state/action-flow (6 of 8 kinds) would be a meaningful perf win,
  mirroring [F-17](#f-17-phase-7-of-the-view-pipeline-refactor-plan--skip-general_view_graph-computation-for-kinds-that-dont-use-it-partial-ibd-skipping-deferred)'s
  `general_view_graph` gating.
- **Discovered:** 2026-07-07, during Phase 7 investigation (deferred rather than acted on).

### O-6: Interconnection-view and action-flow-view layout is non-deterministic run-to-run
- **Where:** unroot-caused — likely `HashMap`/`HashSet` iteration order somewhere upstream of ELK graph
  construction (Rust extraction or TS `prepare`/`layoutPrepared`/`layoutBehaviorGraph`), feeding ELK a
  differently-ordered node list each run.
- **Symptom:** running `spec42 diagrams export examples/webshop --selected-view connections --format svg`
  (interconnection-view) or `--selected-view checkoutPipeline` (action-flow-view) three times back-to-back
  against the *same unchanged binary* produces three SVGs with different MD5 hashes each time — different
  layout coordinates, though the same set of node/edge text content (same count, same strings, just
  different `x`/`y`). `general-view`, `sequence-view`, `state-transition-view`, `browser-view`,
  `grid-view`, `geometry-view` were all confirmed byte-identical across repeated runs in the same check.
- **Confirmed pre-existing, not a regression:** discovered incidentally during Phase 1 dead-code-deletion
  verification of the view-projection refactor plan — reproduced on the unmodified pre-Phase-1 binary, so
  it predates that work.
- **Impact:** makes byte-diff-based regression checking unreliable for these 2 of 8 view kinds
  specifically; future phases must compare structural/text content for interconnection-view and
  action-flow-view rather than raw SVG bytes. Also a latent risk for any consumer expecting stable output
  (e.g. golden-file tests, if any exist for these kinds — check before relying on byte comparison there).
- **Discovered:** 2026-07-07, refactor plan Phase 1 verification.

### O-5: `general-view` can render completely empty for a valid, filter-less `expose` (pre-existing, not a regression)
- **Where:** `crates/sysml_model/src/semantic/visualization/response.rs` general-view construction path
  (unclear exactly which stage yet — not root-caused).
- **Symptom:** `examples/webshop`'s `view structure : GeneralView { expose WebShopExample::webshopSystem; }`
  (no `filter` clause) exports an SVG with zero nodes (`<g class="viz-nodes"></g>` empty, 0 `<text>`
  elements, ~2KB total) via `spec42 diagrams export examples/webshop --selected-view structure --format
  svg`. The *same* exposed root works correctly for `InterconnectionView` (`view connections`, 77 text
  elements rendered) declared right next to it in the same `Views.sysml`, so `webshopSystem` itself is not
  empty — this is specific to the general-view path.
- **Confirmed pre-existing:** reproduced identically on a clean `git stash` baseline (pre-dating all of
  today's O-1/§2/§3 changes), so this is **not** a regression introduced by today's work — logged here as
  newly discovered, not yet fixed.
- **Suggested next step:** since this view has no kind-narrowing `filter` (unlike `productStructure`,
  which triggered [F-7](#f-7-general-view-node-bodies-render-completely-empty-when-the-view-has-a-kind-narrowing-filter)),
  the F-7 `pre_filter_node_ids` fix should not be involved (`pre_filter_node_ids == projected_ids` with no
  filter) — needs fresh root-causing starting from `ProjectedView`/`project_view` for this specific
  exposed id, not assumed to share F-7's cause.
- **Discovered:** 2026-07-07, during §3 CLI verification (see
  [F-10](#f-10-deleted-the-duplicate-rust-prepared_view-preparers-for-6-of-7-view-kinds-root-causes-f-9s-bug-class)).

### O-4: Sprawling layout for a single package with many same-depth siblings
- **Where:** `shared/diagram-renderer/src/render/layout.ts` (`layoutPrepared`, general-view branch).
- **Symptom:** Even after [F-8](#f-8-sprawling-tangled-general-view-layout-partially-fixed-package-hierarchy)
  gave ELK real per-package containment, a package with many nodes at the same tree depth (e.g.
  `PhysicalArchitecture` in `sysml-robot-vacuum-cleaner`, 73 members) still lays out as one very wide
  row *within its own container* (~9200px wide for that package alone in testing).
- **Root cause, confirmed empirically:** ELK's `elk.layered.wrapping.strategy` (`MULTI_EDGE`/`SINGLE_EDGE`)
  was assumed to solve this (see the original plan) but does **not** — it wraps a *long chain of layers*
  into multiple rows/bands; it does not split *one wide layer with many parallel siblings* into a grid.
  Verified with minimal standalone elkjs repros: a 20-node star graph (one root, 20 direct children) with
  wrapping enabled still produces only 2 layout rows (root + one wide children row), not a compact grid.
  The wrapping option was removed from `layout.ts` since it was dead config with zero effect for this
  graph shape.
- **Suggested fix (not yet attempted):** For a package whose direct-children count at one depth exceeds
  some threshold, either (a) manually chunk those children into synthetic sub-groups (their own nested
  ELK containers) to artificially create the layer-chain shape that wrapping *does* handle, or (b) switch
  that specific sub-layout to a grid/box-packing algorithm (ELK's `box` algorithm, or a hand-rolled grid)
  instead of `layered`.
- **Discovered:** 2026-07-07, while verifying [F-8](#f-8-sprawling-tangled-general-view-layout-partially-fixed-package-hierarchy)
  against the real repo.

### O-1: Scoped/incremental IBD build can pick a different (but valid) root than the full-workspace build
- **Where:** `crates/sysml_model/src/semantic/visualization/scope.rs` (`ibd_uri_closure_for_exposed_ids`,
  `workspace_uris_for_ibd_scope` with `IbdBuildScope::ViewExposedPackages`).
- **Symptom:** For some views (confirmed on `sysml-robot-vacuum-cleaner`'s `cleaningHead` view, and
  the `productStructure` general view), the performance-optimized "scoped" build — which only loads a
  subset of workspace files — resolves connectors against a *different* structurally-valid root
  (e.g. the still-abstract `PhysicalArchitecture.AutonomousFloorCleaningRobot.cleaningHead` path)
  than the full-workspace build (`Architecture.CleaningRobotSystemOfInterest.physical.cleaningHead`).
  Both resolve real connectors — this is a scoped-vs-full **parity** gap, not a zero-connector
  regression like [F-4](#f-4-driveModule-interconnection-view-resolved-zero-connectors) below.
- **Root cause (partial):** `collect_definition_uris_for_subtree` (added in F-2/F-4 fix pass) walks
  containment + typing edges from each exposed id to pull in definition files, but doesn't yet chase
  every path a connector's `redefines`/`subsets`-based mirroring can reach, so the scoped file set can
  still miss a file the full-workspace build would have included.
- **Reproduce:** `SYSML_ROBOT_VACUUM_DIR=<checkout> cargo test -p sysml_model --test scoped_ibd_general_view_parity -- --ignored --nocapture`
  (currently fails: `productStructure: filtered IBD connector count mismatch left: 48 right: 38`).
- **Impact:** Only affects the perf-optimized scoped path (`IbdBuildScope::ViewExposedPackages`),
  used for incremental/large-workspace visualization requests — not the default full-workspace path.
- **Suggested fix:** Extend `collect_definition_uris_for_subtree` to also follow `redefines`/`subsets`
  target files, or fall back to full-workspace build whenever the scoped result's root disagrees with
  the full-workspace root for the same exposed id (cheap correctness check, sacrifices some of the
  perf win only on mismatch).
- **Partial fix landed, real repo gap NOT closed:** 2026-07-07 · `collect_definition_uris_for_subtree`
  now also resolves and recurses into `redefines`/`subsets` attribute targets (via a direct
  `node_ids_by_qualified_name` lookup, falling back to `resolve_inherited_member_via_type` for cross-type
  subsets), verified correct with a new synthetic regression test
  (`ibd_uri_closure_follows_subsets_target_into_a_sibling_document`, `scope.rs`). **However**, re-running
  the exact repro above against the real `sysml-robot-vacuum-cleaner` checkout shows the identical failure
  (`left: 48 right: 38`, unchanged) — targeted debug tracing found the actual missing files for this
  specific repo are pulled in via SysML **variation/variant-selection** (`variation part def
  NavigationSensorSuiteChoice :> SensorAssembly { variant ...; }`, resolved through
  `ConfigurationDefinition.selectedModelElementPaths` in `model/variants/ProductVariants.sysml`) — a
  different mechanism entirely, not yet covered by `collect_definition_uris_for_subtree`. Keeping this
  item **Open**: the redefines/subsets fix is real and worth keeping, but a second fix (following variant
  selection paths) is needed to actually close the `productStructure` parity gap.

### O-2: Missing node-body compartments for most non-structural diagram kinds
- **Where:** `shared/diagram-renderer/src/sysml-node-builder.ts` (`renderSysMLNode`), general-view
  projection in `crates/sysml_model/src/semantic/model_projection.rs`.
- **Symptom:** Definition/usage diagram nodes render only **Attributes / Parts / Ports** compartments.
  Per spec, node bodies for other element kinds should also show compartments for actions, states,
  requirements/constraints, items, interfaces, connections, allocations, views — currently these exist
  only as IBD-only "compartment text" stand-ins per the notation inventory, not real structured
  compartments in general-view diagrams.
- **Reference:** Full per-kind breakdown already tracked in
  [`reference/SYSML-NOTATION-INVENTORY.md`](reference/SYSML-NOTATION-INVENTORY.md) — treat that file
  as the source of truth for exactly which compartments are `shared (compartment text only)` vs real.
- **Impact:** Large — this is a feature-completeness gap, not a bug in existing rendering. Scope it as
  its own release, not a hotfix.

### O-3: `root_views` keyed by bare name, not qualified name
- **Where:** `crates/sysml_model/src/semantic/ibd/extract_impl.rs` (`root_views.insert(p.name.clone(), ...)`),
  `crates/sysml_model/src/semantic/ibd/merge.rs` (merge keeps the same bare-name key).
- **Symptom:** Two structurally distinct nodes that happen to share a local name (e.g. a definition-body
  member `PhysicalArchitecture.AutonomousFloorCleaningRobot.driveModule` and an unrelated top-level
  instance also named `driveModule` elsewhere in the workspace) both land under the single `root_views["driveModule"]`
  key. The merge step unions their parts/connectors additively rather than erroring or disambiguating,
  which is safe today only because `select_interconnection_ibd_scope` re-resolves by qualified id — but
  it's a latent footgun for any code path that reads `root_views` directly by name.
- **Suggested fix:** Key `root_views` by qualified name (or a `(name, qualified_name)` pair) and keep a
  secondary bare-name index only for UI dropdown display, so ambiguous names can't silently merge two
  unrelated subtrees.
- **Discovered:** 2026-07-07, while diagnosing [F-4](#f-4-driveModule-interconnection-view-resolved-zero-connectors).

---

## Fixed

### F-20: Sibling subtypes of a shared ancestor cross-contaminate def→instance path mappings — orphan nodes + empty parent containers in Interconnection View
- **Fixed:** 2026-07-07 · `crates/sysml_model/src/semantic/ibd/connectors.rs`
  (`instance_def_mapping_for_part`).
- **Root cause, fully confirmed via live tracing (not just hypothesized):** `instance_def_mapping_for_part`
  builds a `(def_root, instance_root)` mapping from an `IbdPartDto` without checking that the part
  actually represents a *usage* — a `part def` node (a definition, not a usage) can end up as an
  `IbdPartDto` entry when extraction walks a member chain through a usage's inherited type body
  (e.g. resolving `mainElectronics.mainControlPcb` reaches into `MainElectronicsAssembly`'s own
  definition). When that happens, the function produces a nonsense mapping whose "instance" side is
  really a *type's own qualified name* — confirmed live on `sysml-robot-vacuum-cleaner`:
  `(def="PhysicalArchitecture.PhysicalAssembly", instance="PhysicalArchitecture.PowerModule")` and
  `(def="PhysicalArchitecture.PhysicalAssembly", instance="PhysicalArchitecture.MainElectronicsAssembly")`
  — both bogus, since `PowerModule`/`MainElectronicsAssembly` are unrelated sibling subtypes of
  `PhysicalAssembly`, not real instance paths. `extend_instance_def_mappings_with_specializations`
  then does exactly what it's supposed to do with a *good* seed — propagate it across every subtype
  of the shared ancestor — except here the seed itself was garbage, so it produced
  `(def="PhysicalArchitecture.MainElectronicsAssembly", instance="PhysicalArchitecture.PowerModule")`,
  a mapping that remaps any real member of `MainElectronicsAssembly` (like `mainControlPcb`,
  `safetyGpioHarness`) onto the wrong sibling's path. This is exactly what O-8 observed: 2 nodes
  with `qualifiedName` incorrectly rooted at `PhysicalArchitecture.PowerModule.*`, `containerId`
  pointing at a container that was never created (`occ:PhysicalArchitecture.PowerModule`), and —
  confirmed as the same root cause producing a second visible symptom — those same 2 nodes falling
  through `buildInterconnectionElkBuild`'s `if (parentId && nodesById.has(parentId))` check
  (`shared/diagram-renderer/src/render/interconnection-elk-input.ts:43-53`) and rendering as
  ownerless orphan boxes with no containment nesting at all.
- **Fix:** `instance_def_mapping_for_part` now returns `None` immediately if the resolved graph
  node is itself a `part def` — mirroring a guard that `collect_instance_def_mappings`'s *other*
  mapping-collection loop already had for its own candidates (this function was the one path
  missing it). One-line guard, no change to the (correct, once given a real seed) specialization
  propagation logic.
- **Regression test:** `instance_def_mapping_for_part_skips_definition_kind_nodes`
  (`ibd/connectors.rs`) — constructs a minimal `SharedBase`/`SiblingA`/`SiblingB` graph, feeds in a
  bogus `IbdPartDto` representing `SiblingB` itself (mirroring exactly how extraction can produce
  one), and asserts no mapping is returned. Verified red→green: reproduces the exact bogus
  `Some(("Demo.SharedBase", "Demo.SiblingB"))` output when the guard is removed, passes with it in
  place.
- **Verified against the real repo:** re-exported `sysml-robot-vacuum-cleaner`'s `interconnections`
  view before/after — dangling `containerId` count went from 2 to 0, `mainControlPcb` now correctly
  resolves to `PhysicalArchitecture.AutonomousFloorCleaningRobot.mainElectronics.mainControlPcb`
  (nested under `mainElectronics`, itself nested correctly all the way to the root, no dangling
  references anywhere in the response), and its own 11 real children (`wirelessModule`, `flashNode`,
  `leftMotorDriver`, `mcu`, etc.) are now correctly attached under it instead of scattered as
  orphans. Total node count dropped from 71 to 58 (removing the bogus `PowerModule`/
  `MainElectronicsAssembly`-as-self-parts entries and their now-correctly-deduplicated descendants).
  Full `cargo test` (`sysml_model`, `lsp_server`, `workspace`, `server`) green. Re-exported all 8
  baseline view kinds from this session's earlier refactor-plan verification set: 6 of 8
  byte-identical (general/sequence/state-transition/browser/grid/geometry — none of these fixtures
  exercise the sibling-subtype shape), interconnection-view/action-flow-view showed only the
  pre-existing [O-6](#o-6-interconnection-view-and-action-flow-view-layout-is-non-deterministic-run-to-run)
  coordinate non-determinism, confirming the fix is correctly scoped and doesn't touch unrelated
  cases.
- **Not addressed by this fix, confirmed still separate:** re-ran the ignored real-repo
  [O-1](#o-1-scopedincremental-ibd-build-can-pick-a-different-but-valid-root-than-the-full-workspace-build)
  parity test after this fix landed — still fails (with different numbers, since the view under test
  was renamed by ongoing external edits to the same repo) — confirming O-1's scoped-vs-full parity
  gap is a genuinely separate mechanism (SysML variation/variant-selection, per its own tracker
  entry), not fixed as a side effect here.
- **History note:** this item was originally opened as O-8 against `sysml-robot-vacuum-cleaner`'s
  `firmwareDeployment` view; that view's type changed from `InterconnectionView` to `GeneralView` in
  the live external repo mid-investigation (it's being actively edited outside this session),
  invalidating that specific repro. Re-confirmed and fixed against `interconnections`, a different,
  stable view in the same repo exhibiting the identical bug.

### F-19: Click-to-source silently broken for every real interconnection-view export — `prepare_interconnection_prepared_view` hardcoded `uri`/`range` to `None`
- **Fixed:** 2026-07-07 · `crates/sysml_model/src/semantic/prepared_view/preparers/interconnection.rs`.
- **Was:** [F-5](#f-5-click-to-source-does-nothing-in-interconnection-view) threaded `uri`/`range`
  correctly through `IbdPartDto`/`IbdPortDto` → `InterconnectionNodeDto`/`InterconnectionPortDto`
  (verified by a scene-level regression test), but the *final* step —
  `prepare_interconnection_prepared_view`, which builds the `PreparedNodeDto`/port-detail JSON the
  renderer/webview actually consumes — **hardcoded `uri: None, range: None`** on every node and
  never included `uri`/`range` in port-detail JSON at all, regardless of what the scene node
  carried. F-5's own test never caught this because it stopped at the scene DTO, one step before
  the actual bug.
- **Found while investigating a user report** against `sysml-robot-vacuum-cleaner`'s
  `firmwareDeployment` interconnection view: exporting it via the real CLI and inspecting the JSON
  showed raw `ibd.parts[].uri`/`.range` populated correctly for every part, but every node in
  `preparedView.nodes[]` had `uri: undefined, range: undefined` — pinpointing the break to this one
  preparer.
- **Fix:** `prepare_interconnection_scene` now copies `node.uri.clone()`/`node.range.clone()` onto
  each `PreparedNodeDto`, and adds `uri`/`range` fields to each port's detail JSON (matching the
  field names the TS fallback path — `prepare/interconnection-scene.ts`'s `mapPortDetail` — already
  expected, so both code paths agree).
- **Side effect:** the slim interconnection payload legitimately grows by a few KB now that every
  node/port carries real `uri`/`range` JSON. Bumped 3 hardcoded byte-budget regression guards
  (`52_000` → `62_000`) that were unknowingly calibrated against the buggy, artificially-smaller
  payload: `crates/lsp_server/tests/integration/interconnection_visualization.rs`,
  `crates/lsp_server/tests/integration/powersystems_performance.rs`,
  `crates/server/src/diagrams.rs`.
- **Regression test:** `prepared_view_nodes_and_port_details_carry_source_location_for_click_to_source`
  (`prepared_view/preparers/interconnection.rs`) — goes all the way from a raw `IbdDataDto` through
  `build_interconnection_scene` to the final `PreparedViewDto`, specifically closing the gap F-5's
  test left open.
- **Verified via CLI** against both the bundled `examples/drone` fixture (18/19 nodes now carry
  `uri`) and the real `sysml-robot-vacuum-cleaner` `firmwareDeployment` view (21/23 nodes now carry
  `uri`; the 2 without are the synthetic package containers, correctly so). Full `cargo test` green.
  Re-exported all 8 baseline view kinds: the 6 untouched kinds byte-identical; only
  interconnection-view's payload changed, exactly as intended.

### F-18: Phase 8 of the view-pipeline refactor plan — fixed the `standard-views.ts` naming collision and split `behavior.ts`
- **Fixed:** 2026-07-07 · `shared/diagram-renderer/src/views/standard-views.ts` renamed to
  `standard-views-render.ts`; `shared/diagram-renderer/src/prepare/behavior.ts` split into
  `prepare/behavior/{common,action-flow,state,sequence,index}.ts`.
- **Was:** `prepare/standard-views.ts` (prepare-phase: browser/grid/geometry data shaping) and
  `views/standard-views.ts` (render-phase: SVG drawing for the same 3 kinds) shared an identical
  filename across different directories/responsibilities — easy to open the wrong one.
  `prepare/behavior.ts` (487 lines) mixed three largely-independent kind-normalization pipelines
  (activity/action-flow, state-machine, sequence) in one file, including an 11-way nested ternary
  for action-kind classification.
- **Fix:** renamed the render-phase file to `standard-views-render.ts` (1 import site updated in
  `renderer.ts`, plus a comment in `layout.ts`). Split `behavior.ts` into
  `prepare/behavior/action-flow.ts` (`prepareActivity`), `state.ts` (`prepareState`),
  `sequence.ts` (`prepareSequence`), and `common.ts` (the alias-resolution helpers shared by
  activity and state prep — `buildActivityNodeAliasMap`/`resolveActivityNodeRef`), mirroring the
  render-side split that already existed (`views/action-flow.ts`/`state-transition.ts`/`sequence.ts`).
  `prepare/behavior/index.ts` re-exports all three, so `prepare/index.ts`'s existing
  `from "./behavior"` import kept working unchanged (Node/TS resolves it to the new directory's
  index automatically) — zero import-path churn at the only call site. Purely mechanical: no
  logic changed, confirmed by diffing each extracted function's body against the original.
- **Verified:** `tsc --noEmit` clean, TS `npm test` green. Rebuilt the headless bundle and CLI,
  re-exported all 8 baseline fixtures: 6 of 8 byte-identical, interconnection-view/action-flow-view
  showed only the pre-existing [O-6](#o-6-interconnection-view-and-action-flow-view-layout-is-non-deterministic-run-to-run)
  coordinate non-determinism — confirms the split changed nothing observable. Rust `cargo test`
  green (unaffected, TS-only change).

### F-17: Phase 7 of the view-pipeline refactor plan — skip `general_view_graph` computation for kinds that don't use it (partial; `ibd` skipping deferred)
- **Fixed:** 2026-07-07 · `crates/sysml_model/src/semantic/visualization/response.rs`.
- **Was:** compartment folding (`canonical_general_view_graph`, building `general_view_graph`) and
  the `package_groups` derived from it ran unconditionally for **every** view kind on every
  request, even though the plan's original premise ("only general-view needs this") turned out to
  be only half right on closer inspection.
- **Investigation before changing anything:** traced every TS consumer of the response payload
  (`shared/diagram-renderer/src/prepare/*.ts`) directly rather than assuming from the DTO shape.
  Found `prepare/standard-views.ts`'s `graphNodesForStandardView`/`graphEdgesForStandardView` read
  `visualization.generalViewGraph ?? visualization.graph` — so **browser-view, grid-view, and
  geometry-view also need `general_view_graph`**, not just general-view; gating it to general-view
  alone would have silently dropped compartment folding for those 3 kinds instead of just skipping
  unused work. interconnection/sequence/state/action-flow confirmed to never read it (they render
  from their own dedicated `interconnectionScene`/`sequenceDiagrams`/`stateMachines`/
  `activityDiagrams` payloads instead).
- **Also investigated, found unused everywhere, but NOT removed:** grepped every TS `prepare/*.ts`
  file and the VS Code extension host for reads of `visualization.ibd` (as opposed to
  `interconnectionScene`, which is itself derived server-side from `ibd`) — found **zero** direct
  consumers for *any* view kind, including interconnection-view. `package_groups` (top-level DTO
  field, distinct from `prepared.meta.packageContainerGroups` which the renderer actually reads)
  also appears to have zero consumers anywhere in `shared/diagram-renderer` or `vscode/src`. Did
  **not** act on either finding this phase: `ibd`-scope selection has a `resolved_view ==
  "general-view"` special case (`ibd_source` branch) whose purpose isn't fully understood yet and
  which is adjacent to [O-1](#o-1-scopedincremental-ibd-build-can-pick-a-different-but-valid-root-than-the-full-workspace-build)
  — a bug area with real behavioral surprises already found this session. Skipping `ibd`
  computation for non-interconnection kinds is deferred to its own future phase with dedicated
  investigation, rather than folded into this one under time pressure.
- **Fix:** added `renderer_uses_general_view_graph(renderer) -> bool` (matches
  `general-view`/`browser-view`/`grid-view`/`geometry-view`), gating both `general_view_graph` and
  `package_groups` computation behind it; both fields become the `None` they already were on the
  `slim` (interconnection) path when the requested kind doesn't need them.
- **Verified:** full `cargo test` green (no existing test asserted `general_view_graph.is_some()`
  for interconnection/sequence/state/action-flow, confirming the gate is safe). Rebuilt the CLI and
  re-exported all 8 baseline fixtures: 6 of 8 byte-identical, interconnection/action-flow showed only
  the pre-existing [O-6](#o-6-interconnection-view-and-action-flow-view-layout-is-non-deterministic-run-to-run)
  coordinate non-determinism. Cross-checked `--format json` output directly: `generalViewGraph` is
  present for general/browser/grid/geometry and correctly absent for
  interconnection/sequence/state-transition/action-flow. TS `npm test` green.
- **Not done — flagging honestly:** the plan asked for a manual VS Code webview spot-check for this
  phase specifically, on top of CLI verification. This session has no running VS Code extension
  host to click through, so that step wasn't possible here; the TS consumer code was traced
  line-by-line instead as the best available substitute. Recommend an actual webview click-through
  (open general-view, browser-view, and interconnection-view on a real workspace) before shipping
  this change, given the plan's explicit "don't trust CLI/unit tests alone" lesson from earlier this
  session.

### F-16: Phase 6 of the view-pipeline refactor plan — deduped exposed-id filtering
- **Fixed:** 2026-07-07 · new `crates/sysml_model/src/semantic/exposed_ids.rs`;
  `sequence_views/mod.rs`, `state_views/mod.rs`, `visualization/projection.rs`.
- **Was:** `filter_sequence_diagrams_by_exposed_ids` and `filter_state_machines_by_exposed_ids`
  were near-verbatim copy-pasted 3-way id-matching logic (exact match / `::`-prefixed descendant /
  reconstructed `package_path::name` match), differing only in field/type names.
- **Fix:** extracted the shared logic into `exposed_ids::filter_by_exposed_ids`, generic over any
  `T` via a `key_of` closure extracting `(id, package_path, name)`; both call sites now delegate to
  it. Investigated whether `filter_activity_diagrams_by_graph`'s structurally different approach
  (matches against an already-projected graph's action-like nodes by `(name, top_level_package)`,
  not against a raw exposed-ids set) was drift or intentional — confirmed **intentional**
  (action-flow-view filtering needs to agree with whatever `ProjectionStrategy` already scoped into
  the graph, not re-derive exposure independently) and left it alone, adding a doc comment
  cross-referencing the new shared helper so a future reader doesn't "fix" the difference away.
- **Verified:** `cargo build` clean. Full `cargo test` green. Rebuilt the CLI and re-exported all 8
  baseline fixtures: 6 of 8 byte-identical, interconnection-view/action-flow-view showed only the
  pre-existing [O-6](#o-6-interconnection-view-and-action-flow-view-layout-is-non-deterministic-run-to-run)
  coordinate non-determinism (sequence-view and state-transition-view — the two kinds this phase
  actually touches — were both byte-identical, confirming zero behavior change).

### F-15: Phase 5 of the view-pipeline refactor plan — DTO builder for `SysmlVisualizationResultDto`
- **Fixed:** 2026-07-07 · `crates/sysml_model/src/semantic/visualization/response.rs`.
- **Was:** the full ~20-field `SysmlVisualizationResultDto` struct literal was hand-duplicated 4
  times across `build_sysml_visualization_from_artifacts` — 3 of those 4 (no view candidates,
  no candidate matched the request, matched candidate unsupported) were **identical field-for-field**
  except `view_candidates`, `selected_view`, `selected_view_name`, and `empty_state_message`.
- **Fix:** extracted the 3 identical branches into one `empty_visualization_response(...)` helper
  taking exactly those 4 varying values as parameters; all 3 call sites reduced from ~35-line
  literals to a single call each (~90 lines removed net). Also removed the now-dead outer
  `empty_graph` local (its only uses moved inside the new helper). The 4th (success) branch's
  literal was left as-is — it's not duplicated anywhere, so there was nothing to consolidate there
  under this phase's "pure mechanical, no behavior change" scope.
- **Verified:** `cargo build` clean, no new warnings. Full `cargo test` green, including the ts-rs
  golden-binding test (`typescript_bindings.rs`) — confirms no field/type shape changed, as
  expected since only construction-site code moved. Rebuilt the CLI and re-exported all 8 baseline
  fixtures: 6 of 8 byte-identical, interconnection-view/action-flow-view showed the same
  pre-existing [O-6](#o-6-interconnection-view-and-action-flow-view-layout-is-non-deterministic-run-to-run)
  coordinate non-determinism only. Also spot-checked a `--format json` export to confirm the new
  helper serializes correctly end-to-end via the CLI, not just at the Rust type level. TS `npm test`
  green (unaffected, Rust-only change).

### F-14: Phase 4 of the view-pipeline refactor plan — unified the 3 ELK option-builders (and killed dead code found along the way)
- **Fixed:** 2026-07-07 · new `shared/diagram-renderer/src/render/elk-options.ts`;
  `render/layout.ts`, `render/interconnection-elk-input.ts`, `views/behavior-common.ts`.
- **Was:** general-view (`render/layout.ts`), interconnection-view
  (`render/interconnection-elk-input.ts`), and the behavior family (`views/behavior-common.ts`)
  each hand-rolled their own ELK `layoutOptions` object with independently drifted spacing/padding/
  aspectRatio values and no shared defaults.
- **Bigger finding while auditing `layout.ts`:** the "interconnection vs general" ternary branches
  inside `layoutPrepared`'s root `graph.layoutOptions` (and the `width`/`height`/`packageGroups`
  ternaries just above them) were **entirely dead code** — `layoutPrepared` returns early via
  `layoutInterconnectionPrepared(prepared)` whenever `prepared.view === "interconnection-view"`
  (a few lines above), so by the time execution reached those ternaries, the interconnection branch
  could never fire. Confirmed by inspecting the actually-live interconnection ELK options in
  `interconnection-elk-input.ts:263-281`: different key set entirely (`portConstraints: FIXED_ORDER`
  + `portAlignment.default: CENTER` + `crossingMinimization.strategy: LAYER_SWEEP`, no
  `aspectRatio`) versus the dead code's `portConstraints: FIXED_SIDE` + `aspectRatio: 1.6`, no
  `portAlignment`/`crossingMinimization` at all. Not a runtime bug (the dead branch never executed),
  but genuinely misleading to read.
- **Fix:** `render/elk-options.ts` exports `buildElkLayoutOptions(kind, overrides?)` with options
  that were already identical across every *live* call site as shared defaults, and the previously
  hand-rolled (but never behaviorally wrong) values preserved verbatim as per-kind defaults for
  `general`/`interconnection`/`behavior-state`/`behavior-action`. All 3 call sites now build their
  options through it, supplying only real per-invocation deltas (`useHierarchy` for general-view,
  `horizontal` direction for behavior views) as overrides. The dead `isInterconnectionView`
  ternaries and the now-unreachable variable itself were deleted from `layout.ts`.
- **Verified:** TS `npm test` and `tsc --noEmit` clean. Rebuilt the headless bundle and CLI,
  re-exported all 8 baseline fixtures: general/sequence/state-transition/browser/grid/geometry
  byte-identical to pre-Phase-4 output (confirms the consolidation preserved every live value
  exactly); interconnection-view and action-flow-view showed the same pre-existing
  [O-6](#o-6-interconnection-view-and-action-flow-view-layout-is-non-deterministic-run-to-run)
  coordinate non-determinism, same text content otherwise. Full Rust `cargo test` green
  (Phase 4 is TS-only; Rust suite run as a sanity check).

### F-13: Phase 3 of the view-pipeline refactor plan — one canonical element-kind classifier
- **Fixed:** 2026-07-07 · new `crates/sysml_model/src/semantic/element_kind_classify.rs`;
  `view_projection.rs`, `model_projection.rs`, `reference_resolution.rs`.
- **Was:** the same loose "does this `element_type` string look like a part/action/port/attribute"
  substring check was independently reimplemented in **4** places, not the 3 originally audited:
  `is_part_like`/`is_action_like` (`view_projection.rs`), `is_port_like`/`is_attribute_like`/
  `is_parameter_like` (`model_projection.rs`), and a 4th, previously-unflagged instance —
  `is_part_like_kind` (`reference_resolution.rs`), doing the identical `.to_lowercase().contains("part")`
  check but typed to take `&ElementKind` instead of `&str`.
- **Fix:** consolidated all 5 predicates into one `element_kind_classify.rs` module, replacing all 4
  call sites. Audited for behavioral disagreement first (per the risk this phase was flagged with) —
  all 4 implementations turned out identical once normalized, so this was a pure move, not a bugfix.
  `is_definition_element_kind`/`is_reference_element_kind`/`is_part_instance_kind`
  (`ibd/extract_impl.rs`) were also audited but left as-is: `is_definition_element_kind` already
  delegates to the canonical `ElementKind::is_definition()` (no duplication), and the other two are
  IBD-domain-specific concepts (definition/reference/instance distinction for interconnection
  scoping) not duplicated anywhere else — correctly scoped to stay local to the `ibd` module.
- **Verified:** full `cargo test` (one test, `interconnection_export_matches_slim_scoped_lsp_contract_for_drone`,
  failed once under parallel execution then passed reliably across two full reruns — traced to the
  pre-existing [O-6](#o-6-interconnection-view-and-action-flow-view-layout-is-non-deterministic-run-to-run)
  layout non-determinism, not this change, since Phase 3 touches only element-kind classification).
  Rebuilt the CLI and re-exported all 8 baseline fixtures: 6 of 8 byte-identical, the other 2
  (interconnection-view, action-flow-view) confirmed same text content/counts as baseline (only
  coordinates differ, consistent with O-6). TS `npm test` green (unaffected, Rust-only change).

### F-12: Phase 2 of the view-pipeline refactor plan — pinned the ELK layout engine version
- **Fixed:** 2026-07-07 · `vscode/node_modules` (reinstalled), new
  `crates/server/assets/elkjs/VENDORED_VERSION`, `vscode/scripts/check-elkjs-version.js`,
  `vscode/scripts/build-webview.js`, `vscode/scripts/build-headless-renderer.js`.
- **Was:** the CLI/headless SVG export path uses hand-vendored ELK assets
  (`crates/server/assets/elkjs/{elk-api.js,elk-worker.min.js}`, confirmed byte-identical to
  `shared/diagram-renderer/node_modules/elkjs@0.11.1`), while the VS Code webview build resolves
  `elkjs` from `vscode/node_modules` directly. Both `package.json`s pin `elkjs@^0.11.1`, but the
  actually-installed `vscode/node_modules/elkjs` had drifted to `0.8.2` — meaning a webview build
  right now would lay out diagrams with a materially older ELK than the CLI, silently, with nothing
  catching it.
- **Fix:** reinstalled `vscode/node_modules` from the lockfile (`npm ci`), which now correctly
  resolves `elkjs@0.11.1`. Added a `VENDORED_VERSION` marker recording which elkjs version
  `crates/server/assets/elkjs/*` was vendored from, plus a small
  `assertElkjsVersionMatchesVendored` check wired into both `build-webview.js` (checks
  `vscode/node_modules/elkjs`, the webview's actual runtime dependency) and
  `build-headless-renderer.js` (checks `shared/diagram-renderer/node_modules/elkjs`, the source the
  vendored Rust assets should be re-generated from) — either build now fails loudly with a clear
  fix-it message if its resolved elkjs version ever diverges from the vendored marker again, instead
  of silently drifting.
- **Verified:** confirmed the check fires correctly by deliberately mismatching the marker and
  re-running `build-webview.js` (failed with the expected error, exit code 1), then restored it and
  confirmed both builds pass clean. Rebuilt the CLI and re-exported the `general-view` baseline
  fixture — byte-identical to pre-Phase-2 output, as expected (the CLI's vendored assets were already
  correct; this fix only addressed the webview's local install drift and added the guard). Full
  `cargo test` and TS `npm test` green.

### F-11: Phase 1 of the view-pipeline refactor plan — dead-code deletions
- **Fixed:** 2026-07-07 · `crates/server/src/lib.rs`, `crates/sysml_model/src/semantic/ibd/extract_impl.rs`,
  `shared/diagram-renderer/src/views/standard-views.ts`.
- **What:** first phase of a broader consistency/maintainability refactor of the view-projection/rendering
  pipeline (see the refactor plan for the full 8-phase scope). Pure deletions/consolidation only, no
  behavior change:
  - `crates/server/src/lib.rs`: gated `pub mod elk_layout;` behind `#[cfg(test)]` — its only caller,
    `legacy_elk_svg.rs`, was already test-only, so ~1.6MB of embedded ELK JS assets were shipping in every
    release binary for zero runtime benefit.
  - `crates/sysml_model/src/semantic/ibd/extract_impl.rs`: deleted `prune_redundant_top_level_roots`
    (confirmed zero production call sites — only self-exercised by its own unit test) plus the helpers
    that became orphaned once it was gone (`typed_by_name`, `attribute_text`, `last_type_segment`) and
    their test.
  - `shared/diagram-renderer/src/views/standard-views.ts`: removed a verbatim copy of
    `asRecord`/`asArray`/`asString`, now imported from `prepare/util.ts` (the two copies were confirmed
    byte-identical before merging).
  - Triaged 3 other flagged `#[allow(dead_code)]`/"legacy wrapper" sites
    (`relationships.rs` `add_cross_document_edges_for_uri`, `lsp_server` `symbols.rs`'s staged code-lens
    helpers, `sysml_model` `model.rs`'s staged `RelationshipKind` variants) — all 3 confirmed genuinely
    load-bearing or intentionally staged (not organic drift), left as-is; they already carry adequate
    doc comments explaining why.
  - **Not done:** `views/behavior-common.ts`'s `truncateLabel` vs `render/drawing.ts`'s `truncate` were
    flagged as a likely duplicate in the original audit, but on inspection they have **different
    behavior** (`truncate` doesn't trim and appends `...`; `truncateLabel` trims first and appends `..`) —
    merging them would be a real behavior change, not a pure deletion, so left alone as out of scope for
    this phase.
- **Verified via CLI:** rebuilt the headless bundle and `spec42` binary, re-exported all 8 view-kind
  baseline fixtures. 6 of 8 were byte-identical to the pre-change baseline; the other 2
  (interconnection-view, action-flow-view) differed, but investigation found this is pre-existing
  layout non-determinism unrelated to this change (see [O-6](#o-6-interconnection-view-and-action-flow-view-layout-is-non-deterministic-run-to-run))
  — same node/edge text content and counts, only coordinates differ, and the same non-determinism
  reproduces on the unmodified pre-Phase-1 binary across repeated runs.
- **Full workspace `cargo test` and TS `npm test`** green throughout.

### F-10: Deleted the duplicate Rust `prepared_view` preparers for 6 of 7 view kinds (root-causes F-9's bug class)
- **Fixed:** 2026-07-07 · deleted `crates/sysml_model/src/semantic/prepared_view/preparers/graph.rs`,
  `standard.rs`, `behavior.rs`; kept only `interconnection.rs`.
- **Was:** [F-9](#f-9-a-second-independent-rust-prepared-view-builder-silently-dropped-all-compartment-data--the-actual-reason-f-7-didnt-show-up-in-real-output)
  fixed one instance of a whole class of bug: Rust independently reimplemented the TS `prepare/*.ts`
  compartment/layout logic for **all 7** view kinds (`graph.rs` for general-view, `standard.rs` for
  browser/grid/geometry, `behavior.rs` for activity/state/sequence), each a hand-maintained port that
  could (and, for general-view, did) silently drift out of parity with its TS counterpart. `behavior.rs`'s
  `collect_state_machine_nodes` had a second, still-live parity bug of the same kind: it doesn't compute
  composite-state region-nesting the way TS's `attachExplicitRegions`/`attachCompositeRegions` do.
- **Why deletion was safe for 6 of 7 kinds, but not `interconnection-view`:** every consumer of
  `response.prepared_view` (VS Code webview, headless SVG exporter) already falls back to recomputing the
  prepared view from raw graph/IBD/diagram data client-side in TS
  (`preparedViewFromPayload(payload) ?? prepareViewData(payload)`) whenever `prepared_view` is absent. That
  raw data is *always* sent alongside `prepared_view` for every view kind **except**
  `interconnection-view`, where `slim_interconnection_payload` nulls out `graph`/`general_view_graph`/
  `ibd`/`workspace_model` and `prepared_view` becomes the *only* payload — so `interconnection.rs` must
  stay.
- **Fix:** `prepare_view_from_visualization` now only dispatches to
  `prepare_interconnection_prepared_view` for `"interconnection-view"`; every other view kind returns
  `Err(..)`, which `finalize_visualization_response`'s `.ok()` turns into `prepared_view: None`, letting the
  existing TS fallback recompute it exactly as it already does today for any payload where Rust's
  computation is simply missing.
- **Trade-off (accepted, not blocking):** `--format json` CLI export no longer includes a ready-made
  `prepared_view` for the 6 non-interconnection view kinds — JSON consumers now get the same raw
  `graph`/`ibd`/diagram shape the TS fallback consumes. `--format svg` is unaffected (the headless
  renderer's own TS fallback handles it internally).
- **Verified via the actual CLI** (per the F-9 lesson): rebuilt the headless bundle and `spec42` binary,
  re-exported `general-view`, `interconnection-view`, `sequence-view`, `state-transition-view`,
  `action-flow-view`, and `browser-view` from real fixtures (`examples/webshop` and a scratch model) —
  all produced real, non-empty SVG content (dozens of `<text>` elements each) except the
  pre-existing/baseline [O-5](#o-5-general-view-can-render-completely-empty-for-a-valid-filter-less-expose-pre-existing-not-a-regression)
  case, confirmed unrelated to this change by reproducing it identically on a pre-change `git stash`
  baseline. `grid-view`/`geometry-view` aren't exercised by any bundled example or working synthetic
  fixture; verified indirectly via `browser-view`'s successful real-content export, since all three kinds
  shared the exact same deleted source file (`standard.rs`) and identical TS-fallback mechanics.
- **Full workspace test suite** (`cargo test -p sysml_model -p lsp_server -p workspace -p server`) green
  throughout, zero regressions.

### F-9: A second, independent Rust "prepared view" builder silently dropped all compartment data — the actual reason F-7 didn't show up in real output
- **Fixed:** 2026-07-07 · `crates/sysml_model/src/semantic/prepared_view/preparers/graph.rs`
  (`prepare_graph_from_dto`).
- **Was:** [F-7](#f-7-general-view-node-bodies-render-completely-empty-when-the-view-has-a-kind-narrowing-filter)
  fixed compartment computation in `canonical_general_view_graph`/`response.rs`, and this was verified
  correct via direct Rust-level checks. But the CLI (`spec42 diagrams export`) and the bundled headless
  SVG renderer don't consume `general_view_graph` directly — they consume `response.prepared_view`, built
  by a **second, independent Rust reimplementation** of the TS `prepareGraph` (`shared/diagram-renderer/src/prepare/graph.ts`)
  living in `prepared_view/preparers/graph.rs`. That Rust version hand-picked only 4 attribute keys
  (`qualifiedName`/`isPackage`/`isDefinition`/`isReference`) instead of spreading through *all* of
  `node.attributes` the way the TS version does — silently dropping `generalViewDirectAttributes/Parts/Ports`
  (and their `Inherited` counterparts) before they ever reached the renderer. So even with F-7 and F-8
  landed, a user re-exporting via the CLI still saw the exact same broken, header-only-node output —
  **this is why "make you validate the actual rendered diagram using the CLI before you claim you are
  done" caught something the Rust-level unit tests and TS-level layout tests both missed.**
- **Root lesson:** there are *two* independent "prepare" implementations (TS for the VS Code
  webview/live editing path, Rust for the CLI/headless-export path) that must be kept in parity by hand —
  neither is generated from the other. Any future change to what a compartment/attribute needs to reach
  the renderer must be made in **both** `shared/diagram-renderer/src/prepare/graph.ts` and
  `crates/sysml_model/src/semantic/prepared_view/preparers/graph.rs`, and validated via the CLI
  (`cargo build -p server --bin spec42` after `node vscode/scripts/build-headless-renderer.js`, then
  `spec42 diagrams export <path> --selected-view <name> --format svg --output <file>`), not just unit
  tests of either half in isolation.
- **Fix:** `prepare_graph_from_dto` now spreads every entry of `node.attributes` into the prepared node's
  `attributes` JSON (matching the TS `{ ...node.attributes, ... }` pattern) before overlaying the 4
  computed keys.
- **Verified via the actual CLI** (per the instruction that caught this): rebuilt
  `crates/server/assets/diagram-renderer/headless-renderer.js` from source, rebuilt the `spec42` binary,
  ran `spec42 diagrams export .../sysml-robot-vacuum-cleaner/model --selected-view productStructure
  --format svg --output ...` against the real repo, and inspected the resulting SVG directly: node
  heights now vary (58-1792px, previously uniformly 44/70px for all 91 nodes), `CleaningRobotSystemOfInterest`
  grew from `height=70` to `height=96` and its SVG now contains the text `physical :
  AutonomousFloorCleaningRobot`, and the [F-8](#f-8-sprawling-tangled-general-view-layout-partially-fixed-package-hierarchy)
  layout width improvement (10155px vs ~15200px flat) is also confirmed intact in this same real export.
- **Regression test:** `prepare_graph_from_dto_carries_general_view_compartment_attributes_through`
  (`prepared_view/preparers/graph.rs`).

### F-8: Sprawling, tangled general-view layout — partially fixed (package hierarchy)
- **Fixed:** 2026-07-07 · `shared/diagram-renderer/src/render/layout.ts` (`layoutPrepared`).
- **Was:** The ELK graph built for general-view diagrams was completely flat — `graph.children` was a
  plain list of every node with no containment nesting for packages, unlike the Interconnection View
  path (`interconnection-elk-input.ts`), which builds real parent/child ELK containers for IBD. Since
  ELK had no idea packages existed, members of the same package could end up scattered anywhere in one
  flat layered tree. Package "frames" (`general-package-frame`) were (and still are) a pure post-hoc
  bounding box around wherever members landed, so scattered members produced huge/overlapping frames.
  Spacing constants (`elk.spacing.nodeNode: 220`, `nodeNodeBetweenLayers: 280`) were also ~1.5-2x larger
  than necessary.
- **Fix:** `layoutPrepared` now reads `prepared.meta.packageContainerGroups` (already computed by
  `prepare/graph.ts`'s `buildGeneralPackageContainerGroups`) and, when a diagram has 2+ packages, nests
  each package's member nodes under their own ELK container node with `elk.hierarchyHandling:
  "INCLUDE_CHILDREN"` at the root — mirroring the pattern already proven for IBD containers. Node/edge
  position resolution was updated to recurse through the now-nested ELK result (absolute positions via
  parent-offset accumulation; edges collected recursively since ELK can record them on a container's own
  `.edges` array rather than the root's). Spacing was also tightened (`nodeNode: 220→140`,
  `nodeNodeBetweenLayers: 280→180`, `edgeNode: 120→90`, `edgeEdge: 120→80`). Diagrams with 0-1 packages
  are unaffected (same flat layout as before — verified against the small `webshop` example: 10 nodes,
  no `packageContainerGroups`, bounding box unchanged).
- **Verified against the real repo** (`productStructure`, 91 nodes / 3 packages): overall bounding box
  went from ~15200×3300px (flat) to ~10100×2600px (hierarchical) — about a third narrower and a fifth
  shorter — and each package's members are now confirmed to occupy a disjoint, non-overlapping x-range
  (previously could be interleaved anywhere).
- **Regression tests:** `shared/diagram-renderer/src/render/layout.general-view.test.ts` (package
  clustering/disjointness for 2+ packages; flat-layout fallback for &lt;2 packages).
- **Explicitly not fully fixed:** row-wrapping for a single package with very many same-depth siblings
  turned out not to work the way the original plan assumed — see
  [O-4](#o-4-sprawling-layout-for-a-single-package-with-many-same-depth-siblings) for what remains and
  why the obvious next lever (ELK wrapping) is a dead end.

### F-7: General-view node bodies render completely empty when the view has a kind-narrowing `filter`
- **Fixed:** 2026-07-07 · `crates/sysml_model/src/semantic/view_projection.rs` (`ProjectedView`,
  `project_view`), `crates/sysml_model/src/semantic/visualization/response.rs`.
- **Was:** A general view declared with `filter @SysML::PartUsage;` (e.g. the real `productStructure`
  view) legitimately excludes attribute/port nodes from becoming their own diagram boxes — but the
  pipeline filtered the graph down to the `PartUsage`-only node set *before* handing it to
  `canonical_general_view_graph` → `fold_general_view_leaf_details_into_owners`
  (`crates/sysml_model/src/semantic/model_projection.rs:60-109`), whose compartment-folding logic needs
  those same attribute/port nodes present as *input*. Its very first check,
  `if detail_ids.is_empty() { return graph.clone(); }`, always tripped for such views (no attribute/port
  nodes survived the earlier filter), so **every** node rendered as header-only (fixed 44px height, no
  Attributes/Parts/Ports rows at all) — confirmed on the real repo: `generalViewDirectParts` wasn't even
  present as a key on any of 94 nodes' attributes, let alone populated.
- **Fix:** `ProjectedView` now also returns `pre_filter_node_ids` — the node set after scope expansion
  but *before* the kind-narrowing filter runs. `response.rs` builds the compartment-fold input from that
  broader set, then re-narrows the folded output back down to the original (filtered) `projected_ids` for
  the actually-rendered graph — reusing the existing `project_graph_by_ids` helper for both steps. Views
  with no kind filter are unaffected (`pre_filter_node_ids == projected_ids` already).
- **Verified against the real repo:** `productStructure`'s `CleaningRobotSystemOfInterest` node now shows
  `generalViewDirectParts: [{name: "physical", typeName: "AutonomousFloorCleaningRobot", ...}]`; 16 of 91
  nodes now carry populated compartments (the rest are leaf usages with no local attributes/parts/ports
  of their own, which is correct).
- **Regression test:** `part_usage_filter_excludes_attributes_from_node_ids_but_not_pre_filter_node_ids`
  (`view_projection.rs`).

### F-6: Node bodies don't show multiplicity, port direction, or redefines/subsets
- **Fixed:** 2026-07-07 · `crates/sysml_model/src/semantic/model_projection.rs`,
  `shared/diagram-renderer/src/render/drawing.ts` (`formatIbdPortLabel`),
  `shared/diagram-renderer/src/sysml-node-builder.ts` (`collectCompartmentsFromPart`).
- **Was:** Attribute/part/port rows showed only `name`, `name : type`, or `name : type = value` —
  multiplicity, port direction (`in`/`out`), and `redefines`/`subsets` were computed by the graph
  builders and stored in `attributes`, but never read into the display string.
- **Fix:** `format_general_view_detail_display_text` now renders
  `{direction }name{ [multiplicity]} : type{ = value}{ redefines X}{ subsets Y}`. IBD port labels
  (`formatIbdPortLabel`, `collectCompartmentsFromPart`) now prefix direction alongside existing `~`
  conjugation handling.
- **Regression test:** `canonical_general_view_graph_display_text_includes_multiplicity_direction_and_redefines`
  (`model_projection.rs`).
- **Note:** the anonymous-redefinition-stub case (where a node's *name itself* comes from the
  `redefines` attribute fallback) intentionally does not also append `redefines X` — that would be
  redundant (`engine redefines engine`). Only named features get the annotation.

### F-5: Click-to-source does nothing in Interconnection View
- **Fixed:** 2026-07-07 · `crates/sysml_model/src/semantic/ibd/dto.rs` (`IbdPartDto.range`,
  `IbdPortDto.range`/`uri`), `crates/sysml_model/src/semantic/interconnection_scene.rs`,
  `shared/diagram-renderer/src/prepare/interconnection-scene.ts`, `.../prepare/types.ts`.
- **Was:** `InterconnectionSceneNodeDto`/`InterconnectionScenePortDto` never carried `uri`/`range`, so
  the shared click handler (`drawing.ts`, `behavior-interaction.ts` — this part was already correct and
  shared with general-view diagrams) had nothing to navigate to; the extension host fell back to a
  fragile name-based `findElement` lookup that mostly failed.
- **Fix:** Threaded real source location from the graph (`SemanticNode.range`) through
  `IbdPartDto`/`IbdPortDto` → `InterconnectionNodeDto`/`InterconnectionPortDto` → the TS scene
  preparer, mirroring how `graph.ts` already does it for general-view nodes.
- **Regression test:** `scene_nodes_and_ports_carry_source_location_for_click_to_source`
  (`interconnection_scene.rs`).

### F-4: `driveModule` Interconnection View resolved zero connectors (root cause)
- **Fixed:** 2026-07-07 · `crates/sysml_model/src/semantic/ibd/instance_paths.rs`
  (`infer_def_instance_scope_mappings_for_ibd`), `crates/sysml_model/src/semantic/ibd/dto.rs`
  (`DefInstanceMappingDto`), threaded through `extract_impl.rs` / `merge.rs`.
- **Was:** A view that exposes a definition-nested member (e.g.
  `PhysicalArchitecture::AutonomousFloorCleaningRobot::driveModule`, as declared directly inside a
  `part def` body) needs its exposed id translated to the real instantiated path
  (`Architecture::CleaningRobotSystemOfInterest::physical::driveModule`) before connectors can be
  found. That translation (`enrich_root_prefixes_for_interconnection` →
  `infer_def_instance_scope_mappings`) only recognized a narrow naming convention: a package segment
  literally named `architecture`/`Architecture` shared between the def and instance paths. Real
  workspaces that use two independently-named top-level packages (`PhysicalArchitecture` for defs,
  `Architecture` for instances — the exact shape of `sysml-robot-vacuum-cleaner`) never matched, so the
  mapping silently produced nothing and the view legitimately (but wrongly) found zero connectors.
- **Fix:** Added `DefInstanceMappingDto`, populated from the same real typing-edge-derived
  `build_instance_def_mappings` already used internally for connector mirroring, threaded through
  `IbdDataDto` end-to-end (per-URI build → merge → visualization scoping). The old name-heuristic is
  kept as a secondary fallback, not the primary source.
- **Verified against real repo:** `driveModule` view now resolves all 5 connectors (was 0).
- **This was the actual root cause behind the user-reported "driveModule shows no connectors + yellow
  warning banner" bug** — [F-1](#f-1-bare-own-port-connector-endpoints-silently-dropped) and
  [F-2](#f-2-phantom-empty-package-box-above-focused-part) were real, necessary fixes surfaced along
  the way, but this was the piece that made the specific reported view work end-to-end.

### F-3: Interconnection View "no internal connectors" warning is a correct symptom, not a separate bug
- **Status:** No code change needed — `vscode/src/visualization/webview/viewControls.ts`
  (`updateViewStatusBanner`) correctly reports `connectorCount === 0`; it stopped firing once
  [F-1](#f-1-bare-own-port-connector-endpoints-silently-dropped) and
  [F-4](#f-4-driveModule-interconnection-view-resolved-zero-connectors) were fixed.

### F-2: Phantom empty package box above focused part
- **Fixed:** 2026-07-07 · `crates/sysml_model/src/semantic/ibd/extract_impl.rs`
  (`build_container_groups`).
- **Was:** Container-group construction split every part's qualified name on `.` and created a
  container box for **every** prefix depth, without checking whether that prefix was an actual
  composing Part Definition/Usage vs. a package/namespace segment. A view focused on a deeply nested
  part (e.g. `driveModule`) showed an empty box labeled after its enclosing *package*
  (`PhysicalArchitecture`) above the real container (`AutonomousFloorCleaningRobot`).
- **Fix:** `build_container_groups` now takes an `is_non_part_container` predicate and skips prefixes
  that resolve to a non-part-like graph node; dangling `parent_id` references (pointing at a now-excluded
  package prefix) are cleaned up to `None` afterward.
- **Regression test:** `build_ibd_resolves_connectors_with_bare_own_port_endpoint_and_no_phantom_package_box`
  (`extract_impl.rs`).

### F-1: Bare own-port connector endpoints silently dropped
- **Fixed:** 2026-07-07 · `crates/sysml_model/src/semantic/ibd/extract_impl.rs`
  (`qualify_pending_connection_endpoint`).
- **Was:** `connect leftMotor.phaseIn to phaseLeftIn;` inside a `part def` — a 2-segment relative
  member chain (`leftMotor.phaseIn`) plus a bare own-port reference (`phaseLeftIn`) — resolved
  correctly at the graph level (via `resolve_expression_endpoint_strict`), but
  `qualify_pending_connection_endpoint` treated **any** endpoint string containing `::` as "already
  fully qualified" and skipped prefixing it with the enclosing definition. Since
  `expr_node_to_qualified_string` always joins relative member-access chains with `::` (regardless of
  whether the source used `.`), a 2-segment relative chain was indistinguishable from a genuine
  absolute qualified reference — and got left unprefixed, silently failing to match any part, so the
  whole connector was dropped.
- **Fix:** Removed the `"::"`-implies-already-qualified shortcut from
  `qualify_pending_connection_endpoint`; it now always compares against the (dot-normalized) container
  prefix and only skips prefixing if the endpoint already starts with it. Left the sibling
  `qualify_occurrence_endpoint` (a different, ambiguous multi-candidate fallback path used only when no
  specific container is known) with its original shortcut intact, since removing it there caused a
  regression on the bundled `examples/drone` fixture (`drone_connections_scoped_ibd_matches_full_workspace_filter`,
  24 vs 21 connectors) — that path can't safely disambiguate a relative chain from an absolute one
  without a known container.
- **Regression test:** `build_ibd_resolves_connectors_with_bare_own_port_endpoint_and_no_phantom_package_box`
  (`extract_impl.rs`).

---

## Improvement suggestions (not bugs)

- **`allocate` relationships are intentionally not drawn as connector lines in Interconnection
  View — confirmed against the actual SysML v2 spec text, not just Spec42's own docs; not a bug.**
  Investigated after a user question ("is it a spec gap or a bug that firmwareDeployment shows no
  allocate edges"), then re-confirmed against the primary source
  (`C:\Git\elan8\elan8-monorepo\library\SysML_v2.txt`, the OMG SysML v2.0 Part 1 spec text) rather
  than relying on Spec42's own notation-inventory doc alone:
  - **§9.2.20.2.6 InterconnectionView**: "ViewDefinition to present exposed features as nodes,
    nested features as nested nodes, and **connections between features as edges** between (nested)
    nodes." — connections only, no mention of allocation.
  - **§8.2.3.11 Parts Graphical Notation** defines the `interconnection-element` grammar
    production incrementally per relationship kind — `part`/`part-ref` (§8.2.3.11), `port-def`/`port`
    (§8.2.3.12), `connection-def`/`connection`/`connection-relationship` (§8.2.3.13), `interface`
    (§8.2.3.14). **§8.2.3.16 Flows Graphical Notation** explicitly extends it again
    (`interconnection-element =| flow-def | flow`) — showing that when the spec authors *intend* a
    relationship kind to appear in an interconnection view, they add it to this production.
    **§8.2.3.15 Allocations Graphical Notation** (the section right before Flows) defines
    `allocation-def`/`allocation`/`allocate-relationship` but adds them only to `general-relationship`
    and `usage-edge` — **never to `interconnection-element`**. The omission is structural, not an
    oversight: allocation is deliberately scoped to `general-view` (`general-view =|
    interconnection-view`, i.e. GeneralView's grammar is a superset that also carries
    `general-relationship`-family edges like `allocate-relationship`, `satisfy-edge`,
    `verify-relationship`, `expose-relationship` — InterconnectionView is the narrower subset).
  - `crates/sysml_model/src/semantic/ibd/{extract_impl,connectors}.rs` implement exactly this:
    only `RelationshipKind::Connection` is ever consulted when building IBD connectors —
    `RelationshipKind::Allocate` is never referenced (confirmed by grep, zero matches) — matching
    the grammar. `docs/reference/SYSML-NOTATION-INVENTORY.md` independently marks
    `allocate-relationship.svg`/`allocation.svg`/`allocation-def.svg` as **WONTFIX (not in shipped
    UI)**, consistent with this reading. **What the same inventory doc says *should* exist but
    doesn't yet**: `allocations-compartment.svg` is marked "shared (compartment text only)" —
    allocations are supposed to show up as a text line inside affected node bodies (e.g. "allocated
    to: X"),
  which is real, missing functionality, already tracked under
  [O-2](#o-2-missing-node-body-compartments-for-most-non-structural-diagram-kinds) (confirmed:
  `model_projection.rs`'s compartment-folding has zero references to "allocation" anywhere).
- **Qualified/short-name in node header.** Header name is truncated at 26 chars with no fallback to
  show the full qualified name (e.g. on hover/tooltip). Low effort, would help disambiguate nodes with
  long or colliding short names.
- **Requirement-specific notation.** No `subject` line or inline `doc` text rendered for requirement
  nodes anywhere yet — tracked as part of [O-2](#o-2-missing-node-body-compartments-for-most-non-structural-diagram-kinds)
  but called out separately since it's one of the more commonly-hit missing kinds in practice.
- **Scoped-build safety net.** Given [O-1](#o-1-scopedincremental-ibd-build-can-pick-a-different-but-valid-root-than-the-full-workspace-build),
  consider a cheap post-hoc check in the scoped path: if the resolved root's connector count is `0` but
  the exposed id is known to have connectors in a quick full-workspace root lookup, fall back to full
  build for that one request rather than serving a misleadingly-empty scoped result.
- **`root_views` keying**, see [O-3](#o-3-root_views-keyed-by-bare-name-not-qualified-name) — worth
  doing opportunistically next time `extract_impl.rs`/`merge.rs` are touched, not urgent on its own.
