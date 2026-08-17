# Upstream sysml-v2-parser gaps blocking spec42 snapshot work

Tracks semantic gaps discovered while closing the snapshot delta on the parser-owned pipeline that
trace back to the pinned `sysml-v2-parser-next` revision (`7d4fd85`) rather than to
`sysml_resolution`/`sysml_query`. Each entry carries enough detail to file/update an upstream issue
against `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

Every entry below was re-verified against `7d4fd85` by direct inspection of the parser checkout, by
a scratch fixture run through `cargo run -p spec42-snapshot`, or both.

## Open

- Gap 57. The anonymous subsetting/redefinition shorthand member -- `:> annotatedElement :
  SysML::Usage;`, `:>> baseType = ...;` -- authors no declared name, but the parser populates the
  member's `name` field with the subsetting target's spelling *and* reports the same spelling as the
  `subsets`/`redefines` target. Verified against `7d4fd85` with a scratch fixture: `metadata def M {
  :> annotatedElement : SysML::Usage; }` yields an `AttributeUsage` whose `name` is
  `"annotatedElement"` alongside a subsetting relationship to `annotatedElement`.

  KerML gives such a feature the subsetted feature's *effective* name; it does not give it a
  declared one, and the difference is load-bearing for scope. The declared name puts the feature
  into its own owner's owned name tier under exactly the name it is looking up, and that tier
  shadows the inherited one the author meant, so the feature subsets itself. Two declarations in
  `test/snapshots/sysml.library/requirement_derivation.md` are reported as `specialization_cycle`
  for this reason, and every conformance question about them answers from a self-loop.

  `sysml_resolution` cannot correct it locally. Excluding a specialization reference's own source
  from its scope -- which it does for `Redefinition`, per
  `planning/RESOLUTION_LAYER_DESIGN.md` section 11.1 -- is not enough here, because a metadata
  definition commonly authors several of these shorthand members and they all acquire the same
  declared name: excluding only the reference's own source makes two of them resolve to each other,
  turning a self-loop into a two-cycle. Distinguishing a declared name from an effective one needs
  the parser to stop reporting one, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 58. None of the four connection-like definition nodes carries an abstractness field:
  `ConnectionDef`, `FlowDef`, `AllocationDef` and `InterfaceDef` (`src/ast/structure.rs`,
  `src/ast/view.rs`) have `is_individual` and a `derivation_role` but no `is_abstract` and no
  `definition_prefix`, unlike `PartDef`/`ItemDef`/`AttributeDef`, which all carry one. The
  `abstract` prefix parses and is then dropped: `abstract connection def C { end only : T; }`
  lowers with no modifier fact at all. Confirmed with a scratch fixture at `7d4fd85`; the corpus
  authors eight such declarations (`Causation`, `Derivation`, `Multicausation`,
  `ControllingMeasure`, and others).

  An abstract connection-like definition is legitimately allowed an incomplete end set, so
  `sysml_resolution`'s end-count rules cannot exempt one. They keep the guard, which becomes
  correct as soon as the field exists; until then
  `test/snapshots/resolution/structural_feature_conformance.md` shows an abstract declaration being
  reported, so the limitation stays visible. One field per node mirroring `PartDef`, filed upstream
  against `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 59. No spelling authors an end feature that also carries a direction, so KerML
  8.3.3.3.1's prohibition on exactly that combination has no authorable violation.
  `KermlFeatureMember` carries `is_end` and the other modifier flags, but the parser accepts no
  direction alongside `end`: `in end feature x : T;`, `end in feature x : T;` and SysML's `end in
  port x : T;` all fail with `unexpected_keyword_in_scope`, while `derived end feature x : T;`,
  `composite end feature x : T;` and `abstract end feature x : T;` all parse and lower correctly.
  Verified with a scratch fixture at `7d4fd85`.

  `sysml_resolution` therefore does not publish an `end_feature_has_direction` code: it could never
  fire, and an untestable diagnostic is worse than a recorded gap. The sibling restrictions
  (derived/abstract/composite) are published and covered. Needs the direction prefix accepted
  alongside `end` on `KermlFeatureMember` and the SysML usage prefixes, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 41. KerML's implicit self-reference identifier `that` (e.g.
  `test/snapshots/sysml/examples`'s `trig_functions.md`: `inv unitBound { -1.0 <= that & that <=
  1.0 }` inside `datatype UnitBoundedReal :> Real { ... }`, 111 fixtures overall) has no
  lexically-distinguished status in the parser. `src/parser/lex.rs`'s `SYSML_RESERVED_KEYWORDS`
  table -- the parser's own reserved-word list, which tells a genuine language keyword apart from
  an arbitrary identifier -- does **not** contain `"that"`, and a repo-wide grep of `src/` for
  `"that"`/`b"that"` finds no keyword, marker, or expression variant anywhere else either. It lexes
  as a plain, user-choosable identifier flowing through the ordinary `Expression::FeatureRef`
  lexical-lookup path, structurally indistinguishable from a real feature named `that`, so
  `sysml_resolution` cannot resolve it as a self-reference without matching the literal spelling --
  exactly the "reconstruct semantics from spelling" anti-pattern this codebase avoids.

  The same root cause covers `(that as Occurrence).member` (`sysml.library/occurrences.md`,
  `performances.md`, `state_performances.md`): an `Expression::MemberAccess` whose base is an
  `Expression::TypeCheck` wrapping a `that` `FeatureRef`. `flatten_member_access_chain` correctly
  declines it (its root is a cast, not a reference), so the whole chain falls through to the
  unsupported diagnostic rather than half-resolving `.member`.

  Needs `"that"` added to `SYSML_RESERVED_KEYWORDS`, or an equivalent lexically-distinguishing AST
  marker on `FeatureRef` / a dedicated `Expression::ImplicitThat` variant, before there is anything
  here to resolve.

- Gap 42. `RequirementDefBodyElement` (`src/ast/requirement.rs`) is a closed enum covering only a
  small, hand-picked subset of the member kinds a `requirement def` body may legally contain --
  unlike sibling body enums (`PartDefBodyElement`, `ActionDefBodyElement`,
  `ConstraintDefBodyElement`, and now `StateDefBodyElement`), it has no variant for the general
  action/succession/port usage-member zoo, nor for nested definitions of its
  own kind. Re-verified against `7d4fd85`, which closed three of the sub-gaps -- the enum now
  carries `RefDecl`, `ConcernUsage`, and `CalcUsage` variants, all dispatched by
  `lower_requirement_shaped_body`. Still missing: a parameter-member variant for `in
  ref`/`in calc` members (`test/snapshots/sysml.library/trade_studies.md`), a `Port`/`Allocate`
  variant (`sys_ml_v2_spec_annex_a_simple_vehicle_model.md:912,877`), a nested-`requirement def`
  variant (only a `RequirementUsage` variant exists, not a def;
  `requirement_test.md:10`'s `requirement def <'1'> A { ... }` nested inside another `requirement
  def` body), and support for a bare `requirement;` member with no name/body at all
  (`requirement_test.md:9`). `FrameMember` (dispatched via `RequirementDefBodyElement::Frame`) has
  the same narrowness one level down: its parser production only ever parses `frame <name>
  <body>`, with no alternative for the `frame concern <name> : <Type>;` sub-form (BNF
  `FrameConcernMember`, `sys_ml_v2_spec_annex_a_simple_vehicle_model.md:1546`'s `frame concern
  vs:VehicleSafety;`) -- `name()` greedily consumes `concern` as the declared name, then the body
  parser fails on the leftover ` vs:VehicleSafety;`, and the whole member falls to recovery. None
  of this is a mechanical spec42-side dispatch gap (there is no already-typed node these fall
  through to un-dispatched); each needs new upstream AST variants/parser productions before
  `sysml_resolution` has anything to lower. The `state def` half of this gap is closed:
  `StateDefBodyElement` now carries `AttributeUsage`/`ActionUsage`/`SuccessionUsage`/
  `AssertConstraint` variants, all dispatched by `lower_state_def_body`.

- Gap 52. Three SysML declaration-modifier prefixes have no representation in the pinned parser at
  all, so the canonical declaration-fact family (`DeclarationModifiers` in
  `crates/sysml_resolution/src/model.rs`) cannot include them and an element inspector cannot
  report them: `readonly`, SysML `variable`, and `unique`. Re-verified against `7d4fd85` -- the AST
  design note at `src/ast/membership.rs` states they are deliberately out of scope; `readonly` and
  `variable` have no token, field, or starter anywhere in `src/parser`, and `unique` is now
  consumed as a recognized default (`src/parser/attribute.rs`, "recognized and consumed, but not
  recorded") without reaching a field. (`unique` is
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
  inspection of `src/ast/`, re-verified field-by-field against `7d4fd85`:
  (a) **no `multiplicity` field**: `AttributeDef` (`structure.rs`; contrast `AttributeUsage`, which
  has one), `ConstraintUsage` (`view.rs`), `RequirementUsage` (`requirement.rs`), `CalcUsage`
  (`view.rs`), `RequirementActorDecl` (`requirement.rs`; contrast `ActorUsage`, which has one);
  (b) **no `nonunique` field**: `PartUsage` (`structure.rs`), which does carry `ordered`;
  (c) **no `short_name` field**: `ActionUsage`, `OccurrenceUsage`, `ConstraintUsage`, `RefDecl`,
  `EndDecl`, `ReturnDecl`, `ViewUsage`, so the `<short> name` spelling is dropped for those kinds
  even though `AttributeUsage`/`PartUsage`/`PortUsage`/`ItemUsage`/`RequirementUsage` all keep it.
  Each is a one-field addition mirroring an existing sibling, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 55. Only the three keyworded annotation productions survive into the AST: `doc /* ... */`
  (`ast::DocComment`), `comment /* ... */` (`ast::CommentAnnotation`), and
  `rep <language> "..." /* ... */` (`ast::TextualRepresentation`). Plain `/* ... */`, doc-style
  `/** ... */`, and `//` line comments are consumed as lexer trivia by `trivia_len`
  (`src/parser/lex.rs`'s `trivia_len`, called from `ws_and_comments`) and are unreachable from
  any AST node, so an element inspector can never surface a `/** ... */`-style doc block. Separately,
  `DocComment.text` is the raw byte slice between `/*` and `*/` (`src/parser/requirement.rs:724-726`)
  with no leading-`*` stripping and no dedent, so every consumer must normalize it identically or
  they will disagree. Not necessarily a defect -- keeping trivia out of the AST is a defensible
  design -- but it is a hard ceiling on documentation fidelity and is recorded here so the
  limitation is not rediscovered. If `/** ... */` is intended to be an annotation rather than
  trivia, it needs its own production, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 56. `EnumerationBody` is `Body<EnumeratedValue>` (`src/ast/structure.rs`), so its
  `Brace { elements }` holds *only* enumerated values, so an `enum def`'s own body
  annotations are discarded before they reach the AST: `enum def Color { doc /* ... */ enum red; }`
  parses with the `doc` element dropped entirely, unlike every sibling body-element enum
  (`PartDefBodyElement`, `AttributeBodyElement`, `RelationshipBodyElement`, …), all of which carry
  `Doc`/`Comment`/`TextualRep` variants. Confirmed against `cb026cd` while lowering the
  documentation fact family, and still true at `7d4fd85` (the shared `Body<E>` container
  rename from `values` to `elements` did not widen the member type):
  `test/snapshots/documentation_in_bodies.md`'s `enum def Color`
  authors a doc comment that no fact family can recover, while the `part def`/`attribute def`/
  `item def`/`part` usage/`alias` docs in the same fixture all lower correctly. The same node also
  discards each `EnumeratedValue`'s own optional body and `= expr` initializer (`structure.rs`
  documents this: "Only the name and its span are retained"), so a per-literal doc is unreachable
  too. Needs `EnumerationBody::Brace` widened to a proper body-element enum carrying at least
  `Doc`/`Comment`/`TextualRep` alongside the values, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

