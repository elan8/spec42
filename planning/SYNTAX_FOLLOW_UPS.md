# Syntax-fidelity follow-ups

Active record of consumer code that still derives SysML syntax answers from source text instead of
asking the syntax service. Each cluster lands as one change: add the typed query to the syntax
service, migrate the callers, delete the heuristic, and remove its exemption entry from
`crates/sysml_query/tests/syntax_authority.rs`. Exemptions exist only while an entry is listed here.

## Cluster A — declaration headers and outline

Typed queries to add on `ParsedSource`: `SyntaxOutlineNode.kind` as an enum with a `keyword()`
display accessor, `short_name`, `body_range`, `typed_by`, `declaration_at(line)`,
`enclosing_declarations(line)`.

Retires: the declaration-header parsers, brace counting, and same-file definition scans in
`crates/language_service/src/code_actions.rs`; `parse_untyped_part_usage_line` and
`import_statement_ranges` in `crates/lsp_server/src/common/util.rs`; the brace-folding fallback and
keyword-based signature help in `crates/lsp_server/src/lsp_runtime/features/editing_features.rs`;
the declaration-name narrowing in `crates/sysml_tokens/src/ast_ranges.rs`; the outline-kind string
matches in `crates/lsp_server/src/language/symbols.rs`; the declaration-head text split in
`crates/lsp_server/src/views/feature_inspector.rs` (a head range on the outline node).

Behaviour change to note: "definition already exists" checks become publication-wide rather than
same-file.

## Cluster B — imports and namespace references with ranges

Typed queries: `QualifiedName` with prefix keys, `ImportScope`, `SyntaxImport { target, scope,
range, owner_package }`, `imports()`, `type_references()`, `referenced_namespace_roots()`.

Retires: the `import ` line scan in `crates/lsp_server/src/lsp_runtime/navigation.rs` (the
`file://` literal-link branch stays, anchored to the import range); the bounded substring scan in
`crates/server/src/environment.rs` (keep its file budget as a parameter of the query); the `::`
split of qualified names in `crates/language_service/src/navigation.rs`.

## Cluster C — references, cursor text, completion context, recovery

Typed queries: `token_at(position)`, `unit_literal_at(position)`, a structured
`SyntaxDiagnosticCategory` for parser-recovery diagnostics; linked editing via
`navigation().references` filtered to the declaration line.

Retires: `find_reference_ranges` in `crates/language_service/src/symbol.rs` (comments and strings
stop matching); `word_at_position` and the unit-suffix detector in
`crates/language_service/src/text.rs`; the `"recovered_"` code-prefix test in
`crates/lsp_server/src/analysis/diagnostics_postprocess.rs`; `recover_short_name_search_symbols`
in `crates/lsp_server/src/session/library_search.rs` once outline `short_name` exists.
`crates/language_service/src/completion.rs` keeps its line-prefix shape detection as presentation
over text the grammar has not accepted, with keyword tables sourced from the service; a
grammar-driven `completion_context_at(position)` is a later query.

## Other

- Ask upstream `sysml-v2-parser` to export its reserved-keyword table so the service's copy can be
  derived rather than pinned by count.
- Split `kpar` into archive-format and package-naming halves if a SysML-free provisioning crate is
  ever required.
