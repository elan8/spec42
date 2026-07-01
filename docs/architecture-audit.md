# Spec42 Architecture & Technical-Debt Audit

**Date:** 2026-06-25  
**Last updated:** 2026-06-26  
**Scope:** Full workspace (`crates/*`, ~104.5k LOC, 8 crates).  
**Cross-referenced against:** `docs/engineering/ROBOT-VACUUM-PERFORMANCE-ANALYSIS.md`

### Completed since initial audit

| Item | Description | Commit / PR |
|------|-------------|-------------|
| P1-1 | Added `invalidate_query_indexes()` to `insert_workspace_edge` | in-tree |
| — | **`ElementKind` enum** — replaced `element_kind: String` on `SemanticNode` with a type-safe enum (~60 variants + `Unknown(String)` catch-all); serialises via `#[serde(into = "String", from = "String")]` | in-tree |
| — | **`HostSemanticProjection` redesigned** — now 1:1 with the semantic graph: `element_kind: ElementKind`, `attributes: HashMap<String, Value>`, `kind: RelationshipKind`, `connect: Option<ConnectStatementDetail>`; no information dropped on the path to babel42 | in-tree |
| — | **URL normalisation fix** — `FileSystemDocumentProvider::path_to_url` now lowercases Windows drive letters, matching kernel's normalisation; fixes previously-broken `built_workspace_parity` test | in-tree |
| — | `RelationshipKind` got `Serialize, Deserialize` + `#[serde(rename_all = "camelCase")]`; `ConnectStatementDetail` got `Serialize, Deserialize` | in-tree |
| — | New `SemanticGraph::edges_for_uri()` returning full `SemanticEdge` detail | in-tree |
| P1-3 | **`Arc<SemanticGraph>`** — `HostWorkspaceSnapshot` and `InMemoryWorkspace` now share one `Arc<SemanticGraph>`; `build_workspace_snapshot` clones the `Arc` instead of the graph; incremental update still does one unavoidable deep clone for mutation but shares the result | in-tree |
| P2-1 | **Diagnostics engine O(edges_in_uri) optimisation** — added `edges_by_uri` and `connect_edges_by_declaring_uri` indexes to `GraphQueryIndexes`; rewrote five O(all_edges) edge-scan methods; pre-collect `nodes` and `connect_edges` once in `compute_semantic_diagnostics_with_unit_registry` instead of re-querying per pass; fixed stale-cache bug in all three kernel graph-mutation paths (`update_semantic_graph_for_uri`, `rebuild_all_document_links`, staged rebuild) by adding `invalidate_query_indexes()` after relationship linking | in-tree |

---

## Executive Summary

1. **`kernel` is a near-complete second implementation of the host stack.** It re-exports `semantic_core` types but ships its own ~12.8k-LOC validation pipeline, workspace state, LSP runtime, and DTOs that parallel `spec42_host` + `language_service`. `kernel::{ValidationReport, SemanticModelProjection, SemanticModelNode}` are field-for-field twins of `spec42_host::{HostValidationReport, HostSemanticProjection, HostSemanticModelNode}`. The `server` crate depends on both, so both pipelines ship. This is the largest single source of debt.

2. **The `SemanticGraph` is built and held multiple times per snapshot.** `HostWorkspaceSnapshot` stores `semantic_graph` AND an `InMemoryWorkspace` that owns a full clone of the same graph (`build.rs:237-242`). The incremental path clones it again (`update.rs:106`) and once more into the language workspace (`update.rs:176`).

3. **`semantic_core` leaks presentation and transport concerns.** ELK layout, interconnection scene, full visualization projection, IBD layout, `WorkspaceRenderSnapshot`, and ~20 `*Dto` wire types are all exported from a crate that should only own graph + resolution + diagnostics + evaluation.

4. **`element_kind` is now a typed `ElementKind` enum** (`model.rs`) replacing the previous `element_kind: String`. ~158 remaining string comparisons (via `PartialEq<str>` impls) still defeat exhaustiveness checks — see P2-5.

