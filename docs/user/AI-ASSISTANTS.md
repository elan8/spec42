# AI assistants (Copilot, Cursor, Claude, …)

Spec42 gives chatbots **structured** SysML v2 / KerML feedback through the CLI. Raw `@workspace` text alone is not enough for reliable modeling.

## When to use what

| Surface | Best for |
| --- | --- |
| **LSP (VS Code extension)** | Human editing: live diagnostics, hover, completion, navigation, diagrams |
| **VS Code Language Model Tools** (Copilot Agent, VS Code 1.99+) | Four built-in tools via the bundled `spec42` CLI — no extra config in VS Code |
| **CLI** `spec42 check` / `doctor` / `explain-diagnostic` / `model-summary` | CI, scripts, other AI hosts (Cursor, non-VS-Code Copilot), LM Tools backend |

The language server does **not** expose its graph directly to Copilot Chat. Run the CLI after substantive model edits.

### VS Code Copilot (Language Model Tools)

With VS Code **1.99+** and the [Spec42 extension](../../vscode/README.md), Copilot Agent can use four built-in tools (`#spec42Check`, `#spec42Doctor`, `#spec42ModelSummary`, `#spec42ExplainDiagnostic`). The extension runs the bundled `spec42` binary (`check`, `doctor`, `explain-diagnostic`, `model-summary` with JSON output).

Requirements:

- Open a workspace folder (or a `.sysml` / `.kerml` file) so tool `when` clauses match.
- Optional: set `spec42.libraryPaths` like for the language server.

### Other hosts (Cursor, Copilot without VS Code LM Tools, …)

Point the host at the `spec42` binary directly, driven by a per-host skill/instructions doc (see [`.github/copilot-instructions.md`](../../.github/copilot-instructions.md) for the pattern). There is no MCP server or HTTP API — every surface, including VS Code LM Tools, calls the same CLI JSON output described below.

## CLI commands (agent workflow)

Recommended order when debugging a workspace:

1. **`spec42 doctor`** — standard library, config dirs, library paths, Sysand detection
2. **`spec42 check`** — validation report (`summary.error_count`, per-file `diagnostics[].code`, `advice`)
3. **`spec42 explain-diagnostic --code <code>`** — stable explanation for a diagnostic code (optional concrete instances via `--path` + `--line`)
4. **`spec42 model-summary <path>`** — compact semantic graph (nodes + selected relationships), not full AST

```bash
spec42 check path/to/model.sysml --format json
spec42 explain-diagnostic --code unresolved_type_reference --format json
spec42 model-summary path/to/model.sysml --max-nodes 500 --format json
spec42 doctor --format json
```

Global flags: `--library-path`, `--config`, `--stdlib-path`, `--no-stdlib`.

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
5. For structural questions ("what connects to X?"), use **`spec42 model-summary`** with a modest `--max-nodes`.

Repo-level conventions for Copilot are in [`.github/copilot-instructions.md`](../../.github/copilot-instructions.md).

## Related docs

- [SUPPORTED-WORKFLOWS.md](SUPPORTED-WORKFLOWS.md) — release-gating editor and CLI workflows
- [DEVELOPMENT.md](../../DEVELOPMENT.md) — build, parser pin, validation pipeline
- [TROUBLESHOOTING.md](TROUBLESHOOTING.md) — server, libraries, indexing
- [GITHUB-ACTION.md](GITHUB-ACTION.md) — CI validation

## Future (not yet shipped)

- Chat participant "Spec42" with pinned parser/workflow instructions
- Python API for programmatic model access / digital-thread integration (scope TBD, tracked in [#52](https://github.com/elan8/spec42/issues/52))
