# Tier 2: Consolidating `workspace` and `lsp_server`'s Incremental Update Machinery

**Status:** Phases 1, 2, 3a landed 2026-07-02. Phase 3b was redesigned twice before landing:
first around a Babel42 performance finding (design parked — priority redirected to
spec42-internal consolidation per maintainer direction 2026-07-02), then around the
narrower target that shipped: a shared single-document graph-patch primitive between
`workspace` and `lsp_server`. **Phase 3b Steps 1-4 landed 2026-07-02** — see
`docs/engineering/TIER2-PHASE3B-SHARED-GRAPH-PATCH-DESIGN.md`. The shared primitive lives
in `sysml_model` (`patch_graph_for_document`/`finalize_and_evaluate`), re-exported through
`workspace` then `lsp_server` following the existing "protocol-neutral logic lives in
`workspace`" convention; both `workspace::try_incremental_update` and
`lsp_server::update_semantic_graph_for_uri` now delegate to it instead of maintaining their
own sequences. Along the way this fixed a real correctness bug: `workspace` crate's graph
pipeline never called `evaluate_expressions`, so every snapshot built via CLI, MCP, or
Babel42 had unevaluated attribute values — fixed as a byproduct of the consolidation,
confirmed by new regression tests. **Step 5 (the full-rebuild-path duplication, Steps
5a-5c) and Phase 4 (delete now-dead code) are both done as of 2026-07-03** — see the "Step
5 status" and "Phase 4" sections below. Phase 4 turned out to be a small cleanup (two
unused-import removals), not the large `services.rs` shrink originally envisioned, since
the wholesale duplicated-logic merge that would have produced dead code was rescoped away.
**Date:** 2026-07-02
**Related:** `docs/architecture-audit.md` (P1-2, P2-3, P2-4, P2-9), Technical Debt Reduction Plan Tier 2.

## Phase 3 status: split into 3a (done) and 3b (rescoped)

Reading `lsp_server/src/workspace/services.rs` in full before touching it surfaced that
"fold the ~1200 lines into `workspace`'s incremental path" is a bigger merge than
originally estimated: `lsp_server`'s indexing model has concepts `workspace` crate doesn't
— `IndexEntry.include_in_semantic_graph` (files indexed for `sysml/librarySearch` only, not
merged into the graph), a live symbol table rebuilt alongside the graph, and
`rebuild_semantic_graph_staged` returning `(graph, symbols, metrics)` specifically so
`ServerState` can be updated without holding a write lock during the heavy work. Porting
this into `workspace::update_workspace_snapshot` means introducing LSP-specific indexing
concepts into a crate whose whole design point is staying protocol-neutral (the same
guardrail that blocked `tokio` in Phase 1). Confirmed with the maintainer to split:

### Phase 3a — replace the 5 manual `std::thread::spawn` sites with `rayon`. ✅ Done 2026-07-02.

All 5 sites in `services.rs` followed the same pattern: round-robin bucket by
`available_parallelism()`, spawn N threads, `.join().unwrap_or_default()`. Every one of
those `.join().unwrap_or_default()` calls silently swallowed a worker-thread panic and
proceeded with a partial/empty result — a real (if narrow) correctness bug, fixed as a
side effect of this migration since rayon propagates panics through `.collect()`/`.map()`
instead of eating them.

Converted:
- `parse_scanned_entries` — `entries.into_par_iter().map(...).collect()`. Dropped the
  `ordinal`-based manual bucketing and post-hoc sort entirely: `into_par_iter()` on a `Vec`
  is an indexed parallel iterator, so `collect()` already preserves input order regardless
  of which worker finishes first. Also removed the now-fully-unused `ordinal` field from
  `ParsedScanEntry` (it was flagged by the compiler as dead code once the sort was gone —
  `#[derive(Debug)]` does *not* suppress "field never read" lints, contrary to what I
  assumed initially).
- `rebuild_all_document_links`'s graph-build step — `parsed_docs.par_iter().map(build_graph_from_doc).collect()`.
- `rebuild_all_document_links`'s cross-document edge resolution — `uris.par_iter().flat_map(resolve_cross_document_edges_for_uri).collect()`,
  reading `&graph` directly instead of `graph.clone()`-per-worker. The original clone
  wasn't there because `SemanticGraph` needed it for thread-safety (it's `Arc`-backed —
  `SemanticGraph(Arc<SemanticGraphData>)` — so `.clone()` is O(1) and the type is already
  `Sync`); it was there because `std::thread::spawn` requires `'static` closures, so an
  owned clone was the easiest way to get data into the thread. Rayon's scoped parallelism
  has no such requirement, so this version borrows the graph directly — one fewer clone per
  call than the original, not just equivalent.
