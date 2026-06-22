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

## Snapshot lifecycle

1. Resolve libraries via `Spec42Engine::builder()`.
2. Load documents through a `SysmlDocumentProvider` (filesystem, in-memory, or changeset overlay).
3. Build graph, language indexes, and view catalog once.
4. Query `validation()`, `semantic_projection()`, `language_workspace()`, `view_catalog()`, and `prepare_view()` from the same `Arc<HostWorkspaceSnapshot>`.

Snapshots are immutable after construction. Share them across worker threads with `Arc`; types are `Send + Sync`.

## Validation paths

| Consumer | Diagnostics source |
| --- | --- |
| Embedding host (`snapshot.validation()`) | portable `SemanticDiagnostic` DTOs from `semantic_core` |
| CLI / HTTP / MCP | kernel `semantic_report_from_built_workspace` on the same pre-built graph for full parity |

## Concurrency contract

- `Spec42Engine` and `LibraryCatalog` are `Send + Sync` after `build()` and safe to share across worker threads through an `Arc`.
- `EngineBuilder` is not `Sync`; build on one thread, then share the resulting engine.
- `HostWorkspaceSnapshot` is immutable and `Send + Sync`; share through `Arc`.
- Library materialization uses filesystem locks under the configured `cache_dir`.

## Cache contract

- Server embedding mode requires an explicit `cache_dir`.
- `.kpar` archives materialize under `<cache_dir>/materialized/<label>/...`.
- No implicit writes to a user profile directory occur when hosts supply `cache_dir` and explicit library sources.
