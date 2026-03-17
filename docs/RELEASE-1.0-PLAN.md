# SysML Language Server 1.0 Release Plan

This document describes the shortest credible path to a stable `1.0` release of `sysml-language-server`.

The guiding principle for `1.0` is:

1. Robustness first
2. Usability second
3. New features third

`1.0` should mean "safe to use daily for core editing workflows", not "implements all of SysML v2".

## Goals

- Make the language server resilient during normal editing, including invalid intermediate text.
- Make failures visible, understandable, and recoverable for users.
- Make the core editing experience dependable in VS Code:
  diagnostics, hover, definition, references, rename, formatting, semantic tokens.
- Establish release gates and regression coverage so quality does not drift.

## Non-Goals For 1.0

- Full SysML v2 specification coverage
- Full semantic validation of all language constructs
- Shipping every planned visualization view
- Adding broad new LSP features before the current core is stable

## Current Baseline

Strengths:

- Core LSP features exist and the Rust test suite is green.
- There are integration tests for key LSP flows.
- The VS Code extension already has restart, status, explorer, and visualization foundations.

Current risks:

- Some runtime code paths still rely on assumptions that should become guarded fallbacks.
- Some failure modes in the extension are swallowed or only visible in logs.
- Extension tests are still permissive in ways that can hide real regressions.
- Workspace-scale behavior, crash recovery, and degraded-mode behavior are not yet explicit product guarantees.

## Release Criteria

The project is ready for `1.0` only when all of the following are true:

- Editing SysML and KerML files in VS Code no longer causes known server crashes.
- Invalid intermediate edits produce diagnostics or degraded behavior, not process termination.
- Server startup, missing binary, bad config, and repeated crashes are surfaced clearly in the extension UI.
- Core LSP flows are covered by deterministic CI tests:
  diagnostics, hover, definition, references, rename, formatting, semantic tokens, typing/edit scenarios.
- Large-workspace behavior has clear limits and acceptable performance.
- Documentation clearly states supported workflows, current limitations, and troubleshooting steps.

## Workstreams

### 1. Robustness

This is the highest-priority workstream and the main blocker for `1.0`.

Objectives:

- Remove panic-prone assumptions from production code paths.
- Ensure document updates never crash the server.
- Preserve partial functionality when parsing or graph-building fails.
- Make bad client input observable in logs and, where useful, in the UI.

Backlog:

- Audit production `unwrap`, `expect`, and panic-prone assumptions in server runtime code.
- Replace each runtime hard-fail with one of:
  return `None`, return empty result, skip invalid edge, or log and continue.
- Add guards around document update and semantic graph rebuild flows.
- Log invalid incremental changes and mismatched positions instead of silently ignoring them.
- Add defensive handling around visualization/model extraction paths.
- Add smoke tests for repeated open/edit/close cycles.
- Add regression tests for Unicode, UTF-16, and malformed edit ranges.

Definition of done:

- No known crash-on-edit bugs remain.
- Runtime paths no longer use unchecked assumptions where user input can reach them.
- All reproduced crashes have permanent regression tests.

### 2. Usability

After crash resistance, the next biggest value is making the product understandable and recoverable.

Objectives:

- Users should know whether the server is starting, healthy, degraded, or crashed.
- Users should get actionable messages for bad setup and bad state.
- Workspace limits and partial-loading behavior should not be surprising.

Backlog:

- Add explicit extension states:
  `Starting`, `Ready`, `Indexing`, `Degraded`, `Restarting`, `Crashed`.
- Improve startup errors for:
  missing server binary, invalid `serverPath`, invalid `libraryPaths`, failed launch.
- Add automatic restart with capped retries and backoff for server crashes.
- Show a user-facing message after repeated restart failure with:
  `Show Output`, `Restart Server`, and config guidance.
- Improve status bar messaging to show health, not only diagnostics.
- Surface workspace loading limits when not all files are indexed.
- Write a concise troubleshooting guide:
  install, custom server path, logs, common failure modes, recovery.

Definition of done:

- A user can understand what the extension is doing without reading logs.
- Crash recovery and configuration problems are visible and actionable.
- Basic setup works for both bundled and custom-server use.

### 3. Test and CI Hardening

`1.0` needs stronger release confidence than the current happy-path coverage.

Objectives:

- Prevent regressions in editor typing flows.
- Make extension tests fail when the real product is broken.
- Validate workspace scenarios that are close to user reality.

Backlog:

- Make VS Code tests strict in CI:
  do not silently pass when the server failed to start.
