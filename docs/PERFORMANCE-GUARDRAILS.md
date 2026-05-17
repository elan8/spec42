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
