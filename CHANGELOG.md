# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.21.0] - 2026-04-24

### Added

- **Sequence View rendering pipeline** - Added end-to-end sequence-diagram extraction/projection/rendering support in the backend and VS Code visualizer, including lifelines, messages, activations, and fragment handling.
- **Software Architecture add-on workflows** - Added software workspace analysis/project-view APIs plus VS Code add-on UX for running analysis and opening dedicated software architecture visualizations.
- **Expanded domain library coverage** - Added broad new domain-library content across software, electronics, communication, and robotics layers with structured rule sets and updated documentation.
- **Webshop end-to-end examples** - Added a richer webshop sample set (`architecture`, `behavior`, `requirements`, `views`) to validate explicit-view projections and cross-file visualization behavior.

### Changed

- **Parser/runtime alignment to `sysml-v2-parser` 0.7.0** - Updated parser dependency and adapted semantic-model/view extraction paths to newer AST behavior.
- **Visualizer panel/update lifecycle hardening** - Refined panel restore/update timing, content hashing, and refresh flow across startup, restore, and workspace contexts.
- **Workspace visualization model shaping** - Improved explicit-view projection, model parameter parsing, and package/container grouping behavior for more consistent rendered diagrams.

### Fixed

- **Webshop visualization regressions** - Fixed empty/missing structure/interconnection/state/action results in cross-file explicit-view scenarios (including projection and IBD scope fallback behavior).
- **`satisfy` diagnostic false positives** - Fixed typed-member-chain resolution and diagnostic suppression for references like `instance.member` across documents.
- **Action Flow UX issues** - Fixed edge-label behavior for structural flow markers and improved click-to-source reliability/disambiguation.
- **Startup empty visualizer race** - Fixed restore/startup timing cases where the visualizer pane could stay empty with no listed views until manual reopen.

## [0.20.0] - 2026-04-21

### Added

- **Expression evaluation in semantic model** - Added a new evaluation module with unit-aware expression handling and integration coverage for referenced attributes and unit expression behavior.
- **Inlay hints support** - Added inlay-hint capability wiring in the language server surface and related test coverage.
- **Expanded visualization data surface** - Added a dedicated SysML visualization endpoint and richer DTO/model payload support for container groups and connector endpoint relationships.

### Changed

- **Parser and semantic alignment** - Upgraded `sysml-v2-parser` (through `0.6.0`) and updated semantic projection paths for part definitions/usages, activity flows, and state-machine extraction.
- **Visualizer rendering/UX architecture** - Refined view state handling, text measurement/truncation, rendering flow, and loading behavior to improve responsiveness and diagram readability.
- **Action/State view release defaults** - Promoted Action Flow and State Transition views to default-enabled visualizer views and removed experimental labeling/toggle configuration from the extension surface.
- **Workspace and explorer interaction cleanup** - Simplified model-explorer/selection synchronization paths and tightened visualizer payload handling across workspace scenarios.

### Fixed

- **Hover/type resolution robustness** - Improved type resolution behavior in hover and related semantic lookup paths for more reliable editor feedback.

## [0.19.0] - 2026-04-16

### Changed

- **`sysml-v2-parser` upgrade and host alignment** - Updated Spec42 to the newer `sysml-v2-parser` releases through `0.4.0`, aligned semantic/expression handling with the newer AST shapes, and added a shared `spec42_core::sysml_v2` re-export so hosts can use the pinned parser surface consistently.
- **Semantic graph and endpoint resolution** - Expanded semantic graph construction and member resolution around package bodies, interface definitions, part usages, and connection endpoint lookup so port-like members and local references resolve more reliably.

### Fixed

- **Port compatibility diagnostics** - Replaced raw port type-name comparison with SysML v2 style feature-compatibility checks, reducing false `port_type_mismatch` warnings when differently named ports are directionally and structurally compatible.
- **Semantic diagnostic false positives** - Reduced noisy diagnostics around `redefines`, forward-resolvable `satisfy` references, delegated/redefined ports, and syntax-only missing-semicolon guesses so valid models are less likely to receive misleading warnings.