- Add end-to-end tests for:
  typing, invalid syntax while typing, restart after crash, multi-file rename, workspace definition, semantic tokens after edits.
- Add large-workspace smoke tests with bounded fixture size.
- Add regression fixtures for every crash or data-corruption bug fixed before `1.0`.
- Add CI matrix coverage for Windows and Linux at minimum.
- Add a release-check workflow that runs:
  Rust tests, extension tests, packaging checks, and a basic startup smoke test.

Definition of done:

- CI reliably fails when the extension or server is not usable.
- Core workflows are exercised end-to-end, not only via unit tests.
- Release candidates are validated the same way every time.

### 4. Performance and Scale

This work should support stability and usability, not distract from them.

Objectives:

- Keep common workspaces responsive.
- Make indexing and refresh behavior predictable.
- Avoid pathological slowdowns in editor flows.

Backlog:

- Measure document edit latency for small, medium, and large files.
- Add debounce and cancellation where repeated work can pile up.
- Reduce full recomputation where possible in `didChange`.
- Add progress reporting or status updates for workspace scan/index phases.
- Revisit hardcoded file caps and make them configurable or visible.
- Ensure partial workspace indexing is clearly surfaced to the user.

Definition of done:

- Editing remains responsive in expected project sizes.
- Workspace indexing has observable progress and clear limits.
- Performance regressions can be detected before release.

### 5. Documentation and Release Readiness

This is the final layer that turns a good codebase into a usable release.

Objectives:

- Make the support contract explicit.
- Make installation and troubleshooting easy.
- Ship a release that feels deliberate, not accidental.

Backlog:

- Document supported features for `1.0`.
- Document known limitations and experimental areas.
- Add a `1.0` release checklist.
- Prepare release notes focused on:
  stability, usability, supported workflows, known gaps.
- Verify packaging of bundled binaries and extension metadata.

Definition of done:

- A new user can install and troubleshoot the extension without reading the source.
- `1.0` release notes match actual product behavior.

## Suggested Milestones

### Milestone A: Crash-Free Editing

Scope:

- Runtime guard audit
- Edit-path hardening
- Crash regression tests
- Better crash logging

Exit condition:

- No known crash-on-edit issues remain in local and CI scenarios.

### Milestone B: Visible and Recoverable UX

Scope:

- Startup and crash-state UX
- Auto-restart strategy
- Improved status bar and output guidance
- Config validation

Exit condition:

- A user can diagnose or recover from common failures without guessing.

### Milestone C: Release Confidence

Scope:

- Strict VS Code tests
- CI hardening
- Large-workspace smoke coverage
- Release-check workflow

Exit condition:

- The team has repeatable evidence that the product is stable enough to ship.

### Milestone D: 1.0 Packaging and Docs

Scope:

- Final docs
- Release checklist
- Packaging verification
- Known-limitations review

Exit condition:

- The release can be published with clear expectations and low surprise.

## Prioritized Backlog

Priority P0:

- Remove remaining runtime panic points in production paths.
- Add explicit logging for failed incremental edits and degraded parsing.
- Add crash regression tests for edit flows.
- Improve extension startup/crash messaging.
- Stop allowing extension tests to silently pass when the server is unavailable in CI.

Priority P1:

- Auto-restart with backoff and user feedback.
- Better status bar server-health states.
- Large-workspace smoke tests.
- Workspace indexing visibility and file-limit messaging.
- Release checklist and troubleshooting docs.

Priority P2:

- Performance optimizations for repeated recomputation.
- Additional visualization stabilization.
- More LSP features after core reliability goals are met.

## Risks

- Parser limitations may be perceived as server instability if diagnostics and messaging are weak.
- Visualization complexity can consume time that should go to core editor reliability.
- Silent skips in tests can create false confidence.
- Workspace behavior may feel random if caps and partial indexing are not visible.

## Recommended Operating Rule

Until `1.0` ships:

- Every production crash gets a regression test.
- Every user-visible failure gets an explicit message or status.
- No new major feature is prioritized above a known reliability problem in the core edit loop.

## Suggested Next Actions

1. Create a `1.0` milestone in the issue tracker using the workstreams in this document.
2. Open a "runtime guard audit" task for all production `unwrap` and assumption-heavy paths.
3. Open a "typing stability" test task for end-to-end edit scenarios in VS Code.
4. Open a "startup/crash UX" task for extension messaging and restart behavior.
5. Reassess feature additions only after Milestone A and Milestone B are complete.