## Typed upstream, not yet lowered here

Not upstream gaps: the parser now carries these typed facts, and the remaining work is
`sysml_resolution` lowering. Recorded so the coverage they represent stays visible rather than
disappearing with the gap entries they closed.

- **KerML explicit relationship declarations.** `ast::KermlRelationshipDecl`
  (`PackageBodyElement::KermlRelationship`) models `subtype`/`subclassifier`/`typing`/`subset`/
  `redefinition` with their optional `specialization <ident>` prefix, plus `disjoint ... from`,
  `inverse ... of`, and `featuring ... by`. `lower_package_element` reports every one as
  `unsupported_package_member`. This is the one construct whose semantic coverage *narrowed* with
  the parser bump: `subclassifier X specializes Y;` previously reached
  `KermlClassifierDecl` and lowered `X` as a `kerml-classifier` declaration, which named the
  relationship's source as though it were a new classifier
  (`test/snapshots/sysml.library/occurrences.md`).

- **The anonymous, header-less `allocate <source> to <target> { ... }` package member.** It now
  parses to `PackageBodyElement::AllocationUsage`; `lower_package_element` reports it unsupported.
  `AllocationUsage` additionally gained `subsets`/`redefines` and typed `KermlConnectorEnd`
  source/target ends, none of which are lowered.

