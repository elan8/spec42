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

## Use the maintained GitHub Action

For GitHub Actions, use the repository's [Spec42 GitHub Action](https://github.com/elan8/spec42/blob/main/docs/user/GITHUB-ACTION.md). It installs the release-matched CLI, can run `spec42 doctor`, and forwards the check path, workspace root, report format, warning policy, baseline, and library configuration. It can also upload SARIF to GitHub Code Scanning.

Keep one validation command authoritative for both local and CI use. A directory-level check is usually the right command for a multi-file model because it catches broken cross-file imports and relationships that an isolated file cannot resolve.
