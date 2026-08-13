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
- **Scale:** ~131/503 snapshots blocked as of this writing (largest single blocked bucket).
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

## Resolved / not blocked (kept for history)

- Alias declarations (`alias X for Y;`) — investigated as a possible parser gap, but the typed
  AST (`AliasDef.target`) was already a structured `QualifiedReferenceId`. Fixed entirely in
  `sysml_resolution` (commit `422e2216`), not a parser gap.
- Enum definitions (`enum def`, `EnumeratedValue`) — investigated, typed AST already exposes
  stable per-literal identity/spans. Fixed entirely in `sysml_resolution` (commit `99d5ea39`).
