# Spec42 OSS v1.0.0 Release Hardening Plan

Practical plan to bring OSS `Spec42` to v1.0.0 quality by prioritizing reliability, predictable behavior, and user trust over new feature surface.

---

## Goals

- Ship a stable `v1.0.0` for core SysML/KerML authoring workflows in VS Code.
- Keep release risk low by freezing major new feature work during hardening.
- Make release readiness measurable through explicit gates and acceptance criteria.

## Non-goals (for this phase)

- New major LSP feature families.
- PRO-specific functionality.
- Full OMG SysML v2 language coverage claims.

---

## v1.0.0 Scope Baseline

`v1.0.0` is considered release-ready for these workflows:

- Open/edit/save `.sysml` / `.kerml` without server crashes in normal usage.
- Diagnostics while typing invalid intermediate text.
- Reliable hover, go-to-definition, references, rename, document/workspace symbols.
- Formatting, semantic tokens, and folding ranges on representative files.
- `sysml/model` core output quality for currently release-enabled views.

Explicit caveats remain documented (large-workspace tuning, parser coverage boundaries, non-core visualization experimental areas).

---

## Release Gates (Required)

1. **Rust checks pass**
   - `cargo test --workspace`
   - `cargo clippy --workspace --all-targets -- -W clippy::all`

2. **Core LSP integration confidence**
   - Core integration suites pass on CI and locally for release branch.
   - No known hangs/flaky tests in release-gating scenarios.

3. **Regression coverage for recent risk areas**
   - Invalid-intermediate-edit diagnostics behavior.
   - Cross-file typing/definition/reference resolution.
   - Requirements slice diagnostics and model graph checks.

4. **Docs and support readiness**
   - README quickstart works on clean environment.
   - DEVELOPMENT troubleshooting/runbook updated for current CI/test workflows.
   - CHANGELOG contains clear `v1.0.0` release narrative and known limits.

5. **Release packaging sanity**
   - Extension package layout verification passes.
   - Server binary resolution paths validated in extension tests/fixtures.

---

## Recommended Workstreams

### A) Stability and Bug Burn-down

- Triage open defects into:
  - `release-blocker`
  - `high`
  - `post-v1`
- Fix all blockers and high issues that affect core editing loop.
- Add regression tests per fix (unit/integration) before merge.

### B) Test Reliability and Coverage

- Promote deterministic tests for release-critical paths.
- Eliminate known flake patterns (unbounded reads, racey sleeps, non-deterministic ordering assumptions).
- Ensure requirements and semantic diagnostics checks stay visible in CI logs.

### C) Performance and Workspace Confidence

- Validate behavior on representative medium/large workspaces.
- Record and document acceptable bounds for scan/index startup.
- Keep performance tuning scoped to low-risk improvements only.

### D) Documentation and First-Run Experience

- Polish quickstart and "what works today" messaging.
- Keep support boundaries explicit and honest.
- Add short release troubleshooting checklist for common setup issues.

### E) Release Process Readiness

- Cut release candidate (`v1.0.0-rc1`) and run smoke tests.
- Collect issues during RC window, fix only release blockers.
- Tag and publish `v1.0.0` once gates are green.

---

## PR-Sized Backlog (Starter)

1. Define and publish `v1.0.0` release checklist in docs.
2. Add/adjust integration tests for top 3 known regression-prone flows.
3. Remove or fix at least one flaky test pattern from integration harness usage.
4. Add CI summary note for release-critical test group outcomes.
5. Update README with explicit "supported workflows" and "known limits".
6. Prepare changelog draft for `v1.0.0` (added/changed/fixed/known limits).
7. Run RC smoke test matrix (Windows/Linux, single-file + multi-file workspace).

---

## Exit Criteria

- All required checks pass in CI on release candidate branch.
- No unresolved release-blocker issues remain.
- Core workflows validated manually from a clean setup.
- Documentation and changelog reflect actual shipped behavior and boundaries.
- Team can explain, in one page, what `v1.0.0` guarantees and what it does not.

---

## Risk Register (v1-focused)

- **Risk:** Scope creep delays release.  
  **Mitigation:** Feature freeze except blocker fixes and low-risk polish.

- **Risk:** Hidden test flakiness causes late regressions.  
  **Mitigation:** Deterministic request/response synchronization in integration tests.

- **Risk:** Over-claiming language/semantic coverage harms trust.  
  **Mitigation:** Keep support boundaries explicit in README/roadmap/changelog.

- **Risk:** Packaging/runtime mismatch on user machines.  
  **Mitigation:** Validate extension server resolution paths in RC smoke tests.

---

## Suggested Timeline (Lean)

- **Week 1:** Freeze scope, triage issues, stabilize tests.
- **Week 2:** Fix blockers/highs, complete regression coverage and docs polish.
- **Week 3:** RC (`rc1`), smoke tests, blocker-only fixes, release tag.