- **`ViewpointUsage.subsets`/`redefines`** and **`SubjectDecl.redefines`**, both new specialization
  clauses their lowerings do not read; `SubjectDecl.value` is now a `FeatureValue`, so a subject's
  authored value spelling can be recorded through `record_feature_value` like every other one.

- **`EntryAction`/`DoAction`/`ExitAction` declaration facts.** `declared_name`, `type_name`,
  `redefines`, and `effect` are typed; `lower_state_entry_action` and its siblings still read only
  `action_reference` and `body`, so `entry action entryAction :>> 'entry';` lowers as the reference
  form.

- **`VariantTypedUsage::Requirement`.** `lower_variant_usage` dispatches only the `Perform` arm; a
  `variant requirement r1;` inside a `variation requirement` body stays unsupported.

- **`MetadataKeywordUsage.type_reference`.** The `#<Name>` shorthand now carries a resolvable
  `QualifiedReferenceId` alongside its keyword spelling, but every scope still reports the member
  as unsupported rather than resolving it the way `@Name` (`lower_metadata_annotation`) does.

- **`RequireConstraint.target`.** The keyword-less `require <qualified.name>;` shorthand now
  carries an arena-backed target; `lower_require_constraint_member` still defers the whole
  `has_constraint_keyword == false` form.

- **`KermlFeatureMember.crosses` and `is_const`.** Both typed; neither is lowered as a fact.

- **`CollectionOperatorBody.doc`.** A collection operator body's `doc /* ... */` annotation is
  typed upstream, but the whole collection-operator expression family is still unlowered here
  (`lower_expression` has no arm for it), so there is no declaration to attach the fact to yet.

- **`ItemUsage.subsets`.** `lower_item_usage` lowers `redefines` but not the `:>` clause, so an
  authored `item items : Item[0..*] nonunique :> objects;` publishes its typing and drops its
  subsetting. Pre-dates the `7d4fd85` bump; the one-line fix is the same
  `lower_subsetting_relationship` call every sibling usage already makes.
