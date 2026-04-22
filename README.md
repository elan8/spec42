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

## What You Use It For

- `spec42 lsp`: run the language server over stdio for editors
- `spec42 check <path>`: validate a file or recursively validate a folder in CI or local automation
- `spec42 doctor`: inspect resolved config, library paths, and standard-library state
- `spec42 generate ros2 ...`: generate an `ament_python` ROS2 scaffold from SysML models
- `spec42 stdlib ...`: inspect or print the resolved SysML standard library; `clear-cache` removes materialized files (they are re-created from the embedded copy on next use)

Bare `spec42` still starts the LSP server for backward compatibility with existing editor integrations.

## Features

- **LSP**: text sync, diagnostics, hover, completion, go to definition, find references, rename, document symbols, workspace symbol search, code actions, formatting, folding ranges, selection ranges, document links, linked editing, call hierarchy, and type hierarchy.
- **Workspace-aware**: features operate across `.sysml` and `.kerml` files in your workspace and configured library roots.
- **CLI validation**: deterministic file/folder validation without needing to speak JSON-RPC.
- **Bundled standard library**: the official `sysml.library` release is embedded in the `spec42` binary and materialized to your data directory when needed, with compatibility fallback to a legacy VS Code-managed install location.
- **VS Code extension**: snippets, semantic tokens, Model Explorer, Model Visualizer, and standard editor workflows.

## CLI Quick Start

```bash
spec42 --help
spec42 --version
spec42 doctor
spec42 check ./examples/timer/KitchenTimer.sysml
spec42 check ./workspace --format json
spec42 generate ros2 --input ./domain-libraries/business/robotics/examples/inspection-rover/inspection-rover.sysml --output ./generated
spec42 stdlib status
spec42 stdlib path
```

### Validation

`spec42 check` is the supported automation interface.

- It accepts either a single file or a folder.
- Folder input recursively discovers `.sysml` and `.kerml` files.
- It prints human-readable diagnostics by default.
- `--format json` returns machine-readable output.
- It exits nonzero when error-severity diagnostics are found or when CLI setup fails.

Examples:

```bash
spec42 check ./models/timer/KitchenTimer.sysml
spec42 check ./models/workspace --workspace-root ./models/workspace
spec42 check ./models/workspace --format json
```

### ROS2 Generation

`spec42 generate ros2` runs validation preflight, extracts ROS2 generation data from typed SysML model elements, and emits a deterministic `ament_python` package scaffold.

Examples:

```bash
spec42 generate ros2 --input ./domain-libraries/business/robotics/examples/inspection-rover/inspection-rover.sysml --output ./generated
spec42 generate ros2 --input ./domain-libraries/business/robotics/examples/inspection-rover --output ./generated --package-name inspection_rover_bringup --force
spec42 generate ros2 --input ./domain-libraries/business/robotics/examples/inspection-rover/inspection-rover.sysml --output ./generated --dry-run
```

Generated outputs include:

- `package.xml`, `setup.py`, `setup.cfg`, and package/resource boilerplate
- `launch/*.launch.py` from `RosLaunchDescription` models
- `config/*.yaml` parameter files from `RosParameterProfile` + `RosParameter` models
- interface placeholders when modeled (`msg/srv/action`)
- `traceability.json` linking generated artifacts to model element references

Failure modes:

- validation errors stop generation and return a non-zero exit code
- existing output directories require `--force` unless `--dry-run` is used
- missing modeled fields produce placeholder artifacts when possible and warnings in command output

### Standard Library

`spec42` resolves libraries in this order:

1. CLI flags such as `--library-path`, `--stdlib-path`, or `--no-stdlib`
2. environment variables such as `SPEC42_LIBRARY_PATHS` and `SPEC42_STDLIB_PATH`
3. an explicit `--config` file
4. the default user config file
5. materialized data from a previous run (including the embedded standard library)
6. **embedded** standard library (materialized from the binary on first use)
7. the legacy VS Code standard-library install location

Typical workflow:

```bash
spec42 doctor
spec42 check ./models/timer/KitchenTimer.sysml
```

Useful commands:

```bash
spec42 stdlib status
spec42 stdlib path
spec42 stdlib clear-cache
```

### Doctor

`spec42 doctor` prints the resolved runtime environment for CLI and editor troubleshooting:

- current binary version
- config file in use
- config/data directories
- resolved standard-library path and source (including **bundled** when materialized from the embedded copy)
- whether the source is the canonical on-disk install or a compatibility fallback
- whether legacy VS Code fallback was used
- resolved library paths and whether each one exists

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

Related ROS2 intent docs:

- [ROS2 MVP Spec](internal_docs/ROS2_MVP_SPEC.md)
- [ROS2 Executable Model Pattern](internal_docs/ROS2_EXECUTABLE_MODEL_PATTERN.md)

## License

MIT. See [LICENSE](LICENSE). The embedded SysML standard library is subject to separate licensing; see [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