## [0.18.1] - 2026-04-10

### Fixed

- **General View workspace loading hang** - Fixed an infinite traversal in General View scene construction that could leave the visualizer stuck on "Parsing SysML model" for larger workspace visualizations such as `apollo-11-sysml-v2`.
- **Workspace package visibility in VS Code visualizer** - Fixed workspace General View package selection so the visualizer uses workspace-scoped model metadata instead of collapsing back to the currently open file, restoring visibility of packages outside the active document.

## [0.18.0] - 2026-04-10

### Added

- **Bundled SysML standard library** - The official `sysml.library` tree from the SysML v2 Release is repacked at build time, embedded in the `spec42` binary, and materialized into the spec42 data directory when needed (metadata records `repo: "embedded"`).
- **Default library paths in LSP** - `Spec42Config` carries default library search paths from the resolved environment; the server merges host defaults with paths from `initialize` / `didChangeConfiguration` so clients can extend discovery without replacing host layout.
- **`spec42 stdlib` CLI** - Subcommands to show status, print the resolved path, and `clear-cache` to remove materialized standard-library files (re-created from the embedded copy on next use). Legacy `stdlib install` / remove flows were removed in favor of the bundled workflow.
- **Import scope checks** - The semantic model applies import-scope rules so diagnostics and analysis respect SysML v2 import visibility beyond earlier membership/import resolution.

### Changed

- **Standard library resolution order** - After explicit flags, env, config, and a valid managed install under the data directory, resolution prefers **materializing the embedded archive** before falling back to the legacy VS Code `globalStorage` path, so upgrades pick the bundled release without manual cleanup.
- **VS Code Library view** - The extension no longer downloads or manages the standard library from the UI; the Library view shows the bundled release inline and defers materialization to the server.
- **`sysml-v2-parser` upgrade** - Bumped the parser dependency and adjusted semantic graph, token range, and graph-builder paths (including state machines, part definitions/usages, requirement bodies, package bodies) for compatibility with newer AST shapes and Apollo-oriented parsing behavior.
- **Document symbols** - Normalized document symbol extraction in the semantic layer for more consistent outlines and navigation.
- **Semantic graph and import resolution** - Expanded import-resolution and relationship handling (including cross-document typing tests) and refreshed validation/diagnostics integration.

## [0.17.0] - 2026-04-09

### Added

- **Visible parser declaration support** - Added first-class support for `FeatureDecl` and `ClassifierDecl` so these package-level declarations now appear in document symbols, named-element extraction, semantic graph output, feature inspector responses, and General View diagrams.

### Changed

- **Published parser dependency** - Switched Spec42 from the old git-pinned parser dependency to the published `sysml-v2-parser` crate `0.1.0`.
- **Parser compatibility handling** - Updated semantic graph, symbol, and diagnostic/token code paths to tolerate newly introduced parser AST variants without breaking existing behavior.

## [0.16.0] - 2026-04-08

### Added

- **SysML v2 import semantics in semantic resolution** - Added shared import-membership resolution for membership imports, namespace imports, `public` re-export chains, and recursive `::**` imports across semantic linking.

### Changed

- **Import-aware editor navigation** - Updated hover and go-to-definition to resolve symbols through spec-valid import chains instead of relying only on direct local typing edges or symbol-table heuristics.
- **Import-aware semantic diagnostics** - Updated unresolved-type diagnostics to respect SysML v2 import visibility rules so valid `public import` chains resolve cleanly while private-only chains continue to warn.
- **Semantic-model backend architecture** - Introduced a reusable import-resolution layer with semantic-graph caching so future import-aware features can share one standards-aligned backend path.

## [0.15.1] - 2026-04-07

### Fixed

- **Imported library type resolution** - Resolved cross-document typing for unqualified type references introduced via `import` declarations, including KerML modeled declarations from library sources.
- **Loose-file diagnostics after library refresh** - Rebuilt semantic links for non-library documents after initialization/configuration library scans so files outside workspace roots stop reporting stale unresolved-type diagnostics.

