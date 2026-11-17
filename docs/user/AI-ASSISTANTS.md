# AI assistants

Spec42 gives AI assistants such as Copilot, Cursor and Claude **structured** SysML v2 / KerML feedback through the CLI. Raw `@workspace` text alone is not enough for reliable modeling.

## When to use what

| Surface | Best for |
| --- | --- |
| **LSP (VS Code extension)** | Human editing: live diagnostics, hover, completion, navigation, and typed feature inspection |
| **VS Code Language Model Tools** (Copilot Agent, VS Code 1.99+) | Four built-in tools via the bundled `spec42` CLI — no extra config in VS Code |
| **CLI** `spec42 check` / `doctor` / `explain-diagnostic` / `model-export` | CI, scripts, other AI hosts (Cursor, non-VS-Code Copilot), LM Tools backend |

The language server does **not** expose its graph directly to Copilot Chat. Run the CLI after substantive model edits.

### VS Code Copilot (language model tools)

With VS Code **1.99+** and the [Spec42 extension](../../vscode/README.md), Copilot Agent can use four built-in tools (`#spec42Check`, `#spec42Doctor`, `#spec42ModelExport`, `#spec42ExplainDiagnostic`). The extension runs the bundled `spec42` binary (`check`, `doctor`, `explain-diagnostic`, `model-export` with JSON output).

Requirements:

- Open a workspace folder (or a `.sysml` / `.kerml` file) so tool `when` clauses match.
- Optional: set `spec42.libraryPaths` like for the language server.

### Other hosts (Cursor, Copilot without VS Code LM Tools, …)

Point the host at the `spec42` binary directly, driven by a per-host skill/instructions doc (see [`.github/copilot-instructions.md`](../../.github/copilot-instructions.md) for the pattern). There is no MCP server or HTTP API — every surface, including VS Code LM Tools, calls the same CLI JSON output described below.

## CLI commands (agent workflow)

See the [CLI reference](CLI-REFERENCE.md) for every command and option. Recommended order when debugging a workspace:

1. **`spec42 doctor`** — standard library, config dirs, library paths, Sysand detection
2. **`spec42 check`** — validation report (`summary.error_count`, per-file `diagnostics[].code`, `advice`)
3. **`spec42 explain-diagnostic --code <code>`** — stable explanation for a diagnostic code (optional concrete instances via `--path` + `--line`)
4. **`spec42 model-export <path>`** — validation summary plus a read-only, `schema_version`-stamped `projection` of the workspace's resolved structure (elements with their relationship families, resolved expression trees and metadata annotations; connectors; a publication envelope). Unbounded by default; `--max-nodes` bounds the element list, and `projection.truncation` always reports the full count alongside how many were returned.

```bash
spec42 check path/to/model.sysml --format json
spec42 explain-diagnostic --code unresolved_type_reference --format json
spec42 model-export path/to/model.sysml --max-nodes 500 --format json
spec42 doctor --format json
```

Global flags: `--library-path`, `--config`, `--stdlib-path`, `--no-stdlib`.

For the full field-by-field description of the export, see the [model export reference](../reference/MODEL-EXPORT.md).

### `spec42 check` response

- **`summary`**: `error_count`, `warning_count`, `document_count`
- **`documents[]`**: `uri`, `diagnostics[]` with `code`, `message`, LSP `range`
- **`advice`**: environment hints (e.g. missing standard library)

Use `summary.error_count` to decide if the model is clean.

## Agent workflow

1. Edit `.sysml` / `.kerml` in the editor (or let the agent patch files).
2. Run **`spec42 check`** on the changed file or project directory; pass **`--workspace-root`** when validating a single file inside a multi-file project.
3. For each distinct **`code`**, use **`spec42 explain-diagnostic`** if the fix strategy is unclear.
4. If many `unresolved_*` diagnostics appear, run **`spec42 doctor`** before rewriting imports or types.
5. For structure (what specializes what, which elements carry a metadata tag, connector topology), read `model-export --format json`'s `projection`. It is the resolved publication, not a guess from source text; branch on `projection.schema_version`.

## Related guides

- [Features and support](SUPPORTED-WORKFLOWS.md): release-gating editor and CLI workflows
- [Validation and diagnostics](VALIDATION.md): the `spec42 check` command and its reports
- [Troubleshooting](TROUBLESHOOTING.md): server, libraries, indexing
- [GitHub Actions](GITHUB-ACTION.md): CI validation
- [Model export reference](../reference/MODEL-EXPORT.md): the JSON written by `model-export`
