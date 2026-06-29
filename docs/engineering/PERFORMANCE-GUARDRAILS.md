# Nightly Performance Guardrails

Spec42 tracks large-workspace performance as a release-readiness signal. The nightly performance job is intentionally report-only while baselines settle; it records metrics but does not fail pull requests.

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

### Interconnection smoke (nightly CI, in-repo)

The nightly job runs the drone example smoke test (no external fixture):

```bash
cargo test -p lsp_server --test lsp_integration integration::powersystems_performance::drone_interconnection_performance_smoke_report -- --nocapture
```

Output: `target/spec42-perf/drone-interconnection-performance.json` (includes `scopedIbdPerUriMs`, scoped URI counts, and slim-payload sizes).

Optional grid drill-down in nightly CI: set repository variable `SYSML_POWERSYSTEMS_DIR` to a checkout path on the runner (or mount via self-hosted runner).

See [POWER-SYSTEMS-PERFORMANCE-ANALYSIS.md](./POWER-SYSTEMS-PERFORMANCE-ANALYSIS.md) for findings and improvement plan.

See [ROBOT-VACUUM-PERFORMANCE-ANALYSIS.md](./ROBOT-VACUUM-PERFORMANCE-ANALYSIS.md) for embedding-host cold-path profiling on the robot-vacuum showcase fixture.

## Initial Budgets

These budgets are not enforced yet:

| Metric | Report-only budget |
| --- | ---: |
| Workspace indexing | 5000 ms |
| `sysml/model` | 2500 ms |
| `sysml/visualization` | 1500 ms |

Treat repeated regressions above these numbers as release risks. Once the team accepts stable baselines across nightly runners, consider promoting specific budgets into PR gates.

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
