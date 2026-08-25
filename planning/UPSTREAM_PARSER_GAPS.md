# Upstream sysml-v2-parser gaps

This is the active record of information the parser must preserve or distinguish before spec42 can
implement the corresponding semantic or syntax-fidelity behavior without guessing.

The canonical parser currently pinned by the root workspace is
`lukewilliamboswell/sysml-v2-parser@34fd6c4976dc299c3ceb27bf4dc3f15170078408`. Every gap below was
re-exercised against that exact revision, one spelling per document through `spec42 check` (a
second error in the same document suppresses the first as `recovery_cascade_suppressed`, which
made an earlier multi-spelling probe read as "parses"), and by re-reading the owning
`sysml_resolution` lowering; the entries the bump closed were removed rather than annotated. New
upstream work must be based on the full pinned identity, not an abbreviated revision or the old
`sysml-v2-parser-next` dependency alias.

The bump from `c1677e75d3b0b4d2b806fbdf438c2bfb1dfc1056` (12 upstream commits in the parser
speculation-removal performance series) leaves the open gaps below unchanged. Corpus regeneration
newly retains the `#Safety feature z1 : T;` extended usage in
`kerml/coverage_features_advanced.md` as an explicit `unsupported_package_member` rather than
omitting that unsupported semantic member.

The bump from `f52100fd71b5950fba6a8e9ba2760f1a1887ce34` (40 upstream commits: the "gaps wave 2"
and "corpus snapshot wave 3" work, the parser performance pass and the span-backed authored-text
migration) closed gaps 59, 64, 65, 67, 70,
72, 73, 75, 80 and 81 outright and narrowed 62, 66, 74 and 76. It opened one gap, recorded below as
82. It also removed the standard library's last parse recoveries (`Flows.sysml`,
`Interfaces.sysml`, `Items.sysml`, `CausationConnections.sysml`), so the library's publication is
now honestly `unsupported-syntax` rather than `parse-recovery`; the seven members that keep it
there are listed under "Library members still unsupported".

Lowering that the bump unblocked has landed with it: `end` on every occurrence-usage family with
its owned cross feature; `#Tag` extension keywords and the keyword-less `#Tag <name>`
`ExtendedUsage`; `MetadataBody` definition, alias and import members; the declared
`include use case <name> : <Type>;`; `require constraint <name> : <Type>;`'s typing; the
`perform <path>;` reference target; named connector ends; `end derived x : T;` / `end in x : T;`
prefixes; the `parallel` state body modifier; and the `conjugates` / `~` conjugation part. The
members the bump newly types but whose lowering has not caught up are listed under "Typed
upstream, not yet lowered here".

## Ownership and evidence rules

- An **upstream gap** means the parser rejects legal syntax, accepts it but drops authored
  information, or represents two semantically distinct authored forms identically. Spec42 must not
  recover such information by scanning source text or matching display strings.
- A **spec42 migration** means the pinned parser already exposes the required typed node, source
  span, recovery state, traversal, or arena lookup, but a consumer still uses the legacy AST API.
  That work belongs in spec42 and is not grounds for extending the parser with editor or semantic
  policy.
- A **grammar exclusion** is neither: the normative textual grammar has no spelling for the
  shape, so the parser is right to refuse it. Those are recorded in the snapshot registry as
  `abstract_syntax_coverage_gap` issues, not here.
- Each upstream fix needs a parser regression test for accepted and malformed/recovery input as
  appropriate, provenance validation when new spans or arena identities are introduced, and a
  spec42 owning-layer test proving that no source-text reconstruction remains.
- Closing a gap requires re-verifying it against the newly pinned full commit and removing the entry
  from this active plan. Git history, not a completed section here, records the old gap.

## Open semantic grammar and provenance gaps

Each entry below was verified against the pinned revision by parsing a probe fixture with it
directly, by re-reading the owning `sysml_resolution` lowering, or both. Every entry must be
rerun against the exact replacement revision when fixed.

