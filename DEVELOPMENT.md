# Development

Guidance for building, testing, and contributing to Spec42.

## Architecture

The enduring architecture — which crate is the authority for what, the services consumers work
with, the crate map and the invariants the repository enforces — is [`design.md`](design.md). The
short version:

- `crates/sysml_source` is the source authority: the only crate that reads SysML text from disk,
  admits text as an identified document, or normalises a URI or a line ending.
- `crates/sysml_resolution` is the semantic authority: the only crate that calls the parser, holds
  parsed trees, lowers and resolves, decides diagnostics, computes library closure, and constructs
  and publishes a model. It is the only dependant of the parser and of `sysml_source`.
- `crates/sysml_query` is the facade and the only crate a consumer may name for anything SysML:
  `source`, `syntax`, `library` and `publication` services plus the typed `PublishedModel` queries,
  all obtained from one `Services` value per host process.
- `crates/sysml_diagnostics` owns transport-neutral diagnostic values and the host reporting policy
  over them. It decides nothing semantic.
- `crates/sysml_tokens` projects the facade's syntax token roles onto editor token indices, neutral
  over WASM and LSP hosts.
- `crates/kpar` owns KerML Project Archive (KPAR) read, pack, and validate support.
- `crates/language_service` owns protocol-neutral editor intelligence over typed queries. Hosts map
  its DTOs to LSP, HTTP, or Monaco contracts.
- `crates/library_catalog` owns library provisioning: bundled and managed standard/domain
  libraries, their configuration and data directories, resolved to library roots.
- `crates/workspace` is the batch host: engine, directory snapshots, validation, comparison, schema
  versions.
- `crates/session_actor` is a generic tokio actor over embedder-owned state; it knows nothing about
  SysML and is used by `lsp_server`.
- `crates/lsp_server` owns the LSP/runtime host: document lifecycle, workspace orchestration, LSP
  handlers, validation wiring, DTO assembly, and host adapters.
- `crates/server` (`spec42`) owns the CLI, LSP binary, and thin adapters over `workspace`,
  `library_catalog` and `lsp_server`.
- `generator-plugins` contains the repository-owned Rust WebAssembly generators. Diagram semantics
  enter plugins only through typed queries over the immutable publication.
- `vscode` owns the VS Code client, webviews, tests, packaging, and bundled asset staging. Its
  `diagram-renderer` package owns D3/ELK layout and drawing for generator-produced render products;
  it does not derive model semantics.

New capability lands in this order: implementation in the owning authority, a typed contract in
`sysml_query`, then use from a host. A consumer never computes a SysML answer from source text,
facade data, or names; if the answer is missing, extend the service.

## Language Service Structure

Protocol-neutral editor APIs live in `crates/language_service`.

- `dto.rs`: serde-friendly result types (`SourceLocation`, `HoverResult`, completion/rename/outline DTOs, `TextEditSuggestion`, …) using neutral source spans
- `workspace.rs`: `InMemoryWorkspace` builder and `WorkspaceSnapshot` trait
- `navigation.rs`, `references.rs`, `lookup.rs`, `symbol.rs`: hover, definition, references
- `completion.rs`: context detection, candidate ranking, `complete()`
- `outline.rs`, `workspace_symbols.rs`: document symbols, folding ranges, workspace symbol search
- `rename.rs`, `formatting.rs`, `code_actions.rs`: rename edits, document formatting, neutral quick fixes
- `text.rs`, `keywords.rs`: position/word helpers and keyword hover fallback

`lsp_server::workspace::snapshot` implements `WorkspaceSnapshot` for LSP `ServerState`. Feature modules under `lsp_runtime/features/` delegate to `language_service` and map DTOs to `tower_lsp` types (library-path policy and VS Code commands stay in `lsp_server`).

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

Diagnostic rules are owned by `sysml_resolution` and read through `sysml_query`; `lsp_server` maps neutral diagnostics at the LSP boundary.

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

