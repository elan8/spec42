# Development

Guidance for building, testing, and contributing to Spec42.

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
- **CI**: When sysml-parser is used as a standalone repo, its own CI (sysml-parser/.github/workflows/ci.yml) clones the release and runs `cargo test -- --include-ignored`. When sysml-parser is part of this workspace, run the validation locally (see below).
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
