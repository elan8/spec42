# Upstream sysml-v2-parser gaps

This is the active record of information the parser must preserve or distinguish before spec42 can
implement the corresponding semantic or syntax-fidelity behavior without guessing. It also records
the separate, downstream migration required to delete `spec42-sysml-parser`; the two categories must
not be conflated.

The canonical parser currently pinned behind `crates/sysml_parser` is
`lukewilliamboswell/sysml-v2-parser@ec47463f86829bc7caebd44b8ad7db6eea677691`. Every gap below was
re-exercised against that exact revision on 2026-08-20, by parsing a probe fixture with it directly
and by re-reading the owning `sysml_resolution` lowering; the entries the bump closed were removed
rather than annotated. New upstream work must be based on the full pinned identity, not an
abbreviated revision or the old `sysml-v2-parser-next` dependency alias.

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

## Parser-facade removal audit

On 2026-08-19 a direct workspace pin to `204ca48` (the then-current revision), removal of
`crates/sysml_parser`, and removal of
the `sysml_v2_parser::next` namespace were compiled with:

```text
cargo check --workspace --all-targets --offline
```

The experiment produced 130 errors in `sysml_tokens`; this is an error count, **not 130 adapters or
130 upstream gaps**. The edits were reverted after inventorying the failures. **That count predates
`b6291cc`/`ec47463`**, which reshaped the usage/feature prefixes, the multiplicity keyword slots and four
body-element enums, so it is a lower bound on today's figure rather than a current measurement;
re-run the experiment before planning against it. The buildable facade
therefore remains only as a bounded migration boundary: its root re-exports parser 0.54 while
`next` re-exports the pinned revision for semantic construction.

### Capabilities already present upstream

These must be consumed rather than reimplemented or requested again upstream:

- `ParsedDocument` atomically owns `SourceStorage`, `QualifiedReferenceArena`, and `RootNamespace`.
- `ParsedDocument::range` is the canonical byte-span to source-range conversion.
- `ParsedDocument::qualified_reference` and `qualified_declaration_name` resolve document-local
  arena identities to source-backed views with segment spans and separator provenance.
- `ast::visit::Visitor` is an exhaustive, source-ordered, pre-order structural traversal generated
  from the shared traversal inventory. It is appropriate for context-free range/reference
  collection. Policy-complete semantic lowering retains exhaustive scope matches, as the parser's
  visitor contract requires.
- `parse`, `parse_owned`, `parse_for_editor`, and `parse_for_editor_owned` return an atomic document
  (directly or through `ParseResult.document`) and preserve explicit recovery diagnostics.

Consequently, detached-AST compatibility helpers are not an upstream requirement. In particular,
the deprecated `Span::to_lsp_range`, textual access through old `TypingRelationship.target` nodes,
and `ParseResult.root` are APIs spec42 must stop expecting.

### Downstream work required before deletion

1. Change syntax-fidelity APIs to accept `&ParsedDocument`, not a detached `&RootNamespace` plus
   separately supplied source text. Start with `sysml_tokens::ast_semantic_ranges` and its helpers.
   Resolve every `QualifiedReferenceId` through the same document and convert every span with
   `ParsedDocument::range`.
2. Replace `sysml_tokens/src/ast_ranges.rs`'s legacy recursive descent with the upstream visitor
   where classification is genuinely per node kind. Keep LSP token categories, precedence, and
   delta encoding in spec42. Do not move editor presentation policy into the parser.
3. Migrate the remaining legacy syntax consumers: language-service outline, formatting and code
   actions; LSP syntax/symbol adapters and workspace parse state; workspace library closure and
   parse-cache artifacts; KPAR package discovery; server parsing; tests and fuzz targets.
4. Preserve the canonical semantic path already using the pinned revision in `sysml_resolution`;
   after the
   workspace dependency points directly at that revision, remove only the `next` namespace from
   imports.
5. Replace the facade-owned manifest guard with a repository-wide guard that permits a parser
   source/revision only in the root workspace dependency and requires all production manifests to
   use `workspace = true`.
6. Delete `crates/sysml_parser`, remove parser 0.54 from `Cargo.lock`, and prove that only one
   `sysml-v2-parser` package identity remains with `cargo tree -d` and `cargo tree -i
   sysml-v2-parser`.

Facade deletion is complete only when clean and recovery corpus behavior remains equivalent across
parser consumers, semantic-token golden tests retain classifications and exact ranges, outline and
code-action tests retain exact symbols/edits, parse-cache round trips retain source and arena
provenance, snapshot cold/warm parity passes, and the workspace/all-target check succeeds offline.
No temporary source scanner, compatibility DTO, second parse, or silent fallback is an acceptable
migration step.

## Open semantic grammar and provenance gaps

Each entry below was verified against the pinned revision by parsing a probe fixture with it
directly, by re-reading the owning `sysml_resolution` lowering, or both; where a claim is about a
regression, the same probe was run against the earlier revision for comparison. Every entry must be
rerun against the exact replacement revision when fixed.

