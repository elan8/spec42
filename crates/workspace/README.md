# spec42_host

Protocol-neutral embedding API for Spec42 host services.

## Responsibilities

### Phase 1 — library catalog

- typed library catalog resolution (`LibraryArchive`, `LibraryInstallRoot`, `LibraryBundle`);
- explicit `cache_dir` for materialization;
- `Spec42Engine::builder()` producing a versioned `LibraryCatalog`;
- optional embedded standard-library and domain-library bundles via crate features.

### Phase 2 — immutable workspace snapshot

- `Spec42Engine::load_workspace(provider, request, context)` builds one `HostWorkspaceSnapshot`;
- snapshot stores semantic graph, parsed documents, `language_service::InMemoryWorkspace`, `WorkspaceRenderSnapshot`, host validation facts, and semantic projection;
- `prepare_view(view, selected_view)` reuses the built graph (no second graph build);
- `HostFilesystemProvider` and `ChangesetDocumentProvider` adapt workspace input.

### Phase 3 — version metadata, structured errors, and execution context

- `HostSchemaVersions` and `HostArtifactMetadata` on every snapshot (`metadata()`), with RFC3339 `built_at` and per-document content hashes;
- `Spec42HostError` with stable machine-readable codes (no `From<String>` catch-all);
- `HostContext` with cooperative cancellation, deadlines, resource limits, and optional progress callbacks.

### Phase 4 — semantic comparison

- `compare_snapshots(previous, next)` returns a versioned `SemanticComparisonReport`;
- facts-only diffs for elements, relationships, diagnostics, and supported-view payload identities;
- `IdentityPreservationStatus` when library environment or document URI sets differ between revisions.

### Phase 5 — incremental snapshot updates

- `Spec42Engine::update_snapshot(previous, changes, request, context)` returns a new `Arc<HostWorkspaceSnapshot>` without re-reading the workspace from disk;
- `DocumentChanges` overlays added, changed, and removed logical documents on a prior snapshot;
- single changed workspace document can use an experimental graph patch; add/remove/multi-doc edits fall back to in-memory full rebuild;
- previous `Arc` remains valid for readers — snapshots are never mutated in place.

## Snapshot lifecycle

1. Resolve libraries via `Spec42Engine::builder()`.
2. Load documents through a `SysmlDocumentProvider` (filesystem, in-memory, or changeset overlay).
3. Build graph, language indexes, and view catalog once.
4. Query `validation()`, `semantic_projection()`, `language_workspace()`, `view_catalog()`, and `prepare_view()` from the same `Arc<HostWorkspaceSnapshot>`.
5. After editor saves, call `update_snapshot` with `DocumentChanges` instead of reloading from disk when the prior snapshot is still valid.

### View-first performance

For hosts that show a diagram before exporting diagnostics, defer validation during load:

```rust
use spec42_host::{ValidationTiming, WorkspaceLoadRequest};

let request = WorkspaceLoadRequest::single_target(model_dir)
    .with_validation_timing(ValidationTiming::Deferred);

let snapshot = engine.load_workspace(provider, request, context)?;
let view = snapshot.prepare_view("general-view", Some("productStructure"))?;
let report = snapshot.ensure_validation()?; // collect diagnostics on demand
```

`prepare_view` reuses the load-time `WorkspaceRenderSnapshot` and scoped IBD for `general-view` / `interconnection-view`. See [ROBOT-VACUUM-PERFORMANCE-ANALYSIS.md](../../docs/engineering/ROBOT-VACUUM-PERFORMANCE-ANALYSIS.md) for before/after timings on the robot-vacuum fixture (~8.6 s → ~2.8 s release cold path).

Snapshots are immutable after construction. Share them across worker threads with `Arc`; types are `Send + Sync`. `update_snapshot` always returns a **new** `Arc`; existing readers keep the previous snapshot until they adopt the new one.

## Incremental updates

Apply logical document overlays on top of a prior snapshot:

```rust
use spec42_host::{DocumentChanges, EngineBuilder, HostContext, WorkspaceLoadRequest};

let engine = EngineBuilder::default()
    .cache_dir(cache_dir)
    .experimental_incremental_updates(true) // default: true; pass false to force full rebuilds
    .build()?;

let request = WorkspaceLoadRequest::single_target(workspace_root.join("Demo.sysml"));
let previous = engine.load_workspace(provider, request.clone(), HostContext::default())?;

let changes = DocumentChanges::new().replace(updated_document);
let next = engine.update_snapshot(
    previous.as_ref(),
    changes,
    request,
    HostContext::default(),
)?;
```

