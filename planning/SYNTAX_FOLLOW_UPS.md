# Syntax-fidelity follow-ups

Active record of consumer code that still derives SysML syntax answers from source text instead of
asking the syntax service. Each cluster lands as one change: add the typed query to the syntax
service, migrate the callers, delete the heuristic, and remove its exemption entry from
`crates/sysml_query/tests/syntax_authority.rs`. Exemptions exist only while an entry is listed here.

## Quick-fix text scans

`crates/language_service/src/code_actions.rs` still parses declaration headers from line text,
counts braces to find a container's extent, and answers "does this definition already exist" by
scanning the lines of the same file.

The queries it needs exist now — `declaration_at`, `enclosing_declarations`, `body_range`,
`typed_by` — so what is left is the migration and one behaviour decision: the public suggesters
take `source: &str`, and a publication-wide "definition already exists" check needs the published
model rather than the file's own lines. Both are signature changes through
`lsp_server::language`, which is why this did not land with the rest of the outline work. The
file will still spell declaration keywords afterwards, because the text it *writes* (`part def X
{ }`) is presentation, so the exemption narrows rather than disappears.

## Name-only token roles

`crates/sysml_tokens/src/ast_ranges.rs` narrows a wide declaration span to the declared name by
searching the line for `def ` and the package-like keywords. Retiring it needs the syntax service
to publish the *name* span as its own role, which is a change to the semantic-token collector
(`sysml_resolution::syntax::token_ranges`) with an intended golden diff over published tokens —
a reviewed change of its own, not a side effect of an outline query.

## Recovery search over short names

`recover_short_name_search_symbols` (`crates/language_service/src/library_search.rs`) recovers
`<shortName>` declarations from library documents the publication did not admit. The outline now
publishes `short_name`, but only for declarations whose AST node carries an identification — and
this path exists for documents whose parse *failed*, where the outline is empty or partial. It
needs either short names on every declaration form or a recovery-aware query; until then the
text scan is the only answer for an unparseable file.

## A container accessor on published qualified names

`crates/language_service/src/navigation.rs` and `crates/language_service/src/symbol.rs` split a
*published* qualified name on `::` to show its container. That is a gap in the published name
type, not in the syntax service: the fix is a container/segments accessor on the publication's
qualified name, which belongs with the semantic contract.

## Recovery as a diagnostic category

`crates/sysml_diagnostics/src/postprocess.rs` tests parser diagnostic codes for the `recovered_`
prefix to order and suppress recovery cascades. `SyntaxDiagnosticCategory` cannot carry the
answer, because by the time these are `SemanticDiagnostic`s the code is the *publication's*
`DiagnosticCode`, not the parser's. What this needs is a published "produced by recovery" fact on
the diagnostic contract.

## Other

- Ask upstream `sysml-v2-parser` to export its reserved-keyword table so the service's copy can be
  derived rather than pinned by count.
- Split `kpar` into archive-format and package-naming halves if a SysML-free provisioning crate is
  ever required.
