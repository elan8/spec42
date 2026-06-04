# Spec42 / SysML v2 modeling (Copilot)

This repository uses **Spec42** for SysML v2 and KerML. Follow these rules when editing or generating models.

## Language and structure

- Prefer **packages** at the root of `.sysml` files; nest `part def` / `part usage` inside packages unless the file is intentionally a single-package document.
- Use **SysML v2 textual notation** only; do not invent KerML/SysML v1 syntax.
- After substantive edits, validate with MCP **`spec42_check`** or CLI `spec42 check` on the file or directory.

## Diagnostics and fixes

- Use diagnostic **`code`** fields from the validation report; do not guess fixes from message text alone.
- For code meaning and typical fixes, call MCP **`spec42_explain_diagnostic`** with the `code` (and optional `path` / `line`).
- If many diagnostics are `unresolved_type_reference`, `unresolved_import_target`, or `missing_library_context`, run MCP **`spec42_doctor`** before changing model text.

## Libraries and workspace

- Models that import `ScalarValues`, `ISQ`, or other standard packages need the Spec42 **bundled standard library** or explicit `--stdlib-path` / MCP `stdlib_path`.
- Multi-file workspaces: pass **`workspace_root`** to `spec42_check` when validating a single file.
- Configure extra libraries via `spec42.libraryPaths` (VS Code) or MCP `library_paths` / CLI `--library-path`.

## What not to do

- Do not assume Copilot’s `@workspace` index understands SysML semantics.
- Do not “fix” parse errors by deleting valid spec-aligned structure without re-running validation.
- Do not commit secrets or local absolute paths into shared models.

See [docs/AI-ASSISTANTS.md](../docs/AI-ASSISTANTS.md) for MCP setup and tool details.