5. **`SemanticNode.attributes: HashMap<String, serde_json::Value>` is a large untyped bag** (`model.rs:455`) with ~70+ distinct string keys (`multiplicity`, `redefines`, `value`, `portType`, `allocationType`, `analysisEvaluationStatus`, `evaluatedValue`, …). Keys are re-parsed on hot paths; typos fail silently.

6. **The diagnostics engine is one 760-line function with 20 sequential passes** (`engine_impl.rs:42-760`). Roughly 10 passes each re-call `graph.nodes_for_uri()` (fresh `Vec` alloc each time) and several walk *all* graph edges per URI (`graph.rs:631-709`). No per-document parallelism exists in the host path.

7. **Two divergent validation execution models.** `kernel/src/workspace/services.rs` hand-rolls `std::thread::spawn` worker pools at 6 sites; `spec42_host` validation is fully sequential. No `rayon` anywhere despite embarrassingly parallel per-document work.

8. **`String` is the pervasive error type below `spec42_host`.** `semantic_core`, `language_service`, `kernel`, and `server` return `Result<_, String>` widely; the host even matches by code string (`err.code() == "cancelled"`, `update.rs:50`). Only `spec42_host` has a `thiserror` enum.

9. **High `unwrap`/`expect` density:** semantic_core 351, kernel 162, server 84, host 41, language_service 12; plus 33 `panic!/todo!/unimplemented!/unreachable!` macros. A `Mutex` `.expect()` at `graph.rs:104` risks poison-panicking all analysis threads.

10. ~~**`SemanticGraph::insert_workspace_edge` (`graph.rs:787`) omits `invalidate_query_indexes()`**~~ **Fixed.** `invalidate_query_indexes()` has been added to `insert_workspace_edge`.

---

## 1. Architectural Boundaries

### 1.1 `kernel` duplicates `spec42_host` + `language_service` (P1)

`kernel/src/lib.rs` re-exports `semantic_core` for the data model but defines its own:
- `validation.rs:307`, `validation/pipeline.rs:248` — full validation pipeline
- `workspace/services.rs` (1,311 LOC) — workspace state machine
- `lsp_runtime/` — LSP adapter
- `language/` (1,226 LOC) — completion, hover, symbols

These mirror `spec42_host` + `language_service` with identical projection fields. The pipelines have already started diverging (kernel uses `tower_lsp::Range`; host uses `TextRange`).

**Recommendation:** Pick one runtime. Reduce `kernel` to a thin tower-lsp adapter over the host snapshot. Delete the duplicate validation pipeline and DTOs.

### 1.2 `semantic_core` leaks rendering/transport concerns (P2)

The following belong in `diagram_core` or a dedicated `sysml_views` crate, not in the core semantic engine:
- `dto.rs` (~20 `*Dto` wire types)
- `render_snapshot.rs`, `visualization/`
- `interconnection_elk.rs`, `interconnection_scene.rs`, `interconnection_projection.rs`
- `prepared_view/`
- `sequence_views/`, `state_views/`
- `ibd/` (layout concerns)

**Recommendation:** Move all visualization and DTO types out of `semantic_core`. Keep `semantic_core` to: graph, resolution, diagnostics, evaluation, and library loading.

### 1.3 No circular dependencies

The dependency graph is acyclic. This is good.

---

## 2. Data Duplication & Redundancy

### 2.1 ~~`SemanticGraph` cloned per snapshot, twice incrementally~~ **Fixed (P1-3)**

`HostWorkspaceSnapshot.semantic_graph` and `InMemoryWorkspace.semantic_graph` are now both `Arc<SemanticGraph>`. `build_workspace_snapshot` clones the `Arc` (pointer copy) instead of the graph. The incremental update path still performs one unavoidable deep clone (into a `mut` graph for mutation), then wraps the result in `Arc` and shares it into the language workspace — no second deep clone.

Remaining: incremental update (`update.rs`) still re-links all relationships and re-resolves all pending expressions across the entire workspace (P2-3).

### 2.2 Five parallel model representations (P2)

