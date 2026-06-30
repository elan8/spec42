# Spec42 Roadmap

**Current version:** 0.33.x (2026-06-29)  
**Target:** 1.0.0

This document describes what needs to be true for Spec42 1.0, tracks the remaining work, and lists what is deliberately deferred beyond 1.0. It is the single authoritative planning reference; engineering detail lives in the linked documents.

---

## What 1.0 means

Spec42 1.0 is a stable, locally self-contained SysML v2 tooling suite that a practitioner can rely on for daily editing, validation, and CI. It does **not** require cloud connectivity, a commercial license, or an external MBSE platform to deliver value.

The 1.0 bar is:

- **Editor workflows are release-gating.** Formatting, navigation, hover, rename, completion, outline, and semantic highlighting work correctly on the SysML v2 workflows documented in `SUPPORTED-WORKFLOWS.md`.
- **Validation is trustworthy.** The semantic diagnostic engine covers all P0 and P1 check categories with stable codes, correct ranges, and a complete catalog. False positives from the robot-vacuum showcase have been resolved.
- **CI integration is first-class.** `spec42 check` emits text/JSON/SARIF/JUnit, supports baseline-driven suppression and warnings-as-errors, and the GitHub Action is published at `elan8/spec42@v1`.
- **All five standard SysML v2 diagram types render.** General, IBD, Action, State, and Sequence views project from the semantic graph through the shared renderer. Browser, Grid, and Geometry views ship at documented partial coverage.
- **Diagram export is deterministic.** `spec42 diagrams export` produces stable JSON and SVG via the shared renderer for all release-gating views.
- **AI assistant integration works out of the box.** MCP server, VS Code LM Tools, and HTTP API ship in the binary and are documented.
- **Libraries are bundled.** The OMG standard library and Elan8 domain libraries materialize from the binary; no external download or manual setup is required.
- **The conformance matrix is generated and enforced by CI.**
- **Documentation is accurate and references current crate names.**

---

## Current state (as of 0.34.0)

The table below shows what is **already complete** and will ship in 1.0 without additional work.

### Editor features

| Feature | Status |
|---------|--------|
| Semantic highlighting | complete |
| Hover with type details | complete |
| Context-aware completion | complete |
| Go-to definition / references | complete |
| Rename refactoring | complete |
| Document symbols / outline | complete |
| Semantic document folding | complete |
| Model Explorer | complete |
| Model Visualizer | complete |

### Validation and diagnostics

| Category | Status |
|----------|--------|
| Parser / syntax diagnostics | complete (via `sysml-v2-parser`) |
| Library resolution diagnostics | complete |
| Name, import, and namespace checks | complete (P1) |
| Typing, specialization, redefinition | complete (P1) |
| Expressions, values, units, multiplicity | complete (P1) |
| Ports, connections, interfaces, flows | complete (P1) |
| Actions, states, behavior | complete (P2) |
| Requirements, cases, verification | complete (P2) |
| Views, viewpoints, renderings, metadata | complete (P2) |
| Diagnostic catalog with stable codes | complete |
| SARIF output | complete |
| Cascade suppression and deduplication | complete |

### Diagram views

| View | Status |
|------|--------|
| General View | complete |
| Interconnection View (IBD) | complete (including scoped IBD and merged workspace IBD) |
| Action Flow View | complete |
| State Transition View | complete |
| Sequence View | complete |
| Browser View | partial (parentId hierarchy; collapsible tree renderer) |
| Grid View | partial (element table; relationship matrix via `projectionHints`) |
| Geometry View | partial (2D spatial defaults; 3D deferred post-1.0) |

### CLI, HTTP API, and AI integration

