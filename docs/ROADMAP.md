# Spec42 Roadmap

**Target:** 1.0.0

This document describes what Spec42 1.0 means, summarizes the shipped baseline, and lists capabilities deliberately deferred beyond 1.0. It is a **product reference**, not a work tracker — open work lives in [GitHub Issues](https://github.com/elan8/spec42/issues) (milestone [1.0](https://github.com/elan8/spec42/milestone/1)). Live package and parser pins live in `Cargo.toml` / `Cargo.lock`, not here.

---

## What 1.0 means

Spec42 1.0 is a stable, locally self-contained SysML v2 tooling suite that a practitioner can rely on for daily editing, validation, and CI. It does **not** require cloud connectivity, a commercial license, or an external MBSE platform to deliver value.

The 1.0 bar is:

- **Editor workflows are release-gating.** Formatting, navigation, hover, rename, completion, outline, and semantic highlighting work correctly on the SysML v2 workflows documented in `SUPPORTED-WORKFLOWS.md`.
- **Validation is trustworthy.** The semantic diagnostic engine covers all P0 and P1 check categories with stable codes, correct ranges, and a complete catalog. False positives from the robot-vacuum showcase have been resolved.
- **CI integration is first-class.** `spec42 check` emits text/JSON/SARIF/JUnit, supports baseline-driven suppression and warnings-as-errors, and the GitHub Action is published at `elan8/spec42@v1`.
- **All five standard SysML v2 diagram types render.** General, IBD, Action, State, and Sequence views project from the semantic graph through the shared renderer. Browser, Grid, and Geometry views ship at documented partial coverage.
- **Diagram export is deterministic.** `spec42 diagrams export` produces stable JSON and SVG via the shared renderer for all release-gating views.
- **AI assistant integration works out of the box.** The CLI's JSON output (`check`, `model-summary`, `explain-diagnostic`, `doctor`) plus VS Code LM Tools and per-host skill/instructions docs cover assistant workflows without a network surface.
- **Libraries are bundled.** The OMG standard library and Elan8 domain libraries materialize from the binary; no external download or manual setup is required.
- **The conformance matrix is generated and enforced by CI.**
- **Documentation is accurate and references current crate names.**

---

## Current state

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

The public robot-vacuum showcase completes without errors or crashes. The Spec42-owned false
positives found in the 0.44.19 baseline have been eliminated. Project-declared units (`Ah`,
`mAh`, `ms` via `VacuumCleanerQuantitiesAndUnits`) land in the pinned showcase commit; CI runs
the zero-unexpected-warning gate (`robot_vacuum_snapshot`).

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

### CLI and AI integration

| Surface | Status |
|---------|--------|
| `spec42 check` (text/JSON/SARIF/JUnit, baseline, warnings-as-errors) | complete |
| `spec42 doctor` | complete |
| `spec42 diagrams export` (JSON + SVG via shared renderer) | complete |
| `spec42 explain-diagnostic` | complete |
| `spec42 model-summary` | complete |
| VS Code LM Tools (`#spec42Check`, `#spec42Doctor`, `#spec42ModelSummary`, `#spec42ExplainDiagnostic`) | complete |
| GitHub Action (`elan8/spec42@vX.Y.Z`) | complete |

The MCP server (`spec42-mcp`) and read-only HTTP API (`spec42 api serve`) were removed before
1.0 — see [#51](https://github.com/elan8/spec42/issues/51). CLI + per-host skill/instructions
(VS Code LM Tools, Copilot, Cursor, …) is the sole AI-integration surface.

### Embedding API (`workspace` crate)

All five phases of the embedding plan are complete.

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

## 1.0 gates (tracked on GitHub)

Open 1.0 work is tracked in GitHub Issues, not in this file:

| Item | Issue |
|------|-------|
| R7 — `sysml-v2-parser` pin and graph coverage for 1.0 | [#18](https://github.com/elan8/spec42/issues/18) |
| R8 — Ship 1.0.0 (version, release notes, `elan8/spec42@v1`) | [#19](https://github.com/elan8/spec42/issues/19) |

Completed release-prep items (R1–R6, R9) are historical only: domain_rules removal, Sequence View promotion, perf-budget CI, conformance-matrix CI check, crate-name doc refresh, marketplace readiness, and robot-vacuum false-positive elimination (0.45.0). See git history / CHANGELOG for detail.

**Parser note:** part-body `ref action` / `ref state` / nested action·state are real graph nodes (not `opaque member`). Full P5+ unified definition/usage rewrite remains deferred.

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
| Incremental snapshot updates (stable, non-experimental) | Experimental flag ships in 1.0; stable graduation requires benchmark targets |

---

## sysml-v2-parser dependency notes

`sysml-v2-parser` is an external crate (crates.io primary, `.cargo/config.toml` patch for pre-publish testing). Spec42's semantic quality is directly coupled to parser coverage.

**Current pin:** see `sysml-v2-parser` in the workspace `Cargo.toml` / `Cargo.lock` (do not duplicate the version here).

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
| [SUPPORTED-WORKFLOWS.md](user/SUPPORTED-WORKFLOWS.md) | Release-gating editor and CLI workflows |
| [CONFORMANCE-MATRIX.md](reference/CONFORMANCE-MATRIX.md) | Generated SysML v2 feature coverage |
| [DIAGNOSTIC-CATALOG.md](engineering/DIAGNOSTIC-CATALOG.md) | Diagnostic check inventory |
| [PERFORMANCE-GUARDRAILS.md](engineering/PERFORMANCE-GUARDRAILS.md) | Performance budgets and CI reporting |
| [SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md](architecture/SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md) | Shared renderer contract |