`SemanticGraph` → `HostSemanticProjection` → `SemanticModelProjection` (kernel) → `SysmlGraphDto`/`GraphNodeDto` → `WorkspaceModelDto`/`SysmlElementDto` → `PreparedViewDto`.

`ViewIndex` and `ModelExplorerBundle` each clone their own `SysmlGraphDto` (`render_snapshot.rs:110`, `view_index_to_artifacts`).

**Recommendation:** Define one canonical projected-node type. Cache `SysmlGraphDto` by render version rather than re-projecting it.

### 2.3 `WorkspaceRenderSnapshot` has duplicated fields (P3)

It re-stores `version`/`workspace_root_uri`/`workspace_uris` that are already inside `view_index` (`render_snapshot.rs:97-102`).

---

## 3. String-Heavy APIs

### 3.1 ~158 `element_kind` string comparisons survive (P2) — **fixed (production code)**

Despite the new `ElementKind` enum, most call sites in `semantic_core` used to write `element_kind == "port"` or match on `element_kind.as_str()`, enabled by the `PartialEq<str>` impls on `ElementKind`. This defeated the exhaustiveness guarantees the enum was introduced to provide.

**Fixed:** the "allowed target kinds" resolution/allowlist system (`kinds.rs`, `import_resolution.rs`, `relationships.rs`) and essentially all direct `element_kind` comparison sites across `sysml_model` (`diagnostics/**`, `graph_builder/**`, `reference_resolution.rs`, `activity_graph.rs`, `analysis_typing.rs`, `sequence_views`/`state_views` extractors, `evaluation`, `units/type_resolver.rs`, `resolution/mod.rs`, etc.), plus the equivalent call sites in `language_service` and `lsp_server`, now compare against `ElementKind::Variant` instead of string literals. Three duplicate `element_kind_allowed(&str)` helpers were consolidated into one `ElementKind`-typed function in `kinds.rs`.

**Intentionally left as string comparisons** (not an oversight — converting these would silently change behavior or requires a separate design decision):
- Literals with no corresponding `ElementKind` variant, which would only ever match `ElementKind::Unknown(s)` — e.g. `"frame"`, `"doc"`, `"comment"`, `"textualRep"`, `"library package"` in a few `diagnostics/checks/*.rs` and view-extractor spots.
- Generic suffix/substring checks that intentionally span many kinds by naming convention rather than an exact set, and the `ibd/extract_impl.rs` / `ibd/connectors.rs` predicates, `is_attribute_like_kind`'s `.contains("attribute")` in `lsp_server`, which operate on DTO-level `element_type: String` fields downstream of the graph, not `ElementKind` directly.
- `.as_str()`/`Display` usage for output formatting (error messages, sort keys) — not a comparison.

**Update:** `ElementKind::is_definition()` (model.rs) was added as the canonical, exhaustive-match replacement for the `element_kind.as_str().ends_with(" def")` / `.contains("_def")` / `.contains("definition")` heuristic that appeared in `diagnostics/relationship_endpoint_messages.rs`, `ibd/extract_impl.rs`, `prepared_view/graph_norm.rs`, and three `language_service` files — all now delegate to it (via `ElementKind::parse(kind).is_definition()` at the remaining `&str`/DTO boundaries). Use `is_definition()` for any future "is this a `xxx def` kind" check instead of a new suffix/substring heuristic.

**Recommendation:** `PartialEq<str>` on `ElementKind` is still needed for the intentional cases above — do not remove it. The remaining string-typed DTO layer (`PreparedNodeDto.kind`, `InterconnectionNodeDto.kind`, etc. — see section on DTOs) and the IBD extraction predicates are separate follow-up items.

### 3.2 Untyped `SemanticNode.attributes` bag (~70+ keys) (P2)

`model.rs:455` — `HashMap<String, serde_json::Value>` stores at least:
`typeRef`, `specializes`, `allocationType`, `allocationSource`, `allocationTarget`, `objectiveBindingKind`, `objectiveBoundTo`, `rawVerdictToken`, `generalViewDirectAttributes`, `generalViewInheritedAttributes`, `generalViewParts`, `generalViewPorts`, `evaluatedValue`, `evaluatedUnit`, `isReference`, `acceptType`, `lhs`, `rhs`, `redefines`, `subsetsFeature`, `metaclassRole`, `multiplicity`, `portType`, `analysisEvaluationStatus`, `firstSuccessionTarget`, `thenActionCount`, `objectiveCount`, `analysisResultCount`, `exposeTargets`, …