| Gap | Information unavailable to consumers | Minimum upstream acceptance evidence |
| --- | --- | --- |
| 61 | Calc-shaped bodies silently shred `flow`, `message` and anonymous `redefines` members into bare expressions | Give each a typed member variant in the calc-shaped body grammar; prove `flow a.y to b.x1;`, `message m of T;` and `redefines predecessors [0];` produce one node each, and that no keyword reaches the AST as a feature reference |
| 41 | Lexically distinguished implicit `that` self-reference | Produce a dedicated typed form that cannot collide with a user declaration; cover bare, cast, and member-access expressions |
| 52 | `readonly` and `variable` modifiers | Preserve presence and token spans independently from effective/default values |
| 55 | `//` and `/** ... */` comment fidelity, and `DocComment` text normalization | Decide and test whether doc-style trivia is syntax; if syntax, preserve kind, raw span, and one normalized-text policy centrally |

The contribution target is the pinned `lukewilliamboswell/sysml-v2-parser` repository. References
below to `elan8/sysml-v2-parser#121` record where the arena-backed work originated; they do not
authorize changing spec42 to follow a moving upstream branch. A fix is consumed only by updating
the single full revision in spec42 and regenerating the lockfile through the normal dependency
workflow.

- Gap 61. Three member spellings reach a calc-shaped body's AST as **bare expressions rather than
  the members they are**, with no diagnostic at all. Verified by direct parse against `ec47463`:

  ```kerml
  classifier C { flow a.y to b.x1; }        // 4 Expression members: `flow`, `a.y`, `to`, `b.x1`
  classifier C { message m of T; }          // 4 Expression members
  classifier C { redefines predecessors [0]; }  // 2 Expression members: `redefines`, `predecessors`
  ```

  Each keyword arrives as an ordinary `Expression::FeatureRef`, structurally indistinguishable from
  a user feature of that name, and the statement is split into unrelated sibling members. This is
  the "accepts it but drops authored information" case, and it is worse than a rejection: at
  `204ca48` a `class` body answered `unexpected_keyword_in_scope`, which was at least honest.
  `sysml_resolution` lowers what it is given, so the corpus publishes `expressionOperand`
  references whose authored targets are the keywords `flow`, `to` and `redefines`, each raising an
  `unresolved_reference`.

  **Mostly pre-existing, with one scope newly affected.** `classifier`, `struct` and `behavior`
  bodies shredded all three spellings at `204ca48` already. `class` bodies joined them at
  `b6291cc`, which routed `class` through the shared KerML classifier declaration (upstream
  `17da637`) and so moved it off the attribute-shaped body that parsed these members correctly:
  `attribute def A { redefines predecessors [0]; }` still produces one typed node.

  Corpus effect of that one scope change: keyword-shaped expression operands rise 23 → 38 across
  eight fixtures, concentrated in `tests/snapshots/kerml/moments.md` (3 → 8, whose five `class`
  bodies author eleven `redefines <target> [mult];` members). The snapshots pin it rather than
  suppress it.

  A fix needs typed member variants for all three spellings in the calc-shaped body grammar. It
  would also restore `class`-body parity with `204ca48` as a side effect.

