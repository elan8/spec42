# Tier 2 Phase 3b: Lazy Derived Views + Change-Delta API

**Status:** Design — not implemented. Scope expanded 2026-07-02 (see "Part D: per-view
scoping" and "Architectural approach" below) after further discussion; not yet re-reviewed
end-to-end since that expansion.
**Date:** 2026-07-02
**Related:** `docs/engineering/TIER2-LSP-WORKSPACE-CONSOLIDATION.md` (Phase 3b was rescoped
here pending this doc), `docs/architecture-audit.md` P2-3.

## Architectural approach: additive modules in the existing `workspace` crate, not a new crate

Considered and rejected: starting a brand-new crate with an "ideal" API and migrating
`lsp_server`/Babel42/CLI over to it one at a time. Rejected specifically because **this
codebase already tried that pattern once and it didn't finish**: `workspace` crate itself
was introduced (as `spec42_host`, 2026-06-22) as a clean "host embedding API" alongside the
already-existing `kernel` (now `lsp_server`), with the implicit expectation that `kernel`
would eventually migrate onto it. Ten days later `lsp_server` still hasn't — it kept its own
independent ~2988-line reimplementation, which is the entire reason Tier 2 exists. A second
attempt at "introduce a clean new thing, migrate incrementally" needs a real reason to
believe it will finish this time; none was found.

Instead: everything in this doc ships as new, cleanly-designed modules/types **inside**
`workspace` crate, unconstrained by legacy shape (the same move `WorkspaceSession`,
Phase 1, already used successfully — a from-scratch design with zero friction from
anything pre-existing, added straight into the existing crate). This keeps "migrate
consumers one by one" intact as an idea — consumers opt into new *functions* at their own
pace — without the cost and non-completion risk of standing up a second crate.

## Naming: keep wire-facing accessor names, improve doc comments instead of renaming

Considered renaming `language_workspace()`, `view_catalog()`, `semantic_projection()` to
better signal their actual consumer (code-editor page, diagrams page, Model Explorer page
respectively — see the "what is each one actually for" discussion this doc grew out of).
Decision: **don't rename `semantic_projection()`/`HostSemanticProjection`.** That name is
already a wire contract — serialized to JSON and consumed externally by the CLI, the HTTP
API, and presumably Babel42's frontend. Renaming the Rust accessor is cheap; renaming what's
already flowing over the wire to consumers outside this repo is a real compatibility risk
for a clarity gain a doc comment gets for free. `language_workspace()` is lower-risk to
rename (no wire format involved), but still not done here — prefer doc comments over
mechanical churn at every call site unless a rename is separately justified. Action item:
add a doc comment to each of the three accessors stating its actual purpose and typical
caller, e.g.:

```rust
/// Index for per-position editor queries (hover, completion, goto-definition, rename,
/// references). Pass to `language_service::{hover, complete, goto_definition, ...}` — this
/// is not itself a computed answer, just the lookup structure those functions need.
/// Typical caller: a code-editor pane's per-keystroke/per-request feature handlers.
pub fn language_workspace(&self) -> &InMemoryWorkspace { ... }

/// Flat serializable dump of every node and relationship in the graph. Not an index for
/// further queries — feeds the Model Explorer / element browser / relationship search
/// directly. Typical caller: a "show me the model" page, queried on open/refresh rather
/// than per-keystroke.
pub fn semantic_projection(&self) -> &HostSemanticProjection { ... }

/// The full view catalog (every explicit view evaluated). Expensive — evaluates every
/// `view usage` in the workspace in one pass. Prefer `prepare_view(view_id)` (see Part D)
/// when only one specific view is needed; only call this directly if you actually need
/// the whole catalog (e.g. "list all views").
pub fn view_catalog(&self) -> &WorkspaceRenderSnapshot { ... }
```

## Motivation

Three consumers build on `workspace` crate today, discovered by direct inspection (not
assumed):

1. **CLI / MCP / HTTP API** (`crates/server`) — one-shot: build a snapshot, answer one
   question, exit. No caching between invocations.
2. **`lsp_server`** — long-lived live editor session. Never uses `HostWorkspaceSnapshot`
   at all; reimplements a live, minimal, incrementally-mutated `SemanticGraph` +
   symbol-table in `ServerState`, computing every derived view (hover, completion,
   diagnostics, rendering) on demand per LSP request via a zero-copy
   `ServerStateSnapshot` adapter (`crates/lsp_server/src/workspace/snapshot.rs`)
   implementing `language_service::WorkspaceSnapshot`.
3. **Babel42** (`C:\Git\babel42-v2`, `backend/crates/babel42-spec42/src/session.rs`) — also
   a long-lived live editor session (web-based), built directly on `workspace` crate's
   `Spec42Engine`/`HostWorkspaceSnapshot`/`update_snapshot()`, with
   `experimental_incremental_updates(true)` enabled. Calls `update_snapshot()` on every
   edit via its `EditorSession::update_document`.

Tracing what `update_snapshot()` → `try_incremental_update()` →
`assemble_snapshot_from_state()` (`crates/workspace/src/snapshot/update.rs`) actually does
on every Babel42 edit: the semantic graph patch is genuinely incremental (single document),
but `language_workspace`, `render_snapshot`, and `semantic_projection` are **unconditionally
rebuilt from scratch, across the entire workspace**, on every single call — a full
symbol-table rescan, a full diagram/view-catalog rebuild, and a full projection rebuild over
every target file, regardless of whether the edit touched anything those views care about,
and regardless of whether the consuming UI is even looking at them. This is a real,
currently-shipping cost for Babel42, not a hypothetical one.

## Goals

- Don't compute a derived view unless something actually asks for it.
- Give callers a way to know *what changed* after an update, without `workspace` crate
  owning any notion of "subscribers," "topics," or an async runtime.
- Stay non-breaking for existing callers (`crates/server`, Babel42's `EditorSession`)
  wherever the evidence supports it; be explicit and flag it wherever it can't be.
- Don't ask `lsp_server` to change conceptually — it already has this shape (lazy
  graph-only + on-demand snapshot adapter + its own push channel). The goal is giving it
  (and Babel42) a *shared* primitive to delegate to, not a new shape to adopt.

## Non-goals

- No pub/sub, channel, or subscriber registry inside `workspace` crate. That stays
  host-side (see "Host-side push layer" below) — `workspace` must stay free of `tokio`/
  `clap`/`axum`/`rmcp`/`tower-lsp` (enforced by `tests/dependency_guardrails.rs`).
- No cross-generation caching of derived views (i.e. reusing a previous `render_snapshot`
  if the edit didn't affect it). That needs real dependency tracking between graph regions
  and view outputs, which doesn't exist yet — this is the "needs dependency tracking to
  scope relinking to affected files only" item already tracked separately. Out of scope
  here; the win in this doc is purely "don't compute it *this* time if nobody asked," not
  "reuse what I computed last time."
- No change to `WorkspaceSession` (Phase 1/2) — it stays a pure coordination primitive,
  orthogonal to and composable with what's described here (see "Relationship to
  `WorkspaceSession`" below).

## Part A: Lazy derived views on `HostWorkspaceSnapshot`

### Current shape

`crates/workspace/src/snapshot/build.rs`:
```rust
pub struct HostWorkspaceSnapshot {
    ...
    semantic_graph: SemanticGraph,                       // eager, always
    parsed_documents: Vec<WorkspaceParsedDocument>,       // eager, always
    language_workspace: InMemoryWorkspace,                // eager, always — MOVE TO LAZY
    render_snapshot: WorkspaceRenderSnapshot,              // eager, always — MOVE TO LAZY
    validation_report: OnceLock<HostValidationReport>,     // already lazy (opt-in eager)
    semantic_projection: HostSemanticProjection,           // eager, always — MOVE TO LAZY
    full_ibd_cache: OnceLock<IbdDataDto>,                  // already lazy
    ...
}
```

### Proposed shape

```rust
pub struct HostWorkspaceSnapshot {
    ...
    semantic_graph: SemanticGraph,
    parsed_documents: Vec<WorkspaceParsedDocument>,
    language_workspace: OnceLock<InMemoryWorkspace>,
    render_snapshot: OnceLock<WorkspaceRenderSnapshot>,
    render_version: u64,                                   // cheap, always assigned
    validation_report: OnceLock<HostValidationReport>,      // unchanged
    semantic_projection: OnceLock<HostSemanticProjection>,
    full_ibd_cache: OnceLock<IbdDataDto>,                   // unchanged
    ...
}

impl HostWorkspaceSnapshot {
    pub fn language_workspace(&self) -> &InMemoryWorkspace {
        self.language_workspace.get_or_init(|| {
            InMemoryWorkspace::from_graph_and_documents(
                self.semantic_graph.clone(),
                self.parsed_documents.clone(),
                &self.documents,
            )
            .expect("InMemoryWorkspace::from_graph_and_documents is infallible in practice \
                     — see the fallibility note below")
        })
    }

    pub fn view_catalog(&self) -> &WorkspaceRenderSnapshot {
        self.render_snapshot.get_or_init(|| {
            build_render_snapshot(
                &self.semantic_graph,
                &self.parsed_documents,
                &self.library_urls,
                &self.workspace_root_uri,
                self.render_version,
            )
            .expect("build_render_snapshot has no reachable Err path today — see note below")
        })
    }

    pub fn semantic_projection(&self) -> &HostSemanticProjection {
        self.semantic_projection.get_or_init(|| {
            project_host_semantic_model(&self.semantic_graph, &self.validation_target_files)
                .expect("target_files were already resolved successfully earlier in this \
                         same snapshot build — see note below")
        })
    }
}
```

Public accessor **signatures are unchanged** (`&InMemoryWorkspace`, `&WorkspaceRenderSnapshot`,
`&HostSemanticProjection` — no `Result`, no `Option`). Every existing caller (`crates/server`,
Babel42's `EditorSession`) keeps compiling and working exactly as today, they just stop
paying for views they never call.

### The fallibility question (a real decision, not glossed over)

Each of `InMemoryWorkspace::from_graph_and_documents`, `build_render_snapshot`, and
`project_host_semantic_model` has a `Result<_, String>` signature today, but I traced each
body:

- `InMemoryWorkspace::from_graph_and_documents` (`crates/language_service/src/workspace.rs:60`)
  — **no `Err` path in the body at all.** Always returns `Ok`.
- `build_render_snapshot` → `build_view_index` (`crates/sysml_model/src/semantic/render_snapshot.rs:59,83`)
  → `build_workspace_visualization_artifacts` (`crates/sysml_model/src/semantic/visualization/response.rs`)
  — no `Err(` or `?` found in the traced path. Appears infallible in practice today, though
  I did not exhaustively trace every function it calls transitively.
- `project_host_semantic_model` (`crates/workspace/src/snapshot/facts.rs:71`) — the one real
  fallible step is `target_file_urls(target_files)?` (path→URL conversion). But
  `target_files` is the *same* value already successfully resolved earlier in the same
  snapshot-build call (via `discover_target_files`), so re-resolving it inside the lazy
  accessor should not realistically fail except in a TOCTOU race (e.g. the file is deleted
  between snapshot build and this access).

**Recommendation: use `.expect()` with a descriptive message, not a signature change.**
This keeps the migration fully non-breaking. But this is a judgment call based on reading
current code, not a language guarantee — if any of these three builders grows a genuine
error path in the future, the `.expect()` needs to be revisited (at that point, changing
the accessor to return `Result` becomes an actual breaking change, and should be flagged to
Babel42 explicitly rather than silently shipped). Each `.expect()` call site should carry a
comment pointing back to this doc so a future author knows why it's there and what to check
before touching it.

### `render_version`

`assemble_snapshot_from_state` currently computes `render_version` as
`previous.view_catalog().version.wrapping_add(1)` — reading the *previous* snapshot's
already-built `render_snapshot.version` to derive the next one. Once `render_snapshot` is
lazy, that read would force eager computation of the previous snapshot's view catalog just
to number the next one, defeating the point. Fix: store `render_version: u64` as a cheap,
always-assigned plain field on `HostWorkspaceSnapshot` (bumped on every `update_snapshot`
call, independent of whether the view catalog itself is ever computed), and use that field
both for numbering the next update and inside the lazily-built `WorkspaceRenderSnapshot`
itself.

## Part B: Snapshot change-delta

New type, `crates/workspace/src/snapshot/delta.rs`:

```rust
/// What changed between a previous snapshot and the one just produced by
/// `update_workspace_snapshot`. Pure data — no async, no runtime dependency. Callers use
/// this to decide what (if anything) to recompute or push to their own connected clients.
#[derive(Debug, Clone, Default)]
pub struct SnapshotDelta {
    pub changed_uris: Vec<Url>,
    pub added_uris: Vec<Url>,
    pub removed_uris: Vec<Url>,
    pub library_catalog_changed: bool,
}
```

Attached as a plain field on `HostWorkspaceSnapshot` (not a change to `update_snapshot`'s
return type — additive, non-breaking):

```rust
impl HostWorkspaceSnapshot {
    pub fn last_change(&self) -> &SnapshotDelta {
        &self.last_change
    }
}
```

- `build_workspace_snapshot` (full build) populates it with every loaded URI in
  `added_uris` (there's no "previous" to diff against).
- `try_incremental_update`/the full-rebuild fallback path in `update_workspace_snapshot`
  populate it from the `DocumentChanges` the caller passed in (`changed`/`added`/`removed`
  already exist on `DocumentChanges` — this is mostly plumbing the already-available
  information through to the output, not new computation) plus a
  `library_catalog_changed` bool (already computed today for the incremental-eligibility
  check in `can_use_incremental_update`, just not currently surfaced).

Callers (Babel42, and eventually `lsp_server` if it adopts this path) read
`snapshot.last_change()` after an update to decide what they care about — e.g. "the graph
touched `Foo.sysml`, and I have a diagram view open that depends on it → go pull
`view_catalog()` now and push it to my websocket clients." `workspace` crate never needs to
know that "diagram view" or "websocket clients" exist.

## Part C: Host-side push layer (explicitly *not* in `workspace`)

Not implemented here — this is guidance for `lsp_server` and Babel42's server, each
building their own thin wrapper, mirroring the pattern `lsp_server`'s `coordinator.rs`
already proved in Phase 2:

- `lsp_server` already does this (its own `tokio::sync::watch` layered over
  `WorkspaceSession`). No change needed.
- Babel42's server would do the analogous thing: after `EditorSession::update_document`
  calls `update_snapshot()`, read `snapshot.last_change()`, decide which of its own
  websocket-subscribed UI panels care about the touched URIs, and push to those — using
  whatever async/websocket stack Babel42's backend already has. This is new work on
  Babel42's side, not something this doc's `workspace`-crate changes do automatically; it's
  enabled by them, not implemented by them.

## Part D: Per-view scoping (bigger than Parts A-C, own sub-phase)

`Part A`'s `OnceLock` on `view_catalog()` only achieves "pay once per snapshot instead of
once per edit." It does not achieve "pay only for the one view you asked for" — `prepare_view(view_id)`
already exists as a *named* single-view entry point, but its implementation forces the
whole catalog to be built first. Traced the actual cost structure
(`crates/sysml_model/src/semantic/explicit_views.rs`,
`crates/sysml_model/src/semantic/visualization/projection.rs`) before proposing a fix:

- `explicit_views::evaluate_views()` is **already structurally per-view** — it's
  `catalog.usages.iter().map(|usage| { ... })`, each view's evaluation isolated in the loop
  body, sharing only a `node_by_id`/`parent_by_id` lookup built once. Splitting out a
  `evaluate_view(catalog: &ViewCatalog, view_id: &str, semantic_graph, graph) -> Option<EvaluatedView>`
  that evaluates just one usage is a real, moderate, buildable refactor — not a rewrite.
- What it depends on is **not** per-view: `graph: SysmlGraphDto`, built by
  `build_workspace_graph_dto_for_uris()` (`visualization/projection.rs:17`), iterates every
  node/edge for every workspace URI unconditionally — genuinely O(whole workspace graph),
  no way around it without deeper restructuring of the projection step itself (out of scope
  here). `build_view_catalog()` (the AST scan for `view def`/`view usage` declarations) is
  also whole-workspace but cheap (syntax-only, no graph derivation).

Proposed structure — three independently-cacheable layers instead of one monolithic
`OnceLock`:

```rust
pub struct HostWorkspaceSnapshot {
    ...
    workspace_graph_dto: OnceLock<SysmlGraphDto>,       // layer 1: whole-workspace, shared
    view_catalog_ast: OnceLock<ViewCatalog>,             // layer 2: whole-workspace, cheap, shared
    evaluated_views: Mutex<HashMap<String, EvaluatedView>>, // layer 3: per-view, keyed by view id
    ...
}

impl HostWorkspaceSnapshot {
    /// Evaluate and return just the requested view. Builds the shared graph DTO and view
    /// catalog once (cached across all calls on this snapshot), then evaluates only the
    /// requested view (cached per view id) — does not evaluate every other view in the
    /// workspace the way `view_catalog()` does.
    pub fn prepare_view(&self, view_id: &str, ...) -> Result<SysmlVisualizationResultDto, WorkspaceError> {
        let graph = self.workspace_graph_dto.get_or_init(|| build_workspace_graph_dto_for_uris(...));
        let catalog = self.view_catalog_ast.get_or_init(|| build_view_catalog(...));
        // look up or evaluate-and-cache just `view_id` in `self.evaluated_views`
        ...
    }
}
```

This is materially bigger than Parts A-C — it touches `explicit_views.rs`,
`render_snapshot.rs`, and `visualization/response.rs`, not just `HostWorkspaceSnapshot`'s
field types, and needs its own test coverage confirming a single-view request doesn't
evaluate unrelated views. Treat as its own implementation step (Step 5, see migration plan),
not bundled into the Part A/B rollout.

## Relationship to `WorkspaceSession` (Phase 1/2)

No changes to `WorkspaceSession` itself. It answers "is it safe to commit this relink" and
tracks lifecycle/generation; `SnapshotDelta` answers "what did this update actually touch."
They compose but are independent: a host wraps `WorkspaceSession` for the
concurrency/cancellation question and reads `SnapshotDelta` off each committed snapshot for
the "what should I push" question. Neither needs to know about the other.

## Migration plan

**Step 1 — Lazy derived views (Part A).** Convert the three fields, add the fallibility
comments, adjust `render_version` handling. Fully non-breaking for existing callers
(verify: `crates/server`'s handlers, Babel42's `EditorSession` — both only ever call the
accessor methods, never construct `HostWorkspaceSnapshot` fields directly). Test by
confirming `workspace`'s existing test suite passes unchanged, and add a new test asserting
a derived view is *not* computed until its accessor is called (e.g. via a call counter or
by checking a `OnceLock::get().is_none()` before first access).

**Step 2 — `SnapshotDelta` (Part B).** Additive field + accessor. Non-breaking. Test:
`DocumentChanges` round-trips correctly into `changed_uris`/`added_uris`/`removed_uris` for
both the incremental and full-rebuild paths.

**Step 3 — (separate, Babel42-side, not this repo).** Babel42's server adopts
`snapshot.last_change()` to build its own push notifications. Out of scope for spec42 itself
beyond making the primitive available.

**Step 4 — (optional, later) `lsp_server` migration.** With lazy views now precedented in
`workspace`, revisit whether `lsp_server`'s `ServerStateSnapshot` pattern could delegate to
`HostWorkspaceSnapshot`-shaped accessors instead of its own from-scratch per-request
computation — this is the original Tier 2 Phase 3b goal, now much closer to reachable since
both sides have converged on "compute lazily, on demand." Not designed in detail here;
revisit after Steps 1-2 land and Babel42 has validated the win in practice.

**Step 5 — Per-view scoping (Part D).** Larger, separate implementation effort:
`workspace_graph_dto`/`view_catalog_ast` as two more `OnceLock`s, plus a per-view-id cache
for `EvaluatedView`, plus splitting `evaluate_views()` into a callable single-view
`evaluate_view()`. Needs new tests confirming `prepare_view(one_id)` doesn't evaluate every
other view. Do after Steps 1-2 have landed and the simpler lazy-field win has been measured
in practice — this step is worth doing only if the whole-catalog cost (even cached
once-per-snapshot) is still actually showing up as a problem for Babel42's diagrams page.

**Doc-comment step (Naming section above).** Small, independent, can land any time —
add the three accessor doc comments regardless of whether/when Steps 1-5 ship.

## Compatibility check

Grepped Babel42's `EditorSession` (`backend/crates/babel42-spec42/src/session.rs`) — it
only calls `.language_workspace()`, `.ensure_validation()`, `.documents()`,
`.parsed_documents()` via accessor methods, never touches `HostWorkspaceSnapshot` fields
directly. Steps 1-2 as designed require zero changes on Babel42's side to keep compiling
and working — the win (not computing `render_snapshot`/`semantic_projection` unless pulled)
applies automatically, since `EditorSession` today never calls `view_catalog()` or
`semantic_projection()` inside `update_document` — those are presumably called from
whichever request handler serves Babel42's diagram/model-view pages, elsewhere in
`babel42-spec42` or `babel42-server`, not on every edit. (Not fully confirmed — worth a
quick check in Babel42's request handlers before Step 1 ships, to make sure nothing calls
`view_catalog()`/`semantic_projection()` speculatively on every `update_document` today in
a way that would negate the win.)

## Open questions

1. Does `build_workspace_visualization_artifacts` (deeper in the render-snapshot call
   chain) have a genuine error path I didn't trace far enough to find? Worth a closer read
   before committing to `.expect()` there specifically.
2. Should `SnapshotDelta` also carry added/removed *node counts* or just URIs? URIs seem
   sufficient for "should I recompute this view" decisions; richer diffs can be added later
   without breaking this shape.
3. Should Babel42's request handlers be checked (in their repo) before Step 1 ships, to
   confirm none of them call `view_catalog()`/`semantic_projection()` unconditionally on
   every edit today (which would mean Step 1 alone doesn't help without also changing
   Babel42's call sites)?
4. Step 5 (per-view scoping) is real effort — should it be measured/justified with actual
   timing data from Babel42's diagrams page after Steps 1-2 ship, rather than committed to
   upfront? It's plausible the once-per-snapshot `OnceLock` cache alone is enough in
   practice (e.g. if Babel42 debounces edits before triggering a diagram refresh, or if
   `build_workspace_graph_dto_for_uris`'s cost is small relative to per-view rendering on
   typical workspace sizes) and Step 5 turns out not to be worth its own implementation
   cost. Recommend treating Step 5 as conditional on evidence, not scheduled outright.