- `merge_document_graphs_into` — same pattern as the graph-build step above.
- `rebuild_semantic_graph_staged`'s cross-document edge resolution — same pattern as
  `rebuild_all_document_links`'s equivalent step.

`services.rs` shrank 1345 → 1225 lines. `cargo test -p lsp_server` (122+5+148+3 = 278
tests) and full `cargo test --workspace` (114 test binaries, all `ok`) pass unchanged —
same pass counts as before the migration. `cargo clippy -p lsp_server` clean.

### Phase 3b — fold the duplicated graph-update logic into `workspace`.

**First pass (parked):** investigating this surfaced that a third live-editing consumer
exists — Babel42 (`C:\Git\babel42-v2`, `backend/crates/babel42-spec42/src/session.rs`) —
built directly on `workspace` crate (bypassing `lsp_server`/LSP entirely), and that
`update_snapshot()`'s incremental path only patches the semantic graph incrementally;
`language_workspace`/`render_snapshot`/`semantic_projection` are unconditionally rebuilt
from scratch on every edit Babel42 sends. Design for fixing that:
`docs/engineering/TIER2-PHASE3B-LAZY-SNAPSHOT-DESIGN.md` — **parked, not implemented.**
Direction changed 2026-07-02: priority is consolidating spec42's own codebase first;
Babel42 is explicitly left as-is for now. The doc remains the starting point if/when
Babel42 work resumes.

**Second pass (Steps 1-4 done, Step 5 not started):** with Babel42 out of scope, the actual
duplication worth fixing turned out narrower — one specific computation (patch the semantic
graph for a single changed document) written twice, in
`workspace::snapshot::update.rs::try_incremental_update` and
`lsp_server::workspace::services.rs::update_semantic_graph_for_uri`, not the whole
bundle/laziness question. Comparing the two surfaced a real, unrelated correctness bug:
**`workspace` crate's graph pipeline (`finalize_workspace_graph`) never called
`evaluate_expressions`, in either the full-build or incremental path** — confirmed by
grepping `workspace` crate and Babel42's entire backend for the call (zero hits outside
`lsp_server`) and `workspace`'s own test suite for any assertion on `evaluatedValue`/
`evaluationStatus` (also zero). Practical effect: every snapshot built through
`workspace::Spec42Engine` — CLI, MCP, and Babel42 — never got computed attribute values
(`mass = 1 + 2` stayed `"1 + 2"`, never became `3`).

Design and full implementation log: **`docs/engineering/TIER2-PHASE3B-SHARED-GRAPH-PATCH-DESIGN.md`**
— `patch_graph_for_document`/`finalize_and_evaluate` added to `sysml_model` crate (Step 1),
`workspace`'s full-build and incremental paths switched to them (Steps 2-3, fixing the bug),
`lsp_server::update_semantic_graph_for_uri` collapsed to a one-line delegation (Step 4).

**Step 5 (the larger, separate full-rebuild-path duplication) has its own design**:
`docs/engineering/TIER2-PHASE3B-STEP5-FULL-REBUILD-DESIGN.md`. Turns out to be more than
duplication: `sysml_model`'s own doc comments (on `link_workspace_derivations`) already
document the parallel-linking technique `lsp_server` uses for full rebuilds, but
`workspace`'s own full-build path (`build_and_link_graph`) never adopted it —
single-threaded parsing/building, slower sequential linking. Same shape as the Step 2 bug: a
real `workspace`-crate perf gap that `lsp_server` already solved, documented, never ported
back. **Step 5a landed 2026-07-02**: `build_and_link_graph_parallel` added to `sysml_model`
(unused by production call sites yet), with an equivalence test proving it produces
identical nodes and edges to the old sequential path on a fixture exercising cross-document
typing and subject edges — passed on the first run, resolving the tension between two
contradictory doc comments in `sysml_model` (one self-described as "legacy") in favor of the
newer one. The fixture now also covers derivation-connection edges (closed 2026-07-02, see
the Step 5 doc). **Step 5b landed 2026-07-03**: `workspace`'s full-build path
(`build_semantic_graph_from_documents`) now calls `build_and_link_graph_parallel`. The swap
surfaced a real duplicate-edge bug (same-document typing/specializes edges were being added
twice — once during per-document graph build, once during parallel cross-document
resolution, because the resolved-edge insertion used a raw `add_edge` instead of the
deduping `add_semantic_edge_once`); fixed, full `sysml_model` and `workspace` suites pass.
No before/after timing comparison on a realistic fixture has been captured yet (see the Step
5 doc for why). **Step 5c landed 2026-07-03**: full delegation of `lsp_server`'s full-rebuild
functions to `build_and_link_graph_parallel` didn't fit — they operate on already-parsed,
cached `RootNamespace`s, so routing through a function that re-parses from raw content would
be a regression, not a refactor. Scoped down to fixing the same duplicate-edge bug found in
5b, plus a second bug this step turned up: neither `rebuild_all_document_links` nor
`rebuild_semantic_graph_staged` ever called `prepare_analysis_evaluation_context` (zero call
sites anywhere in `lsp_server`, confirmed by grep) — same shape as the earlier
`evaluate_expressions` gap, meaning analysis/verification expressions relying on inherited
typed case context could evaluate against stale context right after a full workspace load,
until the next incremental edit self-healed it. Both fixed; `lsp_server`'s full test suite
and the workspace-wide suite pass clean.

