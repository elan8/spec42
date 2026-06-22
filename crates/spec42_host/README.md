# spec42_host

Protocol-neutral embedding API for Spec42 host services.

## Responsibilities (Phase 1)

- typed library catalog resolution (`LibraryArchive`, `LibraryInstallRoot`, `LibraryBundle`);
- explicit `cache_dir` for materialization;
- `Spec42Engine::builder()` producing a versioned `LibraryCatalog`;
- optional embedded standard-library and domain-library bundles via crate features.

## Concurrency contract (draft)

- `Spec42Engine` and `LibraryCatalog` are `Send + Sync` after `build()` and safe to share across worker threads through an `Arc`.
- `EngineBuilder` is not `Sync`; build on one thread, then share the resulting engine.
- Library materialization uses filesystem locks under the configured `cache_dir`.

## Cache contract

- Server embedding mode requires an explicit `cache_dir`.
- `.kpar` archives materialize under `<cache_dir>/materialized/<label>/...`.
- No implicit writes to a user profile directory occur when hosts supply `cache_dir` and explicit library sources.
