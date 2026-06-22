# Development

Guidance for building, testing, and contributing to Spec42.

## Architecture

Spec42 is a Rust workspace plus a VS Code extension.

- `crates/spec42_host` owns the host embedding API: library catalog resolution, engine builder, and immutable snapshot construction (Phase 2).
- `crates/server` (`spec42`) owns the CLI, LSP binary, MCP binary, read-only HTTP API, and thin adapters over `spec42_host`.
- `crates/kernel` owns the LSP/runtime host: document lifecycle, workspace orchestration, LSP handlers, validation wiring, DTO assembly, and host adapters.
- `crates/language_service` owns protocol-neutral editor intelligence: navigation, completion, document outline/folding, workspace symbol search, rename, formatting, and neutral quick-fix edits. Hosts map its DTOs to LSP, HTTP, or Monaco contracts.
- `crates/semantic_core` owns reusable semantic logic: graph construction, cross-document linking, resolution, evaluation, diagnostics, and graph-first visualization helpers.
- `vscode` owns the VS Code client, webviews, tests, packaging, and bundled asset staging.

Keep reusable semantic/model behavior in `semantic_core`; keep editor intelligence that is shared across hosts in `language_service`; keep protocol, filesystem runtime, and editor-specific behavior in `kernel` or the host crate that owns it.

## Language Service Structure

Protocol-neutral editor APIs live in `crates/language_service`.

- `dto.rs`: serde-friendly result types (`SourceLocation`, `HoverResult`, completion/rename/outline DTOs, `TextEditSuggestion`, …) using `semantic_core` spans
- `workspace.rs`: `InMemoryWorkspace` builder and `WorkspaceSnapshot` trait
- `navigation.rs`, `references.rs`, `lookup.rs`, `symbol.rs`: hover, definition, references
- `completion.rs`: context detection, candidate ranking, `complete()`
- `outline.rs`, `workspace_symbols.rs`: document symbols, folding ranges, workspace symbol search
- `rename.rs`, `formatting.rs`, `code_actions.rs`: rename edits, document formatting, neutral quick fixes
- `text.rs`, `keywords.rs`: position/word helpers and keyword hover fallback

`kernel::workspace::snapshot` implements `WorkspaceSnapshot` for LSP `ServerState`. Kernel feature modules under `lsp_runtime/features/` delegate to `language_service` and map DTOs to `tower_lsp` types (library-path policy and VS Code commands stay in kernel).

Headless tests: `crates/language_service/tests/` (`navigation/`, `completion/`, `outline/`, `inmemory_workspace`, `dto_roundtrip`, `dependency_guardrails`).

Design rationale: [docs/adr/0002-language-service-crate.md](docs/adr/0002-language-service-crate.md).

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

The binary is at `target/release/spec42` (Windows: `target/release/spec42.exe`). Put it on your `PATH` or set the extension setting `spec42.serverPath` to its path. Legacy `sysml-language-server.*` settings are still read for compatibility.

**F5 performance:** Launch Extension uses `target/debug/spec42.exe` by default, which is 3–5× slower on visualization and IBD work. For day-to-day extension development on large workspaces (e.g. power systems), point `spec42.serverPath` at the release binary after `cargo build --release`:

```json
"spec42.serverPath": "c:\\Git\\spec42\\target\\release\\spec42.exe"
```

See [docs/engineering/POWER-SYSTEMS-PERFORMANCE-ANALYSIS.md](docs/engineering/POWER-SYSTEMS-PERFORMANCE-ANALYSIS.md) for measured impact.

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
```

Refresh the OMG stdlib archives with:

```bash
bash scripts/fetch-stdlib-bundle.sh
bash scripts/fetch-domain-libraries-bundle.sh
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

The workspace pins `sysml-v2-parser` in the root `Cargo.toml` as a **crates.io** version (currently **0.25.4**). CI and default local builds resolve from the registry. To test against a sibling checkout before publish, uncomment the `[patch.crates-io]` block in [`.cargo/config.toml`](.cargo/config.toml).

When updating parser behavior:

1. Bump the version in root `Cargo.toml` `[workspace.dependencies]` and run `cargo update -p sysml-v2-parser`.
2. Run `cargo test --workspace` with the embedded stdlib bundle available.
3. Run `cargo test --workspace --no-default-features`.
4. Run targeted workspace/indexing checks in `crates/kernel/tests/integration/workspace.rs`.
5. Update docs if parser compatibility or supported workflow expectations changed.

