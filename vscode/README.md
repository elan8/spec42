<p align="center">
  <img src="https://raw.githubusercontent.com/elan8/spec42/HEAD/vscode/media/screenshot.png" alt="Spec42 SysML v2 Editor Screenshot" />
</p>

# Spec42 VS Code Extension

Spec42 brings practical [SysML v2](https://www.omg.org/sysml/sysmlv2/) and KerML language tooling to VS Code. It is built around the same `spec42` analysis engine used by the CLI, GitHub Action, HTTP API, and assistant integrations, so editor feedback and automated validation stay aligned.

Published Marketplace builds include the matching language server binary. Most users can install the extension, open a `.sysml` or `.kerml` file, and start modeling without a separate CLI setup.

## Highlights

- Live diagnostics, semantic highlighting, completion, hover, snippets, and formatting.
- Definitions, references, rename, document symbols, call hierarchy, and type hierarchy.
- Workspace-first indexing for multi-file models and configured libraries.
- Model Explorer for structural navigation.
- Model Visualizer for supported SysML v2 graphical views.
- Bundled SysML standard-library support through the `spec42` server.
- Built-in Copilot Language Model Tools for validation and model inspection.

## Getting Started

1. Install **SysML v2 Editor** from the Visual Studio Marketplace.
2. Open a `.sysml` or `.kerml` file.
3. Try `examples/timer/KitchenTimer.sysml` if you want a compact first model.
4. Run **SysML: Show SysML Model Explorer** to browse the model.
5. Run **SysML: Open SysML Visualizer** to inspect graphical views.

Useful commands:

- `SysML: Show SysML Model Explorer`
- `SysML: Open SysML Visualizer`
- `SysML: Refresh SysML Model Explorer`
- `SysML: Refresh Visualization`
- `SysML: Validate Model`
- `SysML: Show SysML Output (Logs)`

The Spec42 sidebar also includes example and library views for opening recommended sample workspaces and inspecting configured library state.

## Supported Views

| View | Purpose |
| --- | --- |
| General View | High-level structure and relationships across a model. |
| Interconnection View | Parts, ports, connectors, and architecture wiring. |
| Action Flow View | Control and data flow through actions in a behavior. |
| State Transition View | States and transitions for lifecycle-oriented behavior. |
| Sequence View | Lifelines and messages for Spec42 `SequenceView` models. |
| Browser, Grid, and Geometry Views | Provisional standard-view renderers while upstream SysML graphical notation settles. |

All visualizer views use the shared diagram renderer from `shared/diagram-renderer`. The legacy SysML renderer fallback has been removed.

## Extension Settings

Core settings:

| Setting | Default | Purpose |
| --- | --- | --- |
| `spec42.serverPath` | `spec42` | Path to the `spec42` server binary. Published builds prefer the bundled binary when this stays at the default. |
| `spec42.libraryPaths` | `[]` | Additional library roots indexed for hover, references, completion, and validation. |
| `spec42.workspace.maxFilesPerPattern` | `500` | Discovery cap per workspace folder and file type. |
| `spec42.startup.workspaceIndexing` | `background` | Controls when the full workspace model is loaded: `lazy`, `background`, or `eager`. |
| `spec42.visualization.exportScale` | `2` | Scale factor used for visualizer export. |
| `spec42.statusBar.enabled` | `true` | Shows Spec42 workspace status in the status bar. |
| `spec42.codeLens.enabled` | `true` | Enables Spec42 CodeLens actions where available. |
| `spec42.logging.verbose` | `false` | Enables verbose runtime logs. |
| `spec42.performanceLogging.enabled` | `false` | Emits performance-oriented logs for indexing and visualization diagnostics. |
| `spec42.debug` | `false` | Enables extension debug behavior for development and troubleshooting. |

Standard-library settings are retained for compatibility and display:

| Setting | Purpose |
| --- | --- |
| `spec42.standardLibrary.enabled` | Legacy setting; the standard library is bundled with the server. |
| `spec42.standardLibrary.version` | Display-only hint for the bundled SysML release. |
| `spec42.standardLibrary.repo` | Legacy metadata for the upstream release repository. |
| `spec42.standardLibrary.contentPath` | Legacy metadata for the release content folder. |

Example `settings.json`:

```json
{
  "spec42.libraryPaths": ["../SysML-v2-Release", "./my-company-sysml-library"],
  "spec42.startup.workspaceIndexing": "background"
}
```

## Editor And CLI

The extension focuses on interactive modeling. For terminal workflows, CI, and environment diagnostics, use the `spec42` CLI from a release archive.

```bash
spec42 doctor
spec42 check path/to/model-or-workspace
spec42 check path/to/model-or-workspace --format sarif
```

Both paths use the same analysis engine, so diagnostics should stay consistent between VS Code and automation.

## AI Assistants

With VS Code 1.99+ and Copilot Agent, the extension ships four built-in Language Model Tools:

- `#spec42Check`
- `#spec42Doctor`
- `#spec42ModelSummary`
- `#spec42ExplainDiagnostic`

These tools call the bundled `spec42` CLI; no `mcp.json` is required for the VS Code Copilot path.

For Cursor and other MCP hosts, use the `spec42-mcp` stdio server from the release archive. See [AI assistant setup](https://github.com/elan8/spec42/blob/HEAD/docs/user/AI-ASSISTANTS.md) and the [MCP configuration example](https://github.com/elan8/spec42/blob/HEAD/docs/examples/mcp-vscode.json).

## Troubleshooting

- If the server cannot start, check `spec42.serverPath` and open **SysML: Show SysML Output (Logs)**.
- If imports or library symbols do not resolve, check `spec42.libraryPaths` and run `spec42 doctor`.
- If the Model Explorer looks incomplete, increase `spec42.workspace.maxFilesPerPattern` or run **SysML: Refresh SysML Model Explorer**.
- If the visualizer appears stale, run **SysML: Refresh Visualization** and inspect the SysML output channel.

For broader guidance, see [Troubleshooting](https://github.com/elan8/spec42/blob/HEAD/docs/user/TROUBLESHOOTING.md).

## Development

After changing `shared/diagram-renderer` or webview TypeScript, rebuild the visualizer bundle:

```bash
cd vscode
npm run build:webview
```

`media/webview/visualizer.js` is generated and gitignored.

## Links

- [Source and releases](https://github.com/elan8/spec42)
- [Issues](https://github.com/elan8/spec42/issues)
- [Main README](https://github.com/elan8/spec42#readme)
- [SysML v2](https://www.omg.org/sysml/sysmlv2/)
- [SysML v2 reference libraries](https://github.com/Systems-Modeling/SysML-v2-Release)