## [0.15.0] - 2026-04-07

### Added

- **Initial Zed extension** - Added a Zed extension for SysML v2 with Tree-sitter-based language support, LSP wiring to `spec42`, and automatic download of the matching `spec42` release binary when no configured or `PATH` binary is available.
- **Zed CI and release packaging** - Added CI coverage for building the Zed extension and release packaging that publishes a Zed extension source bundle alongside the existing server archives and VS Code extension artifact.

### Changed

- **Explorer-first sidebar UX** - Upgraded the Model Explorer with richer semantic summaries/tooltips and retired the separate Feature Inspector view from the VS Code extension surface.

## [0.14.0] - 2026-04-02

### Added

- **Visualizer canvas interactions** - Added richer visualizer canvas handling, including zoom/pan improvements for diagram exploration.

### Changed

- **Frontend visualization architecture cleanup** - Removed legacy sequence view/debug-label paths, tightened enabled-view handling, and refined visualizer initialization/render flow.
- **Documentation alignment** - Updated root/development/extension docs to reflect frontend-rendered diagrams and current Spec42 feature/configuration status.
- **Configuration surface reduction** - Removed unused visualization settings and semantic-token dump setting from the VS Code extension contribution surface.
- **Model Explorer navigation UX** - Updated tree selection navigation to better reuse existing editors when opening source locations.

## [0.13.0] - 2026-04-01

### Added

- **State transition visualization** - Added a new state-transition view for visualizing state machines with states and transitions, including SVG export and dedicated test fixtures.
- **Parsing performance benchmarking** - Added Criterion benchmarks (including a `parse_scan` benchmark) and documented how to run them.

### Changed

- **SysML parser + semantic model enhancements** - Updated the `sysml-v2-parser` dependency and expanded action-definition handling in the semantic model (including parameters, perform steps, and richer connection extraction for activity diagrams).
- **Indexing performance** - Improved startup and workspace indexing performance, including graph lookup caching and additional indexing/stdlib parsing optimizations.
- **Repository hygiene** - Removed outdated output/reference files that no longer reflect the current project structure.

### Fixed

- **Visualizer UI** - Disabled the layout-direction button for the state-transition view to avoid presenting a non-applicable control.

## [0.12.0] - 2026-03-31

### Changed

- **CI reliability** - Removed integration tests that depended on an external `C:\Git\sysml-examples\office\office.sysml` fixture path.
- **VS Code extension tests** - Removed a brittle interconnection SVG route-analysis test that depended on webview render/export timing and exact SVG geometry.

## [0.11.0] - 2026-03-30

### Added

- **Requirements relationship visibility controls** - Added `subject` relationship support in the semantic graph and General View, plus a General View toggle to show/hide requirement-related nodes and edges (`subject`, `satisfy`, `verify`) with default ON behavior.
- **Workspace visualization coverage** - Added/expanded integration coverage for multi-file workspace visualization flows and all-package graph expectations.
- **Production troubleshooting switch** - Added `spec42.logging.verbose` so production installs stay quieter by default while enabling deeper runtime logs on demand.

### Changed

- **spec42-core modularization (technical debt reduction)** - Split large hotspot modules into smaller focused units without changing external behavior:
  - `semantic_model/mod.rs` extracted hover/signature and symbol-entry logic.
  - `sysml_model.rs` extracted request-param parsing and graph projection helpers.
  - `semantic_checks.rs` extracted rule helper functions.
  - `lsp_server.rs` began service/helper extraction for qualified symbol lookup flows.
  - `semantic_model/graph_builder.rs` extracted requirement-subject edge helper logic.
- **General + Interconnection visualization data shaping** - Improved workspace/model graph handling (including synthetic-node stripping and root/selection behavior) to better align rendered views with real package/model structure.
- **Startup and indexing performance** - Improved startup traceability and introduced parallel parsing for scanned entries to reduce indexing bottlenecks in larger workspaces.
- **Library UX updates** - Refined standard/custom library management UX and model-explorer package statistics behavior.

