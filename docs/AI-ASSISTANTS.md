# AI assistants (Copilot, Cursor, Claude, …)

Spec42 gives chatbots **structured** SysML v2 / KerML feedback through MCP and the CLI. Raw `@workspace` text alone is not enough for reliable modeling.

## When to use what

| Surface | Best for |
| --- | --- |
| **LSP (VS Code extension)** | Human editing: live diagnostics, hover, completion, navigation, diagrams |
| **VS Code Language Model Tools** (Copilot Agent, VS Code 1.99+) | Same four tools as MCP, via bundled `spec42` CLI — no `mcp.json` in VS Code |
| **MCP `spec42-mcp`** | Cursor and other MCP hosts; also Copilot when MCP is configured manually |
| **CLI** `spec42 check` / `doctor` / `explain-diagnostic` / `model-summary` | CI, scripts, LM Tools backend — same engine as MCP |

The language server does **not** expose its graph directly to Copilot Chat. Configure MCP (or run CLI) after substantive model edits.

## Install MCP

Release archives include `spec42-mcp` next to `spec42`. For local development:

```text
target/release/spec42-mcp
```

### VS Code Copilot (Language Model Tools)

With VS Code **1.99+** and the [Spec42 extension](../vscode/README.md), Copilot Agent can use four built-in tools (`#spec42Check`, `#spec42Doctor`, `#spec42ModelSummary`, `#spec42ExplainDiagnostic`) without MCP configuration. The extension runs the bundled `spec42` binary (`check`, `doctor`, `explain-diagnostic`, `model-summary` with JSON output).

Requirements:

- Open a workspace folder (or a `.sysml` / `.kerml` file) so tool `when` clauses match.
- Optional: set `spec42.libraryPaths` like for the language server.

### MCP (VS Code, Cursor, other hosts)

Use MCP when you want the stdio server explicitly (especially **Cursor**, which does not support VS Code Language Model Tools).

1. Install the Spec42 extension for editing (recommended).
2. Add an MCP server using the template in [`docs/examples/mcp-vscode.json`](examples/mcp-vscode.json).
3. Point `command` at your `spec42-mcp` binary (absolute path on Windows is most reliable).

### Cursor

Same stdio server; add `spec42` under MCP servers in Cursor settings, using the same `command` / `args` as in the example JSON.

## MCP tools

Recommended order when debugging a workspace:

1. **`spec42_doctor`** — standard library, config dirs, library paths, Sysand detection
2. **`spec42_check`** — validation report (`summary.error_count`, per-file `diagnostics[].code`, `advice`)
3. **`spec42_explain_diagnostic`** — stable explanation for a diagnostic code (optional concrete instances via `path` + `line`)
4. **`spec42_model_summary`** — compact semantic graph (nodes + selected relationships), not full AST

### `spec42_check` response

- **`summary`**: `error_count`, `warning_count`, `document_count`
- **`documents[]`**: `uri`, `diagnostics[]` with `code`, `message`, LSP `range`
- **`advice`**: environment hints (e.g. missing standard library)
- **`include_semantic_model`** (optional, default `false`): when `true`, adds `semantic_model` (same projection as CLI semantic validation). Prefer **`spec42_model_summary`** for large workspaces to avoid huge payloads.

Failed validation still returns HTTP-success at the MCP layer; use `summary.error_count` to decide if the model is clean.

## Agent workflow

1. Edit `.sysml` / `.kerml` in the editor (or let the agent patch files).
2. Run **`spec42_check`** on the changed file or project directory; pass **`workspace_root`** when validating a single file inside a multi-file project.
3. For each distinct **`code`**, use **`spec42_explain_diagnostic`** if the fix strategy is unclear.
4. If many `unresolved_*` diagnostics appear, run **`spec42_doctor`** before rewriting imports or types.
5. For structural questions (“what connects to X?”), use **`spec42_model_summary`** with a modest `max_nodes`.

Repo-level conventions for Copilot are in [`.github/copilot-instructions.md`](../.github/copilot-instructions.md).

## Related docs

- [SUPPORTED-WORKFLOWS.md](SUPPORTED-WORKFLOWS.md) — release-gating editor and CLI workflows
- [DEVELOPMENT.md](../DEVELOPMENT.md) — build, parser pin, validation pipeline
- [TROUBLESHOOTING.md](TROUBLESHOOTING.md) — server, libraries, indexing
- [GITHUB-ACTION.md](GITHUB-ACTION.md) — CI validation without MCP

## CLI commands (agent parity)

Same JSON shapes as MCP (pretty-printed on stdout):

```bash
spec42 explain-diagnostic --code unresolved_type_reference --format json
spec42 model-summary path/to/model.sysml --max-nodes 500 --format json
```

Global flags match MCP: `--library-path`, `--config`, `--stdlib-path`, `--no-stdlib`.

## HTTP API

A read-only HTTP server (`spec42 api serve`) exposes the same validation and semantic projections as CLI/MCP for non-agent HTTP clients. Design: [ADR 0001](adr/0001-read-only-systems-modeling-http-api.md), OpenAPI: [docs/api/spec42-readonly-v1.openapi.yaml](api/spec42-readonly-v1.openapi.yaml) (served at `GET /openapi.json`).

```bash
spec42 api serve --workspace-root ./my-model
```

Prefer **MCP or CLI** for AI assistants and CI one-shots; use HTTP when another service needs a stable network contract.

## Future (not yet shipped)

- Chat participant “Spec42” with pinned parser/workflow instructions
- MCP **`spec42_symbols`** for workspace symbol export
