# Tier 2 Phase 3b Step 5: Shared Full-Workspace Graph Build

**Status:** Step 5a landed 2026-07-02 — new function added and equivalence-tested, derivation
coverage gap closed. Step 5b landed 2026-07-03 — `workspace`'s full-build path
(`build_semantic_graph_from_documents`) now calls `build_and_link_graph_parallel`; a real
duplicate-edge bug was found and fixed along the way. Step 5c (migrate `lsp_server`'s
full-rebuild call sites) not started.
**Date:** 2026-07-02
**Related:** `docs/engineering/TIER2-PHASE3B-SHARED-GRAPH-PATCH-DESIGN.md` (Steps 1-4,
landed — this doc is that design's Step 5, split out into its own file given its size and
different risk profile), `docs/engineering/TIER2-LSP-WORKSPACE-CONSOLIDATION.md`.

## What this is

Steps 1-4 consolidated the *single-document* graph-patch sequence (`workspace`'s
incremental update, `lsp_server`'s per-edit patch). This doc covers the other duplicated
computation flagged as a follow-up: building the semantic graph from *many* documents at
once — `sysml_model::build_and_link_graph` (used by `workspace`'s full build: CLI initial
load, Babel42's `EditorSession::open()`, MCP tools) vs. `lsp_server::workspace::services.rs`'s
`rebuild_all_document_links` / `rebuild_semantic_graph_staged` / `merge_document_graphs_into`
(used at LSP startup and library reindex).

## The finding: this isn't just duplication, `workspace`'s version is also slower — and the fix is already documented

`sysml_model`'s own doc comments describe the fix, unprompted, right next to the code that
needs it:

```rust
// crates/sysml_model/src/semantic/relationships.rs:796-806
/// Wire derivation connections after a full parallel cross-document edge resolution.
///
/// When [`resolve_cross_document_edges_for_uri`] is run in parallel for every URI in
/// the workspace, it already resolves typing, specializes, and subject edges for all
/// nodes.  In that full-build path, calling [`link_workspace_relationships`] afterwards
/// redundantly re-resolves those same edges for all 1,681+ nodes.  Use this slimmer
/// variant at the full-build call sites to skip the redundant loops and only wire the
/// one thing the parallel phase does not cover: derivation connections.
///
/// The incremental update path (single-file change) still needs the full
/// [`link_workspace_relationships`] because only one URI's edges were refreshed.
pub fn link_workspace_derivations(g: &mut SemanticGraph) { ... }
```

That's precisely what `lsp_server`'s `rebuild_all_document_links` already does (parallel
per-URI `resolve_cross_document_edges_for_uri` via rayon, then `link_workspace_derivations`
only — not the full sequential `link_workspace_relationships`). But
`sysml_model::build_and_link_graph` — the function `workspace`'s full-build path actually
calls — still uses the slower sequential path: parses and merges every document one at a
time in a loop (no parallelism at all, unlike every rayon-converted `lsp_server` path from
Phase 3a), then calls the full `link_workspace_relationships`, redundantly re-resolving
edges the parallel technique would have handled directly.

**So this is the same shape as the `evaluate_expressions` bug from Steps 1-2**: a real
performance gap in `workspace`'s full-build path (affecting CLI, MCP, and Babel42's session
startup) that `lsp_server` already solved, documented in `sysml_model` itself, but never
ported back into the shared crate's own default implementation.

## Why this is bigger than Steps 1-4

Two things change at once here, not one:

1. **Parsing + graph-building parallelism** — `lsp_server`'s versions operate on
   *already-parsed* documents (`Vec<(Url, RootNamespace)>`, sourced from its own parse
   cache/index) and only need to parallelize `build_graph_from_doc` + merge.
   `build_and_link_graph` takes raw `&[SysmlDocument]` and parses inline — parallelizing it
   means also parallelizing `sysml_v2_parser::parse`, which none of the `lsp_server`
   functions needed to do.
2. **Linking strategy** — swapping `link_workspace_relationships` for the parallel
   cross-edge-resolution + `link_workspace_derivations` pattern changes *how* the same
   edges get computed, not just how many threads are used. Worth a dedicated equivalence
   test before relying on it (see below), even though the pattern is already documented
   and already proven correct in `lsp_server`'s production use.

There's also a real ordering dependency `lsp_server`'s functions don't have to deal with:
`build_and_link_graph` processes workspace documents first (collecting
`workspace_packages`, the declared-package-name set), *then* library documents (using that
completed set via `merge_skip_existing_qualified_names` to avoid a library definition
shadowing a workspace one). That two-phase structure has to stay — each phase can be
internally parallel, but phase 2 can't start until phase 1's `workspace_packages` set is
complete.

## Proposed shared primitive

In `sysml_model::semantic::pipeline`, alongside `build_and_link_graph`. Two new rayon
dependency additions needed: `sysml_model` and `workspace` (currently neither depends on
`rayon` — `lsp_server` does, since Phase 3a). `rayon` is **not** on `workspace`'s
`dependency_guardrails.rs` forbidden list (`tokio`/`clap`/`axum`/`rmcp`/`tower-lsp`/
`lsp_server` are; `rayon` isn't a protocol/runtime commitment the same way, it's a
compute-parallelism library) — worth double-checking that test still passes after adding it,
but no reason to expect it wouldn't.

```rust
/// Parses, builds, and links a semantic graph from many documents in parallel — the
/// full-workspace equivalent of `patch_graph_for_document`. Two phases (workspace
/// documents, then library documents) because library merging needs the complete set of
/// workspace-declared package names to avoid shadowing; each phase parses and builds in
/// parallel internally.
///
/// Uses the parallel cross-document-edge-resolution pattern documented on
/// `link_workspace_derivations` instead of the sequential `link_workspace_relationships` —
/// same resolved edges, computed via per-URI parallel resolution instead of a whole-graph
/// sequential scan.
pub fn build_and_link_graph_parallel(
    documents: &[SysmlDocument],
) -> (SemanticGraph, Vec<WorkspaceParsedDocument>) {
    let (workspace_docs, library_docs): (Vec<_>, Vec<_>) = documents
        .iter()
        .partition(|d| !matches!(d.source_kind, SysmlDocumentSourceKind::Library));

    let mut graph = SemanticGraph::new();
    let mut parsed_docs = Vec::new();

    // Phase 1: workspace documents, parallel parse + build + merge, collecting declared
    // package names along the way (needed before phase 2 can start).
    let workspace_parsed: Vec<_> = workspace_docs
        .par_iter()
        .filter_map(|doc| parse_and_build(doc))
        .collect();
    let workspace_packages: HashSet<String> = workspace_parsed
        .iter()
        .flat_map(|(_, parsed, _)| declared_packages_from_parsed(parsed))
        .collect();
    for (doc_graph, parsed, entry) in workspace_parsed {
        graph.merge(doc_graph);
        parsed_docs.push(entry);
    }

    // Phase 2: library documents, parallel parse + build, merged skipping anything the
    // workspace already declared.
    let library_parsed: Vec<_> = library_docs
        .par_iter()
        .filter_map(|doc| parse_and_build(doc))
        .collect();
    for (doc_graph, _parsed, entry) in library_parsed {
        graph.merge_skip_existing_qualified_names(doc_graph, &workspace_packages);
        parsed_docs.push(entry);
    }

    // Parallel cross-document edge resolution — see link_workspace_derivations's doc
    // comment. Covers what the sequential link_workspace_relationships would otherwise
    // redundantly re-resolve for every node.
    let uris: Vec<Url> = documents.iter().map(|d| d.uri.clone()).collect();
    let resolved_edges: Vec<_> = uris
        .par_iter()
        .flat_map(|uri| resolve_cross_document_edges_for_uri(&graph, uri))
        .collect();
    for (src, tgt, kind) in resolved_edges {
        if let (Some(&s), Some(&t)) = (graph.node_index_by_id.get(&src), graph.node_index_by_id.get(&tgt)) {
            graph.graph.add_edge(s, t, SemanticEdge::plain(kind));
        }
    }
    graph.invalidate_query_indexes();

    link_workspace_derivations(&mut graph);
    prepare_analysis_evaluation_context(&mut graph);
    resolve_workspace_pending_relationships(&mut graph);
    evaluate_expressions(&mut graph);
    graph.invalidate_query_indexes();

    (graph, parsed_docs)
}
```

(`parse_and_build` is a small helper doing parse-with-timing + `build_graph_from_doc`,
returning `None` on parse failure, mirroring the existing `let Ok(parsed) = ... else { continue }`
skip-on-failure behavior in the current sequential version.)

## Equivalence testing — before swapping anything, not after

Given this changes the linking *strategy*, not just adding threads, the migration should
lead with a direct comparison test, not just "run the existing suite and see if it still
passes" (which is necessary but not sufficient — the existing suite might not exercise every
edge kind `link_workspace_relationships` resolves).

