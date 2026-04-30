# Spec42

Language tooling for [SysML v2](https://www.omg.org/sysml/sysmlv2/) and KerML. `spec42` now ships as both:

- an LSP server for editors
- a CLI for validation, troubleshooting, and standard-library diagnostics

![SysML v2](https://img.shields.io/badge/SysML-v2.0-blue)
![VS Code Extension](https://img.shields.io/badge/VS%20Code-Extension-007ACC?logo=visual-studio-code)
[![GitHub Release](https://img.shields.io/github/v/release/elan8/spec42?label=GitHub%20Release)](https://github.com/elan8/spec42/releases)
[![License](https://img.shields.io/github/license/elan8/spec42)](LICENSE)

[![Install from Marketplace](https://img.shields.io/badge/Install-VS%20Code%20Marketplace-007ACC?logo=visual-studio-code)](https://marketplace.visualstudio.com/items?itemName=Elan8.spec42)
[![Download Releases](https://img.shields.io/badge/Download-GitHub%20Releases-24292F?logo=github)](https://github.com/elan8/spec42/releases)

## What It Is

`spec42` is a language tooling stack for teams working with SysML v2 and KerML.
It provides:

- a production LSP server for editor experiences
- a validation and diagnostics runtime for local checks and CI
- standard-library management that works reliably across environments

In short, `spec42` helps you edit, understand, and validate models with consistent behavior from developer workstation to automation pipeline.

## What It Can Do

- **Build trust in model quality early** with live diagnostics while editing and deterministic validation in CI.
- **Understand large systems faster** with navigation and cross-reference workflows (definitions, references, symbols, and hierarchies).
- **Work in multiple SysML v2 views** by combining textual modeling with structural exploration in Model Explorer and graphical views in Model Visualizer.
- **Stay productive across real workspaces** with analysis across `.sysml` and `.kerml` files plus configured library roots.
- **Onboard reliably across environments** with embedded standard-library support and robust resolution behavior.
- **Troubleshoot environment issues quickly** with resolved runtime, config, and library diagnostics when setups differ.

### Supported SysML v2 Views (Current)

- **General View**: a high-level structural view to quickly understand the main elements in a model and how they relate.
- **Interconnection View**: a connection-focused view for inspecting parts, ports, and connectors across the system architecture.
- **Action Flow View**: a behavior-oriented view for following control and data flow through actions in a process.
- **State Transition View**: a lifecycle view that shows states and transitions so you can reason about system behavior over time.

## Usage Model

Most users interact with `spec42` in one of two ways:

- **Inside an editor** through the language server and extension features.
- **Inside automation** through validation-oriented CLI workflows for CI and scripted quality gates.

You can still run `spec42 --help` to see command-level details, but the core value is the shared analysis engine behind both interactive editing and automated validation.

## Installing

Install the VS Code extension from the Marketplace:

- [SysML v2 Editor (Elan8.spec42)](https://marketplace.visualstudio.com/items?itemName=Elan8.spec42)

Download binaries from [Releases](https://github.com/elan8/spec42/releases):

1. **VS Code**: install the `.vsix` from the Extensions view.
2. **Server / CLI**: download the archive for your OS, extract it, and put `spec42` on your PATH.

## Building

```bash
cargo build --release
cd vscode && npm install && npm run compile
```

```bash
cd zed
cargo build --target wasm32-wasip2 --release
```

For development details, see [DEVELOPMENT.md](DEVELOPMENT.md). For troubleshooting, see [docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md). For extension usage and settings, see [vscode/README.md](vscode/README.md).

## License

MIT. See [LICENSE](LICENSE). The embedded SysML standard library is subject to separate licensing; see [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).


