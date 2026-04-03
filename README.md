# Spec42

Language server for [SysML v2](https://www.omg.org/sysml/sysmlv2/) (and KerML). Provides LSP over stdio and a VS Code extension.

![SysML v2](https://img.shields.io/badge/SysML-v2.0-blue)
![VS Code Extension](https://img.shields.io/badge/VS%20Code-Extension-007ACC?logo=visual-studio-code)
[![GitHub Release](https://img.shields.io/github/v/release/elan8/spec42?label=GitHub%20Release)](https://github.com/elan8/spec42/releases)
[![License](https://img.shields.io/github/license/elan8/spec42)](LICENSE)

[![Install from Marketplace](https://img.shields.io/badge/Install-VS%20Code%20Marketplace-007ACC?logo=visual-studio-code)](https://marketplace.visualstudio.com/items?itemName=Elan8.spec42)
[![Download Releases](https://img.shields.io/badge/Download-GitHub%20Releases-24292F?logo=github)](https://github.com/elan8/spec42/releases)

## Features

- **LSP**: text sync, diagnostics, hover, completion, go to definition, find references, rename, document symbols, workspace symbol search, code actions, formatting, folding ranges, selection ranges, document links, linked editing, call hierarchy, and type hierarchy.
- **Workspace-aware**: all features use `.sysml` and `.kerml` files across the workspace (and library paths).
- **Library paths + managed standard library**: configure `spec42.libraryPaths` for custom roots and use the extension-managed pinned SysML v2 standard library support.
- **VS Code extension**: TextMate fallback highlighting plus semantic tokens, snippets, Feature Inspector, Model Explorer, Model Visualizer, workspace indexing feedback, and frontend-rendered diagram views/export (General + Interconnection stable; additional views optional/experimental).

## Available Today

Today `spec42` focuses on dependable core editing and navigation workflows:

- editor navigation: go to definition, references, rename, document/workspace symbols
- hierarchy workflows: call hierarchy and type hierarchy
- editor assistance: hover, completion, linked editing, document links, folding, selection ranges, TextMate fallback highlighting, and semantic tokens
- workspace-aware model exploration with the Feature Inspector, Model Explorer, and visualizer workflows

## Sample Workspaces

Recommended examples under [`vscode/testFixture/workspaces`](vscode/testFixture/workspaces):

- [`single-file`](vscode/testFixture/workspaces/single-file): realistic surveillance-drone architecture with parts, ports, requirements, and connections
- [`state-view`](vscode/testFixture/workspaces/state-view): compact state-machine example for behavior-focused editing and visualization
- [`multi-file`](vscode/testFixture/workspaces/multi-file): minimal cross-file definition/usage workspace for navigation and indexing checks

## Installing

Install from the VS Code Marketplace:

- [SysML v2 Editor (Elan8.spec42)](https://marketplace.visualstudio.com/items?itemName=Elan8.spec42)

Download from [Releases](https://github.com/elan8/spec42/releases):

1. **VS Code**: install the `.vsix` (Extensions → "…" → Install from VSIX). The server binary is bundled.
2. **Server only** (other editors): download the archive for your OS, extract, and put the binary on PATH.

## Building

```bash
cargo build --release          # Rust server → target/release/spec42
cd vscode && npm install && npm run compile   # VS Code extension
```

For development and testing details, see [DEVELOPMENT.md](DEVELOPMENT.md). For extension usage/configuration, snippets, and sample-workspace guidance, see [`vscode/README.md`](vscode/README.md).

## License

MIT. See [LICENSE](LICENSE).