**Step 5a — ✅ Done 2026-07-02.** Added `build_and_link_graph_parallel` to
`sysml_model::semantic::pipeline` (unused by any production call site — added standalone,
matching Step 1's approach), plus two equivalence tests:
- `parallel_build_matches_sequential_build_nodes_and_edges` — builds a 3-document fixture
  (two workspace documents, one library document, cross-document typing via
  `part mobility : MobilitySubsystem`, cross-document subject via
  `subject robot : AutonomousFloorCleaningRobot` resolved through `private import Architecture::*`,
  a same-document `:>` specializes edge, and library merge via
  `merge_skip_existing_qualified_names`) through both `build_and_link_graph` (old,
  sequential) and `build_and_link_graph_parallel` (new), and asserts the resulting node
  sets and edge sets (source, target, kind — every edge, not just attributes) are
  identical. **Passed on the first run** — resolves the open-question tension between the
  two contradictory doc comments (`resolve_cross_document_edges_for_uri`'s "legacy" warning
  vs. `link_workspace_derivations`'s newer guidance) in favor of the newer guidance, at
  least for typing/specializes/subject edges.
- `parallel_build_evaluates_expressions_like_sequential_build` — confirms both paths
  produce the same `evaluatedValue` for an evaluated attribute.

**Added `rayon` to `sysml_model`'s `Cargo.toml`** (resolving open question 3): checked
`sysml_model`'s own `dependency_guardrails.rs` (`sysml_model_stays_runtime_agnostic`, which
forbids `tokio`/`lsp_server`) — `rayon` isn't on that list, and the test suite (49 test
result lines, all `ok`) confirms adding it doesn't trip anything.