**Phase 4 — ✅ Done 2026-07-03, but not the outcome originally scoped.** The original plan
(top of this doc, written before Phase 3b was rescoped) expected `services.rs` to shrink from
~2988 lines to "primarily LSP-protocol glue" once the ~1200-line duplicated-logic merge into
`workspace` crate landed. That merge never happened — Phase 3b was rescoped twice (first
away from Babel42 lazy-snapshot work, then down to the much narrower shared
graph-patch-primitive design) and shipped as small, in-place delegations/bugfixes rather than
wholesale removal of duplicated functions. So there was no large block of orphaned code left
behind to delete. Checked directly: `cargo check -p lsp_server --all-targets` showed exactly
two `dead_code`-adjacent warnings, both unused `SemanticLifecycle` imports (one in
`validation/built_workspace.rs`, one in a `#[cfg(test)]` module in
`views/workspace_artifacts.rs`) — leftovers from the June 2026 `SemanticCoordinator`
introduction (commit `82e1fcd`), predating this Tier 2 initiative entirely, not "resulting"
from Steps 1-5c. Removed both. No other dead code in `lsp_server`'s `workspace/` module: the
compiler finds nothing else unused, because everything Steps 1-4 and 5c touched was either
collapsed to a delegating one-liner in place (`update_semantic_graph_for_uri`) or fixed
in place (the two Step 5c bugs) — neither leaves an orphaned function behind.
`workspace/services.rs` is 1221 lines today (was ~1345 before Phase 3a); the rest of
`workspace/` is 2850 lines total. Verified: `cargo check --workspace --all-targets` (zero
warnings in `lsp_server`; the only remaining workspace-wide warnings are pre-existing,
unrelated dead test-helper functions in `language_service`/`workspace`/`server` test
fixtures — outside this initiative's scope), `cargo test -p lsp_server` and `cargo test
--workspace` (all green), `cargo clippy -p lsp_server --no-deps --all-targets` (clean).

Tier 2 Phase 3b is now fully closed out.

## Phase 2 status (done, 2026-07-02)

`lsp_server/src/workspace/coordinator.rs`'s `SemanticCoordinator` now delegates its
generation/version/transition bookkeeping to an internal `workspace::WorkspaceSession`
instead of tracking `lifecycle`/`version`/`relink_generation` fields itself. Its public API
is **unchanged** — every method signature, every one of the 8 call-site files
(`lsp_runtime/{custom,documents,mod}.rs`, `validation/built_workspace.rs`,
`views/workspace_artifacts.rs`, `workspace/{mod,state}.rs`) needed zero edits.

Two types stayed local to `lsp_server` rather than being replaced by `workspace`'s
equivalents, deliberately:
- **`SemanticLifecycle`** (in `workspace/state.rs`) stays a separate enum from
  `workspace::SessionLifecycle`, translated at the `coordinator.rs` boundary
  (`to_semantic_lifecycle`). It carries LSP-specific inherent methods
  (`supports_semantic_queries`, `suppresses_transient_semantic_diagnostics`) that can't be
  added to a foreign type from another crate — collapsing the two into one type would need
  moving those methods to free functions or an extension trait for a purely cosmetic gain.
- **The `tokio::sync::watch` channel** stays in `coordinator.rs`, since `workspace` crate is
  guarded against depending on `tokio` at all (see Phase 1 below). `SemanticCoordinator` now
  just calls `self.publish()` (send over its own local channel) after every delegated
  transition.

`RelinkToken` in `lsp_server` is now a thin wrapper around `workspace::RelinkToken`
(`generation()` passthrough); the dead `#[cfg(test)] snapshot_version()` accessor (flagged
by clippy as unused before this change) was dropped rather than carried forward.

