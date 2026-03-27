# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
- **Parser reproducibility** - Pinned `sysml-parser` to an explicit git revision in `spec42-core/Cargo.toml` and documented parser update policy in `DEVELOPMENT.md`.

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

- **New sysml-parser** — Switched to a new sysml-parser dependency for improved parsing and alignment with SysML v2.

## [0.3.0] - 2026-03-10

### Added

- **General View diagram** — New diagram view showing the model structure with element hierarchy, attributes, ports, and parts. Nodes use standard SysML-style compartments.

## [0.2.2] - 2026-03-06

### Changed

- VS Code extension display name updated to "SysML v2 Language Support" to meet marketplace requirements and reduce name confusion with other language server extensions.

## [0.2.1] - 2026-03-06

### Fixed

- **UTF-8 / multi-byte handling:** `position_to_byte_offset` now correctly converts LSP character indices to byte offsets (e.g. for "café"). `completion_prefix` iterates by character to avoid panics on multi-byte content. Error masking in `parse_sysml_collect_errors` uses character boundaries so multi-byte lines no longer produce invalid UTF-8.
- **Parse error messages:** Low-level Pest messages are mapped to clearer user-facing text (e.g. "expected metadata_annotation" → "unexpected token; perhaps missing an attribute or expression"); original message is appended for debugging. Additional mappings for package, member, name, identifier, import, expressions, literals, parentheses, etc.

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
