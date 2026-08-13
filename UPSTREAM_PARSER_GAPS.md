# Upstream sysml-v2-parser gaps blocking spec42 snapshot work

Tracks semantic gaps discovered while closing the snapshot delta on the new parser-owned
pipeline (branch `closing-the-gap`, PR lukewilliamboswell/spec42#6) that trace back to the
pinned `sysml-v2-parser-next` revision rather than to `sysml_resolution`/`sysml_query`. Each
entry should carry enough detail to file/update an upstream issue against
`feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

## Open

- Gap 13. Bare forward-declared `classifier X;` (and `classifier X specializes Y;`, `classifier X
  [1] specializes Y disjoint from Z;`, etc.) still collapses to the raw-text fallback node
  `ClassifierDecl { keyword: String, text: String }` in the pinned `0757de13` checkout (see
  `src/ast/kerml_fallback.rs`). Re-verified directly against the checkout while implementing
  KerML `class def` lowering: unlike the sibling `ClassDef` (full-body `class` with `{ }`, gap #2,
  now resolved with typed `identification`/`specializes`/`body`/`membership` fields), this
  no-body/semicolon-terminated `classifier` form has no typed name, membership, or specialization
  fields at all -- only the raw keyword and the full source text of the declaration. There is no
  way to lower this to a named declaration with resolvable specialization without re-parsing the
  captured `text`, which spec42 deliberately avoids (parser-owned resolution boundary). Currently
  routed through the generic `unsupported_package_member` fallback in
  `crates/sysml_resolution/src/model.rs` (`PackageBodyElement::ClassifierDecl` arm). Blocks
  `test/snapshots/kerml/a_2_atoms.md`, `a_2_modeling_instances.md`, and other fixtures using bare
  `classifier` declarations from resolving name/specialization facts for those declarations.
  Needs a typed `ClassifierDecl` shape (e.g. `identification`/`specializes`/`membership` fields
  mirroring `ClassDef` minus the body) filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 14. Bare KerML `feature x : Integer;` (`PackageBodyElement::FeatureDecl`, and its
  `KermlFeatureDecl` sibling for the bodied form) still collapses to the raw-text fallback node
  `FeatureDecl { keyword: String, text: String }` in the pinned `0757de13` checkout (see
  `src/ast/kerml_fallback.rs` and `feature_decl()`/`kerml_feature_decl()` in
  `src/parser/package.rs`). Re-verified directly against the checkout while attempting to lower
  the top-level `feature` gap previously (incorrectly) marked resolved as Gap 1 below: unlike
  `ItemDef`/`ItemUsage` (typed `identification`/`specializes`/`type_name`/`redefines`/`body`
  fields), this construct has no typed name, typing, specialization, subsetting, or redefinition
  fields at all -- only the leading keyword and the full captured source text of the declaration.
  There is no way to lower this to a named declaration with resolvable
  typing/specialization/subsetting/redefinition facts without re-parsing the captured `text`,
  which spec42 deliberately avoids (parser-owned resolution boundary). Currently routed through
  the generic `unsupported_package_member` fallback in `crates/sysml_resolution/src/model.rs`
  (`PackageBodyElement::FeatureDecl`/`PackageBodyElement::KermlFeatureDecl` arms). Blocks
  `test/snapshots/feature_typing.md` and the ~86 other fixtures using bare `feature`
  declarations (see `kerml/feature_chains.md`, `kerml/a_3_8_changing_feature_values.md`, etc.)
  from resolving name/typing/specialization facts for those declarations. Needs a typed
  `FeatureDecl` shape (e.g. `identification`/`type_name` (or a `TypingRelationship`)/`subsets`/
  `redefines`/`membership`/`body` fields mirroring `ItemUsage`) filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

## Resolved / not blocked (kept for history)
- Gap 1. Top-level `feature` declarations were unparsed grammar. Originally recorded as resolved
  upstream in 0757de13 (the raw `unsupported_grammar_form` parser diagnostic is indeed gone, and
  `feature x : Integer;` now parses without a parser-level error). **Correction (re-verified
  while attempting to lower it for `sysml_resolution`):** the resulting AST node
  (`PackageBodyElement::FeatureDecl`) is still a raw/opaque fallback (`{ keyword: String, text:
  String }`, no name/typing/specialization fields), the same pattern as Gap 10/13's
  `ClassifierDecl` correction -- the parser no longer *rejects* the grammar, but it also doesn't
  produce a typed node `sysml_resolution` can lower without re-parsing text. Re-tracked as Gap 14
  above.
- Gap 2. Class specialization (`:>`) inside a `class` body was unparsed. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 3. `CalcDef` dropped the parsed `:>` specialization clause. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 4. `ConstraintUsage` dropped the parsed `:>`/`:>>` specialization clauses. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 5. `AnalysisCaseUsage`/`CaseUsage` dropped the parsed `:>`/`:>>` specialization clauses. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 6. `InterfaceUsage` had no `subsets`/`redefines` fields. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 7. `individual <kind> <name>;` short usage forms were misparsed/unparseable for `item`/`occurrence`/`port`. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 8. `ViewUsage` had no `subsets` field. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 9. `ConcernUsage` had no `specializes`/`subsets`/`redefines` field. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 10. Bare forward-declared `classifier X;` collapsed to a raw-text fallback node. **Correction (re-verified while implementing `class def` lowering):** this specific struct (`ClassifierDecl`) is still a raw/opaque fallback (`{ keyword: String, text: String }`, no name/membership/specialization fields) in `0757de13` -- the earlier "resolved" note conflated it with the separate, now-genuinely-resolved `ClassDef` (gap #2). Re-opened and re-tracked as Gap 13 above.
- Gap 11. `item <name> : <Type>;` nested in an attribute body was captured opaquely. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).
- Gap 12. `#<keyword> def <Name> ...` ExtendedDefinition short form had no grammar production. Resolved upstream in 0757de13 -- confirmed via direct AST/parser inspection of the pinned checkout (typed fields/nodes/dispatch now present); wired into `sysml_resolution` lowering where applicable (see commits on `closing-the-gap`).


- Alias declarations (`alias X for Y;`) — investigated as a possible parser gap, but the typed
  AST (`AliasDef.target`) was already a structured `QualifiedReferenceId`. Fixed entirely in
  `sysml_resolution` (commit `422e2216`), not a parser gap.
- Enum definitions (`enum def`, `EnumeratedValue`) — investigated, typed AST already exposes
  stable per-literal identity/spans. Fixed entirely in `sysml_resolution` (commit `99d5ea39`).