### Fixed

- **Click-to-source regression in General View** - Restored reliable node click behavior and source navigation in workspace visualization scenarios.
- **Diagnostic quality improvements** - Reduced invalid semantic diagnostics around declared type references and built-in type handling (for example `String`), with better diagnostic anchoring.
- **Visualizer command icon consistency** - Updated the visualizer entry icon to `open-preview` for clearer VS Code affordance.

## [0.10.0] - 2026-03-27

### Changed

- **macOS Apple Silicon startup compatibility** - Added `darwin-arm64` server artifact build and VSIX packaging so the language server starts natively on Apple Silicon without architecture mismatch spawn failures.
- **Release packaging validation** - Updated package layout verification to require bundled `darwin-arm64` server binaries in both staged layout checks and VSIX content checks.

## [0.9.1] - 2026-03-27

### Added

- **Untyped part quick fix** - Added a Quick Fix for untyped part usage (for example `part display;`) that can create a matching `part def Display { }` and rewrite usage to `part display : Display;`.
- **Quick-fix diagnostics coverage** - Added unit and integration tests for the new untyped-part diagnostics and code-action flow.

### Changed

- **Parser update** - Updated `sysml-v2-parser` and aligned parser references across `spec42-core` and `server`.
- **Model Explorer auto-refresh** - Added debounced Model Explorer refresh/reload on SysML/KerML save and file-system changes, including workspace-aware re-scan on create/delete.
- **Diagnostic clarity** - Suppressed cryptic empty-type unresolved warnings by ignoring empty declared type references in semantic typing checks.

## [0.9.0] - 2026-03-27

### Added

- **Semantic diagnostics hardening** - Added new semantic diagnostics for invalid multiplicity intervals (`invalid_multiplicity`) and unresolved declared type references (`unresolved_type_reference`), plus guardrails for invalid/self-referential `redefines` metadata (`invalid_redefines_reference`) when available in the semantic graph.
- **Requirements slice hardening** - Added requirement usage typing resolution to requirement definitions (including cross-file linking) and added explicit satisfy-resolution diagnostics (`unresolved_satisfy_source`, `unresolved_satisfy_target`).
- **Edit-loop diagnostics coverage** - Added integration tests covering invalid intermediate edits that later become valid, and semantic diagnostics for unresolved type references.
- **Tiered CI validation workflow** - Added an informational `.github/workflows/full-validation.yml` workflow that runs SysML release workspace validation on PR/schedule/manual dispatch.
- **Requirements slice fixtures/tests** - Added focused requirement fixtures and deterministic integration tests for same-file/cross-file requirement typing and unresolved satisfy diagnostics.

### Changed

- **Required CI fast path** - Updated `.github/workflows/ci.yml` to explicitly run required fast Rust checks (`cargo test --workspace`, `cargo clippy --workspace --all-targets`).
- **CI requirements visibility** - Added explicit requirements-slice integration test invocations to the fast CI workflow for clearer release gating signal.
- **Developer guidance** - Expanded `DEVELOPMENT.md` with semantic diagnostics pipeline/codes and clarified fast-vs-full CI validation strategy.
- **Technical debt hardening** - Added release preflight checks and SHA256 release asset checksums in `.github/workflows/release.yml`, and expanded required extension CI to include the interconnection test suite.
- **Docs and metadata alignment** - Updated `DEVELOPMENT.md` local validation steps to match current repo workflows and aligned extension experimental-view wording in `vscode/package.json`.
- **Parser reproducibility** - Pinned `sysml-v2-parser` in `spec42-core/Cargo.toml` and documented parser update policy in `DEVELOPMENT.md`.
- **VS Code extension packaging** - Updated extension packaging to include runtime dependencies required at activation time and tightened packaging excludes to avoid shipping unnecessary development/test artifacts in the VSIX.

