# Tier 2: Consolidating `workspace` and `lsp_server`'s Incremental Update Machinery

**Status:** Proposal — design only, no implementation started.
**Date:** 2026-07-02
**Related:** `docs/architecture-audit.md` (P1-2, P2-3, P2-4, P2-9), Technical Debt Reduction Plan Tier 2.

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

Add a new `WorkspaceSession` type to `workspace` crate (new module,
`crates/workspace/src/session.rs`) that wraps an `Arc<HostWorkspaceSnapshot>` with the same
token/generation/subscription shape `lsp_server`'s `SemanticCoordinator` already validates in
production:

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

The actual graph computation inside `begin_relink`'s caller still goes through
`Spec42Engine::update_snapshot()` (or an async wrapper around it, e.g. `tokio::task::spawn_blocking`)
— `WorkspaceSession` only owns the *coordination*, not the graph-building. This keeps the
change additive: `workspace` crate gains a new opt-in type, nothing existing moves or breaks.

## Migration plan (phased, each phase independently shippable)

**Phase 1 — Add `WorkspaceSession` to `workspace` crate, unused.**
New type, new tests mirroring `lsp_server`'s existing `coordinator.rs` test suite (adapted).
Zero behavior change to any existing caller. Lowest risk, can land any time.

**Phase 2 — Migrate `lsp_server`'s `SemanticCoordinator`/`ServerState` to delegate token/
generation/subscription bookkeeping to `WorkspaceSession`,** keeping `lsp_server`'s own parse
cache, library-graph cache, and `std::thread::spawn` staged-rebuild logic as-is for now. This
is the highest-value, lowest-risk migration step: it removes the coordinator duplication
(~190 lines) while leaving the graph-building internals (which are already working in
production) untouched.

**Phase 3 — Fold the remaining duplicated graph-update logic** (`services.rs`'s
`update_semantic_graph_for_uri`/`rebuild_semantic_graph_staged`, ~1200 lines) into
`workspace` crate's incremental path, extending `try_incremental_update` to cover the
multi-document / library-change cases it currently bails out of (this is also audit item
P2-3). Replace the manual `std::thread::spawn` pools with `rayon` at the same time (Tier 4)
since both are being touched. This phase should extend the existing
`tests/incremental_parity.rs` property tests in `workspace` crate to cover the async/
cancellation paths before merging.

**Phase 4 — Delete dead code** in `lsp_server/src/workspace/services.rs`/`parse_cache.rs`/
`library_graph_cache.rs` once Phase 3 has parity, and re-measure. Expected outcome: `lsp_server`'s
`workspace/` module shrinks from ~2988 lines to primarily LSP-protocol glue (document sync,
capability wiring) — most of `library_search.rs` (336 lines, symbol-table indexing for
completions) and `import_graph.rs` (189 lines) stay, since they're genuinely LSP-specific
features with no `workspace`-crate equivalent.

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

1. Is `tokio` (or another async runtime) already a dependency available to `workspace`
   crate, or does `WorkspaceSession`'s async relink wrapper need to stay runtime-agnostic
   (e.g. take a caller-supplied executor closure)?
2. Should Phase 3's `rayon` migration (replacing `std::thread::spawn` in the staged rebuild)
   happen inside `workspace` crate (so CLI/MCP batch validation also benefits) or stay
   `lsp_server`-local until Phase 3 actually merges the logic?
3. Is there an existing soak/rollout mechanism (staged binary release, internal dogfooding)
   to de-risk Phase 3 before it becomes load-bearing for every editor keystroke?
