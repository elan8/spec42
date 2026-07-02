# Tier 2 Phase 3b (redirected): Shared Single-Document Graph-Patch Primitive

**Status:** Steps 1-4 landed 2026-07-02 — `workspace` and `lsp_server` both now delegate to
the shared primitive; the duplication this doc set out to fix is resolved. Step 5
(full-rebuild-path follow-up) not started.
**Date:** 2026-07-02
**Related:** `docs/engineering/TIER2-LSP-WORKSPACE-CONSOLIDATION.md` (Phase 3b),
`docs/engineering/TIER2-PHASE3B-LAZY-SNAPSHOT-DESIGN.md` (parked — see "Relationship to
the parked lazy-snapshot design" below), `docs/architecture-audit.md` P2-3.

## Why this doc exists, and why the previous one is parked

The lazy-snapshot design (`TIER2-PHASE3B-LAZY-SNAPSHOT-DESIGN.md`) was motivated by a real
problem in Babel42's usage of `workspace` crate. Direction changed 2026-07-02: **priority
is consolidating spec42's own codebase (`lsp_server` + `workspace` + CLI/MCP) onto a shared
API, not fixing Babel42's performance right now.** Babel42 is explicitly left as-is for the
moment.

With Babel42 off the table, the actual consolidation target is narrower than "make
`HostWorkspaceSnapshot` lazy" — `lsp_server` doesn't use `HostWorkspaceSnapshot` at all, so
that design doesn't help it. What's genuinely duplicated between `workspace` and
`lsp_server` at the *computation* level (not the bundle/laziness level) is one specific
sequence: patching the semantic graph for a single changed document. This doc designs
extracting that sequence into one shared function both crates call.

## The duplication, precisely

**`workspace::snapshot::update.rs::try_incremental_update`** (lines 94-134):
```rust
let mut graph = (*previous.semantic_graph()).clone();   // owned clone, since `previous` is shared
graph.remove_nodes_for_uri(&uri);
if let Ok(parsed) = sysml_v2_parser::parse(&changed.content) {   // parses inline
    let doc_graph = build_graph_from_doc(&parsed, &uri);
    graph.merge(doc_graph);
    add_cross_document_edges_for_uri(&mut graph, &uri);
}
finalize_workspace_graph(&mut graph);   // always runs — no fast-path
```

**`lsp_server::workspace::services.rs::update_semantic_graph_for_uri`** (lines 203-221):
```rust
state.semantic_graph.remove_nodes_for_uri(uri);
if let Some(doc) = doc {   // takes already-parsed content, parsing happens earlier (parse_cache)
    let new_graph = semantic::build_graph_from_doc(doc, uri);
    state.semantic_graph.merge(new_graph);
    semantic::add_cross_document_edges_for_uri(&mut state.semantic_graph, uri);
    if evaluate {   // fast-path: skip relink/resolve/evaluate, defer to async relink
        semantic::link_workspace_relationships(&mut state.semantic_graph);
        semantic::resolve_workspace_pending_relationships(&mut state.semantic_graph);
        semantic::evaluate_expressions(&mut state.semantic_graph);
        state.semantic_graph.invalidate_query_indexes();
    }
}
```

Same computation, five real differences:

1. **Ownership**: `workspace`'s version clones the graph first (needed — `previous` is
   `Arc`-shared, immutable); `lsp_server`'s mutates `&mut state.semantic_graph` directly
   (it's the sole owner). Shared primitive should take `&mut SemanticGraph` — the more
   general shape — and let each caller decide whether to clone before calling.
2. **Parse timing**: `workspace` parses inline; `lsp_server` takes pre-parsed
   `Option<&RootNamespace>` (parsing already happened, possibly cache-hit). Shared
   primitive should take pre-parsed input, matching `lsp_server`'s shape — more composable,
   and `workspace`'s call site just parses first (one line, no real cost).
3. **The `evaluate: bool` fast-path**: `lsp_server` has it (skip relink/resolve/evaluate on
   rapid edits, catch up later via async relink); `workspace` doesn't. Shared primitive
   needs this parameter — `workspace`'s call sites can always pass `true` if they don't
   need the fast-path, but `lsp_server` needs `false` to be available.
4. **`finalize_workspace_graph` vs. the explicit three calls**: `finalize_workspace_graph`
   is `link_workspace_relationships` + `prepare_analysis_evaluation_context` +
   `resolve_workspace_pending_relationships` + `invalidate_query_indexes` — **no
   `evaluate_expressions`**. `lsp_server`'s explicit sequence has the same three
   (`prepare_analysis_evaluation_context` isn't called, but its absence didn't show up as a
   test failure — worth confirming whether that call matters for `lsp_server`'s scenarios
   before assuming it's dispensable) **plus `evaluate_expressions`**.
5. **The bug**: because of #4, `workspace`'s graphs — both full-build and incremental —
   never get expression evaluation. See the "Bug" section above this doc's own intro.

## Proposed shared primitive

Lives in `sysml_model` crate (`crates/sysml_model/src/semantic/pipeline.rs`, alongside the
existing `finalize_workspace_graph`/`build_and_link_graph`), not `workspace` crate — this is
pure graph logic with no `workspace`-specific types (`HostWorkspaceSnapshot`,
`Spec42Engine`) and no LSP-specific types either. Both `workspace` and `lsp_server` already
depend on `sysml_model` directly, so this requires **zero new crate dependencies** on either
side.

```rust
/// Patches `graph` in place for a single changed document: removes that document's
/// existing nodes, rebuilds and merges its subgraph (if parsed content is provided), and
/// refreshes cross-document edges touching it.
///
/// When `evaluate` is `true`, also relinks workspace relationships, resolves pending
/// relationships, and re-evaluates expressions across the graph. Pass `false` to skip
/// those steps for a fast, low-latency patch (e.g. on every keystroke) and call
/// [`finalize_and_evaluate`] later once edits settle.
pub fn patch_graph_for_document(
    graph: &mut SemanticGraph,
    uri: &Url,
    parsed: Option<&RootNamespace>,
    evaluate: bool,
) {
    graph.remove_nodes_for_uri(uri);
    let Some(parsed) = parsed else { return };
    let doc_graph = build_graph_from_doc(parsed, uri);
    graph.merge(doc_graph);
    add_cross_document_edges_for_uri(graph, uri);
    if evaluate {
        finalize_and_evaluate(graph);
    }
}

/// [`finalize_workspace_graph`] plus expression evaluation. Use this (not
/// `finalize_workspace_graph` directly) wherever a graph needs to be in a fully
/// up-to-date, query-ready state — i.e. everywhere `finalize_workspace_graph` is called
/// today except `lsp_server`'s explicit fast-path, which already skips this on purpose.
pub fn finalize_and_evaluate(graph: &mut SemanticGraph) {
    finalize_workspace_graph(graph);
    evaluate_expressions(graph);
    graph.invalidate_query_indexes();  // finalize_workspace_graph already does this once;
                                        // cheap, and evaluate_expressions may add pending-
                                        // relationship-adjacent state worth re-indexing —
                                        // confirm during implementation whether the second
                                        // call is actually necessary or just belt-and-braces.
}
```

**The bug fix rides along naturally here**: `build_and_link_graph` (full build) and
`try_incremental_update`'s `finalize_workspace_graph(&mut graph)` call both become
`finalize_and_evaluate(&mut graph)` instead — one-line changes, fixing the missing-evaluation
gap for every `workspace`-crate consumer (CLI, MCP, and, incidentally, Babel42, even though
Babel42 isn't the reason we're doing this).

## Call-site changes

**`workspace::snapshot::update.rs::try_incremental_update`**:
```rust
let mut graph = (*previous.semantic_graph()).clone();
let parsed = sysml_v2_parser::parse(&changed.content).ok();
patch_graph_for_document(&mut graph, &uri, parsed.as_ref(), true);
```
(Replaces the manual `remove_nodes_for_uri` + `build_graph_from_doc` + `merge` +
`add_cross_document_edges_for_uri` + `finalize_workspace_graph` sequence.)

**`workspace::snapshot::pipeline` (full build, inside `build_and_link_graph`)**: no call-site
change needed beyond swapping `finalize_workspace_graph(&mut graph)` →
`finalize_and_evaluate(&mut graph)` at the end — the per-document loop there already does
its own `build_graph_from_doc`+`merge` (building from many documents at once, not patching
one at a time), so it doesn't call `patch_graph_for_document` — that function is specifically
the *single-document* case.

**`lsp_server::workspace::services.rs::update_semantic_graph_for_uri`**:
```rust
fn update_semantic_graph_for_uri(state: &mut ServerState, uri: &Url, doc: Option<&RootNamespace>, evaluate: bool) {
    sysml_model::patch_graph_for_document(&mut state.semantic_graph, uri, doc, evaluate);
}
```
Collapses to a one-line delegation. `apply_document_changes_impl` (services.rs:457) calls
this same function — no change needed there beyond the fact that `update_semantic_graph_for_uri`
itself shrinks.

## What this does *not* touch

- The full-rebuild-from-many-documents paths (`rebuild_all_document_links`,
  `rebuild_semantic_graph_staged`, `merge_document_graphs_into` in `lsp_server`, vs.
  `build_and_link_graph` in `sysml_model`) — same kind of duplication, larger, used for
  startup/library-reindex rather than per-edit. Real follow-up, not designed here.
- `lsp_server`'s parse caching, library-graph caching, `ServerState`/`SemanticCoordinator`
  machinery — unrelated to this specific duplicated computation, stays as-is.
- Babel42 — explicitly deprioritized per current direction. This fix benefits Babel42
  incidentally (the evaluation bug fix) but nothing here is designed for Babel42's
  performance profile.
- The lazy-snapshot / `SnapshotDelta` / per-view-scoping design — parked, not superseded.
  If Babel42 work resumes later, that doc is still the starting point; this doc solves a
  different, narrower problem discovered along the way.

## Compatibility check

- `lsp_server`'s test suite (`services.rs`'s own `#[cfg(test)] mod tests`, 12 tests
  including `store_document_text_persists_evaluated_attributes`,
  `store_document_text_evaluates_unit_conversions`) already asserts on evaluated values
  through the *existing* explicit-call-sequence path — after the refactor, these same
  assertions should hold through the delegated `patch_graph_for_document` call. If any of
  them fail, that's a real behavioral difference between the old explicit sequence and the
  new shared one (e.g. the `prepare_analysis_evaluation_context` call `lsp_server` currently
  skips) that needs to be understood before shipping, not papered over.
- `workspace`'s existing tests (`incremental_parity.rs`, `incremental_fallback.rs`,
  `incremental_benchmark.rs`) should be extended to assert `evaluatedValue`/
  `evaluationStatus` are present after both a full build and an incremental update — this
  is new coverage, since (per the bug write-up above) nothing today checks this.

