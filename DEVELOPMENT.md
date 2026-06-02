# Development

Guidance for building, testing, and contributing to Spec42.

## Architecture

Spec42 is a Rust workspace plus a VS Code extension.

- `crates/server` (`spec42`) owns the CLI, LSP binary, MCP binary, environment resolution, and standard-library materialization.
- `crates/kernel` owns the LSP/runtime host: document lifecycle, workspace orchestration, LSP handlers, validation wiring, DTO assembly, and host adapters.
- `crates/semantic_core` owns reusable semantic logic: graph construction, cross-document linking, resolution, evaluation, diagnostics, and graph-first visualization helpers.
- `crates/software-architecture` owns the optional software-architecture analysis/RPC provider.
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

The workspace pins `sysml-v2-parser` in the root `Cargo.toml` as a git dependency on `https://github.com/elan8/sysml-v2-parser` with an explicit tag (currently `v0.14.0`). When updating parser behavior:

1. Bump the `tag` in root `Cargo.toml` and run `cargo update -p sysml-v2-parser`.
2. Run `cargo test --workspace` with the embedded stdlib bundle available.
3. Run `cargo test --workspace --no-default-features`.
4. Run targeted workspace/indexing checks in `crates/kernel/tests/integration/workspace.rs`.
5. Update docs if parser compatibility or supported workflow expectations changed.

## Running Tests

### Rust

```bash
cargo test --workspace
cargo test --workspace --no-default-features
cargo clippy --workspace --all-targets -- -D warnings
```

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

## Testing the Extension Manually

1. Build the Rust server: `cargo build` or `cargo build --release`.
2. Open the `vscode/` folder in VS Code.
3. Press F5 to launch the Extension Development Host.
4. Open a folder containing `.sysml` or `.kerml` files.
5. Use the Model Explorer, Visualizer, hover, definition, references, and `spec42 check` to compare editor and CLI behavior.
