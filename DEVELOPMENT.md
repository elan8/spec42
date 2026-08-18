# Development

Guidance for building, testing, and contributing to Spec42.

## Architecture

Spec42 is a Rust workspace plus a VS Code extension.

- `crates/sysml_resolution` owns semantic construction, resolution, and every diagnostic a host reports, settled at the publication barrier; `crates/sysml_query` is the typed read-only facade over it.
- `crates/sysml_diagnostics` owns transport-neutral diagnostic values and the host reporting policy over them. It decides nothing semantic and depends only on `sysml_query`.
- `crates/sysml_tokens` owns SysML v2 semantic tokenization for editor highlighting, neutral over WASM and LSP hosts.
- `generator-plugins` contains the repository-owned Rust WebAssembly generators. Diagram
  semantics enter plugins only through typed queries over the immutable publication.
- `crates/kpar` owns KerML Project Archive (KPAR) read, pack, and validate support.
- `crates/language_service` owns protocol-neutral editor intelligence: navigation, completion, document outline/folding, workspace symbol search, rename, formatting, and neutral quick-fix edits. Hosts map its DTOs to LSP, HTTP, or Monaco contracts.
- `crates/workspace` owns the host embedding API: library catalog resolution, engine building, snapshot construction/comparison, and workspace session lifecycle.
- `crates/workspace_session` owns a protocol-neutral, tokio-actor concurrency wrapper (lock-free reads, superseded-rebuild handling) over embedder-owned session state; currently a standalone scaffold not yet wired into a host.
- `crates/lsp_server` owns the LSP/runtime host: document lifecycle, workspace orchestration, LSP handlers, validation wiring, DTO assembly, and host adapters.
- `crates/server` (`spec42`) owns the CLI, LSP binary, and thin adapters over `workspace` and `lsp_server`.
- `vscode` owns the VS Code client, webviews, tests, packaging, and bundled asset staging.
  Its `diagram-renderer` package owns D3/ELK layout and drawing for generator-produced render
  products; it does not derive model semantics.

Keep semantic construction and diagnostic rules in `sysml_resolution`; all consumers use the typed
`sysml_query` facade. Keep editor intelligence that is shared across hosts in `language_service`;
keep protocol, filesystem runtime, and editor-specific behavior in `lsp_server` or the host crate
that owns it.

## Language Service Structure

Protocol-neutral editor APIs live in `crates/language_service`.

- `dto.rs`: serde-friendly result types (`SourceLocation`, `HoverResult`, completion/rename/outline DTOs, `TextEditSuggestion`, …) using neutral source spans
- `workspace.rs`: `InMemoryWorkspace` builder and `WorkspaceSnapshot` trait
- `navigation.rs`, `references.rs`, `lookup.rs`, `symbol.rs`: hover, definition, references
- `completion.rs`: context detection, candidate ranking, `complete()`
- `outline.rs`, `workspace_symbols.rs`: document symbols, folding ranges, workspace symbol search
- `rename.rs`, `formatting.rs`, `code_actions.rs`: rename edits, document formatting, neutral quick fixes
- `text.rs`, `keywords.rs`: position/word helpers and keyword hover fallback

`kernel::workspace::snapshot` implements `WorkspaceSnapshot` for LSP `ServerState`. Kernel feature modules under `lsp_runtime/features/` delegate to `language_service` and map DTOs to `tower_lsp` types (library-path policy and VS Code commands stay in kernel).

Headless tests: `crates/language_service/tests/` (`navigation/`, `completion/`, `outline/`, `inmemory_workspace`, `dto_roundtrip`, `dependency_guardrails`).

Protocol-neutral editor intelligence stays in the language-service crate; hosts map its DTOs to LSP/HTTP/Monaco.

## LSP Server Structure

The LSP implementation lives under `crates/lsp_server/src/lsp_runtime`.

- `capabilities.rs`: capability payload construction
- `documents.rs`: initialize, document lifecycle, workspace/library indexing, and configuration changes
- `diagnostics.rs`: parser/runtime orchestration plus mapping semantic diagnostics into LSP diagnostics
- `features/*`: completion, editing, navigation, symbols, formatting, and related LSP requests
- `custom.rs`: `sysml/*` custom method request logic
- `hierarchy.rs`, `navigation.rs`, `references_resolver.rs`, `symbols.rs`: feature helpers
- `mod.rs`: `tower-lsp` trait entrypoint that delegates to the modules above