Keys are re-parsed on hot paths with `.get("key").and_then(|v| v.as_str())`. A typo silently produces `None`.

**Recommendation:** Introduce typed per-kind attribute structs (e.g. `PartAttributes`, `RequirementAttributes`) as an `enum NodeAttributes`. Keep a small `extra: HashMap<String, Value>` for forward compatibility. At minimum, replace string key literals with typed constants.

### 3.3 ~~`PendingRelationship.target_kinds: Option<Vec<String>>`~~ **Fixed (P3)**

`graph.rs:164` — now `Option<Vec<ElementKind>>`. Construction (`relationships.rs`) parses the incoming `&[&str]` allowlists via `ElementKind::parse`, and the final match against `tgt_node.element_kind` is a direct enum comparison instead of a string compare.

---

## 4. Performance Hotspots

### 4.1 ~~Diagnostics engine: 20 sequential passes, repeated graph scans~~ **Fixed (P2)**

`engine_impl.rs` now pre-collects `nodes` and `connect_edges` once at the top of `compute_semantic_diagnostics_with_unit_registry` and passes them to all passes. Edge lookup methods (`edges_for_uri`, `connect_statement_edges_for_uri`, `edges_for_uri_as_strings`) use `edges_by_uri` and `connect_edges_by_declaring_uri` indexes built in a single O(all_edges) pass during `build_query_indexes()`, then served in O(edges_in_uri) per query. The stale-cache hazard from direct `graph.graph.add_edge()` calls in all three mutation paths is addressed with explicit `invalidate_query_indexes()` calls.

Remaining: sub-check modules (passes 5–7, 14–18) still call `graph.nodes_for_uri(uri)` internally. A `DiagnosticsContext` struct could eliminate these too (Phase C).

### 4.2 Host validation is single-threaded; kernel uses hand-rolled threads (P2)

`facts.rs:38` iterates URIs sequentially. Diagnostics per document are fully independent and read-only over `&SemanticGraph`.

`kernel/src/workspace/services.rs` has 6 manual `std::thread::spawn` / join sites — error-prone, lacks backpressure.

**Recommendation:** Use `rayon::par_iter()` over the URI list in `facts.rs`. Drop kernel's manual thread pools. Also: `facts.rs:60` clones the entire `host_documents` vec just to compute a summary — eliminate that clone.

### 4.3 Incremental update is a full rebuild minus re-parsing (P2)

`update.rs:101-132`: clones the whole `SemanticGraph`, then `finalize_workspace_graph` re-links **all** relationships and re-resolves **all** pending expressions across the entire workspace (`pipeline.rs:77-81`), followed by a full render snapshot rebuild and full validation.

The only saving is skipping re-parse of unchanged documents. This means editing one file in a 1,000-file workspace rebuilds semantics for all 1,000.

**Recommendation:** Scope `finalize_workspace_graph` to the affected document's URI and its transitive dependents. Combine with the `Arc<SemanticGraph>` change from §2.1.

### 4.4 `import_lookup_cache` is fully invalidated on every mutation (P3)

`SemanticGraph`'s `import_lookup_cache` (`graph.rs:122,139`) is cleared wholesale on every structural change and reset to empty on every clone (`graph.rs:53`). It is therefore only effective within a single validation phase (no cross-edit caching).

**Recommendation:** Persist the cache across incremental updates; invalidate only entries whose prefix matches a mutated URI's packages.

### 4.5 ~~`insert_workspace_edge` missing query index invalidation (P1 — correctness)~~ **Fixed**

`graph.rs:787-795` previously added an edge without calling `invalidate_query_indexes()`. This has been corrected.

---

## 5. Error Handling

### 5.1 `String` as error type throughout (P2)

