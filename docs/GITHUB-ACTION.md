# GitHub Action

Spec42 ships a composite GitHub Action for validating SysML v2 and KerML models in CI.
The Action downloads the matching release binary for the runner platform, optionally runs `spec42 doctor`, and then runs `spec42 check`.

## Basic Usage

```yaml
name: Spec42

on:
  pull_request:
  push:
    branches: [main]

permissions:
  contents: read
  security-events: write

jobs:
  validate-models:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6

      - uses: elan8/spec42@vX.Y.Z
        with:
          path: examples/timer/KitchenTimer.sysml
          format: sarif
          warnings-as-errors: true
```

When `format` is `sarif`, the Action writes `spec42.sarif`, uploads it to GitHub Code Scanning by default, and still fails the job if `spec42 check` reports validation errors.

## Inputs

| Input | Default | Description |
| --- | --- | --- |
| `version` | Action ref | Spec42 release tag to install, for example `v0.26.0`. |
| `path` | required | File or workspace path to validate. |
| `workspace-root` | | Optional workspace root passed to `spec42 check`. |
| `format` | `sarif` | Output format: `text`, `json`, `sarif`, or `junit`. |
| `warnings-as-errors` | `false` | Treat warnings as errors. |
| `baseline` | | Optional baseline file passed to `spec42 check`. |
| `config` | | Optional config file passed as `--config`. |
| `library-path` | | Optional additional library root passed as `--library-path`. |
| `stdlib-path` | | Optional standard library root passed as `--stdlib-path`. |
| `no-stdlib` | `false` | Disable bundled standard library resolution. |
| `run-doctor` | `true` | Run `spec42 doctor` before validation. |
| `upload-sarif` | `true` | Upload SARIF to GitHub Code Scanning when `format` is `sarif`. |
| `sarif-file` | `spec42.sarif` | SARIF output file path. |

## Installing Only

The Action exposes the installed binary path as `executable-path`, so workflows can run additional CLI commands after validation.

```yaml
- id: spec42
  uses: elan8/spec42@vX.Y.Z
  with:
    path: examples/office
    format: text
    upload-sarif: false

- run: |
    "${{ steps.spec42.outputs.executable-path }}" diagrams export examples/office \
      --view general-view \
      --format svg \
      --output target/diagrams
```

## Publishing Notes

The Action is available when this repository has a GitHub release/tag that contains the platform archives produced by `.github/workflows/release.yml`.
Use a version tag that includes this Action, such as the next Spec42 release tag, in downstream workflows so the Action code and downloaded CLI release stay aligned.
