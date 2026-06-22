# ADR 0003: `spec42_host` crate for protocol-neutral embedding

| Field | Value |
| --- | --- |
| **Status** | Proposed |
| **Date** | 2026-06-22 |
| **Authors** | Spec42 maintainers |

## Context

Spec42 today exposes its semantic engine through four primary surfaces:

| Surface | Transport | Primary consumer |
| --- | --- | --- |
| LSP + custom `sysml/*` RPC | stdio JSON-RPC | Editors (VS Code, Zed) |
| CLI (`check`, `doctor`, `diagrams export`, …) | process invocation | CI, scripts |
| MCP (`spec42-mcp`) | stdio MCP | AI assistants |
| HTTP (`spec42 api serve`) | REST | Local integrations, dashboards |

ADR 0002 introduced `language_service` so editor intelligence is available without `tower-lsp`. `semantic_core` already provides graph construction, resolution, diagnostics, and visualization helpers without filesystem or protocol coupling.

Embedding hosts integrate Spec42 as Rust libraries to build immutable projection artifacts for Git-native or service-hosted SysML v2 workflows. An external integration spike proved the semantic boundary is usable, but the practical integration surface still lives in the `spec42` server crate:

- `perform_check_with_semantics`
- `environment::resolve_environment`
- `diagrams::build_diagram_payload`

Hosts must currently construct a `Cli` value and depend on the server crate, which also compiles CLI parsing, MCP, HTTP, embedded-library materialization, and headless rendering that projection workers do not need.

Additional embedding friction observed in the spike:

- passing a raw domain-library `.kpar` archive as a library path is not equivalent to passing Spec42's materialized domain-library root (8 errors and 389 warnings vs. clean validation);
- validation and view projection currently rebuild the workspace independently;
- several reusable APIs return `Result<T, String>`;
- a full robot-vacuum validation plus selected-view projection took approximately 102 seconds in a development build.

The read-only HTTP API (ADR 0001) is useful for debugging and parity tests, but it is not the right production boundary for multi-tenant projection workers: embedding hosts need in-process integration with explicit library catalogs, immutable snapshots, versioned artifacts, and SaaS-style cancellation and resource limits.

## Decision

Introduce a new workspace crate, `crates/spec42_host`, that provides a **stable, protocol-neutral embedding API** for host services.

### Responsibilities

`spec42_host` owns:

1. **Library catalog resolution** — typed distinction between archives, bundles, managed installs, and resolved package roots; explicit cache directory; no implicit writes to a user profile in server embedding mode.
2. **Engine builder** — `Spec42Engine::builder()` replacing `Cli`-shaped environment setup for embedders.
3. **Immutable workspace snapshot** — one build producing validation, semantic projection, language-service indexes, and view catalog / prepared-view payloads.
4. **Host-facing metadata** — engine version, projection schema version, renderer compatibility version, library catalog identity, and source-document hashes.
5. **Execution context** — cancellation, deadlines, resource limits, and progress events for SaaS workers.
6. **Structured errors** — stable error codes with structured context at the public boundary.
7. **Semantic comparison** — facts-only diff between two immutable snapshots (follow-up phase).
8. **Incremental snapshot updates** — new immutable snapshot from a previous snapshot plus document changes (later phase).

### Public API (target)

```rust
let engine = Spec42Engine::builder()
    .cache_dir(cache_dir)
    .library_catalog(library_catalog)
    .build()?;

let snapshot = engine.load_workspace(document_provider, HostContext::default())?;

let validation = snapshot.validation();
let projection = snapshot.semantic_projection();
let language = snapshot.language_workspace();
let views = snapshot.view_catalog();
let prepared = snapshot.prepare_view(renderer_view, selected_view)?;
```

`SysmlDocumentProvider` is the primary workspace input. Filesystem-backed workspaces are adapters, not the only supported mode.

### Layering

```text
semantic_core       — graph, resolution, diagnostics, providers, render snapshot helpers
language_service    — editor intelligence over WorkspaceSnapshot
spec42_host         — library catalog, engine builder, immutable snapshot, host DTOs
kernel              — LSP/runtime adapters
server (spec42)     — CLI, MCP, HTTP; thin wrappers over spec42_host
```

### Dependency rules

- `spec42_host` depends on **`semantic_core` and `language_service` only** (plus shared workspace deps such as `serde`, `thiserror`, `sysml-v2-parser`).
- `spec42_host` must **not** depend on `kernel`, `tower-lsp`, `tokio`, `clap`, `rmcp`, or Axum.
- `server` and `kernel` migrate call sites to `spec42_host` instead of duplicating environment and snapshot assembly.
- Enforce the dependency rule with a crate-level guardrail test, matching ADR 0002.

### Migration of existing surfaces

Existing surfaces remain; they become consumers of `spec42_host`:

