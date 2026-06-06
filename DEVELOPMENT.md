# Development

Guidance for building, testing, and contributing to Spec42.

## Architecture

Spec42 is a Rust workspace plus a VS Code extension.

- `crates/server` (`spec42`) owns the CLI, LSP binary, MCP binary, read-only HTTP API, environment resolution, and standard-library materialization.
- `crates/kernel` owns the LSP/runtime host: document lifecycle, workspace orchestration, LSP handlers, validation wiring, DTO assembly, and host adapters.
- `crates/semantic_core` owns reusable semantic logic: graph construction, cross-document linking, resolution, evaluation, diagnostics, and graph-first visualization helpers.
- `vscode` owns the VS Code client, webviews, tests, packaging, and bundled asset staging.

Keep reusable semantic/model behavior in `semantic_core`; keep protocol, filesystem runtime, and editor behavior in `kernel` or the host crate that owns it.

## LSP Server Structure

The LSP implementation lives under `crates/kernel/src/lsp_runtime`.

- `capabilities.rs`: capability payload construction
- `documents.rs`: initialize, document lifecycle, workspace/library indexing, and configuration changes
- `diagnostics.rs`: parser/runtime orchestration plus mapping semantic diagnostics into LSP diagnostics
- `features/*`: completion, editing, navigation, symbols, formatting, and related LSP requests
- `custom.rs`: `sysml/*` custom method request logic
- `hierarchy.rs`, `navigation.rs`, `references_resolver.rs`, `symbols.rs`: feature helpers
- `mod.rs`: `tower-lsp` trait entrypoint that delegates to the modules above

Semantic diagnostics rule evaluation is owned by `semantic_core::semantic::diagnostics`; kernel code maps neutral diagnostics at the LSP boundary.

## HTTP API

Read-only workspace access lives in `crates/server/src/api` and ships as:

```bash
spec42 api serve --workspace-root ./my-model
```

- Default bind: `127.0.0.1:3842` (loopback only; use `--allow-remote` for other interfaces).
- Endpoints mirror CLI/MCP (`/v1/validate`, `/v1/model/summary`, `/v1/doctor`, …).
- OpenAPI contract: `docs/api/spec42-readonly-v1.openapi.yaml` (served at `GET /openapi.json`).
- Integration tests: `crates/server/tests/api_http.rs`.

Design rationale: [docs/adr/0001-read-only-systems-modeling-http-api.md](docs/adr/0001-read-only-systems-modeling-http-api.md).

## Building

### Rust server

From the repository root:

```bash
cargo build --release
```

The binary is at `target/release/spec42`. Put it on your `PATH` or set the extension setting `spec42.serverPath` to its path. Legacy `sysml-language-server.*` settings are still read for compatibility.

### Embedded standard library bundle

The `spec42` crate embeds the SysML v2 standard library by default. Builds are deterministic and do not download this archive implicitly.

For normal embedded builds, provide the full SysML v2 Release `2026-03` zip in one of these ways:

```powershell
$env:SPEC42_STDLIB_BUNDLE_ZIP = 'C:\path\to\SysML-v2-Release-2026-03.zip'
cargo build -p spec42
```

or place the zip at:

```text
crates/server/cache/sysml-v2-release-2026-03.zip
```

Maintainers can refresh that cache from:

```text
https://github.com/Systems-Modeling/SysML-v2-Release/archive/refs/tags/2026-03.zip
```

For development checks that do not need the embedded library:

```bash
cargo test --workspace --no-default-features
```

### VS Code extension

```bash
cd vscode
npm install
npm run compile
```

## Parser Dependency Policy

The workspace pins `sysml-v2-parser` in the root `Cargo.toml` as a **crates.io** version (currently `0.18.0`). CI and default local builds resolve from the registry. To work against a sibling checkout, use the `[patch.crates-io]` block in [`.cargo/config.toml`](.cargo/config.toml) (enabled while parser 0.18.0 is developed locally).

When updating parser behavior:

1. Bump the version in root `Cargo.toml` `[workspace.dependencies]` and run `cargo update -p sysml-v2-parser`.
2. Run `cargo test --workspace` with the embedded stdlib bundle available.
3. Run `cargo test --workspace --no-default-features`.
4. Run targeted workspace/indexing checks in `crates/kernel/tests/integration/workspace.rs`.
5. Update docs if parser compatibility or supported workflow expectations changed.

