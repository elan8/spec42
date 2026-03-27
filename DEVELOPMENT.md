# Development

Guidance for building, testing, and contributing to Spec42.

## LSP Server Structure

The LSP implementation is progressively being split by concern to reduce `lsp_server.rs` churn and make feature work easier to isolate.

- `spec42-core/src/lsp/types.rs`: shared server/index state types
- `spec42-core/src/lsp/indexing.rs`: workspace/library scan and index update helpers
- `spec42-core/src/lsp/capabilities.rs`: capability payload construction
- `spec42-core/src/lsp/lifecycle.rs`: initialize/scan root helpers
- `spec42-core/src/lsp/navigation.rs`: document-link and selection-range helpers
- `spec42-core/src/lsp/symbols.rs`: inlay/code-lens helpers
- `spec42-core/src/lsp/hierarchy.rs`: moniker/type/call hierarchy item builders
- `spec42-core/src/lsp/custom.rs`: `sysml/*` custom method logic

`spec42-core/src/lsp_server.rs` remains the trait entrypoint and delegates to the modules above.

Note on capability compatibility:
- Type hierarchy handlers are implemented and exercised by integration tests.
- With the current `tower-lsp` surface, type hierarchy capability is advertised via `ServerCapabilities.experimental.typeHierarchyProvider`.

## Building

### Rust server

From the repository root:

```bash
cargo build --release
```

The binary is at `target/release/spec42`. Put it on your PATH or set the extension setting `spec42.serverPath` to its path (legacy `sysml-language-server.serverPath` is still supported).

### VS Code extension

```bash
cd vscode
npm install
npm run compile
```

## Validation tests (SysML v2 suite)

The parser runs a full validation suite over all `.sysml` files in the official [SysML v2 Release](https://github.com/Systems-Modeling/SysML-v2-Release) `sysml/src/validation` directory. The test expects zero parser errors.

- **Standard `cargo test`**: The full validation suite is `#[ignore]`d (slow). It does not run by default.
- **CI fast path (required)**: `.github/workflows/ci.yml` runs `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -W clippy::all`.
- **CI full validation (informational)**: `.github/workflows/full-validation.yml` runs on PR, schedule, and manual dispatch with `continue-on-error: true`, cloning SysML-v2-Release and executing `lsp_workspace_scan_sysml_release`.
- **Locally**: The sysml-parser crate has a sysml-v2-release submodule. Initialize it and run the ignored tests:

  ```bash
  cd sysml-parser && git submodule update --init sysml-v2-release
  cargo test -p sysml-parser -- test_full_validation_suite -- --ignored
  ```

  Or set `SYSML_V2_RELEASE_DIR` to a SysML-v2-Release clone root. If the validation directory is not present, the test returns early without failing.

## Running tests

### Rust (parser + server)

```bash
cargo test
```

This runs workspace tests including sysml-parser unit/validation tests and Spec42 LSP integration tests.

## Semantic diagnostics pipeline

`spec42-core/src/lsp_server.rs` publishes diagnostics in two stages:

1. Parser diagnostics from `sysml_parser::parse_with_diagnostics` (source `sysml`)
2. Semantic diagnostics from configured providers (source `semantic`) only when parse diagnostics are empty

Default semantic checks are implemented in `spec42-core/src/semantic_checks.rs`.

Current built-in semantic diagnostic codes include:

- `connection_endpoint_not_port`
- `port_type_mismatch`
- `unconnected_port`
- `duplicate_connection`
- `invalid_multiplicity`
- `unresolved_type_reference`
- `invalid_redefines_reference` (emitted when `redefines` metadata is available in the semantic graph)

### LSP integration test organization

Integration tests live under `spec42-core/tests/integration/` and are now split by domain:

- Core feature files (`hover`, `completion`, `definition`, `references`, `rename`, etc.)
- Experimental feature surface checks (`experimental_capabilities.rs`, `experimental_requests.rs`)
- Reliability gates for newer handlers (`quality_gates.rs`)
- SysML model graph-focused tests (`model_graph.rs`) plus broader model coverage in `model.rs`

Use `harness::TestSession` for new integration tests to reduce duplicated initialize/open/request boilerplate.

To run only LSP integration tests:

```bash
cargo test -p spec42 --test lsp_integration
```

Optional: set `SYSML_V2_RELEASE_DIR` to run `lsp_workspace_scan_sysml_release`, which indexes the SysML-v2-Release clone and asserts workspace/symbol returns results.

### Layout SVG checks

Backend-rendered diagram SVGs for the full drone fixture are written to `server/tests/output/` during Rust integration tests.

The tests validate semantic expectations instead of exact SVG equality. Use the observed SVGs for manual layout review while iterating on `elk-rust` and diagram generation.

`elk-rust` is vendored as the `vendor/elk-rust` submodule. When you update or patch it, commit the change in the submodule and then commit the updated submodule pointer in this repository.

### VS Code extension tests

```bash
cd vscode
npm install
npm run compile
npm test
```

Extension tests run inside a downloaded VS Code instance. Running them from the CLI is only supported when no other VS Code instance is running. Tests that require the language server (hover, go-to-definition) only assert when `spec42` is on PATH. In CI, the server is built and added to PATH before `npm test`.

## Testing the extension (F5)

1. Build the Rust server: `cargo build` (debug) or `cargo build --release`.
2. Open the `vscode/` folder in VS Code.
3. Press F5 to launch the Extension Development Host.
4. In the new window, open a folder and create a `.sysml` or `.kerml` file. The language server should activate and show diagnostics.