| Gap | Information unavailable to consumers | Minimum upstream acceptance evidence |
| --- | --- | --- |
| 62 | A KerML `Flow` with two payload features cannot be authored, so the at-most-one rule has no violating side | Accept the repeated `of <payload>` clause, or diagnose the second occurrence; prove `flow of Thing of Thing from source to target;` is either represented or reported, never `recovered_calc_body_element` |
| 61 | `message` has no member variant in a calc-shaped body | Give `message` a typed member variant in the calc-shaped body grammar; prove `message m of T;` produces one node whose keyword never reaches the AST as a feature reference |
| 66 | How many `crosses`/`references` clauses one feature authored: two clauses collapse into one relationship | Model each *clause* separately, or diagnose the second occurrence; prove `feature f crosses a crosses b;` is distinguishable from a single clause naming two targets |
| 69 | A *binding* connector declaration cannot carry its end-feature body | Add the typed end-feature-body production for binding-connector declarations; prove a three-ended binding reaches one connector plus all three ends |
| 74 | `require constraint` outside a requirement-shaped body | Admit the member independently of the owner validation rule; prove `part def H { require constraint c : C; }` reaches one typed member rather than `unexpected_keyword_in_scope`, so semantics can report the invalid owner |
| 76 | The shorthand `else` branch as an action-body statement | Add the `if <cond> then <a> else <b>;` alternative; prove it reaches one member with both branches, as `if <cond> then <a>;` already does |
| 77 | Transition effect-action members in a state body, and a `transition` member in an action body | Add the typed transition-body member variants; prove a transition effect action parses in a state body and a `transition` member is admitted in an action body instead of `unexpected_keyword_in_scope` |
| 78 | `abstract` paired with `variation`, in either order | Accept the modifier pair on definition and usage declarations; prove `abstract variation part def Good;` and `abstract variation part good : Base;` parse, as the bare `variation` spellings already do |
| 79 | `expose` in a package body, and `verify`/`render` in a part-definition body | Admit these member forms independently of the owner validation rule; prove their typed owners and spans reach the AST so semantics can report the invalid owner rather than the parser rejecting the spelling |
| 82 | **Regression at the pinned revision.** The kind keyword of a directed parameter (`in action body { ... }`) is consumed and dropped | Record the keyword on `InOutDecl` (or produce the kinded usage node with its direction); prove `in action body { }` and `in body { }` are distinguishable to lowering |
| 41 | Lexically distinguished implicit `that` self-reference | Produce a dedicated typed form that cannot collide with a user declaration; cover bare, cast, and member-access expressions |
| 52 | `readonly` and `variable` modifiers | Preserve presence and token spans independently from effective/default values |
| 55 | `//` and `/** ... */` comment fidelity, and `DocComment` text normalization | Decide and test whether doc-style trivia is syntax; if syntax, preserve kind, raw span, and one normalized-text policy centrally |

The contribution target is the pinned `lukewilliamboswell/sysml-v2-parser` repository. References
below to `elan8/sysml-v2-parser#121` record where the arena-backed work originated; they do not
authorize changing spec42 to follow a moving upstream branch. A fix is consumed only by updating
the single full revision in spec42 and regenerating the lockfile through the normal dependency
workflow.

- Gap 61. One member spelling of three remains unrepresentable in a calc-shaped body, and it is
  rejected honestly rather than shredded: `classifier C { message m of T; }` is
  `unexpected_keyword_in_scope` at `34fd6c4`, while `flow a.y to b.x1;` and `redefines
  predecessors [0];` reach typed nodes. The KerML message declaration cannot be authored in a
  `classifier`, `struct`, `class` or `behavior` body.

