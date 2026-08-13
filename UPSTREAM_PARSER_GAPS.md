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

- Gap 15. Bare `feature`-keyword-led members (and the `member feature ...` visibility-prefixed
  variant) are a hard parse error -- `(code "unrecognized_declaration_in_scope")`, `(source
  "parser")` -- when nested inside any structured type body (KerML `class`/`attribute`/`structure`
  bodies, and relationship bodies), even though the identical construct is accepted (if only to
  the opaque `FeatureDecl` fallback of Gap 14) at package/namespace top level. Root cause: in the
  pinned `0757de13` checkout, `PACKAGE_BODY_GRAMMAR`/`PACKAGE_BODY_STARTERS`
  (`src/parser/grammar_scope.rs:184-290`, built by the `grammar_scope!` macro) register
  `(b"feature", Feature, Extension)` as a package-body starter, and `feature_decl()`
  (`src/parser/package.rs:706-780`, starters `&[b"feature"]` at line 708) is wired in for that
  scope -- but `ATTRIBUTE_BODY_STARTERS` (`src/parser/attribute.rs:28-49`) has no `b"feature"` (or
  `b"member"`) entry at all, so `attribute_body()` (`src/parser/attribute.rs:540-559`) falls
  through to `attribute_body_recovery`/`unexpected_keyword_in_scope_diagnostic`
  (`src/parser/diagnostics.rs:683-722`) for every nested `feature`/`member feature` member. Since
  `feature` and `member` are absent from `SYSML_RESERVED_KEYWORDS`
  (`src/parser/lex.rs:407-520` -- confirmed by direct grep, no `"feature"` entry exists there),
  `is_reserved_keyword` returns false and the diagnostic path picks `unrecognized_declaration_in_scope`
  rather than `unexpected_keyword_in_scope`, even though `feature` plainly is a grammar keyword one
  scope up. Verified end-to-end against the pinned checkout with a standalone
  `sysml_v2_parser_next::parse_for_editor` dump (not just snapshot inspection): e.g. `class A {
  feature innerSpaceDimension : Natural [1]; }` reports `msg="unrecognized declaration \`feature\`
  in attribute body" found="feature innerSpaceDimension : Natural [1];"` at the nested position.
  Blocks `test/snapshots/kerml/argument_resolution.md`, `bare_redefines_feature.md`,
  `binding_connector_bind_kw.md`, `classes.md`, `connector_references.md`, `connectors.md`,
  `coverage_features_advanced.md`, `dependencies.md`, `expressions.md`, `extended_occurrences.md`,
  `inheritance.md`, `inverses.md`, `john_individual_example.md`, `mass_rollup_1.md`,
  `mass_rollup_2.md`, `packets.md`, `product_selection_n_ary.md`,
  `product_selection_owned_ends.md`, `product_selection_unowned_ends.md`, `redefinition.md`,
  `scoping.md`, `textual_representation.md`, `time_varying_features.md`, `vehicle_tanks.md`,
  `vehicles_1.md`, `vehicles_2.md`, `vehicles_3.md`. Needs `feature`/`member` added to
  `ATTRIBUTE_BODY_STARTERS` with a nested-body-aware `feature_decl`/`kerml_feature_decl` dispatch
  arm, filed upstream against `feat/gh-119-arena-backed-references`
  (elan8/sysml-v2-parser#121).

- Gap 16. Bare `connector`-keyword-led members (as opposed to the `connect` alias) are unrecognized
  inside attribute bodies, part-definition bodies, and even package bodies. Root cause: the
  `grammar_scope!` table in `src/parser/grammar_scope.rs:184-290` registers only
  `(b"connect", Connector)` as a body-member starter (no `b"connector"` entry), and
  `ATTRIBUTE_BODY_STARTERS` (`src/parser/attribute.rs:28-49`) likewise has no `b"connector"` entry
  -- `connector.rs:127` only recognizes `feature ... to ...` as a *sub*-clause of an
  already-dispatched connector production, not as a body-member starter itself. Confirmed via
  direct parser dump: `connector a ::> a.x to b;` inside a class body reports
  `msg="unrecognized declaration \`connector\` in attribute body"`. Blocks
  `test/snapshots/kerml/argument_resolution.md`, `connector_all.md`, `connector_references.md`,
  `connectors.md`, `product_selection_n_ary.md`, `product_selection_owned_ends.md`,
  `product_selection_unowned_ends.md`, `vehicle_tanks.md`,
  `test/snapshots/sysml/examples/coverage_connectors.md`. Needs `b"connector"` added alongside
  `b"connect"` in the relevant starter tables and dispatched to the same `Connector` production,
  filed upstream against `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 17. `portion` is not a reserved keyword or a registered body-member starter anywhere in the
  pinned `0757de13` checkout: `SYSML_RESERVED_KEYWORDS` (`src/parser/lex.rs:407-520`) has no
  `"portion"` entry, and `grep -rn portion src/parser` shows the only surviving uses are the
  `OccurrencePortionKind::Snapshot`/`Timeslice` enum variants (`src/parser/occurrence_body.rs`),
  reachable only via the `snapshot`/`timeslice` keywords, not `portion` itself. Constructs such as
  `portion feature all portions: Occurrence[1..*] { ... }` and `portion redefines portionOfLife =
  ...;` therefore always fall into `unrecognized_declaration_in_scope` ("unrecognized declaration
  \`portion\` in attribute body") -- confirmed with a direct parser dump against the pinned
  checkout. Blocks `test/snapshots/kerml/bare_redefines_feature.md`, `camera.md`, `classes.md`,
  `time_varying_features.md`, `time_varying_features_enhanced.md`. Needs a `portion` keyword/
  production (KerML `Portion` usage prefix) added to the grammar, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 18. `var`-keyword-led members are unrecognized wherever they appear (all observed instances
  are nested in attribute/behavior bodies). Root cause: neither `ATTRIBUTE_BODY_STARTERS`
  (`src/parser/attribute.rs:28-49`) nor the `grammar_scope!` `PACKAGE_BODY_GRAMMAR` table
  (`src/parser/grammar_scope.rs:184-290`) register `b"var"` as a body-member starter, so recovery
  always reports `unrecognized declaration \`var\` in attribute body`. Blocks
  `test/snapshots/kerml/behaviors.md`, `expressions.md`, `extended_occurrences.md`,
  `time_varying_features.md`, `time_varying_features_enhanced.md`. Needs a `var` member production
  wired into the relevant starter tables, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 19. `composite`-prefixed feature declarations (e.g. `composite feature engine subsets
  carParts { ... }`) are unrecognized in both attribute bodies and package bodies. Root cause: no
  `b"composite"` entry exists in `ATTRIBUTE_BODY_STARTERS` (`src/parser/attribute.rs:28-49`) or the
  `grammar_scope!` `PACKAGE_BODY_GRAMMAR` table (`src/parser/grammar_scope.rs:184-290`); `composite`
  is not a recognized `FeaturePrefix`/`UsagePrefix` starter anywhere in that table (contrast with
  the neighboring `derived`/`default`/`ordered`/`nonunique` `FeaturePrefix`/`UsagePrefix` entries
  which are handled). Confirmed via direct parser dump: `msg="unrecognized declaration
  \`composite\` in attribute body"` / `"...in package body"`. Blocks
  `test/snapshots/kerml/features.md`, `filtering.md`, `mass_rollup_1.md`, `vehicle_tanks.md`,
  `vehicles_1.md`, `vehicles_2.md`, `vehicles_3.md`. Needs `composite` added as a
  `FeaturePrefix`/`UsagePrefix` starter, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 20. `step`-keyword-led action-step members are unrecognized when nested inside an attribute
  body, even though `step` is an accepted starter in other (action-scoped) body productions
  (`src/parser/package.rs:673`). Root cause: `ATTRIBUTE_BODY_STARTERS`
  (`src/parser/attribute.rs:28-49`) has no `b"step"` entry, so a `step a1 : Action1;`-style member
  nested inside an attribute/class body falls through to
  `unrecognized_declaration_in_scope` ("unrecognized declaration \`step\` in attribute body")
  instead of reaching the `step`-aware production that already exists for action bodies. Blocks
  `test/snapshots/fuzz/fuzz_succession_flow_value_no_name.md`, `test/snapshots/kerml/behaviors.md`,
  `test/snapshots/kerml/coverage_behaviors.md`. Needs `step` added to `ATTRIBUTE_BODY_STARTERS`
  with dispatch to the existing step production, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 21. Nested `class`-keyword definitions inside an attribute/class body are unrecognized, even
  though `class` is a fully supported *top-level* package-body production (`definition_prefix`
  options at `src/parser/package.rs:723`, starters including `b"class"` at line 749, and
  `(b"class", Class, Extension)` in the `grammar_scope!` table,
  `src/parser/grammar_scope.rs:278`). Root cause: `ATTRIBUTE_BODY_STARTERS`
  (`src/parser/attribute.rs:28-49`) has no `b"class"` entry, so a nested `class` definition inside
  another type's body always falls through to `unrecognized_declaration_in_scope`. Blocks
  `test/snapshots/kerml/imports.md`, `test/snapshots/kerml/john_individual_example.md`,
  `test/snapshots/kerml/scoping.md`. Needs `class` added to `ATTRIBUTE_BODY_STARTERS` with dispatch
  to the existing `class`-definition production, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 22. Several KerML explicit-relationship-declaration keywords have no `PackageBody` grammar
  production at all in the pinned `0757de13` checkout, unlike sibling relationship keywords
  (`typing`, `redefinition`) which *are* registered in the same `grammar_scope!`
  `PACKAGE_BODY_GRAMMAR` table (`src/parser/grammar_scope.rs:184-290`) and parse without error.
  Confirmed missing from that table (and from `SYSML_RESERVED_KEYWORDS`,
  `src/parser/lex.rs:407-520`, where applicable) by direct grep of `src/parser`: `type` (as in
  `type UnionType unions A, B;`), `subset` (`subset parent subsets f;`), `featuring` (`featuring F
  of y by C;`), `disjoining` (`disjoining d1 disjoint A from B;`), `specialization`, and `inverse`.
  Each produces `unrecognized_declaration_in_scope` ("unrecognized declaration \`<kw>\` in package
  body") at the point of use, verified with a direct parser dump against
  `test/snapshots/kerml/coverage_relationships.md`'s source (which exercises `type`/`disjoining`/
  `subset`/`featuring` side by side with the *working* `typing`/`redefinition` forms in the same
  file, ruling out a file-wide parse abort). Blocks `test/snapshots/kerml/classifiers.md`,
  `coverage_feature_subdecls.md`, `coverage_features_advanced.md`, `coverage_relationships.md`,
  `feature_chains.md`, `features.md`, `inverses.md`, `unicode_identifiers.md`. Needs `type`/
  `subset`/`featuring`/`disjoining`/`specialization`/`inverse` package-body-member productions
  added alongside the existing `typing`/`redefinition` ones, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 23. Bare `name;` / `name = expr;` / `name : Type;` members with **no leading keyword at all**
  (the "implicit feature" shorthand) are only dispatched in scopes that explicitly wire
  `attribute_feature_binding`/`redefinition_feature_binding` (`src/parser/attribute.rs:440-503`);
  in package bodies, relationship bodies, and metadata bodies the identical shorthand is
  unrecognized -- the leading identifier itself is treated as an unknown "declaration keyword" and
  reported as `unrecognized_declaration_in_scope` ("unrecognized declaration \`<name>\` in package
  body" / "...in metadata body"). Confirmed via direct parser dump, e.g. bare `x;` and `y = x istype
  T or x hastype z;` directly inside a `package { }` body in `kerml/classifications.md`, and bare
  `causeA;`/`effectC;`-style members inside `package { }` bodies in the SysML example fixtures.
  Blocks `test/snapshots/kerml/classifications.md`, `test/snapshots/kerml/expressions.md`,
  `test/snapshots/kerml/vehicle_definitions.md`, `test/snapshots/sysml/coverage_extended.md`,
  `test/snapshots/sysml/examples/ahfcore_lib.md`, `test/snapshots/sysml/examples/ahfnorway_topics.md`,
  `test/snapshots/sysml/examples/cause_and_effect_example.md`,
  `test/snapshots/sysml/examples/requirement_metadata_example.md`,
  `test/snapshots/sysml/examples/risk_metadata_example.md`,
  `test/snapshots/sysml/examples/sys_ml_v2_spec_annex_a_simple_vehicle_model.md`,
  `test/snapshots/sysml/examples/vehicle_analysis_demo.md`,
  `test/snapshots/sysml/examples/vehicle_usages.md`. Needs the bare-name implicit-feature grammar
  extended to package/relationship/metadata body scopes (or an explicit decision that it is
  intentionally attribute-body-only, documented upstream), filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 24. Two additional single-file constructs share the same `unrecognized_declaration_in_scope`
  mechanism but are too narrow to merit their own numbered upstream issue on their own; recorded
  here so the fixture list stays fully accounted for. (a) `expr at { ... }` / `expr while { ... }`
  anonymous-expression-block forms nested in an occurrence body are unrecognized -- no `b"expr"`
  entry in `ATTRIBUTE_BODY_STARTERS` (`src/parser/attribute.rs:28-49`) -- blocking
  `test/snapshots/kerml/extended_occurrences.md`. (b) Bare `inv <name> { ... }` (KerML invariant
  shorthand, as opposed to the supported `inv true/false` boolean-kind form) nested in an attribute
  body is unrecognized for the same reason -- blocking
  `test/snapshots/kerml/textual_representation.md`. Both need starter-table entries added the same
  way as Gaps 15/18/20, filed upstream against `feat/gh-119-arena-backed-references`
  (elan8/sysml-v2-parser#121).

## False-positive check (spec42-side surfacing bug?)
Traced end-to-end for a diverse sample (Gap 15's `feature` case, Gap 17's `portion` case, Gap 22's
`type`/`subset` case, and Gap 23's bare-identifier case) plus a repo-wide search:
`crates/sysml_resolution/src/model/resolver/writer.rs` renders `CanonicalDiagnostic::Parser` by
reading `error.code`/`error.severity` straight off the parser's own `ParseError` and hard-codes
`(source "parser")` -- a direct passthrough, not a spec42 classification. No crate in
`sysml_resolution`/`sysml_query`/`sysml_model` branches on the `unrecognized_declaration_in_scope`
code string or post-processes/discards an AST node alongside it (the only repo hit for that string,
`crates/sysml_model/tests/kerml_relationship_projection.rs`, is itself a test asserting the parser
recovery is a single diagnostic with *no* AST variant produced, i.e. confirming there is nothing
for spec42 to have discarded). All 51 fixtures were re-verified with a standalone
`sysml_v2_parser_next::parse_for_editor` dump against each fixture's isolated `SOURCE` text,
confirming every occurrence carries `(severity error)` and `(source "parser")` and that the parser
itself -- not spec42 -- is the origin of the diagnostic. **Conclusion: no spec42-side surfacing bug
found; all 51 fixtures are genuine upstream parser gaps**, grouped into Gaps 15-24 above.

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