- Gap 41. KerML's implicit self-reference identifier `that` (e.g.
  `tests/snapshots/sysml/examples`'s `trig_functions.md`: `inv unitBound { -1.0 <= that & that <=
  1.0 }` inside `datatype UnitBoundedReal :> Real { ... }`; 116 snapshot fixtures author a bare
  `that`) has no lexically-distinguished status in the parser. `src/parser/lex.rs`'s
  `SYSML_RESERVED_KEYWORDS` table (line 733) -- the parser's own reserved-word list, which tells a
  genuine language keyword apart from an arbitrary identifier -- does **not** contain `"that"`, and
  a repo-wide grep of `ec47463`'s `src/` for the word finds zero hits: no keyword, marker, or
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
  report them: `readonly` and SysML `variable`. Re-verified against `ec47463` -- the AST design
  note at `src/ast/membership.rs:32` states they are deliberately out of scope, and neither word
  appears as a token, field, or starter anywhere in `src/parser` (the only hits are prose comments
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
  reachable has no normalization policy. Re-verified against `ec47463`.

  What `b6291cc` changed: a keyword-less block comment in *member* position is now a real
  annotating element. `CommentAnnotation` (`src/ast/common.rs`) gained `keyword_span:
  Option<Span>`, so `/* ... */` written where a member may appear parses as
  `AnnotatingMember::Comment` with `keyword_span: None`, distinguishable from the authored
  `comment /* ... */` spelling. `sysml_resolution` lowers it through `lower_annotating_member` like
  any other. (Landing it briefly broke the member that followed a keyword-less comment; `ec47463`
  fixed that.)

  What remains: `trivia_len` (`src/parser/lex.rs:535`, called from `ws_and_comments`) still
  consumes `//` line comments and `//* ... */` outright, and it treats `/** ... */` through the
  same `/*` arm as a plain block comment, so a doc-style block is indistinguishable from an
  ordinary one and an element inspector can never surface it as documentation. Comments in
  non-member positions -- mid-declaration, after a clause -- remain trivia in every form.
  Separately, `DocComment.text` is the raw byte slice between `/*` and `*/`
  (`src/parser/requirement.rs:754`'s `doc_comment`, building the node around line 794) with no
  leading-`*` stripping and no dedent, so every consumer must normalize it identically or they will
  disagree. Keeping trivia out of the AST is a defensible design; the entry stays so the ceiling on
  documentation fidelity is not rediscovered. If `/** ... */` is intended to be an annotation, it
  needs its own production, filed upstream against `feat/gh-119-arena-backed-references`
  (elan8/sysml-v2-parser#121).

## Typed upstream, not yet lowered here

Not upstream gaps: the parser carries these typed facts, and the remaining work is
`sysml_resolution` lowering. Recorded so the coverage they represent stays visible rather than
disappearing with the gap entries they closed. Each was re-checked against the owning lowering on
2026-08-20; entries whose lowering has since landed were removed.

- **KerML explicit relationship declarations.** `ast::KermlRelationshipDecl`
  (`PackageBodyElement::KermlRelationship`) models `subtype`/`subclassifier`/`typing`/`subset`/
  `redefinition` with their optional `specialization <ident>` prefix, plus `disjoint ... from`,
  `inverse ... of`, and `featuring ... by`. `lower_package_element` reports every one as
  `unsupported_package_member`. This is the one construct whose semantic coverage *narrowed* when
  the node landed: `subclassifier X specializes Y;` previously reached `KermlClassifierDecl` and
  lowered `X` as a `kerml-classifier` declaration, which named the relationship's source as though
  it were a new classifier. Still open at `ec47463` --
  `tests/snapshots/sysml.library/occurrences.md:736`'s `subclassifier SelfLink specializes
  SelfSameLifeLink;` is the fixture's one `unsupported_package_member`.

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

- **`KermlFeature.crosses`.** The one usage family whose `crosses` clause is dropped:
  `lower_attribute_usage`, `lower_port_usage` and `lower_occurrence_usage` all push it through
  `lower_subsetting_relationship`, `lower_kerml_feature_member` does not. Its former companion
  `is_const` is now the `const` alternative of `FeaturePrefix`'s variability slot and *is* lowered,
  onto `DeclarationModifiers::constant`.

- **The multiplicity and short-name fields the old Gap 53 asked for.** `AttributeDef`,
  `ConstraintUsage`, `RequirementUsage`, `CalcUsage` and `RequirementActorDecl` all carry
  `multiplicity` now, and `ActionUsage`, `OccurrenceUsage`, `ConstraintUsage`, `RefDecl`,
  `EndDecl`, `ReturnDecl` and `ViewUsage` all carry `short_name`. None of the owning lowerings
  reads either: `lower_attribute_def`, `lower_constraint_usage`, `lower_requirement_usage`,
  `lower_calc_usage` and `lower_requirement_actor_decl` make no `multiplicity_facts` call, and
  `lower_action_usage`, `lower_occurrence_usage`, `lower_constraint_usage`, `lower_ref_decl`,
  `lower_return_decl` and `lower_view_usage` intern no short name. The uniqueness half *is* read:
  every declaration that spells `MultiplicityPart` carries `multiplicity_modifiers`, and
  `sysml_resolution` reads it everywhere it read `ordered`/`nonunique` before.

- **`definition_prefix` on the connection-like definitions.** `ConnectionDef`, `FlowDef`,
  `AllocationDef` and `InterfaceDef` gained the `Option<Node<DefinitionPrefix>>` slot the old Gap 58
  asked for, so an authored `abstract connection def C { ... }` is now representable with its exact
  token span. None of `lower_connection_def`, `lower_flow_def`, `lower_allocation_def` or
  `lower_interface_def` mentions the field, so no modifier fact is published, which is why
  `tests/snapshots/resolution/structural_feature_conformance.md` still reports an abstract
  declaration for an incomplete end set.

- **`EnumeratedValue.short_name`, `value` and `body`.** Closing the enumeration half of the old
  Gap 56 gave each literal its short name, its `= expr` initializer as a `FeatureValue`, and a full
  `PartUsageBody`. `lower_enumerated_value` reads none of them. The `enum def`'s own body
  annotations *are* lowered: `EnumerationBodyElement::Annotating` dispatches to
  `lower_annotating_member`, and `tests/snapshots/documentation_in_bodies.md`'s `enum def Color`
  now publishes its doc comment.

- **`CommentAnnotation.keyword_span`.** Whether a comment member was written with the `comment`
  keyword is now a grammatical fact with its own span (see Gap 55). `sysml_tokens` reads it for
  semantic ranges; `sysml_resolution` does not record it, so the two spellings publish identically.

- **`CollectionOperatorBody.doc`.** A collection operator body's `doc /* ... */` annotation is
  typed upstream, but the whole collection-operator expression family is still unlowered here
  (`lower_expression` has no arm for it), so there is no declaration to attach the fact to yet.

- **`ItemUsage.subsets`.** `lower_item_usage` lowers `redefines` but not the `:>` clause, so an
  authored `item items : Item[0..*] nonunique :> objects;` publishes its typing and drops its
  subsetting. Still true at `ec47463`; the one-line fix is the same `lower_subsetting_relationship`
  call every sibling usage already makes.
