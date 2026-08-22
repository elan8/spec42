# Upstream sysml-v2-parser gaps

This is the active record of information the parser must preserve or distinguish before spec42 can
implement the corresponding semantic or syntax-fidelity behavior without guessing. It also records
the separate, downstream migration required to delete `spec42-sysml-parser`; the two categories must
not be conflated.

The canonical parser currently pinned by the root workspace is
`lukewilliamboswell/sysml-v2-parser@f52100fd71b5950fba6a8e9ba2760f1a1887ce34`. Every gap below was
re-exercised against that exact revision, by parsing a probe fixture with it directly and by
re-reading the owning `sysml_resolution` lowering; the entries the bump closed were removed rather
than annotated. New upstream work must be based on the full pinned identity, not an abbreviated
revision or the old `sysml-v2-parser-next` dependency alias.

The bump from `49bdf3f8b8a90b64048acfe1244c2c140c4e5b08` (24 upstream commits, the "corpus coverage
wave 1" work) closed gaps 62, 68, 72, 76, 77, 78 and 79 outright and narrowed 61, 66, 69, 73 and
74; those entries were rewritten or removed here. It also introduced one regression, recorded below
as gap 81. Two consequences are deliberately left as follow-up rather than folded into the bump:
`tests/snapshots/issues.toml` still classifies fixtures against the closed `parser-gap-*` ids, and
each of those fixtures needs re-triage to say whether it is now blocked on a *lowering* gap
instead; and the members the bump newly types are listed under "Typed upstream, not yet lowered
here" where lowering has not caught up.

## Ownership and evidence rules

- An **upstream gap** means the parser rejects legal syntax, accepts it but drops authored
  information, or represents two semantically distinct authored forms identically. Spec42 must not
  recover such information by scanning source text or matching display strings.
- A **spec42 migration** means the pinned parser already exposes the required typed node, source
  span, recovery state, traversal, or arena lookup, but a consumer still uses the legacy AST API.
  That work belongs in spec42 and is not grounds for extending the parser with editor or semantic
  policy.
- Each upstream fix needs a parser regression test for accepted and malformed/recovery input as
  appropriate, provenance validation when new spans or arena identities are introduced, and a
  spec42 owning-layer test proving that no source-text reconstruction remains.
- Closing a gap requires re-verifying it against the newly pinned full commit and removing the entry
  from this active plan. Git history, not a completed section here, records the old gap.

## Parser authority

`crates/sysml_resolution` is the only crate that may name the parser, because it is the one that
lowers the AST to the semantic graph. Every other crate reaches syntax through
`sysml_resolution::syntax`, which returns plain data -- ranges, roles, outline nodes, closure
targets, an opaque `SyntaxDocument` handle. A crate with no parser dependency therefore cannot
hold, cache, serialize, or walk a `ParsedDocument`, and breaking that is a compile error rather
than a review comment.

`crates/source_identity/tests/parser_authority.rs` enforces it in four rules: the root workspace
owns the pin as a bare 40-character git revision; only `crates/sysml_resolution/Cargo.toml` may
mention the parser, and only as `workspace = true`; no manifest may reintroduce a repository-local
facade via a `path =` alias; and `Cargo.lock` must resolve exactly one `sysml-v2-parser`, from git,
with no registry checksum. It lives in `source_identity` because that crate has no parser
dependency and never will -- a guard the guarded thing could disable is not a guard.

The former `crates/sysml_parser` facade, which held a crates.io 0.54 copy and the pinned revision
apart, is deleted. Git history records that migration; it is not pending work.

## Open semantic grammar and provenance gaps

Each entry below was verified against the pinned revision by parsing a probe fixture with it
directly, by re-reading the owning `sysml_resolution` lowering, or both; where a claim is about a
regression, the same probe was run against the earlier revision for comparison. Every entry must be
rerun against the exact replacement revision when fixed.

| Gap | Information unavailable to consumers | Minimum upstream acceptance evidence |
| --- | --- | --- |
| 61 | `message` has no member variant in a calc-shaped body | Give `message` a typed member variant in the calc-shaped body grammar; prove `message m of T;` produces one node whose keyword never reaches the AST as a feature reference |
| 59 | Directed end Features cannot be represented, so `in end feature` is recovered before the end-direction validation can observe it | Accept direction modifiers in the end-feature production and preserve their spans; prove an `in end feature` reaches the AST as one directed end Feature |
| 64 | Conjugation *declarations* (`conjugates`/`ConjugationPart`), as opposed to the `~T` typing flag | Add a `Conjugation` relationship node and a type-declaration `ConjugationPart`; prove `classifier C conjugates A;` produces one conjugation, and two produce two |
| 65 | The `parallel` body modifier on a `state def` | Accept the modifier on the state-def body as `StateDefBody` specifies; prove `state def S parallel { ... }` parses and records isParallel |
| 66 | How many `crosses`/`references` clauses one feature authored: two clauses collapse into one relationship | Model each *clause* separately, or diagnose the second occurrence; prove `feature f crosses a crosses b;` is distinguishable from a single clause naming two targets |
| 67 | Restriction modifiers alongside `end` (`derived`/`abstract`/`composite`/`portion`/`var`) | Accept each prefix with `end` in the spelled order; prove all parse and lower, restoring the `end_feature_invalid_restrictions` coverage this revision made unreachable |
| 69 | A *binding* connector declaration cannot carry its end-feature body | Add the typed end-feature-body production for binding-connector declarations; prove a three-ended binding reaches one connector plus all three ends |
| 70 | Named members in a metadata-feature body are rejected before lowering | Add a typed metadata-feature body-member variant; prove a named feature member reaches the AST with its declaration and span. Still open at `f52100f`: the new `MetadataBody` grammar admits only reference redefinitions (`MetadataBodyUsage`), so `@M { attribute named : Boolean = true; }` is `unexpected_keyword_in_scope` |
| 73 | The `include use case <name>;` spelling has no production, though bare `include <name>;` does | Accept the `use case` keyword pair in the include membership production; prove the spelling reaches one typed node rather than a keyword `FeatureRef` plus a sibling use-case usage |
| 74 | The `require constraint <name> : <Type>;` spelling has no production, though bare `require <name>;` does | Accept the declared-constraint spelling in the require membership production; prove it reaches the AST with its declaration and target instead of `recovered_requirement_body_element` |
| 75 | A part usage in a port definition or port usage body has no production | Add a typed part-usage body-member variant for port bodies; prove the member reaches the AST with its composite modifier |
| 80 | State usage `parallel`/`initial` body modifier is accepted then discarded | Preserve the modifier as a typed `StateUsage` fact with its span; prove `state S parallel { ... }`, `state S initial { ... }`, and unmodified `state S { ... }` remain distinguishable to lowering |
| 81 | **Regression at the pinned revision.** A directed KerML-kinded parameter (`in expr p : T;`, `in bool redefines a { ... }`) in a `calc`/`constraint`-shaped body is dropped to parse recovery | Restore the KerML feature fallback behind the directed-parameter branch; prove `calc def C { in expr p : Boolean; }` reaches one typed member again, as it did at `49bdf3f` and as `behavior B { in expr p : Boolean; }` still does |
| 41 | Lexically distinguished implicit `that` self-reference | Produce a dedicated typed form that cannot collide with a user declaration; cover bare, cast, and member-access expressions |
| 52 | `readonly` and `variable` modifiers | Preserve presence and token spans independently from effective/default values |
| 55 | `//` and `/** ... */` comment fidelity, and `DocComment` text normalization | Decide and test whether doc-style trivia is syntax; if syntax, preserve kind, raw span, and one normalized-text policy centrally |

The contribution target is the pinned `lukewilliamboswell/sysml-v2-parser` repository. References
below to `elan8/sysml-v2-parser#121` record where the arena-backed work originated; they do not
authorize changing spec42 to follow a moving upstream branch. A fix is consumed only by updating
the single full revision in spec42 and regenerating the lockfile through the normal dependency
workflow.

- Gap 61. One member spelling of three remains unrepresentable in a calc-shaped body, and it is
  now rejected honestly rather than shredded. Verified by direct parse at `f52100f`:

  ```kerml
  classifier C { flow a.y to b.x1; }            // one FlowUsage, no diagnostic
  classifier C { redefines predecessors [0]; }  // one AttributeUsage + SubsettingRelationship
  classifier C { message m of T; }              // unexpected_keyword_in_scope
  ```

  At `49bdf3f` all three arrived as bare sibling `Expression::FeatureRef` members -- the keywords
  `flow`, `to`, `redefines` and `message` each structurally indistinguishable from a user feature
  of that name, each raising an `unresolved_reference` in the corpus. That "accepts it but drops
  authored information" case is closed for `flow` and `redefines`, which now reach typed nodes, and
  `message` fails loudly instead of silently, which is the honest state.

  What remains is a genuine grammar gap: `message m of T;` has no member variant in the calc-shaped
  body grammar at all, so the KerML message declaration cannot be authored in a `classifier`,
  `struct`, `class` or `behavior` body.

- Gap 64. There is no conjugation-*declaration* production. The parser models conjugation only as
  the `~T` conjugated-typing flag -- `ast::TypingRelationship.is_conjugated` (`src/ast/core.rs`)
  and the `type_is_conjugated` field on several usage nodes -- so KerML 8.2.4.1.3's `Conjugation`
  and the `ConjugationPart` of a type declaration have no node. `classifier One conjugates A;` is
  reported as `unsupported_grammar_form`. Verified at `f52100f` by direct inspection plus a scratch
  fixture.

  Both KerML conjugation constraints are unreachable for this reason
  (`tests/snapshots/validation/kerml_type_at_most_one_conjugator.md`,
  `kerml_specialization_specific_not_conjugated.md`). Needs a `Conjugation` relationship node and a
  `ConjugationPart` production on type declarations, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 65. The `parallel` state-body modifier is accepted on a state *usage* but not on a `state
  def`. `src/parser/state.rs` consumes an optional `parallel`/`initial` modifier before a state
  usage body, and `src/parser/part/body.rs` does the same for an exhibited state, but no
  `state def` body parser does: `state def Machine parallel { state a; state b; }` -- exactly the
  SysML `StateDefBody` production, `';' | ( isParallel ?= 'parallel' )? '{' StateBodyItem* '}'` --
  fails with `missing_body_or_semicolon` across the whole declaration, while the usage form
  `state machine parallel { ... }` parses and lowers. Verified at `f52100f` with a scratch fixture.

  `tests/snapshots/validation/sysml_state_definition_parallel_subactions.md` is `SKIPPED` for this
  reason while its usage sibling is skipped only for the missing semantic rule. Needs the same
  optional modifier on the state-def body production, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 80. The state-usage parser recognizes `parallel` and `initial` immediately before the body
  (`src/parser/state.rs` at pinned `f52100f`), but binds the result to `_` and constructs
  `ast::StateUsage` without either value or span. `StateUsage::isSubstateUsage(true/false)` is the
  prerequisite of both library-specialization checks, so semantic lowering cannot distinguish
  `state S parallel { state child; }` from the non-parallel form without forbidden source-text
  recovery. Preserve a typed modifier/parallel fact and prove it reaches the owning lowering.

- Gap 66. Clause *count* is unobservable. At `f52100f` `SubsettingRelationship.target` is a `Vec`,
  so `feature two crosses a crosses b;` and `feature tworef references a references b;` no longer
  drop the second target -- the earlier "silently discarded" reading of this gap is closed. What
  the AST cannot express is that the author wrote two separate clauses: both collapse into one
  relationship carrying two targets, indistinguishable from one clause naming two.

  KerML 8.3.3.3.4 states both `validateFeatureOwnedCrossSubsetting` and
  `validateFeatureOwnedReferenceSubsetting` as "at most one **clause**" rules, so the violation
  still cannot be observed, and the model published still disagrees with the model authored --
  now about clause structure rather than about a lost target. Needs either one relationship node
  per authored clause, matching how `relationship_parts` already models repeatable clauses, or an
  explicit diagnostic on the second occurrence. `tests/snapshots/validation/`'s
  `kerml_feature_owned_cross_subsetting.md` and `kerml_feature_owned_reference_subsetting.md` are
  `SKIPPED` for this reason.

- Gap 67. No restriction modifier is accepted alongside `end` on a KerML feature, in either order.
  `derived`, `abstract`, `composite`, `portion` and `var` all fail before or after `end`
  (`unexpected_keyword_in_scope` or `unrecognized_declaration_in_scope`); only `const end feature`
  parses. Re-verified at `f52100f`: `derived end feature` is `unexpected_keyword_in_scope` and
  `var end feature` is `unrecognized_declaration_in_scope`, while `const end feature` parses. This is a
  regression against `204ca48`, where `derived end feature`, `abstract end feature` and
  `composite end feature` all parsed and lowered, and where
  `sysml_resolution`'s `end_feature_invalid_restrictions` fired for them.

  That diagnostic is now unreachable: it appears nowhere in the regenerated corpus's generated
  `DIAGNOSTICS`, only in the authored expectations of
  `tests/snapshots/validation/kerml_feature_end_restrictions.md` and
  `kerml_end_feature_direction.md`. KerML 8.3.3.3.4
  `validateFeatureEndNotDerivedAbstractCompositeOrPortion` therefore has no authorable violation.
  Needs the restriction prefixes accepted alongside `end`, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 81. **Regression introduced by this bump.** A directed KerML-kinded parameter in a
  `calc`-shaped or `constraint`-shaped body is dropped to parse recovery. Verified by direct parse
  at both revisions:

  ```sysml
  calc def C { in expr p : Boolean; }        // f52100f: recovered_calc_body_element
  calc def C { in bool redefines a; }        // f52100f: recovered_calc_body_element
  calc def C { in feature p : Boolean; }     // f52100f: recovered_calc_body_element
  constraint def C { in expr p : Boolean; }  // f52100f: recovered_constraint_body_element
  behavior B { in expr p : Boolean; }        // f52100f: parses (unaffected)
  calc def C { in p : Boolean; }             // f52100f: parses (unaffected)
  ```

  All of these parsed at `49bdf3f`, where the member reached the AST as a `KermlFeature` carrying
  its direction in `BasicFeaturePrefix` and lowered under the kind its keyword names (`expr` ->
  `kerml-expression`, `bool` -> `kerml-boolean-expression`).

  Cause: `calculation_body_element` (`src/parser/constraint.rs`) gained an `in`/`out`/`inout`
  branch that commits to the grammar-owned `in_out_decl` parameter parser after probing `calc` and
  `part` usages, with no fallback to the KerML feature route the member used to take. The branch's
  own comment states the intent -- keep `calc def F { in p : T; }` an `InOutDecl` while
  `behavior B { in p : T; }` stays a keyword-less `Feature` -- but a *kinded* parameter is neither,
  and there is no third arm for it.

  Corpus effect is contained: the Kernel Function and Semantic Libraries author this spelling in
  KerML `function`/`behavior` bodies, which are unaffected, so
  `tests/snapshots/sysml.library/control_functions.md` still parses (it improved to `complete` in
  this bump). Pinned downstream by `calc_def_body_kinded_parameter_is_recovered_by_the_pinned_parser`
  and its redefinition sibling in `crates/sysml_resolution/src/model.rs`, which assert the
  `parse-recovery` completeness so the loss stays visible rather than looking like a complete
  publication.

- Gap 41. KerML's implicit self-reference identifier `that` (e.g.
  `tests/snapshots/sysml/examples`'s `trig_functions.md`: `inv unitBound { -1.0 <= that & that <=
  1.0 }` inside `datatype UnitBoundedReal :> Real { ... }`; 116 snapshot fixtures author a bare
  `that`) has no lexically-distinguished status in the parser. `src/parser/lex.rs`'s
  `SYSML_RESERVED_KEYWORDS` table (line 733) -- the parser's own reserved-word list, which tells a
  genuine language keyword apart from an arbitrary identifier -- does **not** contain `"that"`, and
  a repo-wide grep of `f52100f`'s `src/` for the word finds zero hits: no keyword, marker, or
  expression variant anywhere. It lexes
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

- Gap 52. Two SysML declaration-modifier prefixes have no representation in the pinned parser at
  all, so the canonical declaration-fact family (`DeclarationModifiers` in
  `crates/sysml_resolution/src/model.rs`) cannot include them and an element inspector cannot
  report them: `readonly` and SysML `variable`. Re-verified against `f52100f` -- both spellings answer
  `unrecognized_declaration_in_scope`, the AST design note at `src/ast/membership.rs` states they
  are deliberately out of scope, and neither word appears as a token, field, or starter anywhere in
  `src/parser` (the only hits are prose comments
  and a fixture filename). The third member of this gap, authored `unique`, is **closed**:
  `ast::MultiplicityModifiers` (`src/ast/multiplicity_part.rs`) models the `isOrdered` and
  `isUnique` slots as `Option<Node<_>>` over their authored spellings, so an authored `unique` is
  distinguishable from omission and from `nonunique`, each with its own span, and
  `sysml_resolution` reads both through `MultiplicityModifiers::is_ordered`/`is_unique`. The bare
  SysML `portion` prefix is likewise unrepresentable; the KerML `portion` prefix *is* reachable via
  `FeaturePrefix::is_portion` and is lowered. Needs `readonly`/`variable` added as
  `RefPrefix`/`BasicFeaturePrefix` slots alongside the existing ones, filed upstream against
  `feat/gh-119-arena-backed-references` (elan8/sysml-v2-parser#121).

- Gap 55. Two comment forms remain unreachable from the AST, and the text of the ones that are
  reachable has no normalization policy. Re-verified against `f52100f`.

  What `b6291cc` changed: a keyword-less block comment in *member* position is now a real
  annotating element. `CommentAnnotation` (`src/ast/common.rs`) gained `keyword_span:
  Option<Span>`, so `/* ... */` written where a member may appear parses as
  `AnnotatingMember::Comment` with `keyword_span: None`, distinguishable from the authored
  `comment /* ... */` spelling. `sysml_resolution` lowers it through `lower_annotating_member` like
  any other. (Landing it briefly broke the member that followed a keyword-less comment; `ec47463`
  fixed that.)

  What remains: `trivia_len` (`src/parser/lex.rs`, called from `ws_and_comments`) still
  consumes `//` line comments and `//* ... */` outright, and it treats `/** ... */` through the
  same `/*` arm as a plain block comment, so a doc-style block is indistinguishable from an
  ordinary one and an element inspector can never surface it as documentation. Comments in
  non-member positions -- mid-declaration, after a clause -- remain trivia in every form.
  Separately, `DocComment.text` is the raw byte slice between `/*` and `*/`
  (`src/parser/requirement.rs`'s `doc_comment`) with no
  leading-`*` stripping and no dedent, so every consumer must normalize it identically or they will
  disagree. Keeping trivia out of the AST is a defensible design; the entry stays so the ceiling on
  documentation fidelity is not rediscovered. If `/** ... */` is intended to be an annotation, it
  needs its own production, filed upstream against `feat/gh-119-arena-backed-references`
  (elan8/sysml-v2-parser#121).

## Typed upstream, not yet lowered here

Not upstream gaps: the parser carries these typed facts, and the remaining work is
`sysml_resolution` lowering. Recorded so the coverage they represent stays visible rather than
disappearing with the gap entries they closed. Each was re-checked against the owning lowering on
2026-08-22, at pinned `f52100f`; entries whose lowering has since landed were removed.

- **KerML explicit relationship declarations.** `ast::KermlRelationshipDecl`
  (`PackageBodyElement::KermlRelationship`) models `subtype`/`subclassifier`/`typing`/`subset`/
  `redefinition` with their optional `specialization <ident>` prefix, plus `disjoint ... from`,
  `inverse ... of`, and `featuring ... by`. `lower_package_element` reports every one as
  `unsupported_package_member`. This is the one construct whose semantic coverage *narrowed* when
  the node landed: `subclassifier X specializes Y;` previously reached `KermlClassifierDecl` and
  lowered `X` as a `kerml-classifier` declaration, which named the relationship's source as though
  it were a new classifier. Still open at `f52100f` --
  `tests/snapshots/sysml.library/occurrences.md:736`'s `subclassifier SelfLink specializes
  SelfSameLifeLink;` is the fixture's one `unsupported_package_member`.

- **`EntryAction`/`DoAction`/`ExitAction` declaration facts.** `declared_name`, `type_name`,
  `redefines`, and `effect` are typed; `lower_state_entry_action` and its siblings still read only
  `action_reference` and `body`, so `entry action entryAction :>> 'entry';` lowers as the reference
  form.

- **The `VariantMembership` role on a delegated variant.** `lower_variant_usage` now dispatches
  every `VariantTypedUsage` kind to the lowering its ordinary spelling uses, so a `variant part p :
  T;` publishes a real `PartUsage`. What it does not publish is the *role*: an enumeration literal
  carries `MembershipRole::Variant` (`model/element_kind.rs`), a delegated variant does not,
  because the five `lower_*_usage` functions return `()` and the caller has no `DeclarationId` to
  set it on. Pre-existing -- the `Perform` arm always had it -- and recovering it means changing
  five hot signatures.

- **`variant attribute` in a `variation attribute def` body.** No longer an upstream gap (the
  earlier entry, which claimed the member was dropped upstream with no diagnostic, was stale): at
  `f52100f` `ast::AttributeBodyElement` carries a `VariantUsage` variant, so
  `variation attribute def DiameterChoices :> Diameter { variant attribute diameterSmall; }`
  (`tests/snapshots/sysml/training/36_variation_definitions.md:26`) reaches the AST as a typed
  member. `lower_attribute_body` reports it as `unsupported_attribute_member` rather than
  delegating to `lower_variant_usage` the way `PartDefBodyElement` does, so the member is now
  visible but still unlowered. Pinned by
  `every_variant_typed_usage_delegates_to_its_ordinary_lowering`.

- **`MetadataKeywordUsage.type_reference`.** The `#<Name>` shorthand now carries a resolvable
  `QualifiedReferenceId` alongside its keyword spelling, but every scope still reports the member
  as unsupported rather than resolving it the way `@Name` (`lower_metadata_annotation`) does.

- **`RequireConstraint.target`.** The keyword-less `require <qualified.name>;` shorthand now
  carries an arena-backed target; `lower_require_constraint_member` still defers the whole
  `has_constraint_keyword == false` form.

- **`CommentAnnotation.keyword_span`.** Whether a comment member was written with the `comment`
  keyword is now a grammatical fact with its own span (see Gap 55). `sysml_tokens` reads it for
  semantic ranges; `sysml_resolution` does not record it, so the two spellings publish identically.

- **`KermlFeatureMember.crosses`, `references`, `chains`, `inverse_of` and `is_const`.** All five
  are typed on the node; `lower_kerml_feature_member` (`crates/sysml_resolution/src/model.rs`)
  reads only `typing`, `subsets`, `redefines`, `type_relationships`, `value` and `body`, and
  copies every sibling modifier flag except `is_const`. So an authored `crosses`/`references`/
  `chains`/`inverse of` clause and an authored `const` are all dropped between parse and
  publication. Eleven validation fixtures are `SKIPPED` on exactly this
  (`tests/snapshots/validation/kerml_feature_owned_cross_subsetting.md`,
  `kerml_feature_chaining_*.md`, `kerml_feature_end_is_constant.md`, and others); each names the
  field it is waiting on.

- **`CollectionOperatorBody.doc`.** A collection operator body's `doc /* ... */` annotation is
  typed upstream, but the whole collection-operator expression family is still unlowered here
  (`lower_expression` has no arm for it), so there is no declaration to attach the fact to yet.

- **`FlowUsage`'s declaration-led form (`FlowDeclaration::Declared`).** The bump replaced
  `FlowUsage`'s flat `name`/`type_name`/`from`/`to` fields with `FlowDeclaration`'s two grammar
  alternatives, so a named or typed flow (`flow generateToAmplify from a to b;`,
  `flow f : T from a to b;`) now carries a full `UsageDeclaration` with its own identification,
  typing, multiplicity and specialization clauses. `lower_flow_usage` lowers only the
  `EndpointOnly` shorthand and reports the declared form as unsupported, exactly as it did when
  the same information was untrustworthy; the information is now trustworthy and the lowering is
  the remaining work.

- **`InterfaceEnd`'s declaration label and the n-ary `InterfacePart`.** Interface connect
  endpoints are no longer bare expressions: `InterfaceEndTarget::Named` retains an endpoint's
  declaration label and the authored `::>`/`references` spelling (`connect left ::> port to ...`),
  and `InterfacePart::Nary` retains the parenthesized ordered end list with its commas.
  `lower_interface_end` resolves every endpoint's target as a `ConnectorEnd` reference in both
  forms; the label and the operator spelling are typed but not yet recorded as facts.

- **`GuardedSuccession`'s owned body.** The action-only `first <chain> if <guard> then <end>`
  production is newly typed, and `lower_guarded_succession` lowers its source, guard and target.
  Its `DefinitionBody` and the optional `succession` declaration's identification are typed
  upstream and not yet lowered, mirroring `lower_first_stmt`'s own body deferral.

- **`MetadataBodyUsage.ref_span` and `.operator`.** A metadata body member's optional `ref` keyword
  and its authored redefinition spelling (`:>>` vs `redefines`) each carry their own span, so the
  two spellings and the implicit form are distinguishable upstream. `lower_metadata_body_usage`
  publishes the redefinition relationship and the bound value but not which spelling was authored,
  so the three forms publish identically.

- **`PerformActionTarget::Reference.action`.** The shorthand `perform <path>;` form's action
  reference is now an arena-backed target on a typed perform member (closing the old gap 72).
  `lower_perform` lowers the form's `:>>` redefinition but still does not resolve the action
  reference itself, so the shorthand mints an anonymous performance with no link to what it
  performs.