## Migration plan

**Step 1 — ✅ Done 2026-07-02.** Added `patch_graph_for_document` and `finalize_and_evaluate`
to `sysml_model::semantic::pipeline`, re-exported from the crate root
(`sysml_model::{patch_graph_for_document, finalize_and_evaluate}`) alongside the existing
`build_and_link_graph`/`finalize_workspace_graph`. Not yet wired into either caller (that's
Steps 3-4) — added standalone with 4 new unit tests in `pipeline.rs`:
- `patch_with_none_clears_the_uris_nodes`
- `patch_matches_manual_build_merge_and_cross_edges` (asserts the new function produces the
  identical node set as the manual `build_graph_from_doc` + `merge` +
  `add_cross_document_edges_for_uri` sequence it's meant to replace)
- `evaluate_false_skips_expression_evaluation`
- `evaluate_true_populates_evaluated_value` (`mass = 1 + 2` → `evaluatedValue: 3`)

All pass; full `cargo test -p sysml_model` and `cargo check --workspace` clean, `cargo
clippy -p sysml_model` clean.

**Step 2 — ✅ Done 2026-07-02.** Swapped `finalize_workspace_graph` → `finalize_and_evaluate`
in `sysml_model::build_and_link_graph` (full build — fixes the bug for every consumer that
goes through it, not just `workspace`) and in `workspace::snapshot::update.rs::try_incremental_update`
(incremental). Added two regression tests to `crates/workspace/tests/incremental_parity.rs`:
`full_build_populates_evaluated_attributes` and
`incremental_update_populates_evaluated_attributes` (both assert `mass = 1 + 2` →
`evaluatedValue: 3` — would have failed before this fix, confirmed passing after). Full
`cargo test -p workspace`, `cargo test --workspace`, `cargo check --workspace`, and `cargo
clippy -p workspace -p sysml_model` all clean, zero regressions in existing tests (nothing
depended on the buggy no-evaluation behavior).

**Step 3 — ✅ Done 2026-07-02.** Migrated `try_incremental_update`
(`crates/workspace/src/snapshot/update.rs`) to call `patch_graph_for_document` instead of
its manual `remove_nodes_for_uri`/`build_graph_from_doc`/`merge`/
`add_cross_document_edges_for_uri`/`finalize_and_evaluate` sequence — behavior-neutral as
expected, confirmed mechanical: compiled clean on the first attempt, and all 4
`incremental_parity.rs` tests (including the two evaluated-value regression tests added in
Step 2) pass unchanged through the new code path. `build_graph_from_doc`,
`add_cross_document_edges_for_uri`, and `finalize_and_evaluate` are no longer imported
directly in `update.rs` — only `patch_graph_for_document` is. Full `cargo test -p
workspace`, `cargo test --workspace`, `cargo check --workspace`, `cargo clippy -p
workspace` all clean.

**Step 4 — ✅ Done 2026-07-02.** Migrated `lsp_server::update_semantic_graph_for_uri`
(`crates/lsp_server/src/workspace/services.rs`) to a one-line delegation:
`semantic::patch_graph_for_document(&mut state.semantic_graph, uri, doc, evaluate)`,
collapsing what was ~19 lines of duplicated sequence. Wiring: `patch_graph_for_document`/
`finalize_and_evaluate` added to `workspace::semantic`'s re-export list (a thin
`pub use sysml_model::{...}` shim, matching the existing pattern every other
`workspace::semantic` re-export already uses) and then to `lsp_server`'s own
`crate::semantic` re-export list (`pub use workspace::semantic::{...}`) — no new crate
dependency needed anywhere, following the crate's existing "protocol-neutral logic lives in
`workspace`, `lsp_server` re-exports it" convention rather than reaching around it to
`sysml_model` directly.

