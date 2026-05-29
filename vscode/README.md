<p align="center">
  <img src="https://raw.githubusercontent.com/elan8/spec42/HEAD/vscode/media/screenshot.png" alt="Spec42 SysML v2 Editor Screenshot" />
</p>

# Spec42 VS Code Extension

This extension brings a complete SysML v2 and KerML editing experience to VS Code, powered by the `spec42` language server.
It is designed for day-to-day modeling work: writing models faster, navigating large workspaces, and catching issues early while you edit.

If you are new to `spec42`, this is the recommended starting point.
Published extension builds include the language server binary, so most users can install the extension and begin modeling without separate CLI setup.

## Highlights

- rich language support for `.sysml` and `.kerml`
- workspace-aware diagnostics, navigation, and symbols
- configurable library indexing with `spec42.libraryPaths`
- bundled SysML standard library support via the `spec42` server
- snippets, semantic tokens, Model Explorer, and Model Visualizer
- graphical views for structure, interconnections, actions, and state transitions
- bundled server binary in published builds for simpler onboarding

## What You Can Do With It

- **Author models efficiently** with completion, snippets, hover details, semantic highlighting, and editing assists.
- **Catch quality issues early** with live diagnostics while you work.
- **Move confidently through large models** with definitions, references, rename, symbols, call hierarchy, and type hierarchy.
- **Use multiple SysML v2 views**: text editing for precision, Model Explorer for structure, and Model Visualizer for graphical understanding.
- **Scale to real projects** by analyzing workspace files and configured libraries together.
- **Keep familiar editor ergonomics** with formatting, code actions, linked editing, folding, and selection ranges where supported.

### Supported SysML v2 Views (Current)

- **General View**: a high-level structural view to quickly understand the main elements in a model and how they relate.
- **Interconnection View**: a connection-focused view for inspecting parts, ports, and connectors across the system architecture.
- **Action Flow View**: a behavior-oriented view for following control and data flow through actions in a process.
- **State Transition View**: a lifecycle view that shows states and transitions so you can reason about system behavior over time.

## Getting Started

1. Install the extension.
2. Open any `.sysml` or `.kerml` file.
3. Try [`examples/timer/KitchenTimer.sysml`](../examples/timer/KitchenTimer.sysml) if you want a compact first model.
4. Optionally configure `spec42.libraryPaths` for custom library roots.
5. Use the command palette to open the Model Explorer or Visualizer when needed.

Useful commands:

- `SysML: Show SysML Model Explorer`
- `SysML: Open SysML Visualizer`
- `SysML: Refresh SysML Model Explorer`
- `SysML: Refresh Visualization`
- `SysML: Show SysML Output (Logs)`

## Extension Settings

- **`spec42.serverPath`**
  - Path to the `spec42` binary.
  - Default: `"spec42"`
  - Published builds prefer the bundled binary when this stays at the default.

- **`spec42.libraryPaths`**
  - Array of library-root paths.
  - Files under these paths are indexed for hover, go-to-definition, references, and completion.

- **`spec42.standardLibrary.enabled`**
  - Legacy setting; reserved for future use. The standard library is bundled with the server.

- **`spec42.standardLibrary.version`**
  - Display-only hint for the bundled release; matches the embedded `sysml.library` in the server.
  - Default: `"2026-03"`

- **`spec42.workspace.maxFilesPerPattern`**
  - Discovery cap per workspace folder and file type.
  - Default: `500`

- **`spec42.visualization.exportScale`**
  - Scale factor used for PNG/SVG export.
  - Default: `2`

- **`spec42.visualization.useSharedRenderer`**
  - Use the shared diagram renderer (`shared/diagram-renderer`) for all **SysML** visualizer views (general, interconnection, action-flow, state-transition, sequence). Implements SysML v2 def/usage/reference node shapes and notation-neutral styling on structural views; behavior views use `shared/diagram-renderer/src/views/*`.
  - Default: `true`
  - Set to `false` to use the legacy webview renderers under `renderers/` for those views.
  - See [`docs/SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md`](../docs/SHARED-DIAGRAM-RENDERER-AND-SPEC-CONFORMANCE.md) and [`docs/SHARED-RENDERER-PARITY.md`](../docs/SHARED-RENDERER-PARITY.md).

**Developers:** after changing `shared/diagram-renderer` or webview TypeScript, rebuild the visualizer bundle:

```bash
cd vscode && npm run build:webview
```

(`media/webview/visualizer.js` is gitignored.)

- **`spec42.logging.verbose`**
  - Enable verbose runtime logs.
  - Default: `false`

Example `settings.json`:

```json
{
  "spec42.libraryPaths": ["../SysML-v2-Release", "./my-company-sysml-library"]
}
```

## Editor + CLI

The extension focuses on interactive editor workflows.
For terminal workflows (automation, CI validation, and environment diagnostics), use the `spec42` CLI.
Both experiences share the same core analysis engine, so behavior stays consistent between local editing and automated checks.

Common CLI checks:

```bash
spec42 doctor
spec42 check path/to/model-or-workspace
```

## Troubleshooting

- If the server cannot start, check `spec42.serverPath` and open `SysML: Show SysML Output (Logs)`.
- If libraries do not resolve, validate `spec42.libraryPaths` and compare with `spec42 doctor`.
- If you want a CLI view of the same environment problems, run `spec42 doctor`.
- For broader troubleshooting guidance, see [`docs/TROUBLESHOOTING.md`](../docs/TROUBLESHOOTING.md).

## Links

- Source and releases: `https://github.com/elan8/spec42`
- Issues: `https://github.com/elan8/spec42/issues`
- SysML v2: `https://www.omg.org/sysml/sysmlv2/`
- SysML v2 reference libraries: `https://github.com/Systems-Modeling/SysML-v2-Release`
