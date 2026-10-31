# Validation and Diagnostics

Run the same Spec42 analysis in your editor and in automation. `spec42 check` validates a SysML/KerML file or a workspace directory and can emit reports for people and CI systems.

## Check a workspace

Run validation from the repository root, passing the model directory and an explicit workspace root when useful:

```bash
spec42 check model --workspace-root .
```

The command exits successfully when there are no errors. By default, warnings and information diagnostics are reported but do not fail the command. Make warnings blocking in a quality gate with:

```bash
spec42 check model --warnings-as-errors
```

## Choose a report format

`check` supports `text`, `json`, `sarif`, and `junit` output. For example, write a SARIF report for a CI integration:

```bash
spec42 check model --format sarif > spec42.sarif
```

Reports identify the affected document, location, severity, and message. The text, JSON, and SARIF formats also carry the diagnostic code; use that code as the stable starting point for investigation rather than parsing display text.

For a known, intentionally accepted diagnostic set, pass a prior JSON validation report as a baseline:

```bash
spec42 check model --baseline known-diagnostics.json
```

The baseline filters matching diagnostics from the current report; it is a migration aid, not a substitute for fixing newly introduced problems.

## Investigate a diagnostic

Ask the CLI for the meaning and typical fix of a code, optionally including a model path to list matching instances:

```bash
spec42 explain-diagnostic --code unresolved_type_reference --path model
```

Common next checks are to correct a type or package path, include the defining file in the workspace, or configure the library that owns an external type. The editor can also offer a relevant quick fix when one is available.

## How follow-on diagnostics are reported

One mistake can make later analysis fail too. Spec42 reports such cascades by these rules, identically in the editor and in `spec42 check` unless a rule says otherwise:

- **Parse errors are reported where recovery starts.** Each carries source `parser` and severity error. The parser keeps every declaration it can recover, so a document with a parse error is still analysed and its semantic diagnostics are still reported. The exception today is an unclosed body (`{` without its `}`): that document publishes no declarations, and only the missing-brace error is reported for it.
- **Nothing is dropped by default.** Apart from exact duplicates (same range, severity, code and message), the editor and `spec42 check` show every diagnostic the analysis produced, so both surfaces show the same set for the same files.
- **`--strict-diagnostics` hides reference cascades.** With this flag, `spec42 check` drops `unresolved_type_reference`, `unresolved_import_target`, `unresolved_specializes_reference`, `unresolved_reference`, `ambiguous_reference` and `ambiguous_import_target` diagnostics that start on a line before the document's first parse error. The editor never applies this rule.
- **Each participant in a cycle is reported.** Every declaration in a specialization cycle gets its own `specialization_cycle` error, so the cycle can be broken at any of them.
- **Fix the earliest error first when recovery dominates.** When a document's report is mostly parser recovery, `spec42 check` adds advice to fix the earliest error in each file first; that usually removes the rest.

## Running in CI

For GitHub Actions, use the [Spec42 GitHub Action](GITHUB-ACTION.md) — it installs the release-matched CLI, forwards the check path, workspace root, report format, warning policy, baseline, and library configuration, and can upload SARIF to GitHub Code Scanning.

Keep one validation command authoritative for both local and CI use. A directory-level check is usually the right command for a multi-file model because it catches broken cross-file imports and relationships that an isolated file cannot resolve.
