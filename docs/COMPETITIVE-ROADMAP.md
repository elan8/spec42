# Spec42 1.0 Competitive Roadmap

Spec42's 1.0 wedge is open, local-first SysML v2 tooling with reliable editor feedback, CI validation, transparent conformance, shared-renderer diagrams, and optional Sysand package integration. The goal is not to clone every commercial Syside/Sensmetry surface at once; it is to give teams a clear switching reason where openness, automation, and explainability matter.

Public benchmark sources:

- Sensmetry feature overview: https://sensmetry.com/features/
- Sensmetry roadmap update: https://sensmetry.com/roadmap-2025-q4-update/
- Sysand repository: https://github.com/sensmetry/sysand

| Bucket | Capability | Spec42 1.0 position |
| --- | --- | --- |
| match | Diagnostics, formatting, rename, navigation | Release-gating editor workflows with CLI parity |
| match | Visualization export | VS Code export plus Rust-owned CLI SVG/JSON export; routed SVG uses vendored ELK.js through embedded QuickJS |
| match | CI validation | `spec42 check` with text/json/sarif/junit, warnings-as-errors, and baselines |
| exceed | Transparent conformance | Generated conformance matrix checked in CI |
| exceed | Local-first workflow | Bundled server, bundled standard library, no cloud dependency |
| exceed | Open renderer coverage | Shared renderer with complete core views and provisional Browser/Grid/Geometry |
| exceed | Explainable diagnostics | Diagnostics keep stable codes and source ranges for automation |
| integrate | Sysand package management | Detect status, report in doctor, and ingest dependency roots when present |
| defer | Python Automator equivalent | Post-1.0 |
| defer | ReqIF/DOORS/Polarion bridges | Post-1.0 |
| defer | Editable table/matrix views | Post-1.0 |
| defer | Cloud/team workflow surfaces | Post-1.0 |

## 1.0 Acceptance

- Existing editor/LSP workflows remain stable.
- `spec42 doctor --format json` reports Sysand status without requiring Sysand.
- `spec42 check` supports SARIF/JUnit and baseline-driven CI.
- `spec42 diagrams export` produces deterministic SVG/JSON artifacts for shipped shared-renderer views, with ELK-backed SVG for General, Interconnection, Action Flow, and State Transition.
- `docs/CONFORMANCE-MATRIX.md` is generated from checked-in metadata and enforced by CI.