| Surface | Status |
|---------|--------|
| `spec42 check` (text/JSON/SARIF/JUnit, baseline, warnings-as-errors) | complete |
| `spec42 doctor` | complete |
| `spec42 diagrams export` (JSON + SVG via shared renderer) | complete |
| `spec42 explain-diagnostic` | complete |
| `spec42 model-summary` | complete |
| HTTP API (`spec42 api serve`) | complete |
| MCP server (`spec42-mcp`) | complete |
| VS Code LM Tools (`#spec42Check`, `#spec42Doctor`, `#spec42ModelSummary`, `#spec42ExplainDiagnostic`) | complete |
| GitHub Action (`elan8/spec42@vX.Y.Z`) | complete |

### Embedding API (`workspace` crate)

All five phases of the embedding plan are complete. See [ADR 0003](adr/0003-spec42-host-embedding-crate.md) for the design rationale.

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Engine builder and library catalog | complete |
| 2 | Immutable workspace snapshot | complete |
| 3 | Versioned metadata, structured errors, cancellation | complete |
| 4 | Semantic snapshot comparison | complete |
| 5 | Incremental snapshot updates (experimental flag) | complete |

### Library management

| Feature | Status |
|---------|--------|
| Bundled OMG standard library (KPAR) | complete |
| Bundled Elan8 domain libraries | complete |
| Sysand status detection and doctor reporting | complete |
| User-configured library paths | complete |

---

## Remaining work for 1.0

### ~~R1 — Remove `domain_rules` placeholder crate~~ ✓ done

`crates/domain_rules/` was an empty placeholder (no code, not a workspace member). Removed 2026-06-29.

### ~~R2 — Promote Sequence View from experimental to stable~~ ✓ done

The Sequence View projection is complete per the conformance matrix, but the code path carries an experimental marker. Before 1.0:

- Remove or document the experimental flag.
- Ensure the sequence-view rendering path has regression fixtures alongside General/IBD/Action/State.
- Update `SUPPORTED-WORKFLOWS.md` to include sequence diagrams as a release-gating workflow.

### ~~R3 — Enforce performance guardrails in CI~~ ✓ done

`scripts/check-perf-budgets.mjs` reads a perf report JSON and exits non-zero on any budget violation. Added to the nightly `large-workspace-performance` job (2026-06-29): covers large-workspace and drone-interconnection fixtures. Budgets are nightly gates; to promote to PR gates move the relevant test into `ci.yml`. See `PERFORMANCE-GUARDRAILS.md` for current thresholds.

### ~~R4 — Conformance matrix CI enforcement~~ ✓ done (was already in place)

`ci.yml` already blocks on `node scripts/generate-conformance-matrix.mjs --check` in the `rust-core` job (line 87-88). The script throws if `docs/reference/CONFORMANCE-MATRIX.md` is stale relative to `docs/reference/conformance-metadata.json`.

### ~~R5 — Fix stale crate names in engineering documentation~~ ✓ done

Several engineering documents still reference pre-refactor crate names:

| Old name | Current name |
|----------|-------------|
| `semantic_core` | `sysml_model` |
| `kernel` | `lsp_server` |
| `spec42_host` | `workspace` |

Affected files (see individual update notes below):
- `docs/engineering/DIAGNOSTIC-CHECKS-ROADMAP.md`
- `docs/engineering/VIEW-EXPOSE-ROADMAP.md`
- `docs/engineering/AST-SEMANTIC-COVERAGE.md`
- `docs/engineering/PERFORMANCE-GUARDRAILS.md`
- `docs/architecture/SEMANTIC_CORE_ARCHITECTURE.md`
- `docs/adr/0003-spec42-host-embedding-crate.md`

### ~~R6 — VS Code extension marketplace readiness~~ ✓ done

- The extension must be published or ready to publish to the VS Code Marketplace under a stable publisher ID.
- `vscode/README.md` must reflect current features without references to in-progress work.
- The extension package must pass the marketplace smoke-test checklist (icon, description, category, keywords, changelog).

### R7 — sysml-v2-parser alignment

Track the OMG SysML v2 specification release cycle. The parser is currently at **0.28.0** (pinned 2026-06-29). Before 1.0:

