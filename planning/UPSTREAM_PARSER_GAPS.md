# Upstream sysml-v2-parser gaps

This is the active record of information the parser must preserve or distinguish before spec42 can
implement the corresponding semantic or syntax-fidelity behavior without guessing.

The canonical parser currently pinned by the root workspace is
`elan8/sysml-v2-parser@9f00caf353581a3c0ccc13676c5d8829f90708b3` (parser `main`,
`PARSE_AST_VERSION` 256). It adds `elan8/sysml-v2-parser#139` (`elan8/spec42#140`): a targeted
`verify_requirement_expects_declaration` recovery for the invalid `verify requirement
<feature-chain>;` form (was the generic `recovered_requirement_body_element`) -- a
diagnostic-only change with no AST shape effect, so spec42 passes the code through unchanged. And
`elan8/sysml-v2-parser#138` (`elan8/spec42#138`): a use-case /
`analysis` / `verification` body is a SysML `ActionBody`, so `first`/`then`/`then done` now parse
into the shared `FirstStmt` / `ThenAction` nodes (removing the bespoke `FirstSuccession` /
`ThenDone`), and spec42's `lower_case_family_def_body` lowers them through the same
`lower_first_stmt` / `lower_then_action` the action-def and `entry`/`do`/`exit` state bodies use,
plus `then use case <name> { … }` through `lower_use_case_usage`. This is on top of
`a5557ea` (the Apollo 11 normative-form fixes for `elan8/sysml-v2-parser#132` / `#134` /
`elan8/spec42#100`) and `378e41b` (`#128`/`#137` item-usage trailing `:>`). The five Apollo forms
all reach typed lowering: forms 1, 3, and 5 (keyword-less feature usage in use-case-family bodies,
multiplicity before typing on nested action usages, and the `abstract` prefix on `def`-less
connection usages) landed with `#133`; `#135` closes form 2 (`return :>` anonymous subsetting,
whose chained `->select { … }->collect { … }->sum()` value the parser now retains instead of
recovering) and form 4 (`do action <name> { <body> }` routes the name to `declared_name`, and
spec42's `lower_state_{entry,do,exit}_action` now declares the nested action and walks its body).
Gaps 83 and 84 are removed. The gap list
below was last re-exercised against `65c67de8a38269f8bcaf1bc42500bde30083ff81` (the merge commit
for `elan8/sysml-v2-parser#129`, including the follow-up attribute-body recovery boundary fix),
one spelling per document through `spec42 check` (a
second error in the same document suppresses the first as `recovery_cascade_suppressed`, which
made an earlier multi-spelling probe read as "parses"), and by re-reading the owning
`sysml_resolution` lowering; the entries the bump closed were removed rather than annotated. The
upstream commits since `65c67de8` — directed `ref` declarations in port bodies, serde
prefix-tampering hardening, feature-chain targets on `verify`, and the Apollo 11 normative-form
fixes (`#133`, `#135`) — touch areas outside the remaining gap list; adopting them added a
`PortDefBodyElement::AliasDef` arm to port-definition lowering and the form 2 / form 4 lowering
above. New upstream work must be based on the full pinned identity, not an abbreviated revision or
the old `sysml-v2-parser-next` dependency alias.

The bump from `c81e0b69236d57c64df127104232b54f72646484` closes parser gaps 62, 66, 69, 74,
79 and 82: flow payload and feature-specialization clause identity, binding body ends, directed
action parameter kinds, and invalid membership owners now reach typed lowering. Gap 77 is narrowed
to the transition effect-action spelling in a state body; transitions in action bodies are typed.
The remaining unmet corpus expectations have explicit semantic or lowering blockers rather than
continuing to masquerade as parser gaps.

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
| 61 | `message` has no member variant in a calc-shaped body | Give `message` a typed member variant in the calc-shaped body grammar; prove `message m of T;` produces one node whose keyword never reaches the AST as a feature reference |
| 41 | Lexically distinguished implicit `that` self-reference | Produce a dedicated typed form that cannot collide with a user declaration; cover bare, cast, and member-access expressions |
| 55 | `//` and `/** ... */` comment fidelity, and `DocComment` text normalization | Decide and test whether doc-style trivia is syntax; if syntax, preserve kind, raw span, and one normalized-text policy centrally |

The contribution target is the pinned `elan8/sysml-v2-parser` repository (the canonical upstream;
the `lukewilliamboswell/sysml-v2-parser` fork used during the pipeline-rewrite cycle merged into it
as `#123` and is no longer the target). References below to `elan8/sysml-v2-parser#121` record
where the arena-backed work originated; they do not authorize changing spec42 to follow a moving
upstream branch. A fix is consumed only by updating the single full revision in spec42 and
regenerating the lockfile through the normal dependency workflow.

- Gaps 83 and 84 (Apollo 11 `spec42#100` forms 2 and 4) are closed by
  `elan8/sysml-v2-parser@a5557ea` (`#135`). Form 2: `return :>` anonymous subsetting parses and
  its chained `->select { … }->collect { … }->sum()` value is retained (`ret.value.is_some()`)
  instead of recovering as `recovered_calc_body_element`; `lower_return_decl`'s existing
  `is_subsetting` branch handles the target, and the collection-operator value stays a
  typed-but-unevaluated expression (the `lower_expression` collection-operator arm is a separate
  "typed upstream, not lowered" item, not a recovery). Form 4: `state_behavior_action_target`
  routes a name immediately followed by `{` to `declared_name`, and
  `lower_state_{entry,do,exit}_action` now takes a declared-action branch
  (`lower_state_declared_action`) that pushes the named `{Entry,Do,Exit}ActionBinding`
  declaration, lowers any `: Type` / `:>>` clause, and recurses through `lower_state_def_body`
  so the `first` / `then action` flow resolves against the action's own scope. Apollo 11's
  `Purpose/MissionPhasesPackage.sysml` states and the `satisfy … by …operations.<action>` feature
  chains in `MissionSpecificationPackage.sysml` now resolve.

- Gap 61. One member spelling of three remains unrepresentable in a calc-shaped body, and it is
  rejected honestly rather than shredded: `classifier C { message m of T; }` is
  `unexpected_keyword_in_scope` at `695b2b44`, while `flow a.y to b.x1;` and `redefines
  predecessors [0];` reach typed nodes. The KerML message declaration cannot be authored in a
  `classifier`, `struct`, `class` or `behavior` body.

- Gap 41. KerML's implicit self-reference identifier `that` has no lexically-distinguished status
  in the parser: `SYSML_RESERVED_KEYWORDS` (`src/parser/lex.rs`) does not contain `"that"`, so it
  lexes as a plain identifier flowing through the ordinary `Expression::FeatureRef` path,
  structurally indistinguishable from a real feature named `that`. The same root cause covers
  `(that as Occurrence).member`. Needs `"that"` reserved, or a dedicated `Expression::ImplicitThat`
  variant, before there is anything here to resolve.

- Gap 55. Two comment forms remain unreachable from the AST (`//` and `//* ... */` are consumed
  as trivia; `/** ... */` is a plain block comment), and `DocComment.text` is the raw byte slice
  with no normalization policy. A keyword-less `/* ... */` in member position is a real
  annotating element with `keyword_span: None`. Keeping trivia out of the AST is a defensible
  design; the entry stays so the ceiling on documentation fidelity is not rediscovered.

- Bare n-ary `connect (e1, e2, e3)` (`NaryConnectorPart` as the ConnectionUsage alternative
  `'connect' ConnectorPart`) recovers as `recovered_part_def_body_element` in a part definition
  body. The part-body dispatcher uses binary-only `connect_`, while named
  `connection … connect (e1, e2, e3)` parses through `connect_ends`. Evidence:
  `tests/snapshots/syntax/nary_bare_connect.md` and `tests/snapshots/syntax/connect_and_bind.md`.

### Closed by grammar, not by the parser

Gaps 76, 77, 78 and 52 were recorded as parser gaps because Spec42 fixtures used spellings the
pinned textual BNF does not contain. The compiler corpus now authors the productions instead
(`docs/reference/TEXTUAL-SYNTAX-INVENTORY.md`).

- Gap 76. `IfNode` (SysML BNF 1123-1138) has no `then` keyword; `ActionBodyParameter` is always a
  braced body. `if true { a1; } else { a2; }` is the production. The shorthand `if <cond> then
  <a> else <b>;` is not a parser omission.
- Gap 77. `EffectBehaviorMember` (`do`) precedes `then` in `TransitionUsage` (BNF 1277-1286).
  `transition first idle do notify then running;` is the production; `then … do …` is not.
- Gap 78. `BasicDefinitionPrefix` / `RefPrefix` are the exclusive slot `abstract` | `variation`
  (BNF 219, 278). Dual-keyword `abstract variation` is `abstract-syntax-nonrepresentable-abstract-variation`.
  Bare `variation` spellings parse; remaining variation rules are semantic/lowering blockers.
- Gap 52. SysML `RefPrefix` has `constant` and no `var`/`variable` keyword. KerML `var` remains
  in `BasicFeaturePrefix`. Assignment validation now authors constant vs non-constant SysML
  attributes (`semantic-assignment-action-usage`).

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
at pinned `695b2b44`; entries whose lowering has since landed were removed.

- **Newly typed parser-gap fallout.** Flow payload clauses, ordered feature-specialization clauses,
  binding body ends, invalid-owner requirement/view memberships, and action-body transitions now
  reach lowering. Their remaining semantic facts and validations are tracked by the typed blockers
  in `tests/snapshots/issues.toml`; they are no longer upstream parser gaps.

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
  then done;` inside a nested `state` of an action definition (payload-accept transition
  spelling in that nested state body).
