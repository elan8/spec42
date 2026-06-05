# Spec42 HTTP API

Read-only HTTP access to SysML v2 workspace semantics. Design rationale and scope: [ADR 0001](../adr/0001-read-only-systems-modeling-http-api.md).

## Status

**Implemented** as `spec42 api serve` (read-only, local-first). Contract source: [spec42-readonly-v1.openapi.yaml](spec42-readonly-v1.openapi.yaml).

## OpenAPI

| File | Description |
| --- | --- |
| [spec42-readonly-v1.openapi.yaml](spec42-readonly-v1.openapi.yaml) | Phase 1 read-only API (Spec42-native schema) |

When implemented, the running server serves this document at `GET /openapi.json`.

## Quick start

```bash
spec42 api serve --workspace-root ./my-model
curl http://127.0.0.1:3842/health
curl http://127.0.0.1:3842/v1/doctor
curl -X POST http://127.0.0.1:3842/v1/validate \
  -H "Content-Type: application/json" \
  -d '{"path":"."}'
```

## Related surfaces

| Surface | When to use |
| --- | --- |
| **HTTP API** | External apps, dashboards, non-Rust/non-agent integrations |
| **CLI** | CI, scripts, deterministic one-shot queries |
| **MCP** | AI coding assistants |
| **LSP** | Live editor feedback |