Diagnostic rules are owned by `sysml_resolution` and read through `sysml_query`; kernel code maps neutral diagnostics at the LSP boundary.

## Building

### Rust server

From the repository root:

```bash
cargo build --release
```

The binary is at `target/release/spec42` (Windows: `target/release/spec42.exe`). Put it on your `PATH` or set the extension setting `spec42.serverPath` to its path. Legacy `sysml-language-server.*` settings are still read for compatibility.

**F5 performance:** Launch Extension uses `target/debug/spec42.exe` by default, which is 3–5× slower on visualization and IBD work. For day-to-day extension development on large workspaces (e.g. power systems), point `spec42.serverPath` at the release binary after `cargo build --release`:

```json
"spec42.serverPath": "c:\\Git\\spec42\\target\\release\\spec42.exe"
```

See [docs/engineering/PERFORMANCE-GUARDRAILS.md](docs/engineering/PERFORMANCE-GUARDRAILS.md) for nightly budgets and optional power-systems drill-down.

### Embedded standard library bundle

The `spec42` crate embeds the SysML v2 standard library by default. Builds are deterministic and do not download KPAR archives implicitly.

The pinned standard-library release is defined once in `config/standard-library.json`.
After changing that file, run:

```bash
node scripts/sync-standard-library-config.mjs
```

For normal embedded builds, fetch or place KPAR archives under the unified local cache:

```text
.cache/
  sysml-stdlib-kpar-<version>/     # OMG .kpar files (one per library)
  elan8-domain-libraries-<version>.kpar
  elan8-method-libraries-<version>.kpar
```

Refresh the OMG stdlib and managed library archives with:

```bash
bash scripts/fetch-stdlib-bundle.sh
bash scripts/fetch-kpar-libraries-bundle.sh
```

Optional override for a custom stdlib cache directory:

```powershell
$env:SPEC42_STDLIB_KPAR_DIR = 'C:\path\to\sysml-stdlib-kpar-2026-04'
cargo build -p spec42
```

The stdlib fetch script sparse-checkouts `sysml.library.kpar/` at the pinned release tag (not `master`).
Use the `version` and `repo` values from `config/standard-library.json`.

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

The workspace pins `sysml-v2-parser` in the root `Cargo.toml` as a **crates.io** version (currently **0.47.0**). CI and default local builds resolve from the registry. To test against a sibling checkout before publish, uncomment the `[patch.crates-io]` block in [`.cargo/config.toml`](.cargo/config.toml).

When updating parser behavior:

1. Bump the version in root `Cargo.toml` `[workspace.dependencies]` and run `cargo update -p sysml-v2-parser`.
2. Run `cargo test --workspace` with the embedded stdlib bundle available.
3. Run `cargo test --workspace --no-default-features`.
4. Run targeted workspace/indexing checks in `crates/lsp_server/tests/integration/workspace.rs`.
5. Update docs if parser compatibility or supported workflow expectations changed.

Cross-repo notes for real-model diagnostic quality live in the parser repo: [`docs/CORPUS_MBSE_VACUUM_PARSER_SPEC42_FEEDBACK.md`](../sysml-v2-parser/docs/CORPUS_MBSE_VACUUM_PARSER_SPEC42_FEEDBACK.md).

## Diagnostic quality workflow

- `spec42 check` post-processes diagnostics: deduplication and one root parse error per file (cascades in `relatedInformation`). By default, semantic checks still run on files with parse errors; use `--strict-diagnostics` for the legacy mode that skips semantic checks after a parse error and suppresses shadowed `unresolved_*` warnings.
- Parser-side cascade suppression and dialect-specific codes come from `sysml-v2-parser`; reporting policy lives in `sysml_diagnostics`.
- Corpus regression: set `MBSE_VACUUM_EXAMPLE_DIR` to a checkout of the public vacuum-cleaner example and run `cargo test -p kernel --test lsp_integration mbse_vacuum -- --ignored`.

## Workspace indexing limits