**Derivation-connection coverage gap — ✅ Closed 2026-07-02.** Extended
`equivalence_fixture_documents()` with a `StakeholderNeeds`/`SystemRequirements` document
pair (syntax mirrored from `requirement_derivation_semantics.rs`: a
`#derivation connection { end #original ::> cleanLargeAreas; end #derive ::> cleanAtLeastEighty; }`
block, cross-document via `private import StakeholderNeeds::*`). Added explicit assertions
in `parallel_build_matches_sequential_build_nodes_and_edges` that the
`("StakeholderNeeds::cleanLargeAreas", "SystemRequirements::cleanAtLeastEighty",
"derivation")` edge triple is present in both the sequential and parallel graphs — not just
relying on the blanket edge-set equality — so the test now demonstrably exercises the
`link_workspace_derivations(&mut graph)` call inside `build_and_link_graph_parallel`, the
one thing the parallel cross-edge-resolution phase does not itself cover. Passed on the
first run. Full `cargo test -p sysml_model`, `cargo check --workspace`, `cargo test
--workspace`, and `cargo clippy -p sysml_model --no-deps` all clean.

**Remaining known gap, not blocking**: no large realistic fixture (e.g. `robot_vacuum`) was
used for this pass — worth adding before 5b if one is readily available, to catch anything
the small hand-written fixture misses at the scale the "1,681+ nodes" comment references.

Full `cargo test -p sysml_model` (49 test binaries), `cargo check --workspace`, `cargo test
--workspace`, and `cargo clippy -p sysml_model` all clean.

**Step 5b — ✅ Done 2026-07-03.** Swapped `build_semantic_graph_from_documents` (in
`sysml_model::semantic::workspace_graph`) to call `build_and_link_graph_parallel` instead of
`build_and_link_graph`, wrapped in `Ok(...)` since the parallel function is infallible where
the sequential one returned `Result`. This is `workspace`'s full-build path (also used by
CLI, MCP, Babel42) — the actual perf win lands here.

**Found and fixed a real duplicate-edge bug during the swap.** `cargo test -p sysml_model`
caught a regression immediately:
`attribute_def_quantities::attribute_def_quantity_specialization_resolves_in_workspace`
failed — a single-document `attribute def Voltage :> ElectricPotentialDifferenceValue`
specializes edge came out doubled (2 typing edges instead of 1). Root cause:
`resolve_cross_document_edges_for_uri` resolves typing/specializes/subject refs for *every*
node in a URI, not only refs whose target lives in another document — for a same-document
reference, `build_graph_from_doc` had already wired the identical edge during per-document
graph building. `build_and_link_graph_parallel` was adding the resolved edges via a raw
`graph.graph.add_edge(...)`, which doesn't dedupe, whereas the sequential path's
`link_workspace_relationships` calls `add_typing_edge_for_node`/`add_specializes_edges_for_node`,
both of which go through `add_semantic_edge_once` (`relationships.rs:556`) and skip re-adding
an edge that already exists. **This is why the Step 5a equivalence test didn't catch it**:
`edge_triples()` returns a `BTreeSet`, so a duplicated edge with an identical
(source, target, kind) triple collapses to one set entry — the set-equality assertions were
blind to edge *count*, only edge *identity*. Fixed by routing
`build_and_link_graph_parallel`'s resolved-edge insertion through `add_semantic_edge_once`
instead of a raw `add_edge` (`pipeline.rs`, in the parallel cross-document edge resolution
block). Re-ran the full `sysml_model` suite after the fix — all 199+ tests pass, including
the previously-failing one.