Performance measurements are local review evidence; shared CI runners are not used as benchmark
gates. See [Performance Checks](#performance-checks) for the reproducible commands.

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

## Authority Pipeline Policy

The workspace has a chain with exactly one dependant per link, and a facade in front of it:

```text
sysml-v2-parser ──► sysml_resolution ──► sysml_query ──► every consumer
source_identity ──► sysml_source ──────┘
```

The parser is pinned in the root `Cargo.toml` as a **git revision**. Four things enforce the
chain, and all of them are meant to stay:

- **`deny.toml`**, checked by `cargo deny check bans` in CI: three `[[bans.deny]]` stanzas ban the
  parser outside `sysml_resolution`, `sysml_resolution` outside `sysml_query`, and `sysml_source`
  outside `sysml_resolution`. This reads the resolved dependency graph (dev-dependencies included),
  so a rename or a transitive path is caught by construction. Only the `bans` check runs; the
  licence and advisory gates are deliberately off.
- **`crates/source_identity/tests/{parser_authority,authority_chain}.rs`**, std-only and below the
  whole chain, cover what the graph cannot see: manifest *shape* (the pin is a bare 40-hex git rev
  with no `version`/`branch`/`tag`/`path`, inherited with `workspace = true`), the `fuzz/` nested
  workspace, both lockfiles, and a repository-local facade under a different package name.
- **`crates/sysml_query/tests/architecture.rs`** pins the exact normal dependency sets of the chain
  crates, the facade, `session_actor`, `library_catalog` and `workspace`, lists every consumer as
  designated, and rejects any parser or graph type in the facade's public API.
- **`crates/sysml_query/tests/syntax_authority.rs`** is the guard for what a dependency graph
  cannot express: a consumer re-answering a syntax question from text. It keeps retired helpers
  deleted, rejects downstream functions that shadow a syntax/library/source query, bans parsed
  trees, strata and AST versions outside the authorities and SysML file reads outside
  `sysml_source`, and flags string probes, comparisons and `matches!` arms against reserved
  keywords, qualified-name fragments, operators and braces. Its exemptions are per file with a
  reason and, where one exists, a predicate; every deferred retirement is recorded in
  `planning/SYNTAX_FOLLOW_UPS.md`.

None of these subsumes another. Run `cargo deny check bans` locally before changing any manifest.

The boundary itself is a compile error, not a convention: a consumer holds a `ParsedSource`
handle whose tree is reachable only inside `sysml_resolution`, so a crate without that dependency
cannot walk, cache, or serialise a parser document even while holding one.

### Updating the parser

1. Change the `rev` in the root `Cargo.toml` `[workspace.dependencies]`, then run
   `cargo update -p 'git+https://github.com/lukewilliamboswell/sysml-v2-parser.git?rev=<old-rev>#sysml-v2-parser@<version>'`.
   The bare `cargo update -p sysml-v2-parser` form is ambiguous whenever more than one identity is
   momentarily resolvable, and it is silent about it -- use the fully qualified spec.
2. `cargo check --workspace --all-targets`, then `cargo test --workspace`. Integration tests live
   in one binary per crate (`tests/integration/main.rs` declaring one module per file): every
   `tests/*.rs` file is its own crate that links the whole stack, and the link step -- not the
   tests -- dominated a rebuild. Add a test as a module of that binary, not as a new file beside it.
3. `cargo snapshot update`, review `git diff -- tests/snapshots`, then `cargo snapshot check`
   (the alias in `.cargo/config.toml` runs the tool in release -- the debug profile is 10-30x
   slower on this workload). The snapshot corpus is the primary end-to-end evidence
   for a parser bump; it is not CI-gated, so the diff review is the gate.
4. Re-verify the entries in `planning/UPSTREAM_PARSER_GAPS.md` against the new revision and remove
   the ones it closes. If the bump changes the keyword vocabulary, update
   `crates/sysml_resolution/src/syntax/keywords.rs` and its pinned count.
5. `cargo tree -i sysml-v2-parser`, `cargo tree -i sysml_resolution` and
   `cargo tree -i sysml_source` must each print a single dependant, and `cargo deny check bans`
   must pass.

To build against a local parser checkout, uncomment the `[patch."https://…"]` block in
`.cargo/config.toml`. The patch key is the git URL -- a `[patch.crates-io]` entry resolves nothing,
because the workspace no longer depends on the published crate. Comment it out again before
producing evidence for a bump; an active patch changes what the graph resolves.

### Retiring a string heuristic

A consumer that answers a SysML question from text is exempt in `syntax_authority.rs` only while
`planning/SYNTAX_FOLLOW_UPS.md` records the typed query that retires it. To retire one: add the
query to the syntax service (implementation in `sysml_resolution::syntax`, contract in
`sysml_query::syntax`), migrate the callers, delete the heuristic, remove its exemption, and
remove its planning entry. Never add an exemption without a planning entry.

## Diagnostic quality workflow

- `spec42 check` post-processes diagnostics: deduplication and one root parse error per file (cascades in `relatedInformation`). By default, semantic checks still run on files with parse errors; use `--strict-diagnostics` for the legacy mode that skips semantic checks after a parse error and suppresses shadowed `unresolved_*` warnings.
- Parser-side cascade suppression and dialect-specific codes come from `sysml-v2-parser`; reporting policy lives in `sysml_diagnostics`.
- Corpus regression: set `MBSE_VACUUM_EXAMPLE_DIR` to a checkout of the public vacuum-cleaner example and run `cargo test -p lsp_server --test lsp_integration mbse_vacuum -- --ignored`.

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
cargo test --workspace --exclude server
cargo test -p server --lib
cargo test -p server --test integration multi_file_check
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
cargo test -p server --test integration cli_ai_tools -- --include-ignored
cargo test -p server --test integration kitchen_timer_check -- --include-ignored
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

Spec42 emits structured performance logs when `spec42.performanceLogging.enabled` is true.
Performance measurements are gathered locally on controlled hardware and reviewed with the change;
they are not CI gates.

### Query benchmarks

A representation change is admitted with a benchmark showing it neutral-or-better on the bundled standard-library corpus. The six cases are the cold build, the warm relink one keystroke costs, and the four keystroke-path queries:

```
cargo bench -p spec42-query-bench
cargo run --release -p spec42-query-bench --bin spec42-query-bench-allocations -- \
  --output /tmp/spec42-query-allocations.json
```

The first reports wall time and divan's allocation profile per case. The second writes a versioned
JSON report with allocation counts, bytes, per-element normalization, a dependency-complete corpus
digest, build/toolchain/host metadata, and the exact measurement configuration (the two cannot
share a binary, because each installs its own `#[global_allocator]`). Keep the before/after JSON
artifacts with the change under review; they are structured review evidence, not CI thresholds.

`tools/semantic_benchmark` remains the JSON-reporting cold-build tool with a phase breakdown. Its
report retains raw samples, percentile summaries, corpus identity, configuration, and environment
metadata. The divan set remains the admission benchmark; none of these host timings are CI gates.

## AI assistants

**VS Code extension (Copilot Agent):** requires `engines.vscode` **^1.99.0** for Language Model Tools. Four tools in `vscode/package.json` `contributes.languageModelTools` are registered from `vscode/src/lmTools/` and invoke the same `spec42` binary as the LSP (`check`, `doctor`, `explain-diagnostic`, `model-summary` with `--format json`).

**Other AI hosts (Copilot, Cursor, …):** use the CLI directly plus a per-host skill/instructions doc. Setup: [`docs/user/AI-ASSISTANTS.md`](docs/user/AI-ASSISTANTS.md).

Tests (see [Running Tests](#running-tests) → agent CLI surfaces):

```bash
cargo test -p server --test integration cli_ai_tools -- --include-ignored
cargo test -p server --test integration kitchen_timer_check -- --include-ignored
cd vscode && npm run compile && npm run test:lm-cli-unit
```

`cli_ai_tools` asserts CLI JSON output for `explain-diagnostic` / `model-summary` on the KitchenTimer fixture.

## Validation Pipeline

`spec42 check` uses the same validation engine as the editor host.

Diagnostics are published in two stages:

1. Parser diagnostics, carried by the `ParsedSource` the syntax service returns for a document
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
dependencies, packages and installs the VSIX beneath the repository-local, gitignored
`.cache/vscode-qa-<ABI token>` directory, creates a compact state-transition QA workspace, and opens
it with isolated extension and user-data directories. The durable profile retains workspace trust
and QA-specific settings across rebuilds. Pass `--no-open` to prepare the environment without
launching VS Code. Set `SPEC42_VSCODE_QA_DIR` to use a different state directory, including a
throwaway directory when testing profile initialization.

For the lighter Extension Development Host workflow:

1. Build the Rust server: `cargo build` or `cargo build --release`.
2. Open the `vscode/` folder in VS Code.
3. Press F5 to launch the Extension Development Host.
4. Open a folder containing `.sysml` or `.kerml` files.
5. Use Feature Inspector, hover, definition, references, and `spec42 check` to compare editor and CLI behavior.