- Confirm the pinned parser covers the OMG submission being targeted (currently aligned with 0.32 / 2026-06-22 spec).
- Update graph builders for any new AST body/member enums introduced in parser releases between 0.27.0 and the 1.0 release pin.
- Run the full OMG validation suite (`cargo test --no-default-features`) and triage any new informational failures.

See [AST-SEMANTIC-COVERAGE.md](engineering/AST-SEMANTIC-COVERAGE.md) for the current AST-to-graph coverage matrix.

### R8 — Release version and changelog

- Bump workspace version from `0.33.x` to `1.0.0` across all crates.
- Write a `CHANGELOG.md` (or `RELEASE-NOTES.md`) covering the major 0.x milestones for users upgrading from earlier versions.
- Tag the release in git and publish the GitHub Action at `elan8/spec42@v1`.

---

## Deferred: post-1.0

The following capabilities are explicitly out of scope for 1.0. They may appear in future roadmap cycles.

| Capability | Rationale |
|-----------|-----------|
| OMG Systems Modeling API (element CRUD, commits) | Requires a separate repository/storage layer; tracked as a distinct service |
| Python Automator equivalent | Useful but not blocking adoption |
| ReqIF / DOORS / Polarion bridges | Integration work requiring external partnerships |
| Editable table / matrix views | Authoring in views is a post-1.0 editing surface |
| Cloud / team workflow surfaces | Multi-user and hosted scenarios are post-1.0 |
| 3D Geometry View | Backend spatial model is partial; full 3D deferred |
| Sysand package install / update orchestration | Status detection ships in 1.0; package management requires Sysand CLI integration |
| Full KerML OwnedExpression (`if` / `let` / lambda in constraints) | Incremental tranches only; remaining forms deferred |
| HTTP API caching / in-memory state across requests | Stateless design serves 1.0 use cases |
| Incremental snapshot updates (stable, non-experimental) | Experimental flag ships in 1.0; stable graduation requires benchmark targets |

---

## sysml-v2-parser dependency notes

`sysml-v2-parser` is an external crate (crates.io primary, `.cargo/config.toml` patch for pre-publish testing). Spec42's semantic quality is directly coupled to parser coverage.

**Current pin:** `0.28.0`

**Coupling policy:**

- Pin to a specific version; do not use `>=` ranges.
- Update graph builders when new AST body enums are added (exhaustive `match` in build policy).
- Cache invalidation uses the parser's schema version field — bump the cache key on any AST schema change.
- Run `cargo test --workspace` with stdlib bundle and `--no-default-features` after every parser bump before merging.

**Known deferred parser items:** Full `OwnedExpression` support (`if` / `let` / lambda); `istype` / `hastype` / `as` classification expressions and `@Metaclass` filters are handled via `exprClass` and `conditionIsBoolean` (added in parser 0.23.0).

---

## Related documents

| Document | Purpose |
|----------|---------|
| [SUPPORTED-WORKFLOWS.md](../docs/SUPPORTED-WORKFLOWS.md) | Release-gating editor and CLI workflows |
| [CONFORMANCE-MATRIX.md](reference/CONFORMANCE-MATRIX.md) | Generated SysML v2 feature coverage |
| [COMPETITIVE-ROADMAP.md](engineering/COMPETITIVE-ROADMAP.md) | Competitive positioning and 1.0 acceptance criteria |
| [DIAGNOSTIC-CHECKS-ROADMAP.md](engineering/DIAGNOSTIC-CHECKS-ROADMAP.md) | Diagnostic check inventory and status |
| [AST-SEMANTIC-COVERAGE.md](engineering/AST-SEMANTIC-COVERAGE.md) | Parser AST to semantic graph coverage matrix |
| [PERFORMANCE-GUARDRAILS.md](engineering/PERFORMANCE-GUARDRAILS.md) | Performance budgets and CI reporting |
| [VIEW-EXPOSE-ROADMAP.md](engineering/VIEW-EXPOSE-ROADMAP.md) | View expose and rendering resolution implementation notes |