`coordinator.rs` shrank 191 → 176 lines, but the more important change is that the actual
transition-validity logic (generation counters, `debug_assert`s, the `commit_relink` staleness
check) now has exactly one implementation — in `workspace::WorkspaceSession`, covered by the
12 tests added in Phase 1 — instead of being duplicated logic with its own (previously
nonexistent) test coverage. Full `cargo test --workspace` passes, including
`lsp_server/tests/debt_guardrails.rs` (which enforces `lsp_server`'s own semantic-layer
purity invariants) and `workspace/tests/dependency_guardrails.rs`.

## Phase 1 status (done, 2026-07-02)

`WorkspaceSession`/`SessionLifecycle`/`RelinkToken` added to `crates/workspace/src/session.rs`,
generalizing the token/generation state machine from `lsp_server`'s `SemanticCoordinator`.
Not yet used by any caller — zero behavior change to `Spec42Engine`'s existing API.

**Deviation from the original design sketch below:** the sketch proposed `WorkspaceSession`
owning a `tokio::sync::watch`-based subscription channel for lock-free waiting. That was
rejected: `crates/workspace/tests/dependency_guardrails.rs` enforces that `workspace` never
depends on `tokio` (or `clap`/`axum`/`rmcp`/`tower-lsp`/`lsp_server`) — a deliberate
invariant keeping the crate protocol/runtime-neutral so it stays embeddable by consumers
that don't want a specific async runtime forced on them, and so its public API surface
doesn't get version-coupled to a specific tokio release. `WorkspaceSession` is therefore a
plain synchronous state machine with no `subscribe()`. Phase 2 has `lsp_server` (which
already owns `tokio`) layer its own `watch` channel around `WorkspaceSession::lifecycle()`,
exactly as `SemanticCoordinator` does today — no capability is lost, it's just pushed to the
layer that actually needs it.

Also fixed one edge case the original `SemanticCoordinator` pattern didn't defend against:
`commit_relink` now checks `lifecycle == Reindexing` (not just token generation/version)
before committing, since `reset()` intentionally doesn't bump the generation counter and a
token issued before a `reset()` could otherwise still look "current." See `session.rs`'s
`commit_relink` doc comment.

12 unit tests in `crates/workspace/src/session.rs` cover the full transition table. Full
workspace `cargo check`/`cargo test` pass, including `dependency_guardrails`.

## Problem

`server` (the shipped `spec42` binary) depends on two crates that both build and hold
semantic-graph workspace state:

- **`workspace`** (`crates/workspace`) — `Spec42Engine::load_workspace()` /
  `update_snapshot()`. One-shot, synchronous, immutable-snapshot API: hand in documents,
  get back `Arc<HostWorkspaceSnapshot>`. Used for CLI `check`, MCP tools, and batch/headless
  validation (`crates/server/src/host_snapshot.rs`).
- **`lsp_server`** (`crates/lsp_server`) — the live LSP protocol runtime
  (`lsp_server::run_lsp()`, called from `crates/server/src/lib.rs:227`). Maintains its own
  `ServerState` (`crates/lsp_server/src/workspace/state.rs`) with an async, cancellable,
  incrementally-updated semantic graph, driven by `SemanticCoordinator`
  (`crates/lsp_server/src/workspace/coordinator.rs`) and
  `crates/lsp_server/src/workspace/services.rs` (1345 lines).

An investigation (2026-07-02) found this is **not** copy-paste duplication of everything —
`lsp_server` already re-exports `workspace::semantic::*` for graph-building primitives and
`sysml_model` for diagnostics, and its `validation/` module consumes a prebuilt graph rather
than building its own. The actual duplication is the **orchestration layer**: roughly 1200
lines in `services.rs` reimplement incremental graph updates, parse caching
(`parse_cache.rs`, 243 lines), and library-graph caching (`library_graph_cache.rs`, 369
lines) that `workspace::snapshot::{build.rs,update.rs}` also do, in a different shape.

## Root cause

`workspace`'s API cannot express what `lsp_server` needs:

- `update_snapshot()` is synchronous and returns a *complete* new snapshot — no way to start
  a relink, keep serving the old graph to concurrent readers, and cancel the relink if a
  newer edit arrives before it finishes.