| Input | Behavior |
| --- | --- |
| `DocumentChanges::with_changed` (one workspace doc) | Graph patch (default on; disable with `experimental_incremental_updates(false)`) |
| Add, remove, or multiple changed docs | In-memory full rebuild via `InMemoryDocumentProvider` (no base provider I/O) |
| `experimental_incremental_updates(false)` | Always full-rebuild fallback (still skips filesystem provider) |
| Library catalog change | Always falls back to full rebuild internally, even with the flag on (`can_use_incremental_update` checks `library_catalog_hash`) — prefer calling `load_workspace` directly when you know the catalog changed |

Pass the same `WorkspaceLoadRequest` used for the initial load (`targets`, `workspace_root`, `strict_diagnostics`) so validation and projection scope stay consistent.

**Performance note:** the graph-patch itself skips re-parsing unchanged documents, but
`update_snapshot`'s snapshot assembly (`language_workspace`, `render_snapshot`, eager
`validation_report`, `semantic_projection`) is currently recomputed in full regardless of
the patch — see `docs/engineering/TIER2-UNIFIED-INCREMENTAL-ENGINE-DESIGN.md`. Benchmarking
(`workspace/tests/incremental_benchmark.rs`) has not yet shown a measurable end-to-end win at
this layer; the default was flipped to `true` for single-code-path correctness (no more
`try_incremental_update` drifting from `build_workspace_snapshot`), not a proven speedup.
`lsp_server`'s `ServerState` and Babel42's `EditorSession` don't go through this flag at all —
they hold `IncrementalWorkspace` directly and are unaffected by it either way.

`DocumentChanges` rejects URIs that appear in more than one bucket (`added` / `changed` / `removed`). Use `replace(document)` for single-save editor flows.

**Editor integration:** full workspace open → `load_workspace`; single document save → `update_snapshot` with `DocumentChanges::replace`; catalog or URI-set change → `load_workspace` again.

## Artifact metadata

Every `HostWorkspaceSnapshot` carries `HostArtifactMetadata` via `metadata()`. Persist this JSON alongside rendered artifacts so hosts can detect stale or incompatible snapshots.

| Field | Meaning |
| --- | --- |
| `schema_versions.artifact_metadata_version` | Schema of `HostArtifactMetadata` itself (currently `1`) |
| `schema_versions.projection_schema_version` | Semantic projection DTO layout |
| `schema_versions.renderer_compatibility_version` | Renderer/view compatibility |
| `schema_versions.comparison_schema_version` | `SemanticComparisonReport` layout (currently `1`) |
| `engine_version` | `spec42_host` crate version at build time |
| `library_catalog_hash` | Content hash of the resolved library catalog |
| `built_at` | UTC RFC3339 timestamp (`YYYY-MM-DDTHH:MM:SSZ`) |
| `document_hashes` | Map of document URI → content hash (stable `BTreeMap` key order in JSON) |

Example:

```json
{
  "schema_versions": {
    "artifact_metadata_version": 1,
    "projection_schema_version": 1,
    "renderer_compatibility_version": 1,
    "comparison_schema_version": 1
  },
  "engine_version": "0.35.0",
  "library_catalog_hash": "catalog-hash",
  "built_at": "2026-06-22T12:34:56Z",
  "document_hashes": {
    "file:///demo/A.sysml": "abc123",
    "file:///demo/B.sysml": "def456"
  }
}
```

`Spec42Engine::metadata()` exposes the current `HostSchemaVersions` for pre-flight compatibility checks before loading a workspace.

## Error codes

Host-facing APIs return `HostResult<T>` (`Result<T, Spec42HostError>`). Use `error.code()` for programmatic handling; `Display` includes the code prefix for logs.

| Code | Meaning | Typical host action |
| --- | --- | --- |
| `invalid_document_uri` | URI or path could not be resolved | Fix workspace roots or document paths |
| `parser_failure` | SysML parse or graph build failure | Surface diagnostics to the user |
| `unresolved_library_environment` | Missing catalog, library path, or provider load error | Verify library configuration and `cache_dir` |
| `unsupported_view` | Requested view is not available for this workspace | Offer a different view or fix model |
| `cancelled` | Cooperative cancel or deadline exceeded | Retry or abandon the job |
| `resource_limit_exceeded` | Document count, bytes, or graph size limit hit | Reject oversized workspaces or raise limits |
| `internal_invariant_failure` | Unexpected empty state after prior checks | Log and treat as bug |

The CLI/HTTP/MCP server maps errors to strings via `Display`; embedding hosts should match on `code()` instead.

## HostContext

Pass a `HostContext` to `load_workspace` and `update_snapshot` to control long-running builds in SaaS workers:

