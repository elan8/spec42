# Spec42

Modern language tooling for [SysML v2](https://www.omg.org/sysml/sysmlv2/) and KerML.
`spec42` gives systems engineers a smoother path from model authoring to validation by shipping the same analysis engine as:

- an LSP server for editors
- a CLI for validation, troubleshooting, and standard-library diagnostics

![SysML v2](https://img.shields.io/badge/SysML-v2.0-blue)
![VS Code Extension](https://img.shields.io/badge/VS%20Code-Extension-007ACC?logo=visual-studio-code)
[![License](https://img.shields.io/github/license/elan8/spec42)](LICENSE)

[![Install from Marketplace](https://img.shields.io/badge/Install-VS%20Code%20Marketplace-007ACC?logo=visual-studio-code)](https://marketplace.visualstudio.com/items?itemName=Elan8.spec42)
[![Download Releases](https://img.shields.io/badge/Download-GitHub%20Releases-24292F?logo=github)](https://github.com/elan8/spec42/releases)

## Why Spec42

SysML v2 is powerful, but practical modeling work depends on fast feedback, reliable library resolution, and editor workflows that make large models navigable.
`spec42` focuses on that day-to-day experience:

- **Write with confidence** using live diagnostics, semantic highlighting, completion, hover, and navigation.
- **Understand model structure quickly** with symbols, references, hierarchy features, Model Explorer, and Model Visualizer.
- **Validate the same way locally and in CI** with the `spec42 check` command.
- **Avoid standard-library setup friction** with bundled SysML library support and `spec42 doctor` diagnostics.
- **Integrate with Sysand when present** so package-managed dependencies can participate in library resolution without making Sysand mandatory.
- **Publish deterministic artifacts** with CI-friendly validation formats and Rust-owned SVG/JSON diagram export; routed SysML views use vendored ELK.js through embedded QuickJS.
- **Learn by example** with compact SysML v2 models that progress from workstation and timer examples to richer software and drone systems.

In short, `spec42` helps you edit, understand, and validate models with consistent behavior from developer workstation to automation pipeline.

## Quick Start

### VS Code

1. Install [SysML v2 Editor (Elan8.spec42)](https://marketplace.visualstudio.com/items?itemName=Elan8.spec42).
2. Open a `.sysml` or `.kerml` file.
3. Use the command palette to open **SysML: Show SysML Model Explorer** or **SysML: Open SysML Visualizer**.

Published extension builds include the `spec42` server binary for simpler onboarding.

### CLI

Download a release from [GitHub Releases](https://github.com/elan8/spec42/releases), put `spec42` on your `PATH`, then run:

```bash
spec42 doctor
spec42 check examples/timer/KitchenTimer.sysml
spec42 check examples/timer/KitchenTimer.sysml --format sarif
spec42 sysand status --format json
spec42 diagrams export examples/office --view general-view --format svg --output target/diagrams
```

Use `spec42 doctor` first when library paths, editor setup, or CI behavior differ from what you expect.

### GitHub Action

Use the bundled GitHub Action to validate models in CI:

```yaml
permissions:
  contents: read
  security-events: write

jobs:
  spec42:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - uses: elan8/spec42@v0.26.2
        with:
          path: examples/timer/KitchenTimer.sysml
          format: sarif
          warnings-as-errors: true
```

See [docs/GITHUB-ACTION.md](docs/GITHUB-ACTION.md) for inputs, SARIF upload behavior, and advanced usage.

## What It Can Do

- **Build trust in model quality early** with live diagnostics while editing and deterministic validation in CI.
- **Understand large systems faster** with navigation and cross-reference workflows (definitions, references, symbols, and hierarchies).
- **Work in multiple SysML v2 views** by combining textual modeling with structural exploration in Model Explorer and graphical views in Model Visualizer.
- **Stay productive across real workspaces** with analysis across `.sysml` and `.kerml` files plus configured library roots.
- **Onboard reliably across environments** with embedded standard-library support and robust resolution behavior.
- **Troubleshoot environment issues quickly** with resolved runtime, config, and library diagnostics when setups differ.
- **Track conformance transparently** through the generated [`docs/CONFORMANCE-MATRIX.md`](docs/CONFORMANCE-MATRIX.md).
- **Export diagrams headlessly** as JSON payloads or SVG; General, Interconnection, Action Flow, and State Transition SVG use ELK routing, while Sequence, Browser, Grid, and Geometry remain deterministic native exports.

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

## Learn With Examples

Start with the examples if you are evaluating `spec42` or learning SysML v2:

| Example | Best For |
| --- | --- |
| [`examples/office`](examples/office/README.md) | Smallest first read: parts, ports, connections, simple behavior. |
| [`examples/timer`](examples/timer/README.md) | Recommended first substantial model and flagship validation example. |
| [`examples/intersection`](examples/intersection/README.md) | Controller and state-machine behavior in a familiar system. |
| [`examples/webshop`](examples/webshop/README.md) | Software architecture, interactions, requirements, and views. |
| [`examples/drone`](examples/drone/README.md) | Broader system decomposition with mission behavior and requirements. |

The [`domain-libraries`](domain-libraries/README.md) directory contains reusable SysML v2 library content for software, communication, electronics, robotics, and cross-cutting concerns.

## Installing

Install the VS Code extension from the Marketplace:

- [SysML v2 Editor (Elan8.spec42)](https://marketplace.visualstudio.com/items?itemName=Elan8.spec42)

Download binaries from [Releases](https://github.com/elan8/spec42/releases):

1. **VS Code**: install the `.vsix` from the Extensions view.
2. **Server / CLI**: download the archive for your OS, extract it, and put `spec42` on your PATH.

After installing a binary, verify the environment with:

```bash
spec42 doctor
```

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