Cross-repo notes for real-model diagnostic quality live in the parser repo: [`docs/CORPUS_MBSE_VACUUM_PARSER_SPEC42_FEEDBACK.md`](../sysml-v2-parser/docs/CORPUS_MBSE_VACUUM_PARSER_SPEC42_FEEDBACK.md).

## Diagnostic quality workflow

- `spec42 check` post-processes diagnostics: deduplication and one root parse error per file (cascades in `relatedInformation`). By default, semantic checks still run on files with parse errors; use `--strict-diagnostics` for the legacy mode that skips semantic checks after a parse error and suppresses shadowed `unresolved_*` warnings.
- Parser-side cascade suppression and dialect-specific codes come from `sysml-v2-parser`; post-processing lives in `crates/kernel/src/analysis/diagnostics_postprocess.rs`.
- Corpus regression: set `MBSE_VACUUM_EXAMPLE_DIR` to a checkout of the public vacuum-cleaner example and run `cargo test -p kernel --test lsp_integration mbse_vacuum -- --ignored`. See [docs/engineering/MBSE-VACUUM-CHECK-ANALYSIS.md](docs/engineering/MBSE-VACUUM-CHECK-ANALYSIS.md).

## Workspace indexing limits

Large repositories may truncate file discovery per folder pattern. The VS Code setting `spec42.workspace.maxFilesPerPattern` (default in `vscode/package.json`) caps how many `.sysml` / `.kerml` files are indexed per glob pass. When truncation applies, go-to-definition and workspace symbols may be incomplete for files that were not indexed.

- **User docs:** [docs/user/TROUBLESHOOTING.md](docs/user/TROUBLESHOOTING.md) (increase the cap for large repos).
- **Fixture:** [vscode/testFixture/workspaces/large-workspace](vscode/testFixture/workspaces/large-workspace) sets `maxFilesPerPattern: 2` for manual truncation testing.
- **Integration:** `crates/kernel/tests/integration/workspace.rs` (large-workspace / perf paths pass a higher cap via LSP `initializationOptions`).

## Visualization normalization parity

Authoritative semantic shaping for diagram payloads lives in `semantic_core` (`crates/semantic_core/src/semantic/visualization/payload.rs`) before LSP/CLI serialization. The shared renderer's `normalizeVisualizationPayload` is a thin pass-through (aliases + candidate arrays only).

| Field / behavior | Authoritative source |
|------------------|---------------------|
| `interconnectionScene` | `interconnection_scene.rs` |
| `stateMachines` (labels, sort, filter) | `finalize_state_machines_for_response` |
| `activityDiagrams` (renderability, rank, flow IDs) | `finalize_activity_diagrams_for_response` |
| `sequenceDiagrams` (filter, rank, labels) | `finalize_sequence_diagrams_for_response` |
| Scoped IBD URI set (interconnection LSP) | `IbdBuildScope::ViewExposedPackages` + `ibd_uri_closure_for_exposed_ids` |
| Scoped vs full IBD scene parity | `crates/semantic_core/tests/scoped_ibd_parity.rs` (CI on `examples/drone`) |
| Slim interconnection LSP payload (`ibd` omitted) | `VisualizationBuildOptions::slim_interconnection_payload`; tested in `interconnection_visualization.rs` |
| `viewCandidates` | `explicit_views::build_view_candidates` |

See [docs/architecture/PREPARE-PIPELINE-OVERLAP.md](docs/architecture/PREPARE-PIPELINE-OVERLAP.md).

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

CLI, MCP, and HTTP tests share the same `perform_*` engine and KitchenTimer fixtures. They are **`#[ignore]` by default** so plain `cargo test` stays fast; run them with `--include-ignored` when changing `crates/server` agent or API code:

```bash
cargo test -p spec42 \
  --test api_http \
  --test mcp_tools \
  --test cli_ai_tools \
  --test mcp_protocol \
  --test mcp_binary \
  --test kitchen_timer_check \
  -- --include-ignored
```