`Result<_, String>` is used in:
- `semantic_core`: `pipeline.rs:19`, `render_snapshot.rs:65,89`, `library_loader.rs`
- `kernel`: all of `validation/`, `views/`
- `server`: all entry points
- `spec42_host`: `update.rs:50` matches errors by string code (`err.code() == "cancelled"`)

This makes error categorisation at call sites fragile and prevents typed recovery.

**Recommendation:** Introduce per-crate `thiserror` error enums. Start with `semantic_core` (biggest surface) and `kernel`. Replace the string-code match in `update.rs` with a typed `CancelledError` variant.

### 5.2 High `unwrap`/`expect` density in production paths (P2)

- `semantic_core`: 351 uses
- `kernel`: 162 uses
- `server`: 84 uses
- `spec42_host`: 41 uses
- 33 `panic!/todo!/unimplemented!/unreachable!` macros

The `Mutex` guards in `SemanticGraph` use `.expect()` (`graph.rs:104`). A poisoned lock aborts all analysis for all clients.

**Recommendation:** Triage using `unwrap_or_else(|e| e.into_inner())` for mutex guards (poison recovery). Replace `unwrap()` on `Option` with `?` or `ok_or_else`. Document any `unreachable!()` with a proof it cannot be reached.

---

## 6. Graph Builder Architecture

### 6.1 Builder is large but coherent (P3 — low priority)

`build_graph_from_doc` (`graph_builder/mod.rs:48`) dispatches to ~30 sub-modules totalling ~9,488 LOC via nested `match` (no visitor pattern). This is understandable but any new SysML element type requires touching `mod.rs` plus adding a new file.

### 6.2 `kind: &str` threaded through builder instead of `ElementKind` (P2)

`add_node_and_recurse` and `qualified_name_for_node` receive `kind: &str` (`mod.rs:121,147,162`). A typo becomes `ElementKind::Unknown` at runtime with no compile-time error.

**Recommendation:** Change the builder's kind parameter to `ElementKind` everywhere.

### 6.3 Diagnostics emitted as synthetic graph nodes (P3)

`element_kind == "diagnostic"` nodes are inserted into the semantic graph during building (`engine_impl.rs:54-89`) and filtered back out during validation. This is a hack — the graph is being used as a side-channel for structured errors.

**Recommendation:** Return builder diagnostics through a typed `Vec<BuildDiagnostic>` side-channel, not as graph nodes.

### 6.4 `#kind` suffix encoding in qualified names (P3)

`mod.rs:138-142` — disambiguation suffixes like `::Foo#part_def` are silently embedded in qualified names and flow through projections, comparison, and persistence. This is a hidden string encoding.

**Recommendation:** Document the contract explicitly; consider a structured `QualifiedName { name: String, disambiguator: Option<ElementKind> }` type.

---

## 7. LSP / Server Layer

### 7.1 Two LSP runtimes (P1 — see §1.1)

Both `kernel/src/lsp_runtime/` and `server/src/` provide LSP handling, leading to feature drift.

### 7.2 Manual thread spawning with mutable shared state in kernel (P2)

`kernel/src/workspace/services.rs` spawns threads that mutate `ServerState` (a struct behind an `Arc<Mutex<_>>`). Combined with the large `ServerState` struct and many lock sites, this is a race-condition surface.

**Recommendation:** Adopt the `spec42_host` immutable snapshot model: hold `Arc<HostWorkspaceSnapshot>` and atomically swap the `Arc` on each update. No locks needed for readers.

### 7.3 Large legacy files may be dead code (P3)

- `server/src/legacy_elk_svg.rs` — 872 LOC
- `server/src/mcp/diagnostic_catalog.rs` — 867 LOC

Verify whether these are still reachable; retire if not.

---

## 8. Test Coverage

- **Strong:** `semantic_core` (46 test files), `spec42_host` (20 test files)
- **Thin:** `kernel` (3 integration tests), `language_service` (3 test files)
- **Snapshot tests:** referenced via `insta` in server, semantic_core, host — but `insta` does not appear in any `Cargo.toml` as a dev-dependency. Verify these tests actually run.
- **Missing:** no test verifying that incremental-update output equals full-rebuild output for the same document set (the `built_workspace_parity` test was already broken before this audit).

