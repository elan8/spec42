# Nightly Performance Guardrails

Spec42 tracks large-workspace and drone-interconnection performance in nightly CI. Budget violations fail the nightly job. These are nightly gates only — they do not block PRs.

## Reported Scenarios

### Large workspace (nightly CI)

The report uses `vscode/testFixture/workspaces/large-workspace` and measures:

- workspace indexing/model assembly through `sysml/model` with `["workspaceVisualization"]`
- `sysml/model` with `["graph", "workspaceVisualization"]`
- `sysml/visualization` for `general-view`

The nightly step prints one `SPEC42_PERF_REPORT` JSON line, writes `target/spec42-perf/large-workspace-performance.json`, uploads both the raw log and JSON report as artifacts, and appends a compact bottleneck summary to the GitHub step summary.

### Power systems drill-down (local, optional)

For a full regional grid fixture workspace and `systemContext` diagram, set `SYSML_POWERSYSTEMS_DIR` to an external checkout (spec42 does not ship or assume a default path):

```powershell
$env:SYSML_POWERSYSTEMS_DIR = "C:\path\to\grid-fixture"
cargo test -p lsp_server --test lsp_integration integration::powersystems_performance::powersystems_system_context_performance_report -- --ignored --nocapture
```

Output: `target/spec42-perf/grid-system-context-performance.json`.

### Interconnection smoke (nightly CI, in-repo, budget-enforced)

The nightly job runs the drone example smoke test (no external fixture) and checks its budgets:

```bash
cargo test -p lsp_server --test lsp_integration integration::powersystems_performance::drone_interconnection_performance_smoke_report -- --nocapture
node scripts/check-perf-budgets.mjs target/spec42-perf/drone-interconnection-performance.json
```

Output: `target/spec42-perf/drone-interconnection-performance.json` (includes `scopedIbdPerUriMs`, scoped URI counts, slim-payload sizes, and embedded budgets).

Budget violations exit non-zero and fail the nightly job. Budgets are embedded in the Rust test that emits the report; update them there if the underlying performance improves or the model grows.

Optional grid drill-down in nightly CI: set repository variable `SYSML_POWERSYSTEMS_DIR` to a checkout path on the runner (or mount via self-hosted runner).

See [POWER-SYSTEMS-PERFORMANCE-ANALYSIS.md](./POWER-SYSTEMS-PERFORMANCE-ANALYSIS.md) for findings and improvement plan.

See [ROBOT-VACUUM-PERFORMANCE-ANALYSIS.md](./ROBOT-VACUUM-PERFORMANCE-ANALYSIS.md) for embedding-host cold-path profiling on the robot-vacuum showcase fixture.

## Enforced Budgets

Budgets are embedded in the Rust test that emits each report (not in this document). The values below reflect the current embedded budgets.

| Fixture | Metric | Budget |
| --- | --- | ---: |
| large-workspace | workspace model request | 5 000 ms |
| large-workspace | document model request | 2 500 ms |
| large-workspace | visualization request | 1 500 ms |
| drone-interconnection | workspace model request | 5 000 ms |
| drone-interconnection | cold headless visualization build | 1 500 ms |
| drone-interconnection | warm visualization cache hit | 1 500 ms |

These are nightly gates. Budget violations fail the nightly `large-workspace-performance` job via `scripts/check-perf-budgets.mjs`. To promote to PR gates, move the relevant perf test into `ci.yml`.

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