```rust
use std::sync::Arc;
use std::time::Duration;
use spec42_host::{CancellationToken, HostContext, HostPipelinePhase, HostResourceLimits};

let cancel = CancellationToken::new();
let context = HostContext::default()
    .with_deadline(Duration::from_secs(30))
    .with_limits(HostResourceLimits {
        max_documents: Some(500),
        max_total_bytes: Some(50 * 1024 * 1024),
        ..Default::default()
    })
    .with_progress(Arc::new(|phase| {
        eprintln!("pipeline: {:?}", phase);
    }));
// Share `cancel` with another thread; call `cancel.cancel()` to abort.
```

Pipeline phases (`HostPipelinePhase`): `LoadingDocuments` → `BuildingGraph` → `BuildingLanguageWorkspace` → `BuildingViewCatalog` → `CollectingValidation` → `ProjectingModel`.

Checks run cooperatively at each step. On `cancelled` or `resource_limit_exceeded`, `load_workspace` and `update_snapshot` return `Err` immediately and **never** return a partial snapshot. Deadlines also map to `cancelled`.

## Semantic comparison

Compare two immutable snapshots after loading different workspace revisions:

```rust
use spec42_host::compare_snapshots;

let report = compare_snapshots(&previous, &next)?;
```

`SemanticComparisonReport` is serde-stable and persistable alongside snapshot artifacts. It reports **semantic facts only** — no inferred engineering impact.

| Section | Contents |
| --- | --- |
| `schema_versions.comparison_schema_version` | Report schema (currently `1`) |
| `previous_artifact` / `next_artifact` | `HostArtifactMetadata` from each snapshot |
| `identity_preservation` | `preserved`, `incompatible_environment`, or `document_set_changed` |
| `elements` | added / removed / changed model elements (key: `uri` + `qualified_name`) |
| `relationships` | added / removed edges (`source`, `target`, `kind`) |
| `diagnostics` | introduced / resolved per document |
| `views` | catalog changes + changed supported-view payload hashes |

### Identity preservation

| Status | Meaning | Host action |
| --- | --- | --- |
| `preserved` | Same engine version, library catalog hash, and document URI set | Trust element keys |
| `incompatible_environment` | Engine version or catalog hash differs | Rebuild both snapshots or compare cautiously |
| `document_set_changed` | Document URI set differs (add/remove/rename) | Add/remove churn may reflect URI drift, not model edits |

Phase 4 does not remap URIs across Git renames; use `document_hashes` in artifact metadata for content-level change detection.

### Diagnostic matching

Diagnostics match on `(uri, code, severity, message)`. **Range is excluded** so line shifts from formatting do not appear as resolve+introduce churn.

### View payload identity

For supported views present in both snapshots, `compare_snapshots` hashes a stable fingerprint of each `prepare_view` result (candidate ids, empty-state message, graph counts, prepared-view key). Full diagram geometry is not diffed.

Example (abbreviated):

```json
{
  "schema_versions": { "comparison_schema_version": 1 },
  "identity_preservation": "preserved",
  "elements": {
    "added": [],
    "removed": [],
    "changed": []
  },
  "relationships": { "added": [], "removed": [] },
  "diagnostics": { "by_document": {} },
  "views": {
    "catalog_added": [],
    "catalog_removed": [],
    "catalog_changed": [],
    "changed_view_payloads": []
  }
}
```

## Validation paths

| Consumer | Diagnostics source |
| --- | --- |
| Embedding host (`snapshot.validation()`) | portable `SemanticDiagnostic` DTOs from `semantic_core` |
| CLI / HTTP / MCP | kernel `semantic_report_from_built_workspace` on the same pre-built graph for full parity |

## Concurrency contract

- `Spec42Engine` and `LibraryCatalog` are `Send + Sync` after `build()` and safe to share across worker threads through an `Arc`.
- `EngineBuilder` is not `Sync`; build on one thread, then share the resulting engine.
- `HostWorkspaceSnapshot` is immutable and `Send + Sync`; share through `Arc`. No interior mutability.
- `CancellationToken` is `Send + Sync`; `Clone` shares the same atomic flag; `cancel()` is idempotent.
- `HostContext` is `Clone` and shares cancellation, deadline, limits, and progress callback across threads.
- Library materialization uses filesystem locks under the configured `cache_dir`.

## Cache contract

- Server embedding mode requires an explicit `cache_dir`.
- `.kpar` archives materialize under `<cache_dir>/materialized/<label>/...`.
- No implicit writes to a user profile directory occur when hosts supply `cache_dir` and explicit library sources.