*Note for future equivalence-style tests in this codebase*: comparing edge sets via a
`BTreeSet` of triples verifies edge *identity*, not edge *count* — a duplicate-insertion bug
where the duplicate has the same (source, target, kind) as an already-correct edge will not
show up as a set difference. A multiset/count comparison (or an explicit assertion on
`graph.graph.edge_count()`) would have caught this at Step 5a instead of at the production
swap.

**Verification**: `cargo check -p sysml_model`, `cargo test -p sysml_model` (all green,
including the fixed regression), `cargo check --workspace` (only the pre-existing unrelated
`SemanticLifecycle` unused-import warning in `lsp_server`), `cargo test --workspace` (all
green), `cargo clippy -p sysml_model --no-deps` and `cargo clippy -p workspace --no-deps`
(both clean).

**Perf measurement — honest limitation, not measured against a realistic fixture.**
`workspace/tests/incremental_benchmark.rs`'s
`benchmark_single_document_incremental_vs_full_rebuild` (an 8-file in-memory fixture, no
external download needed) ran clean post-swap (`full=17ms incremental=13ms`, debug build),
but that fixture is too small and the run is debug-mode, so it isn't a meaningful signal for
the parallel build's actual win. `robot_vacuum_performance.rs`'s benchmarks require fetching
an external fixture via `scripts/fetch-robot-vacuum-cleaner.sh` (network access), which
wasn't run as part of this step — so **no before/after timing comparison on a realistic
workspace has been captured yet**. The swap is verified correct (equivalence test + full
test suite), not yet verified faster in practice. Worth doing as a explicit follow-up before
claiming a perf win in the audit doc.

**Step 5c** — Migrate `lsp_server::rebuild_all_document_links` and the
`merge_document_graphs_into`/`rebuild_semantic_graph_staged` pair to delegate to
`build_and_link_graph_parallel` for the core graph-build-and-link computation, keeping
everything `sysml_model` doesn't have a concept of as `lsp_server`-local wrapping:
- `include_in_semantic_graph`-split (search-only library indexing)
- Symbol table rebuild (`refresh_symbols_start` section)
- The staged/lock-free-commit shape (`rebuild_semantic_graph_staged` returning
  `(graph, symbols, metrics)` for `ServerState` to commit without holding a write lock
  during the heavy work)
- `RebuildAllDocumentLinksMetrics`'s detailed per-phase timings — either kept as
  `lsp_server`-local instrumentation wrapping calls into the shared function, or (bigger,
  optional, not designed here) `build_and_link_graph_parallel` could return its own timing
  breakdown if that's valuable to more than one caller.

Run `lsp_server`'s full test suite (278 tests) after this step, same rigor as Step 4.

## What stays `lsp_server`-specific (unchanged from the Steps 1-4 doc's framing)

Same list as before: `include_in_semantic_graph`, incremental symbol-table maintenance,
`library_graph_snapshot` caching, `ServerState`/`SemanticCoordinator`. This step only
targets the "build/link a graph from many documents" computation itself.

## Open questions

1. Does `resolve_cross_document_edges_for_uri`'s `CASE_KINDS` handling, when run for every
   URI in the workspace, produce results identical to `link_case_subject_relationships`
   (called inside the full `link_workspace_relationships`)? The `link_workspace_derivations`
   doc comment asserts yes ("already resolves typing, specializes, and subject edges for
   all nodes") — Step 5a's equivalence test should verify this directly rather than take it
   on faith, especially since `resolve_cross_document_edges_for_uri`'s own doc comment
   calls it "legacy" and recommends against full-build usage, which is in tension with the
   newer `link_workspace_derivations` comment recommending exactly that combination. One of
   these two comments is stale; the equivalence test settles which.
2. Is there an existing large fixture (robot_vacuum?) worth using for the equivalence test,
   given the "1,681+ nodes" figure in the doc comment suggests this was already validated
   against something similarly sized?
3. Should `rayon` be added to `sysml_model` directly, or only to `workspace` (with
   `sysml_model` staying rayon-free and the parallel primitive living in `workspace`
   instead)? `sysml_model` has no equivalent of `workspace`'s `dependency_guardrails.rs`, so
   there's no hard constraint — but worth deciding deliberately rather than defaulting to
   "wherever's convenient," given `sysml_model` is the most widely-depended-on crate in the
   workspace and a new dependency there has the largest blast radius.
