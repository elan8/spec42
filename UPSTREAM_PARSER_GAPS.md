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
  `perform action`) and not blocked by this gap, but is left unimplemented in this slice to keep
  the change focused on the reference case the task targeted. Left routed through the existing
  `unsupported_requirement_definition_member`/`unsupported_requirement_usage_member` fallback via
  `RequirementDefBodyElement::RequireConstraint`. Needs `name` changed to
  `Option<QualifiedReferenceId>` (or a new `Option<Node<QualifiedReferenceId>>` field alongside a
  separate declared-name string for the `has_constraint_keyword` case, since the field currently
  serves both a reference-target role and a declared-name role depending on
  `has_constraint_keyword`), filed upstream against `feat/gh-119-arena-backed-references`
  (elan8/sysml-v2-parser#121).

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
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

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
- Gap 26. `RenderingUsage` had no `subsets`/`redefines` fields. Resolved upstream in cb026cd -- confirmed via direct AST inspection of the pinned checkout (`subsets: Option<Node<SubsettingRelationship>>`/`redefines: Option<Node<SubsettingRelationship>>` now present alongside `multiplicity`/`ordered`/`nonunique`/`value`).
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