- Gap 62. KerML flow declarations parse and lower, including the declaration-led form (see
  "Lowering that the bump unblocked"). What remains is the *at-most-one-payload* rule's violating
  side: `behavior M { flow of Thing of Thing from source to target; }` is
  `recovered_calc_body_element` at `34fd6c4`, so KerML 8.3.4.9.2 `validateFlowPayloadFeature` has
  no authorable violation and `tests/snapshots/validation/kerml_flow_payload_feature.md` stays
  blocked.

- Gap 66. Clause *count* is unobservable. At `34fd6c4` `feature two crosses source crosses
  target;` parses, and both targets now lower and resolve as `crossSubsetting` references (the
  `unsupported_reference` they used to settle as is gone: reference and cross subsetting joined
  the subsetting resolution pass). What the AST cannot express is that the author wrote two
  separate clauses: `KermlFeature::crosses` is one `Option<Node<SubsettingRelationship>>` whose
  `target` is a `Vec`, indistinguishable from one clause naming two targets. KerML 8.3.3.3.4
  states both `validateFeatureOwnedCrossSubsetting` and `validateFeatureOwnedReferenceSubsetting`
  as "at most one **clause**" rules, so the violation cannot be observed.
  `kerml_feature_owned_cross_subsetting.md` and `kerml_feature_owned_reference_subsetting.md` are
  blocked on `parser-gap-66-subsetting-clause-count`.

- Gap 69. Unchanged: a binding connector with an end-feature body is `unexpected_keyword_in_scope`
  at `34fd6c4`, so `kerml_binding_connector_is_binary.md`, `kerml_connector_binary_specialization.md`
  and `generated_conditional_binary_connector_specialization.md` stay blocked.

- Gap 74. Narrowed. The declared `require constraint c : C;` / `assume constraint a : A;` parse in
  a requirement-shaped body with their typing (`RequireConstraint::typing`), and lower, so
  `generated_conditional_requirement_constraint_specialization.md`,
  `sysml_requirement_constraint_membership_is_composite.md` and
  `generated_requirement_constraint_derived_facts_parser_gap.md` are unblocked. What remains is the
  owner rule's violating side: `part def H { require constraint c : C; }` is
  `unexpected_keyword_in_scope` at `34fd6c4`, so
  `validateRequirementConstraintMembershipOwningType` cannot be authored and
  `sysml_requirement_constraint_membership_owning_type.md` stays blocked.

- Gap 76. Narrowed to one spelling. `accept when true;` and `accept at now;` parse in an action
  body and lower through `lower_accept_trigger`; the three trigger-argument fixtures now wait on a
  semantic rule (`semantic-trigger-invocation-argument-typing`) and the accept derived-fact
  fixtures on the action-parameter identity gap. `action def P { action a1; action a2; if true
  then a1 else a2; }` is still `recovered_action_body_element` at `34fd6c4`, so
  `sysml_if_action_usage_parameters.md` stays blocked.

- Gap 77. Unchanged: `transition aTransition first start accept apayload : Anything via receiver
  then done;` in a state body and a `transition` member in an action body are
  `recovered_state_body_element` and `unexpected_keyword_in_scope` respectively at `34fd6c4`.

- Gap 78. Unchanged. Probed one spelling per document at `34fd6c4`: `abstract variation part def
  Good;` is `recovered_package_body_element`, `abstract variation part good : Base;` is
  `recovered_part_def_body_element`, and `attribute def X { abstract variation attribute a; }` is
  `unsupported_grammar_form`; the bare `variation` spellings parse. Seven fixtures stay blocked.

- Gap 79. Unchanged: `expose` in a package body is `recovered_package_body_element`, and `verify`
  / `render` in a part-definition body are `unexpected_keyword_in_scope` at `34fd6c4`.

