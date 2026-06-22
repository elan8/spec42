# ADR 0001: Read-only HTTP API for SysML v2 workspace access

| Field | Value |
| --- | --- |
| **Status** | Accepted |
| **Date** | 2026-06-05 |
| **Authors** | Spec42 maintainers |

## Context

Spec42 today exposes its semantic engine through three surfaces:

| Surface | Transport | Primary consumer |
| --- | --- | --- |
| LSP + custom `sysml/*` RPC | stdio JSON-RPC | Editors (VS Code, Zed) |
| CLI (`check`, `model-summary`, `diagrams export`, …) | process invocation | CI, scripts, VS Code LM Tools |
| MCP (`spec42-mcp`) | stdio MCP | AI assistants (Cursor, Copilot) |

These surfaces work well for editing, automation, and agent workflows. They are **not** suitable for arbitrary external software that expects a network API: dashboards, PLM bridges, custom traceability tools, local microservices, or language-agnostic integrations.

The OMG **Systems Modeling API and Services** specification defines the industry-standard interoperability layer for SysML v2. It includes a Platform Independent Model (PIM) and REST/HTTP and OSLC platform-specific models (PSMs). The reference implementation ([SysML-v2-API-Services](https://github.com/Systems-Modeling/SysML-v2-API-Services)) is repository-centric: projects, branches, commits, element CRUD, and persistent identity.

Spec42's product wedge is **open, local-first tooling** (see [COMPETITIVE-ROADMAP.md](../engineering/COMPETITIVE-ROADMAP.md)): bundled server, file-based workspaces, no cloud dependency. A full OMG-conformant repository API would overlap heavily with collaborative repository products and is out of scope for the near term.

At the same time, `semantic_core` and `kernel` already produce rich read projections — validation reports, semantic graphs, visualization DTOs, diagram exports — reused by LSP custom methods, CLI, and MCP. A thin HTTP layer on top of those projections would unlock interoperability without changing Spec42's local-first identity.

## Decision

Add a **read-only HTTP API** to Spec42, exposed as:

```bash
spec42 api serve --workspace-root <path> [--bind 127.0.0.1:3842]
```

### Principles

1. **Read-only** — no element creation, mutation, commits, or branches in phase 1.
2. **File-backed** — the server watches a configured workspace root on disk; no separate model repository.
3. **Engine parity** — every endpoint maps to existing CLI/MCP/LSP logic (`perform_check`, `perform_doctor`, `build_model_summary`, diagram export, …). No second semantic pipeline.
4. **Local by default** — bind to loopback; no authentication in phase 1.
5. **Spec42-native schema first** — OpenAPI describes Spec42 DTOs; OMG REST/HTTP PSM mapping is an explicit later phase, not a blocker for shipping.

### Non-goals (phase 1)

- OMG Systems Modeling API conformance (projects, commits, element UUID CRUD).
- OSLC bindings.
- Multi-tenant or authenticated access.
- Write-back to `.sysml` / `.kerml` files.
- Long-lived subscription / WebSocket push (LSP remains the live editor channel).
- Replacing MCP or CLI for AI/CI workflows.

## Architecture

### Placement

Keep the HTTP server inside `crates/server` (`spec42` crate):

```text
crates/server/src/
  api/
    mod.rs          # router, shared state, error mapping
    handlers.rs     # thin wrappers → perform_* / kernel views
    args.rs         # clap: ApiServeArgs
  cli.rs            # Command::Api { serve }
  lib.rs            # run_api_serve()
```

Rationale:

- Reuses `resolve_environment`, `perform_check`, `perform_doctor`, `build_model_summary`, and diagram export without new crate boundaries.
- Ships in the same release artifact as `spec42` (no extra binary to distribute initially).
- A separate `spec42-api` binary can be split out later if startup time or deployment packaging demands it.

### HTTP stack

Use **axum** on the existing **tokio** runtime:

- Mature ecosystem, good OpenAPI tooling (`utoipa` or hand-written schema).
- Fits async LSP patterns already in the workspace.
- Add `axum` and `tower-http` (CORS, trace, compression) as `spec42` dependencies.

### Server state

```text
ApiServerState {
  environment: ResolvedEnvironment,   # library paths, stdlib, sysand — from resolve_environment
  workspace_root: PathBuf,              # required at startup
  config: Arc<Spec42Config>,            # kernel default + environment overlay
}
```

**Stateless request model (phase 1):** each request re-runs validation/graph build from disk, matching CLI behaviour. This avoids cache invalidation complexity and keeps results consistent with `spec42 check`. Optional in-memory cache with TTL is deferred to phase 2 if profiling shows it is needed.

### Error model

JSON error envelope on all non-2xx responses:

```json
{
  "error": {
    "code": "workspace_not_found",
    "message": "Workspace root does not exist: /path/to/ws"
  }
}
```

Map internal `Result<T, String>` errors to appropriate HTTP status codes (`400` for bad params, `404` for missing paths/elements, `500` for internal failures).

## API surface (v1)

Base URL: `http://127.0.0.1:3842`  
API prefix: `/v1`  
OpenAPI document: `GET /openapi.json` (served from embedded spec; source in [docs/api/spec42-readonly-v1.openapi.yaml](../api/spec42-readonly-v1.openapi.yaml)).

### Operational

| Method | Path | Maps to | Purpose |
| --- | --- | --- | --- |
| `GET` | `/health` | — | Liveness (process up) |
| `GET` | `/ready` | `perform_doctor` (light check) | Readiness (stdlib resolvable, workspace exists) |
| `GET` | `/v1/meta` | build metadata | Spec42 version, API version, workspace root |

### Environment

| Method | Path | Maps to | Purpose |
| --- | --- | --- | --- |
| `GET` | `/v1/doctor` | `perform_doctor` | Library paths, stdlib status, Sysand detection — same JSON as `spec42 doctor --format json` |

### Validation

| Method | Path | Maps to | Purpose |
| --- | --- | --- | --- |
| `POST` | `/v1/validate` | `perform_check` | Validation report for a path relative to workspace root |

Request body:

```json
{
  "path": "models/Drone.sysml",
  "warnings_as_errors": false
}
```

Omit `path` or set `"."` to validate the entire workspace. Response: `ValidationReport` (same shape as `spec42 check --format json`).

### Semantic model

| Method | Path | Maps to | Purpose |
| --- | --- | --- | --- |
| `POST` | `/v1/model/summary` | `perform_model_summary` | Compact graph (capped nodes, filtered relationship kinds) |
| `POST` | `/v1/model/projection` | `perform_check_with_semantics` | Full `SemanticModelProjection` (nodes + all relationships) |

Request body (both):

```json
{
  "path": "models/Drone.sysml",
  "max_nodes": 500
}
```

`max_nodes` applies only to `/model/summary`. Prefer summary for large workspaces; use projection only when the client needs the complete relationship set.

### Element lookup

| Method | Path | Maps to | Purpose |
| --- | --- | --- | --- |
| `GET` | `/v1/elements` | semantic projection + filter | Search by `q` (substring on qualified name), `kind`, `uri` |
| `GET` | `/v1/elements/{qualified_name}` | semantic projection + lookup | Single element by qualified name (URL-encoded) |

Response item shape mirrors `SemanticModelNode` plus outgoing `relationships` for the detail endpoint.

These endpoints are **Spec42-native** — qualified names and file URIs, not OMG element UUIDs.

### Diagnostics

| Method | Path | Maps to | Purpose |
| --- | --- | --- | --- |
| `GET` | `/v1/diagnostics/codes` | diagnostic catalog | List known stable diagnostic codes |
| `GET` | `/v1/diagnostics/explain/{code}` | `perform_explain_diagnostic` | Catalog entry + optional instances |

Query params for explain: `path`, `line` (same semantics as CLI/MCP).

### Diagrams

| Method | Path | Maps to | Purpose |
| --- | --- | --- | --- |
| `POST` | `/v1/diagrams/export` | `diagrams export` | Deterministic SVG or JSON for a shared-renderer view |

Request body:

```json
{
  "path": "models/Drone.sysml",
  "view": "general",
  "format": "svg"
}
```

Response: raw `image/svg+xml` or `application/json` body (not JSON-wrapped) for direct consumption by browsers and CI caches.

### Deferred to phase 2

| Endpoint | Rationale |
| --- | --- |
| `POST /v1/model/graph` | Full `SysmlModelResultDto` (LSP `sysml/model` parity) — heavier payload, needs workspace indexing |
| `POST /v1/visualization` | LSP `sysml/visualization` parity |
| `GET /v1/library/search` | LSP `sysml/librarySearch` parity |
| Response caching / ETag | Add when profiling shows repeated full-workspace builds are a bottleneck |

## Relationship to OMG Systems Modeling API

The OMG REST/HTTP PSM centres on **repository resources**:

```text
/projects /commits /branches /elements/{id} /queries …
```

Spec42 phase 1 intentionally does **not** implement that resource model. Instead it exposes **workspace operations** that external tools can call today against on-disk SysML v2 text.

| OMG concept | Spec42 phase 1 | Future |
| --- | --- | --- |
| Project | `workspace_root` server config | Map to OMG `/projects` wrapper |
| Commit / branch | Current files on disk (implicit HEAD) | Dedicated repository layer |
| Element by UUID | Not available | Requires persistent identity store |
| Element query | `GET /v1/elements?q=…` on semantic projection | Map to OMG query endpoint subset |
| Validation | `POST /v1/validate` | No OMG equivalent (Spec42 extension) |

Phase 2 may add an **`/omg` compatibility prefix** that adapts responses to OMG OpenAPI schemas where a faithful read-only mapping exists (for example element listing → `Element` schema subset). Full conformance remains a separate ADR and likely a host-service concern for write/commit workflows.

## Security

| Topic | Phase 1 choice |
| --- | --- |
| Bind address | Default `127.0.0.1:3842` |
| Authentication | None |
| Authorization | Read-only; no path traversal outside `workspace_root` |
| CORS | Disabled by default; opt-in `--cors-origins` for local dev UIs |
| Rate limiting | None (local tool) |

Document clearly that binding to `0.0.0.0` without auth exposes workspace contents to the network. A `--allow-remote` flag (off by default) gates non-loopback binds.

## CLI

```bash
spec42 api serve \
  --workspace-root ./my-model \
  [--bind 127.0.0.1:3842] \
  [--allow-remote] \
  [--cors-origins http://localhost:5173]
```

Global flags (`--library-path`, `--config`, `--stdlib-path`, `--no-stdlib`) apply the same way as for `check` and LSP.

## Documentation and discovery

- OpenAPI 3.1 spec: [docs/api/spec42-readonly-v1.openapi.yaml](../api/spec42-readonly-v1.openapi.yaml)
- User guide section in README / DEVELOPMENT.md (post-implementation)
- Link from [AI-ASSISTANTS.md](../user/AI-ASSISTANTS.md) as an alternative to MCP for non-agent HTTP clients

## Implementation plan

### Milestone 1 — skeleton (MVP)

- [x] `spec42 api serve` subcommand and `--help`
- [x] axum router: `/health`, `/ready`, `/v1/meta`, `/v1/doctor`
- [x] Integration test: start server on ephemeral port, call `/health`

### Milestone 2 — core read API

- [x] `POST /v1/validate`
- [x] `POST /v1/model/summary`
- [x] `GET /v1/diagnostics/explain/{code}`
- [x] OpenAPI served at `/openapi.json`
- [x] CLI integration tests mirroring `cli_ai_tools.rs` parity pattern

### Milestone 3 — extended read

- [x] `POST /v1/model/projection`
- [x] `GET /v1/elements`, `GET /v1/elements/{qualified_name}`
- [x] `POST /v1/diagrams/export`
- [x] `GET /v1/diagnostics/codes`

### Milestone 4 — hardening

- [x] Path traversal guards
- [x] `--allow-remote` / bind validation
- [x] Performance guardrails doc entry (max_nodes defaults, payload size guidance)
- [x] Update COMPETITIVE-ROADMAP and CONFORMANCE-MATRIX metadata

## Consequences

### Positive

- External tools can consume SysML v2 semantics without LSP or MCP.
- Same validation/graph results as CI (`spec42 check`) — single source of truth.
- Foundation for future OMG read-subset mapping and host-service integration.
- Complements rather than replaces CLI/MCP.

### Negative / trade-offs

- Another surface to maintain and test.
- Stateless re-parse per request may be slow on large workspaces (mitigate with `max_nodes`, path scoping, future cache).
- Spec42-native schema means clients are not OMG-API-compatible until phase 2 adapter.
- No real-time push; clients must poll for changes.

### Risks

| Risk | Mitigation |
| --- | --- |
| Scope creep toward full OMG API | ADR non-goals; separate ADR for repository API |
| Duplicated handler logic | Thin handlers only; shared `perform_*` functions |
| Security misconfiguration | Default loopback; `--allow-remote` gate |

## Alternatives considered

| Alternative | Why not (now) |
| --- | --- |
| Full OMG REST/HTTP PSM from day one | Large effort; needs repository/commits; conflicts with local-first wedge |
| Separate `spec42-api` crate/binary immediately | Extra release complexity; no proven need yet |
| gRPC instead of REST | Worse fit for browser/ops tooling; OpenAPI is OMG direction |
| Extend LSP over TCP | LSP is editor-oriented; custom RPC not standard for integrators |
| MCP over HTTP/SSE | Emerging pattern but not industry standard for SysML interoperability |
| Put API only in a downstream host service | Leaves standalone Spec42 / CI / local workspace users without HTTP access |

## References

- [OMG Systems Modeling API and Services](https://www.omg.org/spec/SystemsModelingAPI/1.0/)
- [SysML-v2-API-Services (reference implementation)](https://github.com/Systems-Modeling/SysML-v2-API-Services)
- [SysML-v2-API-Cookbook](https://github.com/Systems-Modeling/SysML-v2-API-Cookbook)
- [SEMANTIC_CORE_ARCHITECTURE.md](../architecture/SEMANTIC_CORE_ARCHITECTURE.md)
- [AI-ASSISTANTS.md](../user/AI-ASSISTANTS.md)
- [COMPETITIVE-ROADMAP.md](../engineering/COMPETITIVE-ROADMAP.md)