Large repositories may truncate file discovery per folder pattern. The VS Code setting `spec42.workspace.maxFilesPerPattern` (default in `vscode/package.json`) caps how many `.sysml` / `.kerml` files are indexed per glob pass. When truncation applies, go-to-definition and workspace symbols may be incomplete for files that were not indexed.

- **User docs:** [docs/user/TROUBLESHOOTING.md](docs/user/TROUBLESHOOTING.md) (increase the cap for large repos).
- **Fixture:** [vscode/testFixture/workspaces/large-workspace](vscode/testFixture/workspaces/large-workspace) sets `maxFilesPerPattern: 2` for manual truncation testing.
- **Integration:** `crates/lsp_server/tests/integration/workspace.rs` (large-workspace / perf paths pass a higher cap via LSP `initializationOptions`).

## Running Tests

Spec42 uses two Rust integration layers in CI:

| Layer | Scope | Typical runtime |
| --- | --- | --- |
| **Core (fast path)** | Workspace crates except slow `spec42` integration binaries; `spec42` unit tests; `multi_file_check` | Minutes |
| **Agent CLI surfaces** | CLI agent-tool integration tests on real fixtures | Several minutes (stdlib materialization) |

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

### Rust (agent CLI surfaces)

CLI agent-tool tests share the same `perform_*` engine and KitchenTimer fixtures. They are **`#[ignore]` by default** so plain `cargo test` stays fast; run them with `--include-ignored` when changing `crates/server` agent CLI code:

```bash
cargo test -p spec42 \
  --test cli_ai_tools \
  --test kitchen_timer_check \
  -- --include-ignored
```

| Integration test | Surface |
| --- | --- |
| `cli_ai_tools` | CLI JSON output for `explain-diagnostic` / `model-summary` |
| `kitchen_timer_check` | `perform_check` smoke on bundled example |
| `kpar_stdlib_embed_smoke` | Embedded OMG KPAR stdlib resolves `ScalarValues::Real` |
| `multi_file_check` | Multi-file workspace import smoke |

CI runs core and agent CLI layers as separate jobs (see `.github/workflows/ci.yml`).

Focused LSP integration tests:

```bash
cargo test -p lsp_server --test lsp_integration
```

The LSP integration test modules live under `crates/lsp_server/tests/integration/`. Use
`harness::TestSession` for new tests to avoid duplicated initialize/open/request boilerplate.

### SysML v2 validation suite

The full validation suite over the official SysML v2 Release is ignored by default and informational in CI. To run it locally:

```bash
git clone --depth 1 https://github.com/Systems-Modeling/SysML-v2-Release.git sysml-v2-release
SYSML_V2_RELEASE_DIR=$PWD/sysml-v2-release cargo test -p lsp_server --test lsp_integration lsp_workspace_scan_sysml_release -- --nocapture
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
npm run test:multi-file          # multi-file workspace smoke
npm run test:ux-unit             # status bar, snippets, and examples provider
npm run test:library-unit        # library status view model
```

Default `npm test` (`.vscode-test.mjs`) runs the extension smoke and editor integration suites.

### VS Code smoke troubleshooting

- **`Server process exited with code 0` in the SysML output channel** is normal during an intentional `sysml.restartServer` stop. It does not by itself indicate failure.
- Look instead for **`restartServer failed`**, **`extension server crashed`**, or a **`waitFor` timeout** in the test host console (`[spec42-test][...]` lines when `SPEC42_TEST_DEBUG=1`).
- CI sets `SPEC42_SERVER_PATH` for smoke runs; test fixtures should not hardcode machine-specific `spec42.serverPath` values.

### Packaging Checks

```bash
cd vscode
npm run package
```

The package prepublish hook stages the example and domain-library content before compiling the extension.

## Performance Checks

Spec42 emits structured performance logs when `spec42.performanceLogging.enabled` is true. CI also runs a report-only large-workspace performance step so changes can be tracked before budgets become hard gates.

Current report-only budgets are documented in `docs/engineering/PERFORMANCE-GUARDRAILS.md`. Treat regressions there as release-risk signals while the nightly step remains non-blocking.

## AI assistants

**VS Code extension (Copilot Agent):** requires `engines.vscode` **^1.99.0** for Language Model Tools. Four tools in `vscode/package.json` `contributes.languageModelTools` are registered from `vscode/src/lmTools/` and invoke the same `spec42` binary as the LSP (`check`, `doctor`, `explain-diagnostic`, `model-summary` with `--format json`).

