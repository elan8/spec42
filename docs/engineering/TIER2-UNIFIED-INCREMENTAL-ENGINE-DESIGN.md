# Tier 2: Unified Incremental Engine — Move `lsp_server`'s Incremental Machinery Into `workspace`, Layer Snapshot On Top

**Status:** Phases 1-4 landed 2026-07-03 — see the phase write-ups below. Phase 4 shipped in
a narrower shape than originally planned: `lsp_server`'s two full-rebuild functions now
delegate their graph computation to `IncrementalWorkspace`, but `ServerState` still holds a
plain `SemanticGraph` field (not the engine itself) — see "Phase 4" below for why that's the
right call, not a shortfall. Along the way, `sysml_model` gained
`link_parsed_documents_parallel`/`_from` (the merge/link half of
`build_and_link_graph_parallel`, usable with already-parsed documents and an optional base
graph), closing the Phase 2 parse-cache gap for real. **Phase 5 dropped 2026-07-03 — see
"Phase 5" below.**
**Date:** 2026-07-03
**Related:** `docs/engineering/TIER2-LSP-WORKSPACE-CONSOLIDATION.md` (Phases 1-3a, Phase 3b
Steps 1-4, Step 5a-5c, Phase 4 all landed — this doc addresses the split those phases
deliberately left alone), `docs/engineering/TIER2-PHASE3B-LAZY-SNAPSHOT-DESIGN.md` (parked;
its Part D was the basis for the now-dropped Phase 5 — see "Phase 5" below for why it stays
parked), `docs/architecture-audit.md` Tier 2.

## Problem: two independently-implemented "keep the graph up to date" pipelines

Everything shipped in Tier 2 so far (Phases 1-3a, 3b Steps 1-4, Step 5a-5c) pulled shared
*graph algorithms* down into `sysml_model` — `patch_graph_for_document`,
`finalize_and_evaluate`, `build_and_link_graph_parallel` — and had both `workspace` and
`lsp_server` call the same primitives instead of hand-copied sequences. That fixed three real
bugs (a missing `evaluate_expressions` call, a duplicate-edge insertion, a missing
`prepare_analysis_evaluation_context` call), each caused by the same shape of problem: two
paths doing "the same thing" via independently-maintained sequences that quietly drifted.

What Tier 2 deliberately did *not* touch is the layer above the algorithms: **`workspace` and
`lsp_server` still maintain two structurally different, independently-implemented engines for
"keep a semantic graph up to date as documents change over time."**

- `workspace`'s version (`snapshot/build.rs`, `snapshot/update.rs`) is eager-first: a full
  build computes a `HostWorkspaceSnapshot` from scratch; incremental update
  (`try_incremental_update`) is a secondary path, still gated behind
  `experimental_incremental_updates` — a feature flag, not the default.
- `lsp_server`'s version (`ServerState`, `parse_cache.rs`, `library_graph_cache.rs`,
  `services.rs`) is incremental-first and production-hardened — it's the live LSP hot path,
  handling every keystroke in every connected editor — but has no equivalent of
  `workspace`'s snapshot value type or its CLI/MCP/HTTP consumers.

Neither reuses the other's caching, indexing, or update sequencing beyond the low-level
algorithms Tier 2 already unified. Babel42 (a separate repo, a live web editor built directly
on `workspace`'s snapshot API with `experimental_incremental_updates(true)`) inherits the
weaker of the two: it gets `workspace`'s secondary incremental path, not `lsp_server`'s
mature one, and pays for it — full `language_workspace`/`render_snapshot`/`semantic_projection`
recomputation on every edit (documented in `ROBOT-VACUUM-PERFORMANCE-ANALYSIS.md`-adjacent
findings from earlier investigation, not yet fixed).

This doc proposes closing that gap by inverting which side is primary: move the portable
parts of `lsp_server`'s incremental engine into `workspace`, make it the one path every
consumer uses, and redefine the eager "snapshot" API as a thin view taken off that engine's
current state instead of an independent computation.

## Decision: no separate crate

Considered: extract a `snapshot` crate (the eager-computation half of `workspace` —
`snapshot/`, `comparison/`, `engine.rs`, `robot_vacuum_perf.rs`, ~3,170 of `workspace`'s
5,827 lines by direct count) so `workspace` itself stays a lean, protocol-neutral core.
**Rejected, for two reasons:**

1. **Precedent.** `sysml_model` keeps parsing, graph building, diagnostics, view rendering,
   and IBD extraction — all logically distinct — in one crate with module boundaries and a
   test-enforced dependency guardrail (`sysml_model/tests/dependency_guardrails.rs`), not
   separate crates. `workspace`'s own `session`/`library`/`snapshot`/`comparison` split
   already follows that convention. A new crate boundary here would be the inconsistent
   choice, not the natural one.
2. **This codebase already tried "stand up a clean new crate, migrate consumers over"
   once and it didn't finish** — `workspace` crate itself was introduced (as `spec42_host`,
   2026-06-22) as a "host embedding API" alongside the already-existing `kernel` (now
   `lsp_server`), with the implicit expectation that `kernel` would eventually migrate onto
   it. It never did — `lsp_server` kept its own independent reimplementation, which is the
   entire reason Tier 2 exists. See `TIER2-PHASE3B-LAZY-SNAPSHOT-DESIGN.md`'s
   "Architectural approach" section, which rejected a *different* new-crate proposal for the
   same reason. A second attempt needs a real reason to believe it finishes this time; moving
   pieces within an existing crate (as Phase 1's `WorkspaceSession` did, successfully) has a
   better track record here.

More fundamentally: once the eager-build pipeline stops being an independent computation (see
next section), there's less reason to wall it off behind a crate boundary in the first place.

## What happens to "snapshot"

Split into two separable questions with different answers.

**The eager-build *pipeline* disappears.** Today `build_and_link_graph` /
`build_and_link_graph_parallel` (full build) and `try_incremental_update` (patch) are two
independently-implemented paths kept in sync by hand — the exact shape that produced all
three bugs Tier 2 has fixed so far: one path did something the other forgot. If there is only
one engine, a full build becomes the degenerate case of incremental update: *new engine,
apply one changeset containing every document, done.* There is no second pipeline left to
drift out of sync with.

**The snapshot *value type* survives**, because stateless callers (CLI invocations, most
MCP/HTTP requests) still want "give me a frozen, cheaply-shareable answer," not "hand me a
live session I have to manage the lifecycle of." So `HostWorkspaceSnapshot` stays, but
demoted: not an independently-computed thing, but a read-only view taken off the engine's
current state — closer to an `engine.snapshot()` method than a build pipeline.

## Proposed layering inside `workspace`

### Layer 1 — the engine (new; absorbs `session.rs` plus two `lsp_server` modules)

A new stateful type — call it `IncrementalWorkspace` pending a better name — wrapping:

- `SemanticGraph` (already `Arc`-backed, cheap to clone/share).
- A document index generalizing `lsp_server`'s `IndexEntry` (uri → parsed doc, content,
  parse metadata, `include_in_semantic_graph` flag).