- `lsp_server` needs exactly that: parse on every `didChange`, relink asynchronously, drop
  stale relinks superseded by a newer edit, and let query handlers (hover, completion, etc.)
  wait on a lifecycle signal (`Cold → Indexing → Ready → Reindexing`) rather than block on
  the whole rebuild.

`lsp_server` solved this itself with a token/generation pattern already proven in
production:

```rust
// crates/lsp_server/src/workspace/coordinator.rs
pub(crate) fn subscribe(&self) -> watch::Receiver<SemanticLifecycle>;
pub(crate) fn schedule_relink(&mut self) -> RelinkToken;
pub(crate) fn is_token_current(&self, token: &RelinkToken) -> bool;
pub(crate) fn commit_relink(&mut self, token: &RelinkToken) -> bool;
```

`workspace` crate has no equivalent — its own incremental path
(`crate::snapshot::update::try_incremental_update`, gated behind
`Spec42Engine::experimental_incremental_updates()`) is a narrower, synchronous,
single-document-change optimization with no cancellation and no subscription model. It's the
right computational core, just missing the async wrapper `lsp_server` needs.

## Goal

Move the token/generation/subscription *pattern* down into `workspace` crate as a reusable
type, so `lsp_server` can delegate to it instead of reimplementing it, while leaving
`workspace`'s existing synchronous API (`load_workspace`/`update_snapshot`) untouched for its
current callers (CLI, MCP, batch validation).

## Non-goals

- Do not change the LSP wire protocol or `lsp_server`'s public `run_lsp()` entry point.
- Do not require CLI/MCP/batch callers of `workspace::Spec42Engine` to change at all —
  `WorkspaceSession` (below) wraps the existing API, it doesn't replace it.
- Do not attempt to merge parse caching / library-graph caching in the same pass as the
  coordinator extraction — those are separable follow-ups (see Phase 3).
- Do not touch `sysml_model`'s graph-building or diagnostics logic — already shared.

## Proposed design

> **Note:** this section is the original design sketch. See "Phase 1 status" above for how
> it changed during implementation — mainly, no `tokio`/subscription channel inside
> `workspace` crate, and no bundled snapshot storage. `WorkspaceSession` ended up as a
> smaller, pure state-machine type; the sketch below is kept for historical context on the
> original shape.

Add a new `WorkspaceSession` type to `workspace` crate (new module,
`crates/workspace/src/session.rs`) with the same token/generation/subscription shape
`lsp_server`'s `SemanticCoordinator` already validates in production:

```rust
pub struct WorkspaceSession {
    engine: Spec42Engine,
    current: ArcSwap<HostWorkspaceSnapshot>,   // lock-free read path for query handlers
    lifecycle: watch::Sender<SessionLifecycle>,
    generation: AtomicU64,
}

pub enum SessionLifecycle { Cold, Indexing, Ready, Reindexing }

pub struct RelinkToken { generation: u64 }

impl WorkspaceSession {
    pub fn new(engine: Spec42Engine, initial: Arc<HostWorkspaceSnapshot>) -> Self;
    pub fn current(&self) -> Arc<HostWorkspaceSnapshot>;   // lock-free read
    pub fn subscribe(&self) -> watch::Receiver<SessionLifecycle>;
    pub fn begin_relink(&self) -> RelinkToken;              // bumps generation
    pub fn is_current(&self, token: &RelinkToken) -> bool;  // check before committing
    pub fn commit_relink(&self, token: RelinkToken, snapshot: Arc<HostWorkspaceSnapshot>) -> bool;
}
```

**What actually shipped** (`crates/workspace/src/session.rs`): `WorkspaceSession` owns only
`lifecycle: SessionLifecycle`, `version: u64`, `relink_generation: u64` — no `tokio`, no
snapshot storage, no `ArcSwap`/`AtomicU64` (plain `&mut self` mutation, matching
`SemanticCoordinator`'s existing external-locking discipline). Method names match:
`begin_startup`/`complete_startup`/`schedule_relink`/`is_token_current`/`commit_relink`/
`begin_library_reindex`/`complete_reindex`/`bump_version`/`reset`. No `subscribe()` or
`current()` — see "Phase 1 status" for why.

The actual graph computation inside `schedule_relink`'s caller still goes through
`Spec42Engine::update_snapshot()` (or an async wrapper around it, e.g. `tokio::task::spawn_blocking`
in `lsp_server`, which already has the runtime) — `WorkspaceSession` only owns the
*coordination*, not the graph-building or the snapshot storage. This keeps the change
additive: `workspace` crate gains a new opt-in type, nothing existing moves or breaks.

## Migration plan (phased, each phase independently shippable)

**Phase 1 — Add `WorkspaceSession` to `workspace` crate, unused. ✅ Done 2026-07-02.**
New type, new tests mirroring `lsp_server`'s existing `coordinator.rs` transition table (it
turned out `coordinator.rs` itself had no tests — this is now the first test coverage of the
pattern). Zero behavior change to any existing caller.