- Gap 82. **Regression at the pinned revision**, visible only now that `perform <path>;` resolves
  its target. `src/parser/action.rs`'s `in_out_decl_inner` consumes an `action` keyword after the
  direction ("Library shorthand: `in action body { ... }` (treat as name `body` typed as
  action)") and binds nothing: `InOutDecl` has no slot for it, so `Actions.sysml`'s `in action
  body { ... }` reaches lowering as a plain parameter and `then perform body;` performs a
  `ParameterUsage`. `sysml_resolution`'s `validatePerformActionUsageReference` therefore skips a
  parameter target rather than report a defect the author did not write
  (`check/host.rs`, `collect_behavior_structure`). Needs the keyword recorded, or the kinded
  usage node produced with its direction.

- Gap 41. KerML's implicit self-reference identifier `that` has no lexically-distinguished status
  in the parser: `SYSML_RESERVED_KEYWORDS` (`src/parser/lex.rs`) does not contain `"that"`, so it
  lexes as a plain identifier flowing through the ordinary `Expression::FeatureRef` path,
  structurally indistinguishable from a real feature named `that`. The same root cause covers
  `(that as Occurrence).member`. Needs `"that"` reserved, or a dedicated `Expression::ImplicitThat`
  variant, before there is anything here to resolve.

- Gap 52. `readonly`, `variable` and `var` have no representation in the pinned parser's SysML
  usage prefixes (`unrecognized_declaration_in_scope`), so `validateAssignmentActionUsage`'s
  time-varying fact never reaches semantics and `sysml_assignment_action_usage.md` stays
  blocked. The KerML `var` prefix on a feature is reachable and lowered. Related grammar
  exclusion, not a parser gap: KerML's `EndFeaturePrefix` spells only `const? end` and `var`
  lives in the exclusive `BasicFeaturePrefix` alternative, so a variable end feature has no
  spelling in either language (`abstract-syntax-nonrepresentable-variable-end`).

- Gap 55. Two comment forms remain unreachable from the AST (`//` and `//* ... */` are consumed
  as trivia; `/** ... */` is a plain block comment), and `DocComment.text` is the raw byte slice
  with no normalization policy. A keyword-less `/* ... */` in member position is a real
  annotating element with `keyword_span: None`. Keeping trivia out of the AST is a defensible
  design; the entry stays so the ceiling on documentation fidelity is not rediscovered.

### Closed by grammar, not by the parser

Two violating sides this document used to attribute to gap 64 are grammar exclusions. KerML's
`ConjugationPart = ( 'conjugates' | '~' ) OwnedConjugation` (BNF 462) admits one clause per type
declaration, so a second owned Conjugation has no spelling (`classifier Two conjugates A
conjugates B;` is `unsupported_grammar_form`, honestly), and `TypeDeclaration` makes
`SpecializationPart` and `ConjugationPart` exclusive alternatives, so `classifier C conjugates A
specializes B;` is likewise refused. The first is recorded as
`abstract-syntax-nonrepresentable-second-conjugation`; the second is authorable through a
standalone `subclassifier C specializes B;` declaration, which is a spec42 lowering gap
(`lowering-gap-kerml-relationship-declarations`, below).

## Typed upstream, not yet lowered here

Not upstream gaps: the parser carries these typed facts, and the remaining work is
`sysml_resolution` lowering. Recorded so the coverage they represent stays visible rather than
disappearing with the gap entries they closed. Each was re-checked against the owning lowering
at pinned `34fd6c4`; entries whose lowering has since landed were removed.