- **`parse_cache.rs`** (currently `lsp_server/src/workspace/parse_cache.rs`, 243 lines):
  SHA-256-keyed disk cache of parsed `RootNamespace` values. Verified portable as-is — no
  `tokio`, and its one `Url` reference (`tower_lsp::lsp_types::Url`) is confirmed to be a
  direct `pub use url::Url` re-export, i.e. the exact type `workspace` already depends on.
  Zero type-shimming needed to relocate it.
- **`library_graph_cache.rs`** (currently `lsp_server/src/workspace/library_graph_cache.rs`,
  369 lines): disk cache of the fully-built library subgraph, two-level invalidation
  (path-config hash + per-file size/mtime fingerprint). Same portability check passed —
  only `tower_lsp::lsp_types::Url` (= `url::Url`), no `tokio`.
- Lifecycle/generation tracking — either `WorkspaceSession`'s existing state machine
  (Cold/Indexing/Ready/Reindexing) becomes this type's internal bookkeeping, or stays a
  separate composed type; not decided here.

Exposes roughly:

```rust
impl IncrementalWorkspace {
    fn apply_changes(&mut self, changes: DocumentChanges) -> UpdateOutcome { ... }
    fn current_graph(&self) -> SemanticGraph { ... } // Arc clone, cheap
    fn snapshot(&self) -> HostWorkspaceSnapshot { ... } // Layer 2
}
```

Stays fully synchronous — no `tokio`, same guardrail `workspace` already enforces
(`workspace/tests/dependency_guardrails.rs`). Async wrapping (background rebuild,
cancellation, staged/lock-free commit under concurrent request handling) stays local to each
embedder — `lsp_server` keeps its own `tokio::sync::watch` layer, the same pattern
`SemanticCoordinator` already uses on top of `WorkspaceSession` today (Phase 2).

**What does *not* move**: the incremental symbol table and `include_in_semantic_graph`-split
library search indexing (`library_search.rs`, 336 lines) are genuinely LSP/completion-specific
with no `workspace`-crate equivalent — same conclusion Phase 4's dead-code sweep already
reached about `library_search.rs`/`import_graph.rs` staying `lsp_server`-local.

### Layer 2 — `snapshot` (demoted from pipeline to view)

`HostWorkspaceSnapshot` becomes a point-in-time read off Layer 1's current state, not an
independently computed thing. `load_workspace_snapshot` (today: a from-scratch build) and
`update_workspace_snapshot` (today: a separate incremental path) both become thin — apply a
changeset to the engine (a full changeset for the "load" case, a small one for "update"),
then call `.snapshot()`.

This is also where **Part D of the parked `TIER2-PHASE3B-LAZY-SNAPSHOT-DESIGN.md`** was
originally expected to become relevant again: once the graph itself is incrementally
maintained instead of freshly rebuilt on every snapshot, it stops making sense to eagerly
recompute `language_workspace` / `render_snapshot` / `semantic_projection` on every call, so
making those `OnceLock`-per-generation looked like the natural next step. **Superseded —
dropped along with Phase 5, see "Phase 5" below.** The premise (Babel42 stays on the
snapshot pipeline, so laziness there fixes its per-edit cost) stopped holding once the
maintainer decided Babel42 will call `IncrementalWorkspace` directly instead of going through
`snapshot` at all — see Phase 5's write-up for the reasoning. `HostWorkspaceSnapshot`'s
derived fields stay eagerly computed, as they already were before this whole design.