## [0.8.0] - 2026-03-27

### Added

- **Library experience and discoverability** - Added richer library search DTOs and surfaced stronger library status/search flows in the VS Code extension, with improved support for workspace + managed library usage.
- **LSP feature coverage** - Added type hierarchy support and completed additional remaining LSP capabilities, with integration-test coverage updates to keep behavior stable.

### Changed

- **Server architecture and diagnostics** - Refactored key parts of the LSP server and model-explorer context handling, improved code lens generation, and integrated tracing-based logging to make troubleshooting and diagnostics clearer.
- **Parser and dependency updates** - Updated parser/dependency stack and related tests to improve reliability and maintainability.
- **Documentation and marketplace metadata** - Refreshed README badges/screenshots/instructions and corrected the VS Code Marketplace badge link.

## [0.7.0] - 2026-03-25

### Changed

- **Diagram quality and UX** - Improved diagram behavior across General View and Interconnection View, including stronger connector handling for typed/interface-based connections, backend-root-driven interconnection filtering, fit-to-window defaults, and related visualizer usability updates.

## [0.6.0] - 2026-03-23

### Changed

- **Interconnection visualizer layout and routing** - Improved IBD layout quality and connector routing reliability for denser models, including clearer root selection behavior and more readable exported diagrams.
- **Diagnostics quality** - Improved diagnostics stability and clarity in the VS Code workflow, with better behavior during active edit/update cycles.

## [0.5.0] - 2026-03-13

### Added

- **Interconnection View release enablement** - Promoted `interconnection-view` from experimental opt-in to a release-enabled default view with CI-backed export coverage.
- **Multi-workspace VS Code test fixtures** - Added stronger fixture coverage for single-file, multi-file, large-workspace, and timer-based visualization scenarios.

### Changed

- **Visualization UX** - Improved interconnection root selection, root summaries, port direction legend support, and clearer no-connector guidance in the visualizer.
- **Source-to-diagram behavior** - Editor-driven diagram selection now reveals and centers matching elements more reliably when available.
- **VS Code test infrastructure** - Standardized cross-platform test workspace `serverPath` configuration so Windows and Linux test hosts boot the language server consistently.

### Fixed

- **Crash and edit-loop robustness** - Hardened incremental edit handling, malformed range handling, repeated open/edit/close lifecycles, and semantic token refresh after edits.
- **VS Code integration stability** - Fixed startup and restart issues in test and CI environments, and made extension-host tests deterministic again.
- **Definition and visualization regressions** - Stabilized go-to-definition test coverage and visualizer export checks across representative fixtures.

## [0.4.0] - 2026-03-12

### Changed

- **New sysml-v2-parser** - Switched to a new sysml-v2-parser dependency for improved parsing and alignment with SysML v2.

## [0.3.0] - 2026-03-10

### Added

- **General View diagram** - New diagram view showing the model structure with element hierarchy, attributes, ports, and parts. Nodes use standard SysML-style compartments.

## [0.2.2] - 2026-03-06

### Changed

- VS Code extension display name updated to "SysML v2 Language Support" to meet marketplace requirements and reduce name confusion with other language server extensions.

## [0.2.1] - 2026-03-06

### Fixed

- **UTF-8 / multi-byte handling:** `position_to_byte_offset` now correctly converts LSP character indices to byte offsets (e.g. for "cafe"). `completion_prefix` iterates by character to avoid panics on multi-byte content. Error masking in `parse_sysml_collect_errors` uses character boundaries so multi-byte lines no longer produce invalid UTF-8.
- **Parse error messages:** Low-level Pest messages are mapped to clearer user-facing text (e.g. "expected metadata_annotation" -> "unexpected token; perhaps missing an attribute or expression"); original message is appended for debugging. Additional mappings for package, member, name, identifier, import, expressions, literals, parentheses, etc.

### Changed

- Removed unused `line_char_to_byte_offset` from kerml-parser (server already has equivalent logic). Extended `improve_pest_error_message` with more grammar-rule mappings.

