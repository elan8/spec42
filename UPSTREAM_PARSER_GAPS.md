# Upstream sysml-v2-parser gaps blocking spec42 snapshot work

Tracks semantic gaps discovered while closing the snapshot delta on the new parser-owned
pipeline (branch `closing-the-gap`, PR lukewilliamboswell/spec42#6) that trace back to the
pinned `sysml-v2-parser-next` revision rather than to `sysml_resolution`/`sysml_query`. Each
entry should carry enough detail to file/update an upstream issue against
`feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

## Open

- Gap 13. **Partially resolved upstream in `cb026cd`.** Bare forward-declared `classifier X;` with
  no `specializes`/`disjoint from`/`unions`/`intersects` clause and no body still collapses to the
  opaque `KermlBareDeclaration { keyword, name_span, multiplicity }` node (see
  `src/ast/kerml_fallback.rs`) -- it carries a name *span* but no typed identification/membership,
  so it still can't be lowered to a resolvable declaration and is routed through the generic
  `unsupported_package_member` fallback (`PackageBodyElement::KermlBareDeclaration` arm). However,
  every other shape this gap's examples named -- `classifier X specializes Y;`, `classifier X [1]
  specializes Y disjoint from Z;`, and in fact any bodied/specializing/disjoint-from/unions/
  intersects form of any KerML classifier keyword (`function`/`datatype`/`metaclass`/`struct`/
  `assoc`/`behavior`/`interaction`/`predicate`/`multiplicity`/`subclassifier`/`classifier`/`class`/
  `assoc struct`) -- now parses into the fully typed `KermlClassifierDecl` node (identification,
  `specializes: Option<Node<TypingRelationship>>`, `type_relationships`, `body: CalcDefBody`,
  `membership`), confirmed via direct AST inspection and lowered end-to-end in
  `crates/sysml_resolution/src/model.rs` (`lower_kerml_classifier_decl`,
  `DeclarationKind::KermlClassifier`). Verified with a resolution-layer probe: `classifier X;`
  alone still lowers to nothing resolvable, but `classifier Y specializes X;` lowers to a
  `kerml-classifier` declaration with a resolved `specialization` reference. Re-open a narrower
  upstream ask if the truly bare no-clause form still matters: give `KermlBareDeclaration` a typed
  name (not just a span) so it can at least get a named declaration with no relationships, filed
  upstream against `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 14. **Mostly resolved upstream in `cb026cd`.** Bare KerML `feature x : Integer;` and its
  prefixed/typed siblings (`derived`/`abstract`/`composite`/`portion`/`var`/`end`/`member`
  prefixes; `feature`/`step`/`expr`/`bool` kind keywords; `:`/`:>`/`:>>`/`references`/`chains`/
  `inverse of`/type-relationship clauses; `= expr`/`:= expr` values; `{ }` bodies) now parse into
  the fully typed `KermlFeatureMember` node (see `src/ast/kerml_fallback.rs`:
  `typing`/`subsets`/`redefines`/`value`/`body`/`membership` fields, plus `chains`/`inverse_of`/
  `type_relationships`/`references` not yet lowered) instead of the old opaque
  `FeatureDecl { keyword, text }` raw-text fallback -- confirmed via direct AST inspection and
  lowered end-to-end in `crates/sysml_resolution/src/model.rs` (`lower_kerml_feature_member`,
  `DeclarationKind::KermlFeature`), covering typing (`FeatureTyping`), `subsets`/`redefines`
  (`Subsetting`/`Redefinition`), value-expression evaluation, and owned-member structure via the
  shared `lower_calc_def_body` walker. One narrower case remains genuinely unresolved: the
  *plainest* unprefixed `feature x : Integer;` (no `derived`/`abstract`/other prefix at all) was
  observed via a resolution-layer probe to still not reach `KermlFeatureMember` (no declaration
  is produced for it at all) -- the disambiguation between the old and new productions appears to
  key off a leading prefix/kind-keyword combination not yet fully characterized. Needs the
  remaining plain-`feature`-with-no-prefix case folded into the same `KermlFeatureMember`
  production, filed upstream against `feat/gh-119-arena-backed-references`
  (elan8/sysml-v2-parser#121). `references`/`chains`/`inverse_of`/`type_relationships` facts on
  `KermlFeatureMember` are typed but not yet lowered in `sysml_resolution` -- follow-up work, not
  an upstream gap.

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
  **Re-verified against `cb026cd` (this pass):** still accurate -- `ATTRIBUTE_BODY_STARTERS`
  (`src/parser/attribute.rs:28-49`) has no `b"feature"`/`b"member"` entry, and
  `attribute_body_element`'s `alt` list (`src/parser/attribute.rs:191-259`) has no arm dispatching
  to `feature_decl`/`kerml_feature_decl`; `test/snapshots/kerml/behaviors.md`'s nested
  `in x1 = A::x;` (an inner member of a bare `feature`-led block) and every other fixture listed
  above still reports `unrecognized_declaration_in_scope`. Citation line numbers unchanged from the
  `0757de13` write-up (`src/parser/attribute.rs:28-49`, `src/parser/diagnostics.rs`,
  `src/parser/lex.rs:407-520` all still resolve to the same regions in `cb026cd`).

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
  **Re-verified against `cb026cd` (this pass):** still accurate -- `ATTRIBUTE_BODY_STARTERS`
  (`src/parser/attribute.rs:28-49`) has no `b"connector"` entry and `attribute_body_element`'s `alt`
  list has no connector-decl arm; `test/snapshots/kerml/connectors.md`,
  `test/snapshots/kerml/time_varying_car_driver.md`'s `var connector drive from engine to
  transmission;`, and the other fixtures above still report `unrecognized_declaration_in_scope`.

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
  **Re-verified against `cb026cd` (this pass):** still accurate -- `grep -rn portion
  src/parser` in the `cb026cd` checkout still surfaces only `OccurrencePortionKind::{Snapshot,
  Timeslice}`, no bare `portion` keyword production; `SYSML_RESERVED_KEYWORDS`
  (`src/parser/lex.rs:407-...`) still has no `"portion"` entry.

- Gap 18. `var`-keyword-led members are unrecognized wherever they appear (all observed instances
  are nested in attribute/behavior bodies). Root cause: neither `ATTRIBUTE_BODY_STARTERS`
  (`src/parser/attribute.rs:28-49`) nor the `grammar_scope!` `PACKAGE_BODY_GRAMMAR` table
  (`src/parser/grammar_scope.rs:184-290`) register `b"var"` as a body-member starter, so recovery
  always reports `unrecognized declaration \`var\` in attribute body`. Blocks
  `test/snapshots/kerml/behaviors.md`, `expressions.md`, `extended_occurrences.md`,
  `time_varying_features.md`, `time_varying_features_enhanced.md`. Needs a `var` member production
  wired into the relevant starter tables, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).
  **Re-verified against `cb026cd` (closing-the-gap, composite-step/var-modifiers slice):** still
  accurate -- `ATTRIBUTE_BODY_STARTERS` in the pinned checkout has no `b"var"` entry, confirmed by
  direct inspection of `src/parser/attribute.rs:28-49`. `test/snapshots/kerml/behaviors.md`'s
  `out var y1;` still reports `unrecognized_declaration_in_scope`. No `sysml_resolution` lowering
  work is possible until this lands upstream.

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
  **Re-verified against `cb026cd` (this pass):** still accurate -- no `b"composite"` entry in
  `ATTRIBUTE_BODY_STARTERS` (`src/parser/attribute.rs:28-49`) or the `grammar_scope!`
  `PACKAGE_BODY_GRAMMAR` table (`src/parser/grammar_scope.rs`); `test/snapshots/kerml/features.md`
  and the other fixtures above still report `unrecognized_declaration_in_scope`.

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
  **Re-verified against `cb026cd` (closing-the-gap, composite-step/var-modifiers slice):** still
  accurate -- `ATTRIBUTE_BODY_STARTERS` in the pinned checkout has no `b"step"` entry, confirmed by
  direct inspection of `src/parser/attribute.rs:28-49`; `KermlClassifierKeyword::Behavior` bodies
  (`src/parser/package.rs:683`) dispatch through the same attribute-body production, so
  `behavior A { ... composite step b : B { ... } }` in `test/snapshots/kerml/behaviors.md` still
  reports `unrecognized_declaration_in_scope` for the whole `step` member (range covers both the
  `composite` prefix and the nested `in x1 = A::x;` body -- neither the `composite` ownership
  modifier nor the nested qualified-reference-valued parameter is reachable for lowering while the
  member itself is opaque). No `sysml_resolution` lowering work is possible until this lands
  upstream; nothing to implement in this slice.

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
  **Re-verified against `cb026cd` (this pass):** still accurate -- `ATTRIBUTE_BODY_STARTERS`
  (`src/parser/attribute.rs:28-49`) has no `b"class"` entry and `attribute_body_element`'s `alt`
  list has no classifier-decl arm; the fixtures above still report
  `unrecognized_declaration_in_scope`. See also Gap 38 (new, this pass), which generalizes this
  same missing-dispatch pattern to `struct` and the rest of the classifier-keyword family.

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
  **Re-verified against `cb026cd` (this pass):** still accurate -- `grep -n
  '"type"\|"subset"\|"featuring"\|"disjoining"\|"specialization"\|"inverse"'
  src/parser/grammar_scope.rs` finds no entries for these keywords;
  `test/snapshots/kerml/coverage_relationships.md`'s `type B;` and the other fixtures above still
  report `unrecognized_declaration_in_scope`.

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
  **Re-verified against `cb026cd` (this pass):** still accurate -- `attribute_feature_binding`/
  `redefinition_feature_binding` (`src/parser/attribute.rs:440-503`) remain wired only into
  `attribute_body_element`; package-body/relationship-body/metadata-body dispatch (`grammar_scope.rs`,
  `RELATIONSHIP_BODY_STARTERS` in `src/parser/lex.rs:170`, `METADATA_BODY_STARTERS` in
  `src/parser/attribute.rs:69-80`) still has no bare-name-shorthand arm; the fixtures above still
  report `unrecognized_declaration_in_scope`.

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

- Gap 25. `ViewpointUsage` (`src/ast/view.rs`) has no `subsets`/`redefines` fields at all --
  `struct ViewpointUsage { name, type_name, body: RequirementDefBody, membership }` -- unlike its
  sibling `ViewUsage`, which was fixed for this exact gap class (Gap 8, resolved upstream in
  `0757de13`: `ViewUsage` now carries `subsets`/`redefines`/`multiplicity`). Verified directly
  against the pinned `0757de13` checkout while attempting `viewpoint` usage-side lowering
  (following `04274711`'s def-side `viewpoint def` work as the template). Without a
  `SubsettingRelationship` field there is no way to lower a `viewpoint` usage member to a
  declaration with resolvable specialization facts consistent with every sibling usage kind
  (`ViewUsage`, `RequirementUsage`, etc.), so it is left routed through the existing
  `unsupported_*_member` fallback wherever `PackageBodyElement::ViewpointUsage` etc. appear.
  Needs `subsets`/`redefines` fields added to `ViewpointUsage` mirroring `ViewUsage`, filed
  upstream against `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).
  **Update (exhaustive `unsupported_package_member` audit, this pass):** the plain `name`/
  `type_name`/body shape (no `subsets`/`redefines` clause) is now lowered
  (`lower_viewpoint_usage`, `crates/sysml_resolution/src/model.rs`), wired at
  `PackageBodyElement::ViewpointUsage`/`PartDefBodyElement::ViewpointUsage` -- resolving e.g.
  `test/snapshots/sysml/training/42_viewpoint_example.md`'s and
  `test/snapshots/sysml/validation/11a_view_viewpoint.md`'s `viewpoint 'system structure
  perspective' { frame ...; require constraint { ... } }` end to end (`completeness complete`).
  Only the `:>`/`:>>` header form described above remains genuinely blocked upstream.

- Gap 27. `AllocationUsage` (`src/ast/behavior.rs`) has no `subsets`/`redefines` fields, and its
  `allocate ... to ...` ends are captured as raw `Option<Node<Expression>>` (`source`/`target`),
  not as typed `end` declarations the way `AllocationDef`'s body uses
  `ReferenceKind::ConnectorEnd`-shaped `end` members. `struct AllocationUsage { name, type_name,
  type_is_conjugated, source: Option<Node<Expression>>, target: Option<Node<Expression>>, body:
  DefinitionBody, membership }`. Verified directly against the pinned `0757de13` checkout while
  attempting `allocation` usage-side lowering (following `04274711`'s def-side `allocation def`
  work as the template, which reused `ReferenceKind::ConnectorEnd` via the shared
  `lower_occurrence_body_element` walker for `end` declarations in `AllocationDef`'s body).
  Two independent problems block a faithful usage-side lowering: (1) no
  `SubsettingRelationship` fields to resolve specialization/redefinition facts, matching Gap
  8/25/26's class; (2) `source`/`target` are opaque `Expression` nodes rather than structured
  connector-end references, so even ignoring (1) there is no typed AST shape to route through the
  existing `ReferenceKind::ConnectorEnd` machinery without re-parsing/interpreting the
  `Expression` tree, which spec42 deliberately avoids. Left routed through the existing
  `unsupported_*_member` fallback wherever `PackageBodyElement::AllocationUsage` etc. appear.
  Needs `subsets`/`redefines` fields (mirroring `ViewUsage`/`ConnectionUsage`) and a typed
  source/target end shape (mirroring `AllocationDef`'s `end` declarations or `ConnectionUsage`'s
  connector ends) added to `AllocationUsage`, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 28. `FlowUsage` (`src/ast/behavior.rs`) has no `subsets`/`redefines` fields, and its `from
  ... to ...` ends are captured as raw `Option<Node<Expression>>` (`from`/`to`), not as typed
  `end` declarations the way `FlowDef`'s body uses `ReferenceKind::ConnectorEnd`-shaped `end`
  members -- the standalone `flow ... from ... to ...;` usage form is genuinely a different AST
  shape from the definition-side body, not merely a usage/def pairing with identical fields.
  `struct FlowUsage { kind: FlowUsageKind, name: Option<String>, type_name, type_is_conjugated,
  payload: Option<Node<PayloadFeature>>, from: Option<Node<Expression>>, to:
  Option<Node<Expression>>, body: DefinitionBody, membership }`. Verified directly against the
  pinned `0757de13` checkout while attempting `flow` usage-side lowering (following `04274711`'s
  def-side `flow def` work as the template, which reused `ReferenceKind::ConnectorEnd` via the
  shared `lower_occurrence_body_element` walker for `end` declarations in `FlowDef`'s body). Same
  two-part gap as Gap 27 (`AllocationUsage`): (1) no `SubsettingRelationship` fields at all; (2)
  `from`/`to` are opaque `Expression` nodes, not structured connector-end references, so there is
  no typed shape to route through the existing `ReferenceKind::ConnectorEnd` machinery without
  re-parsing/interpreting the `Expression` tree. Left routed through the existing
  `unsupported_*_member` fallback wherever `PackageBodyElement::FlowUsage` etc. appear. Needs
  `subsets`/`redefines` fields and a typed from/to end shape (mirroring `FlowDef`'s `end`
  declarations or `ConnectionUsage`'s connector ends) added to `FlowUsage`, filed upstream
  against `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).
  **Update (exhaustive `unsupported_package_member` audit, this pass):** also confirmed
  `FlowUsage` has no `is_abstract` field either (unlike `RenderingUsage`/`CaseUsage`/etc., which
  all do), so even the header-only `abstract flow msg of C;` (no `subsets`, `test/snapshots/kerml/
  behaviors.md`) fails to parse into `FlowUsage` at all -- same missing-field gap class, one more
  field. And the exact same two-part gap (no `subsets`/`redefines`, and for `UseCaseUsage`/
  `VerificationCaseUsage` no multiplicity/`nonunique` either) generalizes to two more usage kinds
  confirmed this pass: `UseCaseUsage`/`VerificationCaseUsage` (`src/ast/requirement.rs`) both have
  only `name`/`type_name`/`is_abstract`/`body`/`membership` -- no `subsets`/`redefines`/
  multiplicity/`nonunique` fields at all -- so the Systems Library's own base-feature declaration
  idiom (`test/snapshots/sysml.library/use_cases.md`'s `use case useCases : UseCase[0..*]
  nonunique :> cases { ... }`, `verification_cases.md`'s analogous `verificationCases`) still
  fails to parse into either node. The plain `use case <name>[: <Type>] { ... }`/`verification
  <name>[: <Type>] { ... }` header shape (no multiplicity/subsets) is unaffected and now lowered
  (`lower_use_case_usage`/`lower_verification_case_usage`, `crates/sysml_resolution/src/model.rs`,
  this pass) -- resolves e.g. `test/snapshots/sysml/validation/18_use_case.md`,
  `9_verification_simplified.md`, `test/snapshots/sysml/training/34_verification_case_usage_example.md`,
  `35_use_case_usage_example.md` end to end. Needs `subsets`/`redefines`/multiplicity/`nonunique`
  fields added to `UseCaseUsage`/`VerificationCaseUsage` (mirroring `CaseUsage`/
  `AnalysisCaseUsage`) and an `is_abstract` field added to `FlowUsage`, filed upstream against the
  same `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 29. `RequireConstraint` (`src/ast/requirement.rs`), the `require`/`assume`-prefixed
  constraint member inside `requirement def`/`requirement usage` bodies (BNF form: `(require|
  assume) constraint`? name? body), captures its target/usage `name` as a bare `Option<String>`,
  not a structured `QualifiedReferenceId`: `struct RequireConstraint { is_assume: bool,
  has_constraint_keyword: bool, name: Option<String>, body: RequireConstraintBody }`. Verified
  directly against the pinned `0757de13` checkout while attempting to lower the shorthand
  `require <name>;`/`assume <name>;` form (representative fixture:
  `test/snapshots/sysml/training/32_requirement_groups.md`'s `require fullVehicleMassLimit;`) into
  a resolved required-/assumed-constraint relationship, following `daf4dd3d`'s `Succession`
  end-reference lowering as the template (a plain/qualified name resolved via
  `Expression::FeatureRef` -> `push_reference` with a `local: QualifiedReferenceId`). Every
  working authored-reference case in `sysml_resolution` (`AliasBinding`, `Succession`,
  `FeatureTyping`, `VerifyRequirementMember.target`, etc.) sources its target from a typed
  `QualifiedReferenceId` index into the parser's own qualified-reference table, which carries the
  span and (if present) dotted segments `push_reference` needs to resolve and render the
  relationship; `RequireConstraint.name` is parsed via the plain unqualified-identifier
  `parser::lex::name` combinator (`alt((quoted_name, basic_name))`, no `::`-segment support) into
  a raw `String` with no separate span and no parser-table entry at all, so there is no
  `QualifiedReferenceId` to hand to `push_reference` and no way for `sysml_resolution` to
  synthesize one (the qualified-reference table belongs to the immutable, parser-owned document).
  This blocks the shorthand `require <name>;`/`require <name> { ... }`/`assume <name>;` reference
  form specifically (`has_constraint_keyword == false`); the `require constraint <name>? { ... }`
  form (`has_constraint_keyword == true`) is a genuine new nested-declaration site (like `subject`/
  `perform action`) and not blocked by this gap. Left routed through the existing
  `unsupported_requirement_definition_member`/`unsupported_requirement_usage_member` fallback via
  `RequirementDefBodyElement::RequireConstraint`. Needs `name` changed to
  `Option<QualifiedReferenceId>` (or a new `Option<Node<QualifiedReferenceId>>` field alongside a
  separate declared-name string for the `has_constraint_keyword` case, since the field currently
  serves both a reference-target role and a declared-name role depending on
  `has_constraint_keyword`), filed upstream against `feat/gh-119-arena-backed-references`
  (elan8/sysml-v2-parser#121). **Update:** the `has_constraint_keyword == true` form (`require
  constraint { ... }`/`assume constraint <name> { ... }`) is now implemented
  (`lower_require_constraint_member`, `crates/sysml_resolution/src/model.rs`) -- its body is the
  exact same `ConstraintDefBody`-shaped `Vec<Node<ConstraintDefBodyElement>>` already walked for
  `Constraint`/`AssertConstraintMember`, so no typed-AST change was needed for that half. The
  `has_constraint_keyword == false` shorthand-reference gap described above remains open and
  unimplemented, as does the closely-related discovery that `RequireConstraint` also has no `:
  Type` typing or `:>>` redefinition field after the name at all (`assume constraint c1 : C;`,
  `require constraint c1 :>> c;` -- both `test/snapshots/sysml/examples/requirement_test.md` --
  fail to parse as `RequireConstraint` and fall to raw-text recovery instead, never reaching
  `RequirementDefBodyElement::RequireConstraint` in the first place).

- Gap 30. `ThenTarget` (`src/ast/behavior.rs`) has no `Send` variant: `then send <expr> to
  <target>;` (a `then`-prefixed send shorthand, e.g. `then send new S() to b;` in `Simple Tests/
  ActionTest.sysml`) does not parse into a distinguishable `ThenTarget::Send` case the way `then
  merge`/`then fork`/`then decide` each get their own variant (`ThenTarget::{Merge,Fork,Decide}`).
  Verified directly against the pinned `0757de13` checkout while wiring `ThenTarget::Accept`'s
  sibling case: `enum ThenTarget { Action(Box<Node<ActionUsage>>), Perform(...), Merge(...),
  Fork(...), Decide(...), Accept(Node<TransitionAccept>), Feature(Node<Expression>) }` -- no `Send`
  arm exists at all. In practice `then send new S() to b;` is swallowed by the parser as
  `ThenTarget::Feature`'s bare-expression fallback (or fails to parse the trailing `to b;` clause
  cleanly), losing the `send`-suffixed action-usage shape (`ActionUsage.send`/`.to`) that a
  standalone `action <name> send { ... }`/`action <name> send via <src> to <tgt>;` usage already
  carries and that `sysml_resolution`'s `lower_accept_send_clauses` already resolves for the latter
  two forms (see commits on `closing-the-gap`). Left routed through the existing
  `unsupported_action_definition_member`/`unsupported_action_usage_member` fallback wherever a
  `then send ...;` statement appears. Needs a `Send(Node<ActionUsage>)` (or equivalent structured)
  variant added to `ThenTarget`, mirroring `Merge`/`Fork`/`Decide`'s own dedicated variants, filed
  upstream against `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 33. `ActionBodyDecl` (`src/ast/behavior.rs`) is a raw/opaque textual fallback (`{ keyword:
  String, text: String }`, no name/typing/value fields) for `attribute`/`calc`/`event` declarations
  -- and a nested `action def ...`'s own name -- found directly inside an action def/usage body
  (BNF `ActionDefBodyElement::Decl`/`ActionUsageBodyElement::Decl`), e.g. `attribute mass = 5;`
  written as a sibling of ordinary action-body statements rather than at package/part/attribute-def
  scope. Verified directly against the pinned `cb026cd` checkout while investigating action-body
  imperative-statement resolution (`Decl`/`Assign`/`If`/`While`/`Loop`/`ForLoop` audit): `struct
  ActionBodyDecl { pub keyword: String, pub text: String }` (`behavior.rs:499-502`), produced by
  `action_body_decl` (`src/parser/action.rs:1376-1405`, which only recognizes the `attribute`/
  `calc`/`event` keywords and captures everything up to the terminating `;`/`{` as an unparsed
  `text` blob via `take_until_terminator`) and by `nested_action_def_decl`
  (`src/parser/action.rs:1352-1374`, which fully parses a nested `action def ...` via the ordinary
  `action_def` production but then deliberately discards the parsed result, keeping only
  `keyword: "action"` and `text: format!("def {name}")` -- the comment there reads "Kept as a
  lightweight Decl so we do not bump AST shape for this recovery/parity fix; Spec42 already ignores
  `Decl`"). Unlike every other body-decl-shaped construct this branch's audit found adequate
  (`DefaultReferenceUsage`, `InOutDecl`, etc.), there are no structured fields here at all to lower
  -- no declared name to intern, no typing/value expression to resolve, nothing but an opaque
  string. Left routed through the existing `unsupported_action_definition_member`/
  `unsupported_action_usage_member` fallback (unchanged from prior behavior). Needs `ActionBodyDecl`
  widened to a real typed node (or `ActionDefBodyElement::Decl` retired in favor of dispatching
  `attribute`/`calc`/`event`/nested `action def` through their own already-typed AST productions,
  the way every other action-body-element variant does), filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 34. `UseCaseDefBodyElement` (`src/ast/requirement.rs`) has no production for the full
  `ref use case <name> : <Type> :>> <target>;` declaration form (BNF `ReferenceUsage` with the
  `use case` feature-kind keyword and an explicit type, as used pervasively by
  `Systems Library/UseCases`, e.g. `ref use case self : UseCase :>> Case::self;`). Verified
  directly against the pinned `cb026cd` checkout while investigating `test/snapshots/sysml.
  library/use_cases.md`'s residual `unsupported_use_case_definition_member` diagnostics: the enum
  (`requirement.rs:604-646`) carries only two `ref`-shaped variants -- `RefRedefinition(Node<
  RefRedefinition>)`, produced by `ref_redefinition`/`ref_redefinition_inner`
  (`src/parser/usecase.rs:171-195`), which parses only the bare shorthand `ref :>> <target> { ...
  }` (no `use case` keyword, no name, no explicit type, and a mandatory *braced body* rather than
  a `;` terminator) -- and no `Ref(Node<RefDecl>)` variant at all (the shape `1773ae40` wired for
  the other 9 body-element sites). `use_case_def_body_element`'s alternative list
  (`src/parser/usecase.rs:520-602`) has no production that accepts `ref` followed by `use case`,
  a name, `:`, a type, `:>>`, a target, and `;`; the whole statement fails every alternative and
  falls through to token-level error recovery, which is exactly the fine-grained per-word
  `unsupported_use_case_definition_member` diagnostic spray observed at `use_cases.md`'s `ref use
  case self : UseCase :>> Case::self;` and `ref use case start: UseCase :>> start { ... }` lines
  (5 diagnostics per statement, one per token, rather than one diagnostic for the whole line as
  seen for other genuinely-unsupported single statements). This is not the `1773ae40`-style
  "add the missing `RefDecl` dispatch arm" mechanical gap it first appears to be -- there is no
  `RefDecl` node reachable from `UseCaseDefBodyElement` to dispatch; the AST simply cannot
  represent this construct. Needs either a new typed variant (e.g. `RefUseCaseUsage(Node<
  RefDecl>)` or similar) added to `UseCaseDefBodyElement` with a parser production for the full
  `ref use case <name> : <Type> [:>> <target>];` form, or `ref_redefinition` widened to also
  accept the named/typed spelling, filed upstream against `feat/gh-119-arena-backed-references`
  (elan8/sysml-v2-parser#121). **Update (exhaustive `unsupported_use_case_definition_member`/
  `unsupported_analysis_case_definition_member` audit, this pass):** since `UseCaseDefBody`/
  `UseCaseDefBodyElement` is the one shared body shape reused by `use case def`/`analysis def`/
  `case def`/`verification def` alike (`lower_case_family_def_body` in `sysml_resolution`), the same
  root cause generalizes to three further shapes, all confirmed via a direct
  `sysml_v2_parser_next::parse_for_editor_owned` probe (temporary `crates/sysml_resolution/examples/
  dump_case_ast.rs`, removed after use) showing each fragments into a bare `Expression(FeatureRef(..))`
  per leading token exactly like the `ref use case` case above: (a) the plain, non-`ref`-prefixed
  nested usage spelling with an explicit type/specialization clause, e.g. `use_cases.md`'s `abstract
  use case subUseCases : UseCase[0..*] :> useCases, subcases { ... }` and `abstract ref use case
  includedUseCases : UseCase[0..*] :> useCases, enclosedPerformances { ... }` -- confirming
  `UseCaseDefBodyElement` has no plain `UseCaseUsage`/`AnalysisCaseUsage`-shaped variant for a nested
  `use case` usage at all (only `AnalysisCaseUsage` exists; there is no sibling `UseCaseUsage(Node<
  UseCaseUsage>)` arm, even though the parser's own `use_case_usage` function, `src/parser/
  usecase.rs:673-679`, exists and is used elsewhere); (b) the identical `ref <kind> <name> : <Type>
  :>> <target>;` shape for the `analysis` keyword sibling of `use case`, e.g. `analysis_cases.md`'s
  `ref analysis self : AnalysisCase :>> Case::self;`, which fragments the same way (`ref` alone
  becomes a bare `Expression(FeatureRef("ref"))`, `analysis self : AnalysisCase :>> Case::self;`
  separately becomes a working `AnalysisCaseUsage`); and (c) the named+typed `include use case
  <name> : <Type> [mult];` form (`use_case_test.md`'s `include use case uc1 : UC1;`/`include use case
  uc2 { ... }`, `sys_ml_v2_spec_annex_a_simple_vehicle_model.md`'s `include use case
  getInVehicle_a:>getInVehicle [1..5];`) -- a structurally distinct AST node from (a)/(b):
  `IncludeUseCase`'s own parser production, `include_use_case_inner` (`src/parser/usecase.rs:64-81`),
  only ever parses `include <target QualifiedReferenceId> [mult] <body>` (a *reference* to an
  existing use case), never the full nested-usage-declaration spelling with its own `use case`
  keyword, name, and type -- so `qualified_reference` greedily (and wrongly) consumes just the
  literal identifier `use` as `target`, then fails to find a body/terminator after `case uc1 : UC1;`
  remains, and the whole statement falls to the same per-token recovery. All three need either new
  `UseCaseDefBodyElement` variants (a plain `UseCaseUsage` arm for (a), reusing the existing
  `AnalysisCaseUsage`-style dispatch shape; a `RefAnalysisUsage`/generalized `Ref<Kind>Usage` arm for
  (b); and either a name/type-carrying variant on `IncludeUseCase` itself or a new
  `IncludeUseCaseUsage`-shaped node for (c)) or widened parser productions accepting the fuller
  spellings, filed upstream against the same `feat/gh-119-arena-backed-references`
  (elan8/sysml-v2-parser#121).

- Gap 35. `SubjectDecl` (`src/ast/requirement.rs:118-123`) has no `redefines` field and its parser
  production, `subject_decl_inner` (`src/parser/requirement.rs:482-558`), only recognizes a
  literal `:` before the type reference -- there is no alternative branch or field for a `:>>`
  redefinition spelling. Verified directly against the pinned `cb026cd` checkout while
  investigating `test/snapshots/sysml.library/use_cases.md`'s `subject subj :>> Case::subj;`
  (shared `subject` grammar used identically by requirement/concern/case-family bodies, e.g.
  `RequirementDefBodyElement::SubjectDecl`/`UseCaseDefBodyElement::SubjectDecl`, both routed
  through the same `subject_decl` parser function): with `:>>` following the name, `type_name`'s
  `opt(preceded(tag(":"), ...))` matches the leading `:` of `:>>`, leaving `>> Case::subj;` for
  `qualified_reference`, which fails and backtracks to `None`; the subsequent `;`/brace check then
  fails on the still-unconsumed `:>>`, so `subject_decl_inner` fails outright and the whole
  statement falls through to whole-line error recovery (a single
  `unsupported_use_case_definition_member` diagnostic spanning the full `subject subj :>> Case::
  subj;` line). This contradicts the original hypothesis that `sysml_resolution`'s
  `lower_subject_declaration` (`18c2c201`) merely fails to read an existing `redefines` field --
  no such field exists on `SubjectDecl`, and the parser cannot parse `:>>` here at all (unlike
  every sibling declaration kind -- `ActorRedefinitionAssignment`, `RefDecl`, etc. -- which do
  carry dedicated `redefines`/`:>>` support). Needs `SubjectDecl` widened with a `redefines:
  Option<Node<SubsettingRelationship>>` field (mirroring `RefDecl`/other declaration kinds) and
  `subject_decl_inner` taught to parse `:>>` as an alternative to `:`, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121). **Update:** the same root
  cause blocks two further real-fixture shapes found auditing `unsupported_requirement_definition_
  member` -- the type-less redefinition-with-value form `subject :>> vehicle = vehicle_large;`/
  `subject :>> mass = vehicle.mass;` (`test/snapshots/sysml/examples/evsample.md`,
  `vehicle_requirement_derivation.md`: `type_name`'s `:`-prefixed `opt(...)` still consumes the
  leading `:` of `:>>` and fails the same way with no type present either) and the `default`
  keyword form `subject generateTorque default engine1.generateTorque;`
  (`sys_ml_v2_spec_annex_a_simple_vehicle_model.md:907`, OMG spec Annex A), which
  `subject_decl_inner` cannot parse at all -- after the name it only ever tries `:` (type), `[`
  (multiplicity), or `=` (value); there is no `default`-keyword alternative to `=` the way e.g.
  `RefDecl`'s value clause might support. Both stay routed through the existing
  `unsupported_requirement_definition_member` fallback via `Other`/whole-statement recovery, same
  as the `:>>` case above. **Update (exhaustive `unsupported_use_case_definition_member`/
  `unsupported_analysis_case_definition_member` audit, this pass):** `SubjectDecl` is equally
  missing a `subsets: Option<Node<SubsettingRelationship>>` field -- the plain `:>` spelling (not
  just `:>>` redefines) is blocked by the exact same `type_name`-consumes-the-leading-`:`-of-`:>`
  mechanism, confirmed against four further fixtures: `use_cases.md`'s/`analysis_cases.md`'s
  `subject subj :>> Case::subj;` (the already-documented `:>>` case, now also confirmed present in
  the case-family bodies this audit covers, not just requirement bodies), `analysis_individual_
  example.md`'s `subject vehicle : Vehicle_1 :> vehicle_c1 { ... }` (a `:` type *and* a trailing `:>`
  subsets clause together -- `type_name` successfully parses `: Vehicle_1`, but the following `multiplicity`/
  `value` alternatives don't match `:>` either, so the whole statement still fails at the final `;`/
  brace check with `:> vehicle_c1 { ... }` unconsumed), `rationale_metadata_example.md`'s `subject
  alternatives :> engine [2] = (engine4cyl, engine6cyl);`, and `sys_ml_v2_spec_annex_a_simple_
  vehicle_model.md`'s `subject vehicleAlternatives[2]:>vehicle_b;`. `evsample.md`'s `subject :>>
  vehicle :> vehicle_large;`/`subject :>> vehicle :> vehicle_compact;` combine both missing fields in
  one statement (a type-less `:>>` redefinition target immediately followed by a `:>` subsets
  clause). Because `subject_decl_inner` fails outright rather than partially matching, a statement
  blocked this way loses its *entire* nested body to the same whole-statement recovery, not just its
  own header -- confirmed via `analysis_individual_example.md`'s `subject vehicle : Vehicle_1 :>
  vehicle_c1 { individual action :>> fuelConsumption : FuelEconomyAnalysis_1 { ... } }`: the nested
  `individual action ...` member (itself a fully well-formed, otherwise-lowerable `ActionUsage`
  member with an `is_individual`/`:>>` redefinition combination `sysml_resolution` already handles
  elsewhere) still surfaces as its own separate `unsupported_analysis_case_definition_member`
  diagnostic purely because the raw-text body-recovery mechanism re-segments the failed subject's
  braced content per-statement rather than swallowing it as one opaque blob -- not a distinct gap in
  its own right, just this same root cause's blast radius. Needs `SubjectDecl` widened with a
  `subsets: Option<Node<SubsettingRelationship>>` field alongside the already-requested `redefines`
  one, filed upstream against the same `feat/gh-119-arena-backed-references`
  (elan8/sysml-v2-parser#121).

- Gap 41. KerML's implicit self-reference identifier `that` (e.g. `test/snapshots/sysml/examples`'s
  `trig_functions.md`: `inv unitBound { -1.0 <= that & that <= 1.0 }` inside `datatype
  UnitBoundedReal :> Real { ... }`, 111 fixtures overall) has no lexically-distinguished status
  in the pinned `cb026cd` parser checkout. Checked the exact question that determines whether
  `sysml_resolution` may recognize it: `src/parser/lex.rs:407-536`'s `SYSML_RESERVED_KEYWORDS`
  table (the parser's own reserved-word list, used to tell a genuine language keyword apart from
  an arbitrary identifier for diagnostics) does **not** contain `"that"` -- it lexes as a plain,
  user-choosable identifier flowing through the ordinary `Expression::FeatureRef` lexical-lookup
  path, structurally indistinguishable from a real feature named `that`. This confirms and
  extends the finding already noted in passing in commit `50b93050`. Per this task's own
  explicit instruction, matching the literal string `"that"` in `sysml_resolution` under these
  conditions would be exactly the "reconstruct semantics from spelling" anti-pattern the codebase
  has consistently avoided (see e.g. the `enum_name_not_semantic` guard: string values must never
  be confused with identifiers by spelling) -- so implementing KerML `that` self-reference
  resolution was correctly **not attempted** in this pass, rather than shipped as a
  string-matching workaround. Context-threading the enclosing declaration through
  `classify_constraint_expression`/`classify_calc_expression`/`lower_constraint_expression`/
  `lower_calc_expression` (and their `_node`/eval-tree counterparts) is still worth doing on its
  own architectural merits (many constraint/calc call sites would benefit from knowing their
  owning declaration for other reasons), but it does not unblock `that` specifically without a
  parser-side change. Needs `"that"` added to `SYSML_RESERVED_KEYWORDS` (or an equivalent
  lexically-distinguishing AST marker on `FeatureRef`/a dedicated `Expression::ImplicitThat`
  variant) in the upstream parser before `sysml_resolution` can resolve it as a structural
  self-reference rather than an ordinary lexical lookup, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).
  **Update (exhaustive `unsupported_calc_definition_member` audit, this pass):** after widening
  `lower_calc_expression`'s `BinaryOp` support to match `lower_constraint_expression`
  (comparison/logical/range/coalesce operators, plus a new `Expression::Conditional` arm on both
  -- see `crates/sysml_resolution/src/model.rs`) and wiring the previously-undispatched
  `CalcDefBodyElement::{Succession,TypedParameter,Import,Comment,AssertConstraint}` variants,
  199 baseline occurrences dropped to 11, and all 11 residual ones trace to this exact gap in one
  further manifestation not previously called out explicitly: `(that as Occurrence).member`
  (`sysml.library/occurrences.md`, `performances.md`, `state_performances.md`) is an
  `Expression::MemberAccess` whose `base` is an `Expression::TypeCheck` (the `as` cast) wrapping a
  `that` `FeatureRef`, not a `FeatureRef`/`FeatureChainRef` directly -- `flatten_member_access_chain`
  correctly declines it (its root is a `TypeCheck`, not a reference), so the whole member-access
  chain falls through to the existing unsupported diagnostic rather than partially resolving `.member`
  while leaving `that` itself unresolved. This is the same root cause as the bare `that` case above,
  not a distinct gap -- no `sysml_resolution` change can unblock it without the same upstream
  `SYSML_RESERVED_KEYWORDS`/`Expression::ImplicitThat` fix.

- Gap 42. `StateDefBodyElement`/`RequirementDefBodyElement` (`src/ast/behavior.rs`,
  `src/ast/requirement.rs`) are closed enums covering only a small, hand-picked subset of the
  member kinds a `state def`/`requirement def` body may legally contain -- unlike sibling body
  enums (`PartDefBodyElement`, `ActionDefBodyElement`, `ConstraintDefBodyElement`, etc.), neither
  has a variant for the general action/attribute/constraint/succession/`ref`/port/calc
  usage-member zoo, nor for nested definitions of their own kind. Found exhaustively auditing
  `unsupported_state_definition_member`/`unsupported_requirement_definition_member` across the
  full corpus (this pass, against `cb026cd`): `StateDefBodyElement` (behavior.rs:828-853) has no
  `Action`/`Attribute`/`Constraint`/`AssertConstraintMember`/`Succession` variant at all, so a
  `state def`'s own members that use these (pervasive in the Systems Library's
  `Systems Library/States.sysml`, e.g. `attribute :>> isTriggerDuring;`, `action :>> subactions :>
  middle { ... }`, `succession stateSequencing first [0..1] exclusiveStates then [0..1]
  exclusiveStates { ... }`, `assert constraint {notEmpty(exclusiveStates) implies ...}` --
  `test/snapshots/sysml.library/states.md`) all fail to parse as any typed `StateDefBodyElement`
  variant and fall to the raw-text `Other`/error-recovery fallback (this settles this task's first
  investigation lead: bare `constraint`/`assert constraint` in a state body is *not* a
  spec42-side dispatch gap the way `dad50e75`'s original `assert constraint` slice wired it into
  other body enums -- there is no `Constraint`/`AssertConstraintMember` variant on
  `StateDefBodyElement` to dispatch from). `RequirementDefBodyElement` (requirement.rs:53-86) is
  similarly missing a `Ref`/`RefDecl` variant (`ref requirement :>> self: RequirementCheck;`,
  `ref part actors : Part[0..*] { ... }` -- `test/snapshots/sysml.library/requirements.md`,
  `views.md`), a parameter-member variant for `in ref`/`in calc` members
  (`test/snapshots/sysml.library/trade_studies.md`), a `Port`/`Allocate` variant
  (`sys_ml_v2_spec_annex_a_simple_vehicle_model.md:912,877`), a nested-`requirement def` variant
  (only a `RequirementUsage` variant exists, not a def; `requirement_test.md:10`'s `requirement def
  <'1'> A { ... }` nested inside another `requirement def` body), and support for a bare
  `requirement;` member with no name/body at all (`requirement_test.md:9`). `FrameMember`
  (requirement.rs:308-311, dispatched via `RequirementDefBodyElement::Frame`) has the same
  narrowness one level down: its parser production (`frame_member`,
  `src/parser/requirement.rs:466-476`) only ever parses `frame <name> <body>`, with no alternative
  for the `frame concern <name> : <Type>;` sub-form (BNF `FrameConcernMember`,
  `sys_ml_v2_spec_annex_a_simple_vehicle_model.md:1546`'s `frame concern vs:VehicleSafety;`) --
  `name()` greedily consumes `concern` as the declared name, then the body parser fails on the
  leftover ` vs:VehicleSafety;`, and the whole member falls to recovery. None of this is a mechanical
  spec42-side dispatch gap in the `fbcb58bf`/this-pass mold (there is no already-typed node these
  fall through to un-dispatched); each needs new upstream AST variants/parser productions before
  `sysml_resolution` has anything to lower, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 43. `EntryAction`/`DoAction`/`ExitAction` (`src/ast/behavior.rs:858-889`) support only two
  shapes: a reference-path form (`entry action <path> ...;`, `action_reference:
  Option<QualifiedReferenceId>`) or an empty/opaque `;`/`{ }` body -- there is no field for
  declaring a *new* named, typed, or redefining nested action (`entry action <name> :>>
  <target>;`, e.g. `test/snapshots/sysml.library/states.md`'s `entry action entryAction :>>
  'entry';`/`do action doAction: Action :>> 'do';`/`exit action exitAction: Action :>> 'exit';`,
  and `test/snapshots/sysml/examples/state_test.md`'s `do action b :>> c;`), nor for `assign`/
  `send`/`accept` effect bodies written directly under `entry`/`do`/`exit` rather than inside a
  `transition`'s own effect clause (e.g. `test/snapshots/sysml/examples/assignment_test.md`'s
  `entry assign counter.count := 0;`/`do assign counter.count := counter.count + 1;`,
  `test/snapshots/sysml/training/25_change_and_time_triggers.md`'s `entry assign
  vehicle.maintenanceTime := ...;`). Verified against the pinned `cb026cd` checkout:
  `state_behavior_action_target` (`src/parser/state.rs:145-168`), the shared header parser for
  all three keywords, deliberately refuses to swallow `send`/`accept`/`assign` as a bare
  reference-path target (returning `Err` so a sibling production can pick the statement up), but
  no such sibling production exists for `entry`/`do`/`exit` -- unlike `Transition::effect`
  (`TransitionEffect::{Send,Accept,Assign}`), which *does* have typed fields for exactly these
  shapes. Left routed through `unsupported_state_definition_member` via `Other`/whole-statement
  recovery for both sub-shapes. Needs `EntryAction`/`DoAction`/`ExitAction` widened with an
  optional declared-name + redefinition-target pair (mirroring `RefDecl`) for the first shape, and
  either a shared `TransitionEffect`-shaped field or dedicated `Assign`/`Send`/`Accept` variants
  for the second, filed upstream against `feat/gh-119-arena-backed-references`
  (elan8/sysml-v2-parser#121).

- Gap 44. `VariantTypedUsage` (`src/ast/structure.rs:714-721`) has no `Requirement` kind: its
  kind-keyword kit is `Part`/`Attribute`/`Item`/`Port`/`Perform` only, so the kind-keyword form
  `variant requirement <name>;` (`test/snapshots/sysml/examples/variability_test.md:38`'s
  `variant requirement r1;` inside `variation requirement r { ... }`) can never be typed as a
  `VariantUsage.typed` value the way `variant part <name>;`/`variant attribute <name>;` etc. can.
  Confirmed against the pinned `cb026cd` checkout: since `requirement` is not one of the five
  recognized kind keywords, the statement cannot match either `VariantUsage`'s typed-kind branch
  or its untyped bare-reference branch (`reference: Option<QualifiedReferenceId>`, which would
  stop at the bare word `requirement` and leave ` r1;` dangling before the terminator), so it
  fails to parse as `VariantUsage` at all and falls to `RequirementDefBodyElement`'s `Other`
  fallback -- distinct from (and upstream of) `lower_variant_usage`'s deliberate scope boundary
  added this pass, which only covers the already-parseable untyped `variant <path>;` form. Needs a
  `Requirement(Box<Node<RequirementUsage>>)` variant added to `VariantTypedUsage`, mirroring the
  existing five, filed upstream against `feat/gh-119-arena-backed-references`
  (elan8/sysml-v2-parser#121).

- Gap 45. `UseCaseDefBodyElement` (`src/ast/requirement.rs:604-646`) has no `InOutDecl` variant, so
  the bare `in <name> = <value>;`/`out <name> [:> <Type>] = <value>;` parameter-declaration
  shorthand (no `attribute` keyword) that sibling body enums already support
  (`ConstraintDefBodyElement::InOutDecl`, `CalcDefBodyElement::InOutDecl`, both already lowered by
  `sysml_resolution`'s `lower_parameter_declaration`) is unrecognized inside `use case`/`analysis`/
  `case`/`verification` bodies. Found exhaustively auditing `unsupported_use_case_definition_member`/
  `unsupported_analysis_case_definition_member` across the full corpus (this pass, against
  `cb026cd`): `in_out_decl` (`src/parser/action.rs:334-364`) already parses the `attribute`-keyword-
  optional shorthand fine on its own -- confirmed via a direct `sysml_v2_parser_next::
  parse_for_editor_owned` probe (temporary `crates/sysml_resolution/examples/dump_case_ast.rs`,
  removed after use) -- but `use_case_def_body_element`'s alternative list (`src/parser/
  usecase.rs:520-602`) never calls it, so a directed parameter member such as
  `test/snapshots/sysml/training/33_analysis_case_usage_example.md`'s `in scenario = cityScenario;`
  (nested inside a `analysis <name> : <Type> { ... }` usage body) or
  `test/snapshots/sysml/examples/evsample.md`'s `out voltage :> ISQ::electricPotential =
  vehicle.battery.batteryBehavior.output.voltage;`/`out voltage = vehicle.battery.batteryBehavior.
  output.voltage;` (directly inside an `analysis def`'s own body) falls straight through to the
  opaque `Other`/whole-statement-recovery fallback. This is the same missing-variant pattern Gap 42
  catalogued for `StateDefBodyElement`/`RequirementDefBodyElement` -- there is no `sysml_resolution`-
  side fix available; `lower_parameter_declaration` already exists and would lower this construct
  immediately if the typed AST offered it. Needs an `InOutDecl(Node<InOutDecl>)` variant added to
  `UseCaseDefBodyElement` with `use_case_def_body_element` taught to dispatch to `in_out_decl`
  (mirroring `ConstraintDefBodyElement`/`CalcDefBodyElement`'s existing wiring), filed upstream
  against `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 46. Bare `actor <name>;`/`actor <name> [multiplicity];` (no `: Type` at all) has no grammar
  production reachable from `UseCaseDefBodyElement`. Found exhaustively auditing
  `unsupported_use_case_definition_member` across the full corpus (this pass, against `cb026cd`):
  `use_case_def_body_element`'s own `actor_usage` production (`actor_usage_inner`, `src/parser/
  usecase.rs:680-716`) requires a mandatory `:` type clause (`type_name` is not `opt(...)`-wrapped,
  unlike the rest of the production's optional fields) -- confirmed by direct inspection, there is
  no anonymous-type branch. A second, thinner struct genuinely shaped for the untyped form already
  exists (`ActorDecl { identification: Identification }`, `src/ast/requirement.rs:448-451`, parsed
  by a same-named but distinct `actor_decl` function, `src/parser/usecase.rs:430-446`), but it is
  wired only into `PackageBodyElement::Actor` at *top-level package scope*
  (`try_package_body_dispatch!(..., ActorUsage, actor_decl, PackageBodyElement::Actor)`,
  `src/parser/package.rs:1930-1936`) -- `use_case_def_body_element`'s alternative list never calls
  it, and `UseCaseDefBodyElement` has no `ActorDecl`-shaped variant at all. Confirmed via a direct
  `sysml_v2_parser_next::parse_for_editor_owned` probe (temporary `crates/sysml_resolution/examples/
  dump_case_ast.rs`, removed after use): `use case def U { actor environment; }` lowers the whole
  `actor environment;` statement to `UseCaseDefBodyElement::Other`. Blocks OMG spec Annex A's
  `test/snapshots/sysml/examples/sys_ml_v2_spec_annex_a_simple_vehicle_model.md` (`actor
  environment;`/`actor road;`/`actor driver;`/`actor passenger [0..4];`/`actor driver [0..1];`/
  `actor passenger [0..1];`, 8 occurrences across two nested use-case usages). (Separately,
  `sysml_resolution` doesn't lower `PackageBodyElement::Actor` either -- an existing, pre-existing,
  unrelated scope gap at package scope, not part of this task's two target diagnostic families, left
  untouched.) Needs either `ActorUsage.type_name` widened to `Option<QualifiedReferenceId>` (mirroring
  `SubjectDecl`'s already-optional type) or a new `UseCaseDefBodyElement::ActorDecl(Node<ActorDecl>)`
  variant wired to the existing `actor_decl` production, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 47. The canonical anonymous flow-usage shorthand `flow from <a> to <b>;` (no declared name,
  no `: Type`, no `of <payload>` clause -- OMG spec Annex A's own preferred spelling, e.g.
  `test/snapshots/sysml/training/14_action_definition_example.md`'s `flow from focus.image to
  shoot.image;`) misparses its own `from` keyword as the flow's declared *name*, rather than
  recognizing the statement as anonymous. Found exhaustively auditing
  `unsupported_action_usage_member`/`unsupported_action_definition_member` across the full corpus
  (this pass, against `cb026cd`): `flow_usage_member`'s dispatch (`src/parser/flow.rs:221-256`)
  tries `name(peek)` first for any input not starting with `of`, then disambiguates the anonymous
  form from the named form by checking whether the token *after* the parsed name starts with `.`
  or the `to` keyword (`is_anonymous = fragment.starts_with(b".") ||
  starts_with_keyword(fragment, b"to")`, `src/parser/flow.rs:237-247`) -- but for `flow from
  focus.image to shoot.image;`, `name()` first greedily consumes the identifier `from` itself
  (`from` is absent from `SYSML_RESERVED_KEYWORDS`, `src/parser/lex.rs:407-...`, so it lexes as an
  ordinary identifier), leaving ` focus.image to shoot.image;` as the post-name remainder -- which
  starts with neither `.` nor `to`, so `is_anonymous` is `false` and the whole statement is routed
  to `flow_usage_named` instead. That production then re-parses starting from `from`, producing
  `FlowUsage { name: Some("from"), from: Some(FeatureRef(focus.image)), to:
  Some(FeatureRef(shoot.image)), .. }` -- confirmed via a direct `sysml_v2_parser_next::
  parse_for_editor` AST dump (temporary `crates/sysml_resolution/examples/dump_action_ast.rs`,
  removed after use) against a minimal `action def Foo { flow from focus.image to shoot.image; }`
  repro, with zero parse errors reported (the statement "successfully" parses to the wrong shape,
  it does not fail visibly). The identical `succession flow` keyword pair (`kind:
  FlowUsageKind::SuccessionFlow`) shares the exact same `flow_usage_member` dispatch and
  misparses identically (`test/snapshots/sysml/training/14_action_succession_example_2.md`'s
  `succession flow from focus.image to shoot.image;`). This is a *silent* misparse, not a rejection
  -- `sysml_resolution`'s `lower_flow_usage` cannot tell a real declared name (e.g. `flow
  generateToAmplify from a to b;`, which parses correctly since `generateToAmplify` is not `from`)
  apart from this artifact using only the typed AST (both are an ordinary non-empty `Option<String>`
  `name` field), so it cannot safely accept named `FlowUsage`s at all without risking silently
  synthesizing a spurious declaration literally named `from` for the (far more common in the corpus)
  anonymous form -- string-matching the literal value `"from"` would be exactly the
  "reconstruct semantics from spelling" anti-pattern this codebase has consistently avoided (see
  Gap 41's `that` write-up). `lower_flow_usage` (`crates/sysml_resolution/src/model.rs`) was
  widened this pass to accept the unambiguous *payload*-bearing anonymous form (`flow of <payload>
  from <a> to <b>;`, e.g. `test/snapshots/sysml/examples/picture_taking.md`'s `flow of Exposure
  from focus.xrsl to shoot.xsf;`, which does not hit this ambiguity since `of` is checked before the
  name-dispatch branch, `src/parser/flow.rs:234-236`) but conservatively continues to defer every
  `name.is_some()` case -- both the misparsed `from`-named statements and genuinely-named flows
  alike -- pending an upstream fix. Confirmed blocking (misparsed `from`-named form only, all
  verified via the same probe): `test/snapshots/sysml/training/14_action_definition_example.md`,
  `14_action_shorthand_example.md`, `14_action_succession_example_1.md`,
  `14_action_succession_example_2.md`, `15_action_decomposition.md`,
  `16_conditional_succession_example_1.md`, `16_conditional_succession_example_2.md`,
  `17_fork_join_example.md`, `17_merge_example.md`, `21_messaging_example.md`,
  `21_messaging_with_ports.md`, and `test/snapshots/sysml/examples/flashlight_example.md`'s two
  *genuinely*-named `succession flow onOffCmdFlow from ...;`/`succession flow lightFlow from ...;`
  statements (real names, not the misparse, but withheld for the same reason). Needs
  `flow_usage_member`'s anonymous-vs-named disambiguation (`src/parser/flow.rs:237-247`) taught to
  also treat a post-name remainder starting with the `from` keyword as anonymous (mirroring the
  existing `.`/`to` checks), or `from`/`to` added to `SYSML_RESERVED_KEYWORDS` so `name()` itself
  refuses to consume them, filed upstream against `feat/gh-119-arena-backed-references`
  (elan8/sysml-v2-parser#121).

**Re-verification pass note (this pass, against `cb026cd`):** Gaps 15-24 were re-checked by
grepping the current `cb026cd` checkout for the same starter tables/productions cited in each
entry's original write-up; every one of the 10 gaps (15, 16, 17, 18, 19, 20, 21, 22, 23, 24) is
still fully reproducible -- none have been resolved upstream since the earlier `0757de13`-era
write-up. All cited line ranges (`src/parser/attribute.rs:28-49`, `src/parser/attribute.rs:191-259`,
`src/parser/lex.rs:170`, `src/parser/lex.rs:407-...`, `src/parser/grammar_scope.rs`) still resolve
to the same regions in `cb026cd`, so no citation-line corrections were needed. The parser now reaches
further into previously-blocked content (58 fixtures now carry at least one
`unrecognized_declaration_in_scope (source "parser")` diagnostic, up from the ~51 at the
Gap 15-24 baseline, for 222 total occurrences), surfacing three new distinct root causes catalogued
below as Gaps 37-39.

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
- Gap 26. `RenderingUsage` had no `subsets`/`redefines` fields. Resolved upstream in cb026cd -- confirmed via direct AST inspection of the pinned checkout (`subsets: Option<Node<SubsettingRelationship>>`/`redefines: Option<Node<SubsettingRelationship>>` now present alongside `multiplicity`/`ordered`/`nonunique`/`value`). **Update (exhaustive `unsupported_package_member` audit, this pass):** the parser-side fix landed but `sysml_resolution` was never updated to match -- `PackageBodyElement::RenderingUsage` was still unconditionally `push_unsupported`. Implemented `lower_rendering_usage` (mirroring `lower_view_usage`) plus a `RenderingUsageBody` walker recursing into nested `view`/`rendering` usage members; wired at `PackageBodyElement`/`PartDefBodyElement::RenderingUsage`. Resolves `test/snapshots/sysml.library/views.md`'s `renderings`/`asTextualNotation`/`asTreeDiagram`/`asInterconnectionDiagram`/`asElementTable` base-feature declarations end to end.
- Gap 31. `InOutDecl` had no grammar support for the `nonunique`/`ordered` collection modifiers on a parameter declaration. Resolved upstream in cb026cd -- confirmed via direct AST inspection of the pinned checkout (`InOutDecl.ordered`/`InOutDecl.nonunique` fields now present, mirroring the fields already added to sibling usage kinds).

- Gap 32. `KermlFeatureMember` (`src/ast/kerml_fallback.rs`) has no `crosses` field, so a
  KerML association-end's trailing `crosses <feature>.<path>;` clause on the plain `end feature
  ...` form (no end-level name before `feature`, distinct from the named `KermlEndMember` form) is
  parsed but silently dropped, e.g. `end feature shorterOccurrence: Occurrence redefines
  sourceOccurrence crosses longerOccurrence.timeEnclosedOccurrences;` (representative fixture:
  `test/snapshots/kerml/end_outer_specializations.md`). Verified directly against the pinned
  `cb026cd` checkout while wiring `KermlEndMember`/`KermlFeatureMember` association-end lowering
  for `sysml_resolution`: `struct KermlFeatureMember { ..., subsets:
  Option<Node<SubsettingRelationship>>, redefines: Option<Node<SubsettingRelationship>>,
  references: Option<Node<SubsettingRelationship>>, chains: Option<QualifiedReferenceId>,
  inverse_of: Option<QualifiedReferenceId>, ... }` (`kerml_fallback.rs:272-329`) -- no `crosses`
  field anywhere on the struct, and a repo-wide grep of `src/ast/*.rs` for `crosses\b` confirms
  every other typed AST node that models a `crosses` cross-subsetting clause (`ConnectionEnd`,
  `InterfaceUsage`'s `EndDecl` at `structure.rs:1228`, `OccurrenceUsage` at `structure.rs:1612`)
  has a dedicated `crosses: Option<Node<SubsettingRelationship>>` field that `KermlFeatureMember`
  lacks. `sysml_resolution` already has a general `ReferenceKind::Crosses` (mapped from
  `SubsettingKind::Crosses` in the shared `lower_subsetting_relationship`) ready to consume such a
  field the moment it exists -- this is purely a missing parser field, not missing
  `sysml_resolution` wiring. `end happensDuring [1..*] subsets timeCoincidentOccurrences feature
  thatOccurrence: Occurrence redefines longerOccurrence;`'s `subsets`/`redefines` clauses (the
  named `KermlEndMember` form) are unaffected and fully lowered. Needs a `crosses:
  Option<Node<SubsettingRelationship>>` field added to `KermlFeatureMember`, mirroring the fields
  already present on `ConnectionEnd`/`EndDecl`/`OccurrenceUsage`, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).


- Alias declarations (`alias X for Y;`) — investigated as a possible parser gap, but the typed
  AST (`AliasDef.target`) was already a structured `QualifiedReferenceId`. Fixed entirely in
  `sysml_resolution` (commit `422e2216`), not a parser gap.
- Enum definitions (`enum def`, `EnumeratedValue`) — investigated, typed AST already exposes
  stable per-literal identity/spans. Fixed entirely in `sysml_resolution` (commit `99d5ea39`).

- Gap 37. `Dependency`'s optional `RelationshipBody` (BNF: `dependency` DependencyDeclaration
  (`;` | `{` doc/comment/rep/metadata* `}`)) rejects any owned member other than
  `doc`/`comment`/`rep`/`@` metadata annotations -- an ordinary nested `feature` member inside the
  braced form is unrecognized, even though the identical unbodied `dependency ... ;` statement (and
  every other `dependency` shape) parses and lowers correctly. Root cause: in the `cb026cd`
  checkout, `relationship_body_annotations` (`src/parser/body.rs:24-51`) drives
  `parse_structured_brace_members` off `RELATIONSHIP_BODY_STARTERS`
  (`src/parser/lex.rs:170`: `&[b"doc", b"comment", b"rep", b"@"]`) -- no `b"feature"` (or any other
  member-starter) entry -- so a `feature e;` member nested inside a `dependency ... { }` body falls
  straight through to `unrecognized_declaration_in_scope`. Confirmed empirically against
  `test/snapshots/kerml/dependencies.md`: the file's two unbodied `dependency` statements
  (`dependency Use from 'Application Layer' to 'Service Layer';` and
  `dependency from 'Service Layer' to 'Data Layer';`) both resolve fully in the `# SMG` block (two
  `(kind dependency)` declarations with resolved `dependencyClient`/`dependencySupplier`
  references), but the third, bodied statement (`dependency z to x, y { feature e; }`) produces the
  file's sole `unrecognized_declaration_in_scope` diagnostic and never gets a third dependency
  declaration in the `# SMG` output at all -- the whole bodied statement is dropped, not merely its
  `feature e;` member. Blocks `test/snapshots/kerml/dependencies.md`. Needs `RELATIONSHIP_BODY_STARTERS`
  widened with member-starter entries (`feature` at minimum, mirroring the owned-member support the
  BNF's `ownedRelatedElement*` implies for a KerML `RelationshipBody`), filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 38. Nested classifier-keyword declarations *other than* `class` (e.g. `struct`, and by the
  same mechanism `classifier`/`metaclass`/`behavior`/`interaction`/`predicate`/`multiplicity`/
  `subclassifier`) are unrecognized when they appear inside another type's attribute/structured
  body -- the same gap class as Gap 21 (`class`), but for the rest of the classifier-keyword family
  Gap 21's fix (as literally worded, "`class` added to `ATTRIBUTE_BODY_STARTERS`") would not cover.
  Root cause: `attribute_body_element` (`src/parser/attribute.rs:191-259`) dispatches a fixed `alt`
  list of productions -- `doc_comment`, `attribute_def`, `attribute_usage`,
  `value_keyword_binding`, `attribute_feature_binding`, `occurrence_usage`, `timeslice_usage`,
  `snapshot_usage`, `connect_`, `metadata_keyword_usage`, `metadata_keyword_prefix`,
  `assert_constraint_member`, `ref_decl`, `part_usage`, `item_usage`, and finally the opaque
  `capture_opaque_member(ATTRIBUTE_OPAQUE_STARTERS)` fallback -- none of which reach
  `classifier_decl`/`kerml_classifier_decl` (the productions that already handle `struct`/
  `classifier`/etc. at package-body scope, `src/parser/package.rs:925-937`: starters
  `&[b"class", b"classifier", b"struct", b"structure", b"subclassifier"]`), and
  `ATTRIBUTE_BODY_STARTERS`/`ATTRIBUTE_OPAQUE_STARTERS` (`src/parser/attribute.rs:28-67`) have no
  `b"struct"`/`b"classifier"`/etc. entry either, so a nested `struct Car1_ { ... }` falls through to
  `unrecognized_declaration_in_scope` the same way nested `class` did before Gap 21 was filed.
  Confirmed against the pinned `cb026cd` checkout by direct inspection of
  `attribute_body_element`'s alternative list (no classifier-decl arm present) and empirically via
  `test/snapshots/kerml/time_varying_car_driver.md`, whose `struct Car1_ { ... }` (nested directly
  inside the enclosing `part`/occurrence body) produces `unrecognized_declaration_in_scope` spanning
  the entire `struct Car1_ { ... }` block. Blocks `test/snapshots/kerml/time_varying_car_driver.md`.
  Needs `struct`/`classifier`/`metaclass`/`behavior`/`interaction`/`predicate`/`multiplicity`/
  `subclassifier` added to `ATTRIBUTE_BODY_STARTERS` with dispatch to the existing
  `classifier_decl`/`kerml_classifier_decl` production (the same fix shape as Gap 21, generalized to
  the rest of the keyword family), filed upstream against `feat/gh-119-arena-backed-references`
  (elan8/sysml-v2-parser#121).

- Gap 39. The bare `#<keyword>+ <Name> { ... }` extended-usage shorthand (a `#`-prefixed metadata
  tag directly prefixing a plain named member with a body, but with **no** `def`/other declaration
  keyword at all) has no grammar production -- only the `def`-suffixed sibling,
  `ExtendedDefinition` (`#<keyword>+ 'def' <Name> ...`, SysML BNF/§8.2.2.27, resolved upstream for
  spec42 as Gap 12), is supported. Root cause: `extended_definition_inner`
  (`src/parser/metadata_annotation.rs:177-207`) parses `many1` extended-definition prefix tags via
  `extended_definition_prefix_tag` (`src/parser/metadata_annotation.rs:140-163`) and then requires a
  literal `'def'` token before the name (per the doc comment at
  `src/parser/metadata_annotation.rs:164`: "`DefinitionExtensionKeyword+ 'def' DefinitionDeclaration
  ..."); there is no alternative production anywhere in `src/parser` that accepts one-or-more `#tag`
  prefixes directly followed by a bare name and brace body with no `def` keyword. Confirmed against
  the pinned `cb026cd` checkout: `test/snapshots/sysml/examples/ahfcore_lib.md`'s
  `#clouddd ArrowheadCore{ ... }` (a `#`-tagged bare-name usage with a multi-member body, structurally
  parallel to the `#service port def Authorisation { ... }` forms elsewhere in the same file that
  *do* parse because they include `port def`) produces a single `unrecognized_declaration_in_scope`
  diagnostic whose range spans from that statement through effectively the rest of the file
  (`(range (start 22 10) (end 54 0))`), i.e. the missing `def` causes the whole remainder of the
  package body to fall into unrecovered error-token consumption rather than a single per-statement
  diagnostic. Blocks `test/snapshots/sysml/examples/ahfcore_lib.md`. Needs a new grammar production
  (or `extended_definition` widened with an optional-`def` branch) covering the bare
  `#<keyword>+ <Name> { ... }` extended-usage shorthand, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 40. `metadata def`/`metadata` bodies use a dedicated, much narrower body-element parser
  (`metadata_body_element`, `src/parser/attribute.rs:1157-1175`, reached via `metadata_body` at
  `src/parser/attribute.rs:1193`) than every other attribute-shaped body (`attribute def`, `item
  def`, `part def`, etc., which use `attribute_body_element`, `src/parser/attribute.rs:191-260`).
  `metadata_body_element`'s alternation is just `doc | attribute_def | attribute_usage |
  metadata_binding | opaque-capture-fallback` -- it has **no** `ref_decl`, `part_usage`,
  `item_usage`, `connect_`, or `assert_constraint_member` arms at all, unlike
  `attribute_body_element`, which added all of these over time (see the "Before opaque capture so
  these no longer land in `Other`" comments at `src/parser/attribute.rs:238-249`). Confirmed by
  direct AST inspection at the pinned `cb026cd` checkout: a `ref self : Type :>> Other::self;`
  member inside an `attribute def { ... }` body parses into a structured `AttributeBodyElement::
  RefDecl` node (fully resolvable), while the byte-identical member inside a `metadata def { ...
  }` body falls straight to `AttributeBodyElement::Other` (opaque, unresolvable) via
  `metadata_body_element`'s fallback arm -- reproduced with a temporary
  `sysml_v2_parser_next::parse_for_editor_owned` probe from `crates/sysml_resolution` (not
  committed). This blocks `test/snapshots/sysml.library/metadata.md`'s `MetadataItem`'s `ref self
  : MetadataItem redefines Metaobject::self, Item::self;` member: it is not a multi-target
  specialization bug (`MetadataDef.specializes: Option<Node<TypingRelationship>>` and
  `lower_metadata_def` in `crates/sysml_resolution/src/model.rs:5029-5063` both already handle
  the `:> Metaobject, Item` multi-target specialization clause correctly, confirmed resolving both
  targets to separate `Subclassification`/`specialization` references end-to-end -- verified via
  a `sysml_resolution` unit-test probe), it is that the `ref self : ... redefines ...;` member
  *inside the metadata def's own body* never reaches `sysml_resolution` as a `RefDecl` node in the
  first place, so `lower_attribute_body`'s `AttributeBodyElement::Other` arm
  (`crates/sysml_resolution/src/model.rs:3858-3864`) correctly reports
  `unsupported_attribute_member` for input it was never given a chance to lower -- there is no
  `sysml_resolution`-side fix available; the gap is entirely upstream in
  `metadata_body`/`metadata_body_element`'s narrower grammar. Needs `metadata_body_element` widened
  to include the same `ref_decl`/`part_usage`/`item_usage`/`connect_`/`assert_constraint_member`
  arms `attribute_body_element` already has (or `metadata_def`/`metadata_usage` switched to reuse
  `attribute_body`/`attribute_body_element` directly, if no metadata-specific body semantics
  require the separate production), filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).
  **Update (exhaustive `unsupported_attribute_member` audit, this pass):** this is the single
  largest root cause of the 328-occurrence baseline for that diagnostic -- 195 of 328 (59%), all
  `derived ref item <name> : <Type>[mult] subsets/redefines <target>[, <target>]*;` members nested
  directly in `metadata def` bodies, confirmed via the same mechanism this gap already describes.
  Adds three more confirming fixtures to the blocked list above:
  `test/snapshots/sysml.library/sys_ml.md` (192 occurrences -- the Systems Library's own
  reflective KerML-abstract-syntax metadata defs, e.g. `metadata def ActionUsage specializes Step,
  OccurrenceUsage { derived ref item actionDefinition : Behavior[0..*] ordered redefines
  behavior, occurrenceDefinition subsets Metadata::metadataItems; }`) and
  `test/snapshots/sysml.library/modeling_metadata.md` (2 occurrences: `item risk : Risk [0..1] {
  ... }` inside `metadata def StatusInfo { ... }`, `ref explanation : Anything [0..1] { ... }`
  inside `metadata def Rationale { ... }`).

- Gap 49. Four further distinct root causes found during the same exhaustive
  `unsupported_attribute_member` audit (baseline 328 occurrences across the non-parser-grammar-
  blocked corpus; 195 traced to Gap 40 above, 2 fixed in `sysml_resolution` this pass by widening
  `flatten_member_access_chain` to see through `Expression::Parenthesized`/`Expression::TypeCheck`
  wrappers -- see the commit on `closing-the-gap` touching `crates/sysml_resolution/src/model.rs`
  -- and the remaining ~131 trace to the four gaps below), all confirmed via direct
  `sysml_v2_parser_next::parse_for_editor_owned` AST-dump probes (temporary
  `crates/sysml_resolution/examples/dump_attr_ast*.rs`, removed after use) against `cb026cd`:

  (a) **`AttributeBodyElement` (`src/ast/structure.rs:347-377`) has no `Bind`, `Connect`-typed
  named-connection, or `Calc`/`CalcDef`/plain-`ConstraintUsage` variant** -- the same
  enum-narrowness pattern Gap 42 catalogued for `StateDefBodyElement`/`RequirementDefBodyElement`,
  here for the body shared by `attribute def`/`item def`/usage bodies. Confirmed by direct AST
  inspection: `attribute_body_element`'s `alt` list (`src/parser/attribute.rs:191-259`) has no arm
  dispatching to `bind_`/`connection_usage`/`interface_usage`/`calc_def`/`calc_usage`/a plain
  (non-`assert`) `constraint_usage`, even though `binding`/`connection` are both registered as
  `ATTRIBUTE_BODY_STARTERS` (`src/parser/attribute.rs:28-49`, so they don't misfire as
  `unrecognized_declaration_in_scope`) and their own parser productions (`bind_`,
  `connection_usage`) are used successfully elsewhere (e.g. `PartUsageBodyElement`). Every
  instance falls straight to `AttributeBodyElement::Other`. Largest contributor:
  `test/snapshots/sysml.library/shape_items.md` (76 occurrences: 51 `binding [1] bind [mult] a.b =
  [mult] c;` named/multiplicity-qualified bind statements nested in `item def` bodies, e.g.
  `ConeOrCylinder`'s `binding [1] bind [0..*] base.edges = [0..*] be;`, plus 25 `connection
  :MatesWith connect [mult] a to [mult] b;` named/typed connection usages, e.g. `connection
  :MatesWith connect [1] tfe to [1] tfe;`). Also blocks `test/snapshots/sysml/examples/
  product_selection_owned_ends.md`/`product_selection_unowned_ends.md` (2 occurrences each:
  `connection ps1 : ProductSelection connect myCart to products { :>> info = info1; }`),
  `test/snapshots/sysml.library/time.md` (2 occurrences: `private calc getElapsedUtcTime { in
  iso8601DateTime: Iso8601DateTimeEncoding; return : Real; }` nested in an `attribute def`'s own
  body -- confirmed via probe that the *sibling* `attribute :>> num = getElapsedUtcTime(val);`
  statement parses and lowers perfectly fine; only the nested `calc` definition itself is
  unreachable), `test/snapshots/sysml.library/items.md` (1 occurrence: `abstract constraint
  checkedConstraints: ConstraintCheck[0..*] :> constraintChecks, ownedPerformances { doc ... }`,
  a plain nested constraint usage with no `assert` keyword -- `assert_constraint_member`
  (`src/parser/occurrence_body.rs:694-730`) requires a literal `assert` tag and has no
  bare-`constraint` alternative), and `test/snapshots/sysml.library/state_space_representation.md`
  (1 occurrence: bare `constraint { stateSpace.order == order }`, same root cause). Needs
  `attribute_body_element` widened with `Bind`/`Connect`-multi-end/`Calc`/plain-`ConstraintUsage`
  variants and dispatch arms (mirroring how `PartUsageBodyElement`/`PartDefBodyElement` already
  reach these same underlying productions), filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

  (b) **`attribute_feature_binding`'s bare `:>>`/`:>` shorthand (no `attribute` keyword) supports
  only a single redefinition/subsets target**, unlike every other redefinition/subsets clause in
  the grammar. Root cause: `attribute_feature_binding` (`src/parser/attribute.rs:283-345`) builds
  its `subsets`/`redefines` relationship via `single_target_subsetting(prefix_span, kind,
  qualified_reference(input)?)` -- a direct `qualified_reference` call that stops at the first
  comma -- instead of `subsetting`/`redefinition` (`src/parser/usage.rs:378-412`), which both call
  the comma-aware `specialization_targets` (`src/parser/usage.rs:275-285`) and are what the *full*
  `attribute` production (`attribute_usage`, dispatched when the `attribute` keyword is present)
  uses for the exact same clause. Confirmed via probe: `attribute :>> A::x, B::y { ... }` (full
  keyword form) parses fine with both targets in `SubsettingRelationship.target: Vec<...>`, while
  the byte-identical bare-shorthand `:>> A::x, B::y { ... }` (no `attribute` keyword) fails
  `attribute_feature_binding` entirely once it hits the comma and falls to
  `AttributeBodyElement::Other`. Confirmed blocking `test/snapshots/sysml.library/si.md` (3
  occurrences, e.g. `kelvin`'s nested `:>> ThermodynamicTemperatureUnit::quantityDimension::
  quantityPowerFactors, TemperatureDifferenceUnit::quantityDimension::quantityPowerFactors;`) and
  `test/snapshots/sysml.library/us_customary_units.md` (1 occurrence, the outer `:>>
  ThermodynamicTemperatureUnit::quantityDimension, TemperatureDifferenceUnit::quantityDimension {
  ... }` header itself, since it too has two comma-separated targets with no `attribute` keyword).
  Needs `attribute_feature_binding`'s prefix-target parsing switched from `qualified_reference` to
  `specialization_targets` (matching `subsetting`/`redefinition`), filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

  (c) **A parenthesized tuple/vector literal immediately followed by a bracket-suffixed identifier
  (`(a, b, c)[frameName]`) has no `Expression` grammar production**, breaking the whole enclosing
  statement. This is the Domain Geometry libraries' idiom for tagging a literal 3-vector with the
  coordinate frame it's expressed in (e.g. `new Translation( (0, shape.width/2, 0)[source] )`);
  the same `[frameName]` bracket suffix used for ordinary unit annotations on a *scalar* literal
  (`18 [mm]`, which parses fine) has no equivalent production when the base is a parenthesized
  tuple rather than a single numeric literal. Confirmed via probe: the failing statement's own
  sibling members parse and lower correctly (e.g. `test/snapshots/sysml/examples/
  simple_quadcopter.md`'s outer `:>> transformation : TranslationRotationSequence { ... }` lowers
  fine as an `AttributeUsage`; only its nested `:>> elements = (new Translation( (0,
  shape.width/2, 0)[source] ));` value statement falls to `AttributeBodyElement::Other`, because
  the whole statement fails to parse once its value expression cannot be parsed as any
  `Expression` variant). Confirmed blocking `test/snapshots/sysml/examples/simple_quadcopter.md`
  (16 occurrences), `test/snapshots/sysml/examples/vehicle_geometry_and_coordinate_frames.md` (5
  occurrences), and `test/snapshots/sysml/examples/car_with_shape_and_csg.md` (3 occurrences), all
  in the same `(new Translation(...)[frame])`/`(new Rotation(...)[frame], angle[unit])` idiom.
  Needs a new `Expression` production for a bracket-suffixed parenthesized tuple, filed upstream
  against `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

  (d) **`ref_decl`'s optional declared-name parse (`opt(name)`, `src/parser/connector.rs:394`)
  greedily consumes a following `redefines`/`subsets` keyword as the name** when no real name is
  present before it, since neither is in the reserved-keyword set checked at that point (contrast
  with `attribute_feature_binding`'s own `is_reserved_shorthand_starter` guard, which exists
  precisely to prevent this class of misparse for `:>>`/`:>` but isn't shared with `ref_decl`).
  Confirmed via probe: `private ref redefines Item::incomingTransferSort,
  subobjects::incomingTransferSort;` (`test/snapshots/sysml.library/items.md`, 1 occurrence) fails
  to parse as `RefDecl` at all and falls to `AttributeBodyElement::Other` -- `name()` consumes the
  literal word `redefines` as the declared name, leaving the comma-separated qualified-reference
  target list unconsumed. Needs `ref_decl`'s name parse guarded the same way
  `attribute_feature_binding`'s is (reject `redefines`/`subsets`/other relationship keywords as a
  bare name when no separate name token precedes them), filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 36. KerML `const` end-feature prefix (`const end [1] feature a;` / `const end feature
  b;`, representative fixture: `test/snapshots/kerml/associations.md`, `assoc struct C { ... }`)
  is not recognized as a keyword anywhere in the pinned `cb026cd` checkout: a repo-wide grep of
  `src/parser/*.rs` and `src/ast/*.rs` for `"const"`/`is_const` finds no such keyword or flag
  (only unrelated `const fn`/`Rust const` declarations and the distinct `constant`/`is_constant`
  KerML `RefPrefix` keyword, which is a different token). `KermlFeatureMember`
  (`src/ast/kerml_fallback.rs:274-329`) has a `is_end` flag and five other prefix-flag fields
  (`is_member`/`is_derived`/`is_abstract`/`is_composite`/`is_portion`/`is_var`) but no `is_const`
  -- so this is not a case of an existing flag the resolver merely fails to read (unlike
  `abstract`, which is represented and simply left semantically inert for reference resolution).
  Confirmed by dumping the typed AST directly (temporary `examples/dump_ast.rs` in
  `crates/sysml_resolution`, removed after use) for `assoc struct C { const end [1] feature a;
  const end feature b; }`: the parser does **not** attach `const` to the following `end`
  member at all. Instead it mis-parses the bare word `const` as an independent package-body
  member of kind `Expression(FeatureRef(QualifiedReferenceId(..)))` -- i.e. a bare
  expression-statement referencing an identifier named `const` -- immediately followed by a
  *separate* `KermlFeature(KermlFeatureMember { is_end: true/false, name: "a"/"b", ... })` member
  for the `end ... feature ...;` remainder, with no relationship between the two nodes. This
  is why `sysml_resolution`/the snapshot emits `unresolved_reference` pointing exactly at the
  `const` token's span (columns 2-7 on both fixture lines) rather than any diagnostic on the
  `end` member itself: the resolver is correctly reporting that a dangling `FeatureRef` to a
  nonexistent element named `const` cannot be resolved. This is a structural parser gap (missing
  grammar production / misrouted fallback), not a missing lowering in `sysml_resolution` -- there
  is no field to read and no correct AST shape to attach a `const` semantic to. Needs a `const`
  prefix keyword added to `KermlFeatureMember` (and/or wherever `KermlEndMember`'s owned feature
  is parsed) in the upstream parser, mirroring how `is_abstract`/`is_var`/`is_derived` are
  recognized, before `sysml_resolution` can represent or safely ignore it. Not yet filed
  upstream as one of the tracked issues against `feat/gh-119-arena-backed-references`
  (elan8/sysml-v2-parser#121) as of this writing -- filing is the next step before revisiting
  this fixture.

- Gap 50. `MetadataKeywordUsage` (`src/ast/structure.rs`), the `#<name>` shorthand for a metadata
  usage/annotation (BNF `PrefixMetadataAnnotation`/`MetadataUsage`'s `#`-sigil spelling), captures
  its annotation-type tag as a bare `keyword: String` -- not a `QualifiedReferenceId` the way its
  `@Name`-sigil sibling `MetadataAnnotation.reference` is (fully resolved by
  `lower_metadata_annotation`, `crates/sysml_resolution/src/model.rs`). Found exhaustively
  auditing `unsupported_package_member` (this pass, against `cb026cd`): the single largest root
  cause of the 155-occurrence baseline for that diagnostic -- 87 of 155 (56%) trace here, spread
  across `test/snapshots/kerml/a_2_atoms.md`, `a_2_modeling_instances.md`,
  `a_3_2_without_connectors.md` through `a_3_8_changing_feature_values.md` (the OMG KerML Annex A
  "atom" idiom, `#atom\n\tclassifier MyBike specializes Bicycle;` -- 60 of the 87), plus
  `test/snapshots/sysml/examples/ahfcore_lib.md`/`ahfnorway_topics.md` (`#service port def ...`/
  `#clouddd ...`), `test/snapshots/sysml/coverage_extended.md`/`examples/coverage_metadata.md`
  (`#situation ...`/`#Classified part def ...`), `test/snapshots/sysml/examples/metadata_test.md`/
  `requirement_metadata_example.md` (`#Security enum def ...`/`#goal requirement ...`), and
  `test/snapshots/sysml/validation/14c_language_extensions.md`/`training/41_user_keyword_example.md`
  (`#fmeaspec requirement ...`/`#causation connect ...`). Confirmed via direct parser-source
  inspection (`src/parser/metadata_annotation.rs`): both productions that build this node --
  `metadata_keyword_usage_inner` (the standalone `#<name>[: <Type>][about ...]{...}`/`;` form) and
  `metadata_keyword_prefix` (the bare-tag-immediately-before-another-declaration form, e.g. `#atom`
  followed by `classifier MyBike ...;` as two *separate* `PackageBodyElement`s -- confirmed the
  parser does not retain any structural link between the tag and the element it annotates) -- both
  parse `keyword` via the plain single-identifier `name` combinator
  (`src/parser/metadata_annotation.rs:91,242`), never `qualified_reference`, so there is no
  `QualifiedReferenceId` index for `sysml_resolution` to hand to `push_reference`/
  `lower_typing_relationship` the way every other resolved reference in this crate requires. This
  is not a newly-introduced gap in this pass -- `MetadataKeywordUsage` was already unconditionally
  routed to `push_unsupported`/no-op at every one of its 9 pre-existing call sites across
  `PartDefBodyElement`/`PartUsageBodyElement`/`AttributeBodyElement`/`ActionDefBodyElement`/
  `ActionUsageBodyElement`/`StateDefBodyElement`/`RequirementDefBodyElement`/
  `UseCaseDefBodyElement`/`PortDefBodyElement` before this pass, confirming this is a consistent,
  deliberate prior scope boundary rather than an oversight this pass could mechanically close.
  Also confirmed `lower_extended_definition`'s own doc comment (`#<keyword>+ 'def' <Name>`, Gap
  12) independently documents "the `#`-prefix keyword tags... are out of scope" for exactly this
  reason. Needs `MetadataKeywordUsage.keyword` changed to `QualifiedReferenceId` (mirroring
  `MetadataAnnotation.reference`) in both parser productions, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 51. The anonymous, header-less `allocate <source> to <target> { ... };` package/definition-
  member statement (SysML's shorthand `AllocationUsage` spelling with no `allocation` keyword, no
  declared name, and no `:` type -- OMG spec Annex A's own preferred style, e.g.
  `test/snapshots/sysml/validation/12b_allocation.md`'s `allocate torqueGenerator to powerTrain {
  allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque; }`,
  `test/snapshots/sysml/training/38_allocation_usage_example.md`'s identical shape) has no
  `PackageBodyElement`/`PartDefBodyElement`/`PartUsageBodyElement` variant reachable at all. Found
  exhaustively auditing `unsupported_package_member` (this pass, against `cb026cd`): the typed
  `ast::Allocate { source: Node<Expression>, target: Node<Expression>, body: ConnectBody }` node
  this shape would parse into exists (`src/ast/behavior.rs`) and is already reachable from other
  body-element enums (`PartUsageBodyElement::Allocate`, already lowered by `lower_allocate` in
  `sysml_resolution`), but `PackageBodyElement`'s own alternative list has no `Allocate` arm and no
  starter registered for it, so the bare package/definition-scoped spelling falls to whole-
  statement recovery instead. `test/snapshots/sysml/validation/12b_allocation_1.md`'s named
  variant (`allocation torqueGenAlloc : LogicalToPhysical allocate logical ::> torqueGenerator to
  physical ::> powerTrain { ... }`) and `training/38_allocation_definition_example.md`'s
  single-line form share the same root cause one level up: `AllocationUsage.source`/`target` are
  raw `Option<Node<Expression>>`, not the typed `Allocate` node, so Gap 27's already-tracked
  missing-typed-end-shape half of that gap covers those two, but the header-less bare-`allocate`
  form is a distinct missing-variant gap, not a missing-field one. Needs an
  `Allocate(Node<Allocate>)` variant added to `PackageBodyElement` (and ideally
  `PartDefBodyElement`) with a starter/dispatch arm reusing the existing `allocate` production,
  filed upstream against `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 52. Three SysML declaration-modifier prefixes have no representation in the pinned parser at
  all, so the canonical declaration-fact family (`DeclarationModifiers` in
  `crates/sysml_resolution/src/model.rs`) cannot include them and an element inspector cannot
  report them: `readonly`, SysML `variable`, and `unique`. Confirmed against `cb026cd` -- the AST
  design note at `src/ast/membership.rs:32` states they are deliberately out of scope, and there is
  no token, field, or starter for any of the three anywhere in `src/parser`. (`unique` is
  especially load-bearing: `ordered`/`nonunique` are both modeled as plain `bool` fields on ~11
  nodes, so a consumer can distinguish "authored `nonunique`" from "not authored", but it can never
  distinguish an authored `unique` from the default.) Note this is distinct from Gap 17's bare
  `portion` prefix, which is likewise unrepresentable in SysML scope; the KerML `portion` prefix
  *is* reachable via `KermlFeatureMember.is_portion` and is lowered. Needs `readonly`/`variable`/
  `unique` added as `FeaturePrefix`/`UsagePrefix` fields alongside the existing `derived`/
  `ordered`/`nonunique` ones, filed upstream against `feat/gh-119-arena-backed-references`
  (elan8/sysml-v2-parser#121).

- Gap 53. Several usage/definition nodes are missing a `multiplicity`, `nonunique`, or `short_name`
  field their siblings all carry, so the canonical multiplicity/short-name facts are absent for
  those declaration kinds rather than merely unwritten. Confirmed by direct field-by-field
  inspection of `cb026cd`'s `src/ast/` while lowering the declaration-fact family:
  (a) **no `multiplicity` field**: `AttributeDef` (`structure.rs`; contrast `AttributeUsage`, which
  has one), `ConstraintUsage` (`view.rs`), `RequirementUsage` (`requirement.rs`), `CalcUsage`
  (`view.rs`), `RequirementActorDecl` (`requirement.rs`; contrast `ActorUsage`, which has one);
  (b) **no `nonunique` field**: `PartUsage` (`structure.rs`), which does carry `ordered`;
  (c) **no `short_name` field**: `ActionUsage`, `OccurrenceUsage`, `ConstraintUsage`, `RefDecl`,
  `EndDecl`, `ReturnDecl`, `ViewUsage`, so the `<short> name` spelling is dropped for those kinds
  even though `AttributeUsage`/`PartUsage`/`PortUsage`/`ItemUsage`/`RequirementUsage` all keep it.
  Each is a one-field addition mirroring an existing sibling, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 54. `SubjectDecl.value` is a bare `Option<Node<Expression>>` (`src/ast/requirement.rs:122`)
  rather than the `Option<Node<FeatureValue>>` every other value-carrying node uses, so the
  authored value *spelling* is lost for subjects: `subject s = e;`, `subject s := e;`, and
  `subject s default e;` are indistinguishable once parsed, whereas `FeatureValue`'s
  `kind`/`is_default`/`has_operator` fields keep all five spellings apart everywhere else. This
  blocks recording a `FeatureValueRecord` for `DeclarationKind::SubjectUsage` in
  `crates/sysml_resolution/src/model.rs`; the subject's multiplicity and typing facts are lowered
  normally. Needs `SubjectDecl.value` widened to `Option<Node<FeatureValue>>`, filed upstream
  against `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121). See also Gap 35, which
  tracks `SubjectDecl`'s separate missing `subsets`/`redefines` fields.

- Gap 55. Only the three keyworded annotation productions survive into the AST: `doc /* ... */`
  (`ast::DocComment`), `comment /* ... */` (`ast::CommentAnnotation`), and
  `rep <language> "..." /* ... */` (`ast::TextualRepresentation`). Plain `/* ... */`, doc-style
  `/** ... */`, and `//` line comments are consumed as lexer trivia by `trivia_len`
  (`src/parser/lex.rs:209-226`, called from `ws_and_comments` at `:196`) and are unreachable from
  any AST node, so an element inspector can never surface a `/** ... */`-style doc block. Separately,
  `DocComment.text` is the raw byte slice between `/*` and `*/` (`src/parser/requirement.rs:724-726`)
  with no leading-`*` stripping and no dedent, so every consumer must normalize it identically or
  they will disagree. Not necessarily a defect -- keeping trivia out of the AST is a defensible
  design -- but it is a hard ceiling on documentation fidelity and is recorded here so the
  limitation is not rediscovered. If `/** ... */` is intended to be an annotation rather than
  trivia, it needs its own production, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 56. `EnumerationBody::Brace { values: Vec<Node<EnumeratedValue>> }`
  (`src/ast/structure.rs:1539`) holds *only* enumerated values, so an `enum def`'s own body
  annotations are discarded before they reach the AST: `enum def Color { doc /* ... */ enum red; }`
  parses with the `doc` element dropped entirely, unlike every sibling body-element enum
  (`PartDefBodyElement`, `AttributeBodyElement`, `RelationshipBodyElement`, …), all of which carry
  `Doc`/`Comment`/`TextualRep` variants. Confirmed against `cb026cd` while lowering the
  documentation fact family: `test/snapshots/documentation_in_bodies.md`'s `enum def Color`
  authors a doc comment that no fact family can recover, while the `part def`/`attribute def`/
  `item def`/`part` usage/`alias` docs in the same fixture all lower correctly. The same node also
  discards each `EnumeratedValue`'s own optional body and `= expr` initializer (`structure.rs:1544`
  documents this: "Only the name and its span are retained"), so a per-literal doc is unreachable
  too. Needs `EnumerationBody::Brace` widened to a proper body-element enum carrying at least
  `Doc`/`Comment`/`TextualRep` alongside the values, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).
