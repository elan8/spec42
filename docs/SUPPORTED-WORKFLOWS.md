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
- Experimental diagram views via `spec42.visualization.enableExperimentalViews` (legacy `sysml-language-server.visualization.enableExperimentalViews` is still supported)
  Useful for evaluation and iteration, but not yet release-gating for `1.0`.

## Experimental Areas

The following areas are intentionally not release-gating for `1.0`:

- `action-flow-view`
- `state-transition-view`
- `sequence-view`
- Additional visualization routing and layout quality beyond `general-view` and `interconnection-view`
- Broader SysML v2 language coverage outside the currently well-tested subset
- Deep semantic validation beyond the existing parser and graph-based support

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
- deterministic CI coverage for the release-critical editor features

It does not mean:

- every planned SysML feature is implemented
- every visualization is stable
- the full specification is covered
