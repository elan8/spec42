<p align="center">
  <img src="https://raw.githubusercontent.com/elan8/spec42/HEAD/vscode/media/screenshot.png" alt="Spec42 SysML v2 Editor Screenshot" />
</p>

## Spec42: SysML v2 Language Support

This extension adds **SysML v2** (and **KerML**) language support to VS Code, powered by **Spec42** (`spec42`).

If you work with **MBSE** and want fast feedback while editing models, this extension provides IDE features like diagnostics, navigation, and completions across your workspace and (optionally) your library sources.

## Highlights

- **Language Server Protocol (LSP)** features for `.sysml` and `.kerml`
- **Workspace-aware**: features work across your whole workspace, not just the active file
- **Library indexing**: point the extension to library roots (for example the SysML v2 release repo) for richer navigation and completion
- **Managed standard library** tooling for pinned SysML v2 library installs
- **Model Explorer + Model Visualizer** for model navigation and diagram inspection/export
- **Frontend-rendered diagrams** in the webview (General/Interconnection stable; additional views optional/experimental)
- **Bundled server binary** in published builds (with a safe fallback to `spec42` on your PATH)
- **Syntax highlighting** for SysML v2 / KerML

## Features

- **Diagnostics**: syntax/validation feedback as you type
- **Hover**: quick info on symbols
- **Completion**: suggestions while editing
- **Navigation**: go to definition, find references, rename
- **Symbols**: document symbols and workspace symbol search
- **Code actions & formatting**: where supported by the server
- **Model Explorer**: workspace/package tree with model navigation
- **Model Visualizer**: diagram views with SVG/PNG/JSON export and theme-aware rendering

## Getting started

1. Install the extension.
2. Open any `.sysml` or `.kerml` file.
3. (Optional) Configure library roots for better cross-file navigation and completion.
4. Use the command palette to open the Model Explorer/Visualizer when needed.

## Configuration

This extension contributes the following settings:

- **`spec42.serverPath`**
  - Path to the `spec42` binary.
  - Default: `"spec42"`
  - Notes:
    - In published builds, the extension will try to use a **bundled** server when `serverPath` is left at the default.
    - Set an absolute path (or workspace-relative path) if you want to use a custom build.

- **`spec42.libraryPaths`**
  - An array of paths to **library roots** (absolute or workspace-relative).
  - Files under these paths are indexed for hover, go-to-definition, and completion.

- **`spec42.standardLibrary.enabled`**
  - Enable managed standard library support.
  - Default: `true`

- **`spec42.standardLibrary.version`**
  - Pinned SysML v2 release tag used for managed standard library installs.
  - Default: `"2026-02"`

- **`spec42.workspace.maxFilesPerPattern`**
  - File discovery cap per workspace folder and file type for indexing.
  - Increase for larger repositories if needed.
  - Default: `500`

- **`spec42.visualization.enableExperimentalViews`**
  - Enables experimental visualizer views (for example action flow / state transition / sequence).
  - Default: `false`

- **`spec42.visualization.exportScale`**
  - Scale factor used for PNG/SVG diagram export.
  - Default: `2`

- **`spec42.logging.verbose`**
  - Enables verbose runtime logs for troubleshooting visualizer/webview/message flow.
  - Default: `false`

Example `settings.json`:

```json
{
  "spec42.libraryPaths": [
    "../SysML-v2-Release",
    "./my-company-sysml-library"
  ]
}
```

## Troubleshooting

- **The server can’t be started**
  - If the bundled server isn’t available for your platform/arch (or you’re using a dev build), install `spec42` separately and ensure it’s on your PATH, or set `spec42.serverPath` to the binary location.

- **Libraries don’t resolve**
  - Make sure each entry in `spec42.libraryPaths` points to the **root folder(s)** that contain the library sources (and that the paths are correct relative to the opened workspace).

- **Visualizer looks empty or stale**
  - Save the active SysML/KerML file and reopen the visualizer.
  - If needed, enable `spec42.logging.verbose` and check Output -> **SysML** for fetch/render diagnostics.

## Links

- Source & releases: `https://github.com/elan8/spec42`
- Issues: `https://github.com/elan8/spec42/issues`
- SysML v2: `https://www.omg.org/sysml/sysmlv2/`
- SysML v2 reference libraries: `https://github.com/Systems-Modeling/SysML-v2-Release`

