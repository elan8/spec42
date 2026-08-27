# Syntax-fidelity follow-ups

Active record of consumer code that still derives SysML syntax answers from source text instead of
asking the syntax service. Each cluster lands as one change: add the typed query to the syntax
service, migrate the callers, delete the heuristic, and remove its exemption entry from
`crates/sysml_query/tests/syntax_authority.rs`. Exemptions exist only while an entry is listed here.

## Name-only token roles

`crates/sysml_tokens/src/ast_ranges.rs` narrows a wide declaration span to the declared name by
searching the line for `def ` and the package-like keywords. Retiring it needs the syntax service
to publish the *name* span as its own role, which is a change to the semantic-token collector
(`sysml_resolution::syntax::token_ranges`) with an intended golden diff over published tokens —
a reviewed change of its own, not a side effect of an outline query.

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
