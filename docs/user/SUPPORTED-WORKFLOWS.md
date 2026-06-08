# Supported Workflows

This document describes what the project currently supports well, what is usable with caveats, and what is still experimental on the path to `1.0`.

## Stable Core Workflows

These are the workflows the project is actively hardening for `1.0` and treats as release-gating:

- Editing `.sysml` and `.kerml` files in VS Code without crashing the server
- Diagnostics for invalid intermediate text while typing
- Hover on keywords and indexed symbols
- Go to definition within a file and across indexed workspace files
- Find references across indexed workspace files
- Rename for indexed symbols
- Document symbols and workspace symbol search
- Formatting
- Semantic tokens
- Folding ranges
- Deterministic CLI validation reports for CI (`text`, `json`, `sarif`, `junit`)
- Optional Sysand package-manager detection and dependency-root ingestion
- Shared-renderer visualization and deterministic CLI SVG/JSON diagram export; routed views use ELK.js through embedded QuickJS in the Rust exporter
- Generated conformance reporting for language, validation, views, CLI, and Sysand integration

## Usable With Caveats

These workflows are available and useful, but still have known limits that should be assumed by users and contributors:

- Workspace indexing in larger repositories
  The extension can truncate discovery per workspace folder and file type based on `spec42.workspace.maxFilesPerPattern` (legacy `sysml-language-server.workspace.maxFilesPerPattern` is still supported).
- Library path indexing
  Useful for hover, definition, and completion, but dependent on parser coverage and available files.
- Model Explorer workspace mode
  Works for practical navigation, but partial indexing and parser recovery can affect completeness.
- General visualization view
  Usable for inspection and export, but still downstream of parser/model quality.
- Interconnection visualization view
  Release-enabled for structural connection inspection, export, and root-based exploration, with known caveats mainly around very dense routing/layout.
- Shared vs legacy renderer scope and SysML graphical-notation roadmap: [`SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md`](../architecture/SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md)
- Action Flow, State Transition, and Sequence views are available by default in the visualizer and are treated as release-gating workflows for `1.0`.
- Sequence View targets Spec42-authored `SequenceView` + `SoftwareInteractions` models; full UML sequence-diagram parity (fragments, advanced message kinds) is still evolving.

## Experimental Areas

The following areas are intentionally not release-gating for `1.0`:

- Diagram export beyond deterministic SVG/JSON, such as PNG/PDF or browser-identical rendering
- Full visual parity between CLI SVG and the VS Code renderer; the 1.0 CLI target is stable ELK layout/routing parity for routed views
- Broader SysML v2 language coverage outside the currently well-tested subset
- Deep semantic validation beyond the existing parser and graph-based support
- Editable table/matrix views
- Python automation APIs and ReqIF/DOORS/Polarion bridges

## Current Support Boundaries

The project does not currently claim:

- Full OMG SysML v2 specification coverage
- Full semantic validation for all constructs
- Stable behavior for every visualization type
- Production-grade performance for very large repositories without tuning

## What `1.0` Means

For this project, `1.0` means:

- safe daily use for core editing workflows
- clear failure states and recovery in VS Code
- deterministic CI coverage for editor, validation, report, package-detection, and shared-renderer workflows
- a credible open/local-first alternative for teams comparing SysML v2 tooling ecosystems

It does not mean:

- every planned SysML feature is implemented
- every visualization is stable
- the full specification is covered
- Spec42 replaces Sysand; Sysand is treated as optional package infrastructure