### Layer 3 — `comparison`

Unaffected. It diffs two `snapshot` values; doesn't care how they were produced.

### `Spec42Engine` / `EngineBuilder`

Stays the outer façade tying library-catalog resolution to engine construction. Its public
method signatures (`load_workspace`, `update_snapshot`) don't need to change for `server`
crate (CLI/MCP/HTTP) callers — only what they delegate to underneath changes.

### `lsp_server` after migration

- `ServerState` holds a `workspace::IncrementalWorkspace` instead of its own
  `semantic_graph` + `index` + `parse_cache` + `library_graph_cache` fields.
- `SemanticCoordinator` keeps only its `tokio::sync::watch` notification channel and the
  staged/lock-free-commit wrapper (`rebuild_semantic_graph_staged`'s
  `(graph, symbols, metrics)`-without-holding-a-write-lock shape) — both genuine
  LSP-concurrency concerns `workspace` is guardrailed against owning.
- `services.rs` shrinks substantially: the three full-rebuild functions
  (`rebuild_all_document_links`, `merge_document_graphs_into`,
  `rebuild_semantic_graph_staged`) collapse to calls into
  `IncrementalWorkspace::apply_changes`, keeping only the staged-commit wrapper and
  per-phase metrics (or those metrics move into the engine as built-in instrumentation,
  shrinking `lsp_server` further — not decided here).
- Symbol-table maintenance and `include_in_semantic_graph`-split search indexing stay
  `lsp_server`-local, reading off the engine's current graph after each update.

## Migration phasing (risk-ordered)

Same discipline as Tier 2's earlier steps: each phase independently shippable, verified with
the full relevant test suites before moving on, higher-risk phases behind a flag if needed.

