# Spec42 Troubleshooting

This guide now covers both the CLI and the VS Code extension.

## Start With `spec42 doctor`

Run:

```bash
spec42 doctor
```

Check:

- which config file is in use
- which standard-library path was resolved
- whether the resolved library paths actually exist
- whether the CLI fell back to a legacy VS Code standard-library install

If you need machine-readable output:

```bash
spec42 doctor --format json
```

## CLI Validation Looks Different From The Editor

Run:

```bash
spec42 check path/to/model-or-workspace
```

Common causes:

- the CLI and the editor are resolving different library roots
- no standard library is installed for CLI use
- a custom `--stdlib-path` or `--library-path` is missing
- the workspace root used by the CLI is too narrow

What to do:

1. Compare `spec42 doctor` with your editor setup.
2. If needed, pass `--workspace-root` explicitly.
3. Add `--library-path` or `--stdlib-path` explicitly to confirm the issue.
4. Install the managed standard library with `spec42 stdlib install`.

## No Standard Library Is Found

Check:

```bash
spec42 stdlib status
spec42 stdlib path
```

What `spec42` tries, in order:

1. explicit CLI flags
2. environment variables
3. explicit config file
4. default user config
5. managed `spec42` install
6. legacy VS Code standard-library install location

What to do:

1. Run `spec42 stdlib install`.
2. If you already have a library checkout, pass `--stdlib-path /path/to/sysml.library`.
3. Use `--no-stdlib` only when you intentionally want to validate without it.

## Server Does Not Start In VS Code

Common causes:

- `spec42.serverPath` points to a file that does not exist
- the bundled server binary is missing from the extension package
- the configured binary is not executable

What to do:

1. Check `spec42.serverPath`.
2. Use an absolute path if you are pointing at a custom build.
3. Open `SysML: Show SysML Output (Logs)`.
4. Restart the server after fixing the path.

## Missing Hover, Definition, Or References

Common causes:

- the server is still indexing
- the file contains invalid intermediate syntax
- the required library roots are not included in `spec42.libraryPaths`

What to do:

1. Wait until the status bar no longer shows `Indexing`.
2. Check diagnostics in the current document.
3. Validate `spec42.libraryPaths` and remove broken paths.
4. Restart the server if the document was already open during a crash.

## Workspace Results Are Incomplete

Workspace indexing uses `spec42.workspace.maxFilesPerPattern`.

What to do:

1. Increase `spec42.workspace.maxFilesPerPattern` for larger repositories.
2. Re-run `SysML: Refresh SysML Model Explorer`.
3. Use smaller focused workspaces if indexing becomes too slow.

## Visualizer Or Model Explorer Looks Wrong

Possible causes:

- the workspace model is only partially indexed
- the current view is experimental
- the visualizer is stale after file changes

What to do:

1. Run `SysML: Refresh SysML Model Explorer`.
2. Run `SysML: Refresh Visualization`.
3. If needed, enable `spec42.logging.verbose` and inspect the SysML output channel.