Cross-repo notes for real-model diagnostic quality live in the parser repo: [`docs/CORPUS_MBSE_VACUUM_PARSER_SPEC42_FEEDBACK.md`](../sysml-v2-parser/docs/CORPUS_MBSE_VACUUM_PARSER_SPEC42_FEEDBACK.md).

## Diagnostic quality workflow

- `spec42 check` post-processes diagnostics: deduplication, one root parse error per file (cascades in `relatedInformation`), and suppression of misleading semantic warnings after the first parse error in a file.
- Parser-side cascade suppression and dialect-specific codes come from `sysml-v2-parser`; post-processing lives in `crates/kernel/src/analysis/diagnostics_postprocess.rs`.
- Optional corpus regression: set `MBSE_VACUUM_EXAMPLE_DIR` to a checkout of the public vacuum-cleaner example and run `cargo test -p kernel --test lsp_integration mbse_vacuum -- --ignored`.

## Workspace indexing limits

Large repositories may truncate file discovery per folder pattern. The VS Code setting `spec42.workspace.maxFilesPerPattern` (default in `vscode/package.json`) caps how many `.sysml` / `.kerml` files are indexed per glob pass. When truncation applies, go-to-definition and workspace symbols may be incomplete for files that were not indexed.

- **User docs:** [docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) (increase the cap for large repos).
- **Fixture:** [vscode/testFixture/workspaces/large-workspace](vscode/testFixture/workspaces/large-workspace) sets `maxFilesPerPattern: 2` for manual truncation testing.
- **Integration:** `crates/kernel/tests/integration/workspace.rs` (large-workspace / perf paths pass a higher cap via LSP `initializationOptions`).

## Running Tests

Spec42 uses two Rust integration layers in CI:

| Layer | Scope | Typical runtime |
| --- | --- | --- |
| **Core (fast path)** | Workspace crates except slow `spec42` integration binaries; `spec42` unit tests; `multi_file_check` | Minutes |
| **Agent/API surfaces** | CLI, MCP, and HTTP parity/integration tests on real fixtures | Several minutes (stdlib materialization) |

### Rust (core, fast path)

```bash
cargo test --workspace --exclude spec42
cargo test -p spec42 --lib
cargo test -p spec42 --test multi_file_check
cargo clippy --workspace --all-targets -- -D warnings
```

Full workspace including agent surfaces (local pre-push equivalent of CI):

```bash
cargo test --workspace
```

Without embedded stdlib:

```bash
cargo test --workspace --no-default-features
```

### Rust (agent/API surfaces)

CLI, MCP, and HTTP tests share the same `perform_*` engine and KitchenTimer fixtures. Run them together when changing `crates/server` agent or API code:

```bash
cargo test -p spec42 \
  --test api_http \
  --test mcp_tools \
  --test cli_ai_tools \
  --test mcp_protocol \
  --test mcp_binary \
  --test kitchen_timer_check
```

| Integration test | Surface |
| --- | --- |
| `api_http` | Read-only HTTP API (`spec42 api serve` router) |
| `mcp_tools` | MCP tool handlers (`spec42_check`, `spec42_doctor`, …) |
| `mcp_protocol` | MCP JSON-RPC over in-memory transport |
| `mcp_binary` | `spec42-mcp` stdio binary |
| `cli_ai_tools` | CLI JSON parity with MCP |
| `kitchen_timer_check` | `perform_check` smoke on bundled example |

CI runs core and agent/API layers as separate jobs (see `.github/workflows/ci.yml`).

Focused LSP integration tests:

```bash
cargo test -p kernel --test lsp_integration
```

The LSP integration test modules live under `crates/kernel/tests/integration/`. Use `harness::TestSession` for new tests to avoid duplicated initialize/open/request boilerplate.

Requirements slice checks:

```bash
cargo test -p kernel --test lsp_integration integration::model::lsp_sysml_model_graph_resolves_requirement_usage_typing_same_file
cargo test -p kernel --test lsp_integration integration::model::lsp_sysml_model_graph_resolves_requirement_usage_typing_cross_file
cargo test -p kernel --test lsp_integration integration::diagnostics::unresolved_satisfy_reference_emits_semantic_diagnostic
```

### SysML v2 validation suite

The full validation suite over the official SysML v2 Release is ignored by default and informational in CI. To run it locally:

```bash
git clone --depth 1 https://github.com/Systems-Modeling/SysML-v2-Release.git sysml-v2-release
SYSML_V2_RELEASE_DIR=$PWD/sysml-v2-release cargo test -p kernel --test lsp_integration lsp_workspace_scan_sysml_release -- --nocapture
```

If `SYSML_V2_RELEASE_DIR` is not set or does not contain the expected validation directory, the test returns early without failing.

