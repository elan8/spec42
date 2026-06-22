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

## Snapshot lifecycle

1. Resolve libraries via `Spec42Engine::builder()`.
2. Load documents through a `SysmlDocumentProvider` (filesystem, in-memory, or changeset overlay).
3. Build graph, language indexes, and view catalog once.
4. Query `validation()`, `semantic_projection()`, `language_workspace()`, `view_catalog()`, and `prepare_view()` from the same `Arc<HostWorkspaceSnapshot>`.

Snapshots are immutable after construction. Share them across worker threads with `Arc`; types are `Send + Sync`.

## Artifact metadata

Every `HostWorkspaceSnapshot` carries `HostArtifactMetadata` via `metadata()`. Persist this JSON alongside rendered artifacts so hosts can detect stale or incompatible snapshots.

| Field | Meaning |
| --- | --- |
| `schema_versions.artifact_metadata_version` | Schema of `HostArtifactMetadata` itself (currently `1`) |
| `schema_versions.projection_schema_version` | Semantic projection DTO layout |
| `schema_versions.renderer_compatibility_version` | Renderer/view compatibility |
| `schema_versions.comparison_schema_version` | Reserved for Phase 4 comparison reports (`0` until implemented) |
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
    "comparison_schema_version": 0
  },
  "engine_version": "0.32.0",
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

Pass a `HostContext` to `load_workspace` to control long-running builds in SaaS workers:

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

Checks run cooperatively at each step. On `cancelled` or `resource_limit_exceeded`, `load_workspace` returns `Err` immediately and **never** returns a partial snapshot. Deadlines also map to `cancelled`.

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
