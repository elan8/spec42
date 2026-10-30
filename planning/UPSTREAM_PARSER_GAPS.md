# Upstream sysml-v2-parser gaps

The parser baseline is `elan8/sysml-v2-parser@8cee8c16e5bb26327bb5062a4e54767610f943ef`
(v0.58.0, AST 284). This file records remaining parser and lowering gaps. The immutable dependency
pin is owned by `Cargo.toml`; consumed parser changes are covered by the semantic snapshot corpus.

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
| occurrence-usage-typings | Every typing target of an `occurrence` usage: `ast::OccurrenceUsage::type_name` is one `QualifiedReferenceId`, so `occurrence x : A, B;` silently keeps only `A` (no recovery diagnostic), unlike `PartUsage`'s structured `typing` clause | Give `OccurrenceUsage` the same structured `TypingRelationship` clause `PartUsage`/`AttributeUsage` carry (every comma-separated target); prove `occurrence x : A, B;` produces both targets in order, so spec42 can lower each FeatureTyping |
| 41 | Lexically distinguished implicit `that` self-reference | Produce a dedicated typed form that cannot collide with a user declaration; cover bare, cast, and member-access expressions |
| 55 | `//` and `/** ... */` comment fidelity, and `DocComment` text normalization | Decide and test whether doc-style trivia is syntax; if syntax, preserve kind, raw span, and one normalized-text policy centrally |

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
  then done;` inside a nested `state` of an action definition (a payload-accept transition spelling).
