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

- **`variant attribute` in a `variation attribute def` body is dropped upstream, silently.**
  `ast::AttributeBodyElement` has no `VariantUsage` variant at all, so
  `variation attribute def DiameterChoices :> Diameter { variant attribute diameterSmall; }`
  (`tests/snapshots/sysml/training/36_variation_definitions.md:26`) publishes **no member and no
  diagnostic** -- the fixture's `DiameterChoices` has zero owned declarations. This is an upstream
  gap rather than a dispatch gap: there is no typed node to dispatch. The same spelling inside a
  `variation part def` body works, because `PartDefBodyElement` does carry the variant. Pinned by
  `every_variant_typed_usage_delegates_to_its_ordinary_lowering`, which fails if it ever starts
  publishing.

- **`MetadataKeywordUsage.type_reference`.** The `#<Name>` shorthand now carries a resolvable
  `QualifiedReferenceId` alongside its keyword spelling, but every scope still reports the member
  as unsupported rather than resolving it the way `@Name` (`lower_metadata_annotation`) does.

- **`RequireConstraint.target`.** The keyword-less `require <qualified.name>;` shorthand now
  carries an arena-backed target; `lower_require_constraint_member` still defers the whole
  `has_constraint_keyword == false` form.

- **`CommentAnnotation.keyword_span`.** Whether a comment member was written with the `comment`
  keyword is now a grammatical fact with its own span (see Gap 55). `sysml_tokens` reads it for
  semantic ranges; `sysml_resolution` does not record it, so the two spellings publish identically.

- **`CollectionOperatorBody.doc`.** A collection operator body's `doc /* ... */` annotation is
  typed upstream, but the whole collection-operator expression family is still unlowered here
  (`lower_expression` has no arm for it), so there is no declaration to attach the fact to yet.