| Integration test | Surface |
| --- | --- |
| `api_http` | Read-only HTTP API (`spec42 api serve` router) |
| `mcp_tools` | MCP tool handlers (`spec42_check`, `spec42_doctor`, …) |
| `mcp_protocol` | MCP JSON-RPC over in-memory transport |
| `mcp_binary` | `spec42-mcp` stdio binary |
| `cli_ai_tools` | CLI JSON parity with MCP |
| `kitchen_timer_check` | `perform_check` smoke on bundled example |
| `kpar_stdlib_embed_smoke` | Embedded OMG KPAR stdlib resolves `ScalarValues::Real` |
| `multi_file_check` | Multi-file workspace import smoke |

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

### Robot vacuum showcase (local only)

The [sysml-robot-vacuum-cleaner](https://github.com/elan8/sysml-robot-vacuum-cleaner) model is vendored under `third_party/` for local integration tests. The directory is gitignored; fetch it once with:

```bash
bash scripts/fetch-robot-vacuum-cleaner.sh
```

Pinned version: `config/robot-vacuum-cleaner.json`. Override the checkout path with `SYSML_ROBOT_VACUUM_DIR` if needed.

These tests stay `#[ignore]` in CI (slow; ~1–2 minutes). Run locally with `--ignored`:

```bash
cargo test -p spec42_host --test robot_vacuum_snapshot -- --ignored --nocapture
cargo test -p kernel --test lsp_integration robot_vacuum -- --ignored --nocapture
```

### Incremental update benchmark (local only)

Compare full in-memory rebuild vs. single-document `update_snapshot` on a synthetic multi-file workspace:

```bash
cargo test -p spec42_host --test incremental_benchmark -- --ignored --nocapture
```

The benchmark stays `#[ignore]` in CI. Enable `experimental_incremental_updates(true)` on the engine builder before measuring incremental timings.

### Robot vacuum performance analysis (local only)

Profile the **embedding host** cold path (`load_workspace` + `prepare_view`) on the vendored robot-vacuum fixture. Requires the checkout from `scripts/fetch-robot-vacuum-cleaner.sh`.

```bash
# Single release report → target/spec42-perf/robot-vacuum-host-phases.json
cargo test -p spec42_host --test robot_vacuum_performance \
  robot_vacuum_host_phase_performance_report --release -- --ignored --nocapture

# Median matrix (3 scenarios × 3 runs) → target/spec42-perf/robot-vacuum-host-matrix.json
cargo test -p spec42_host --test robot_vacuum_performance \
  robot_vacuum_host_performance_matrix_report --release -- --ignored --nocapture

# Profiling example (profiling profile = release + debuginfo)
cargo build -p spec42_host --profile profiling --example profile_robot_vacuum
target/profiling/examples/profile_robot_vacuum --embedded-libs
```

CPU flamegraphs need `kernel.perf_event_paranoid <= 1` (see [ROBOT-VACUUM-PERFORMANCE-ANALYSIS.md](docs/engineering/ROBOT-VACUUM-PERFORMANCE-ANALYSIS.md)).

The perf harness uses `ValidationTiming::Deferred` (view-first embedding). Release regression ceilings are enforced in `robot_vacuum_host_phase_performance_report` via `release_perf_thresholds()` (load ≤ 3 s, prepare ≤ 2.5 s, total ≤ 5.5 s). Use a **release** or **profiling** binary for IDE integration — debug builds are ~5.8× slower on the same path (see analysis doc).

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
npm run test:state-view          # state-transition-view integration + SVG export
npm run test:interconnection     # interconnection-view integration
npm run test:interconnection-drone
npm run test:multi-file          # multi-file workspace smoke
npm run test:workspace-smoke
npm run test:ux-unit             # status bar, snippets, examples provider, panel controller, update flow
npm run test:library-unit        # library status view model
```

Default `npm test` (`.vscode-test.mjs`) runs extension smoke, visualization export, placeholder/empty-state tests, and message handler tests. Controller, update-flow, gate, dtoAdapter, and render-tracker unit tests run via `npm run test:ux-unit`. Shared payload normalization tests live in `shared/diagram-renderer/src/prepare/normalize-payload.test.ts`. CI `vscode-smoke` also runs the view integration suites above.

### VS Code smoke troubleshooting

- **`Server process exited with code 0` in the SysML output channel** is normal during an intentional `sysml.restartServer` stop. It does not by itself indicate failure.
- Look instead for **`restartServer failed`**, **`extension server crashed`**, or a **`waitFor` timeout** in the test host console (`[spec42-test][...]` lines when `SPEC42_TEST_DEBUG=1`).
- CI sets `SPEC42_SERVER_PATH` for smoke runs; test fixtures should not hardcode machine-specific `spec42.serverPath` values.
- If a visualization integration test times out, check whether the language server reached `ready` before the visualizer opened (`configureServerForTests` logs when debug is enabled).

### Packaging Checks

```bash
cd vscode
npm run verify:package-layout
npm run package
```

Package staging copies the example and domain-library content into the extension package layout before validation.

## Performance Checks

Spec42 emits structured performance logs when `spec42.performanceLogging.enabled` is true. CI also runs a report-only large-workspace performance step so changes can be tracked before budgets become hard gates.

Current report-only budgets are documented in `docs/engineering/PERFORMANCE-GUARDRAILS.md`. Treat regressions there as release-risk signals while the nightly step remains non-blocking.

## AI assistants

**VS Code extension (Copilot Agent):** requires `engines.vscode` **^1.99.0** for Language Model Tools. Four tools in `vscode/package.json` `contributes.languageModelTools` are registered from `vscode/src/lmTools/` and invoke the same `spec42` binary as the LSP (`check`, `doctor`, `explain-diagnostic`, `model-summary` with `--format json`). No extra MCP config in VS Code for these tools.

**MCP and HTTP API:** `spec42-mcp` and `spec42 api serve` expose the same validation and semantic projections as the CLI. Setup: [`docs/user/AI-ASSISTANTS.md`](docs/user/AI-ASSISTANTS.md), HTTP design: [`docs/adr/0001-read-only-systems-modeling-http-api.md`](docs/adr/0001-read-only-systems-modeling-http-api.md).

Tests (see [Running Tests](#running-tests) → agent/API surfaces):

```bash
cargo test -p spec42 \
  --test api_http \
  --test mcp_tools \
  --test cli_ai_tools \
  --test mcp_protocol \
  --test mcp_binary \
  --test kitchen_timer_check \
  -- --include-ignored
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

Example workspaces are versioned as a Git submodule at the **repository root**:

- `examples/` → [elan8/sysml-examples](https://github.com/elan8/sysml-examples)

Elan8 domain libraries are **bundled inside the Spec42 server binary** as a **KPAR** (KerML Project Archive). The pinned version lives in `config/domain-libraries.json`. CI fetches or packs the archive with `scripts/fetch-domain-libraries-bundle.sh` before building.

The OMG standard library is bundled from `sysml.library.kpar` at the pinned SysML v2 Release tag (see `config/standard-library.json`). CI fetches only that directory via sparse git checkout in `scripts/fetch-stdlib-bundle.sh` before building.

For local development, `build.rs` prefers, in order:

1. `SPEC42_DOMAIN_LIBRARIES_BUNDLE_ZIP` (path to `.kpar`, CI/release)
2. `SPEC42_DOMAIN_LIBRARIES_SOURCE_DIR` (pack on the fly with `kpar-pack`)
3. A sibling checkout at `../sysml-domain-libraries` (packed to KPAR when no cached bundle exists)
4. Cached `.cache/elan8-domain-libraries-{version}.kpar`

Domain library releases are published from [elan8/sysml-domain-libraries](https://github.com/elan8/sysml-domain-libraries) via the `release-kpar` GitHub Action when a `v*` tag is pushed. Pack locally with:

```bash
cargo run -p kpar --bin kpar-pack -- --root ../sysml-domain-libraries --version 0.1.0 --output elan8-domain-libraries-0.1.0.kpar
```

`vscode/.gitignore` ignores `vscode/examples` so duplicate checkouts under `vscode/` are not committed. If you see the same example folders twice in the Spec42 **Examples** view, remove the extra copy under `vscode/examples` and keep the root submodule.

The VS Code **Examples** sidebar lists folders from the canonical root `examples/` only (not both `vscode/examples` and `../examples`). Hidden directories such as `.github` are excluded.

## Testing the Extension Manually

1. Build the Rust server: `cargo build` or `cargo build --release`.
2. Open the `vscode/` folder in VS Code.
3. Press F5 to launch the Extension Development Host.
4. Open a folder containing `.sysml` or `.kerml` files.
5. Use the Model Explorer, Visualizer, hover, definition, references, and `spec42 check` to compare editor and CLI behavior.
