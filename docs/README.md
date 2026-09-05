# Spec42 Documentation

Lean reference docs for Spec42. Open work lives in [GitHub Issues](https://github.com/elan8/spec42/issues) (milestone [1.0](https://github.com/elan8/spec42/milestone/1)) — do not track TODOs in Markdown.

The user guide lives here in `docs/user/` and on GitHub — there is no separate hosted docs site.

## User docs

| Topic | Document |
| --- | --- |
| Getting started | [user/GETTING-STARTED.md](user/GETTING-STARTED.md) |
| Supported product workflows | [user/SUPPORTED-WORKFLOWS.md](user/SUPPORTED-WORKFLOWS.md) |
| Diagram view | [user/DIAGRAM-VIEW.md](user/DIAGRAM-VIEW.md) |
| Feature Inspector | [user/FEATURE-INSPECTOR.md](user/FEATURE-INSPECTOR.md) |
| Libraries & dependencies | [user/LIBRARIES.md](user/LIBRARIES.md) |
| Examples | [user/EXAMPLES.md](user/EXAMPLES.md) |
| Validation and diagnostics | [user/VALIDATION.md](user/VALIDATION.md) |
| GitHub Action and CI validation | [user/GITHUB-ACTION.md](user/GITHUB-ACTION.md) |
| Troubleshooting | [user/TROUBLESHOOTING.md](user/TROUBLESHOOTING.md) |
| AI assistants, MCP, and VS Code LM Tools | [user/AI-ASSISTANTS.md](user/AI-ASSISTANTS.md) |

## Contributor docs

| Topic | Document |
| --- | --- |
| Development workflow and tests | [../DEVELOPMENT.md](../DEVELOPMENT.md) |
| 1.0 product definition | [ROADMAP.md](ROADMAP.md) |

## Architecture and API

| Topic | Document |
| --- | --- |
| System architecture (authorities, services, crate map) | [../design.md](../design.md) |
| Semantic core contracts | [architecture/SEMANTIC_CORE_ARCHITECTURE.md](architecture/SEMANTIC_CORE_ARCHITECTURE.md) |
| Read-only HTTP API | [api/README.md](api/README.md) |

## Generated reference

| Topic | Document |
| --- | --- |
| Conformance matrix | [reference/CONFORMANCE-MATRIX.md](reference/CONFORMANCE-MATRIX.md) (from [conformance-metadata.json](reference/conformance-metadata.json)) |
| SysML notation inventory | [reference/SYSML-NOTATION-INVENTORY.md](reference/SYSML-NOTATION-INVENTORY.md) (`scripts/generate-notation-inventory.mjs`) |
| What's included (Spec42 + bundled library versions) | [reference/WHATS-INCLUDED.md](reference/WHATS-INCLUDED.md) (`scripts/sync-docs-meta.mjs`) |
| Domain libraries (KPAR package/file tree) | [reference/DOMAIN-LIBRARIES.md](reference/DOMAIN-LIBRARIES.md) (`scripts/sync-docs-meta.mjs`) |
| Method libraries (KPAR package/file tree) | [reference/METHOD-LIBRARIES.md](reference/METHOD-LIBRARIES.md) (`scripts/sync-docs-meta.mjs`) |
