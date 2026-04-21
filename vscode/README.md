<p align="center">
  <img src="https://raw.githubusercontent.com/elan8/spec42/HEAD/vscode/media/screenshot.png" alt="Spec42 SysML v2 Editor Screenshot" />
</p>

## Spec42 VS Code Extension

This extension adds SysML v2 and KerML language support to VS Code, powered by the `spec42` language server.

The extension is for editor workflows. For automation, validation in CI, and environment inspection, use the `spec42` CLI directly:

```bash
spec42 check path/to/model-or-workspace
spec42 doctor
spec42 stdlib status
```

## Highlights

- LSP support for `.sysml` and `.kerml`
- workspace-aware navigation and diagnostics
- configurable library indexing with `spec42.libraryPaths`
- bundled SysML standard library (via the `spec42` server)
- snippets, semantic tokens, Model Explorer, and Model Visualizer
- bundled server binary in published builds

## Features

- diagnostics
- hover
- completion
- go to definition, references, rename
- document symbols and workspace symbols
- type hierarchy and call hierarchy
- folding ranges, selection ranges, document links, linked editing, semantic highlighting
- CodeLens where supported by editor/theme configuration
- code actions and formatting where supported by the server
- SysML/KerML snippets
- Model Explorer and Model Visualizer

## Getting Started

1. Install the extension.
2. Open any `.sysml` or `.kerml` file.
3. Optionally configure `spec42.libraryPaths` for custom library roots.
4. Use the command palette to open the Model Explorer or Visualizer when needed.

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
  - Default: `"2026-02"`

- **`spec42.workspace.maxFilesPerPattern`**
  - Discovery cap per workspace folder and file type.
  - Default: `500`

- **`spec42.visualization.exportScale`**
  - Scale factor used for PNG/SVG export.
  - Default: `2`

- **`spec42.logging.verbose`**
  - Enable verbose runtime logs.
  - Default: `false`

Example `settings.json`:

```json
{
  "spec42.libraryPaths": ["../SysML-v2-Release", "./my-company-sysml-library"]
}
```

## CLI + Extension

The extension still manages editor startup and editor-facing commands, but the CLI is now the preferred path for:

- validating files and workspaces outside VS Code
- checking library and stdlib resolution with `spec42 doctor`
- inspecting the resolved standard library with `spec42 stdlib status` / `spec42 stdlib path` and clearing materialized cache with `spec42 stdlib clear-cache`

This keeps VS Code focused on the editing experience while making `spec42` usable in terminals and automation too.

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