1. **Relocate the two caches into `workspace`, unused. ✅ Done 2026-07-03.** Pure move —
   `parse_cache.rs`/`library_graph_cache.rs` into `workspace` (`crates/workspace/src/`),
   declared `pub mod` in `lib.rs` but not wired into any of `workspace`'s own code yet (kept
   the same low-risk shape as Phase 1's `WorkspaceSession` addition in the earlier
   consolidation effort — additive, unused by anything until a later phase wires it up).
   `lsp_server`'s copies deleted; `crates/lsp_server/src/workspace/mod.rs` now re-exports
   `pub(crate) use workspace::{library_graph_cache, parse_cache};` in place of the old
   `pub(crate) mod` declarations, so every existing `crate::workspace::parse_cache::...` /
   `crate::workspace::library_graph_cache::...` call site in `lsp_server` kept working
   unchanged — zero call-site churn beyond the two-line module re-export.

   Type-portability check from the design held exactly as predicted: `tower_lsp::lsp_types::Url`
   is a direct `pub use url::Url` re-export, so `library_graph_cache.rs` needed only an
   import-path change (`tower_lsp::lsp_types::Url` → `url::Url`, same type). Added `dirs = "5"`
   and `bincode = { version = "2", features = ["serde"] }` to `workspace/Cargo.toml`
   (`sha2`/`serde_json`/`walkdir`/`url`/`tracing`/`tempfile` were already present); removed
   the now-unused `bincode`/`sha2`/`dirs` entries from `lsp_server/Cargo.toml` (nothing else
   in `lsp_server` used them directly). `env!("CARGO_PKG_VERSION")` in the on-disk cache
   header resolves to the same value regardless of which crate it's compiled in (workspace
   uses `version.workspace = true` everywhere), so existing on-disk caches on a developer's
   machine remain valid across the move — not required for correctness (caching is
   best-effort/silently-falls-back-on-miss by design) but a nice bonus.

   One test helper needed adapting: `parse_cache.rs`'s test module used
   `crate::common::util::parse_for_editor(src).root` (an `lsp_server`-local helper); replaced
   with `sysml_v2_parser::parse(src).expect("parse")` directly, matching the pattern used
   elsewhere in `sysml_model`/`workspace` tests.

   **Verification**: `cargo check -p workspace`, `cargo check -p lsp_server`,
   `cargo check --workspace --all-targets` (zero errors, zero unused-import warnings),
   `cargo test -p workspace` (all 9 relocated tests pass, plus the rest of the suite; total
   green), `cargo test -p workspace --test dependency_guardrails` (still passes —
   the caches don't pull in anything the guardrail forbids), `cargo test -p lsp_server`
   (green; lib-test count dropped from 122 to 113, exactly the 9 tests that moved, no other
   change), `cargo test --workspace` (all green), `cargo clippy -p workspace --no-deps
   --all-targets` and `cargo clippy -p lsp_server --no-deps --all-targets` (both clean —
   the only warnings present are two pre-existing, unrelated items in
   `snapshot/facts.rs`/`tests/support/comparison_fixtures.rs`, confirmed via `git status` to
   be untouched by this move).
2. **Build `IncrementalWorkspace` in `workspace`, standalone, not wired anywhere. ✅ Done
   2026-07-03.** New module `crates/workspace/src/incremental.rs`, re-exported at the crate
   root (`workspace::{IncrementalWorkspace, WorkspaceUpdateMetrics}`), not called from
   `Spec42Engine`/`HostWorkspaceSnapshot`/`lsp_server` yet.

   **Shape**: wraps a `SemanticGraph` plus a `HashMap<Url, WorkspaceParsedDocument>` (the
   same `WorkspaceParsedDocument` type `sysml_model`'s pipeline already returns — reused
   rather than inventing a parallel "document index" type). Three operations:
   - `load(&[SysmlDocument]) -> WorkspaceUpdateMetrics` — full rebuild, a thin wrapper around
     `build_and_link_graph_parallel`.
   - `apply_document(&SysmlDocument, cache_dir: Option<&Path>) -> WorkspaceUpdateMetrics` —
     incremental patch, wrapping `patch_graph_for_document`. When `cache_dir` is given, the
     parse is served through this engine's own relocated `parse_cache` (Phase 1) before
     falling back to a fresh parse — the first real use of that relocation.
   - `remove_document(&Url) -> WorkspaceUpdateMetrics` — deletes one document's nodes.

   Both `load` and `apply_document` delegate to `sysml_model`'s existing pipeline functions
   as a single opaque call rather than re-implementing their internal sequencing — the
   deliberate choice to avoid recreating the "two places implement the same sequence, and
   they drift" bug shape Step 5b/5c already found and fixed twice. This is also why two new
   exports needed adding: `build_and_link_graph_parallel` existed in `sysml_model` but wasn't
   re-exported at its crate root (fixed — `sysml_model/src/lib.rs`) or reachable through
   `workspace::semantic` (fixed — `workspace/src/semantic/mod.rs`, alongside
   `WorkspaceParsedDocument`).

   **Metrics — the timing piece requested alongside this phase.** Added
   `WorkspaceUpdateMetrics { document_count, parse_ms, graph_update_ms, total_ms, node_count,
   edge_count }`, returned by every operation. This is coarser than `lsp_server`'s current
   `RebuildAllDocumentLinksMetrics`, which splits the graph-update step into 7 sub-phases
   (remove-nodes, rebuild-graphs, cross-edge-resolution, workspace-relationship-linking,
   pending-relationship-resolution, expression-evaluation, refresh-symbols) — deliberately:
   those finer phases live *inside* `build_and_link_graph_parallel`/`patch_graph_for_document`,
   and breaking them out here would mean either (a) those `sysml_model` functions growing
   their own returned timing breakdown (a real option, not built here — see the updated open
   questions below), or (b) `workspace` re-implementing the phase sequencing itself just to
   get the timing points, which is exactly the duplication this whole design exists to avoid.
   So for now: `parse_ms` is real and separately measured for `apply_document` (this engine
   does its own parse-cache-or-fresh-parse step before calling into `sysml_model`), but
   always `0` for `load` (which parses internally inside `build_and_link_graph_parallel` —
   that time is folded into `graph_update_ms` instead, documented on the field).
   `lsp_server`'s three full-rebuild functions still produce their own 7-phase metrics for
   now; whether/how those get replaced by this coarser shape (accepting less granularity) or
   a future finer `sysml_model`-side breakdown is a Phase 4 decision, not resolved here.

   **A known gap surfaced along the way**: `load`'s doc comment flags that it does *not* yet
   benefit from the engine's own parse cache — `build_and_link_graph_parallel` always parses
   from raw document content, so a full load re-parses everything even if every document is
   already cached. Fixing that needs either a new `sysml_model` entry point that accepts
   pre-parsed documents (skipping its internal parse step), or looping `apply_document` once
   per document for a "load" (losing the parallel merge/link `build_and_link_graph_parallel`
   does). Left open — not needed for Phase 2's standalone scope, but should be resolved
   before Phase 3 wires `load` into `Spec42Engine`'s hot full-load path, since that's exactly
   where the parse cache is supposed to pay off.

   **Equivalence tests** (inline `#[cfg(test)] mod tests`, matching `sysml_model::pipeline`'s
   own convention for this kind of orchestration module):
   - `load_matches_build_and_link_graph_parallel_directly` — confirms `load` produces
     identical node/edge sets to calling `build_and_link_graph_parallel` directly, not just
     "produces a graph" (the Step 5a lesson about set-based comparisons silently tolerating
     duplicate-insertion bugs — this test compares the same underlying data two independent
     ways, not two derived summaries of it).
   - `apply_document_matches_full_reload_after_edit` — the engine-layer version of the parity
     check `workspace/tests/incremental_parity.rs` already does at the `HostWorkspaceSnapshot`
     layer: patch one document via `apply_document`, then confirm the result matches a fresh
     `load` of the post-edit document set.
   - `apply_document_evaluates_expressions` — the same `evaluate_expressions`-regression
     shape Steps 1-4 fixed, re-checked at this new layer.
   - `remove_document_clears_its_nodes`, `apply_document_uses_parse_cache_when_provided` —
     the two operations not otherwise covered by the parity tests above.

   **Verification**: `cargo check -p workspace` (clean, first try), `cargo test -p workspace
   --lib incremental` (5/5 pass, first run), `cargo test -p workspace --test
   dependency_guardrails` (still passes — nothing new here pulls in a forbidden dependency),
   `cargo test -p workspace` (full suite green, 37 lib tests now vs. 32 before Phase 2),
   `cargo test --workspace` (green), `cargo test -p lsp_server` (green, unaffected — nothing
   wired up yet), `cargo clippy -p workspace --no-deps --all-targets` and `cargo clippy -p
   sysml_model --no-deps` (both clean after fixing one `cloned_ref_to_slice_refs` lint in a
   test; the handful of other warnings present are the same pre-existing, unrelated dead-code
   items in `snapshot/facts.rs`/`tests/support/comparison_fixtures.rs` noted in the Phase 1
   write-up).
3. **Migrate `Spec42Engine`/`snapshot` module to build on it. ✅ Done 2026-07-03.** This is
   where the eager and incremental pipelines actually merged into one.

   **Full-build side** (`snapshot/build.rs`'s `build_workspace_snapshot`): the two-line
   `build_semantic_graph_from_documents(&documents).map_err(map_graph_error)?` call replaced
   with `IncrementalWorkspace::new().load(&documents)` plus `.graph()`/`.documents()`. Nearly
   a no-op change in practice — `build_semantic_graph_from_documents` already delegated to
   `build_and_link_graph_parallel` since Step 5b, the same function `load` wraps, and it was
   already `Ok(...)`-wrapping an infallible call, so dropping the now-vestigial `Result`
   changed nothing observable. This is also why `map_graph_error` became genuinely dead code
   (its only call site) — removed, from `error/map.rs` and its re-export in `error.rs`.

   **Incremental side** (`snapshot/update.rs`'s `try_incremental_update`): this is where the
   real merge happened. Replaced the hand-written sequence — deep-clone the previous graph,
   a bespoke `patch_parsed_documents` helper (filter out the changed URI, re-parse, push),
   and a direct `sysml_v2_parser::parse` + `patch_graph_for_document` call — with
   `IncrementalWorkspace::from_parts(previous.semantic_graph_arc(), previous.parsed_documents().to_vec())`
   followed by one `apply_document(changed, None)` call. `from_parts` (new on
   `IncrementalWorkspace`) reconstructs engine state from a previous snapshot without
   deep-copying the graph up front — the `Arc` clone stays cheap until `apply_document`
   actually mutates it, same cost shape as before. `patch_parsed_documents` deleted entirely;
   `apply_document`'s existing miss/hit handling (drop the document from the index on a
   parse failure, replace it on success) matches its exact behavior — verified by parity
   tests, not just read by inspection.

   Deliberately **not** changed as part of this migration: `cache_dir: None` is passed to
   `apply_document` here, meaning `workspace`'s incremental-update path still doesn't use the
   parse cache — matching its pre-migration behavior exactly (it never used one). Wiring the
   cache through `Spec42Engine`'s already-known `cache_dir` is a real, available follow-up,
   deliberately deferred so this phase stayed a pure behavior-preserving refactor rather than
   bundling in a feature change (see open question 4, superseded/narrowed by this note).

   **A determinism fix surfaced along the way**: `IncrementalWorkspace::documents()` was
   returning `HashMap` iteration order (unspecified) from Phase 2; before wiring it into a
   snapshot path whose output order might matter to a consumer, changed it to sort by URI —
   checked first that no existing code or test depended on the old insertion-derived order
   (`HostWorkspaceSnapshot::documents()`, which some tests do index by position, is a
   separate `Vec<SysmlDocument>` field populated from the raw input list, not derived from
   `IncrementalWorkspace` — unaffected either way).

   **New engine-level equivalence test**: `from_parts_then_apply_document_matches_load` —
   confirms `from_parts` + `apply_document` produces the same graph as a fresh `load` of the
   post-edit document set, the same shape `try_incremental_update` now depends on.

   **Verification**: `cargo check -p workspace --all-targets` (clean — only the same
   pre-existing, unrelated dead-code warnings from Phase 1/2), `cargo test -p workspace --lib
   incremental` (6/6, including the new `from_parts` test), `cargo test -p workspace --test
   incremental_parity` (all 4 pass — this is the exact test file that exercises the migrated
   `try_incremental_update` path end-to-end, including the two regression tests from Steps
   1-4 for `evaluate_expressions`), `cargo test -p workspace` (full suite green, 38 lib tests
   now vs. 32 before Phase 2), `cargo test --workspace` (green — including `server`'s 44-test
   suite, the real production consumer of `Spec42Engine`/`HostWorkspaceSnapshot`), `cargo
   clippy -p workspace --no-deps --all-targets` and `cargo clippy -p server --no-deps` (both
   clean, zero warnings in any file touched by this phase).
4. **Migrate `lsp_server`'s full-rebuild functions to delegate to it. ✅ Done 2026-07-03, in
   a narrower shape than "ServerState holds it."**

   **What changed shape from the original plan, and why.** The original wording was
   "`ServerState` to hold and delegate to it" — literally storing `IncrementalWorkspace` as a
   `ServerState` field. Investigating turned up two reasons not to:
   - `ServerState.semantic_graph: SemanticGraph` is read directly at 40+ call sites across 10
     files outside `services.rs`/`state.rs` (hover, completion, diagnostics, views, symbol
     search, etc.) — all of them just want a `&SemanticGraph`, none care how it was computed.
     Changing the field's type would force a mechanical rewrite of all 40+ call sites for
     zero behavioral benefit (`SemanticGraph`'s `Arc` backing already makes
     `IncrementalWorkspace::graph()` a cheap clone, so there's no perf reason to route reads
     through the engine either). `ServerState.semantic_graph` stays exactly as it was —
     unchanged type, unchanged call sites.
   - `IncrementalWorkspace` doesn't track `include_in_semantic_graph` (files indexed for
     `sysml/librarySearch` only) — deliberately, per the design's own "what does not move"
     section. `lsp_server`'s `IndexEntry`/`index` (the richer, authoritative document
     registry) stays exactly as it was too.

   So "delegate to it" became: `rebuild_all_document_links` and `rebuild_semantic_graph_staged`
   construct a **scratch `IncrementalWorkspace`** each call (`IncrementalWorkspace::new()`),
   feed it the currently-relevant documents, pull `.graph()` back out, and discard the
   engine instance — `ServerState` still just holds a plain `SemanticGraph` field, written
   via a single assignment (`state.semantic_graph = graph;`), same shape as before. This is
   a real, meaningful reduction in duplicated *logic* (the actual goal) without an
   unjustified reduction in duplicated *state* (which wasn't duplicated to begin with — the
   40+ read call sites were already sharing one field).

   **A real architectural mismatch this surfaced, and how it was resolved.** Comparing the
   two `lsp_server` functions against `link_parsed_documents_parallel` line by line found:
   - `rebuild_all_document_links` merges *every* included document with a plain
     `SemanticGraph::merge` — it has never distinguished "workspace" from "library" content
     the way `sysml_model`'s pipeline does (`IndexEntry` has no `SysmlDocumentSourceKind`
     field, only `include_in_semantic_graph: bool`). Tagging every document `Workspace` when
     calling `IncrementalWorkspace::load_parsed` reproduces this exactly — `link_parsed_documents_parallel`'s
     partition only special-cases `Library`, so an all-`Workspace` input takes the plain
     `.merge()` path for everything, matching the prior behavior with zero semantic change.
     Documented inline at the call site, not just here, since it's easy to miss.
   - `rebuild_semantic_graph_staged` *does* have a real workspace/library distinction
     (`workspace_uris`/`library_uris`, feeding `merge`/`merge_skip_existing_qualified_names`
     respectively) **and** a `base_graph` reuse path (library-graph-cache hit — skip
     rebuilding library nodes that are already merged into a cached graph). Neither was
     supported by `IncrementalWorkspace`/`link_parsed_documents_parallel` as they stood after
     Phase 2. Rather than drop the base-graph optimization (a real, meaningful perf feature,
     not an implementation detail) or duplicate the merge/link sequence a third time to
     preserve it, extended `sysml_model` itself: `link_parsed_documents_parallel_from(base_graph, documents)`
     — the same function, generalized to seed `graph`/`uris` from an existing `SemanticGraph`
     (via its `all_uris()`) instead of always starting empty. `link_parsed_documents_parallel`
     becomes a thin `link_parsed_documents_parallel_from(SemanticGraph::new(), documents)`
     wrapper. `IncrementalWorkspace` gained the matching `load_parsed_from`. Both new
     equivalence-tested (sysml_model: `link_parsed_documents_parallel_from_matches_building_everything_together`;
     workspace: `load_parsed_from_matches_load_when_base_graph_holds_one_document`) before
     being wired into `lsp_server`.

   **Metrics — open question 2, resolved.** `RebuildAllDocumentLinksMetrics`'s 7-phase
   breakdown (remove-nodes, rebuild-graphs, cross-edge-resolution,
   workspace-relationship-linking, pending-relationship-resolution, expression-evaluation,
   refresh-symbols) can't be preserved once the graph computation is one delegated call into
   `IncrementalWorkspace` — those phases live inside `link_parsed_documents_parallel`, and
   re-instrumenting them from the `lsp_server` side would mean re-implementing that function's
   internal sequencing a third time just to get timing points, exactly the duplication this
   whole design exists to eliminate. Decision made explicitly rather than deferred again: the
   struct's field *names* stay (avoiding churn in `lsp_runtime/documents.rs`'s structured log
   fields), but the five now-unmeasurable phases report `0`, and the combined delegated-call
   time is reported through the pre-existing `cross_document_edges_ms` field (which already
   played the role of "whole graph computation umbrella timer" before this change — the
   closest existing field to keep the number meaningful for anyone reading the logs).
   `refresh_symbols_ms` and `total_ms` are still measured for real, since symbol-table refresh
   and the outer wrapping stayed `lsp_server`-local.

   **`merge_document_graphs_into` deleted** — its logic is now `link_parsed_documents_parallel_from`'s.

   **Verification** (this is the production LSP hot path — full rigor, no shortcuts):
   `cargo check -p lsp_server --all-targets` (clean, zero warnings), `cargo test -p lsp_server`
   (all green: 113 lib tests including the 5 tests specifically targeting
   `rebuild_all_document_links` — library-relinking, public-re-export-chain-relinking,
   evaluated-attribute/referenced-attribute/unit-conversion recomputation — plus the 148-test
   integration suite exercising real fixtures — drone, webshop, powersystems, kitchen-timer —
   through full startup/scan/hover/diagnostics/rename flows that go through both migrated
   functions), `cargo test --workspace` (green throughout — `sysml_model` 201 tests,
   `workspace` 41 lib tests, up from 199/38 with the new equivalence tests), `cargo clippy -p
   lsp_server --no-deps --all-targets`, `cargo clippy -p sysml_model --no-deps`, `cargo
   clippy -p workspace --no-deps --all-targets` (all clean — one `type_complexity` lint from
   this phase's own code fixed via a `SourceTaggedDocument` type alias in `sysml_model`).

   **A residual, honestly-flagged risk not fully closed by tests**: `rebuild_all_document_links`'s
   prior implementation removed nodes only for currently-included URIs before rebuilding them
   in place (`std::mem::take` + per-URI `remove_nodes_for_uri`), rather than unconditionally
   replacing the whole graph. In every reachable state this repo's invariants allow, that's
   equivalent to a full rebuild (the graph should only ever contain nodes for URIs currently
   in `index`) — but that equivalence rests on an invariant, not a type-level guarantee, and
   `IncrementalWorkspace::load_parsed`'s unconditional `self.graph = graph` would silently
   drop anything violating it. The full test suite (including the two functions' own
   dedicated tests) found no such case, which is meaningful evidence, not proof.
5. **Dropped 2026-07-03 — decided against, not deferred.** Originally: make the derived
   snapshot fields (`language_workspace`/`render_snapshot`/`semantic_projection`) lazy per
   generation via `OnceLock` (Part D of the parked lazy-snapshot design), on the premise that
   this is what fixes Babel42's per-edit recompute cost.

   **That premise stopped holding.** The maintainer's actual plan is for Babel42 to call
   `IncrementalWorkspace` directly (mirroring how `lsp_server` already uses it) instead of
   going through `Spec42Engine::update_snapshot()`/`HostWorkspaceSnapshot` at all — keeping
   `snapshot` for CLI/MCP only. Once Babel42 stops calling the snapshot pipeline on every
   edit, laziness *inside that pipeline* stops being relevant to Babel42's cost at all — it
   isn't optimizing the code path Babel42 will actually run, it's optimizing a code path
   Babel42 will simply no longer be on.

   What's left of the original motivation, assessed and rejected as not worth pursuing
   separately:
   - **CLI**: one-shot (load, use, exit) — nothing else reads a snapshot's unused accessor
     methods after the process exits, so eager computation of a field the caller never reads
     costs real work but no *correctness* risk, and the win from skipping it is bounded by
     "one process lifetime." Not worth the `OnceLock`-per-field complexity for that.
   - **MCP**: a longer-lived process could in principle benefit (a tool call that only wants
     diagnostics shouldn't pay for `render_snapshot`), but this was never the case that
     motivated the design — it was reasoned about from Babel42's edit-per-keystroke cost,
     which no longer applies here. No usage data suggests this is actually a bottleneck for
     MCP today. **Decision: current eager behavior is fine as-is for both.**

   The real remaining need for Babel42 — cheap, on-demand derivation of a specific view from
   whatever graph state it currently holds — is already satisfied by the pattern
   `lsp_server`'s own `views/`/`analysis/` modules already use (call straight into
   `sysml_model`/`language_service` against a `&SemanticGraph`, no snapshot involved). That's
   not a gap this design needs to close; it's a gap in what's *callable outside `lsp_server`*
   — see the follow-up discussion on generalizing `workspace/library_search.rs` and the
   feature-inspector-style query orchestration, which is a separate, smaller piece of work
   from what Phase 5 was originally scoped as.

## Risks

- **This eventually touches the production LSP hot path** (Phase 4 of the migration).
  Regressions are directly user-visible: stale diagnostics, hangs, or lost edits under rapid
  typing. `lsp_server`'s existing incremental logic has presumably already found and fixed
  subtle cancellation-race bugs; the migration must not regress behavior it already gets
  right — same risk this doc's predecessor (`TIER2-LSP-WORKSPACE-CONSOLIDATION.md`) flagged
  for the original, larger Phase 3b scope that got rescoped down to avoid exactly this risk.
  Phasing 1-3 (workspace-only, unused by `lsp_server` until Phase 4) is deliberately
  structured so most of the design and equivalence-testing risk is retired before the
  hot-path migration begins.
- **Concurrency semantics differ per embedder.** `lsp_server` needs "read the previous state
  while a background rebuild runs" (its staged/lock-free-commit shape); CLI/MCP/HTTP
  typically don't (single request, blocking is fine); Babel42's backend concurrency model is
  unknown from this repo and should be checked before assuming the same wrapper fits it.
  Keeping this wrapper embedder-local (per "what does not move" above) is meant to sidestep
  designing one concurrency model that has to fit all three, but it's worth confirming this
  assumption holds once Babel42's actual session code is reviewed.
- **No realistic-fixture perf baseline exists yet** for the current incremental vs. full-build
  paths (flagged as an open gap in Step 5b) — this migration should establish one before/after,
  not just assert improvement.

## Open questions

1. Does `WorkspaceSession`'s existing lifecycle state machine become `IncrementalWorkspace`'s
   internal bookkeeping, or stay a separately composed type? **Resolved by how Phase 4
   actually shipped, not by picking one of the two options**: `IncrementalWorkspace` never
   ended up holding any lifecycle state, because it never ended up living inside `ServerState`
   at all — Phase 4 uses it as a scratch, per-call computation helper, constructed fresh each
   time. `SemanticCoordinator`'s Cold/Indexing/Ready/Reindexing lifecycle stays exactly where
   it already was (`WorkspaceSession`, delegated to since Phase 2 of the original
   consolidation), completely untouched by this design. The question only becomes live again
   if a future phase gives `IncrementalWorkspace` a persistent home.
2. Should the per-phase timing metrics `RebuildAllDocumentLinksMetrics` currently produces
   move into the engine as built-in instrumentation, or stay `lsp_server`-local wrapping?
   **Resolved by Phase 4**: neither, cleanly — the 7-phase breakdown is gone, collapsed into
   the pre-existing `cross_document_edges_ms` field as one combined number. Field names
   (and therefore `lsp_runtime/documents.rs`'s structured log fields) kept as-is; five fields
   now always report `0`. See Phase 4's write-up for the reasoning.
3. Does Babel42's `EditorSession` need its own concurrency wrapper analogous to
   `lsp_server`'s staged-commit shape, or is blocking-per-edit acceptable there? **Partially
   resolved 2026-07-06**: checked against Babel42's actual code
   (`babel42-v2/backend/crates/babel42-spec42/src/session.rs`,
   `babel42-v2/backend/crates/babel42-app/src/editor.rs`). Babel42 does need one — every
   hover/completion request calls `EditorSession::update_document` (which internally re-parses
   and patches the graph) while holding a per-session `std::sync::Mutex<CachedEditorSession>`
   for the whole call, so concurrent reads against the *same* editing session already
   serialize today, independent of any slow rebuild — the same architectural shape as the
   `lsp_server` render-path bug this design's sibling investigation found and fixed (see
   `crates/workspace_session`, added 2026-07-06: a Tokio-actor concurrency wrapper — lock-free
   snapshot reads plus generation-token-superseded background rebuilds — built standalone and
   not yet wired into either `lsp_server` or Babel42). Whether Babel42 adopts
   `workspace_session`'s `SessionActor<M>` as-is, or some lighter variant, is still open; see
   item 5 below for a related, more urgent gap found during the same investigation.
4. Should `IncrementalWorkspace::load` gain a variant that accepts already-parsed documents
   (to actually benefit from this engine's own parse cache on a full load)? **Resolved,
   fully**: `load_parsed`/`load_parsed_from` (pre-parsed input, no cache lookup) and
   `load_with_cache` (parses through the cache in parallel) both added in Phase 4, backed by
   a new `sysml_model` primitive (`link_parsed_documents_parallel`/`_from`) rather than a
   `workspace`-side reimplementation. Not yet wired into `Spec42Engine`'s full-load path
   (Phase 3 used plain `load()`, since `server` crate's document providers hand back raw
   `SysmlDocument`s, not pre-parsed ones) — `lsp_server` is the first real consumer. Wiring
   `Spec42Engine` up to a warm cache on CLI/MCP/HTTP restarts is a small, available follow-up,
   not done here.
5. **New, found 2026-07-06 — Babel42's `EditorSession` never re-resolves cross-document
   references after an edit.** `EditorSession::update_document`
   (`babel42-v2/backend/crates/babel42-spec42/src/session.rs`) calls only
   `self.workspace.apply_document(...)` — the single-document incremental patch. There is no
   Babel42 equivalent of `lsp_server`'s async full relink (`rebuild_semantic_graph_staged`,
   the cross-document-edge-resolution pass — expensive, ~2.3s in a captured `lsp_server` log
   on a real workspace, which is precisely why `lsp_server` runs it asynchronously rather than
   inline). Concretely: if editing file A changes something file B's cross-file reference
   depends on, Babel42's hover/completion/diagnostics for B will not reflect it until B itself
   is next edited (which forces its own `apply_document` patch) — there is currently no path
   that ever re-links B against A's change otherwise.
   **This should be fixed** — not investigated further here, flagged for a future phase to
   scope. Two independent decisions needed: (a) does Babel42's actual feature set need
   full cross-file accuracy at all (it currently exposes no diagram/visualization endpoint,
   unlike `lsp_server` — the answer may be "yes, for correctness of hover/goto-definition
   across files" even without a diagram feature); (b) if yes, should the relink run inline
   (blocking that one request — Babel42's per-request model may tolerate this, unlike
   `lsp_server`'s shared long-lived state) or asynchronously via the same
   `workspace_session::SessionActor`/`report_job_result` pattern `lsp_server` will eventually
   adopt (see item 3) — in which case this gap and item 3's concurrency-wrapper question
   should likely be resolved together, not separately.

## Effort estimate

Phase 1: small, self-contained (~1 session). Phase 2: medium — new type plus equivalence
tests (~1-2 sessions). Phase 3: medium-large — merges two pipelines, needs full parity
verification across `workspace`/`server` (~1-2 sessions). Phase 4: large, the highest-risk
phase, needs async/cancellation test coverage and staged rollout (~2-3 sessions, possibly
its own tracked initiative given the hot-path risk). Phase 5: separate follow-on, not
estimated here.

Total: treat Phases 1-3 as one initiative (workspace-internal, low external risk) and Phase 4
as a second, separately-scoped initiative given its risk profile — matching how Tier 2 itself
was split into Phases 1-3a (low risk, landed quickly) versus 3b (needed its own design doc
and got rescoped once already for size/risk reasons).
