# Spec42

Language server for [SysML v2](https://www.omg.org/sysml/sysmlv2/) (and KerML). Provides LSP over stdio and a VS Code extension.

## Features

- **LSP**: text sync, diagnostics, hover, completion, go to definition, find references, rename, document symbols, workspace symbol search, code actions, formatting.
- **Workspace-aware**: all features use `.sysml` and `.kerml` files across the workspace (and library paths).
- **Library paths**: add roots (e.g. [SysML-v2-Release](https://github.com/Systems-Modeling/SysML-v2-Release)) to `spec42.libraryPaths` in settings for hover, go-to-definition, and completion from libraries (legacy `sysml-language-server.libraryPaths` is still supported).
- **VS Code extension**: syntax highlighting for `.sysml` / `.kerml`.

## Installing

Download from [Releases](https://github.com/elan8/spec42/releases):

1. **VS Code**: install the `.vsix` (Extensions → "…" → Install from VSIX). The server binary is bundled.
2. **Server only** (other editors): download the archive for your OS, extract, and put the binary on PATH.

## Building

```bash
cargo build --release          # Rust server → target/release/spec42
cd vscode && npm install && npm run compile   # VS Code extension
```

For development and testing details, see [DEVELOPMENT.md](DEVELOPMENT.md).

## Roadmap

- [Roadmap](docs/ROADMAP.md)

## License

MIT. See [LICENSE](LICENSE).
