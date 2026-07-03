# Tier 2: Unified Incremental Engine — Move `lsp_server`'s Incremental Machinery Into `workspace`, Layer Snapshot On Top

**Status:** Phase 1 landed 2026-07-03 — see "Phase 1 status" below. Phases 2-5 not started.
**Date:** 2026-07-03
**Related:** `docs/engineering/TIER2-LSP-WORKSPACE-CONSOLIDATION.md` (Phases 1-3a, Phase 3b
Steps 1-4, Step 5a-5c, Phase 4 all landed — this doc addresses the split those phases
deliberately left alone), `docs/engineering/TIER2-PHASE3B-LAZY-SNAPSHOT-DESIGN.md` (parked;
Part D of this doc explains how it becomes relevant again once this ships),
`docs/architecture-audit.md` Tier 2.

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

This is also where **Part D of the parked `TIER2-PHASE3B-LAZY-SNAPSHOT-DESIGN.md` becomes
relevant again**, rather than a separate initiative: once the graph itself is incrementally
maintained instead of freshly rebuilt on every snapshot, it stops making sense to eagerly
recompute `language_workspace` / `render_snapshot` / `semantic_projection` on every call.
Making those `OnceLock`-per-generation (computed only when a caller actually asks) is the
natural next step here, and is what actually fixes Babel42's per-edit full-recompute cost —
as a side effect of this unification, not a dedicated project. Not designing that piece in
detail here; flagged as a likely Step 5 of the migration plan below.

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
2. **Build `IncrementalWorkspace` in `workspace`, standalone, not wired anywhere.**
   Equivalence-test it against both `build_and_link_graph_parallel` (full-load case) and
   `lsp_server`'s current incremental behavior (patch case) — same rigor as Step 5a's
   sequential-vs-parallel equivalence tests.
3. **Migrate `Spec42Engine`/`snapshot` module to build on it.** This is where the eager and
   incremental pipelines actually merge into one. Full `workspace` + `server` crate test
   suites must pass unchanged (same parity bar as Steps 1-4).
4. **Migrate `lsp_server`'s `ServerState` to hold and delegate to it**, keeping only the
   `tokio` wrapper and protocol-specific state local. Highest-risk phase — this is the
   production LSP hot path, every file edit in every connected editor goes through it.
   Recommend staging behind a flag (`experimental_incremental_updates` already exists and
   could be reused, or a new one) with a rollback lever, full 270+-test `lsp_server` suite,
   and manual editor-integration verification before it's load-bearing.
5. **(Follow-on, separate from this migration's core risk)** Make the derived snapshot views
   lazy per generation (Part D of the lazy-snapshot design, reactivated per above) — the
   piece that actually resolves Babel42's per-edit recompute cost.

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
   internal bookkeeping, or stay a separately composed type? Affects Phase 2's design but not
   its risk profile.
2. Should the per-phase timing metrics `RebuildAllDocumentLinksMetrics` currently produces
   move into the engine as built-in instrumentation (useful to `workspace`'s other
   consumers too), or stay `lsp_server`-local wrapping? Same open question Step 5c's design
   left unresolved for the smaller scope; revisit here for the full engine.
3. Does Babel42's `EditorSession` need its own concurrency wrapper analogous to
   `lsp_server`'s staged-commit shape, or is blocking-per-edit acceptable there? Needs
   checking against Babel42's actual code before Phase 4 assumes `lsp_server`'s shape is the
   only one needed.
4. Should `IncrementalWorkspace::apply_changes` reuse `patch_graph_for_document` per document
   for small changesets and switch to `build_and_link_graph_parallel`'s batch strategy above
   some size threshold (mirroring what `lsp_server`'s `rebuild_semantic_graph_staged` already
   does informally), or always go through one uniform path? Affects Phase 2's implementation,
   not the overall design.

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