**Recommendation:** Add an incremental-vs-full property test. Confirm `insta` dev-dep is wired up. Raise `kernel` and `language_service` coverage or delete the duplicate pipelines they test.

---

## 9. Dependency Management

**Good:** `serde`, `serde_json`, `thiserror`, `petgraph`, `sysml-v2-parser` are hoisted into `[workspace.dependencies]`.

**Needs work:** `walkdir`, `sha2`, `toml`, `zip` are re-declared in multiple `Cargo.toml` files with independent version strings — drift risk. Hoist all into `[workspace.dependencies]`.

`tower-lsp 0.20` is only needed if the kernel LSP runtime stays. If kernel is collapsed into a thin adapter, this dependency can be removed from `semantic_core`/`spec42_host`.

---

## Prioritized Action List

### P1 — Critical (correctness or blocking debt)

| # | Finding | Location |
|---|---------|----------|
| ~~P1-1~~ | ~~Add `invalidate_query_indexes()` in `insert_workspace_edge`~~ **Done** | `graph.rs` |
| P1-2 | Eliminate the `kernel` duplicate validation stack | `kernel/src/validation/`, `kernel/src/workspace/services.rs` |
| ~~P1-3~~ | ~~`Arc<SemanticGraph>` — stop deep-cloning the graph on each snapshot/edit~~ **Done** | `build.rs`, `update.rs` |

### P2 — Important (performance, maintainability)

| # | Finding | Location |
|---|---------|----------|
| ~~P2-1~~ | ~~Refactor `engine_impl.rs` diagnostics into shared-input passes + index edges by URI~~ **Done** | `engine_impl.rs`, `graph.rs` |
| P2-2 | `rayon` par_iter for host validation; drop `host_documents.clone()` | `facts.rs:38,60` |
| P2-3 | Scope `finalize_workspace_graph` to changed URI + dependents | `update.rs:101`, `pipeline.rs:77` |
| P2-4 | `thiserror` enums for `semantic_core` and `kernel`; remove string-code matching | `update.rs:50`, `pipeline.rs:19` |
| P2-5 | `ElementKind` enum introduced ✓; still need: remove `PartialEq<str>` and thread `ElementKind` through builder | `model.rs:331`, `graph_builder/mod.rs:121` |
| P2-6 | Typed node attributes; replace `HashMap<String, Value>` | `model.rs:455` |
| P2-7 | Triage `unwrap`/`expect`; fix mutex poison risk | `graph.rs:104`, widespread |
| P2-8 | Extract rendering/DTO out of `semantic_core` into `diagram_core` | `semantic_core/src/semantic/dto.rs`, `visualization/`, `ibd/` |
| P2-9 | Replace `kernel`'s manual thread pools with the host immutable-snapshot model | `kernel/src/workspace/services.rs` |

### P3 — Nice to Have

| # | Finding | Location |
|---|---------|----------|
| P3-1 | Persist `import_lookup_cache` across incremental updates | `graph.rs:53,122,139` |
| P3-2 | Trim duplicated fields in `WorkspaceRenderSnapshot` | `render_snapshot.rs:97-102` |
| P3-3 | Typed builder-diagnostic side-channel (not graph nodes) | `engine_impl.rs:54-89` |
| P3-4 | ~~`PendingRelationship.target_kinds` → `Vec<ElementKind>`~~ **Fixed** | `graph.rs:164` |
| P3-5 | Hoist `walkdir`/`sha2`/`toml`/`zip` into workspace deps | Multiple `Cargo.toml` |
| P3-6 | Add incremental-vs-full property test; confirm `insta` wiring | `tests/` |
| P3-7 | Investigate and retire `legacy_elk_svg.rs` and `mcp/diagnostic_catalog.rs` if dead | `server/src/` |
| P3-8 | Structured `QualifiedName` type to replace `#kind` suffix encoding | `graph_builder/mod.rs:138` |