### VS Code

```bash
cd vscode
npm install
npm run compile
npm test
```

Extension tests run inside a downloaded VS Code instance. Tests that require the language server only assert fully when `spec42` is on `PATH` or `SPEC42_SERVER_PATH` points to the in-repo binary. In CI, the server is built and added to the environment before `npm test`.

Useful focused suites:

```bash
npm run test:state-view
npm run test:interconnection
npm run test:multi-file
npm run test:workspace-smoke
```

### Packaging Checks

```bash
cd vscode
npm run verify:package-layout
npm run package
```

Package staging copies the example and domain-library content into the extension package layout before validation.

## Performance Checks

Spec42 emits structured performance logs when `spec42.performanceLogging.enabled` is true. CI also runs a report-only large-workspace performance step so changes can be tracked before budgets become hard gates.

Current report-only budgets are documented in `docs/PERFORMANCE-GUARDRAILS.md`. Treat regressions there as release-risk signals even while the CI step remains non-blocking.

## AI assistants

**VS Code extension (Copilot Agent):** requires `engines.vscode` **^1.99.0** for Language Model Tools. Four tools in `vscode/package.json` `contributes.languageModelTools` are registered from `vscode/src/lmTools/` and invoke the same `spec42` binary as the LSP (`check`, `doctor`, `explain-diagnostic`, `model-summary` with `--format json`). No extra MCP config in VS Code for these tools.

**MCP and HTTP API:** `spec42-mcp` and `spec42 api serve` expose the same validation and semantic projections as the CLI. Setup: [`docs/AI-ASSISTANTS.md`](docs/AI-ASSISTANTS.md), HTTP design: [`docs/adr/0001-read-only-systems-modeling-http-api.md`](docs/adr/0001-read-only-systems-modeling-http-api.md).

Tests (see [Running Tests](#running-tests) → agent/API surfaces):

```bash
cargo test -p spec42 \
  --test api_http \
  --test mcp_tools \
  --test cli_ai_tools \
  --test mcp_protocol \
  --test mcp_binary \
  --test kitchen_timer_check
cd vscode && npm run compile && npm run test:lm-cli-unit
```

MCP protocol tests use the `rmcp` client dev-dependency with an in-memory duplex transport; `mcp_binary` exercises `spec42-mcp` via stdio; `cli_ai_tools` and `api_http` assert JSON parity with MCP handlers on the KitchenTimer fixture.

## Validation Pipeline

`spec42 check` and MCP `spec42_check` use the same validation engine as the editor host.

Diagnostics are published in two stages:

1. Parser diagnostics from `sysml_v2_parser::parse_with_diagnostics`
2. Semantic diagnostics from `semantic_core` only when parse diagnostics are empty

Semantic diagnostic codes and mapping behavior are covered by focused tests in `crates/kernel/tests/integration/diagnostics.rs`.

## Visualization Checks

Backend visualization payloads are covered by Rust integration tests in `crates/kernel/tests/integration/model.rs` and workspace tests in `workspace.rs`.

Frontend rendering and SVG export checks live under `vscode/src/test/suite`. SVG artifacts are written under the relevant `vscode/testFixture/workspaces/*/test-output/diagrams/` directory and are validated by semantic/layout expectations rather than exact byte-for-byte snapshots.

Action Flow and State Transition views are stable-facing and should remain release-gating. Sequence View remains experimental.

## Examples and domain libraries

Large model content is versioned as Git submodules at the **repository root** only:

- `examples/` → [elan8/sysml-examples](https://github.com/elan8/sysml-examples)
- `domain-libraries/` → [elan8/sysml-domain-libraries](https://github.com/elan8/sysml-domain-libraries)

`vscode/.gitignore` ignores `vscode/examples` and `vscode/domain-libraries` so duplicate checkouts under `vscode/` are not committed. If you see the same example folders twice in the Spec42 **Examples** view, remove the extra copy under `vscode/examples` and keep the root submodule.

The VS Code **Examples** sidebar lists folders from the canonical root `examples/` only (not both `vscode/examples` and `../examples`). Hidden directories such as `.github` are excluded.

## Testing the Extension Manually

1. Build the Rust server: `cargo build` or `cargo build --release`.
2. Open the `vscode/` folder in VS Code.
3. Press F5 to launch the Extension Development Host.
4. Open a folder containing `.sysml` or `.kerml` files.
5. Use the Model Explorer, Visualizer, hover, definition, references, and `spec42 check` to compare editor and CLI behavior.
