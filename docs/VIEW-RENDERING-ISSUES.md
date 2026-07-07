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

## Open

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
