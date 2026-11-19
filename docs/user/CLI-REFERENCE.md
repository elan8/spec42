# CLI reference

Everything the editor analyses is also available from the `spec42` command, with the same analysis
engine. This page describes each command. Run `spec42 --help` for the command list and
`spec42 <command> --help` for every option of one command. To install the tool, see
[Getting started](GETTING-STARTED.md#install-the-command-line-tool).

## Commands

| Command | Purpose |
| --- | --- |
| [`check`](#check) | Validate a model file or workspace and write a report. |
| [`explain-diagnostic`](#explain-diagnostic) | Explain a diagnostic code and list matching instances. |
| [`doctor`](#doctor) | Show the configuration, standard library and Sysand status Spec42 resolved. |
| [`init`](#init) | Create a validated starter workspace. |
| [`diagram`](#diagram) | List a model's diagram views, or render one as SVG or JSON. |
| [`model-export`](#model-export) | Export the validation summary and the resolved structure as JSON. |
| [`generate`](#generate) | Run a sandboxed WebAssembly generator against a model. |
| [`bundle`, `unbundle`](#bundle-and-unbundle) | Create and unpack KPAR library archives. |
| [`stdlib`, `libraries`, `sysand`](#stdlib-libraries-and-sysand) | Inspect the standard library, managed libraries and Sysand integration. |
| `lsp` | Run the language server. The VS Code extension starts it for you; you rarely run it by hand. |

## Exit codes

| Code | Meaning |
| --- | --- |
| `0` | The command succeeded. For `check`, there are no errors; warnings alone do not fail it. |
| `1` | `check` found errors, or warnings with `--warnings-as-errors`. |
| `2` | The command could not run, for example because a path does not exist or an argument is invalid. The reason is written to standard error. |
| `10`–`15` | Only `generate`; see [its exit codes](#generate). |

## Options shared by most commands

| Option | Effect |
| --- | --- |
| `--config <FILE>` | Use this configuration file. |
| `--library-path <DIR>` | Add a library root. Repeatable. |
| `--stdlib-path <DIR>` | Use this standard library instead of the bundled copy. |
| `--no-stdlib` | Do not load the standard library. Use it only when you mean to validate without it. |
| `--kpar-library-path <ID=PATH>` | Override the install path of a managed KPAR library, for example `domain=C:/libs/domain`. Repeatable. An id that is not a registered library registers an extra KPAR library. |
| `--project-library <RESOURCE=PATH>` | Bind an authored project resource identity to a local KPAR archive. Repeatable. |
| `--disable-kpar-library <ID>` | Disable a registered KPAR library by id, for example `domain`. Repeatable. |
| `-V`, `--version` | Print the version (before a command). |
| `-h`, `--help` | Print help. |

The `SPEC42_CONFIG_DIR`, `SPEC42_DATA_DIR` and `SPEC42_STDLIB_PATH` environment variables relocate
the user configuration directory, the data directory and the standard library. See
[Troubleshooting](TROUBLESHOOTING.md) for how Spec42 resolves the standard library.

## check

```text
spec42 check <PATH> [options]
```

Validates a `.sysml` or `.kerml` file or a workspace directory.

| Option | Effect |
| --- | --- |
| `--workspace-root <DIR>` | The workspace root to resolve imports against. Pass it when checking one file inside a multi-file project. |
| `--format <FORMAT>` | `text` (default), `json`, `sarif` or `junit`. |
| `--warnings-as-errors` | Treat warnings as errors, so they fail the command. |
| `--baseline <FILE>` | Filter out the diagnostics in a prior JSON report, as a migration aid. |
| `--strict-diagnostics` | Legacy mode that skips semantic diagnostics after parse errors and suppresses shadowed unresolved warnings. |

```bash
spec42 check model --workspace-root .
spec42 check model --format sarif > spec42.sarif
```

A clean run ends with a summary line and exits with `0`:

```text
Checked 4 document(s): 0 error(s), 0 warning(s), 0 info(s)
```

See [Validation and diagnostics](VALIDATION.md) for the report formats, baselines and how follow-on
diagnostics are reported.

## explain-diagnostic

```text
spec42 explain-diagnostic --code <CODE> [options]
```

Explains a diagnostic code: what it means and the usual fix.

| Option | Effect |
| --- | --- |
| `--code <CODE>` | The diagnostic code, for example `unresolved_type_reference`. Required. |
| `--path <PATH>` | A file or directory to validate, listing the diagnostics that match the code. |
| `--line <N>` | With `--path`, keep only instances on this 0-based line. |
| `--workspace-root <DIR>` | The workspace root, as for `check`. |
| `--format <FORMAT>` | `json` (default) or `text`. |

```text
$ spec42 explain-diagnostic --code unresolved_type_reference --format text
code: unresolved_type_reference
severity: warning
alignment: spec_constraint
meaning: A type name on a usage or feature does not resolve to a known definition in the workspace or libraries.
typical fix: Add or import the missing definition, fix the qualified name, or configure library paths / standard library.
```

## doctor

```text
spec42 doctor [options]
```

Shows what Spec42 resolved on this machine: the configuration file and directories, the standard
library it will use and its roots, the managed libraries and Sysand detection. Run it first when
results differ from what you expect. `--format json` gives the same information for scripts.

```text
spec42 <version>
mode: doctor
config file: (none)
config dir: <user configuration directory>
data dir: <user data directory>
resolved stdlib: <data dir>/standard-library/versions/2026-04/kpar
stdlib library roots (10):
  ...
```

## init

```text
spec42 init <PATH>
```

Creates a starter workspace in a new or empty directory and runs a semantic check before reporting
success. It never overwrites existing files.

```text
$ spec42 init my-model
Created starter SysML v2 workspace at my-model (6 files; validation passed).
```

## diagram

```text
spec42 diagram <PATH> [--view <NAME>] [options]
```

Without `--view`, lists the model's authored diagram views as `<view kind>  <qualified name>`.
With `--view`, renders that view.

| Option | Effect |
| --- | --- |
| `--view <NAME>` | The qualified name of the view to render, for example `Shop::fulfilment`. |
| `--kind <KIND>` | The view kind to render (`general-view`, `interconnection-view`, …), for a view typed by more than one view definition. The listing shows the kinds. |
| `--document <DOC>` | The document that declares the view, as listed or as a path suffix such as `views/Views.sysml`, when a qualified name is declared in more than one document. |
| `--format <FORMAT>` | `svg` (default) renders the view. `json` writes its diagram product, or lists the views as JSON when no view is given. |
| `-o`, `--output <FILE>` | Write to this file instead of standard output. |
| `--depth <N>` | For general and interconnection views: how many levels of members to draw below the exposed elements (default `1`; `all` draws every level). |
| `--stats` | Report the size of the view's diagram product instead of rendering it. |

```text
$ spec42 diagram model
state-transition-view  Views::timerStateMachine
$ spec42 diagram model --view Views::timerStateMachine -o timer.svg
```

A selected view with no exposed roots produces a `warning[empty_diagram]` on standard error. See
[Diagram views](DIAGRAM-VIEW.md) for the view kinds.

## model-export

```text
spec42 model-export <PATH> [options]
```

Writes the validation summary and a read-only, schema-versioned JSON export of the resolved
structure. The top-level keys are `workspace_root`, `summary` and `projection`.

| Option | Effect |
| --- | --- |
| `--max-nodes <N>` | Bound the returned element list. `projection.truncation` always reports the full count. Unbounded by default. |
| `--library-elements <MODE>` | Which non-workspace elements to include: `none` (default), `referenced` or `all`. |
| `--format <FORMAT>` | `json` (default). |

```bash
spec42 model-export model --max-nodes 500 --format json
```

The [model export reference](../reference/MODEL-EXPORT.md) describes the fields.

## generate

```text
spec42 generate <GENERATOR> <PATH> [options] [-- <GENERATOR_ARGS>...]
```

Runs a sandboxed WebAssembly generator, a core module implementing the Spec42 generator ABI, against
the resolved model and writes the files it produces. Arguments after `--` are passed to the
generator unchanged.

| Option | Effect |
| --- | --- |
| `--output <DIR>` | The generator's output directory (default `generated`). |
| `--check` | Compare the generated bytes with the files on disk without writing. Exits `15` on drift. |
| `--dry-run` | Print the proposed operations without writing. |
| `--force` | Allow overwriting files that the previous generation manifest does not own. |
| `--format <FORMAT>` | `text` (default), `json`, `sarif` or `junit`. |
| `--strict-diagnostics` | As for `check`. |

Limits keep a generator contained:

| Option | Default | Limits |
| --- | --- | --- |
| `--max-memory-bytes` | 268435456 | The generator's linear memory. |
| `--max-fuel` | unmetered | The generator's instruction budget. Supplying it also reports `fuel_consumed`. |
| `--timeout-seconds` | none | Wall-clock time. |
| `--max-files` | 1000 | The number of files written. |
| `--max-file-bytes` | 16777216 | The size of one file. |
| `--max-total-bytes` | 134217728 | The size of all files together. |

Exit codes:

| Code | Meaning |
| --- | --- |
| `10` | The model could not be loaded or has validation errors, so nothing was generated. |
| `11` | The generator artifact is invalid or built for an incompatible generator API. |
| `12` | The generator reported an error, trapped or was cancelled. |
| `13` | The generator exceeded a resource limit. |
| `14` | The output was refused by the output policy. |
| `15` | `--check` found that the files on disk differ from the generated output. |

## bundle and unbundle

```text
spec42 bundle [DIRECTORY] [options]
spec42 unbundle <ARCHIVE> [options]
```

`bundle` creates a validated local KPAR archive from a model directory (the current directory by
default). `unbundle` validates an archive and unpacks it atomically.

| Option | Command | Effect |
| --- | --- | --- |
| `-o`, `--output <FILE>` | `bundle` | The archive to create. Defaults to `{name}-{version}.kpar`, taken from `.project.json`. |
| `--exclude <PREFIX>` | `bundle` | Exclude an archive-path prefix. Repeatable. By default every SysML and KerML file is included. |
| `--archive-prefix <PREFIX>` | `bundle` | Add a path prefix to every source file in the archive. |
| `--no-compress` | `bundle` | Store the source bytes without ZIP compression. |
| `-d`, `--directory <DIR>` | `unbundle` | The destination directory. |

```text
$ spec42 bundle my-model -o my-model.kpar
Bundled my-model into my-model.kpar
$ spec42 unbundle my-model.kpar -d unpacked
Unbundled 4 source file(s) into unpacked
```

## stdlib, libraries and sysand

Each of these reports on the libraries Spec42 resolved. `stdlib` and `libraries` have the same three
subcommands:

| Subcommand | Effect |
| --- | --- |
| `status` | Show the pinned and installed version and whether the library is ready. |
| `path` | Print the resolved library directory. |
| `clear-cache` | Delete the materialized files from the data directory. They are re-created from the copy embedded in the binary on next use. |

`stdlib` covers the SysML v2 standard library; `libraries` covers the managed KPAR libraries such as
the Elan8 domain libraries.

```text
$ spec42 stdlib status
pinned version: 2026-04
installed version: 2026-04
install path: <data dir>/standard-library/versions/2026-04/kpar
ready: yes
source: bundled
```

`spec42 sysand status` reports whether the optional Sysand package manager is installed and the
project root it detected. See [Projects and libraries](LIBRARIES.md) for how Spec42 uses Sysand.