- **KerML explicit relationship declarations.** `ast::KermlRelationshipDecl`
  (`PackageBodyElement::KermlRelationship`) models `subtype`/`subclassifier`/`typing`/`subset`/
  `redefinition` with their optional `specialization <ident>` prefix, plus `disjoint ... from`,
  `inverse ... of`, `featuring ... by` and, since this bump, `conjugation ... conjugate ...
  conjugates ...`. `lower_package_element` reports every one as `unsupported_package_member`.
  `tests/snapshots/sysml.library/occurrences.md`'s `subclassifier SelfLink specializes
  SelfSameLifeLink;` is the library's one such member, and
  `kerml_specialization_specific_not_conjugated.md` waits on it
  (`lowering-gap-kerml-relationship-declarations`).

- **`EntryAction`/`DoAction`/`ExitAction` declaration facts.** `declared_name`, `type_name`,
  `redefines`, and `effect` are typed; `lower_state_entry_action` and its siblings still read only
  `action_reference` and `body`, so `entry action entryAction :>> 'entry';` lowers as the reference
  form.

- **`variant attribute` in a `variation attribute def` body.** `ast::AttributeBodyElement` carries
  a `VariantUsage` variant; `lower_attribute_body_element` reports it as
  `unsupported_attribute_member` rather than delegating to `lower_variant_usage`.

- **`MetadataKeywordUsage.reference`.** The `#<Name>;` / `#<Name> { ... }` member (no declared
  usage) carries a resolvable `QualifiedReferenceId`, but every scope still reports the member as
  unsupported. The prefix form of the same keyword (`UsageExtensionKeyword`) is lowered.

- **`RequireConstraint.target`.** The keyword-less `require <qualified.name>;` shorthand carries an
  arena-backed target; `lower_require_constraint_member` still defers the whole
  `has_constraint_keyword == false` form.

- **`CommentAnnotation.keyword_span`.** Whether a comment member was written with the `comment`
  keyword is a grammatical fact with its own span; `sysml_resolution` does not record it, so the
  two spellings publish identically.

- **`KermlFeatureMember.is_const`.** `crosses`, `references`, `chains` and `inverse of` now lower
  and resolve; the `const` flag on a KerML feature is still not copied into the declaration
  modifiers.

- **`CollectionOperatorBody.doc`.** The whole collection-operator expression family is unlowered
  (`lower_expression` has no arm for it), so there is no declaration to attach the fact to yet.

- **`InterfaceEnd`'s declaration label and the n-ary `InterfacePart`.** `lower_interface_end`
  resolves every endpoint's target as a `ConnectorEnd` reference in both forms; the label and the
  `::>`/`references` spelling are typed but not yet recorded as facts.

- **`GuardedSuccession`'s owned body.** `lower_guarded_succession` lowers its source, guard and
  target; its `DefinitionBody` and the optional `succession` declaration's identification are
  typed upstream and not yet lowered.

- **`MetadataBodyUsage.ref_span` and `.operator`, `SubsettingRelationship.spelling`,
  `Conjugation.spelling`, `ConnectorEndName.operator`.** Authored spellings (`:>>` vs
  `redefines`, `crosses` vs `=>`, `conjugates` vs `~`, `::>` vs `references`) are typed upstream
  as emission provenance; the semantic publication deliberately records the relationship and not
  the spelling.

- **`StateBodyModifier::Initial`.** The pinned parser admits `state s initial { ... }`, which has
  no production in `StateDefBody`/`StateUsageBody` (SysML BNF 1192, Pilot `SysML.xtext`); lowering
  keeps it visible as `unsupported_state_definition_member` rather than inventing a fact.

## Library members still unsupported

The seven standard-library members that keep the 94-document library at `unsupported-syntax`,
none of them changed by this bump:

- `Occurrences.kerml`: `subclassifier SelfLink specializes SelfSameLifeLink;` (KerML relationship
  declaration, above).
- `Actions.sysml`: `in whileTest default {true} { ... }` twice (an expression body as a feature
  value; the expression family is unlowered).
- `Cases.sysml`: `return ref result[0..*] { ... }` (a `ref` return parameter).
- `Requirements.sysml`: `return result = allTrue(assumptions()) implies allTrue(constraints())
  { ... }` (a constraint definition's return with a value).
- `Views.sysml`: `satisfy requirement viewpointConformance by that { ... }`
  (`SatisfiedRequirement::Declaration`, deliberately unsupported: it declares an inline
  requirement rather than referencing one).
- `Actions.sysml`: `transition aTransition first start accept apayload : Anything via receiver
  then done;` inside a nested `state` of an action definition (gap 77's payload-accept spelling).
