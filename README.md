# Spec42

Language server for [SysML v2](https://www.omg.org/sysml/sysmlv2/) (and KerML). Provides LSP over stdio and a VS Code extension.

[VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=Elan8.spec42)
[![GitHub Release](https://img.shields.io/github/v/release/elan8/spec42?label=GitHub%20Release)](https://github.com/elan8/spec42/releases)
[![License](https://img.shields.io/github/license/elan8/spec42)](LICENSE)

## Features

- **LSP**: text sync, diagnostics, hover, completion, go to definition, find references, rename, document symbols, workspace symbol search, code actions, formatting.
- **Workspace-aware**: all features use `.sysml` and `.kerml` files across the workspace (and library paths).
- **Library paths + managed standard library**: configure `spec42.libraryPaths` for custom roots and use the extension-managed pinned SysML v2 standard library support.
- **VS Code extension**: syntax highlighting, Model Explorer, and Model Visualizer with frontend-rendered diagram views/export (General + Interconnection stable; additional views optional/experimental).

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
