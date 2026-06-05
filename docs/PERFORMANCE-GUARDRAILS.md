# Performance Guardrails

Spec42 tracks large-workspace performance as a release-readiness signal. The CI guardrail is intentionally report-only while baselines settle; it records metrics but does not fail the build.

## Reported Scenario

The report uses `vscode/testFixture/workspaces/large-workspace` and measures:

- workspace indexing/model assembly through `sysml/model` with `["workspaceVisualization"]`
- `sysml/model` with `["graph", "workspaceVisualization"]`
- `sysml/visualization` for `general-view`

The CI step prints one `SPEC42_PERF_REPORT` JSON line and appends it to the GitHub step summary.

## Initial Budgets

These budgets are not enforced yet:

| Metric | Report-only budget |
| --- | ---: |
| Workspace indexing | 5000 ms |
| `sysml/model` | 2500 ms |
| `sysml/visualization` | 1500 ms |

Treat repeated regressions above these numbers as release risks. Once the team accepts stable baselines across CI runners, convert the report-only step into a hard gate.

## Debounced workspace diagnostics

After `textDocument/didOpen` and `textDocument/didChange`, the kernel still publishes diagnostics for the edited document immediately, then schedules a **450ms debounced** republish of all project files in the server index (excluding configured library paths). That pass is O(project files) and keeps cross-file semantic diagnostics consistent when imports or references change in another file.

Large workspaces should rely on `spec42.workspace.maxFilesPerPattern` for Model Explorer discovery limits; the debounced diagnostic republish uses whatever the language server has already indexed (opened files plus startup workspace scan).

## Read-only HTTP API

`spec42 api serve` is **stateless**: each request re-parses and re-validates from disk (same engine as `spec42 check`). There is no in-memory cache in phase 1.

| Endpoint | Guidance |
| --- | --- |
| `POST /v1/model/summary` | Prefer over `/v1/model/projection` for large workspaces; default `max_nodes` is 500 |
| `POST /v1/model/projection` | Full semantic graph; can be large — scope `path` to a file or small directory |
| `GET /v1/elements` | Default `limit` 100, max 5000; always runs a full projection for the scoped `path` |
| `POST /v1/diagrams/export` | ELK-backed SVG for routed views; comparable cost to `spec42 diagrams export` |

For automation over large trees, prefer scoped paths, `model/summary` with a modest `max_nodes`, or the CLI/MCP surfaces when a one-shot subprocess is acceptable.