**Other AI hosts (Copilot, Cursor, …):** use the CLI directly plus a per-host skill/instructions doc. Setup: [`docs/user/AI-ASSISTANTS.md`](docs/user/AI-ASSISTANTS.md).

Tests (see [Running Tests](#running-tests) → agent CLI surfaces):

```bash
cargo test -p spec42 \
  --test cli_ai_tools \
  --test kitchen_timer_check \
  -- --include-ignored
cd vscode && npm run compile && npm run test:lm-cli-unit
```

`cli_ai_tools` asserts CLI JSON output for `explain-diagnostic` / `model-summary` on the KitchenTimer fixture.

## Validation Pipeline

`spec42 check` uses the same validation engine as the editor host.

Diagnostics are published in two stages:

1. Parser diagnostics from `sysml_v2_parser::parse_with_diagnostics`
2. Semantic diagnostics published by `sysml_resolution`, adapted through `sysml_diagnostics`

Semantic diagnostic codes and mapping behavior are covered by focused tests in
`crates/lsp_server/tests/integration/diagnostics.rs`.

## Examples and domain libraries

Example workspaces are versioned as a Git submodule at the **repository root**:

- `examples/` → [elan8/sysml-examples](https://github.com/elan8/sysml-examples)

Elan8 domain and method libraries are **bundled inside the Spec42 server binary** as separate **KPAR** archives. Pins live in [`config/libraries/`](config/libraries) (`domain.json`, `method.json`). CI fetches or packs them with `scripts/fetch-kpar-libraries-bundle.sh` before building.

The OMG standard library is bundled from `sysml.library.kpar` at the pinned SysML v2 Release tag (see `config/standard-library.json`). CI fetches only that directory via sparse git checkout in `scripts/fetch-stdlib-bundle.sh` before building.

For local development, `build.rs` prefers, in order (per managed library):

1. `SPEC42_KPAR_LIBRARY_BUNDLE_<ID>` (path to `.kpar`, CI/release; legacy `SPEC42_DOMAIN_LIBRARIES_BUNDLE_ZIP` still maps to domain)
2. `SPEC42_KPAR_LIBRARY_SOURCE_DIR_<ID>` (pack on the fly with `spec42 bundle`; the source root must contain its authoritative `.project.json`)
3. A sibling checkout from `pack.siblingRelative` in the library config (packed when no cached bundle exists)
4. Cached `.cache/<artifact>` from the library pin

Domain library releases are published from [elan8/sysml-domain-libraries](https://github.com/elan8/sysml-domain-libraries) via the `release-kpar` GitHub Action when a `v*` tag is pushed. Pack locally with:

```bash
cargo run -p server --no-default-features --bin spec42 -- bundle ../sysml-domain-libraries -o elan8-domain-libraries-0.3.0.kpar
```

`vscode/.gitignore` ignores `vscode/examples` so duplicate checkouts under `vscode/` are not committed. If you see the same example folders twice in the Spec42 **Examples** view, remove the extra copy under `vscode/examples` and keep the root submodule.

The VS Code **Examples** sidebar lists folders from the canonical root `examples/` only (not both `vscode/examples` and `../examples`). Hidden directories such as `.github` are excluded.

## Testing the Extension Manually

On macOS, prepare a packaged, isolated QA installation with:

```bash
scripts/setup-macos-vscode-qa.sh
```

The script builds the repository generator plugins and embedded-stdlib server, installs locked npm
dependencies, packages and installs the VSIX beneath `/tmp/spec42-vscode-qa`, creates a compact
state-transition QA workspace, and opens it with isolated extension and user-data directories. Pass `--no-open` to
prepare the environment without launching VS Code. Set `SPEC42_VSCODE_QA_DIR` to use another state
directory.

For the lighter Extension Development Host workflow:

1. Build the Rust server: `cargo build` or `cargo build --release`.
2. Open the `vscode/` folder in VS Code.
3. Press F5 to launch the Extension Development Host.
4. Open a folder containing `.sysml` or `.kerml` files.
5. Use Feature Inspector, hover, definition, references, and `spec42 check` to compare editor and CLI behavior.