### Added

- Unit tests for multi-byte edge cases: `position_to_byte_offset`, `word_at_position`, `completion_prefix`, and `parse_sysml_collect_errors` with UTF-8 in the error region.

## [0.2.0] - 2025-03-06

### Added

- Calc def result expressions: parser now supports bare result expressions (e.g. `capacity / currentDraw`) without a final semicolon, per SysML v2 7.19.2.
- Full validation suite test: parses all `.sysml` files in SysML-v2-Release `sysml/src/validation`; test is `#[ignore]`d (run with `cargo test -p kerml-parser -- --ignored`); CI runs it with `--include-ignored`. Per-file summary (pkgs, members, lines) logged when running the suite.

### Fixed

- "position" no longer incorrectly marked as keyword in semantic tokens; it is a contextual keyword only and valid as an identifier (e.g. `out position : String`).
- Shared reserved keyword list: single source of truth in `language::RESERVED_KEYWORDS` for semantic token fallback and goto-definition/rename suppression; eliminates discrepancies between keyword lists.

## [0.1.0] - 2026-03-05

### Added

- LSP over stdio: text sync, diagnostics, hover, completion, go to definition, find references, rename, document symbols, workspace symbol search, code actions, formatting.
- Workspace-aware indexing for `.sysml` and `.kerml` files (workspace folders and root URI).
- VS Code extension with SysML/KerML syntax highlighting and language configuration.
- Parser aligned with the [SysML v2 Release](https://github.com/Systems-Modeling/SysML-v2-Release) validation suite.

### Known limitations

- Parser is aligned with the SysML v2 Release validation suite; it does not claim full OMG spec compliance.
- Some constructs may have incomplete semantic token or outline coverage.

[0.21.0]: https://github.com/elan8/spec42/releases/tag/v0.21.0
[0.20.0]: https://github.com/elan8/spec42/releases/tag/v0.20.0
[0.19.0]: https://github.com/elan8/spec42/releases/tag/v0.19.0
[0.18.1]: https://github.com/elan8/spec42/releases/tag/v0.18.1
[0.18.0]: https://github.com/elan8/spec42/releases/tag/v0.18.0
[0.17.0]: https://github.com/elan8/spec42/releases/tag/v0.17.0
[0.15.1]: https://github.com/elan8/spec42/releases/tag/v0.15.1
[0.11.0]: https://github.com/elan8/spec42/releases/tag/v0.11.0
[0.15.0]: https://github.com/elan8/spec42/releases/tag/v0.15.0
[0.14.0]: https://github.com/elan8/spec42/releases/tag/v0.14.0
[0.13.0]: https://github.com/elan8/spec42/releases/tag/v0.13.0
[0.12.0]: https://github.com/elan8/spec42/releases/tag/v0.12.0
[0.10.0]: https://github.com/elan8/spec42/releases/tag/v0.10.0
[0.9.1]: https://github.com/elan8/spec42/releases/tag/v0.9.1
[0.9.0]: https://github.com/elan8/spec42/releases/tag/v0.9.0
[0.8.0]: https://github.com/elan8/spec42/releases/tag/v0.8.0
[0.7.0]: https://github.com/elan8/spec42/releases/tag/v0.7.0
[0.6.0]: https://github.com/elan8/spec42/releases/tag/v0.6.0
[0.5.0]: https://github.com/elan8/spec42/releases/tag/v0.5.0
[0.4.0]: https://github.com/elan8/spec42/releases/tag/v0.4.0
[0.3.0]: https://github.com/elan8/spec42/releases/tag/v0.3.0
[0.2.2]: https://github.com/elan8/spec42/releases/tag/v0.2.2
[0.2.1]: https://github.com/elan8/spec42/releases/tag/v0.2.1
[0.2.0]: https://github.com/elan8/spec42/releases/tag/v0.2.0
[0.1.0]: https://github.com/elan8/spec42/releases/tag/v0.1.0
