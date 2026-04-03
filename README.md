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
- **VS Code extension**: syntax highlighting, Model Explorer, Model Visualizer, workspace indexing feedback, and frontend-rendered diagram views/export (General + Interconnection stable; additional views optional/experimental).

## Available Today

Today `spec42` focuses on dependable core editing and navigation workflows:

- editor navigation: go to definition, references, rename, document/workspace symbols
- hierarchy workflows: call hierarchy and type hierarchy
- editor assistance: hover, completion, linked editing, document links, folding, and selection ranges
- workspace-aware model exploration and visualizer workflows

Advanced model-intelligence surfaces such as a dedicated feature inspector or dashboard are not part of the current release surface yet.

## Installing

Install from the VS Code Marketplace:

- [SysML v2 Editor (Elan8.spec42)](https://marketplace.visualstudio.com/items?itemName=Elan8.spec42)

Download from [Releases](https://github.com/elan8/spec42/releases):

1. **VS Code**: install the `.vsix` (Extensions → "…" → Install from VSIX). The server binary is bundled.
2. **Server only** (other editors): download the archive for your OS, extract, and put the binary on PATH.

## Export GitHub code scanning findings

If you enabled GitHub code scanning (CodeQL), you can export JSON findings (requires `gh` and `gh auth login`):

```powershell
.\scripts\export-code-scanning.ps1
```

Default output directory: `.\code-scanning-export\`

- `code-scanning-alerts-<timestamp>.json` (all code scanning alerts)

## Building

```bash
cargo build --release          # Rust server → target/release/spec42
cd vscode && npm install && npm run compile   # VS Code extension
```

For development and testing details, see [DEVELOPMENT.md](DEVELOPMENT.md). For extension usage/configuration, see [`vscode/README.md`](vscode/README.md).

## Roadmap

- [Roadmap](docs/ROADMAP.md)

## License

MIT. See [LICENSE](LICENSE).
