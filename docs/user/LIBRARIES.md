# Library & Dependencies

Spec42 bundles the SysML v2 standard library plus the Elan8 domain libraries. For exact release pins, see [What's included](../reference/WHATS-INCLUDED.md).

## Standard Library

The SysML v2 standard library is bundled with Spec42 — no separate installation needed. It provides base definitions like `Part`, `AttributeValue`, `ScalarValue`, SI units, and more.

The **Library** view in the Spec42 sidebar shows a dashboard for bundled libraries, configured custom paths, and Sysand dependency status.

You can:

- Search for symbols by name, package, or type.
- Browse indexed library content when you do not know the exact name yet.
- Open a symbol definition directly in the editor.
- Copy a qualified name or import statement from a result.
- Open details for the standard library and the bundled domain libraries.

## Domain Libraries

Elan8 domain libraries extend the standard library with reusable vocabulary for domain and technical modeling (for example robotics, software, electronics, and communication). They are bundled and available automatically.

See the [Domain libraries](../reference/DOMAIN-LIBRARIES.md) reference for the bundled version and the KPAR package/file tree.

## Custom Libraries

You can add your own library paths in the **Library** section:

1. Click the **Manage custom library paths** button (gear icon) in the **Library** view.
2. Add the folder path containing your `.sysml` or `.kerml` library files.
3. The language server restarts and indexes the new library.

If a configured path is missing, the Library dashboard shows it as a warning so you can fix the path quickly.

## Project scope and Sysand dependencies

A `.project.json` identifies a model project and declares its dependencies through `usage`
resource identities and optional `versionConstraint` values. Nested manifest projects are separate
analysis scopes. The configured KerML/SysML standard-library baseline is always preserved; an
explicit standard-library usage constrains that baseline instead of selecting another copy.

Put source-selection rules in `syside.toml` at the project root (the editor workspace folder for a
manifestless project):

```toml
include = ["model/**", "library/**"]
exclude = ["**/scratch/**", "**/draft.sysml"]
# Optional: the installed environment, relative to this directory.
# sysand-dir = ".sysand"
```

Patterns use `/` separators and are relative to that directory. `*` matches within one path
component; `**` can cross directories. Exclusions win. An omitted `include` admits all model files
allowed by the existing filesystem ignore policy; `include = []` admits none. Only `.sysml` and
`.kerml` files are model sources. The installed environment is excluded from workspace sources.
Source selection is shared by the CLI and VS Code; opening or changing an excluded file does not
add it to the editor's project publication. VS Code watches these project inputs and rebuilds the
project when they change, retaining included unsaved buffers and clearing excluded diagnostics.

This supports the project-root `include`, `exclude`, and `sysand-dir` subset of
[Syside configuration](https://docs.sensmetry.com/editor/configuration/). Global and
`syside.user.toml` merging and other Syside settings are not implemented. `sysand.toml` is not a
source-scope manifest and arbitrary strings in it are never treated as library paths.

[Sysand](https://github.com/sensmetry/sysand) can install the dependencies declared by a project.
Run dependency installation explicitly (for example, `sysand sync`) before analysis. Spec42 reads
`sysand-lock.toml` and the installed environment's `env.toml`; it never downloads packages or
invokes installation during analysis. No Sysand executable is required to read an existing install.

- Only dependencies requested by the owning manifest and their locked transitive closure are
  admitted. Unrelated installed projects are ignored.
- Exact resource identities and installed paths come from Sysand metadata, rather than directory
  names or project display names. Lock-selected versions must still satisfy the authored version
  constraints and override higher generic catalog candidates.
- Missing or ambiguous installations, changed project identities/usages, invalid paths, malformed
  metadata, and unsupported format versions are explicit project-admission errors.
- Editable paths are resolved relative to the environment's parent (the Sysand workspace root).
  Non-editable paths are relative to the
  environment and must remain inside it, including after symlink resolution.

The reader supports resource-form `.project.json` usages, Sysand lock versions `0.5`/`0.6`, and
installed-environment versions `0.1`/`0.2`. Other usage forms, including Sysand-specific typed index
and directory usages, are not yet supported by the KPAR project decoder and fail explicitly.
The external format baseline is
[sensmetry/sysand at `5d0b2d8`](https://github.com/sensmetry/sysand/tree/5d0b2d8de33c0bcc1e1c0fe6849b93b446e7a8f5/core/src).

The **Library** panel shows Sysand availability and detected dependencies. **Refresh** restarts the
language server after dependencies are installed or updated. Manifest, lock, source configuration,
and environment metadata changes inside workspace folders are also watched automatically.
Use **Refresh** for an environment configured outside the editor workspace.
