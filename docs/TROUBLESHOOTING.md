# Spec42 Troubleshooting

This guide covers the most common setup and runtime issues in the VS Code extension.

## Quick Checks

If the extension does not behave as expected:

1. Open the command palette and run `SysML: Show SysML Output (Logs)`.
2. Check the status bar for the current server state:
   `Starting`, `Indexing`, `Ready`, `Degraded`, `Restarting`, or `Server stopped`.
3. Run `SysML: Restart SysML Language Server`.
4. Confirm that the active file is a `.sysml` or `.kerml` document.

## Server Does Not Start

Common causes:

- `spec42.serverPath` points to a file that does not exist (or legacy `sysml-language-server.serverPath` is set to a missing file).
- The bundled server binary is missing from the extension package.
- The configured binary is not executable.

What to do:

1. Open VS Code settings and check `spec42.serverPath`.
2. If you use a custom binary, point it to an absolute path.
3. Open the SysML output channel and inspect the startup error.
4. Restart the server after fixing the path.

## Repeated Crashes or Restarts

The extension will attempt automatic restart a limited number of times. After repeated failures it stops restarting automatically and shows an error.

What to do:

1. Open `SysML: Show SysML Output (Logs)`.
2. Check whether the crash happens during startup or after editing.
3. Try reproducing in a smaller workspace or a single file.
4. If the issue is edit-related, keep the failing snippet as a regression fixture.

## Missing Hover, Definition, or References

Common causes:

- The server is still indexing.
- The file contains invalid intermediate syntax.
- The relevant library files are not included in `spec42.libraryPaths` (or legacy `sysml-language-server.libraryPaths`).

What to do:

1. Wait until the status bar no longer shows `Indexing`.
2. Check whether diagnostics are present in the current document.
3. Validate `spec42.libraryPaths` and remove broken paths.
4. Restart the server if the document was already open during a crash.

## Workspace Results Are Incomplete

Workspace indexing uses the setting `spec42.workspace.maxFilesPerPattern` (legacy `sysml-language-server.workspace.maxFilesPerPattern` is still supported).

What this means:

- The extension scans up to the configured number of `.sysml` files per workspace folder.
- It also scans up to the same number of `.kerml` files per workspace folder.
- If the limit is reached, the extension warns that workspace results may be incomplete.

What to do:

1. Increase `spec42.workspace.maxFilesPerPattern` for larger repositories.
2. Re-run `SysML: Refresh SysML Model Explorer` after changing the setting.
3. Use smaller focused workspaces if indexing becomes too slow.

## Model Explorer or Visualizer Looks Wrong

Possible causes:

- The workspace model is only partially indexed.
- The current view is experimental.
- The file changed while the visualizer still shows older data.

What to do:

1. Run `SysML: Refresh SysML Model Explorer`.
2. Run `SysML: Refresh Visualization`.
3. Check whether the current visualization view is experimental in the roadmap/docs.
