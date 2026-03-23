# Spec42 Plugin Ecosystem Proposal

## Purpose

This document captures a practical plugin strategy for Spec42 in VS Code, including:

- when to use an Activity Bar tab,
- how to separate SysML v2 editing from Spec42 operations, and
- which plugin categories make sense as open source vs commercial offerings.

## UX Positioning in VS Code

### SysML v2 Editing Workflow (No Activity Bar tab required)

For day-to-day modeling, users can stay in the editor context:

- open a SysML v2 source file,
- view diagram/preview side-by-side,
- iterate directly in file context.

This keeps authoring lightweight and avoids adding extra navigation overhead.

### Spec42 Operations Workflow (Activity Bar tab recommended)

Use a dedicated `Spec42` Activity Bar container for project-level operations such as:

- document generation,
- source code generation,
- validation/check suites,
- artifact browsing and reruns.

This gives a clear mental model:

- **Editor context = modeling**
- **Activity Bar context = production workflows**

## Proposed Spec42 Activity Bar Information Architecture

### Generate

- Generate documents
- Generate source code
- Generate diagram bundles

### Build and Validate

- Validate model
- Run consistency checks
- Show latest run status

### Artifacts

- Recent outputs
- Open output folder
- Re-run latest job

### Core Action Bar (always visible)

- `Validate`
- `Generate`
- `Run All Checks`
- `Open Last Output`
- `Refresh`

## Design Guardrails

- Keep the Spec42 tab project-aware (workspace/profile level), not file-only.
- Do not duplicate native VS Code panes without added value.
- Ensure long-running operations have progress, logs, and cancel/retry.
- Persist useful session state (last target/profile, last output, collapsed sections).

## Plugin Types That Make Sense

## Open Source Plugin Categories

### Model quality and linting

- Rule packs for naming, structure, and traceability.
- Fast local checks with CI-friendly outputs.

### Diagram and artifact generation

- Deterministic SVG/PNG/PDF generation.
- Reproducible outputs suitable for version control.

### Template-based document generation

- Markdown/Asciidoc/Sphinx exports from model data.
- Document packs for specs, interface docs, and architecture docs.

### Code skeleton generation

- Interfaces/stubs/DTO/config scaffolds from model definitions.
- Language-specific packs (for example Rust, TypeScript, C++).

### Import/export connectors

- JSON/YAML/CSV and other pragmatic interchange formats.
- Round-trip support with conflict reporting.

### CLI and CI integration helpers

- Shared commands/presets for local and CI parity.
- Stable machine-readable output for pipelines.

### Visualization and model exploration

- Cross-reference explorers and dependency graphs.
- Model diffs between revisions.

## Commercial Plugin Categories

### Enterprise ALM and requirements integration

- Bi-directional sync with enterprise systems.
- Impact analysis and traceability dashboards.

### Compliance and certification packs

- Domain-specific validation libraries.
- Audit/evidence exports for certification workflows.

### Advanced code generation suites

- Production-grade generators and runtime integration.
- Trace links from generated code back to model elements.

### Collaboration and governance workflows

- Review gates, approvals, and sign-off trails.
- Role-based policies and quality thresholds.

### Portfolio analytics

- Multi-repo architecture metrics and drift detection.
- Management dashboards and trend tracking.

### Secure enterprise deployment

- On-prem and air-gapped operation.
- Enterprise authentication and policy controls.

## Packaging Strategy

- Keep a strong open source core: model processing, validation engine, basic generators, and CI usability.
- Publish open extension packs for community rules/templates/connectors.
- Offer commercial add-ons where enterprise value is highest: compliance, governance, enterprise integrations, and advanced generation.

## Prioritized Roadmap (Suggested)

### Phase 1 (open source foundation)

- Spec42 Activity Bar with `Generate`, `Build and Validate`, and `Artifacts`.
- Deterministic diagram/doc generation.
- Baseline lint/quality checks and CI output format.

### Phase 2 (open source ecosystem growth)

- Connector SDK + reference connectors.
- Plugin marketplace examples and templates.
- Model diff and traceability explorer improvements.

### Phase 3 (commercial expansion)

- Enterprise ALM integrations.
- Compliance packs with audit-ready reporting.
- Governance workflows and advanced code generation.

## Decision Criteria for New Plugins

A plugin should be built when it:

- removes recurring manual work,
- has clear inputs/outputs,
- runs reliably in local and CI contexts,
- improves traceability,
- provides enough value to justify long-term maintenance.
