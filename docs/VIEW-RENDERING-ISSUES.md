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