**Resolved the open question**: `lsp_server`'s old explicit sequence skipped
`prepare_analysis_evaluation_context` (which `finalize_and_evaluate` includes via
`finalize_workspace_graph`). Full `lsp_server` test suite — 278 tests
(122+5+148+3, unchanged counts) — passes with zero failures through the new code path,
confirming that extra call is harmless for every scenario the test suite covers.

Full `cargo check --workspace`, `cargo test --workspace`, `cargo clippy -p lsp_server` all
clean. `services.rs` 1225 → 1214 lines — a small further reduction on top of Phase 3a, but
the more important outcome is that the actual duplicated *computation* (not just line
count) now has exactly one implementation, shared by both crates.

**Step 5 (follow-up, not this doc)** — Same treatment for the full-rebuild-from-many-documents
duplication (`rebuild_all_document_links`/`rebuild_semantic_graph_staged` vs.
`build_and_link_graph`).

## Open questions

1. Does `prepare_analysis_evaluation_context` (called by `finalize_workspace_graph`, hence
   by `workspace`, but *not* called by `lsp_server`'s explicit sequence today) matter for
   `lsp_server`'s scenarios? If `lsp_server`'s tests still pass once it starts going through
   `finalize_and_evaluate` (which includes this call), that's fine — it means `lsp_server`
   was just missing a call to something either harmless-if-run-twice or beneficial-and-
   previously-absent, not that adding it breaks something. If a test fails or behavior
   changes, that needs investigation before Step 4 ships.
2. Should `finalize_and_evaluate`'s second `invalidate_query_indexes()` call be removed if
   it proves redundant with the one already inside `finalize_workspace_graph`? Marked as a
   detail to confirm during implementation, not a blocking design question.
3. Is the evaluation-bug fix (Step 2) worth shipping standalone, ahead of the rest of this
   consolidation, given it's a real correctness gap independent of any duplication concern?
   It's a small, isolated change (`finalize_workspace_graph` → `finalize_and_evaluate` at
   two call sites) that could land on its own timeline if it matters more urgently.