| Surface | Current integration point | After migration |
| --- | --- | --- |
| `spec42 check` | `perform_check_with_semantics(&Cli, …)` | `Spec42Engine` + snapshot query |
| `spec42 doctor` | `resolve_environment(&Cli)` | engine builder + `LibraryCatalog` report |
| HTTP `/v1/validate`, `/v1/model/*` | `perform_*` in `crates/server` | same snapshot builder as CLI |
| MCP model summary | `perform_check_with_semantics` | same snapshot builder |
| LSP / kernel | `validate_paths_with_semantics`, render snapshot | kernel adapts snapshot or shared builder |
| Embedding host projection worker | fake `Cli` + server crate | direct `spec42_host` dependency |

`resolve_environment(&Cli)` remains as a thin adapter over the engine builder for backward compatibility during migration.

## Non-goals

- Replacing LSP, MCP, or HTTP as debugging and parity surfaces.
- Defining host storage, billing, multi-tenant orchestration, or Git workflows.
- Full OMG Systems Modeling API repository semantics (projects, commits, element UUID CRUD).
- Inferring engineering impact in semantic comparison results.
- Shipping incremental snapshot updates before the full-rebuild path is correct, measured, and covered by parity tests.

## Implementation plan

Detailed sequencing, acceptance criteria, and file-level tasks live in [HOST-EMBEDDING-IMPLEMENTATION-PLAN.md](../engineering/HOST-EMBEDDING-IMPLEMENTATION-PLAN.md).

### Phase summary

| Phase | Focus |
| --- | --- |
| 1 | `spec42_host` crate skeleton + typed library catalog |
| 2 | `HostWorkspaceSnapshot` — single build for validation and views |
| 3 | Version metadata, structured errors, `HostContext` |
| 4 | `compare_snapshots` API |
| 5 | Incremental `update_snapshot` API |

Phases 1 and 2 are the critical path for host embedding MVP.

### Parity requirement

Every phase must preserve external behaviour on CLI, HTTP, and MCP unless a host API change is explicitly versioned. `spec42_host` embedding integration tests and `language_service` headless tests are additional embedding baselines.

## Consequences

### Positive

- Embedding hosts can depend on a small crate without CLI, MCP, HTTP, or LSP compile-time cost.
- One immutable snapshot maps directly to host projection artifacts.
- Explicit library catalogs prevent silent correctness failures (archive vs. materialized root).
- CLI, HTTP, MCP, and LSP share one embedding path — single source of truth for validation and projection.
- Versioned host schemas enable reproducible artifacts across deployments.
- Structured errors and cancellation support SaaS projection workers.

### Trade-offs

- New public API surface to maintain, version, and test.
- Extraction from `server` is a non-trivial refactor; care is required to avoid a second parallel pipeline.
- `spec42_host` initially performs full workspace rebuilds; incremental updates come later.
- Some kernel concerns (document lifecycle, LSP sync) remain outside `spec42_host` by design.

### Risks

| Risk | Mitigation |
| --- | --- |
| `spec42_host` duplicates pipeline logic | Extract from `server`; make `server` a consumer, not a parallel implementation |
| Public schema churn | Introduce `HostSchemaVersions` before hosts persist production artifacts |
| Incremental update correctness | Ship only after full-rebuild parity tests; keep full rebuild as fallback |
| Implicit profile/cache writes in servers | Require explicit `cache_dir` or read-only catalog in embedding mode |
| Scope creep toward full OMG repository API | Non-goals; repository semantics remain a host-service concern |

## Alternatives considered

| Alternative | Why not |
| --- | --- |
| Keep embedding through the `spec42` server crate | Forces fake `Cli` values, heavy dependencies, and duplicated workspace rebuilds — confirmed by integration spike |
| Use the read-only HTTP API as the production embedding boundary | Process/network overhead, no in-memory changesets, poor fit for multi-tenant workers |
| Put all embedding logic in the host service | Duplicates library resolution and snapshot assembly; drifts from CLI/LSP parity |
| Extend `language_service` to own validation and views | Violates layering; `language_service` is editor intelligence, not full workspace projection |
| Depend on `kernel` from embedders | Pulls LSP/runtime concerns into the embedding host |
| gRPC sidecar around `spec42` CLI | Child-process boundary; no shared in-memory snapshots or cancellation |

## Relationship to other ADRs

| ADR | Relationship |
| --- | --- |
| [0001](0001-read-only-systems-modeling-http-api.md) | HTTP API becomes a thin consumer of `spec42_host`; remains useful for parity and local debugging |
| [0002](0002-language-service-crate.md) | `spec42_host` builds on `language_service::InMemoryWorkspace` / `WorkspaceSnapshot` inside immutable snapshots |

## References

- [HOST-EMBEDDING-IMPLEMENTATION-PLAN.md](../engineering/HOST-EMBEDDING-IMPLEMENTATION-PLAN.md)
- [SEMANTIC_CORE_ARCHITECTURE.md](../architecture/SEMANTIC_CORE_ARCHITECTURE.md)
- [STDLIB-RESOLUTION-GUIDE.md](../engineering/STDLIB-RESOLUTION-GUIDE.md)
- [ADR 0002: `language_service` crate](0002-language-service-crate.md)