**Phase 2 — Migrate `lsp_server`'s `SemanticCoordinator`/`ServerState` to delegate token/
generation bookkeeping to `WorkspaceSession`. ✅ Done 2026-07-02.** Kept `lsp_server`'s own
parse cache, library-graph cache, `std::thread::spawn` staged-rebuild logic, and `tokio::sync::watch`
subscription channel as-is (the last one stays local since `workspace` can't depend on
`tokio` — see Phase 1). Removed the duplicated generation/version transition logic itself;
see "Phase 2 status" above.

**Phase 3a — Replace the manual `std::thread::spawn` pools with `rayon` (Tier 4). ✅ Done
2026-07-02.** See "Phase 3 status" above.

**Phase 3b — Fold the remaining duplicated graph-update logic** (`services.rs`'s
`update_semantic_graph_for_uri`/`rebuild_semantic_graph_staged`, ~1200 lines) into
`workspace` crate's incremental path, extending `try_incremental_update` to cover the
multi-document / library-change cases it currently bails out of (this is also audit item
P2-3). **Rescoped — needs its own design doc first** (see "Phase 3 status" above); this
phase should extend the existing `tests/incremental_parity.rs` property tests in
`workspace` crate to cover the async/cancellation paths before merging.

**Phase 4 — Delete dead code** in `lsp_server/src/workspace/services.rs`/`parse_cache.rs`/
`library_graph_cache.rs` once Phase 3 has parity, and re-measure. Originally expected to
shrink `lsp_server`'s `workspace/` module from ~2988 lines to primarily LSP-protocol glue —
**that expectation assumed the large Phase 3 merge, which was rescoped away (see "Phase 3
status" above); see the "Phase 4" section below for what actually shipped.**

## Risks

- **This is the production LSP hot path.** Every SysML file edit in every connected editor
  goes through this code. Regressions here are directly user-visible (stale diagnostics,
  hangs, or lost edits under rapid typing).
- **Cancellation races** are the hardest part to get right — `commit_relink` must correctly
  reject a relink superseded by a newer edit without deadlocking or dropping a valid update.
  `lsp_server`'s existing `coordinator.rs` has presumably already found and fixed these bugs;
  the design must not regress behavior it already gets right.
- **`experimental_incremental_updates` is already a feature flag** on `Spec42Engine` — reuse
  it (or a similarly-scoped flag) to stage Phase 3 behind a flag before it's load-bearing for
  the live LSP path, so Phase 2 can ship (coordinator only) well ahead of Phase 3 (graph
  logic) with a rollback lever available for each.

## Effort estimate

Phase 1: small, self-contained (~1 session). Phase 2: medium, touches `lsp_server`'s state
wiring but not its graph logic (~1-2 sessions). Phase 3: large — this is the ~1200-line
duplicated-logic removal and needs new async-path test coverage (multi-session). Phase 4:
small cleanup once Phase 3 has landed and soaked.

Total: treat as its own tracked initiative, not a single debt-reduction session. Recommend
starting with Phase 1 (pure addition, no risk) whenever there's appetite to begin.

## Open questions for the maintainer

1. ~~Is `tokio` (or another async runtime) already a dependency available to `workspace`
   crate...~~ **Resolved 2026-07-02: no.** `workspace` must stay runtime-agnostic —
   `tests/dependency_guardrails.rs` forbids `tokio` there deliberately, to keep the crate
   embeddable without committing consumers to a specific async runtime or version. Confirmed
   with the maintainer; `WorkspaceSession` has no async API of its own (see "Phase 1 status").
2. Should Phase 3's `rayon` migration (replacing `std::thread::spawn` in the staged rebuild)
   happen inside `workspace` crate (so CLI/MCP batch validation also benefits) or stay
   `lsp_server`-local until Phase 3 actually merges the logic?
3. Is there an existing soak/rollout mechanism (staged binary release, internal dogfooding)
   to de-risk Phase 3 before it becomes load-bearing for every editor keystroke?
