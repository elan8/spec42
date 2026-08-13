# Upstream sysml-v2-parser gaps blocking spec42 snapshot work

Tracks semantic gaps discovered while closing the snapshot delta on the new parser-owned
pipeline (branch `closing-the-gap`, PR lukewilliamboswell/spec42#6) that trace back to the
pinned `sysml-v2-parser-next` revision rather than to `sysml_resolution`/`sysml_query`. Each
entry should carry enough detail to file/update an upstream issue against
`feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

## Open

### 1. Top-level `feature` declarations are unparsed grammar

- **Symptom:** `feature x : Integer;` at package scope produces `unsupported_grammar_form`
  (source: parser), cascading to `unsupported_package_member` (semantic). Zero declarations
  published.
- **Representative input:** `feature x : Integer;`
- **Representative snapshot:** `test/snapshots/feature_typing.md`
- **Scale:** ~131/503 snapshots blocked as of this writing (largest single blocked bucket). Also
  covers bare top-level `class X { ... }`/`class B specializes A { ... }` KerML declarations --
  both `"feature"` and `"class"` are keyword-starters routed through the same
  `kerml_semantic_decl` raw-text catch-all in `sysml-v2-parser`'s `src/parser/package.rs`, so a
  `class`-only fixture with no `feature` at all hits the identical `unsupported_grammar_form`/
  `unsupported_package_member` pair (see `test/snapshots/kerml/inheritance.md`, where `class A`,
  `class B specializes A`, `feature y: A { ... }`, and `feature w subsets y;` are all unlowered
  this way, leaving `alias z for y::g;`/`alias us for w::g;` unresolved as a downstream effect).
- **Status:** reported to Luke directly (parser repo owner); branch already diverges from
  `main`, which has more grammar coverage merged. Likely a rebase/merge issue rather than
  missing grammar work, per other local `sysml-v2-parser` worktrees at `main`.

### 2. Class specialization (`:>`) inside a `class` body is unparsed

- **Symptom:** `class B :> A { }` produces the same `unsupported_grammar_form` /
  `unsupported_package_member` pair as (1).
- **Representative snapshot:** `test/snapshots/class_specialization.md`
- **Status:** same as (1) — reported, awaiting upstream branch update.

### 3. `CalcDef` drops the parsed `:>` specialization clause

- **Symptom:** `calc def X :> Y { ... }` parses successfully (no `unsupported_grammar_form`,
  no parser-level error) but the `:>` specialization relationship is silently discarded before
  it ever reaches `sysml_resolution`. `ast::view::CalcDef` (crate `sysml-v2-parser`,
  `src/ast/view.rs`) has only `identification`, `body`, and `membership` fields — no
  `specializes: Option<Node<TypingRelationship>>` field, unlike the structurally analogous
  `ActionDef`/`ItemDef`/`PartDef`/`RequirementDef` etc., which all carry one. The shared
  `parse_definition_prefix` helper (`src/parser/definition_prefix.rs`) *does* parse the `:>`
  clause into `DefinitionPrefixResult::specializes` for every definition kind including calc —
  `parser::constraint::parse_calc_def` (`src/parser/constraint.rs`) simply never reads
  `prefix.specializes` when constructing the `CalcDef` node, so the parsed relationship is
  thrown away rather than surfaced as unsupported. This is a genuine typed-AST gap, not a
  grammar/tokenizing gap: the parse succeeds and the information exists transiently inside the
  parser but never reaches the AST.
- **Representative input:** `abstract calc def Calculation :> Action, Evaluation { ... }`
  (Systems Library `Calculations.sysml`).
- **Representative snapshots:** `test/snapshots/sysml.library/calculations.md`,
  `test/snapshots/sysml.library/vector_calculations.md`,
  `test/snapshots/sysml.library/tensor_calculations.md`,
  `test/snapshots/sysml.library/spatial_items.md`,
  `test/snapshots/sysml.library/time.md`,
  `test/snapshots/sysml/examples/sys_ml_v2_spec_annex_a_simple_vehicle_model.md` — 6 of the 26
  fixtures using `calc def` specialize via `:>`.
- **Impact:** blocks lowering `calc def`/`calc` declaration facts (`DeclarationKind::
  CalculationDefinition`) with full parity to the other `*Definition` kinds, since the
  in-scope requirement includes `:>` specialization resolving through the existing
  ancestor-closure fixed point (same shape as `action def`, see commit `f4ae83f7`) and the typed
  AST currently offers no field to lower that relationship from for `calc def` specifically
  (`CalcUsage.redefines`/`type_name` are present and fine; only `CalcDef.specializes` is
  missing).
- **Status:** blocking. Needs an upstream `sysml-v2-parser` change adding
  `specializes: Option<Node<TypingRelationship>>` to `ast::view::CalcDef` and wiring
  `parser::constraint::parse_calc_def` to populate it from `prefix.specializes` (mirroring
  every other `*_def` parser in that module). Not attempted here — out of scope for
  `sysml_resolution`/`sysml_query`/`spec42-snapshot` to work around without either (a)
  re-parsing the specialization clause independently (duplicating parser logic, fragile) or (b)
  shipping calc def declarations with unconditionally-absent specialization facts, silently
  under-reporting a relationship that real Systems Library content depends on.

### 4. `ConstraintUsage` drops the parsed `:>`/`:>>` specialization clauses

- **Symptom:** `constraint name :> a, b { ... }` and `constraint name :>> Qualified::target;`
  parse successfully (no `unsupported_grammar_form`, no parser-level error) but the `:>`
  subsetting / `:>>` redefinition relationship is silently discarded before it ever reaches
  `sysml_resolution`. `ast::view::ConstraintUsage` (crate `sysml-v2-parser`,
  `src/ast/view.rs`) has only `name`, `type_name: Option<QualifiedReferenceId>`, `body`, and
  `membership` fields — no `subsets`/`redefines` fields, unlike the structurally analogous
  `StateUsage`/`PortUsage` (which carry `subsets: Option<Node<SubsettingRelationship>>` and
  `redefines: Option<Node<SubsettingRelationship>>`, or a structured `typing:
  Option<Node<TypingRelationship>>`). The shared `feature_usage_header` parser
  (`src/parser/usage.rs`) *does* parse `:>`/`:>>` clauses (via `specialization_clauses`) into
  `UsageHeader` — `parser::constraint::constraint_usage` (`src/parser/constraint.rs`, lines
  75-98) calls `feature_usage_header` and only reads `header.type_reference` when constructing
  the `ConstraintUsage` node, silently dropping any subsetting/redefinition the header parsed.
  This is a genuine typed-AST gap, not a grammar/tokenizing gap: the parse succeeds and the
  information exists transiently inside the parser but never reaches the AST. Note `ConstraintDef`
  (the `constraint def` form) is unaffected — it has a proper `specializes: Option<Node<
  TypingRelationship>>` field parsed from `parse_definition_prefix`, with full parity to
  `ActionDef`/`StateDef`/`ConnectionDef`.
- **Representative input:** `Systems Library/Constraints.sysml`'s `abstract constraint
  constraintChecks: ConstraintCheck[0..*] nonunique :> booleanEvaluations { ... }` and
  `Systems Library/Requirements.sysml`'s `constraint assumptions :>>
  RequirementConstraintCheck::assumptions;` (both already covered by the parser's own
  `constraint_usage_tests` regression tests in `src/parser/constraint.rs`, which assert the
  parse succeeds but do not assert on subsetting/redefinition since there is no field to hold
  it).
- **Impact:** blocks lowering `constraint` usage declaration facts
  (`DeclarationKind::ConstraintUsage`) with full parity to the other `*Usage` kinds, since the
  in-scope requirement includes `:>`/`:>>` specialization resolving through the existing
  ancestor-closure fixed point (same shape as `state`/`connection` usage, see commits
  `36387ec3`/`d0675e2c`) and the typed AST currently offers no field to lower that relationship
  from for `constraint` usage specifically (`ConstraintUsage.type_name`/`body`/`membership` are
  present and fine; only `subsets`/`redefines` are missing). `ConstraintDef` itself (the `def`
  form) is not blocked by this gap.
- **Status:** blocking. Needs an upstream `sysml-v2-parser` change adding `subsets:
  Option<Node<SubsettingRelationship>>` and `redefines: Option<Node<SubsettingRelationship>>`
  fields to `ast::view::ConstraintUsage` and wiring `parser::constraint::constraint_usage` to
  populate them from the `UsageHeader` it already computes (mirroring `StateUsage`/`PortUsage`).
  Not attempted here — out of scope for `sysml_resolution`/`sysml_query`/`spec42-snapshot` to
  work around without either (a) re-parsing the specialization clauses independently
  (duplicating parser logic, fragile) or (b) shipping constraint usage declarations with
  unconditionally-absent subsetting/redefinition facts, silently under-reporting a relationship
  that real Systems Library content (`Constraints.sysml`, `Requirements.sysml`) depends on.

### 5. `AnalysisCaseUsage` (and `CaseUsage`) drop the parsed `:>`/`:>>` specialization clauses

- **Symptom:** `analysis a : A1 :> b { ... }` / `analysis a :>> b;` (and the same for bare
  `case` usages) parse successfully (no `unsupported_grammar_form`, no parser-level error) but
  any `:>` subsetting / `:>>` redefinition clause on the usage is silently discarded before it
  ever reaches `sysml_resolution`. `ast::AnalysisCaseUsage` and `ast::CaseUsage` (crate
  `sysml-v2-parser`, `src/ast/requirement.rs`) have only `name`, `type_name:
  Option<QualifiedReferenceId>`, `is_abstract`, (`is_individual` for analysis), `body`, and
  `membership` — no `subsets`/`redefines` fields, unlike the structurally analogous
  `RequirementUsage` (which carries `subsets: Option<Node<SubsettingRelationship>>` and
  `references: Option<Node<SubsettingRelationship>>`) or `PortUsage`/`StateUsage`. The shared
  `usage_header` parser (`src/parser/usage.rs`) *does* parse `:>`/`:>>` clauses into
  `UsageHeader::subsets`/`redefines` (see `usage_header_accepts_typing_then_specialization` /
  `usage_header_accepts_specialization_then_typing` tests in that file) —
  `parser::case::case_like_usage_body` (`src/parser/case.rs`, shared by both `CaseUsage` and
  `AnalysisCaseUsage`) calls `usage_header` and only reads `header.type_reference` when
  constructing the usage node, silently dropping any subsetting/redefinition the header parsed.
  This is a genuine typed-AST gap, not a grammar/tokenizing gap — the parse succeeds and the
  information exists transiently inside the parser but never reaches the AST. Note
  `AnalysisCaseDef`/`CaseDef` (the `def` forms) are unaffected — both have a proper
  `specializes: Option<Node<TypingRelationship>>` field parsed from `parse_definition_prefix`,
  with full parity to `ActionDef`/`OccurrenceDef`/`ConnectionDef`.
- **Representative input:** `analysis fuelEconomyAnalysis_1 : FuelEconomyAnalysis_1 :> baseAnalysis { ... }`
  (pattern not currently exercised by fixtures, but reachable per the parser's own grammar —
  discovered while verifying field parity ahead of lowering `analysis def`/`analysis` usage
  declarations, see commit adding `DeclarationKind::AnalysisCaseDefinition`).
- **Representative snapshots:** none yet (no fixture currently authors `:>`/`:>>` on an
  `analysis`/`case` usage — this is a latent gap, not one presently manifesting as a wrong
  snapshot), but any future fixture doing so would silently drop the relationship.
- **Impact:** blocks lowering `analysis`/`case` usage declaration facts
  (`DeclarationKind::AnalysisCaseUsage`) with full parity to the other `*Usage` kinds, since the
  in-scope requirement includes `:>`/`:>>` specialization resolving through the existing
  ancestor-closure fixed point (same shape as `occurrence`/`connection` usage, see commits
  `798d7287`/`d0675e2c`) and the typed AST currently offers no field to lower that relationship
  from for `analysis`/`case` usage specifically (`type_name`/`body`/`membership` are present and
  fine; only `subsets`/`redefines` are missing). `AnalysisCaseDef`/`CaseDef` themselves (the
  `def` forms) are not blocked by this gap, so this slice lands `analysis def` lowering only and
  defers `analysis` usage lowering until this is fixed upstream.
- **Status:** blocking (usage side only). Needs an upstream `sysml-v2-parser` change adding
  `subsets: Option<Node<SubsettingRelationship>>` and `redefines: Option<Node<
  SubsettingRelationship>>` fields to `ast::AnalysisCaseUsage`/`ast::CaseUsage` and wiring
  `parser::case::case_like_usage_body` to populate them from the `UsageHeader` it already
  computes (mirroring `RequirementUsage`/`PortUsage`/`StateUsage`). Not attempted here — out of
  scope for `sysml_resolution`/`sysml_query`/`spec42-snapshot` to work around without either (a)
  re-parsing the specialization clauses independently (duplicating parser logic, fragile) or (b)
  shipping analysis/case usage declarations with unconditionally-absent subsetting/redefinition
  facts.

### 6. `InterfaceUsage` has no `subsets`/`redefines` fields at all (unlike `ConnectionUsageMember`)

- **Symptom:** `interface i : I :> baseI;` / `interface i :>> redefinedI;` package/definition/
  usage-level interface usages parse successfully (no `unsupported_grammar_form`, no parser-level
  error) but any `:>` subsetting / `:>>` redefinition clause is silently discarded before it ever
  reaches `sysml_resolution`. `ast::InterfaceUsage` (crate `sysml-v2-parser`,
  `src/ast/structure.rs`) is a three-variant enum (`TypedConnect`, `Connection`, `Declaration`),
  and none of the three variants carry `subsets: Option<Node<SubsettingRelationship>>` /
  `redefines: Option<Node<SubsettingRelationship>>` fields — each only has `interface_type:
  Option<QualifiedReferenceId>`, a bare unstructured reference, not even a structured
  `TypingRelationship` (unlike `ConnectionUsageMember::type_reference`, which is also a bare
  `QualifiedReferenceId` but *does* carry separate `subsets`/`redefines: Option<Node<
  SubsettingRelationship>>` fields alongside it). This is a genuine typed-AST gap, not a grammar/
  tokenizing gap.
- **Representative input:** `interface i : I :> baseInterface;` (pattern not currently exercised
  by fixtures — discovered while verifying field parity ahead of lowering `interface def`/
  `interface` usage declarations, see the commit adding `DeclarationKind::InterfaceDefinition`).
- **Representative snapshots:** none yet (no fixture currently authors `:>`/`:>>` on an
  `interface` usage — this is a latent gap, not one presently manifesting as a wrong snapshot),
  but any future fixture doing so would silently drop the relationship.
- **Impact:** blocks lowering `interface` usage declaration facts (`DeclarationKind::
  InterfaceUsage`) with full parity to the other `*Usage` kinds, since the in-scope requirement
  includes `:>`/`:>>` specialization resolving through the existing ancestor-closure fixed point
  (same shape as `occurrence`/`connection` usage, see commits `798d7287`/`d0675e2c`) and the typed
  AST currently offers no field to lower that relationship from for `interface` usage
  specifically. `InterfaceDef` itself (the `def` form) is not blocked by this gap — its
  `specializes: Option<Node<TypingRelationship>>` field has full parity with `ConnectionDef`/
  `ActionDef`/`OccurrenceDef` — so this slice lands `interface def` lowering only (reusing the
  same `ReferenceKind::ConnectorEnd` machinery `connection def` uses for its `end`/`connect`
  structure, since `InterfaceDefBody`/`InterfaceDefBodyElement` share that shape) and defers
  `interface` usage lowering until this is fixed upstream.
- **Status:** blocking (usage side only). Needs an upstream `sysml-v2-parser` change adding
  `subsets: Option<Node<SubsettingRelationship>>` and `redefines: Option<Node<
  SubsettingRelationship>>` fields to each `ast::InterfaceUsage` variant (mirroring
  `ConnectionUsageMember`), and ideally widening `interface_type` to a structured
  `TypingRelationship` for full parity too. Not attempted here — out of scope for
  `sysml_resolution`/`sysml_query`/`spec42-snapshot` to work around without either (a) re-parsing
  the specialization clauses independently (duplicating parser logic, fragile) or (b) shipping
  interface usage declarations with unconditionally-absent subsetting/redefinition facts.

### 7. `individual <kind> <name>;` short usage forms are misparsed or entirely unparseable for `item`, `occurrence`, and `port`

- **Symptom:** Three distinct parser-side bugs, all discovered via
  `test/snapshots/sysml/coverage_individual.md`'s bottom (usage) block:
  1. `individual item i1;` lowers as `(kind item-def)` (an `ItemDefinition`), not
     `(kind item)` (an `ItemUsage`) — the wrong declaration kind entirely, worse than an
     `unsupported` diagnostic.
  2. `individual occurrence o1;` is dropped silently: no declaration, no per-line
     `unsupported` diagnostic — instead a parse-recovery cascade (`recovered_package_body_element`
     / `recovery_cascade_suppressed`, source `parser`) swallows it and the following line.
  3. `individual port po1;` is dropped the same way as (2) — the same recovery cascade absorbs
     it.
  Compare with `individual part p2;` / `individual action a1;` / `individual state s1;` in the
  same fixture, which all lower correctly as their usage kinds (`part`/`action`/`state`) — so
  this is specific to `item`/`occurrence`/`port`, not `individual` usages in general.
- **Root cause (three separate parser bugs, all in `sysml-v2-parser`
  `src/parser/package.rs`'s `try_package_body_element`/`try_package_body_behavior` `alt` chains
  and their constituent parsers):**
  1. **`item` (dispatch-order shadowing):** `src/parser/item.rs::item_usage` already has
     correct, dedicated support for this exact form (`let (input, is_individual) =
     opt(preceded(tag(b"individual"), ws1))...` — comment even cites `individual item ii : II1;`
     from `Simple Tests/IndividualTest.sysml:4`). The bug is dispatch order:
     `try_package_body_behavior` tries `item_def` (package-level, **`require_def: false`** via
     `parse_item_def(input, false)`) *before* `item_usage`. `item_def`'s
     `DefinitionPrefixOptions::new(b"item").individual_allowed()` has no `.def_required()`, so it
     happily matches `individual item i1` treating `i1` as the *definition's* identification
     name with no `def` keyword present at all, and wins the `alt` race before `item_usage` is
     ever tried. `action_def`/`state_def` do not have this problem because both correctly set
     `.def_required()` on their package-level parser (see `src/parser/action.rs::action_def`,
     `src/parser/state.rs::state_def`), so `individual action a1;`/`individual state s1;`
     legitimately fail `action_def`/`state_def` and fall through to `action_usage`/`state_usage`.
     `item_def`'s package-level variant is the odd one out. Since `ast::ItemDef` has no field
     recording whether a `def` keyword was actually present in the source, this cannot be
     disambiguated downstream in `sysml_resolution` — the AST node spec42 receives is already the
     wrong shape by the time it arrives.
  2. **`occurrence` (missing kind-keyword handling in `individual_usage`):**
     `src/parser/occurrence_body.rs::individual_usage` consumes the literal `individual` token
     and then goes straight to `occurrence_usage_tail`, which expects the *name* next — it has no
     handling for an intervening kind keyword (`occurrence`/`item`/`port`/etc., BNF
     `OccurrenceUsagePrefix` a la `individual occurrence o1;`). So on `individual occurrence o1;`
     it consumes `individual`, then misreads the literal token `occurrence` as the usage's *name*
     (`name = "occurrence"`), successfully parses a short, nameless-bodied `OccurrenceUsage`, and
     leaves `o1;` as unconsumed trailing input — which the outer package-body loop then can't
     parse as a fresh statement, producing the recovery cascade that swallows both `o1` and
     whatever follows. `occurrence_def`/`occurrence_usage` (no `individual` prefix) are tried
     first in the same `alt` chain but both require the literal `occurrence` keyword up front, so
     neither matches a bare `individual occurrence o1;` (they need `occurrence` first, not
     `individual`).
  3. **`port` (no `individual` support anywhere):** `src/parser/port.rs` has zero references to
     `"individual"` — neither `port_def`/`parse_port_def` nor `port_usage` accept an `individual`
     prefix in any form. `individual port po1;` therefore cannot be parsed as a `PortDef` or
     `PortUsage` by any code path; it also isn't recognized by `occurrence_def`/`occurrence_usage`
     (wrong keyword) and hits the same `individual_usage` name-misparse trap as (2) (`port`
     misread as the name, `po1;` left dangling), landing in the same recovery cascade.
- **Also affects the `def` side, and is the actual trigger of the cascade:** the fixture's
  `recovered_package_body_element`/`recovery_cascade_suppressed` pair is anchored at source line
  6 (`individual state def D6;`), not at the usage block — i.e. the cascade *starts* at D6 and
  swallows everything from there through the end of the usage block (D6 through D17, then
  o1/po1 within the usage block, resyncing only at statement boundaries the recovery logic
  happens to regain, e.g. `p1`/`i1`(misclassified)/`p2`/`a1`/`s1`). `state_def`
  (`src/parser/state.rs::state_def`) and `connection_def`
  (`src/parser/connection.rs::connection_def`) both omit `.individual_allowed()` on their
  `DefinitionPrefixOptions` (unlike `occurrence_def`/`item_def`/`part_def`/`action_def`/
  `analysis_case_def`, which all set it and correctly lower D2/D3/D4/D5/D13), so `individual
  state def D6;`/`individual connection def D7;` fail to parse as `StateDef`/`ConnectionDef` at
  all even though `sysml_resolution` already has working `lower_state_def`/`lower_connection_def`
  dispatch arms that would lower them correctly if the parser produced the node. D8/D9/D11/D12/
  D14/D15/D16/D17 (`calc`/`constraint`/`concern`/`case`/`verification`/`view`/`viewpoint`/
  `rendering` def) are separately not yet wired to any lowering arm in `sysml_resolution`
  (unconditional `push_unsupported`) regardless of `individual`, so those are not new gaps here —
  D6/D7 are the only two definitions in this fixture blocked specifically by a missing
  `individual_allowed()` parser option that otherwise has a working consumer. D1 (bare
  `individual def D1;`, no kind keyword) has no defined SysML semantic without a following kind
  keyword and is correctly left unsupported (`IndividualDef` → `push_unsupported`) — not a bug.
- **Representative input:** the full `test/snapshots/sysml/coverage_individual.md` source (D1–D17
  def block plus the `individual p1; individual occurrence o1; individual item i1; individual
  part p2; individual port po1; individual action a1; individual state s1;` usage block).
- **Representative snapshot:** `test/snapshots/sysml/coverage_individual.md`.
- **Impact:** blocks correctly lowering `individual item`/`individual occurrence`/
  `individual port` package/definition/usage-level short usage forms; `individual item x;`
  actively mis-lowers as a definition rather than merely being unsupported, which is worse than
  silence. Not fixable in `sysml_resolution` without either re-parsing text independently
  (fragile, out of scope) or accepting the wrong classification.
- **Status:** blocking. Needs upstream `sysml-v2-parser` changes: (1) reorder
  `try_package_body_behavior` to try `item_usage` before `item_def`, or switch the package-level
  `item_def` dispatch to `item_def_required` (mirroring `action_def`/`state_def`) so `item_def`
  no longer shadows `individual item x;`; (2) teach
  `src/parser/occurrence_body.rs::individual_usage` to optionally consume a following kind
  keyword (`occurrence`/`item`/`port`/etc, per BNF `OccurrenceUsagePrefix`) before parsing the
  name, instead of misreading the keyword as the name; (3) add `individual`-prefix support to
  `src/parser/port.rs::port_usage` (and decide whether `port_def` should accept it too, mirroring
  `item_def`'s `individual_allowed()`); (4) add `.individual_allowed()` to
  `src/parser/state.rs::state_def` and `src/parser/connection.rs::connection_def` so `individual
  state def`/`individual connection def` parse at all (their `sysml_resolution` lowering already
  exists and would work once the parser stops rejecting the input). Not attempted here — out of
  scope for `sysml_resolution`/`sysml_query`/`spec42-snapshot` to work around.

### 8. `ViewUsage` has no `subsets` field (unlike `OccurrenceUsage`/`StateUsage`/`PortUsage`)

- **Symptom:** `view v :> baseView { ... }` (bare `:>` subsetting, no explicit `: Type`) parses
  successfully (no `unsupported_grammar_form`, no parser-level error) but the subsetting
  relationship is silently discarded before it ever reaches `sysml_resolution`.
  `ast::view::ViewUsage` (crate `sysml-v2-parser`, `src/ast/view.rs`) has `name`, `type_name:
  Option<QualifiedReferenceId>`, `redefines: Option<Node<SubsettingRelationship>>`,
  `multiplicity`, `body`, and `membership` fields, but no `subsets: Option<Node<
  SubsettingRelationship>>` field at all — unlike the structurally analogous
  `OccurrenceUsage`/`StateUsage`/`PortUsage`, which all carry independent `subsets` and
  `redefines` fields side by side. `parser::view::view_usage`
  (`src/parser/view.rs::view_usage`) calls `parse_feature_usage_header` (which parses `:>`/`:>>`
  clauses via `feature_usage_header` into a `UsageHeader` that *does* have a `subsets` field) and
  only reads `header.type_reference`/`header.redefines` when constructing the `ViewUsage` node,
  silently dropping `header.subsets`. This is a genuine typed-AST gap, not a grammar/tokenizing
  gap: the parse succeeds and the information exists transiently inside the parser but never
  reaches the AST. Note `ViewDef` (the `view def` form) is unaffected — it has a proper
  `specializes: Option<Node<TypingRelationship>>` field with full parity to `ConnectionDef`/
  `ActionDef`/`OccurrenceDef`.
- **Representative input:** `test/snapshots/sysml/validation/11b_safety_and_security_feature_views.md`'s
  `view vehicleMandatorySafetyFeatureView :> vehicleSafetyFeatureView { ... }` — real fixture
  content, not just a latent/hypothetical case (confirmed via `grep` across `test/snapshots/` for
  bare `:>` on a `view` usage; no other fixture uses this form, and no fixture uses `:>` at all on
  a `view` usage other than this one).
- **Impact:** blocks lowering `view` usage declaration facts (`DeclarationKind::ViewUsage`) with
  full parity to the other `*Usage` kinds, since the in-scope requirement includes `:>`/`:>>`
  specialization resolving through the existing ancestor-closure fixed point (same shape as
  `occurrence`/`connection` usage, see commits `798d7287`/`d0675e2c`) and the typed AST currently
  offers no field to lower the `:>` relationship from for `view` usage specifically (`type_name`/
  `redefines`/`body`/`membership` are present and fine; only `subsets` is missing). `ViewDef`
  itself (the `def` form) is not blocked by this gap, so this slice lands `view def` lowering only
  (`DeclarationKind::ViewDefinition`, mirroring `InterfaceDefinition`'s def-only precedent, entry
  #6) and defers `view` usage lowering until this is fixed upstream. View-specific semantics
  (`render`, viewpoint `satisfy` binding, `expose`/`filter` composition) remain out of scope for
  this slice regardless and continue to surface as explicit unsupported diagnostics
  (`unsupported_view_definition_member` for `view def` body members not yet modeled).
- **Status:** blocking (usage side only). Needs an upstream `sysml-v2-parser` change adding
  `subsets: Option<Node<SubsettingRelationship>>` to `ast::view::ViewUsage` and wiring
  `parser::view::view_usage` to populate it from the `UsageHeader` it already computes (mirroring
  `OccurrenceUsage`/`StateUsage`/`PortUsage`). Not attempted here — out of scope for
  `sysml_resolution`/`sysml_query`/`spec42-snapshot` to work around without either (a) re-parsing
  the specialization clause independently (duplicating parser logic, fragile) or (b) shipping view
  usage declarations with unconditionally-absent subsetting facts, silently under-reporting a
  relationship real content (`11b-Safety and Security Feature Views.sysml`) depends on.

### 9. `ConcernUsage` has no `specializes`/`subsets`/`redefines` field at all -- affects the `concern def` form itself, not just usage

- **Symptom:** `concern def ConcernCheck :> RequirementCheck { ... }` (real fixture content, see
  below) parses successfully (no `unsupported_grammar_form`, no parser-level error) but the `:>`
  relationship is silently discarded before it ever reaches `sysml_resolution`. Unlike every other
  `*def` kind covered so far, `concern def` is **not** modeled as its own struct at all:
  `ast::ConcernUsage` (crate `sysml-v2-parser`, `src/ast/requirement.rs`) is the single struct
  backing both the bare `concern ...` usage form and the `concern def ...` definition form, with
  `is_definition: bool` as "the sole discriminator between the two" (per the struct's own doc
  comment). It carries only `name: String`, `type_name: Option<QualifiedReferenceId>` (a bare
  reference, not a structured `TypingRelationship`), `body: RequirementDefBody`, `is_definition`,
  and `membership` -- no `specializes`/`subsets`/`redefines` field of any kind.
  `parser::requirement::concern_usage` (`src/parser/requirement.rs`, lines 976-995) parses the
  header via `feature_usage_header` (which does parse `:>`/`:>>` into `UsageHeader::subsets`/
  `redefines`) but only reads `header.type_reference` when constructing the node, dropping
  whatever `:>`/`:>>` clause was parsed -- for *both* the `concern` and `concern def` textual
  forms alike, since they share the exact same parser function and AST struct.
- **Representative input:** `concern def ConcernCheck :> RequirementCheck { ... }` (Systems
  Library `Requirements.sysml`, `test/snapshots/sysml.library/requirements.md:162`).
- **Representative snapshots:** `test/snapshots/sysml.library/requirements.md`,
  `test/snapshots/sysml/coverage_individual.md` (`individual concern def D11;`),
  `test/snapshots/sysml/examples/view_test.md` (`concern def C { ... }`, no `:>`),
  `test/snapshots/sysml/examples/sys_ml_v2_spec_annex_a_simple_vehicle_model.md`
  (`concern def VehicleSafety { ... }`, no `:>`).
- **Impact:** blocks lowering `concern def` (`DeclarationKind::ConcernDefinition`) with parity to
  every other `*Definition` kind landed so far, since the in-scope requirement includes an
  optional `:>` specialization relationship resolving through the existing ancestor-closure fixed
  point (same shape as `requirement def`/`viewpoint def`) and the typed AST currently offers no
  field to lower that relationship from -- for the def form specifically, not merely for a
  separate usage form the way every prior "def-only lands, usage deferred" precedent (#5, #6, #8)
  worked. This is a **both def and usage affected** gap per the task's stop condition, not a
  def-only-lands slice: shipping `concern def` lowering here would either silently drop a real
  `:>` relationship real Systems Library content depends on (`ConcernCheck :> RequirementCheck`)
  or require re-parsing the clause independently (duplicating parser logic, fragile, out of
  scope). Not attempted here; all four fixtures above stay on the `unsupported_package_member`
  fallback for their `concern`/`concern def` members.
- **Status:** blocking (both def and usage). Needs an upstream `sysml-v2-parser` change either
  splitting `concern def` into its own `ConcernDef` struct carrying a proper
  `specializes: Option<Node<TypingRelationship>>` field (mirroring `RequirementDef`/`ViewpointDef`,
  the cleanest fix given `concern def`'s own BNF `ConcernDefinition` production already exists
  separately from `ConcernUsage` per the spec) or adding `subsets`/`redefines: Option<Node<
  SubsettingRelationship>>` fields to the shared `ConcernUsage` struct and wiring
  `parser::requirement::concern_usage` to populate them from the `UsageHeader` it already
  computes. Not attempted here -- out of scope for `sysml_resolution`/`sysml_query`/
  `spec42-snapshot` to work around.

### 10. Bare forward-declared `classifier X;` (and `feature`) collapse to a raw-text fallback node -- no name/membership field at all

- **Symptom:** `classifier SpatialFrame;` (a `;`-terminated forward declaration with no body)
  produces `unsupported_grammar_form` (source: parser), cascading to
  `unsupported_package_member` (semantic). Zero declaration is published for `SpatialFrame`, so
  every reference to it -- including a same-file `alias multiplicity for SpatialFrame;` --
  reports `unresolved_reference` even though nothing about the reference or the resolver's
  lookup is at fault: there is no declaration to find.
- **Root cause:** `PackageBodyElement::ClassifierDecl` (crate `sysml-v2-parser`,
  `src/ast/kerml_fallback.rs`) is a raw catch-all fallback struct -- `{ keyword: String, text:
  String }`, no structured name/body/membership fields whatsoever -- functionally identical to
  `FeatureDecl` (see gap #1) but for the `classifier`/`class` bare-declaration grammar form.
  `sysml_resolution`'s lowering (`crates/sysml_resolution/src/model.rs`,
  `PackageBodyElement::ClassifierDecl` arm) has no typed field to read a name or members from,
  so it can only call `push_unsupported`; there is no parser-contract surface in scope to lower
  against.
- **Representative input:** `classifier SpatialFrame;` at package scope.
- **Representative snapshot:** `test/snapshots/kerml/keyword_as_name.md` (line ~23; the
  `alias multiplicity for SpatialFrame;` at line ~30 stays `unresolved_reference` purely as a
  downstream consequence -- verified by direct declaration-list inspection: only the enclosing
  `Package` and the two `Alias` declarations exist for that file, `SpatialFrame` has zero
  declarations of any kind).
- **Status:** blocking, same shape as gap #1/#2 (raw catch-all fallback node, no structured AST
  to lower from). Needs an upstream `sysml-v2-parser` change giving bare/forward-declared
  `classifier`/`class` declarations (and their KerML `feature` sibling, see gap #1) a proper
  typed AST node with a `name` field, mirroring the body-bearing forms. Not attempted here --
  out of scope for `sysml_resolution`/`sysml_query`/`spec42-snapshot` to work around without
  re-parsing the fallback `text` blob independently (duplicating parser logic, fragile).

### Cases traced back to gap #1, not new gaps

- `test/snapshots/kerml/inheritance.md`'s `alias z for y::g;` (same-file qualified lookup to a
  directly-owned member) and `alias us for w::g;` (same-file qualified lookup through a
  `subsets` relationship) were investigated as possible `sysml_resolution` visibility/qualified-
  lookup bugs (the originating hypothesis was a KerML-vs-SysML default-visibility mismatch in
  `exported_names`/`MembershipIndex`). Direct declaration-list inspection during this
  investigation shows this hypothesis does not hold: for this fixture, only the enclosing
  `Package` plus the two top-level `Alias` declarations (`z`, `us`) exist at all. `class A`,
  `class B specializes A`, `feature y: A { ... feature g redefines f; }`, and `feature w subsets
  y;` are every one of them entirely unlowered -- each produces its own
  `unsupported_grammar_form`/`unsupported_package_member` pair, the exact fallback described in
  gap #1 (`kerml_semantic_decl`'s `"class"`/`"feature"` keyword-starter catch-all, not a
  dedicated typed node). `y` and `w` never become resolvable `DeclarationId`s in the first place,
  so `y::g`/`w::g` cannot resolve regardless of any visibility rule -- there is nothing in
  `sysml_resolution` to fix for these two cases specifically. No resolver change was made; no
  fixture changed as a result of investigating this pair. Gap #1 already tracks the fix (and its
  scale note already covers `class`, not just `feature`, per the shared `kerml_semantic_decl`
  fallback path in `sysml-v2-parser`).

## Resolved / not blocked (kept for history)

- Alias declarations (`alias X for Y;`) — investigated as a possible parser gap, but the typed
  AST (`AliasDef.target`) was already a structured `QualifiedReferenceId`. Fixed entirely in
  `sysml_resolution` (commit `422e2216`), not a parser gap.
- Enum definitions (`enum def`, `EnumeratedValue`) — investigated, typed AST already exposes
  stable per-literal identity/spans. Fixed entirely in `sysml_resolution` (commit `99d5ea39`).
