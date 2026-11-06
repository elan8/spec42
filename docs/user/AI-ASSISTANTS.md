# AI assistants (Copilot, Cursor, Claude, …)

Spec42 gives chatbots **structured** SysML v2 / KerML feedback through the CLI. Raw `@workspace` text alone is not enough for reliable modeling.

## When to use what

| Surface | Best for |
| --- | --- |
| **LSP (VS Code extension)** | Human editing: live diagnostics, hover, completion, navigation, and typed feature inspection |
| **VS Code Language Model Tools** (Copilot Agent, VS Code 1.99+) | Four built-in tools via the bundled `spec42` CLI — no extra config in VS Code |
| **CLI** `spec42 check` / `doctor` / `explain-diagnostic` / `model-export` | CI, scripts, other AI hosts (Cursor, non-VS-Code Copilot), LM Tools backend |

The language server does **not** expose its graph directly to Copilot Chat. Run the CLI after substantive model edits.

### VS Code Copilot (Language Model Tools)

With VS Code **1.99+** and the [Spec42 extension](../../vscode/README.md), Copilot Agent can use four built-in tools (`#spec42Check`, `#spec42Doctor`, `#spec42ModelExport`, `#spec42ExplainDiagnostic`). The extension runs the bundled `spec42` binary (`check`, `doctor`, `explain-diagnostic`, `model-export` with JSON output).

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
4. **`spec42 model-export <path>`** — validation summary plus a read-only, `schema_version`-stamped `projection` of the workspace's resolved structure (elements with their relationship families, resolved expression trees and metadata annotations; connectors; a publication envelope). Unbounded by default; `--max-nodes` bounds the element list, and `projection.truncation` always reports the full count alongside how many were returned.

```bash
spec42 check path/to/model.sysml --format json
spec42 explain-diagnostic --code unresolved_type_reference --format json
spec42 model-export path/to/model.sysml --max-nodes 500 --format json
spec42 doctor --format json
```

Global flags: `--library-path`, `--config`, `--stdlib-path`, `--no-stdlib`.

Model-export schema 2 adds invocation, sequence and meta-cast node kinds; consumers must accept
these operations or report them as unsupported. `membership.role` identifies entry/do/exit
state subactions for both inline and reference forms. A trigger invocation's `trigger_kind`
is `when`, `at` or `after`; its `expression` tree carries the condition, instant or duration.
Read connector topology from `projection.connectors`, whose ordered `ends` retain each
feature-chain endpoint's `root`, `terminal` and full `path` with per-hop resolution outcomes.
For example, `connect left.output to right.input;` retains the `left` and `right` occurrence
roots even when both ports are inherited from shared definitions. Generic element relationships
alone do not describe that topology. End order does not by itself imply directed signal flow.

`spec42 diagram` emits `warning[empty_diagram]` on stderr when a selected view has no
exposed roots. Check its expose and filter declarations: `filter @SysML::PartUsage;`, for
example, excludes an exposed part definition. A fully resolved empty selection still reports
completeness `complete`; the warning does not alter the JSON written to stdout.

Elements retain authored `name` and `qualified_name` separately from
`effective_name` and `effective_qualified_name`. Use the effective fields to find unnamed
redefinitions; `null` means no complete effective name is settled. `multiplicity_expressions`
links to the range and its authored lower/upper expression elements (a single-bound range has
only an upper expression). `exposure` retains namespace, recursive and filtered exposure flags.
`evaluation` publishes the settled status and optional value; `analysis` publishes a verdict,
computed result or an explicit unsettled status. An unsupported or nonconstant expression is
never a failing verdict. Expression nodes distinguish invocations, sequences and meta casts;
this does not imply that the evaluator executes those operations. Generator ABI v7 continues to
represent these newer operators as explicitly unsupported nodes with their operands retained.

Expression `result_typing` reports static result types separately from evaluated values.
The strict conformance diagnostic covers bound scalar datatype values and assignments; defaults
retain the existing declaration reporting policy, while structural and unit bindings need their
own intersection/measurement rules. Computed Integer results do not become Natural merely
because their value is nonnegative.
Operator signatures come from admitted standard-library specializations; named invocations
retain their declared return type; sequence size can refine to Positive when a direct feature
operand has a proven positive lower multiplicity. Positive sums and products use static
operand-domain guarantees. Unresolved, ambiguous and unsupported typings
remain explicit.
`contextual_evaluations` identifies the requested, effective and authored elements for each
inherited expression evaluated in a usage. A false contextual constraint reports at that usage
and links to its authored declaration. Derived values use the same evaluator with contextual
bindings; cycles and nonconstant inputs remain explicit. Quantity comparisons currently fold
matching unit spellings only; unit conversion and arbitrary user calculation execution remain
unsupported. Generator ABI v7 does not expose these additional static/contextual fields.

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

Repo-level conventions for Copilot are in [`.github/copilot-instructions.md`](../../.github/copilot-instructions.md).

## Related docs

- [SUPPORTED-WORKFLOWS.md](SUPPORTED-WORKFLOWS.md) — release-gating editor and CLI workflows
- [DEVELOPMENT.md](../../DEVELOPMENT.md) — build, parser pin, validation pipeline
- [TROUBLESHOOTING.md](TROUBLESHOOTING.md) — server, libraries, indexing
- [GITHUB-ACTION.md](GITHUB-ACTION.md) — CI validation

## Future (not yet shipped)

- Chat participant "Spec42" with pinned parser/workflow instructions
- Python API for programmatic model access / digital-thread integration (scope TBD, tracked in [#52](https://github.com/elan8/spec42/issues/52))
