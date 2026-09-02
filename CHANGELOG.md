# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- **Metadata-prefixed dependencies lower in definition bodies.** `#refinement dependency X to Y;`
  in a `requirement def`, `action def`, or `part def` body no longer reports the `#refinement`
  prefix as an `unsupported_*_definition_member`. The dependency lowers with resolved
  client/supplier endpoints and the `#tag` prefix (stacked prefixes included) becomes an authored
  metadata annotation on the dependency, the same fact `#Tag` on a usage and `@Tag` already
  publish. A prefix that does not precede a dependency stays an explicit unsupported member.

- **Project manifests now constrain rather than select the standard-library baseline.** Every
  project retains the host-admitted KerML/SysML baseline, including manifests with `usage: []`;
  matching authored stdlib usages act as compatibility pins. A version-incompatible baseline fails
  admission explicitly; a disabled (`--no-stdlib`) or unavailable baseline leaves the pin inert so
  loading still proceeds. Unresolved imports then distinguish an intentionally disabled baseline
  from one that is unavailable and explain how to recover.

- **Recursive imports (`import P::**`, `import P::*::**`) bring nested-namespace members into
  scope.** The suffix parsed and lowered, but name resolution left the reference `Unsupported`, so
  it was reported as `unsupported_filtered_import` and every name that depended on it cascaded to
  `unresolved_*`. A recursive import now re-exports the importable members of the target namespace
  and, transitively, of every publicly-visible namespace nested under it; a private nested package
  and its subtree stay excluded. Filtered recursive imports (`import P::** [ ... ]`) remain
  unsupported. Fixes #102.

- **Bumped the pinned `sysml-v2-parser` revision `f04d51e` -> `7634eaf`.** Picks up the five Apollo
  11 normative-form parser fixes (`elan8/sysml-v2-parser#133`, for `#132` / issue #100). Three now
  reach typed lowering in spec42: a keyword-less feature usage with an explicit typing/redefinition/
  value in a use-case-family body (`analysis`/`use case`/`verification` and their `def`s) lowers as
  a `DefaultReferenceUsage` instead of `recovered_use_case_body_element`; a nested `action` usage
  with a multiplicity written before the typing (`action subfunctions[*] : Function :>> subactions;`)
  and a `def`-less `abstract connection` usage of the same shape both parse without recovery or
  `unsupported_grammar_form`. A calc-body `return :>>` now lowers its `::`-qualified redefinition
  target as an authored `Redefinition` reference (matching the case-family `return`), so a leading
  `return :>> <target>` no longer silently drops the authored intent. `first`/`then` action-flow
  statements are accepted in the new `StateDefBodyElement::FirstStmt` / `ThenAction` members.
  Two forms remain parser-blocked and are tracked in `planning/UPSTREAM_PARSER_GAPS.md` (gaps 83,
  84): `return :>` (subsetting) with a qualified target still recovers, and `do action <name> { … }`
  with a body but no typing clause is still parsed as an action reference. Partially addresses #100.

- **Bumped the pinned `sysml-v2-parser` revision `4b65812` -> `f04d51e`.** Picks up keyword-less
  `first ... then ...` succession usages in occurrence and item definition bodies (authored
  `succession` spelling preserved, malformed attribute-body recovery no longer swallows a following
  valid succession), directed `ref` declarations in port bodies, feature-chain targets on `verify`,
  and hardened serde prefix-tampering checks.

- **Interconnection views render as nested parts with ports on the node boundary.** Schema-5
  products were being drawn as a generic graph, so `PortUsage` nodes became a vertical stack of
  boxes. The renderer now adapts the published `parts` / `ports` / `connectors` metadata into the
  SysML 8.2.3.11 interconnection notation (the same IBD layout the pre-schema-5 scene used).

- **Diagram node labels show authored types only.** Compact `typing` on a projected element is
  the FeatureTyping the author wrote (`part root : Assembly` → `: Assembly`), not the effective
  type closure that every `part` inherits from `Parts::parts`, `Items::items`, and the rest of
  the kernel chain. Untyped usages render with no `: Type` line. Hover and the Feature Inspector
  still list the full effective-type set.

- **The VS Code diagram viewer is a persistent, project-scoped view in the secondary side bar.**
  It lists every authored diagram view in the model (grouped by file) and keeps a live render of
  the selected one: a toolbar dropdown switches views and re-generates in place, `Home` /
  `JSON` / `SVG` / `PNG` sit next to it, and the view regenerates on its own whenever the model
  changes. The server emits a `spec42/publicationChanged` notification after each workspace
  publication rebuild; the view reconciles against it. A backoff reconcile loop runs whenever the
  view is visible and its render does not match the current publication, so a slow index, a
  missed notification, a language-server restart, or a stalled webview all recover with no user
  action and no manual refresh control. `Spec42: Open Diagram` focuses the view. PNG /
  standalone-SVG export re-renders with a literal colour scheme so the file stands alone outside
  the webview.

- **`SequenceView` projects message edges and their order.** A `message` on an `occurrence def`
  lifeline already lowered with resolved `flowSource` / `flowTarget` and `succession` order, but
  the projection dropped it: `ElementKind::FlowConnectionUsage` did not bridge to
  `Metaclass::FlowUsage` (so message nodes never reached `metadata.messages`), and nothing was
  required of a `SequenceView` (so an unresolved message end reported `complete`). The generator
  ABI now also carries `DiagramEdge.origin_occurrence` -- the element a composed edge was
  declared by -- and the VS Code sequence renderer builds its lifelines and ordered messages
  from the schema-5 projection instead of a legacy DTO it was never handed. Wire schema
  fingerprint and compatibility token move; generator guests must be rebuilt. The `examples`
  submodule is bumped so `examples/webshop` uses the normative `occurrence def` + `message`
  form.

- **Bumped the pinned `sysml-v2-parser` revision `c81e0b6` -> `695b2b4`.** The parser now
  preserves ordered and repeated flow-payload and feature-specialization clauses, binding body
  ends, directed action parameter kinds, action-body transitions, and invalid-owner
  requirement/view memberships. `sysml_resolution` consumes the new exhaustive AST shapes;
  obsolete parser blockers are retired or narrowed, while remaining semantic work is tracked by
  typed lowering/semantic blockers in the snapshot corpus.

- **Bumped the pinned `sysml-v2-parser` revision `c1677e7` -> `34fd6c4`.** The 12-commit upstream
  speculation-removal performance series preserves the existing parser-gap inventory; snapshot
  regeneration now reports the retained `#Safety feature z1 : T;` extended usage explicitly as an
  unsupported package member.

- **Bumped the pinned `sysml-v2-parser` revision `f52100f` -> `c1677e7`.** The 40-commit bump
  (upstream "gaps wave 2", "corpus snapshot wave 3", the parser performance pass and the
  span-backed authored-text migration) closes gaps
  59, 64, 65, 67, 70, 72, 73, 75, 80 and 81 in `planning/UPSTREAM_PARSER_GAPS.md` and narrows 62,
  66, 74 and 76, each re-probed at the new revision one spelling per document through `spec42
  check`. It opens one gap, 82: a directed `in action body { ... }` parameter reaches the AST as
  a plain `InOutDecl` with its `action` keyword consumed, so `validatePerformActionUsageReference`
  deliberately does not report a `perform` of a parameter. Across the snapshot corpus 74 fixtures
  reach `complete` (48 from `unsupported-syntax`, 26 from `parse-recovery`) and no fixture
  regresses from `complete`. The standard library's last parse recoveries are gone, which is what
  makes the next entry visible.

- **The semantic boundary consumes parser AST v240 without recreating source facts.** Declaration
  names, short names, string literals, comment bodies, opaque declaration text and fallback
  operators are resolved through their owning `ParsedDocument` spans. Semantic names and literal
  values are decoded once at lowering; authored comment bytes remain authored. The grammar-owned
  implicit `objective` role is derived by objective-membership lowering rather than fabricated in
  the syntax tree. Integer literals outside `i64` now remain explicit parser recovery instead of
  silently becoming zero.

- **The 94-document standard library publishes `unsupported-syntax` honestly, and 1,963 of its
  unsupported members now lower.** Parser recovery in `Flows.sysml`/`Interfaces.sysml`/
  `Items.sysml` had masked the library's unsupported syntax behind `parse-recovery`; with the bump
  the completeness is the true one, and 94 library-admitting fixtures record it. The members that
  lower: `:>> x = value;` in an attribute body (1,885, the ISQ quantity definitions), dotted
  feature-chain targets of `subsets`/`references`/`crosses`/`redefines`/`chains`/`inverse of`
  (resolved hop by hop through the member-access resolver, keeping their relationship kind),
  `inverse of` (`featureInverting`), the declaration-led `flow f : T ... from a to b;` form in every
  body that admits a flow, SysML `succession` usages in connection, occurrence, state, requirement
  and part-usage bodies, and a usage's `intersects` clause. Seven library members remain
  unsupported (`subclassifier` relationship declarations, `{true}` expression bodies, `return ref`,
  a constraint's `return result = ...`, `satisfy requirement ... by that`, a transition with an
  accept payload in a state body); each is named in the gaps document.

- **The generator API treats an unsupported-syntax publication the way it already treated parser
  recovery.** `satisfy_relationships` and `requirement_verifications` returned an error for a
  publication whose only defect was unsupported syntax in an admitted library, while every other
  query and the crate's own `outcome` helper handed the settled values over with the incomplete
  flag set; a requirement's typing likewise collapsed to `Unsupported` instead of the wire's
  `Recovered*` family. All three now follow the same policy, `SatisfyRelationship::recovered` and
  `RequirementVerification::recovered` are documented as "the publication is not complete", and
  the generation fixtures are byte-identical to before the bump. No wire schema change.

- **Nine constructs the bump unblocked now lower, and five validation rules fire on them.**
  `end` on every occurrence-usage family (`end part`, `end [1] part bead : TireBead;`, `end
  theCauses [*] occurrence theCause :> causes;`) publishes the `end` modifier and the owned cross
  feature as a `ReferenceUsage`; `#Tag` extension keywords on any usage, `ref` declaration or
  enumerated value lower as `metadataAnnotation` references, and the keyword-less `#Tag <name>`
  `ExtendedUsage` is its own `DeclarationKind::ExtendedUsage` (`ElementKind::Usage`); a
  `MetadataBody`'s nested definition, alias and import members lower; `include use case v : V;`
  declares a nested use case; `require constraint c : C;` keeps its typing; a `perform <path>;`
  target is a `referenceSubsetting` reference; a named connector end (`connect bead references
  t.bead to ...`) is a positional end declaration; `end derived x : T;` / `end in x : T;` carry
  their `RefPrefix`; a state definition's or usage's `parallel` body modifier is the `parallel`
  modifier fact; and a classifier's `conjugates A` / `~ A` clause is a `conjugation` reference,
  which `deriveTypeOwnedConjugator` now answers. `::>`/`references`, `crosses`, `inverse of` and
  a usage's `intersects` clause resolve through the subsetting pass instead of settling as
  `unsupported_reference`.

  New diagnostics: `end_feature_has_direction` (KerML `validateFeatureEndNoDirection`),
  `metadata_body_feature_invalid` (`validateMetadataFeatureBody`), `port_owned_usage_composite`
  and `port_nested_usage_composite` (SysML `validatePortDefinitionOwnedUsagesNotComposite` /
  `validatePortUsageNestedUsagesNotComposite`, over a canonical `usage_is_composite` fact that
  follows the Pilot: a usage is composite unless `ref`, `end`, attribute-like or a reference
  usage) and `parallel_state_substate_transition` (`validateStateDefinitionParallelSubactions` /
  `validateStateUsageParallelSubactions`); `end_feature_invalid_restrictions` also covers
  `portion`. The KerML end-feature fixtures author their violating side in SysML's
  `DefaultReferenceUsage` spelling, the one production in either language that puts a `RefPrefix`
  beside `end`. Fourteen validation fixtures came off `blocked_by`; the two whose violating side has
  no concrete-syntax spelling at all (a second `conjugates` clause, a `var` end feature) are now
  `by_construction` fixtures blocked on `abstract_syntax_coverage_gap` issues, and the fixtures
  whose remaining blocker is semantic rather than syntactic name it (`lowering-gap-kerml-relationship-declarations`,
  `lowering-gap-state-usage-specialization-facts`, `semantic-trigger-invocation-argument-typing`,
  `semantic-feature-chaining-rules`, `parser-gap-66-subsetting-clause-count`).

- **The snapshot tool builds each fixture once.** The per-fixture sequential/parallel
  construction lanes are gone; that invariant is proven once by the authority's own
  `construction_schedule_parity` test over the examples corpus. `cargo snapshot check` runs the
  tool in release over the cached library stratum: a full check is now ~2 seconds. The workspace
  test suite runs in ~18 seconds (from ~56), with integration tests as one binary per crate and
  the resolver and parser compiled at opt-level 1 in dev.

- **A symbol entry names its element by handle; the qualified name is read from the
  publication.** `SymbolEntry::qualified_name` is gone. Document and workspace symbols are
  produced in bulk -- one entry per declaration in a document, every one carrying a `Box<str>`
  copy of the `::`-joined path the publication already stores. Consumers read it through the
  borrowed `PublishedModel::qualified_name`, `display_label` takes the name it should fall back
  to, and the LSP, generator and report edges materialise text only where a protocol demands
  it. Rendering is unchanged, so snapshot output is byte-identical. Enforcement: the owned-string
  inventory's product list in `architecture.rs` shrank by one entry; the three entries that
  remain are synthesised text with nothing to borrow from, and the guard's comment says why.

- **Authored text is a handle too: `sysml_contract::TextId`.** A published fact that quoted text
  the publication had already interned was allocating a second copy of it per result.
  `TextId` is the publication-scoped slot of one interned run, with the same validity story as
  `SymbolId`, and `PublishedModel::text` / `PublishedResolution::text` borrows the run back.
  `AuthoredUnit::authored` -- the unit token as written between the brackets -- is the first fact
  to carry the handle instead of a `Box<str>`; the hover edge reads the text through the
  publication. Rendering is unchanged, so snapshot output is byte-identical. Enforcement: the
  owned-string inventory's *product* list in `architecture.rs` shrank by one entry.

- **A qualified-reference candidate names its element by handle.**
  `QualifiedReferenceTarget::qualified_name` is gone. The caller supplied that name in the
  `QualifiedElementReference` it asked with, so handing a `Box<str>` copy back -- once per
  candidate of an ambiguous or wrong-kind outcome -- duplicated both the request and the
  publication's own storage. Consumers read it through the borrowed
  `PublishedResolution::qualified_name`. Rendering is unchanged, so snapshot output is
  byte-identical. Enforcement: the owned-string inventory's *product* list in `architecture.rs`
  shrank by one entry.

- **An element inspection names its element by handle; the qualified name is read from the
  publication.** `ElementInspection::qualified_name` is gone. It was a `Box<str>` copy of the
  `::`-joined display path the publication already stores, allocated once per inspection -- and
  inspections are produced in bulk by details, hover and the feature inspector. Consumers now read
  it through the borrowed `PublishedModel::qualified_name` / `PublishedResolution::qualified_name`,
  which slices the settled blob, and the LSP and generator edges materialise an owned string only
  where their protocols demand one. Rendering is unchanged, so snapshot output is byte-identical.
  Enforcement: the owned-string inventory's *product* list in `architecture.rs` shrank by one entry.
- **Diagram scene ids are typed; relationships, edges and transitions carry no synthesised
  string.** `DiagramRelationship::semantic_id`, `DiagramEdge::semantic_id` and
  `DiagramStateTransition::semantic_id` were `Box<str>`s composed from an occurrence key and a
  suffix, allocated per scene item and used only to order the scene and to pair a transition with
  its origin. A relationship now carries its `ordinal`; an edge and a transition carry the `origin`
  index into the projection's elements, and a transition its `DiagramTransitionRole`. Ordering is
  unchanged (the same key is compared, once, at construction), and the generator boundary renders
  the transition id through `DiagramViewProjection::transition_scene_id`, so protocol output is
  byte-identical. Enforcement: the owned-string inventory's product list in `architecture.rs`
  shrank by three entries.

- **A projected diagram relationship states its kind as an enum, not as text.**
  `DiagramRelationship::kind` was a `Box<str>` of the canonical reference name, so every consumer
  that dispatched on it -- the edge composer, the transition-feature lookup, the generator
  boundary -- was a string comparison no compiler checks, where a typo silently produces a
  missing edge instead of an error. `sysml_contract::DiagramRelationshipKind` has one variant per
  reference kind the resolution authority publishes (58 of them), and `name()` returns the same
  canonical text, so the published product and the scene keys derived from it are byte-identical.
  `DiagramIncompleteReason`'s three relationship arms carry the enum too.

  Making the set exhaustive surfaced two comparisons that could never match: the state-transition
  scene looked up a `transitionGuard` relationship and the completeness rule required
  `messageSource`/`messageTarget` for a sequence view, none of which any reference kind emits.
  Both are now written as what they always evaluated to -- an absent guard, and nothing required
  of a sequence view -- rather than as a lookup that quietly fails. Enforcement: the owned-string
  inventory's product list in `architecture.rs` shrank by one entry.

- **A diagram catalog entry names its view by handle; the display name is read at the edge.**
  `DiagramViewCatalogEntry` no longer carries a `Box<str>` of the view usage's name. The catalog
  lists every authored standard view in a workspace, so it was allocating one copy per entry --
  and a second for the entry the projection echoes back -- for text only a view picker renders.
  The authored-name-else-qualified-name fallback stays the authority's rule rather than moving to
  each consumer: `PublishedResolution::diagram_view_name` (facade: `diagrams().view_name`)
  applies it and borrows the result. Catalog order was already the handle's canonical identity
  order, so nothing about ordering or output changes. Enforcement: the owned-string inventory's
  product list in `architecture.rs` shrank by one entry.

- **A state-transition vertex names its element by handle; the label is read at the edge.**
  `DiagramStateVertex` is now `Copy` -- a `SymbolId`, a vertex kind and a `SourceLocation`. The
  label it used to carry was a copy of the element's authored name, which the publication already
  stores, allocated once per vertex for text only a renderer needs. A consumer reads it with
  `symbol_name`, which borrows from the settled symbol blob. Output is byte-identical: the
  generator boundary materialises the same `unwrap_or_default` empty label for an anonymous state.
  Enforcement: the owned-string inventory's product list in `architecture.rs` shrank by one entry.

- **A navigation result names its element by handle; the name is read at the editor edge.**
  `NavigationTarget` is now `Copy` -- a `SymbolId` and a `SourceLocation` -- and
  `RenameOutcome::Ready` no longer carries a `Box<str>` of the name either. A definition or
  reference query that returns several candidates was allocating a copy of a name the publication
  already stores, once per candidate, for text most consumers never render. `PublishedModel`
  (and `PublishedResolution`) grew a borrowed `symbol_name`, which slices the settled symbol blob;
  the LSP edge materialises there, where the protocol demands an owned string anyway. Candidate
  ordering still breaks ties on the authored name, read through the publication, so output is
  byte-identical. Enforcement: the owned-string inventory's *product* list in `architecture.rs`
  shrank by one entry.

- **The owned-string inventory says which owned strings are debt.** `FACADE_OWNED_STRING_FIELDS`
  split into `FACADE_OWNED_STRING_INPUT_FIELDS` -- names a consumer hands in to ask a question,
  which no publication storage backs and which are correctly owned -- and
  `FACADE_OWNED_STRING_PRODUCT_FIELDS`, the copies a query hands back, which is the list meant to
  shrink. Both sets are asserted exactly and are checked for overlap, so a product cannot be
  reclassified as an input to escape the rule.

- **A published location names its document by handle, not by a copy of the URI.**
  `sysml_contract::DocumentId` is a `Copy`, publication-scoped ordinal with the same validity
  story as `SymbolId`: valid for exactly the publication that minted it, and
  `DocumentToken`/`PublishedModel::resolve_document_token` is what survives a rebuild or crosses a
  process boundary. `SourceLocation::document` is now that handle, so `SourceLocation` is `Copy`
  and every reference, rename occurrence, symbol entry and inspection stops carrying a per-result
  `Box<str>` of a string the publication already holds once. Hosts materialise the identity where
  they need text, through the borrowed `PublishedModel::document_identity`; `document_of` is the
  inverse a host uses when an editor request names a document by URI. Contractually ordered
  results still sort by identity, not by ordinal, so output is byte-identical. Enforcement: the
  `FACADE_OWNED_STRING_FIELDS` inventory in `architecture.rs` shrank by one entry.

- **The keystroke-path facade products are borrowed views over the publication, not owned copies
  of it.** `visible_members` answers with `VisibleMembers<'m>`/`VisibleMemberRef<'m>`: handles plus
  `&'m str` accessors that slice the publication's settled symbol, qualified-name and document
  blobs, so a completion request over the standard library allocates 4 times instead of 487. The
  qualified name comes from the barrier-settled `QualifiedNameIndex` that every other query reads,
  retiring a second owner-chain derivation. `PublishedDiagnostics<'m>` borrows the settled
  diagnostic sequence rather than cloning it per query, and `Diagnostic`, `DiagnosticLocation` and
  `RelatedLocation` publish their storage through `&str` accessors. Enforcement: the
  `FACADE_OWNED_STRING_FIELDS` inventory in `architecture.rs` shrank from 29 entries to 23.

- **The editor host and the batch host are siblings again: `lsp_server` no longer depends on
  `workspace`.** Batch validation — walk a directory, publish once, collect the diagnostics the
  publication settled — moved from `lsp_server::validation` to `workspace::validation`, where the
  rest of the batch path already lived. There is now one validation report type
  (`HostValidationReport`, grown an `advice` field and shared with the snapshot's own eager
  validation), one summariser, one target-discovery helper, and one entry point whose reporting
  policy the caller chooses. `server` reaches validation through `workspace` and keeps
  `lsp_server` only to launch the editor host; it no longer re-derives library advice by
  re-scanning the workspace's `.sysml` text and grepping the report it was handed. The report
  crosses the graph as neutral `SemanticDiagnostic` values, and `spec42 check --format json` keeps
  publishing the LSP diagnostic shape as a CLI projection, so existing baselines still apply.
  Enforcement: `architecture.rs` pins the exact dependency sets of both hosts, fails if
  `lsp_server/src/validation` reappears, and adds two AST guards — no host function takes SysML
  text as a parameter, and no host field keys derived data by document — each against an
  allow-list that only ever shrinks.

- **Spec42 now has a sole-authority pipeline: the parser is depended on only by `sysml_resolution`,
  `sysml_resolution` only by `sysml_query`, and `sysml_source` only by `sysml_resolution`; every
  consumer works with the facade's services and nothing else.** `design.md` at the repository root
  states the architecture: a source authority (`sysml_source`: admission, URI and line-ending
  policy, providers, digests), a semantic authority (`sysml_resolution`: the one parser call and
  its memo, lowering, resolution, library closure, publication and its session lifecycle), and the
  `sysml_query` facade with `source`, `syntax`, `library` and `publication` services obtained from
  one `Services` per host. A source revision is now parsed once — the editor's syntax queries and
  the semantic build share one memoised tree — and the editor host's index holds reference-counted
  documents and trees, so its actor's copy-on-write clone on every mutation no longer copies every
  file's text and tree. `semantic_publication` and `workspace_session` are retired
  (`session_actor` is the generic actor that remains); `workspace` is split into the
  `library_catalog` provisioning crate and a batch host. The on-disk parse cache and the unwired
  content-addressed cache substrate are deleted: the bundled standard library parses in
  milliseconds in parallel, and a disk tier may return only with a benchmark showing it beats
  recomputation. Enforcement: three `cargo deny` bans, std-only manifest and lockfile guards in
  `source_identity`, exact dependency pins in `architecture.rs`, and a new
  `syntax_authority.rs` guard that rejects retired helpers, shadowed service queries, caches and
  SysML file reads outside the authorities, and string probes for SysML syntax; the heuristics it
  still exempts are recorded with their retiring queries in `planning/SYNTAX_FOLLOW_UPS.md`.
- **Editor answers about declarations, imports and the cursor now come from the syntax service,
  with visible differences.** The outline publishes a typed `SyntaxOutlineKind` with the authored
  keyword as an accessor, each declaration's short name, its typing, and — new — its true
  multi-line extent, its header range and its body range; `ParsedSource` answers
  `declaration_at`/`enclosing_declarations`, `imports`, `type_references`,
  `referenced_namespace_roots`, `token_at`, `unit_literal_at` and `occurrences_of`. Consequences a
  user sees: folding regions now cover multi-line declarations (outline ranges used to end on the
  declaration's opening line, so almost nothing was foldable); workspace symbols and library
  search are classified by their element kind instead of arriving uniformly as "variable"; linked
  editing and the feature inspector no longer trigger from a keyword written in a comment or a
  string, and renaming through linked editing no longer rewrites a name inside a comment or a
  string literal; document links anchor to each import's own range rather than to any line
  containing the word `import`; and whether a workspace uses the standard library is decided from
  the namespaces its sources name rather than from a substring of its bytes. Signature help reads
  the declaration the cursor is in rather than the leading keyword of its line.
- **Library closure is resolved from parsed facts rather than text, with visible differences.**
  A `SysML::` mention in a comment no longer admits the `SysML` package (authored imports and
  typing references do); `import sysml::*` admits every package under a standard-library root by
  provenance rather than by a `sysml.library` path suffix; a unit catalogue is recognised by an
  attribute with a short name typed by a `…Unit` anywhere in the file, including at package level;
  a malformed library file still contributes the packages and imports it did declare.
- **Semantic-token highlighting no longer paints `provides`, `requires` or `value` as keywords.**
  The lexer's keyword table was a second copy that included three words OMG 8.2.2.1.2 does not
  reserve; both tables are now the facade's single 128-word vocabulary.
- **Every publication now admits the standard-library packages its implied specializations
  anchor into.** The library closure seeds the root packages of the resolver's generated library
  rules (`Parts`, `Items`, `States`, `Views`, `Requirements`, …) from standard-library roots, so
  `missing_library_anchor` no longer appears for models that never import them — including a
  workspace that declares its own `package Views`, since anchors resolve by standard-library
  role, not by bare name. `crates/server/tests/examples_are_clean.rs` now asserts every
  `examples/` workspace validates with no errors, warnings or infos.
- **Line endings are normalised to LF before a document is digested, on every path.** The
  editor already sent LF text; batch snapshots of CRLF files now carry the same digests the
  editor would, so `document_digests` in persisted artifact metadata change for CRLF files.

- **`spec42 check` and the workspace comparison harness agree on severity labels.** Both surfaces
  now render a severity through `sysml_diagnostics::severity_label`, the single owner. The
  comparison harness previously emitted `"information"` where the CLI emitted `"info"` for the same
  published diagnostic; the CLI's spelling wins, so comparison snapshots that recorded
  `"information"` now record `"info"`. The CLI text, JUnit, JSON, and SARIF outputs are unchanged.

- **`Definition::usage`/`directedUsage` and `Usage::usage`/`directedUsage` derive from the same
  effective feature membership.** The four SysML collections read the specialization closure that
  `Type::inheritedMembership` now publishes, selecting the usages a definition or usage owns *and*
  inherits, rather than returning a typed unavailable-fact outcome. One snapshot fixture came off
  `blocked_by` and the `lowering-gap-definition-usage-effective-feature-membership-closure` issue
  is retired.

- **KerML `Type` inherited-membership and feature collections derive from the canonical
  specialization closure.** `deriveTypeInheritedMembership`, `deriveTypeFeatureMembership`,
  `deriveTypeFeature`, `deriveTypeEndFeature`, `deriveTypeDirectedFeature`,
  `deriveTypeInheritedFeature`, `deriveTypeInput` and `deriveTypeOutput` now publish values instead
  of a typed unavailable-fact outcome. The inherited closure reuses the specialization ancestor
  index the resolver already owns, and a member a nearer feature redefines -- authored `:>>` or
  implied alike -- is not inherited. `deriveTypeOwnedFeatureMembership`,
  `deriveTypeMultiplicity` and `deriveTypeOwnedConjugator` stay explicitly unsupported: their
  normative result is a relationship or multiplicity identity the publication still does not own.
  Two snapshot fixtures came off `blocked_by` and the
  `lowering-gap-type-inherited-membership-closure` issue is retired.

- **Bumped the pinned `sysml-v2-parser` revision `49bdf3f` -> `f52100f`.** The 24-commit "corpus
  coverage wave 1" bump closes gap 68 in `planning/UPSTREAM_PARSER_GAPS.md` outright and narrows
  eleven more (61, 62, 66, 69, 72, 73, 74, 76, 77, 78, 79), each re-probed at the new revision
  through the blocked fixture's own source. Visible behavior changes: a metadata body
  is its own `MetadataBody` production whose members are reference redefinitions rather than nested
  attribute declarations, so `@Risk { totalRisk = 0.3; }` now publishes an anonymous feature with a
  real `redefinition` relationship and its evaluated value instead of a fabricated declaration
  named after the overridden feature; `flow`'s endpoints, `perform`'s target and `interface`'s
  connect ends are grammar-owned productions rather than expressions, so their endpoints, action
  references and endpoint labels are source-backed; and the newly typed `GuardedSuccession`,
  `then if`, and KerML type-body `package`/`library package` members lower rather than falling
  through. Across the snapshot corpus 24 fixtures leave `parse-recovery` -- 16 of them reaching
  `complete` -- and no fixture regresses from `complete`.

  One upstream regression came with it, recorded as gap 81 and pinned by two `sysml_resolution`
  tests: a directed KerML-kinded parameter (`in expr p : T;`, `in bool redefines a { ... }`) in a
  `calc`- or `constraint`-shaped body is now dropped to parse recovery. KerML `function`/`behavior`
  bodies, where the standard libraries author that spelling, are unaffected.

- **Two members the parser bump unblocked now lower.** A KerML `flow of T from a to b;` in a
  calc-shaped body carries its payload feature and both connector ends through
  `lower_flow_usage` instead of `unsupported_calc_definition_member`, and the declared
  `verify requirement <name> : <Type>;` form lowers as a named `VerifyRequirement` usage, so the
  generated requirement-verification library specialization applies to it. Five snapshot fixtures
  came off `blocked_by` and the `lowering-requirement-members` issue is retired.

- **The normative KerML and SysML validation constraints are now a traceable snapshot corpus.**
  `tests/snapshots/validation` covers all 180 constraints the two specifications name `validate*`
  -- 88 in KerML 1.0 and 92 in SysML 2.0 -- across 179 fixtures, each carrying its specification,
  OMG document identifier, exact clause, constraint name, a conforming and a violating example, and
  an authored `EXPECTED DIAGNOSTICS`. Rules the compiler does not enforce yet stay visible as
  `BLOCKED` by a typed issue that names the owning layer and concrete gap.
  The snapshot report now reconciles fixture evidence against the generated constraint manifest;
  `derive*` and `check*` are represented alongside `validate*` constraints rather than hidden by
  a separate planning inventory.

- **Bumped the pinned `sysml-v2-parser` revision `ec47463` -> `49bdf3f`.** Control nodes are the
  visible behavior change: `ControlNodeDeclaration` makes `merge continue;` a *declaration* rather
  than a reference to an existing element, so a named control node publishes as a named
  declaration instead of an anonymous node with an unresolved `joinInput`-style input reference.
  `Expression` folds `Parenthesized` and `Tuple` into one `Sequence` production and replaces
  `LiteralWithUnit` with `Bracket`, whose unit operand is a source-backed qualified reference
  rather than copied text, so unit identity is now read from the arena. `KermlFeature` moves its
  `chains`/`inverse of`/`featured by` clauses into `relationship_parts`, and `VariantUsage` gains
  an explicit `Reference`/`Typed` form discriminant. Sixteen new body-element variants across the
  usage families are reported as unsupported members rather than dropped.


- **Diagrams now cross the generator boundary.** The repository-owned Rust WASM diagram plugin
  emits a versioned JSON render product from the immutable model query API, and the VS Code webview
  renders it with the relocated D3/ELK package. All eight view kinds are selectable from the start:
  state transitions use their typed projection, while views awaiting owner-defined queries report
  explicit incomplete products instead of reconstructing semantic graphs. The obsolete Rust
  `diagram` crate is removed and `generator-plugins` is the home for production WASM plugins.

- **Every diagnostic Spec42 reports is settled by the immutable publication.** `sysml_resolution`
  now owns the conformance families the graph engine ran -- namespace identity, connection,
  behavior, requirement/case, view and inherited-value conformance -- and the two authoring hints,
  deciding each from a fact an earlier phase settled rather than from text. The typed contract
  gains an owner-produced message, the identity of the element a diagnostic is about, a note per
  related site, and `information` severity; `diagnostics().for_document()` answers one document
  from a prebuilt index. Workspace validation, the LSP, and `spec42 check` read that one result:
  the graph-backed engine, its check modules, helpers and entry points are deleted, and
  `sysml_diagnostics` is now the neutral diagnostic shape plus the one reporting policy a host has
  -- report only what the parser rejected for a document that does not parse.

  Diagnostic codes change with it. Every unresolved endpoint reports `unresolved_reference` or
  `unresolved_type_reference` at the same range instead of a per-relationship spelling
  (`unresolved_satisfy_source`, `unresolved_allocate_target`, `unresolved_ref_type_reference` and
  their siblings), because the publication settles every authored reference the same way;
  `ambiguous_name_reference` becomes `ambiguous_reference`; port compatibility reports
  `port_type_mismatch` or `flow_direction_incompatible` from the settled conjugation and feature
  directions rather than from the spelling of a type reference. `unresolved_pending_relationship`
  and its expression sibling described the graph's own pending queues and have no counterpart.
  **This is a knowingly partial cutover.** Deleting the graph engine dropped the families whose
  owning fact the publication does not hold -- metadata `about` and body bindings, user-defined
  keywords, case objective and verdict shape, initial-transition cardinality, allocation-usage
  typing, import kind conformance, and view expose filtering. Models that used to receive those
  checks no longer do. View `expose` members are now lowered, so `view_expose_unresolved` and
  `view_expose_empty` are owned and reported, and a view's expose members are no longer an
  unsupported construct.
  Three answers improve: a `connect a.fill to b.fill` is checked rather than skipped, two ports are
  compatible when they offer each other the features they expect rather than when one definition
  specializes the other, and a fan-out to distinct usages is no longer one connection repeated.

- **The feature inspector reads the immutable publication.** `sysml_resolution` publishes
  `element_details`: one cohesive answer per element carrying its inspection, what each authored
  relationship family settled to, the types it has once inheritance is taken into account, the
  features it inherits and the type that declares each, its metadata bindings, both directions of
  its relationships with authored/implied provenance, and both evaluation channels -- the value and
  a new `AnalysisEvaluation` verdict channel for analysis cases, verification cases, requirements
  and constraints. `sysml/featureInspector` is now a projection of that and nothing else: the
  qualified-name and simple-name target searches, the subsetting-chain and specialization-chain
  walks, and the "found a target, call it resolved" status inference are deleted, as is the
  unreachable inherited-attribute code lens that read evaluation facts by graph node. LSP type
  hierarchy moves to `PublishedModel::types()`, where a subsetting or redefinition is a hierarchy
  step as the specification says it is.

  The `sysml/featureInspector` response changes with it. The generic `attributes` map is gone --
  every key it carried is a typed field, and reading `evaluatedValue` out of a map is how
  presentation became a second truth store. `evaluation` has one variant per published state so a
  missing value cannot read as success, `analysis` is its own channel, relationship families report
  `partial`, `ambiguous` and `unsupported` beside `resolved` and keep ambiguous candidates out of
  `targets`, `referencedElement` becomes the tagged `referenced` outcome, relationships carry their
  provenance, and element kinds are the published OMG metaclass names. Three answers change because
  the publication owns them now: a KerML `feature`'s typing is authored-but-unresolved rather than
  not applicable, an unqualified name does not cross two documents that both declare `package P`,
  and an anonymously redefined feature is attributed to the type that declares it under a name.

- **Expression, value and unit conformance moved to the immutable publication.** `sysml_resolution`
  now lowers authored unit tokens, `filter` conditions and invocation arities as typed facts, settles
  them at the publication barrier, and answers them through `PublishedModel::evaluation()` alongside
  the evaluated value and the measurement reference a feature's type requires. A unit token resolves
  to the unit declaration it names -- `[kg]` is `SI::kilogram` -- and its dimension is that unit's
  measurement-reference type, so dimension compatibility is the ordinary specialization question
  rather than a string comparison against a hand-maintained alias table. "No catalog admitted",
  "unknown symbol", "ambiguous symbol" and "not a single unit symbol" are each their own outcome.
  `sysml_diagnostics::checks::expression_conformance` and the graph-derived unit catalog it read
  (`UnitRegistry::from_graph`, `graph_ingest`, `type_resolver`) are deleted. `invalid_enumeration_value`
  is retired: a string literal is not an enumeration literal whether or not the enum declares a member
  of that name, so `attribute_value_type_mismatch` reports it, and reports any value whose type is
  unrelated to its feature's. Hover reads the published evaluation for both the evaluated value and
  the unit literal.

- **Bumped the pinned `sysml-v2-parser` revision `b6291cc` → `ec47463`.** One upstream commit,
  closing the two gaps the previous bump left open. A keyword-less block comment ahead of a typed
  feature carrying a KerML type-relationship clause (`/* c */ feature f : T unions x;`) parses
  again in every calc-shaped body, for all four of `unions`, `intersects`, `differences` and
  `disjoint from`; that removes the recovery diagnostics and the keyword-named expression operands
  the previous pin published across seven fixtures. And a `requirement def` body now admits the
  usage families it inherits from the general member grammar -- `action`, `succession`, `perform`,
  `state`, `item`, `part`, `connect` and connection usages -- each dispatched here to the lowering
  its package- or part-level spelling already uses, except `SuccessionUsage`, which has no lowering
  in any scope and reports unsupported exactly as it does in part-usage and state-def bodies.
  `FirstMergeBody` became the shared `Body<E>` container, so its brace arm is destructured by field.

  Corpus-wide against the pre-bump baseline, parser recovery diagnostics fall 106 → 82 and
  `unsupported_*` diagnostics 489 → 462.

  **One pre-existing upstream defect is newly visible**, filed as gap 61 in
  `planning/UPSTREAM_PARSER_GAPS.md`: a calc-shaped body shreds `flow a.y to b.x1;`,
  `message m of T;` and the anonymous `redefines predecessors [0];` into bare expression members,
  with each keyword arriving as an ordinary feature reference and no diagnostic raised.
  `classifier`, `struct` and `behavior` bodies did this at `204ca48` already; `class` bodies joined
  them when `b6291cc` routed `class` through the shared KerML classifier declaration, moving it off
  the attribute-shaped body that parsed these members correctly. Keyword-shaped expression operands
  rise 23 → 38 across eight fixtures, concentrated in `tests/snapshots/kerml/moments.md`, whose
  `class` bodies author eleven `redefines <target> [mult];` members. The snapshots pin it.

- **Bumped the pinned `sysml-v2-parser` revision `204ca48` → `b6291cc`.** Upstream replaced the
  loose modifier booleans with the prefix components the grammar actually spells, and
  `sysml_resolution` reads them through three new splitters. `OccurrenceUsagePrefix` (SysML BNF 564)
  now carries `abstract`/`variation`, `derived`, `constant`, `ref`, `individual`, the portion kind
  and the direction for `part`/`item`/`port`/`occurrence` usages, each as the authored keyword's own
  span; `FeaturePrefix` (KerML BNF 584) does the same for KerML features, where `end`-ness is the
  alternative taken rather than a flag beside it; and `MultiplicityPart`'s ordering and uniqueness
  slots are two `Option`s over their authored spellings, so an authored `unique` is finally
  distinguishable from omission. Every definition kind gained the `definition_prefix` slot, so
  `variation` is recorded alongside `abstract` on requirement, case, analysis-case,
  verification-case, use-case, view, rendering and occurrence definitions.

  Three parser nodes were folded into the productions that spell them, and the model follows.
  `ClassDef` is gone: `class` routes through the shared KerML classifier declaration, so
  `DeclarationKind::ClassDefinition` is now the single kind every `class` spelling reaches and
  `KermlClass` is deleted. `TypedParameterMember` merged into `KermlFeature`, so `in expr p :
  Boolean = a;` lowers under the kind its keyword names (`kerml-expression`) instead of the generic
  `parameter`, keeping its direction on both the declaration and its typing reference.
  `KermlEndMember` merged into `FeaturePrefix`'s owned cross feature, which **inverts an ownership**:
  in `end happensDuring subsets ... feature thatOccurrence : ...;` the end-prefixed feature now owns
  the cross feature rather than the reverse.

  Newly typed members are lowered rather than reported unsupported: `calc` usages at package level,
  `ref` declarations in port bodies, `part` usages in constraint-def bodies, and nested
  `requirement def`s, `port` usages and `allocate` members in requirement bodies. Across the
  snapshot corpus that removes 31 `unsupported_*` diagnostics and adds 4.

  Two follow-on effects of that refactor were resolved by the `ec47463` bump below.

- **Bumped the pinned `sysml-v2-parser-next` revision `7eb7869` → `7d4fd85`.** The shared
  `RefPrefix` modifier chain (`abstract`/`variation`, `derived`, `constant`, `ref`, and the
  `in`/`out`/`inout` direction) is now accepted on every usage rather than a hand-picked few, and
  nine body-element sets gained the members the spec always allowed them: `ref` declarations in
  requirement, port, view, view-def, rendering and occurrence bodies; `end` and in/out parameter
  declarations in part-usage bodies; connection usages in occurrence bodies; concern and calc
  usages in requirement bodies; use-case, case and verification-case usages in use-case bodies;
  viewpoint usages and `satisfy` members in view-def bodies; and `require` constraints in
  constraint-def bodies. `sysml_resolution` lowers all of them through the owners that already
  existed for each node kind, and `item` usages now carry their `variation`, `derived` and
  `constant` modifier facts (`ItemUsage`'s bare `is_abstract` flag became the two-valued
  `usage_prefix`, so `variation item` is representable for the first time). Across the snapshot
  corpus `unsupported_parser_construct` falls 230 → 6 and `unsupported_grammar_form` 270 → 46,
  parser recovery diagnostics fall 114 → 104, and resolved references rise 16839 → 17497 with no
  fixture losing a resolution.
- **Bumped the pinned `sysml-v2-parser-next` revision `cb026cd` → `7eb7869`.** The parser's opaque
  `Other(String)` body member is gone from eleven scopes: content a scope cannot parse is now a
  recovery node with its authored span and a diagnostic, and a spec-valid member the scope does not
  model is an explicit `Unsupported` node. Body delimiters are retained (`Body<E>` is one container
  for all 27 declaration bodies), the four annotating members are one `AnnotatingMember` production,
  a `ref` body is the general usage-member set, an `if` branch keeps its authored braced/brace-less
  spelling, and an action usage's absent body is distinct from `;`. Across the snapshot corpus,
  parser recovery errors fall 224 → 81 and 49 fixtures reach `complete`; members that used to be
  swallowed opaquely now lower for real (KerML classifier/connector/feature/invariant members in
  part, attribute, package and relationship bodies; `bind`, `calc`, `connection` and constraint
  members in attribute bodies; `attribute`/`calc`/nested `action def` in action bodies; `ref` and
  in/out parameters in use-case bodies; `then send ...;` continuations; typed `flow` ends). A `:>`
  clause on a parameter is now lowered as the subsetting it is rather than as a typing, an untyped
  `actor x;` no longer invents a type reference, and `type X ...` gains its own `Type` element kind.
  `subclassifier X specializes Y;` is the one construct whose coverage narrows: it is a
  relationship declaration upstream now, not a classifier declaration, and is reported as an
  unsupported package member until it is lowered.
- Moved active plans under `planning/` and adopted a remove-on-completion policy.

## [0.50.0] - 2026-08-07

- **Bumped `sysml-v2-parser` 0.53.0 → 0.54.0** ([#18](https://github.com/elan8/spec42/issues/18)) —
  pulls in #70 (`parse()`/`parse_for_editor()` equivalence), the #78 follow-up (full-library
  strict-diagnostics gaps: `in`/`ref`/`calc` redefinition forms, nested `calc def`, `abstract
  calc`), and #85 (connector-end/interface/flow shorthand gaps). `PARSE_AST_VERSION` moved 70 → 71
  for this release's AST-shape changes, which broke exhaustive matches across `sysml_model` and
  `sysml_tokens`: new `InOutDecl.is_redefinition`, `CalcDefBodyElement::{CalcUsage,CalcDef,
  PartUsage}`, `ConstraintDefBodyElement::{Constraint,AttributeUsage}`,
  `RequirementDefBodyElement::{SubjectRef,VariantUsage,Constraint}`,
  `StateDefBodyElement::InOutDecl`, `OccurrenceBodyElement::EndDecl`,
  `InterfaceDefBodyElement::FlowUsage`, `InterfaceUsageBodyElement::EndDecl`,
  `InterfaceUsage::TypedConnect.name`, `EndDecl.crosses`, `UseCaseDefBodyElement::{ActionUsage,
  AnalysisCaseUsage,CalcUsage,AttributeUsage,RequirementUsage,PartUsage,Expression}`, and
  `Expression::{Conditional,Extent}`. Each new variant got real graph-builder/semantic-token
  handling (not stub catch-alls) by mirroring the closest existing sibling variant's
  materialization, reusing shared helpers where one already existed (`materialize_part_usage`,
  `materialize_attribute_usage`, `materialize_requirement_usage`,
  `materialize_top_level_action_usage`, `materialize_analysis_case_usage`, `collect_semantic_
  ranges_ref_decl`) and factoring two new ones where duplication would otherwise have tripled
  (`calc_constraint_def::{build_calc_def_body_elements,materialize_calc_usage}`, now shared by
  `part_def.rs`'s top-level `calc` usage arm, this module's own nested-calc-in-calc rollups, and
  analysis/verification case bodies' nested `calc` usages). Also fixed two independent breaking
  changes from the same bump: `RefBody::Brace`'s `elements` are now `RefBodyElement`-wrapped
  (`action.rs`'s `add_ref_decl` now filters to the `Action(..)` variant before recursing), and
  `CaseReturnDecl.value_expression` was renamed/retyped to `value: Option<Node<FeatureValue>>`
  (`analysis_case.rs` now unwraps `.value.expression`). A few genuinely new constructs
  (`RequirementDefBodyElement::Expression`'s bare analysis-case result expression,
  `AssertConstraint` in the newly-reachable body contexts) were left as documented no-ops rather
  than guessed at, matching this codebase's existing "not yet modeled" precedent for the same
  constructs elsewhere. Full workspace (`cargo test --workspace`, 130 suites) green; `cargo fmt`,
  `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo doc --no-deps --workspace`
  all clean. This is a partial step on #18, not a 1.0 pin confirmation -- the OMG-library-baseline
  and roadmap-documentation acceptance criteria are still open.

- **Removed the MCP server and read-only HTTP API** ([#51](https://github.com/elan8/spec42/issues/51)) —
  `spec42-mcp` (stdio MCP server) and `spec42 api serve` (read-only HTTP API, `docs/api/`) are gone.
  Investigation found no AI host actually depended on either surface: VS Code Language Model Tools
  and Cursor already call the `spec42` CLI directly, and no external/partner consumer of the HTTP
  API was found. CLI JSON output (`check`, `model-summary`, `explain-diagnostic`, `doctor`) plus a
  documented per-host skill/instructions pattern (see `docs/user/AI-ASSISTANTS.md`) is now the sole
  AI-integration surface for 1.0. `diagnostic_catalog` (shared by the CLI and the removed MCP tools)
  moved from `crates/server/src/mcp/` to `crates/server/src/diagnostic_catalog.rs`.

- **Fixed interconnection-view layout non-determinism** (O-6, tracked since 0.35.0's Known Issues below and [#22](https://github.com/elan8/spec42/issues/22)) — repeated `spec42 diagrams export` for the same unchanged model could produce different node coordinates each run. Root cause: `merge_ibd_payloads_inner` and `normalize_ibd_to_instance_paths` (`crates/sysml_model/src/semantic/ibd/{merge,instance_paths}.rs`) deduplicate parts/ports/connectors through a `HashMap`, then collect the deduped values straight into the output `Vec` via `.into_values()` — `HashMap`'s default hasher is randomly seeded per process, so that order (stable within one run) differed across separate CLI invocations, silently propagating into `finalize_merged_ibd_connectors`'s instance/def remap and from there into which of several structurally-equal-but-differently-worded connector representations survived dedup, and ultimately into ELK's node/edge order. Fixed by sorting every `HashMap`-derived collection by its DTO's own natural key right after collection, and switching `IbdDataDto.root_views` from `HashMap` to `BTreeMap` (it serializes directly to a JSON object, so its hash-randomized key order was a second, independent source of byte-level non-determinism). Verified byte-identical SVG output across repeated exports of both `examples/webshop` and the larger `sysml-robot-vacuum-cleaner` fixture; action-flow-view was independently re-verified deterministic on both fixtures too (the underlying `activity_graph.rs` extraction doesn't use the `HashMap`-to-`Vec` pattern that caused the interconnection-view bug). Remaining known non-determinism: `stats.modelBuildTimeMs` in the JSON payload is a real wall-clock measurement and legitimately varies run to run — structural (not byte) comparison is still required for that one field.

- **Supports typed numeric analysis results and objective evaluation.** Analysis `return attribute`
  declarations are materialized with their type and expression, inherited by typed analysis usages,
  evaluated as numeric values rather than Boolean verdicts, and checked against typed objective
  requirement constraints. Existing `return ref` and verification verdict behavior is preserved.

- **Fixed a high-severity DNS rebinding vulnerability in the bundled MCP server** ([RUSTSEC-2026-0189](https://rustsec.org/advisories/RUSTSEC-2026-0189)) — `rmcp` (the crate backing `spec42-mcp`'s Streamable HTTP server transport) was pinned to 0.9.1; upgraded to 1.4+ (resolved to 1.8.0), which also drops the unmaintained `paste` dependency (replaced upstream by `pastey`). The upgrade's API changes (tool/result construction moved to non-exhaustive structs with builder methods) are internal to `crates/server/src/mcp/server.rs`; MCP tool behavior is unchanged (confirmed via the existing `mcp_tools`/`mcp_protocol`/`mcp_binary` integration suites).
- **Removed the unused `adm-zip` VS Code extension dependency**, which carried a high-severity vulnerability ([GHSA-xcpc-8h2w-3j85](https://github.com/advisories/GHSA-xcpc-8h2w-3j85)). Neither `adm-zip` nor `@types/adm-zip` were referenced anywhere in the extension's source; the scripts that do handle VSIX/zip files already use `@vscode/test-electron`'s downloader and the shell `unzip` command. Also picked up a `brace-expansion` fix via `npm audit fix` (no source changes needed).
- **Hardened CI**: added a `cargo audit` job (dependency vulnerability scanning), `npm audit --omit=dev` steps for both `shared/diagram-renderer` and `vscode`, and a `cargo doc --no-deps --workspace` step with `RUSTDOCFLAGS=-D warnings` to `ci.yml`'s `rust-core` job (fixed the 14 pre-existing broken/private intra-doc links this surfaced, across `sysml_model`, `workspace`, and `server`). Also pinned `rust-toolchain.toml` and every CI/release Rust-install step to an exact version (`1.97.1`) instead of the floating `stable` channel, so a new stable Rust release adding a default-warn lint can no longer break CI on an unrelated day.

## [0.49.0] - 2026-07-29

- **Aligns standard-view projection with SysML v2.** `GeneralView`, `BrowserView`, `GridView`,
  and `GeometryView` now respect their specified exposed scope and relationship-filter
  semantics. Browser and Grid presentations are no longer marked provisional; Geometry remains
  explicitly partial while authored spatial geometry and 3D rendering are deferred.
- **Keeps focused architecture views focused.** Product decomposition retains the explicitly
  exposed part usages and definitions with their membership and typing relationships, avoiding
  unrelated firmware and requirement-subject expansion.
- **Updates the bundled Elan8 libraries.** Domain libraries now use `0.3.0` and method
  libraries use `0.2.0`, including the new root-level `Elan8` package organization and the
  expanded electronics, mechanical, sensing, actuation, assembly, and real-time vocabulary.

## [0.48.0] - 2026-07-28

- **Lets you move the Visualizer between the secondary sidebar and an editor tab.** Commands
  `sysml.moveVisualizerToEditor` and `sysml.moveVisualizerToSidebar` keep the same webview
  session; closing the editor tab returns the view to the sidebar. From the editor tab you can
  use VS Code's "Move into New Window" for a separate window.
- **Simplifies the Visualizer toolbar.** Home, Legend, Export, and layout direction are
  icon-only actions with hover labels; the view dropdown still shows the selected view name.
- **Serves Library dashboard status over LSP.** The extension asks the language server for
  bundled, custom, and Sysand library status instead of reconstructing it only from local
  config, so the Library view stays aligned with what the server actually loaded.
- **Refreshes Spec42 user docs and keeps library overviews in sync with product pins.** New
  guides cover Model Explorer and Feature Inspector; What's Included and domain/method library
  pages are generated from `vscode/package.json`, `config/libraries/*.json`, and the bundled
  KPAR artifacts (package/file trees). Help links to those pages, and the broken OMG SysML v2
  language-spec URL is corrected to `https://www.omg.org/spec/SysML/2.0/`.

## [0.47.3] - 2026-07-27

- **Pins Elan8 method libraries to `0.1.1`.** Domain remains `0.2.0`; What's Included / Library
  defaults follow `config/libraries/*.json`.

## [0.47.2] - 2026-07-27

- **Treats domain and method libraries as managed KPAR pins.** Config lives under
  `config/libraries/`, fetch/pack scripts and workspace embedding load each pinned `.kpar`, and
  the VS Code Library defaults are generated from those pins.
- **Adds golden parity tests between the webview and headless diagram export paths.**
  `shared/diagram-renderer` fixtures now render through both a real jsdom DOM (the same
  path the VS Code webview uses) and the headless virtual DOM (`spec42 diagrams export` /
  `POST /v1/diagrams/export`), and assert identical structural SVG markers (node classes,
  edge classes, marker ids) against a checked-in golden fixture per view. A regression that
  breaks parity between the two paths — for example a renderer change relying on a DOM API
  the headless virtual DOM doesn't implement — now fails CI instead of silently shipping a
  divergent CLI/API export. See `docs/engineering/DIAGRAM-EXPORT-QUALITY-ANALYSIS.md`.

## [0.47.1] - 2026-07-27

- **Moves interconnection port labels into the ELK layout graph itself, with a dedicated overlay
  layer.** ELK now reserves node space for each port label, positions it inside the port boundary,
  and returns the coordinates the renderer uses, instead of the label being placed heuristically
  after connector routing. A new overlay layer draws port labels above connector lines so crossing
  routes no longer obscure them.

## [0.47.0] - 2026-07-27

- **Completes the Feature Inspector's resolved-semantics view.** Protocol v2 now exposes
  declared and effective typing separately, specialization, subsetting and redefinition targets,
  inherited features with their declaring definitions, multiplicity, direction, feature
  modifiers, documentation and applied metadata. The VS Code inspector renders each semantic
  concept as a navigable section while remaining compatible with 0.46 server responses.
- **Keeps interconnection labels legible in dense routing areas.** Connector paths are painted
  before a dedicated top label layer, labels prefer the longest horizontal route segment instead
  of crowded bend points, and a canvas-colored halo prevents crossing lines from obscuring text.
- **Expands deterministic SysML v2 code actions.** Unresolved and ambiguous references can now
  import or qualify matching symbols and create missing definitions; untyped part usages can
  create and apply a matching `part def`; implicit redefinitions can be made explicit with `:>>`;
  and requirement declarations can generate a verification-case skeleton. Verification and
  analysis cases that trigger `case_subject_missing` offer a preferred subject fix, while
  definition declarations offer a refactor that creates a correctly typed usage with a derived
  lower-camel name. Contextual library actions help when a symbol requires standard or custom
  library configuration. The source edits preserve indentation and avoid duplicate declarations.

## [0.46.1] - 2026-07-26

- **Accepts canonical item flows between action parameters without a false
  `succession_endpoint_invalid` warning.** Plain `flow producer.signal to
  consumer.signal` relationships carry items; they are not action-succession
  relationships. Succession validation now skips parameter-to-parameter item
  flows while retaining the existing diagnostic for flows from action usages
  to invalid non-action targets.

- **Keeps ignored model copies out of host/CLI workspaces and deduplicates view
  candidates by semantic ID.** The graph-first filesystem provider now honors
  `.gitignore`/`.ignore` with the same rules as the LSP scanner, preventing
  generated `target/` fixtures from re-entering validation when
  `--workspace-root` points at a repository root. Public view catalogs no longer
  repeat a view when duplicated inputs reach candidate projection.

- **Adds the VS Code Feature Inspector.** The Spec42 view container groups the
  Visualizer and Feature Inspector. The inspector accepts standard and legacy
  request shapes, and the extension sends both URI representations during the
  0.46.1 transition so packaged clients also interoperate with older server
  binaries. Optional cancellation tokens are omitted correctly instead of becoming
  a spurious positional JSON-RPC parameter; the server also tolerates that legacy
  `[params, null]` transport artifact. Its selection-driven protocol now
  distinguishes exact keywords, element names, resolved references, values, and
  units. Keywords show only structured language help shared with hover;
  references make the resolved target primary; values and units use a compact
  value context; and model elements use one canonical declaration/identity/type/
  value/relationship/source layout with server-supplied semantic roles. It
  follows editor selection, buffers lazy-webview updates, and opens from diagram
  navigation. VS Code 1.106 is now the minimum because that release made the
  secondary-sidebar view containers generally available.

- **Improves diagram export reliability for large graphs.** Headless rendering
  now handles layout failures and oversized ELK graphs with a deterministic
  fallback instead of producing an empty or failed export.

- **Materializes standard dependency endpoints semantically.** A
  `dependency name from client to supplier` now contributes a typed
  client-to-supplier `dependency` edge, including across imported documents,
  instead of projecting only an isolated relationship node. Dependencies owned
  by a `part def` are accepted as standard definition members and retain their
  nested qualified identity.

- **Improves hover and source navigation fidelity.** Hover documentation for all
  128 reserved words is audited against the OMG SysML 2.0 textual grammar and
  protected by a normative keyword-set regression test. Enum/reserved-keyword
  token classification is more precise, and diagram navigation can jump to the
  corresponding model element.

## [0.46.0] - 2026-07-24

- **Fixes the stack-overflow crash on startup** originally reported against this workspace
  (`thread 'main' has overflowed its stack`, exit code `STATUS_STACK_OVERFLOW`, during the initial
  workspace scan/parse/relink). Root cause and fix:

  - **Parser dependency `sysml-v2-parser` 0.47.0.** Expression parsing (parenthesized groups,
    argument lists, postfix/feature chains) was recursive-descent with no depth limit; deeply
    nested or pathological expressions could overflow the parser's native stack before the fix in
    0.46.1, and 0.47.0 additionally fixes real, previously-silent misparses this surfaced while
    hardening it: `notEmpty(x)` (Kernel Semantic Library, used ~16 times) was parsed as
    `not Empty(x)`, and `newSeq` (Kernel Function Library) as `new Seq()` — both `Unsupported`
    keyword-boundary bugs, not crashes, so nothing previously caught them. See that crate's own
    changelog for full detail. `PARSE_AST_VERSION` moved `43` → `44`; both the parse cache and the
    library graph cache (see below) invalidate automatically.

  - **Spec42's own expression-tree code made stack-safe too.** The parser fix alone only moved the
    risk downstream: this workspace has its own family of functions that walk the parsed
    expression tree (semantic-fact normalization, attribute/diagnostic text rendering, the
    quantity- and analysis-expression evaluators, host/API snapshot projection) using native
    recursion with no depth limit of their own, previously safe only because the old parser
    rejected pathologically deep input before any of this code ever saw it. All of them —
    `sysml_model::semantic::ast_util::declared_expression`, `graph_builder::expressions::{
    expression_to_debug_string, expr_node_to_qualified_string, classify_expression}`,
    `extracted_model::expr_to_string`, the `QuantityParser` and `AnalysisExprParser` evaluators,
    and `workspace::snapshot::facts::project_expression` — are now iterative (explicit heap-`Vec`
    work stacks instead of call-stack recursion), verified output-identical against the full test
    suite plus new 200,000-deep regression tests for each. `DeclaredExpression` and the analysis
    evaluator's own `AnalysisExpr` also picked up custom iterative `Drop` impls, since Rust's
    default derive walks a deeply nested recursive value's teardown the same unsafe way.

  - **`library_graph_cache`'s version check now also covers the parser's AST schema
    (`PARSE_AST_VERSION`)**, not just spec42's own version — previously a parser-only dependency
    bump like this one could leave a stale, pre-fix library graph cached on disk until spec42's
    own version next changed.

  - Two directory walkers (`workspace::library::bundle::directory_contains_models`,
    `server::environment::workspace_references_standard_library`) that recursed via
    `fs::read_dir` with no symlink-cycle guard now use `walkdir` with `follow_links(false)`,
    matching the existing safe pattern in `workspace::scan`.

- **Fixes duplicated views and slow startup when a workspace has a gitignored `target/` (or
  similar) directory containing stray `.sysml`/`.kerml` files** — e.g. leftover manual negative-test
  fixtures or exported diagram artifacts. `lsp_server::workspace::scan::scan_sysml_files` previously
  walked every file under a workspace root unconditionally (`walkdir::WalkDir`, no exclusions), so
  such directories were scanned and merged into the live workspace model right alongside the real
  source files, producing duplicate/triplicate entries for anything named the same across copies and
  needlessly bloating startup time. Now uses `ignore::WalkBuilder` (`follow_links(false)`,
  `require_git(false)` so `.gitignore` is honored even outside an actual git working tree) — the
  same ignore-rule engine ripgrep and rust-analyzer use — so gitignored directories are skipped like
  everywhere else in the project. Verified against the real `sysml-robot-vacuum-cleaner` project:
  candidate files dropped from 84 to the real 28, with 0 remaining under `target/`.

- **Fixes a stack-overflow crash on ordinary, valid edits** (e.g. opening/relinking
  `PhysicalArchitecture.sysml` after the fixes above) that is unrelated to either of them: on
  Windows, the OS gives a process's initial thread a 1 MiB stack by default (vs. 8 MiB typical on
  Linux/macOS), while every other thread Rust spawns — including tokio's own worker and
  blocking-pool threads — defaults to a larger stack. Debug builds don't inline nom's heavily-generic
  combinator chains, so ordinary, only-a-few-levels-deep SysML constructs (a `part` nested inside a
  `part def`, with a `port` inside that) can genuinely need more than 1 MiB of native stack to parse,
  with no pathological nesting or unbounded recursion involved — confirmed by reproducing the crash
  in complete isolation with a ~10-line file, and confirming it disappears entirely either in a
  release build or by simply moving the same parse call onto any Rust-spawned thread. `spec42`'s
  `main()` now spawns the actual server/CLI logic onto a dedicated thread with a 64 MiB stack instead
  of running it on the OS-provided main thread, matching what every other thread in the process
  already gets.

## [0.45.0] - 2026-07-24

- **Parser dependency `sysml-v2-parser` 0.46.0.** Part-body `ref action` / `ref state` /
  nested `action`·`state` (Systems Library `Parts.sysml` `performedActions` shape) materialize
  as real `action`/`state` nodes with reference ownership facts, typed headers, and
  `subsetsFeature` — not `"opaque member"`. Plain `ref` visibility is projected. Token ranges
  cover the new `PartDefBodyElement` / `PartUsageBodyElement` variants. Full P5+ unified
  definition/usage rewrite remains deferred.

- **R9 robot-vacuum zero-warning gate.** Showcase pin advanced to
  `2a08e9007319a45ef5a4c65c78423eab18e4fe5e` (project `Ah` / `mAh` / `ms` units via
  `VacuumCleanerQuantitiesAndUnits`). CI fetches the pin and runs `spec42 check` via
  `robot_vacuum_check` (zero errors / zero warnings) plus the host view smoke
  (`robot_vacuum_snapshot`).

## [0.44.19] - 2026-07-23

- **One release version across Cargo, VS Code, and Zed.** The VS Code package manifest and lock,
  Zed extension manifest, and Zed crate manifest/lock now match the Cargo workspace version.
  `scripts/sync-workspace-version.mjs` derives these values from `[workspace.package]`, and the
  release preflight rejects version drift.

- **Bounded parser stack consumption.** Parser-heavy library-closure loading and batch
  diagnostics now run on named workers with a known 2 MiB stack instead of inheriting an
  arbitrary caller's remaining stack. The temporary 32 MiB stack around the entire CLI has
  been removed. Together with `sysml-v2-parser` 0.45.1, deeply nested input now produces a
  `nesting_too_deep` diagnostic before recursive descent, while wide models remain unrestricted.
  Regression coverage exercises both entry points from a 256 KiB caller stack and parses a
  10,000-member flat package.

- **General View renderer also drops documentation/comment boxes.** Server-side
  folding already excludes them from `generalViewGraph`, but cached projections
  and any fallback to raw `graph` still showed `<documentation> Unnamed` nodes.
  `isNonDiagramSemanticElementType` now treats `documentation`/`comment` like
  imports (not diagram nodes), so Babel42 Explorer Views hide them immediately.
  Coverage: `omits documentation and comment nodes from general-view diagrams`.

- **General View no longer draws documentation/comment as standalone boxes.** SysML §7.4
  Documentation annotates its owning element; Spec42's notation inventory already marks
  `documentation-node` as not shipped and expects compartment text instead. Anonymous
  `documentation` children (empty name → renderer "Unnamed") plus membership + Annotation
  edges were appearing as dual-arrow boxes under action defs in General View. They are now
  filtered like parameters/require-constraints in `is_general_view_inline_detail`, while the
  owner's `doc` attribute remains. Coverage:
  `canonical_general_view_graph_filters_documentation_nodes`.

- **Owned metadata / parameter qualified names now nest under the owning element.** Body
  `@annotations` already recorded membership/`parent_id` on the annotated owner, but
  `add_metadata_annotation_node` built the usage QN from the caller's type-resolution
  prefix. Requirement (and concern/view) bodies deliberately keep that prefix as the
  enclosing package so typing of `subject`/`actor`/etc. resolves siblings — which made
  `@RequirementRole` appear as `Pkg::RequirementRole` instead of `Pkg::Req::RequirementRole`.
  The same helper mismatch also affected nested owners when the body walker passed an
  outer container (e.g. metadata inside a `calc def`). Fixed by always nesting the usage QN
  under `parent_id` while keeping the separate prefix only for typing edges (same split
  `#keyword` metadata already used). Applied the same ownership/typing split to
  `add_in_out_decl` and calc `return` parameters so Outline/tree UIs that nest by QN
  ancestry match graph ownership. Coverage:
  `requirement_body_metadata_annotation_materializes_on_graph`,
  `calc_def_metadata_annotation_qn_nests_under_calc`, and a QN assert on the existing
  part-body metadata test.

## [0.44.18] - 2026-07-20

- **S42-004 slice: `PartUsage`/`RefDecl` multi-target typing now wired, plus the
  `Actions.sysml` redefines-then-typing data-loss bug closed.** `sysml-v2-parser` 0.45.0 stopped
  collapsing a comma-separated `:` typing clause on `PartUsage`/`RefDecl` to a single joined
  display string (`typing: Option<Node<TypingRelationship>>` now carries every target, and
  `RefDecl.redefines` carries the `:>>` target that previously vanished entirely once
  `action_ref_decl` saw `:>>` -- see Systems Library `Actions.sysml`'s `SendAction.sentMessage`/
  `AcceptMessageAction.acceptedMessage`). Both graph-builder consumers only ever read the old
  flattened `type_name: String` for edge resolution -- `usage_builders::materialize_part_usage`
  and the shared `ref_decl::materialize_ref_decl` (used by every `ref`-declaration context:
  action bodies, connection/interface def bodies, part def/usage bodies) -- so a workspace
  element's typing edge to the second (or later) comma-separated target, and the `Actions.sysml`
  redefines target, were never real graph edges even though the parser now retains them
  losslessly. Fixed by looping over `typing_targets(n.typing.as_deref())` (the same helper
  `AttributeDef`/`AttributeUsage`/every `xxxDef` specializes clause already use) instead of
  passing the joined string as a single target, and setting the existing generic `"redefines"`
  node attribute from `n.redefines` so the already-generic `link_subsetting_family_edges_for_node`
  pass wires it, same as `PartUsage.redefines` already does -- no new edge-wiring mechanism
  needed either way. No `PROJECTION_SCHEMA_VERSION` bump: this corrects which edges/attributes
  the existing open-ended `HashMap<String, Value>` attribute system produces, not
  `HostSemanticProjection`'s typed shape. Regression coverage:
  `crates/sysml_model/tests/part_def_body_semantics.rs`'s
  `part_usage_multi_target_typing_emits_an_edge_per_target`, and
  `crates/sysml_model/tests/ref_relationship_semantics.rs`'s
  `multi_target_ref_typing_emits_an_edge_per_target` /
  `action_ref_redefines_then_multi_target_typing_wires_both` (the latter mirrors the exact
  `Actions.sysml` shape). Babel42-side DTO consumption not yet done -- see
  `babel42-v2/docs/spec42-systems-modeling-api-gaps.md` S42-004.

## [0.44.17] - 2026-07-19

- **S42-005 slice: workspace elements that reference a library type now
  resolve to a real, addressable node instead of an empty `target_id`.**
  `project_host_semantic_model` only ever projected nodes whose URI was in
  `target_files` (workspace documents), even though `library_urls` was
  already threaded in for the (correctly implemented, but previously
  dead-in-practice) `is_library_element` marking. A workspace element's
  `Typing`/`Specializes`/`Subsetting`/etc. edge to a library target was a
  real graph edge, but since the library node was never projected, the
  relationship-building loop's `semantic_ids.get(&tgt_id.qualified_name)
  .unwrap_or_default()` silently produced an empty `target_id` -- worse
  than a dangling reference, since there was no ID at all to report as
  unresolved. Fixed with a deliberately narrow, one-hop policy: after
  projecting workspace nodes, a second pass walks each workspace document's
  outgoing edges and projects the specific target node only if it falls
  under one of the caller's `library_urls` (via the existing
  `uri_under_any_library` check), marking it `is_library_element: true`.
  Not a full library-closure projection -- the referenced element's own
  further outgoing edges (its supertype chain) and containing
  package/parent chain are not pulled in this round, so `owner`/
  `owningNamespace` stay unresolved for these nodes, matching the class of
  limitation other "additive, not complete" fixes this session have
  shipped. Node construction factored into `build_host_semantic_model_node`
  so both passes share the same ~70-line conversion. `project_semantic_model`
  (`crates/workspace/src/incremental.rs`) now takes `library_urls: &[Url]`
  and forwards it instead of hardcoding `&[]`; its two non-babel42 callers
  (its own test, `lsp_server`'s `built_workspace.rs`) pass `&[]` to
  preserve today's LSP validation behavior exactly -- confirmed via a
  regression test (`validate_paths_with_semantics_excludes_non_target_library_nodes`)
  that a workspace element referencing something outside `target_files`
  stays excluded when `library_urls` is empty, i.e. the new pass is gated
  on genuine library membership, not merely "outside the workspace." New
  test `projection_includes_library_element_referenced_by_workspace_typing`
  uses a genuine two-document (library + workspace) fixture to exercise the
  fix end-to-end. No `PROJECTION_SCHEMA_VERSION` bump: existing fields only,
  more nodes and correctly-resolved `target_id`s. Deliberately out of scope
  this round: whole-closure/`excludeUsed` query-parameter support,
  recursive inclusion of a referenced library element's own further typing
  chain, and projecting its containing package/parent chain.

## [0.44.16] - 2026-07-19

- **S42-002 slice: `references`/`crosses` now materialize as real
  `ReferenceSubsetting`/`CrossSubsetting` edges, not discarded text.** The
  parser already fully parses `::>`/`references` and `=>`/`crosses` into
  `references`/`crosses: Option<Node<SubsettingRelationship>>` AST fields,
  and spec42 already read them -- but only into write-only display attributes
  `referencesFeature`/`crossesFeature` that nothing ever read again, unlike
  `subsetsFeature`/`redefines`. Same shape of bug as the S42-004 typing fix
  earlier this session: data already parsed and present, just discarded
  before becoming a real edge. Fixed: new `RelationshipKind::
  ReferenceSubsetting`/`CrossSubsetting` (KerML 8.3.4.4/8.3.4.5, both
  `Subsetting` specializations); `link_subsetting_and_redefinition_edges_for_node`
  renamed to `link_subsetting_family_edges_for_node` and generalized to a
  loop over all four subsetting-family attribute keys instead of two
  hand-duplicated blocks, now also resolving `referencesFeature`/
  `crossesFeature` to real targets via the same `resolve_subsets_or_redefines_target`
  helper `subsetsFeature`/`redefines` already used. New
  `HostRelationshipMetaclass::ReferenceSubsetting`/`CrossSubsetting`, mapped
  in `relationship_metaclass` (`facts.rs`). No `PROJECTION_SCHEMA_VERSION`
  bump: new enum variants only, matching this session's established
  precedent. Everything else in S42-002's original backlog (`TypeFeaturing`,
  `FeatureInverting`, `Unioning`, `Disjoining`, `Intersecting`,
  `Differencing`, general non-port `Conjugation`) remains a genuine parser
  gap -- confirmed via direct grep of `sysml-v2-parser`: no AST, no parser
  functions for any of them (`Intersecting` is tokenized and explicitly
  discarded by `skip_intersects_clause`). Also confirmed during this
  investigation and worth recording: plain `Subsetting`/`Redefinition` and
  imports/aliases (`NamespaceImport`/`MembershipImport`/`AliasMembership`,
  shipped under S42-003) were *already* real addressable relationships --
  the S42-002 doc prose predates that work and had gone stale. Regression
  coverage: extended `crates/workspace/src/snapshot/facts.rs`'s
  `projection_relationship_family_subset_redefine_specialize_annotation`
  test with `references`/`crosses` fixtures.

## [0.44.15] - 2026-07-19

- **S42-001 slice: `semanticId` no longer breaks on a position-shifting, non-renaming edit.**
  `semantic_element_id` (`crates/workspace/src/snapshot/facts.rs`) hashed
  `(uri, kind, declaration_range.start.line, declaration_range.start.character)` -- not
  `qualified_name` at all -- so *any* edit that shifted a declaration's line/column (an unrelated
  sibling inserted above it, reordering, reformatting) silently reassigned its ID, even though the
  element and its qualified name were completely untouched. This was strictly worse than "renames
  break identity": almost any edit anywhere in a file could reshuffle every downstream element's
  ID. Fixed by hashing `qualified_name` instead of position: `NodeId`
  (`crates/sysml_model/src/semantic/model.rs`), the graph's own primary key, is already
  `(uri, qualified_name)`, and `qualified_name_for_node`'s `#kind`/`#kind2` disambiguation loop
  already guarantees that pair is collision-free within a document -- `semantic_element_id` just
  wasn't using it. `semantic_relationship_id` inherits the fix automatically for every
  relationship kind except `Connection` (the only kind whose discriminator carried position, to
  distinguish multiple connectors between the same endpoint pair); `edge_identity_discriminator`
  now uses only `declaring_uri` + the endpoint expression text for that, dropping the position
  component. Internal hash version tags bumped v2 -> v3 on both functions (hash-domain hygiene,
  not a schema change). No `PROJECTION_SCHEMA_VERSION` bump: same opaque `s42e:`/`s42r:`-prefixed
  string field, same shape, only the value-computation recipe changed. **Deliberately still
  deferred** (S42-001's larger, harder remainder): true cross-commit rename tracking (needs a
  real cross-snapshot matching heuristic or a persisted identity ledger -- `compare_elements` in
  `crates/workspace/src/comparison/elements.rs` already matches by `(uri, qualified_name)` for
  diffing, but isn't reachable from any babel42 HTTP endpoint); the narrow "same-name sibling
  swap" case (two same-named-colliding siblings can still swap which one gets the `#kind`/
  `#kind2` suffix, and therefore which ID, if their relative order changes); the separate
  IBD/interconnection diagram ID scheme (already qualified-name/dot-path keyed, untouched here).
  Regression coverage: new `crates/workspace/tests/semantic_id_stability.rs` (node ID stable
  across a position shift; typing relationship ID stable likewise; a `Connection` edge stable when
  its `connect` statement moves but endpoints don't change; an actual rename still changes the ID,
  as expected; two distinct `Connection` edges between the same endpoint pair still get two
  distinct IDs).

## [0.44.14] - 2026-07-19

- **S42-004 slice: every explicit comma-separated typing/specialization target is now
  materialized, not just the first.** SysML v2 allows a feature or definition to be typed/
  specialized by multiple comma-separated targets (`attribute reading : Weight, Height;` is
  equivalent to `attribute reading defined by Weight defined by Height;`; `part def Combined
  specializes BaseA, BaseB;` is two independent `Subclassification` relationships). The parser
  already retained this losslessly in `TypingRelationship.target: Vec<Node<RelationshipTarget>>`
  (used by `AttributeDef`/`AttributeUsage.typing` and by `specializes` on every `xxxDef` struct),
  but every spec42 consumer discarded everything after the first target -- `ast_util::typing_target`/
  `typing_target_display` called `.first()`, and `TypeReferenceTarget for TypingRelationship`/
  `for Node<TypingRelationship>` did the same when a builder passed the whole relationship struct
  directly into `add_typing_edge_if_exists`/`add_specializes_edge_if_exists`. New
  `ast_util::typing_targets` returns every target; `graph_builder::mod::insert_def_specialization_attr`/
  `wire_def_specialization_edge` (the shared choke point for ~13 `xxxDef` materializers routed
  through `package_body/materialize.rs`, `calc_constraint_def.rs`, `view_def.rs`) and every direct
  `AttributeDef`/`AttributeUsage` typing consumer (`attribute_body.rs`, `usage_builders.rs`,
  `part_def.rs`, `package_body/materialize.rs`, `occurrence_body.rs`, `port_def.rs`,
  `requirement_body.rs`, `verification.rs`, `analysis_case.rs`) now loop over every target instead
  of resolving one. Deliberately out of scope (unchanged from S42-004's existing deferral, now
  documented explicitly): multi-target typing for usage kinds other than `AttributeDef`/
  `AttributeUsage` (their AST only ever retained a flattened `type_name: Option<String>`, so
  recovering multiple targets needs a parser AST change -- the actual "large, multi-release" part
  of S42-004); implied/standard-library default typing; the `is_implied` flag (exists on both the
  parser AST and the projection layer already, but nothing ever sets it `true`, so wiring it
  through today would be a no-op); end-owning-type/cross-feature/feature-chain collections. No
  `PROJECTION_SCHEMA_VERSION` bump: more edges of the already-existing `RelationshipKind::Typing`/
  `Specializes` kinds, no new fields. Regression coverage:
  `crates/workspace/tests/snapshot_single_build.rs`'s new
  `snapshot_materializes_every_target_of_a_multi_typed_attribute_and_specializes_clause`.

## [0.44.13] - 2026-07-19

- **`of X` payload feature on named flows now materializes as a real child node (SysML v2
  8.2.2.16 `PayloadFeature`).** An audit of this session's work against the OMG spec text found
  that `of qty : Payload[1..3]` (a *named* payload feature) was a hard parser failure -- the
  whole flow statement dropped into error recovery -- because `FlowUsage::payload` was parsed as
  a bare `Expression` (`optional_payload`, `sysml-v2-parser/src/parser/flow.rs`), which only
  happened to work for the bare-type-reference form (`of Payload`). Per spec, `of X` is a real
  owned `FeatureMembership` (`PayloadFeature : Feature = Identification? PayloadFeatureSpecializationPart
  ValuePart?`), not a value reference. Fixed upstream in `sysml-v2-parser` 0.43.0: `FlowUsage::payload`
  is now `Option<Node<PayloadFeature>>` (`name`/`type_name`/`multiplicity`), parsed by a new
  `payload_feature` combinator accepting typing and multiplicity in either order (mirroring
  `feature_usage_header`'s existing either-order handling, but retaining the multiplicity instead
  of discarding it). New `ElementKind::FlowPayload` (kind-string `"flow payload"`).
  `materialize_flow_usage` (`graph_builder/flow_usage.rs`) now materializes the payload as a real
  child node of the flow for every *named* flow with a payload -- unconditionally on
  `flow.payload.is_some()`, not gated on whether a name/multiplicity was actually written, since
  `Identification?` only makes the *name* optional, not the feature itself; a bare `of Payload`
  gets a synthetic `"_payload"` name, matching the existing `"_assign"`/`"_terminate"` convention
  for unnamed control-flow children. Unnamed flows (`flow of Payload from a to b;`) have no node
  to own the feature and keep only the existing edge-level `payload_type_id` scalar (deliberate
  scope limit, not a regression -- avoids a second synthetic-containment precedent in the same
  session). The new child node is additive: `add_flow_edge_if_both_exist`'s existing
  `payload_type_id` resolution is unchanged (simplified to read the now-plain `type_name` field
  instead of extracting it from an expression), so `FlowStatementDetail.payload_type_id` still
  resolves for both named and unnamed flows. No `PROJECTION_SCHEMA_VERSION` bump: new enum
  variant + additive node only, same class as the `Else`/`ConjugatedPortDefinition` additions.
  Also fixed in the same audit: `TextualRepresentation` was missing its
  `representedElement`/`documentedElement`-style back-reference (KerML `AnnotatingElement`) --
  see `babel42-v2`'s changelog for that fix, which lives entirely on that side (DTO +
  `openapi.json` only, no spec42 change needed). Regression coverage:
  `crates/workspace/tests/snapshot_single_build.rs`'s extended
  `snapshot_projects_flow_detail_with_payload_and_succession_kind` (bare `of Payload` still gets
  a synthetic-name child node) and new
  `snapshot_materializes_named_flow_payload_feature_with_multiplicity` (name/type/multiplicity
  all correct, `payload_type_id` regression guard, real `Typing` edge to the resolved type).

## [0.44.12] - 2026-07-19

- **S42-009: conjugated port definitions synthesized (KerML 8.3.12.2-8.3.12.4, SysML v2
  8.2.2.12/8.4.8.1-8.4.8.2).** `port p : ~P;` previously discarded the `~` during type
  resolution -- `p`'s typing edge pointed straight at `P`. Per spec, `~P` denotes an implicit
  `ConjugatedPortDefinition`, nested as a real `ownedMember` of `P` (not a sibling, not
  ownerless -- confirmed directly against the OMG spec text, correcting an earlier draft of
  this work), linked back via a `PortConjugation` relationship (a `Conjugation`, not a
  `Typing`/`Specializes` edge). `materialize_port_def`
  (`graph_builder/package_body/materialize.rs`) now eagerly materializes this conjugate
  alongside every non-conjugated port def -- matching the spec's own eager-parsing semantics,
  not lazily on first `~`-typed usage -- so usage-time resolution
  (`add_typing_edge_for_node`, `relationships.rs`) is a pure lookup (resolve `P`, then find its
  already-materialized conjugate child), never node creation, eliminating an entire class of
  find-or-create raciness. New `ElementKind::ConjugatedPortDefinition` and
  `RelationshipKind::PortConjugation`; both added to the relevant typing-target allowlists
  (`kinds.rs`) to avoid a spurious `incompatible_type_kind` diagnostic regression.
  `effective_port_features`/`port_definition_qualified_name` (`diagnostics/helpers.rs`) now
  follow the conjugate's `PortConjugation` edge to find the real `PortDef` for feature/name
  lookups, fixing a silent regression the conjugate redirect would otherwise have introduced in
  `connection_conformance.rs`'s `port_compatibility_mismatch` check (previously untested for
  conjugated ports). Two independent typing-resolution engines needed the same conjugation-aware
  fix -- `add_typing_edge_for_node` (`relationships.rs`, the immediate per-document and deferred
  `link_workspace_relationships` paths) and `resolve_typing_edge_cross_document_inner`
  (`relationships/cross_document.rs`, the parallel full-build and scoped-incremental paths) --
  discovered by a test failure showing both a correct and an incorrect typing edge coexisting
  from the same usage; fixing only one silently left the other producing the pre-fix (direct-to-
  `P`) edge. No `PROJECTION_SCHEMA_VERSION` bump: new enum variants only, same class as the
  `Else`/`Terminate`/`While`/`If` additions. Deliberately out of scope this round: the
  conjugate's own features (the in/out-reversed mirror of `P`'s owned features) are not
  independently materialized as child nodes -- comparable in size to the already-deferred S42-004
  implied-typing work. Regression coverage:
  `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_materializes_conjugated_port_definition_eagerly` and
  `snapshot_conjugated_port_structural_mismatch_uses_feature_check_not_fallback`.

## [0.44.11] - 2026-07-18

- **Flow payload resolves to a real type; `TextualRepresentation` is addressable.**
  `FlowStatementDetail` (the `Flow`/`SuccessionFlow` edge detail attached to a resolved
  `succession flow dataFlow of Payload from a to b;`) gains `payload_type_id: Option<String>`:
  `of Payload` previously stayed raw debug text (`payload_expression`) with no resolved
  reference, unlike `source`/`target`, whose edge endpoints were already resolved to real
  feature IDs. `add_flow_edge_if_both_exist` (`graph_builder/flow_usage.rs`) now resolves the
  payload the same way `add_typing_edge_if_exists` resolves any type reference (it names a
  type, not a feature path, so it goes through `resolve_type_target_in_workspace` +
  `TYPING_TARGET_KINDS`, not the feature-path resolver used for `from`/`to`). The
  graph-builder layer stores the resolved qualified name; `workspace`'s `project_host_semantic_model`
  translates it into the same semantic-ID space as `source_id`/`target_id` before exposing it,
  mirroring the existing two-step handoff. New `ElementKind::TextualRepresentation`
  (kind-string `"textualRep"`, unchanged) gives `rep <name>? language "..." { ... }` nodes —
  already fully materialized by `materialize_textual_rep`/`requirement_body.rs` with
  `language`/`text` attributes, just previously falling into `Unknown("textualRep")` — a
  distinct, addressable classification instead. `PROJECTION_SCHEMA_VERSION` bumped `12` → `13`:
  new `FlowStatementDetail` field, same class as the 0.44.9 `isPortion`/`portionKind` addition.
  Regression coverage: `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_projects_flow_detail_with_payload_and_succession_kind` (extended) and new
  `snapshot_materializes_textual_representation_as_addressable_node`.
- **S42-009 (conjugated ports) status clarified.** `isConjugated` was already fully wired and
  tested (a `port p : ~P` usage already reports `isConjugated: true` via `HostFeatureProperties`).
  The remaining gap — synthesizing implicit `ConjugatedPortDefinition`/`ConjugatedPortTyping`
  elements per the KerML metamodel, so `p`'s typing points at the conjugated definition rather
  than directly at `P` — is deliberately deferred: it would be the first "implicit node not
  present in source text" pattern in this codebase, a genuine architectural decision on the
  same footing as S42-004/S42-005, not a small fix. See
  spec42-systems-modeling-api-gaps.md S42-009 for the updated status.

## [0.44.10] - 2026-07-18

- **`if`'s `else` branch materialized; `add_for_loop` nesting bug fixed.** `add_if_stmt`
  (`crates/sysml_model/src/semantic/graph_builder/action.rs`) previously flagged `hasElse` but
  never walked the else body (deferred in 0.44.5). It now materializes a nested `"else"`-kind
  child node under the `if` node when an else branch is present, whose own children are the else
  body's elements -- mirroring the existing `for`/`while` body-wrapper pattern rather than
  flattening then/else children into one list. New `ElementKind::Else` (kind-string `"else"`). No
  `PROJECTION_SCHEMA_VERSION` bump: new `ElementKind` value plus the existing generic node/attrs
  shape, same class as the 0.44.5 `Terminate`/`While`/`If` additions. Separately, `add_for_loop`
  was passing the outer `container_prefix` instead of the loop's own qualified name as the
  `container_prefix` for its body recursion -- the only control-flow body materializer with this
  bug (`while`/`if` already passed their own qualified name correctly) -- so nested actions inside
  a `for` loop did not have qualified names reflecting the loop's nesting. Fixed to pass
  `Some(qualified.as_str())`. Regression coverage: `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_materializes_terminate_while_and_if_control_nodes`, extended with a `for` loop fixture
  asserting nested qualified-name scoping and an `else` branch fixture asserting the new `"else"`
  node, its `parent`, and its child.

## [0.44.9] - 2026-07-18

- **`isPortion`/`portionKind` added to declared feature properties (S42-008).** `DeclaredFeatureProperties`
  and `HostFeatureProperties` gain `is_portion: bool` and `portion_kind: Option<String>`.
  `occurrence_usage_feature_properties` (`ast_util.rs`) now populates both from
  `OccurrenceUsage::portion_kind` (already set by the `snapshot`/`timeslice`/`then timeslice`
  parser forms, unchanged this round) — previously only the raw `"portionKind"` debug attribute
  existed, with no slot in the typed feature-properties system Babel42's `isPortion`/`portionKind`
  DTO fields actually read. All other `*_feature_properties` builders (`part`/`attribute`/`port`/
  `item`/`definition`) explicitly set `is_portion: false, portion_kind: None`, since only
  occurrences have portion semantics. `PROJECTION_SCHEMA_VERSION` bumped `11` → `12`: new
  `HostFeatureProperties` fields, same class as the 0.44.0 `content_expression_id` addition.
  Regression coverage: `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_populates_occurrence_def_and_usage_facts`, extended with `is_portion`/`portion_kind`
  assertions for `snapshot`/`timeslice`/plain `occurrence` usages.
- **`isReadOnly`/`isSufficient`/`isVariable`/`mayTimeVary` confirmed out of scope.** Exhaustive
  search of `sysml-v2-parser`'s grammar found no `readonly`, `variable`, `sufficient`, or
  time-varying keyword production anywhere — `readonly`/`variable` were already documented as
  deliberately out of scope (not SysML textual keywords, "Modifier completeness audit"); this
  extends the same finding to `sufficient` and time-varying, none of which have a textual BNF
  production either. Babel42's hardcoded `null` for these four properties is therefore not a gap
  to close via keyword wiring — see spec42-systems-modeling-api-gaps.md S42-008 for the
  reclassification.

## [0.44.8] - 2026-07-18

- **`expose` materializes as a real Import.** `annotate_view_usage_body`
  (`crates/sysml_model/src/semantic/graph_builder/view_def.rs`) previously only recorded
  `hasExpose`/`exposeTargets` text attributes on the owning view node -- no addressable element
  or relationship existed for an `expose` member at all. `expose` is normatively an Import per
  `ExposeMember`'s own BNF doc comment; new `materialize_expose_member` (using
  `sysml-v2-parser` 0.42.0's `ExposeMember::is_import_all`/`::is_recursive`) creates a real
  `"import"`-kind child node with the exact same attribute shape `materialize_import` already
  uses for ordinary `import` statements (`importTarget`/`importAll`/`recursive`). This reuses
  the existing `membership_kind`/`membership_relationship_metaclass` pipeline entirely --
  `HostMembershipKind::Import` already classifies further into
  `HostRelationshipMetaclass::NamespaceImport`/`::MembershipImport` based on the `importAll`
  attribute -- so `expose` publishes as a concrete `NamespaceImport`/`MembershipImport`
  relationship with zero new classification logic. The old `hasExpose`/`exposeTargets` summary
  attributes are left in place alongside the new node.
- **`filter`'s condition is now a real addressable `Expression`.** Both `add_view_filter_node`
  and `build_filter_member` (`view_def.rs`) previously only stored a debug-text `"condition"`
  attribute. Both now also set `declared_facts.own_expression` (mirroring
  `TransitionGuard`'s 0.44.0 `content_expression_id` fix, S42-003), so `filter`'s condition is
  the real parsed `Expression`, not rendered text.
- No `PROJECTION_SCHEMA_VERSION` bump: both changes reuse existing `HostSemanticProjection`
  shape (generic node/attrs, and the pre-existing `content_expression_id` mechanism). Regression
  coverage: `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_classifies_expose_as_namespace_or_membership_import` and
  `snapshot_projects_filter_condition_as_an_addressable_expression`.
- **Deliberately still open:** `filter`'s own node kind (`ElementKind::Filter`) and a `view
  rendering` member (`ElementKind::ViewRendering`) remain without their own confirmed Systems
  Modeling API concrete resource shape at the Babel42 layer (tracked as a Babel42-side follow-up,
  not a Spec42 gap); `expose`'s optional bracket-filter form (`expose X [ expr ];`) still skips
  its filter expression's content entirely, same as before this round.

## [0.44.7] - 2026-07-18

- **`OccurrenceDef.isAbstract` fix.** 0.44.6 set a raw `"isAbstract"` attribute on `occurrence
  def` nodes, but Babel42's `isAbstract` DTO field only reads
  `node.facts.feature_properties.is_abstract` or a `"definitionPrefix"` string attribute (the
  `PartDef`-only convention) — a raw attribute alone was silently dropped.
  `materialize_occurrence_def` now builds `DeclaredFeatureProperties` directly (`OccurrenceDef`
  has a plain `is_abstract: bool`, not the `DefinitionPrefix` enum
  `definition_feature_properties` expects) and attaches it the same way `PartDef` does. Caught
  by the downstream Babel42 contract test before this shipped anywhere.

## [0.44.6] - 2026-07-18

- **Occurrence definition/usage facts enriched.** `materialize_occurrence_def`
  (`crates/sysml_model/src/semantic/graph_builder/package_body/materialize.rs`)
  never set `isAbstract`, unlike its `requirement def`/`case def` siblings.
  `materialize_occurrence_usage`
  (`crates/sysml_model/src/semantic/graph_builder/usage_builders.rs`) never
  called `attach_feature_properties` at all (so `is_individual` was silently
  discarded) and never surfaced `portionKind`/`isThen`/the `subsets`/
  `redefines`/`references`/`crosses` subsetting-clause targets already
  parsed onto `OccurrenceUsage` — all four AST fields existed, unlike
  `AttributeUsage`'s identical shape, which already projects them. Both
  gaps closed: `occurrence def` now sets `isAbstract`; `occurrence`/
  `individual`/`snapshot`/`timeslice`/`then timeslice` usages now attach
  `occurrence_usage_feature_properties` (new `ast_util.rs` helper, mirroring
  `item_usage_feature_properties`, populating `is_individual` from
  `OccurrenceUsage::is_individual`) and the same four subsetting-clause
  attributes `AttributeUsage` already sets. No `PROJECTION_SCHEMA_VERSION`
  bump: existing `HostFeatureProperties`/attribute shape, no new fields.
  Regression coverage: `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_populates_occurrence_def_and_usage_facts`.

## [0.44.5] - 2026-07-18

- **`terminate`/`while`/`if` control nodes materialized.** `ActionDefBodyElement::TerminateStmt`/
  `::WhileStmt`/`::IfStmt` and their `ActionUsageBodyElement` counterparts previously matched a
  silent no-op arm in both `build_from_action_def_body` and `build_from_action_usage_body`
  (`crates/sysml_model/src/semantic/graph_builder/action.rs`) and were dropped from the graph
  entirely, unlike their sibling control statements (`perform`/`merge`/`decide`/`join`/`fork`/
  `for`/`assign`, all already materialized). New `ElementKind::Terminate`/`::While`/`::If`
  (kind-strings `"terminate"`/`"while"`/`"if"`) plus `add_terminate_stmt`/`add_while_stmt`/
  `add_if_stmt` close the gap: `terminate` records an optional `terminateTarget` debug-text
  attribute (mirroring `merge`/`decide`/`join`/`fork`'s existing text-only target pattern);
  `while` records a `whileCondition` debug-text attribute and walks its body as children via the
  existing `build_from_action_def_body`; `if` records an `ifCondition` attribute plus a
  `hasElse: bool` flag and walks only its `then` branch as children this round. The `else` branch
  body, when present, is flagged by `hasElse` but not yet walked -- materializing it needs a
  design decision on how to scope a second child branch, deferred as a follow-up. No
  `PROJECTION_SCHEMA_VERSION` bump: new `ElementKind` values plus the existing generic node/attrs
  shape, same class as the pre-existing `merge`/`decide`/`join`/`fork` additions. Regression
  coverage: `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_materializes_terminate_while_and_if_control_nodes`.

## [0.44.4] - 2026-07-18

- **`ConcernDefinition`/`ConcernUsage` classification.** `sysml-v2-parser` 0.41.0 added
  `ConcernUsage::is_definition` (previously the parser matched the optional `concern def` keyword
  but discarded whether it was present, since `concern_usage` handles both textual forms via one
  AST struct -- no separate `ConcernDef` node exists, unlike `case`/`case def`).
  `materialize_concern_usage` (`crates/sysml_model/src/semantic/graph_builder/package_body/materialize.rs`)
  now branches on that flag to materialize kind-string `"concern def"` (`ElementKind::ConcernDef`)
  or `"concern"` (`ElementKind::Concern`) instead of always tagging `"concern"`. `ElementKind::ConcernDef`
  was already in `is_definition()` and the typing/specializes allowlists (`kinds.rs`), so
  `membership_kind()`'s generic `kind.is_definition() => OwningMembership` rule and the existing
  `ElementKind::Concern => FeatureMembership` arm now classify both forms correctly with no
  further change. `concern` usages are only legal at package level (no `PartDefBodyElement`
  variant exists), so there is no nested-in-`part def` dispatch gap to fix here, unlike the
  `case`/`use case`/`analysis`/`verification` family. No `PROJECTION_SCHEMA_VERSION` bump: pure
  classification, same class as the 0.44.1 `CaseUsage` fix. Regression coverage:
  `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_classifies_concern_def_separately_from_concern_usage`.

## [0.44.3] - 2026-07-17

- **Nested `use case`/`analysis`/`verification` def and usage support in `part def` bodies.**
  `PartDefBodyElement::UseCaseDef`/`::UseCaseUsage`, `::AnalysisCaseDef`/`::AnalysisCaseUsage`,
  and `::VerificationCaseDef`/`::VerificationCaseUsage` were already parsed correctly
  (`sysml-v2-parser`'s `part_def_body_element`), and the package-level
  `PackageBodyElement` counterparts already dispatched to the same materializers with full
  body-walking and typing-edge resolution -- but `graph_builder/part_def.rs`'s `PDBE` match had
  no arm at all for any of these six variants, unlike the sibling `PDBE::CalcUsage`/`PDBE::CaseDef`
  arms. All six fell to the `_ => {}` catch-all and were silently dropped from the graph when
  nested inside a `part def { ... }` body -- the exact same bug class already fixed once for
  `case`/`case def` in 0.44.1. Fixed by adding the six missing dispatch arms, reusing the existing
  `materialize_use_case_def`/`_usage`, `materialize_analysis_case_def`/`_usage`, and
  `materialize_verification_case_def`/`_usage` builders the package-level dispatch already calls
  (widened from `pub(super)` to `pub(crate)`, same as the `case` builders were in 0.44.1). No
  `PROJECTION_SCHEMA_VERSION` bump: this is dispatch-coverage only, not a `HostSemanticProjection`
  shape change. Regression coverage: `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_materializes_use_case_analysis_and_verification_nested_in_part_def`.

## [0.44.2] - 2026-07-17

- **`ConstraintUsage` classification, package-level.** `sysml-v2-parser` 0.40.0 added a distinct
  `ConstraintUsage` AST node (previously bare `constraint c : X;` folded into `ConstraintDef` at
  parse time, since no usage-side node existed to classify it separately). `ElementKind` already
  had a `"constraint"` string arm (`ElementKind::Constraint`) from an earlier round, but nothing
  ever constructed it -- `graph_builder/calc_constraint_def.rs` gains `build_constraint_usage`
  (materializing kind-string `"constraint"`, mirroring `build_constraint_def`'s metadata
  extraction over the shared `ConstraintDefBody` type, plus a typing edge when `type_name` is
  present), dispatched from `PackageBodyElement::ConstraintUsage` in
  `graph_builder/package_body/mod.rs`. No `PROJECTION_SCHEMA_VERSION` bump: this adds
  graph-builder dispatch coverage and consumes a new (but backward-compatible) parser AST node,
  not a `HostSemanticProjection` shape change. Regression coverage:
  `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_materializes_bare_constraint_usage_and_resolves_its_typing`, covering both the simple
  typed form and the real `Systems Library/Constraints.sysml` `constraintChecks` shape (`abstract`
  + typing + trailing multiplicity + `nonunique` + subsetting, all `def`-less).
  Nested-in-another-body `constraint` members (e.g. `Requirements.sysml`'s
  `RequirementConstraintCheck::assumptions`) remain out of scope -- the parser change only covers
  package-level dispatch; see `sysml-v2-parser` CHANGELOG 0.40.0 for the parser-side detail and
  `babel42-v2`'s `spec42-systems-modeling-api-gaps.md` for the API-level tracking.

## [0.44.1] - 2026-07-17

- **`CaseUsage` classification and nested `case`/`case def` support (S42-003 follow-up).**
  `ElementKind` had a `"case def"` string arm but no `"case"` arm, so a bare `case c : X;`
  usage — even at package level, where it already materialized — fell to
  `ElementKind::Unknown("case")` and could never become a concrete API resource. Added
  `ElementKind::Case` plus its `Display`/`FromStr` arms
  (`crates/sysml_model/src/semantic/model.rs`). Separately, `case`/`case def` nested inside a
  `part def { ... }` body had no dispatch arm at all in
  `crates/sysml_model/src/semantic/graph_builder/part_def.rs` (unlike the sibling
  `PartDefBodyElement::CalcUsage` arm), so it was silently dropped from the graph entirely at
  both the definition and usage level — not merely misclassified. Fixed by adding
  `PartDefBodyElement::CaseDef`/`::CaseUsage` dispatch arms that reuse the existing
  `materialize_case_def`/`materialize_case_usage` builders package-level dispatch already
  calls, widening those two functions from `pub(super)` to `pub(crate)` so the sibling
  `graph_builder::part_def` module can reach them through `package_body`'s existing
  `pub(crate) use materialize::*` re-export instead of duplicating the body-walking logic.
  No `PROJECTION_SCHEMA_VERSION` bump: this changes graph-builder dispatch coverage and
  `ElementKind` classification, not `HostSemanticProjection`'s shape. Regression coverage:
  `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_materializes_case_def_and_case_usage_nested_in_part_def`.

## [0.44.0] - 2026-07-17

- **`TransitionFeatureMembership` baseline (S42-003)** — a `transition` statement's `accept`
  trigger, `if` guard, and `do` effect now materialize as real, addressable child graph nodes
  (`ElementKind::TransitionTrigger`/`TransitionGuard`/`TransitionEffect`) owned via a new
  `HostMembershipKind::TransitionFeatureMembership`, mirroring how every other owned node gets a
  `Membership` automatically via `membership_kind()` (`crates/workspace/src/snapshot/facts.rs`).
  Trigger and effect use a simplified/uniform attribute representation this round (reusing the
  same rendering already used on the parent `transition` node), not full
  `AcceptActionUsage`/`ActionUsage` typing (deferred). Guard is different: its content is the
  real `Node<Expression>` from the parser AST, so it's projected losslessly via the existing
  `ast_util::declared_expression()` converter (unmodified) into a genuine addressable
  `Expression`, referenced from the new `HostElementFacts::content_expression_id` field
  (`crates/sysml_model/src/semantic/graph_builder/state.rs`,
  `crates/workspace/src/snapshot/facts.rs`).
  **Design note — a `FeatureValue`-reuse approach was considered and rejected:** an earlier draft
  planned to attach guard/trigger/effect content by reusing `HostFeatureValue`
  (`DeclaredFeatureValueKind`) to avoid a schema bump, but `HostFeatureValue` normatively means
  "this feature's value is X" (a KerML value-assignment) — none of guard/trigger/effect are
  value-assignments, so that would have shipped an incorrect `@type: FeatureValue` label on the
  consuming API. `content_expression_id` instead represents "this element's substance is X",
  a distinct, correctly-scoped concept.
- **Fix: the `transition` node's own membership was `OwningMembership`, should be
  `FeatureMembership`** — `ElementKind::Transition` was missing from `membership_kind()`'s
  `FeatureMembership` arm (documented as a known gap in 0.43.2). Fixed alongside the above since
  both required touching the same function.
- All existing flattened string attributes on the `transition` node (`source`, `target`,
  `guardExpression`, `conditionIsBoolean`, `exprClass`, `effectExpression`, `actionKind`,
  `payloadType`, `acceptType`, `acceptName`, …) are unchanged — purely additive; zero changes
  required in `behavior_conformance.rs` or `state_views/graph_extractor.rs`.
- **`PROJECTION_SCHEMA_VERSION` bump: 10 → 11.** Unlike 0.43.1/0.43.2's pure enum widening (no
  bump needed, since `Option<HostMembershipKind>` and `HostFeatureValue.kind: String` were
  already open wire fields), this release adds a genuinely new field
  (`HostElementFacts::content_expression_id`) to a core wire struct — closer in kind to the
  `feature_properties: Option<HostFeatureProperties>` addition that bumped v5→v6, so bumped for
  consistency and to force reprojection of cached artifacts.
  **Deferred, not in scope:** `transitionLinkSource`/`payload` `ParameterMembership`s, `Succession`
  as an addressable owned `Feature`, `BindingConnector`s, and full `AcceptActionUsage`/
  `ActionUsage` typing of trigger/effect remain open — see spec42-systems-modeling-api-gaps.md
  (babel42-v2) S42-003.

## [0.43.2] - 2026-07-17

- **Specialized memberships: Parameter/ViewRendering (S42-003)** — `HostMembershipKind` gains
  `ParameterMembership` and `ViewRenderingMembership` variants. `view rendering` member nodes
  previously fell into the generic `FeatureMembership` bucket. `membership_kind()`
  (`crates/workspace/src/snapshot/facts.rs`) now also takes the owner's `ElementKind`:
  `InOutDecl` (`in`/`out`/`inout`) is shared grammar between genuine Behavior parameters
  (Action/Calc definition and usage bodies — KerML 8.3.19.2 `ParameterMembership`) and Port/
  PortDef directed features, which reuse the same production but are ordinary
  `FeatureMembership`, not parameters. Both previously defaulted to `OwningMembership` since
  `InOutParameter` wasn't dispatched at all. No `PROJECTION_SCHEMA_VERSION` bump: this only
  changes which value some node kinds receive, not the wire shape.
  **Known remaining gap:** `TransitionFeatureMembership` (trigger/guard/effect of a
  `TransitionUsage`) is not yet implementable — Spec42 flattens a transition's guard/effect into
  string attributes (`guardExpression`/`effectExpression`) on the `transition` node rather than
  materializing them as addressable child features, so there is no node to classify as
  `TransitionFeatureMembership` yet. The `transition` node's own membership within its owning
  state is also still unclassified (falls to `OwningMembership`, should be `FeatureMembership`)
  and is deliberately left alone in this release rather than mixed in with the correct fix.

## [0.43.1] - 2026-07-16

- **Specialized memberships: Subject/Stakeholder/Objective (S42-003)** — `HostMembershipKind`
  gains `SubjectMembership`, `StakeholderMembership`, and `ObjectiveMembership` variants,
  alongside the existing `ActorMembership`/`VariantMembership`. `subject`/`stakeholder` member
  nodes previously fell into the generic `FeatureMembership` bucket in `membership_kind()`
  (`crates/workspace/src/snapshot/facts.rs`); `objective` member nodes weren't handled at all and
  silently defaulted to `OwningMembership` — a real misclassification, not a deliberate choice.
  No `PROJECTION_SCHEMA_VERSION` bump: `membership_kind: Option<HostMembershipKind>` already
  exists on the wire shape, this only changes which value three node kinds receive.

## [0.43.0] - 2026-07-16

- **Enumeration vertical slice** — requires `sysml-v2-parser` **0.39.0**, which gives each
  enumerated value inside `enum def { ... }` a real spanned AST node (`EnumeratedValue`) instead
  of a bare `String`.
  - New `ElementKind::EnumeratedValue` (`"enumerated value"`) — `materialize_enum_def` now walks
    `EnumerationBody::Brace { values }` and materializes each enumerated value as its own
    addressable child node, owned by the enclosing `EnumDef`. Initializer expressions (`= expr`)
    and inline bodies (`active { ... }`) remain discarded; only name + span are retained, matching
    the parser's own scope for this construct.
  - New `ElementKind::Enumeration` (`"enumeration"`) — the kind-string an `enum status : Status;`
    usage node was already materialized with by `graph_builder/part_def.rs`'s nested-in-`part def`
    handler, but which had no `ElementKind::parse` mapping (fell to `Unknown("enumeration")`) and
    no package-level dispatch at all (`PackageBodyElement::EnumerationUsage` was a no-op).
    Package-level `enum status : Status;` now materializes and resolves its typing edge the same
    way the nested form already did (`EnumDef` was already a valid typing target).
  - No `PROJECTION_SCHEMA_VERSION` bump: `ElementKind` serializes as a plain string
    (`#[serde(into = "String", from = "String")]`), so new kind variants are additive at the wire
    level, same as the Requirements/Behavior slice's new definition/usage kinds in 0.42.0.

## [0.42.1] - 2026-07-16

- **Fix: `ConstraintDef`/`CalcDef`/`CaseDef` typing resolution** — added the three to
  `TYPING_TARGET_KINDS` (`crates/sysml_model/src/semantic/kinds.rs`), matching what
  `SPECIALIZES_TARGET_KINDS` already allowed. A nested `calc`/`case` usage's Typing edge to its
  definition previously never resolved.
- **Fix: `materialize_case_usage` never wired a typing edge** — unlike its sibling
  analysis/verification/use-case usage builders, it never called `add_typing_edge_if_exists` even
  though `CaseUsage.type_name` is captured by the parser. Fixed alongside the allowlist change
  above, since the allowlist fix alone wasn't sufficient to make `case` usage typing resolve.
- No `PROJECTION_SCHEMA_VERSION` bump — both fixes only improve resolution fidelity of an
  already-projected fact; `HostSemanticProjection`'s shape is unchanged.

## [0.42.0] - 2026-07-16

- **Host projection schema v10** — `Satisfy` and `Subject` relationships classify as
  their own `HostRelationshipMetaclass` instead of the generic `Relationship` fallback.
  Both were already real, resolved graph edges; only the metaclass classification was
  missing.
- **Bug fix: `case`/`case def` bodies were silently dropped** — `materialize_case_def`/
  `materialize_case_usage` created the case node but never walked its body, unlike the
  sibling `use_case`/`analysis_case`/`verification_case` builders. `subject`/`actor`/
  `objective`/`include` members inside a `case`/`case def` are now materialized the same
  way they already were for `use case`/`analysis`/`verification case`.

## [0.41.0] - 2026-07-16

- **Host projection schema v9** — Addressable `HostConnectorEnd` facts for resolved
  `Connection`/`Interface` connect statements, and a `FlowStatementDetail` (payload,
  source/target expression text) plus a distinct `SuccessionFlow` relationship kind for
  `flow`/`succession flow` usages. Requires `sysml-v2-parser` **0.38.0**, which fixes a
  parser bug (PAR-007) where a package-level `connection`/`interface` usage typed with an
  inline `connect ... to ...` clause was misclassified as a definition with the clause
  silently discarded.
- **Connector ends** — `HostSemanticProjection.connector_ends` projects the binary
  `from`/`to` ends of a resolved `connect` statement as addressable, explicitly-ordered
  facts (`end_index` 0/1), derived from the relationship's already-resolved
  `source_id`/`target_id`. N-ary `connect (a, b, c, ...)` ends beyond the binary pair are
  parsed (`ConnectStmt::extra_ends`) but not yet resolved to feature IDs anywhere in the
  graph builder, so they are not projected yet — tracked as a follow-up.
- **Flow detail** — `HostSemanticModelRelationship.flow` carries payload and source/target
  expression text for `Flow`/`SuccessionFlow` edges, mirroring `Connection`'s existing
  `connect` field. `RelationshipKind::SuccessionFlow` distinguishes `succession flow` from
  plain `flow`/`message` (previously all three collapsed into one generic `Flow` edge kind).

## [0.40.0] - 2026-07-16

- **Host projection schema v8** — Addressable Documentation elements with Annotation
  edges, Import/Alias as NamespaceImport / MembershipImport / AliasMembership
  relationships (with visibility), first-class `element_type` for ReferenceUsage,
  and AttributeUsage multiplicity projection. Requires `sysml-v2-parser` **0.37.0**.
- **Documentation** — `doc /* … */` materializes as `ElementKind::Documentation`
  children while keeping `HostElementFacts.documentation` text on the annotated element.
- **Import / Alias relationships** — Containment memberships for import/alias nodes
  use dedicated metaclasses instead of generic Membership.
- **ReferenceUsage** — `ElementKind::Ref` projects `facts.element_type = "ReferenceUsage"`.
- **AttributeUsage multiplicity** — Structured multiplicity ranges from parser 0.37
  project as addressable `HostMultiplicity` facts.

## [0.39.0] - 2026-07-16

- **Host projection schema v7** — Extends v6 feature properties with composite/reference
  ownership and conjugation, adds documentation/short-name facts, library-element flagging,
  richer membership kinds, and relationship metaclasses for Subsetting, Redefinition,
  Subclassification, and Annotation.
- **RefDecl ownership** — Package-level and nested `ref` declarations materialize as
  `ElementKind::Ref` with `is_reference=true` / `is_composite=false` and retain typed
  feature values when present. Ordinary part/item/port/attribute usages default to
  composite ownership.
- **Membership taxonomy** — Containment memberships distinguish FeatureMembership (usages),
  OwningMembership (definitions/packages), plus Import, Alias, VariantMembership, and
  ActorMembership.
- **Relationship family** — Graph edges for `subsets`/`redefines` resolve to Subsetting /
  Redefinition; def `specializes` projects as Subclassification; Annotation edges keep an
  Annotation metaclass.
- **Library / conjugation / names** — `is_library_element` uses the library URI set;
  conjugated port typing (`~Type`) sets `is_conjugated`; `doc` and `shortName` attributes
  lift into typed `HostElementFacts` fields.

## [0.38.0] - 2026-07-16

- **Host projection schema v6** — Projected elements now carry typed
  `feature_properties` on `HostElementFacts` for explicitly declared modifiers:
  direction, abstract/variation, individual, derived, constant, end, ordered,
  and unique. These facts come from the parser AST via `DeclaredFeatureProperties`
  and are separate from the legacy display attribute map.
- **Feature property materialization** — Part, attribute, port, and item usages,
  part definitions, attribute definitions, and directed port parameters retain
  declaration modifiers as semantic facts instead of dropping them after parse.

## [0.37.0] - 2026-07-16

- **Typed host projection v5** — Added API-oriented element facts for declared
  and effective names, ownership, owning membership, and library status. These
  facts are separate from the legacy display-oriented attribute map.
- **Addressable ownership** — Parent containment now records an explicit
  membership form (`OwningMembership` or `FeatureMembership`) and gives both
  the child and relationship the same stable membership identity.
- **Deterministic relationship identities** — Relationship IDs no longer depend
  on graph enumeration order. Explicit connection facts use their source range
  and endpoint expressions as an identity discriminator.

## [0.36.0] - 2026-07-16

- **Parser dependency** — Bumped `sysml-v2-parser` to **0.36.0** and aligned
  Spec42 with typed relationship targets, `Membership`, import visibility, and
  typed `FeatureValue` AST nodes.
- **Host projection** — Projects declared feature values and their structural
  expression trees as addressable `HostFeatureValue` and `HostExpression`
  facts, preserving bind/default/initial semantics without parser debug text.

## [0.35.0] - 2026-07-08

### Added

- **`sysml-v2-parser` upgraded 0.28.0 → 0.32.0**, unlocking new SysML v2 syntax and fixing several evaluation/diagnostic gaps along the way:
  - **0.29.0**: state `do`/`exit` actions, `decide`/`join`/`fork` control nodes, negated `assert`, structured `Transition.effect` (enum, not raw string), structured `AssignStmt.lhs`/`ForLoop.range`. `ElementKind::Decide`/`Join`/`Fork` added; `graph_builder::action` materializes them like `merge` already was; `graph_builder::state` now handles `StateDefBodyElement::Do`/`::Exit` (previously only `Entry`); `AssertConstraintMember.is_negated` captured as an `isNegated` attribute. Deliberately not wired up: `Satisfy.is_negated` and `ConnectStmt.extra_ends` (n-ary connect) — both additive fields that default safely, so existing behavior is unaffected.
  - **0.31.0**: connection ends redefined via `::>` pointing at a nested feature path (not a type name) no longer false-flag an unresolved-type-reference diagnostic (`ElementKind::InterfaceEnd` with a dotted `type_ref` is now recognized and skipped). Reusable requirement/analysis/verification/use-case templates (`is_parametric_definition`) no longer incorrectly report "incomplete" for unassigned values while still surfacing real `failed_constraint` violations. Boolean-equality expressions (`s.flag == true`) inside `require constraint` bodies now evaluate correctly instead of falling through to the numeric/quantity comparison path.
  - **0.32.0**: `variant` members inside a `variation part def` body (`variant part manual : ManualTransmission;`, and the `attribute`/`item`/`port` equivalents) now materialize with their full declared-type node shape (attributes, typing edge, body recursion) via new `materialize_item_usage`/`materialize_variant_usage`, instead of always collapsing to an untyped stub. Requirement usages now carry an `isAbstract` attribute. Expanded reserved-keyword hover coverage.
- **VS Code: `sysml.showVisualizer`/`sysml.showVisualizerActive` editor-title commands** — an icon button in the editor tab toolbar (for `.sysml`/`.kerml` files) that reveals the SysML Visualizer panel, addressing low discoverability since the visualizer moved to the secondary sidebar in 0.34.0. The icon dims to signal the panel is already open, tracked via a `webviewView.onDidChangeVisibility` listener (previously the `sysml.visualizerOpen` context key only updated on first-resolve/dispose, so it went stale after switching tabs without closing the panel).
- **`workspace_session` crate** — actor-model concurrency wrapper (`SessionActor` + lock-free `SnapshotHandle` reads) for embedder-owned session state, enabling non-blocking reads while background work runs.
- **Incremental Workspace Engine** (`crates/workspace/src/incremental.rs`) — standalone engine supporting full-load and incremental-patch graph operations, with `WorkspaceUpdateMetrics` for perf visibility; `build_view_catalog`/`render_view` render a single view without building a full snapshot; `validate_workspace` computes diagnostics directly without one either.

### Changed

- **LSP responsiveness** — hover/completion requests no longer block on in-flight background relink or startup indexing, via the new `workspace_session` actor and a synchronous `WorkspaceSession` state machine (`SessionLifecycle`/`RelinkToken`) in the `workspace` crate. `SemanticCoordinator` (`lsp_server`) has been removed entirely, its responsibilities folded into `WorkspaceSession` (~657 lines removed).
- **Parallel semantic graph building** — workspace and library documents are now parsed/linked in parallel via `rayon` (`build_and_link_graph_parallel`); `lsp_server`'s worker dispatch also moved off hand-rolled `std::thread::spawn` onto rayon, which additionally fixes worker-thread panics that were previously silently swallowed.
- **Semantic graph builder consistency audit** — centralized `part`/`attribute`/`occurrence`/`requirement` usage materialization (previously reimplemented independently per containing context, causing attribute/subtree drift depending on where a usage appeared); specialization edges now wired before body recursion (fixes inherited-member resolution ordering); spec-correct effective-name resolution for unnamed redefining usages (SysML v2 §7.6.5); `package_body.rs`'s dispatcher split into 34 named functions.
- **`ElementKind` type-safety migration** — string-based element-kind/type checks replaced with the `ElementKind` enum across `sysml_model` and `language_service` (completion, import resolution, diagnostics, hover, references); `ElementKind::is_definition()` added as the canonical, compiler-checked replacement for scattered `.ends_with(" def")`/`.contains("_def")` string-suffix heuristics.
- **General-view layout** — ELK graph now nests members under real per-package containers (`elk.hierarchyHandling: INCLUDE_CHILDREN`) instead of laying out every node in one flat tree, and spacing constants were tightened; diagrams with real package structure are meaningfully narrower and less tangled. A single package with many same-depth siblings can still lay out as one very wide row (tracked as a known issue, see below).
- **ELK layout engine version pinned** across the CLI/headless renderer and the VS Code webview build — the webview's `node_modules` had silently drifted to elkjs 0.8.2 while the CLI used the intended 0.11.1; both now resolve the same pinned version with a build-time assertion that fails loudly on drift.
- **Internal view-pipeline refactor (8 phases)** — general-view/interconnection-view/behavior-view rendering code reorganized (ELK option unification, DTO builder dedup, duplicate Rust "prepared view" preparers deleted for 6 of 7 view kinds, exposed-id filter dedup, dead-code deletion); no intended behavior change, verified via before/after CLI export diffing across all 8 baseline view kinds.
- **VS Code settings cleanup** — Removed eight display-only / legacy configuration properties (`spec42.standardLibrary.*`, `spec42.domainLibraries.*`) that were not user-configurable; removed `spec42.codeLens.enabled` (code lens disabled for now) and `spec42.startup.workspaceIndexing` (only `background` mode is tested); fixed encoding corruption in `spec42.debug` description. Sync scripts updated accordingly.

### Fixed

- **Interconnection View: several real connector/rendering bugs found and fixed against a real-world workspace** (`sysml-robot-vacuum-cleaner`):
  - Bare own-port connector endpoints (e.g. `connect leftMotor.phaseIn to phaseLeftIn;`) were silently dropped — a 2-segment relative member chain was indistinguishable from an already-qualified reference and went unprefixed, so the whole connector failed to match.
  - A view exposing a definition-nested member (declared directly inside a `part def` body) resolved **zero connectors** whenever the definition and instance paths used independently-named top-level packages — the def→instance path translation only recognized a narrow `Architecture`/`architecture` naming convention.
  - Sibling subtypes sharing a common ancestor could cross-contaminate def→instance path mappings when a `part def` (not a usage) leaked into the mapping seed, producing orphan nodes and empty parent containers.
  - A package that only ever contains other packages/definitions (e.g. a top-level `PhysicalArchitecture` wrapping a `part def`) rendered as an empty, disconnected box — two independent container-building mechanisms never linked a root-level definition container to its real owning package container.
  - Click-to-source did nothing — scene/prepared-view DTOs weren't carrying `uri`/`range` through to the final payload the renderer consumes, despite the upstream data having them.
  - Node bodies weren't showing multiplicity, port direction, or `redefines`/`subsets` annotations even though the data existed.
- **General-view node bodies rendered completely empty** (header-only, no Attributes/Parts/Ports compartments) whenever the view had a kind-narrowing `filter` clause (e.g. `filter @SysML::PartUsage;`) — the filter ran before compartment-folding needed its input nodes.
- **Duplicate edges during graph linking** — `add_semantic_edge_once` now deduplicates edges that could previously be added more than once via `rebuild_all_document_links`/`rebuild_semantic_graph_staged`.
- **Stale analysis evaluation context after a full workspace load** — inherited context wasn't being reapplied before expression evaluation on some code paths; fixed via `prepare_analysis_evaluation_context`.
- **Evaluated attributes not computed on some incremental-update paths** — fixed via new `finalize_and_evaluate`/`patch_graph_for_document`, replacing `finalize_workspace_graph`.
- **Windows drive-letter normalization** — `workspace::snapshot::discovery::path_to_file_url` now lowercases the drive letter in `file://` URIs (`file:///C:/…` → `file:///c:/…`), matching the normalization applied by `FileSystemDocumentProvider`; previously target URIs and document URIs could differ in case, causing `project_host_semantic_model` to return an empty projection on Windows.
- **`HostSemanticProjection` excludes diagnostic pseudo-nodes** — `ElementKind::Diagnostic` nodes (internal builder markers for unresolvable connect/allocate endpoints) are now filtered out of `HostSemanticProjection`; they are already emitted as first-class `SemanticDiagnostic` entries via `HostValidationReport`.
- **Serde defaults for `HostSemanticModelNode.attributes` and `HostSemanticModelRelationship.connect`** — Added `#[serde(default)]` so older persisted projection artifacts without these fields deserialize correctly instead of failing with a missing-field error.
- **Backward-compatible deserialization for renamed element and relationship kinds** — `ElementKind::parse` now accepts `"requirement usage"` (→ `Requirement`) and `"verification case"` (→ `Verification`); `RelationshipKind::Subject` accepts `"verification"` via `#[serde(alias)]` and `from_persisted_type`; canonical serialization is unchanged.
- **Document URI normalization at ingestion** — `workspace::snapshot::build::enrich_document_hashes` now normalizes each document's URI (drive-letter case) via `language_service::uri::normalize_uri` before the semantic graph is built, complementing the existing `path_to_file_url` canonicalization. Previously the graph itself could be indexed under a differently-cased URI than the canonicalized `target_urls`, causing `project_host_semantic_model` and `collect_host_validation_report` (parse-error diagnostics) to silently return empty results.
- **`interface def` no longer collapses into the same kind as `interface` usage** — added `ElementKind::InterfaceDef`; `interface def Foo { ... }` now produces `ElementKind::InterfaceDef` nodes instead of being mislabeled as the bare `interface` usage kind, matching the part/port/action def-vs-usage pattern. Typing/specializes/subject allowlists in `kinds.rs` updated accordingly.
- **`entry` actions in state bodies no longer become `ElementKind::Unknown`** — `graph_builder::state` now builds entry-action nodes as `ElementKind::Action` (with a `compartment: "entry"` attribute) instead of the unrecognized literal `"entry"`; `state_views::graph_extractor::compartment_action` updated to match on the new attribute.
- **`ElementKind::ActorDef` removed (was never valid SysML v2 syntax)** — the official grammar only defines a bare `actor Foo;` usage (`ActorUsage : PartUsage`), no `actor def` definition form exists. `graph_builder::package_body`'s `PBE::Actor` handler was mislabeling this usage as `"actor def"`; it now builds `ElementKind::Actor` like the equivalent handling elsewhere (`use_case.rs`, `requirement_body.rs`).
- **Flaky diagnostics test removed** — `did_change_republishs_peer_diagnostics_after_debounce` deleted; peer-file diagnostic republish timing is non-deterministic under CI load and the behavior is covered by workspace integration tests.

### Known Issues

- ~~**Interconnection-view and action-flow-view layout is non-deterministic run-to-run**~~ — fixed in `[Unreleased]`, see the O-6 entry above.
- **Scoped/incremental IBD builds can resolve a different (but still valid) root than a full-workspace build** for some views in workspaces that use SysML variant-selection — a parity gap, not a correctness bug, but scoped and full-workspace exports of the same view can disagree.

## [0.34.0] - 2026-06-30

### Added

- **SysML v2 Quick Reference panel** — New Help section in the Spec42 sidebar with a "SysML v2 Quick Reference" entry that opens a rich editor tab covering definitions, usages, relationship symbols, annotations, and views — verified against the OMG SysML v2 Language Specification.
- **Spec42 documentation site** — VitePress site published at `https://elan8.github.io/spec42/` via GitHub Pages, with getting-started guide, example walkthroughs, visualizer reference, library guide, and SysML v2 quick reference. Automated deployment via `.github/workflows/docs.yml`.
- **Help sidebar panel** — Sidebar section with quick actions: open visualizer, open recommended example, open quick reference, link to Spec42 docs, link to OMG SysML v2 spec.
- **Drone interconnection performance smoke report** — Nightly CI enforces latency budgets against the in-repo drone example (`drone_interconnection_performance_smoke_report`).

### Changed

- **Visualizer moved to secondary sidebar** — SysML Visualizer is now a persistent sidebar panel (`WebviewViewProvider`) in the secondary sidebar instead of a transient editor panel; removed the editor-title toolbar button.
- **Semantic token accuracy** — Improved token classification for state definitions, flow definitions, port definitions, action definitions, and part definitions; new golden-token regression suite; better declaration-range resolution in `ast_util.rs`.
- **LSP document processing** — New `workspace/coordinator.rs`; cleaner semantic lifecycle management with explicit `SemanticLifecycle` states; improved relink scheduling and reduced redundant re-processing.
- **Library graph handling** — Enhanced caching and concurrency in LSP server library graph loading.
- **Diagram legend accuracy** — Specialization edge now shows a correct hollow triangle; containment shows a proper diamond; removed the incorrect "Hierarchy" entry.
- **Nightly CI cleanup** — `large-workspace-performance` job (tiny fixture, no budget signal) replaced by `performance` (drone interconnection, blocking); `interconnection-visualization-performance` job (always skipped without external fixture) removed; stdlib bundle caching added to `sysml-release-validation` job.
- **Library sidebar** — Simplified styling and layout of the library panel.
- **Release surface alignment** — Rust workspace, `spec42` server, VS Code extension, Zed extension, and GitHub Action examples aligned at `0.34.0`.

### Fixed

- **Visualizer open state tracking** — `visualizerOpen` in the extension debug state now correctly reflects whether the visualizer view has been resolved, rather than always returning `true` after activation.
- **`BaseVisualizationPanelController.dispose()`** — Restore state is now cleared on dispose in all code paths (previously only cleared when going through `VisualizationPanel.dispose()`).
- **Flaky diagnostics test** — `did_change_republishs_peer_diagnostics_after_debounce` now uses barrier-based polling with a 5-second deadline instead of a fixed `sleep(250ms)`, eliminating race conditions on loaded CI runners.

## [0.33.0] - 2026-06-29

### Added

- **`workspace` crate** — Protocol-neutral embedding for workspace build, snapshot loading, semantic comparison, incremental updates, cancellation, and resource limits (ADR 0003).
- **Semantic comparison** — `compare_snapshots` returns a versioned `SemanticComparisonReport` for element, relationship, diagnostic, and view catalog diffs with identity-preservation assessment.
- **Artifact metadata** — `HostArtifactMetadata` records schema versions, engine version, library catalog hash, and per-document source hashes.
- **TypeScript bindings (`sysml_model`)** — `ts-rs` integration exports shared DTOs (including unified `Position` and `Range`) for the diagram renderer and extension surfaces.
- **Component view module** — Part and port expansion for semantic graph queries.
- **Library parse caching** — LSP server caches library graph parses across workspace sessions.
- **`WorkspaceRenderCache`** — LSP visualization warm-cache integration for `sysml/visualization` requests.
- **Incremental snapshot updates** — Experimental incremental workspace updates with parity, fallback, and benchmark tests.
- **Robot vacuum showcase** — Integration tests and performance analysis documentation for the release-gating fixture.
- **Performance guardrails** — Nightly budget enforcement via `scripts/check-perf-budgets.mjs` on large-workspace and drone-interconnection fixtures.
- **`edges_for_uri`** — Semantic graph query method for URI-scoped edge lookup with improved serialization.
- **Parent-child relationship handling** — Semantic graph evaluation now models parent-child relationships explicitly.

### Changed

- **Crate restructuring** — `semantic_core` renamed to `sysml_model`, `kernel` renamed to `lsp_server`, and embedding concerns extracted into the new `workspace` crate.
- **Parser dependency** — Bumped `sysml-v2-parser` to **0.28.0** on [crates.io](https://crates.io/crates/sysml-v2-parser); graph builders updated for new AST body/member enums.
- **DTO unification** — Neutral `TextPosition`/`TextRange` span types and unified `Position`/`Range` DTOs across `sysml_model`.
- **LSP server** — Refactored document handling for improved concurrency; library graph caching; streamlined diagnostics and visualization data paths.
- **Semantic graph** — `Arc`-based memory management, query performance improvements, and stale-cache invalidation fixes.
- **IBD payload merging** — Refactored merge path with enhanced workspace instance mapping.
- **Element kind handling** — Refactored predicates for improved type safety and consistency.
- **CI workflows** — Updated for crate renaming; diagram renderer performance tracking in CI.
- **Documentation** — Engineering docs, Sequence View notes, and architecture references updated for current crate names.
- **Release surface alignment** — Rust workspace, `spec42` server, VS Code extension, Zed extension, and GitHub Action examples aligned at `0.33.0`.

### Fixed

- **Stale semantic graph cache** — Cache invalidation after graph mutations restores correct query results.
- **LSP integration test** — Diagnostic handling for loose-file workspaces.
- **MCP tools test** — Additional error-message assertion for tool failure paths.

## [0.32.0] - 2026-06-22

### Added

- **`language_service` crate** — Shared language-service layer extracted from the kernel; integrated into LSP runtime and core tests.
- **`preparedView` pipeline** — Rust builds render-ready `preparedView` DTOs for interconnection and standard views; LSP can emit slim interconnection payloads that omit duplicate `ibd`, `graph`, and `interconnectionScene` fields when `preparedView` is present.
- **Headless SVG export** — Shared `@spec42/diagram-renderer` headless path for CLI/API diagram export and VS Code export smoke tests, aligned with the webview renderer.
- **Standard views** — Browser, grid, and geometry view preparers in `semantic_core` and the shared diagram renderer.
- **Projection hints** — View projection hints and rendering resolution for defined SysML views; standard-view defaults and swim-lane handling in activity diagrams.
- **IBD build scoping** — `ViewExposedPackages` scope for interconnection builds reduces work on large workspaces; scoped-vs-full parity and performance smoke tests.
- **Legacy ELK SVG (test-only)** — Rust `legacy_elk_svg` module retains `interconnectionScene` ELK parity probes; production export uses headless `preparedView`.
- **Diagram export quality analysis** — Engineering notes and integration coverage for export/readability invariants.

### Changed

- **Interconnection scene schema version 2** — Canonical interconnection scenes and prepared views use schema version 2; integration tests poll `preparedView` instead of legacy `interconnectionScene`/`ibd` counts.
- **Visualization workspace cache** — Model Explorer and visualizer share workspace render snapshots; slim `preparedView` interconnection responses are cached on warm `sysml/visualization` requests.
- **Parser dependency** — Bumped `sysml-v2-parser` to **0.25.6** on [crates.io](https://crates.io/crates/sysml-v2-parser).
- **Library closure resolution** — Further hardening of import-scoped library loading and workspace-wins merge behavior.
- **CI and agent surfaces** — Core vs agent/API/MCP test split in CI; ignored integration tests documented for MCP/API workflows.
- **Release surface alignment** — Rust workspace, `spec42` server, VS Code extension, Zed extension, and GitHub Action examples aligned at `0.32.0`.

### Fixed

- **Slim interconnection cache** — Visualization response cache accepts `preparedView`-only interconnection payloads (restores warm-cache behavior after slim-payload refactor).
- **IBD unit tests** — Correct `extract_impl` test imports for `merge_ibd_payloads` and related IBD helpers after module layout changes.
- **Diagram renderer types** — TypeScript `PreparedView` passthrough casts in shared prepare/headless export paths.

## [0.31.0] - 2026-06-15

### Added

- **KPAR crate (`crates/kpar`)** — Read, validate, and materialize KerML Project Archives (`.project.json`, `.meta.json`, SHA-256 checksums, textual `.sysml`/`.kerml` sources). The top-level `spec42 bundle` command creates local archives, while `extract_archive_subset` lives in `legacy.rs` for zip subset helpers used in tests.
- **Domain libraries via KPAR** — Bundled domain libraries ship as `elan8-domain-libraries-{version}.kpar`. `config/domain-libraries.json` pins `format`, `version`, and release `artifact`. Runtime materializes the embedded KPAR on first use; managed install path no longer uses a `tree/` subdirectory when `contentPath` is empty.
- **Standard library via OMG KPAR** — Bundled stdlib embeds the OMG `sysml.library.kpar` archives from the pinned SysML v2 Release tag. Multiple KPAR subroots are materialized and mounted for semantic indexing (`stdlib_roots` in environment resolution).
- **Domain library release automation** — GitHub Action on [elan8/sysml-domain-libraries](https://github.com/elan8/sysml-domain-libraries) (`release-kpar.yml`) builds and publishes the KPAR asset plus `SHA256SUMS.txt` on `v*` tags.
- **KPAR embed smoke test** — `crates/server/tests/kpar_stdlib_embed_smoke.rs` verifies embedded OMG KPAR stdlib materialization and `ScalarValues::Real` import resolution via `spec42 check`.

### Changed

- **Library distribution format** — Standard and domain libraries use KPAR only. Tree-format zip extraction and repack fallbacks removed from `build.rs`, `stdlib.rs`, `domain_libraries.rs`, and fetch scripts.
- **Stdlib fetch (KPAR-only)** — `scripts/fetch-stdlib-bundle.sh` sparse-checkouts only `sysml.library.kpar/` from the pinned OMG release tag (not the full release zip and not `master`). All embed KPAR inputs live under repo-root `.cache/` (stdlib subdirectory + domain `.kpar` file).
- **Domain library fetch** — `scripts/fetch-domain-libraries-bundle.sh` downloads the release `.kpar` asset or packs locally from `SPEC42_DOMAIN_LIBRARIES_SOURCE_DIR` / sibling `../sysml-domain-libraries`.
- **Build embed pipeline** — `build.rs` embeds `bundled-sysml-kpar/*.kpar` for the OMG standard library and raw `.kpar` bytes for domain libraries. Maintainer override: `SPEC42_STDLIB_KPAR_DIR` (replaces `SPEC42_STDLIB_BUNDLE_ZIP`).
- **Library path resolution** — `spec42 doctor` and environment resolution expose multiple stdlib roots when using the KPAR distribution; VS Code library settings sync `format: kpar` from `config/*.json`.
- **CI and release workflows** — `ci.yml`, `release.yml`, and `nightly.yml` cache embed KPAR inputs under `.cache/` (`SPEC42_STDLIB_KPAR_DIR` and `SPEC42_DOMAIN_LIBRARIES_BUNDLE_ZIP`). OMG ratchet jobs in `nightly.yml` still clone the full `SysML-v2-Release` checkout separately.
- **Local build cache** — `.cache/` at the repository root is gitignored; KPAR build inputs are never committed.

### Removed

- **Tree-format library bundles** — No support for textual `sysml.library/` trees or git-archive zip repack as embed input. Installation from bytes requires KPAR archives.
- **`SPEC42_STDLIB_BUNDLE_ZIP`** — Replaced by `SPEC42_STDLIB_KPAR_DIR` pointing at a directory of OMG `.kpar` files.

### Added

- **Library import closure tests** — Kernel `validate_paths` integration test for webshop-like workspaces with duplicate domain-library packages; semantic_core tests for import-scoped closure, conditional unit catalogs, and workspace-wins graph merge.
- **Semantic resolution contract** — New [resolution-contract.md](crates/semantic_core/docs/resolution-contract.md) documenting name-resolution pipeline stages and resolver entry points; `kinds.rs` canonical element-kind predicates; refactored graph pipeline for materialization, merging, linking, and pending resolution.
- **Flow usage projection** — Dedicated `flow_usage` module for consistent flow edge emission across body elements.
- **Metadata OMG 14c compliance** — Shorthand metadata redefine features, `subsetsFeature` projection for redefine shorthand, and nightly OMG 14c metadata compliance check.
- **Import namespace resolution** — `import_namespace_target_candidates` for accurate all/membership import target generation.
- **Requirement satisfaction diagnostics** — Checks for requirement satisfaction by parts per SysML specifications.
- **Robot vacuum showcase test** — Integration test validating diagnostics on the showcase model.

### Changed

- **Parser dependency** — Bumped `sysml-v2-parser` to **0.25.4** on [crates.io](https://crates.io/crates/sysml-v2-parser); removed local patch configuration. Flow handling refactored through the new `flow_usage` module.
- **Verification case diagnostics** — Then-action return semantics aligned with SysML v2 (verification cases with then-actions and no explicit return are valid); removed outdated then-action count checks in requirement case conformance.
- **Semantic metadata graph building** — Improved parent ID handling for semantic metadata definitions in the graph builder.
- **Release surface alignment** — Rust workspace, `spec42` server, VS Code extension, Zed extension, and GitHub Action examples aligned at `0.31.0`.

### Fixed

- **Library import closure** — Library files enter the semantic graph only through transitive `import` closure; workspace-declared packages satisfy imports without loading same-named library copies (fixes ambiguous `view_expose_unresolved` and empty views when domain libraries include examples such as webshop). Unit catalogs (`ScalarValues`, `ISQ`, `QUDV`, etc.) load only when the closure needs them. Graph merge prefers workspace declarations when duplicate package names slip through (`spec42 check`, LSP, and filesystem provider).
- **Import resolution** — Enhanced namespace import target resolution for all and membership imports.
- **Cyclic state definitions** — Diagnostic checks for cyclic state definition graphs.
- **Verdict evaluations** — Improved verdict evaluation diagnostics.
- **Monetary units** — Correct resolution of monetary units from indexed catalogs during evaluation.

## [0.30.0] - 2026-06-12

### Added

- **Canonical interconnection pipeline** — Language server emits `interconnectionScene` (schema version 1) for Interconnection views; shared `@spec42/diagram-renderer` prepares and lays out scenes with ELK; VS Code and export paths consume the same pipeline.
- **IBD and interconnection scope** — Merged workspace IBD, scoped interconnection filtering, package container groups, and diagram export improvements for internal block diagrams.
- **Visualization performance** — Lazy single-view projection, workspace visualization artifact cache (shared by Model Explorer and visualizer), per-diagram response cache, and slimmer interconnection LSP payloads. Documented in [docs/engineering/POWER-SYSTEMS-PERFORMANCE-ANALYSIS.md](docs/engineering/POWER-SYSTEMS-PERFORMANCE-ANALYSIS.md) with `powersystems_system_context_performance_report` integration test.
- **Domain libraries bundled like stdlib** — Removed the `domain-libraries` git submodule. Elan8 domain libraries are embedded in the Spec42 server binary, materialized under the data directory on first use, and shown in the VS Code Library dashboard next to the standard library. Local dev auto-detects a sibling `../sysml-domain-libraries` checkout or uses `SPEC42_DOMAIN_LIBRARIES_SOURCE_DIR`.
- **Workspace lifecycle feedback** — Clear indexing/ready states for semantic queries; visualizer and Model Explorer respect lifecycle before building diagrams.
- **Analysis typing and evaluation** — Richer analysis usage typing, inherited analysis results, and conformance-matrix updates for metadata and quantities.
- **Diagnostics** — Engineering-units validation, SysML kind-compatibility checks, and improved reference resolution messaging.

### Changed

- **Parser dependency** — Bumped `sysml-v2-parser` to **0.25.2** on [crates.io](https://crates.io/crates/sysml-v2-parser): metadata in constraint bodies, structured recovery for state `ref` and part-usage bind/ref bodies, case-body `ref :>>` and verdict spans, expression classification, and related graph/diagnostic follow-through since 0.29.1.
- **Interconnection visualization** — Legacy interconnection webview paths removed; layout, routing, and drawing consolidated in the shared renderer.
- **LSP perf logging** — `backend:sysmlVisualizationRequest` includes `cacheHit`, `ibdMs`, `viewEvalMs`, and `sceneMs`.
- **Release surface alignment** — Rust workspace, `spec42` server, VS Code extension, Zed extension, and GitHub Action examples aligned at `0.30.0`.

### Fixed

- **Visualization response cache** — Do not cache or serve incomplete `interconnection-view` responses (avoids stale empty scenes in integration tests and after fast view switches).
- **Extension build** — Resolve `elkjs` when bundling `diagram-renderer` into the VS Code extension host.
- **Interconnection integration tests** — Poll until `interconnectionScene` is ready before asserting drone/grid diagram invariants.

## [0.29.1] - 2026-06-09

### Added

- **Graph depth P4b–P8 (semantic depth)** — RenderingDef and OccurrenceUsage body walks; shared case-body wiring (`SubjectRef`, `FirstSuccession`, `ThenUseCaseUsage`, `RefRedefinition`); `MetadataAnnotation` in action/requirement bodies; typed graph facts (`valueIsBoolean`, `rhsIsBoolean`); state/interface/payload semantic tokens; hover signatures for view rendering, ref redefinition, filters, and verdicts.
- **Graph depth P1** — Action/interface/requirement body projection: `ActionDefBody` and `ActionUsageBody` walked (`then action`, nested actions, assign/ref/state/for); interface ends expose `portType` with end-typing `Connection` wiring; requirement `verify` and `subject` members emit graph nodes and `Subject` edges. Integration tests: `action_body_semantics`, `interface_body_semantics`, `requirement_body_semantics`.
- **Graph depth P2** — Action-flow graph enrichment for activity diagrams; semantic token recursion for action/requirement bodies; `item def`/`individual def` `AttributeBody` projection; `require constraint` child graph nodes. Integration tests: `activity_graph_semantics`, `item_def_body_semantics`, `action_definitions`.
- **Graph depth P3** — `definition_body`/`occurrence_body` walkers for occurrence and flow definitions; `PartDefBody` completion (`enum`/`item` usages, opaque members, occurrence brace bodies); semantic token expansion for part/item/metadata inner identifiers and requirement body gaps; hover and symbol kinds for P3 node kinds. Integration tests: `definition_body_semantics`, `part_def_body_semantics`, `part_def_tokens`.
- **Graph depth P4a** — Flow/allocation `DefinitionBody` semantics and semantic tokens; General View filters `require constraint` child nodes while preserving inline constraint text. Integration tests: `definition_body_semantics`, `flow_def_tokens`, `model_projection`.

### Changed

- **Parser dependency** — Bumped `sysml-v2-parser` to **0.20.1** on [crates.io](https://crates.io/crates/sysml-v2-parser/0.20.1) (flow/allocation definition bodies, `MetadataAnnotation` in action bodies, `AttributeDef.value_span`, literal-with-unit spans).
- **Release surface alignment** — Rust workspace, `spec42` server, VS Code extension, Zed extension, and GitHub Action examples aligned at `0.29.1`.

## [0.29.0] - 2026-06-08

### Added

- **Parser 0.19.0 graph projection** - `PayloadClause` on actions/transitions, `FinalState`, `MetadataKeywordUsage`, viewpoint `stakeholder`/`purpose`/`TextualRep`; parser-wave fixtures and extended `p2_diagnostics_semantics` coverage.
- **`missing_final_state`** - Information diagnostic when a state definition has state usages but no `final` state.
- **AST-based Boolean classification** - Transition guards and view/import filters set `conditionIsBoolean` at graph-build time (replaces string heuristics when available).
- **Verification case `attribute def`** - Local attribute definitions in verification bodies are projected on the semantic graph.

### Changed

- **Parser dependency** - Bumped `sysml-v2-parser` to **0.19.0** on [crates.io](https://crates.io/crates/sysml-v2-parser) (`PayloadClause`, `TransitionAccept`, `FinalState`, metadata keywords, viewpoint members).
- **`viewpoint_rep_language_unresolved`** - Diagnostic range prefers parser `language_span` on `textualRep` nodes when present.
- **Release surface alignment** - Rust workspace, `spec42` server, VS Code extension, Zed extension, and GitHub Action examples aligned at `0.29.0`.

## [0.28.0] - 2026-06-08

### Added

- **Read-only HTTP API** - `spec42 api serve` exposes workspace validation, doctor, model summary, and diagram export over loopback HTTP; documented in [docs/api/README.md](docs/api/README.md) and [docs/adr/0001-read-only-systems-modeling-http-api.md](docs/adr/0001-read-only-systems-modeling-http-api.md).
- **P2 semantic diagnostics** - Behavior, expression, requirement/case, and view/metadata conformance checks (for example `transition_guard_non_boolean`, `accept_payload_incompatible`, `assignment_value_incompatible`, `view_filter_non_boolean`, `verification_case_invalid_shape`, `viewpoint_reference_unresolved`, `metadata_keyword_collision`); catalog entries and integration tests in `p2_diagnostics_semantics`.
- **Diagnostic pipeline modularization** - Dedicated collectors for behavior, requirement/case, and view/metadata conformance; shared Boolean filter helpers; graph-builder facts for transition guards, view filters, viewpoint bodies, send/accept payloads, and case shape counters.
- **Unit suffix handling** - Quantity/unit suffix validation and richer hover for attributed values with units.
- **Inherited analysis results** - Analysis-case objectives can inherit `analysis result` bindings from specializing definitions.
- **Documentation index** - [docs/README.md](docs/README.md) with user, engineering, architecture, reference, and archive folders; legacy top-level doc paths updated.
- **CodeQL** - `.github/workflows/codeql.yml` for security analysis of the Rust codebase.

### Changed

- **Parser dependency** - Bumped `sysml-v2-parser` to **0.18.0** (metadata definition bodies, `expose` feature chains, port body depth, and related graph-builder follow-through).
- **GitHub Action Marketplace name** - Renamed action listing to `Spec42 SysML Check` so the Marketplace `name` does not collide with the GitHub user `spec42`.
- **CI workflows** - Refactored standard-library bundle handling, integration-test timeouts, performance metrics logging, and Tool42-driven Rust test orchestration.
- **Release surface alignment** - Rust workspace, `spec42` server, VS Code extension, and Zed extension versions aligned at `0.28.0`.

### Fixed

- **Standard library test configuration** - Corrected stdlib path resolution and error handling in integration tests.
- **Model Explorer selection sync** - Retry path for editor-to-explorer selection under concurrent refresh (carried forward from post-0.27.1 work).

## [0.27.1] - 2026-06-05

### Added

- **Diagnostic roadmap** - Added [docs/engineering/DIAGNOSTIC-CHECKS-ROADMAP.md](docs/engineering/DIAGNOSTIC-CHECKS-ROADMAP.md) to document status and rollout sequencing for diagnostic quality checks.
- **Regression coverage** - Expanded integration and unit tests for unresolved-reference diagnostics, rename/reference behavior, and startup/update flow gating.

### Changed

- **Semantic diagnostics** - Refined unresolved relationship and reference diagnostics, including clearer endpoint messaging and post-processing behavior.
- **Reference and rename resilience** - Improved reference resolution and rename flows to handle broader document/editing scenarios with better stability.
- **Model Explorer sync reliability** - Added a retry path for debug source-to-explorer selection sync to reduce CI flakiness under concurrent refresh activity.
- **Release surface alignment** - Rust workspace, `spec42` server, VS Code extension, and Zed extension versions aligned at `0.27.1`.

## [0.27.0] - 2026-06-04

### Added

- **AI assistant integration** - [docs/AI-ASSISTANTS.md](docs/AI-ASSISTANTS.md), [`.github/copilot-instructions.md`](.github/copilot-instructions.md), and [docs/examples/mcp-vscode.json](docs/examples/mcp-vscode.json) for Copilot/Cursor MCP setup.
- **MCP tools** - `spec42_doctor`, `spec42_model_summary`, and `spec42_explain_diagnostic` on `spec42-mcp`; `spec42_check` gains optional `include_semantic_model`.
- **MCP test coverage** - Handler error-path tests, in-process `rmcp` protocol smoke (`mcp_protocol`), subprocess `spec42-mcp` smoke (`mcp_binary`), and unit tests for the diagnostic code catalog; CI builds `spec42-mcp` explicitly.
- **VS Code Language Model Tools** - Four Copilot Agent tools (`spec42_check`, `spec42_doctor`, `spec42_model_summary`, `spec42_explain_diagnostic`) via bundled `spec42` CLI; extension `engines.vscode` ^1.99.0.
- **CLI agent commands** - `spec42 explain-diagnostic` and `spec42 model-summary` (JSON parity with MCP); integration tests in `cli_ai_tools`.
- **Semantic relink** - LSP/code-action path to relink symbols when the semantic graph has moved ahead of editor state.
- **Unresolved type quick fix** - Quick fix to create a matching type definition for unresolved type references.
- **Library view quick fixes** - Quick fixes and clearer library status handling in the library webview.
- **Model Explorer selection sync** - Editor selection and Model Explorer tree stay aligned when navigating structure from either side.
- **Visualization commands** - Additional palette commands for opening and refreshing visualizer views.

### Changed

- **Parser dependency** - Bumped `sysml-v2-parser` to **0.17.0** on [crates.io](https://crates.io/crates/sysml-v2-parser). Structured view/part bodies, `implies` in expressions, and usage-header `:>` / `::>` / `=>` on attribute and port usages (see parser `CHANGELOG.md`).
- **Graph builder for parser 0.17.0** - Maps `AttributeUsage` and `PortUsage` `subsets` / `references` / `crosses` into semantic node attributes.
- **Definition insertion** - Snippet/insertion logic and indentation handling for new definitions in SysML bodies.
- **Hover documentation** - Richer hover text for semantic elements.
- **README and extension metadata** - Onboarding and marketplace-facing description updates for AI assistant and workspace workflows.
- **Release surface alignment** - Rust workspace, `spec42` server, VS Code extension, and Zed extension versions aligned at `0.27.0`.

### Fixed

- **Action smoke CI** - Corrected actionlint invocation in `.github/workflows/action-smoke.yml`.

## [0.26.3] - 2026-06-04

### Fixed

- **GitHub Action version resolution** - Install step now uses `github.action_ref` when `inputs.version` is omitted. Composite actions do not expose `GITHUB_ACTION_REF` to `run` scripts, which caused `No Spec42 version was provided and GITHUB_ACTION_REF is empty` in downstream workflows such as [sysml-examples](https://github.com/elan8/sysml-examples).
- **GitHub Action release ref validation** - Reject branch-style refs (for example `main` → `vmain`) with a clear error; only semver tags like `v0.26.3` download release archives.
- **GitHub Action SARIF upload on forks** - SARIF upload runs with `continue-on-error` so fork PRs still fail only on validation results, not Code Scanning upload restrictions.

### Added

- **Action smoke CI** - `.github/workflows/action-smoke.yml` runs actionlint and exercises the composite Action against a published release binary.

### Changed

- **Release surface alignment** - Rust workspace, `spec42` server, VS Code extension, and Zed extension versions aligned at `0.26.3`.

## [0.26.2] - 2026-06-04

### Fixed

- **GitHub Action manifest** - Quoted the `format` input description in `action.yml` so YAML parsers accept the colon in `spec42 check: text, ...` (fixes `Mapping values are not allowed in this context` when loading `elan8/spec42@v0.26.1`).

### Changed

- **Release surface alignment** - Rust workspace, `spec42` server, VS Code extension, and Zed extension versions aligned at `0.26.2`.

## [0.26.1] - 2026-06-04

### Added

- **GitHub Action** - Composite Action (`action.yml`) that downloads the matching Spec42 release binary for the runner OS, runs `spec42 doctor`, validates models with `spec42 check`, and uploads SARIF to GitHub Code Scanning by default. Documented in [docs/GITHUB-ACTION.md](docs/GITHUB-ACTION.md).

### Changed

- **Release surface alignment** - Rust workspace, `spec42` server, VS Code extension, and Zed extension versions aligned at `0.26.1`.

## [0.26.0] - 2026-06-03

### Removed

- **Software Architecture add-on** - Removed the experimental Rust workspace analyzer (`software-architecture` crate), `software/*` LSP RPCs, dedicated software visualizer views (`software-module-view`, `software-dependency-view`), legacy `generalView.ts` / `graphBuilders.ts` extension renderer path, and the Spec42 Add-ons sidebar UI with related settings and commands.

### Added

- **Workspace import graph** - [`workspace/import_graph.rs`](crates/kernel/src/workspace/import_graph.rs) detects workspace files that import a changed package so `didChange` can republish importer diagnostics immediately (debounced full-workspace republish remains as a backstop).
- **Import/namespace regression coverage** - Added semantic-core tests for recursive namespace import (`::**`) so nested members remain resolvable in cross-document typing and ref scenarios.
- **Import and ref test matrix** - Extended tests for membership imports (`import Pkg::*`), qualified package declarations, part-def/part-usage ref parity, and multi-file `perform_check` workspaces.
- **Nested port semantics tests** - Integration tests for nested port bodies in port definitions and part usages.
- **CLI check workspace-root smoke coverage** - Added server smoke coverage that validates `perform_check` behavior with an explicit `workspace_root`, matching common `spec42 check` workspace invocations.
- **Semantic index ready notification** - LSP clients receive `spec42/semanticIndexReady` after workspace indexing so startup diagnostics filtering and Model Explorer can align with graph readiness.
- **Docs** - [AST-SEMANTIC-COVERAGE.md](docs/engineering/AST-SEMANTIC-COVERAGE.md) prioritization matrix and [LEGACY-RENDERER-SUNSET.md](docs/archive/LEGACY-RENDERER-SUNSET.md) removal plan for legacy webview renderers.

### Changed

- **Parser dependency** - Bumped `sysml-v2-parser` to **0.16.0** on [crates.io](https://crates.io/crates/sysml-v2-parser) (no local path / git pin required for CI). Brings requirement-body `actor` declarations, `enum` usages in part bodies, diagnostic taxonomy and cascade suppression, and fewer false positives on spec-aligned models (see parser `CHANGELOG.md`).
- **Graph builder for parser 0.16.0** - Handles `RequirementActorDecl` in requirement bodies and `EnumerationUsage` in part def/usage bodies; drops kernel priority for removed `missing_statement_separator_between_members` parse code.
- **Examples sidebar** - Lists example workspaces from a single canonical `examples/` root (repository submodule) instead of scanning both `vscode/examples` and `../examples`; hides dot-prefixed folders such as `.github`.
- **Workspace-first extension behavior** - Default workspace indexing is `background`; Model Explorer shows indexing status until the workspace model is ready and no longer falls back to the active file tree when a workspace folder is open. Status bar and **Validate Model** summarize diagnostics across workspace SysML/KerML files. `sysml/model` graph requests from the extension use `workspaceVisualization` when a workspace is open.
- **Cross-file diagnostic refresh** - After `didChange`, importer files are republished in the same handler once the semantic graph is updated; a debounced workspace pass still runs after idle typing.
- **Untyped part usage severity** - `untyped_part_usage` is **Information** (was Warning) to match SysML typing optionalities on usages.
- **Analysis diagnostics on requirements** - Requirement definitions are no longer skipped for analysis status collection (only `constraint def` / `calc def` remain excluded).
- **Nested port bodies in semantic graph** - Port usage bodies (`PortBody::Brace`) are now walked in the graph builder (port def, part def, and part usage paths) so nested ports appear in the workspace graph and views.
- **Semantic tokens for ports** - Token range collection recurses nested port bodies and includes `InOutDecl` members in port definitions.
- **Shared renderer default alignment** - Webview `htmlBuilder` fallback for `useSharedRenderer` matches `package.json` default (`true`).
- **Ref assignment graph parity** - `ref` assignments inside `part def` bodies now emit `reference` edges the same way as `part usage` bodies, reducing reliance on identifier-only fallbacks in downstream models.
- **Type disambiguation for view symbols** - Import/type resolution now includes view/viewpoint suffix disambiguation paths, improving nested namespace resolution for viewpoint conformance and view typing.
- **Release surface alignment** - Rust workspace, `spec42` server, VS Code extension, and Zed extension versions aligned at `0.26.0`.

### Fixed

- **Flaky cross-file diagnostic integration test** - `did_change_republishs_peer_diagnostics_after_debounce` now waits for the latest peer `publishDiagnostics` instead of racing debounce timing against `hover`.
- **Standard SysML view types in examples** - Spec42 standard view types (`InterconnectionView`, `SequenceView`, `StateTransitionView`, `ActionFlowView`, and related) are recognized for type diagnostics like `GeneralView`, so `examples/webshop/Views.sysml` no longer reports spurious `unresolved_type_reference` warnings.
- **Cross-file allocate/satisfy endpoints** - Imported simple names such as `CheckoutService` in `allocate CheckoutService to CommerceCluster` resolve workspace-wide, so `unresolved_allocate_source` is no longer reported when the target exists in another file.
- **Model Explorer diagnostic nodes** - Internal builder diagnostic nodes (for example `unresolved_allocate_source`) are excluded from workspace/document graph payloads and tree building, so they no longer appear as spurious children in Model Explorer.
- **Allocate/satisfy diagnostic clarity** - Relationship endpoint diagnostics now explain when a name resolves to a type definition instead of a usage (`allocate_endpoint_prefers_usage`, `satisfy_endpoint_prefers_usage`), suggest concrete usage paths, and hint at case-mismatched part usages when applicable.
- **Part connection and port compatibility diagnostics** - Clearer semantic messages for allocation typing and homonymous port definitions.

## [0.25.0] - 2026-06-02

### Added

- **`diagram_core` crate** - Added a shared Rust diagram-core layer for reusable view/model shaping and renderer-facing contracts used by visualization pipelines.
- **`sysml_semantic_tokens` crate** - Introduced a dedicated semantic-token crate with parser-aware range extraction, keyword classification, and focused token tests.
- **Shared diagram renderer package** - Added `shared/diagram-renderer` with normalized graph preparation, node-notation handling, theme support, and view-specific renderer modules (general/IBD/action/state/sequence).
- **Pending-relationship diagnostics** - Added explicit semantic diagnostics for unresolved pending relationships to improve cross-document and late-resolution feedback.
- **Viewpoint conformance coverage** - Added viewpoint conformance fixtures and semantic tests to validate conformance graph and diagnostic behavior.
- **General/interconnection notation audit artifacts** - Added BNF/sign-off and notation-inventory docs plus generation tooling to track renderer/spec parity.

### Changed

- **Parser dependency upgrade** - Bumped `sysml-v2-parser` to git tag `v0.14.0` (`https://github.com/elan8/sysml-v2-parser`) and switched back from local path wiring to the tagged GitHub source. This brings qualified package identification support and `ref part` assignment parsing improvements.
- **Semantic graph/evaluation architecture** - Expanded semantic graph construction, relationship resolution, and analysis evaluation paths (including unit-aware handling) for stronger cross-file behavior and more consistent graph-first outputs.
- **Visualization/webview pipeline** - Refactored webview orchestration with render scheduling, quiescence/gating, shared-renderer adapters, and updated UI control/state flows for more predictable refresh behavior.
- **Workspace/library closure handling** - Improved document/workspace services and library-closure flows to better preserve cross-document linkage and indexing consistency.
- **Release surface alignment** - Updated Rust workspace/server, VS Code extension, and Zed extension versions in lockstep for `0.25.0`.

### Fixed

- **Reference and endpoint resolution edge cases** - Improved handling for typed/member-chain and pending-expression endpoint resolution so semantic links and diagnostics are less sensitive to declaration order and container context.
- **IBD/interconnection rendering consistency** - Fixed multiple interconnection/IBD shaping and endpoint-mirroring issues that could lead to missing or unstable rendered structure.
- **Verification/subject relationship propagation** - Corrected subject and verified-requirement relationship wiring so verification semantics are represented consistently in the semantic graph.

## [0.24.0] - 2026-05-15

### Added

- **`semantic_core` crate** - Extracted reusable semantic engine (graph building, resolution, evaluation, diagnostics, and graph-first visualization) for use by `kernel`, future hosts, and services without LSP coupling.
- **MCP server** - Added `spec42-mcp` stdio server with `spec42_check` tool exposing the same validation pipeline as the CLI for AI assistant workflows.
- **`software-architecture` crate** - Promoted software-architecture support from an in-tree plugin to a dedicated crate with custom RPC provider wiring.
- **Graph-first diagnostics engine** - Added neutral diagnostics collection in `semantic_core` with kernel/LSP adapter integration.
- **Semantic core architecture guide** - Added `docs/architecture/SEMANTIC_CORE_ARCHITECTURE.md` documenting module layout and graph-first visualization/diagnostics flows.
- **Content submodule helper** - Added `scripts/update-content-submodules.ps1` for updating `domain-libraries` and `examples` submodules.

### Changed

- **Parser dependency upgrade** - Updated `sysml-v2-parser` integration to `v0.10.0` and aligned graph-builder paths.
- **Visualization and DTO architecture** - Moved graph-first visualization, sequence extraction, IBD/interconnection projection, and shared DTOs into `semantic_core`; slimmed `kernel` to host/runtime concerns.
- **Sequence view extraction** - Reworked sequence-diagram extraction to operate on the semantic graph rather than legacy view-layer paths.
- **Release and CI packaging** - Extended release workflow and validation pipelines for `spec42-mcp`, recursive submodule checkout, and updated binary staging.
- **Default logging** - Reduced default runtime logging noise for quieter editor sessions.

### Fixed

- **IBD composite scope** - Pruned IBD payloads to retain unconnected parts that belong under the same composite structure.
- **Build and duplication guardrails** - Removed duplicate semantic implementations and added dependency/debt guardrail tests to keep crate boundaries stable.

## [0.23.0] - 2026-05-05

### Added

- **Domain libraries as submodules** - Added `domain-libraries` and `examples` as tracked submodules to keep large model content versioned independently while preserving repository reproducibility.
- **Brace-based folding fallback** - Added LSP folding support that can derive foldable regions from brace structure when AST-driven folding ranges are unavailable.
- **Incomplete analysis diagnostics** - Added semantic-diagnostic support for incomplete analysis expressions so partially authored models surface clearer feedback during edit loops.

### Changed

- **SysML release alignment** - Updated bundled SysML release references from `2026-02` to `2026-03`.
- **Parser dependency upgrade** - Updated `sysml-v2-parser` integration to `v0.9.0` and aligned dependent surfaces.
- **Environment resolution behavior** - Enhanced environment/config/data-directory resolution to better support custom installation layouts.
- **Submodule/content tracking** - Replaced committed in-repo domain-library/example snapshots with submodule pointers to current upstream content.

### Fixed

- **Semantic evaluation status accuracy** - Corrected semantic evaluation status reporting so type errors are reflected accurately in analysis results.

## [0.22.0] - 2026-04-30

### Added

- **Semantic validation expansion** - Added deeper analysis diagnostics for verification semantics, allocation conformance, objective bindings, invalid verdict values, and analysis-constraint expressions.
- **Expression evaluation improvements** - Expanded semantic expression evaluation for qualified references, local attributes, unit-aware values, requirement bodies, and analysis cases.

### Changed

- **Kernel crate and validation architecture** - Reorganized the former core package into the `crates/kernel` workspace layout, split validation and visualization internals into smaller modules, and removed the deprecated `semantic_model` compatibility layer.
- **Parser/dependency alignment** - Updated the SysML v2 parser integration and dependency setup to track the newer parser surface used by current semantic and visualization paths.
- **Release/package staging** - Added VS Code package staging support and strengthened package-layout verification for release artifacts.
- **Documentation and onboarding** - Refreshed the root, VS Code, Zed, example, and domain-library READMEs to better explain `spec42`, guide first-time users, and promote example-driven evaluation.

### Fixed

- **Analysis diagnostic accuracy** - Improved semantic graph construction around verification, package bodies, requirement bodies, analysis cases, and expression references so diagnostics are more precise and less dependent on incidental graph shape.
- **Visualization model shaping** - Hardened visualization extraction/projection behavior for semantic analysis data and current view rendering paths.

## [0.21.0] - 2026-04-24

### Added

- **Sequence View rendering pipeline** - Added end-to-end sequence-diagram extraction/projection/rendering support in the backend and VS Code visualizer, including lifelines, messages, activations, and fragment handling.
- **Software Architecture add-on workflows** - Added software workspace analysis/project-view APIs plus VS Code add-on UX for running analysis and opening dedicated software architecture visualizations.
- **Expanded domain library coverage** - Added broad new domain-library content across software, electronics, communication, and robotics layers with structured rule sets and updated documentation.
- **Webshop end-to-end examples** - Added a richer webshop sample set (`architecture`, `behavior`, `requirements`, `views`) to validate explicit-view projections and cross-file visualization behavior.

### Changed

- **Parser/runtime alignment to `sysml-v2-parser` 0.7.0** - Updated parser dependency and adapted semantic-model/view extraction paths to newer AST behavior.
- **Visualizer panel/update lifecycle hardening** - Refined panel restore/update timing, content hashing, and refresh flow across startup, restore, and workspace contexts.
- **Workspace visualization model shaping** - Improved explicit-view projection, model parameter parsing, and package/container grouping behavior for more consistent rendered diagrams.

### Fixed

- **Webshop visualization regressions** - Fixed empty/missing structure/interconnection/state/action results in cross-file explicit-view scenarios (including projection and IBD scope fallback behavior).
- **`satisfy` diagnostic false positives** - Fixed typed-member-chain resolution and diagnostic suppression for references like `instance.member` across documents.
- **Action Flow UX issues** - Fixed edge-label behavior for structural flow markers and improved click-to-source reliability/disambiguation.
- **Startup empty visualizer race** - Fixed restore/startup timing cases where the visualizer pane could stay empty with no listed views until manual reopen.

## [0.20.0] - 2026-04-21

### Added

- **Expression evaluation in semantic model** - Added a new evaluation module with unit-aware expression handling and integration coverage for referenced attributes and unit expression behavior.
- **Inlay hints support** - Added inlay-hint capability wiring in the language server surface and related test coverage.
- **Expanded visualization data surface** - Added a dedicated SysML visualization endpoint and richer DTO/model payload support for container groups and connector endpoint relationships.

### Changed

- **Parser and semantic alignment** - Upgraded `sysml-v2-parser` (through `0.6.0`) and updated semantic projection paths for part definitions/usages, activity flows, and state-machine extraction.
- **Visualizer rendering/UX architecture** - Refined view state handling, text measurement/truncation, rendering flow, and loading behavior to improve responsiveness and diagram readability.
- **Action/State view release defaults** - Promoted Action Flow and State Transition views to default-enabled visualizer views and removed experimental labeling/toggle configuration from the extension surface.
- **Workspace and explorer interaction cleanup** - Simplified model-explorer/selection synchronization paths and tightened visualizer payload handling across workspace scenarios.

### Fixed

- **Hover/type resolution robustness** - Improved type resolution behavior in hover and related semantic lookup paths for more reliable editor feedback.

## [0.19.0] - 2026-04-16

### Changed

- **`sysml-v2-parser` upgrade and host alignment** - Updated Spec42 to the newer `sysml-v2-parser` releases through `0.4.0`, aligned semantic/expression handling with the newer AST shapes, and added a shared `spec42_core::sysml_v2` re-export so hosts can use the pinned parser surface consistently.
- **Semantic graph and endpoint resolution** - Expanded semantic graph construction and member resolution around package bodies, interface definitions, part usages, and connection endpoint lookup so port-like members and local references resolve more reliably.

### Fixed

- **Port compatibility diagnostics** - Replaced raw port type-name comparison with SysML v2 style feature-compatibility checks, reducing false `port_type_mismatch` warnings when differently named ports are directionally and structurally compatible.
- **Semantic diagnostic false positives** - Reduced noisy diagnostics around `redefines`, forward-resolvable `satisfy` references, delegated/redefined ports, and syntax-only missing-semicolon guesses so valid models are less likely to receive misleading warnings.

## [0.18.1] - 2026-04-10

### Fixed

- **General View workspace loading hang** - Fixed an infinite traversal in General View scene construction that could leave the visualizer stuck on "Parsing SysML model" for larger workspace visualizations such as `apollo-11-sysml-v2`.
- **Workspace package visibility in VS Code visualizer** - Fixed workspace General View package selection so the visualizer uses workspace-scoped model metadata instead of collapsing back to the currently open file, restoring visibility of packages outside the active document.

## [0.18.0] - 2026-04-10

### Added

- **Bundled SysML standard library** - The official `sysml.library` tree from the SysML v2 Release is repacked at build time, embedded in the `spec42` binary, and materialized into the spec42 data directory when needed (metadata records `repo: "embedded"`).
- **Default library paths in LSP** - `Spec42Config` carries default library search paths from the resolved environment; the server merges host defaults with paths from `initialize` / `didChangeConfiguration` so clients can extend discovery without replacing host layout.
- **`spec42 stdlib` CLI** - Subcommands to show status, print the resolved path, and `clear-cache` to remove materialized standard-library files (re-created from the embedded copy on next use). Legacy `stdlib install` / remove flows were removed in favor of the bundled workflow.
- **Import scope checks** - The semantic model applies import-scope rules so diagnostics and analysis respect SysML v2 import visibility beyond earlier membership/import resolution.

### Changed

- **Standard library resolution order** - After explicit flags, env, config, and a valid managed install under the data directory, resolution prefers **materializing the embedded archive** before falling back to the legacy VS Code `globalStorage` path, so upgrades pick the bundled release without manual cleanup.
- **VS Code Library view** - The extension no longer downloads or manages the standard library from the UI; the Library view shows the bundled release inline and defers materialization to the server.
- **`sysml-v2-parser` upgrade** - Bumped the parser dependency and adjusted semantic graph, token range, and graph-builder paths (including state machines, part definitions/usages, requirement bodies, package bodies) for compatibility with newer AST shapes and Apollo-oriented parsing behavior.
- **Document symbols** - Normalized document symbol extraction in the semantic layer for more consistent outlines and navigation.
- **Semantic graph and import resolution** - Expanded import-resolution and relationship handling (including cross-document typing tests) and refreshed validation/diagnostics integration.

## [0.17.0] - 2026-04-09

### Added

- **Visible parser declaration support** - Added first-class support for `FeatureDecl` and `ClassifierDecl` so these package-level declarations now appear in document symbols, named-element extraction, semantic graph output, feature inspector responses, and General View diagrams.

### Changed

- **Published parser dependency** - Switched Spec42 from the old git-pinned parser dependency to the published `sysml-v2-parser` crate `0.1.0`.
- **Parser compatibility handling** - Updated semantic graph, symbol, and diagnostic/token code paths to tolerate newly introduced parser AST variants without breaking existing behavior.

## [0.16.0] - 2026-04-08

### Added

- **SysML v2 import semantics in semantic resolution** - Added shared import-membership resolution for membership imports, namespace imports, `public` re-export chains, and recursive `::**` imports across semantic linking.

### Changed

- **Import-aware editor navigation** - Updated hover and go-to-definition to resolve symbols through spec-valid import chains instead of relying only on direct local typing edges or symbol-table heuristics.
- **Import-aware semantic diagnostics** - Updated unresolved-type diagnostics to respect SysML v2 import visibility rules so valid `public import` chains resolve cleanly while private-only chains continue to warn.
- **Semantic-model backend architecture** - Introduced a reusable import-resolution layer with semantic-graph caching so future import-aware features can share one standards-aligned backend path.

## [0.15.1] - 2026-04-07

### Fixed

- **Imported library type resolution** - Resolved cross-document typing for unqualified type references introduced via `import` declarations, including KerML modeled declarations from library sources.
- **Loose-file diagnostics after library refresh** - Rebuilt semantic links for non-library documents after initialization/configuration library scans so files outside workspace roots stop reporting stale unresolved-type diagnostics.

## [0.15.0] - 2026-04-07

### Added

- **Initial Zed extension** - Added a Zed extension for SysML v2 with Tree-sitter-based language support, LSP wiring to `spec42`, and automatic download of the matching `spec42` release binary when no configured or `PATH` binary is available.
- **Zed CI and release packaging** - Added CI coverage for building the Zed extension and release packaging that publishes a Zed extension source bundle alongside the existing server archives and VS Code extension artifact.

### Changed

- **Explorer-first sidebar UX** - Upgraded the Model Explorer with richer semantic summaries/tooltips and retired the separate Feature Inspector view from the VS Code extension surface.

## [0.14.0] - 2026-04-02

### Added

- **Visualizer canvas interactions** - Added richer visualizer canvas handling, including zoom/pan improvements for diagram exploration.

### Changed

- **Frontend visualization architecture cleanup** - Removed legacy sequence view/debug-label paths, tightened enabled-view handling, and refined visualizer initialization/render flow.
- **Documentation alignment** - Updated root/development/extension docs to reflect frontend-rendered diagrams and current Spec42 feature/configuration status.
- **Configuration surface reduction** - Removed unused visualization settings and semantic-token dump setting from the VS Code extension contribution surface.
- **Model Explorer navigation UX** - Updated tree selection navigation to better reuse existing editors when opening source locations.

## [0.13.0] - 2026-04-01

### Added

- **State transition visualization** - Added a new state-transition view for visualizing state machines with states and transitions, including SVG export and dedicated test fixtures.
- **Parsing performance benchmarking** - Added Criterion benchmarks (including a `parse_scan` benchmark) and documented how to run them.

### Changed

- **SysML parser + semantic model enhancements** - Updated the `sysml-v2-parser` dependency and expanded action-definition handling in the semantic model (including parameters, perform steps, and richer connection extraction for activity diagrams).
- **Indexing performance** - Improved startup and workspace indexing performance, including graph lookup caching and additional indexing/stdlib parsing optimizations.
- **Repository hygiene** - Removed outdated output/reference files that no longer reflect the current project structure.

### Fixed

- **Visualizer UI** - Disabled the layout-direction button for the state-transition view to avoid presenting a non-applicable control.

## [0.12.0] - 2026-03-31

### Changed

- **CI reliability** - Removed integration tests that depended on an external `C:\Git\sysml-examples\office\office.sysml` fixture path.
- **VS Code extension tests** - Removed a brittle interconnection SVG route-analysis test that depended on webview render/export timing and exact SVG geometry.

## [0.11.0] - 2026-03-30

### Added

- **Requirements relationship visibility controls** - Added `subject` relationship support in the semantic graph and General View, plus a General View toggle to show/hide requirement-related nodes and edges (`subject`, `satisfy`, `verify`) with default ON behavior.
- **Workspace visualization coverage** - Added/expanded integration coverage for multi-file workspace visualization flows and all-package graph expectations.
- **Production troubleshooting switch** - Added `spec42.logging.verbose` so production installs stay quieter by default while enabling deeper runtime logs on demand.

### Changed

- **spec42-core modularization (technical debt reduction)** - Split large hotspot modules into smaller focused units without changing external behavior:
  - `semantic_model/mod.rs` extracted hover/signature and symbol-entry logic.
  - `sysml_model.rs` extracted request-param parsing and graph projection helpers.
  - `semantic_checks.rs` extracted rule helper functions.
  - `lsp_server.rs` began service/helper extraction for qualified symbol lookup flows.
  - `semantic_model/graph_builder.rs` extracted requirement-subject edge helper logic.
- **General + Interconnection visualization data shaping** - Improved workspace/model graph handling (including synthetic-node stripping and root/selection behavior) to better align rendered views with real package/model structure.
- **Startup and indexing performance** - Improved startup traceability and introduced parallel parsing for scanned entries to reduce indexing bottlenecks in larger workspaces.
- **Library UX updates** - Refined standard/custom library management UX and model-explorer package statistics behavior.

### Fixed

- **Click-to-source regression in General View** - Restored reliable node click behavior and source navigation in workspace visualization scenarios.
- **Diagnostic quality improvements** - Reduced invalid semantic diagnostics around declared type references and built-in type handling (for example `String`), with better diagnostic anchoring.
- **Visualizer command icon consistency** - Updated the visualizer entry icon to `open-preview` for clearer VS Code affordance.

## [0.10.0] - 2026-03-27

### Changed

- **macOS Apple Silicon startup compatibility** - Added `darwin-arm64` server artifact build and VSIX packaging so the language server starts natively on Apple Silicon without architecture mismatch spawn failures.
- **Release packaging validation** - Updated package layout verification to require bundled `darwin-arm64` server binaries in both staged layout checks and VSIX content checks.

## [0.9.1] - 2026-03-27

### Added

- **Untyped part quick fix** - Added a Quick Fix for untyped part usage (for example `part display;`) that can create a matching `part def Display { }` and rewrite usage to `part display : Display;`.
- **Quick-fix diagnostics coverage** - Added unit and integration tests for the new untyped-part diagnostics and code-action flow.

### Changed

- **Parser update** - Updated `sysml-v2-parser` and aligned parser references across `spec42-core` and `server`.
- **Model Explorer auto-refresh** - Added debounced Model Explorer refresh/reload on SysML/KerML save and file-system changes, including workspace-aware re-scan on create/delete.
- **Diagnostic clarity** - Suppressed cryptic empty-type unresolved warnings by ignoring empty declared type references in semantic typing checks.

## [0.9.0] - 2026-03-27

### Added

- **Semantic diagnostics hardening** - Added new semantic diagnostics for invalid multiplicity intervals (`invalid_multiplicity`) and unresolved declared type references (`unresolved_type_reference`), plus guardrails for invalid/self-referential `redefines` metadata (`invalid_redefines_reference`) when available in the semantic graph.
- **Requirements slice hardening** - Added requirement usage typing resolution to requirement definitions (including cross-file linking) and added explicit satisfy-resolution diagnostics (`unresolved_satisfy_source`, `unresolved_satisfy_target`).
- **Edit-loop diagnostics coverage** - Added integration tests covering invalid intermediate edits that later become valid, and semantic diagnostics for unresolved type references.
- **Tiered CI validation workflow** - Added an informational `.github/workflows/full-validation.yml` workflow that runs SysML release workspace validation on PR/schedule/manual dispatch.
- **Requirements slice fixtures/tests** - Added focused requirement fixtures and deterministic integration tests for same-file/cross-file requirement typing and unresolved satisfy diagnostics.

### Changed

- **Required CI fast path** - Updated `.github/workflows/ci.yml` to explicitly run required fast Rust checks (`cargo test --workspace`, `cargo clippy --workspace --all-targets`).
- **CI requirements visibility** - Added explicit requirements-slice integration test invocations to the fast CI workflow for clearer release gating signal.
- **Developer guidance** - Expanded `DEVELOPMENT.md` with semantic diagnostics pipeline/codes and clarified fast-vs-full CI validation strategy.
- **Technical debt hardening** - Added release preflight checks and SHA256 release asset checksums in `.github/workflows/release.yml`, and expanded required extension CI to include the interconnection test suite.
- **Docs and metadata alignment** - Updated `DEVELOPMENT.md` local validation steps to match current repo workflows and aligned extension experimental-view wording in `vscode/package.json`.
- **Parser reproducibility** - Pinned `sysml-v2-parser` in `spec42-core/Cargo.toml` and documented parser update policy in `DEVELOPMENT.md`.
- **VS Code extension packaging** - Updated extension packaging to include runtime dependencies required at activation time and tightened packaging excludes to avoid shipping unnecessary development/test artifacts in the VSIX.

## [0.8.0] - 2026-03-27

### Added

- **Library experience and discoverability** - Added richer library search DTOs and surfaced stronger library status/search flows in the VS Code extension, with improved support for workspace + managed library usage.
- **LSP feature coverage** - Added type hierarchy support and completed additional remaining LSP capabilities, with integration-test coverage updates to keep behavior stable.

### Changed

- **Server architecture and diagnostics** - Refactored key parts of the LSP server and model-explorer context handling, improved code lens generation, and integrated tracing-based logging to make troubleshooting and diagnostics clearer.
- **Parser and dependency updates** - Updated parser/dependency stack and related tests to improve reliability and maintainability.
- **Documentation and marketplace metadata** - Refreshed README badges/screenshots/instructions and corrected the VS Code Marketplace badge link.

## [0.7.0] - 2026-03-25

### Changed

- **Diagram quality and UX** - Improved diagram behavior across General View and Interconnection View, including stronger connector handling for typed/interface-based connections, backend-root-driven interconnection filtering, fit-to-window defaults, and related visualizer usability updates.

## [0.6.0] - 2026-03-23

### Changed

- **Interconnection visualizer layout and routing** - Improved IBD layout quality and connector routing reliability for denser models, including clearer root selection behavior and more readable exported diagrams.
- **Diagnostics quality** - Improved diagnostics stability and clarity in the VS Code workflow, with better behavior during active edit/update cycles.

## [0.5.0] - 2026-03-13

### Added

- **Interconnection View release enablement** - Promoted `interconnection-view` from experimental opt-in to a release-enabled default view with CI-backed export coverage.
- **Multi-workspace VS Code test fixtures** - Added stronger fixture coverage for single-file, multi-file, large-workspace, and timer-based visualization scenarios.

### Changed

- **Visualization UX** - Improved interconnection root selection, root summaries, port direction legend support, and clearer no-connector guidance in the visualizer.
- **Source-to-diagram behavior** - Editor-driven diagram selection now reveals and centers matching elements more reliably when available.
- **VS Code test infrastructure** - Standardized cross-platform test workspace `serverPath` configuration so Windows and Linux test hosts boot the language server consistently.

### Fixed

- **Crash and edit-loop robustness** - Hardened incremental edit handling, malformed range handling, repeated open/edit/close lifecycles, and semantic token refresh after edits.
- **VS Code integration stability** - Fixed startup and restart issues in test and CI environments, and made extension-host tests deterministic again.
- **Definition and visualization regressions** - Stabilized go-to-definition test coverage and visualizer export checks across representative fixtures.

## [0.4.0] - 2026-03-12

### Changed

- **New sysml-v2-parser** - Switched to a new sysml-v2-parser dependency for improved parsing and alignment with SysML v2.

## [0.3.0] - 2026-03-10

### Added

- **General View diagram** - New diagram view showing the model structure with element hierarchy, attributes, ports, and parts. Nodes use standard SysML-style compartments.

## [0.2.2] - 2026-03-06

### Changed

- VS Code extension display name updated to "SysML v2 Language Support" to meet marketplace requirements and reduce name confusion with other language server extensions.

## [0.2.1] - 2026-03-06

### Fixed

- **UTF-8 / multi-byte handling:** `position_to_byte_offset` now correctly converts LSP character indices to byte offsets (e.g. for "cafe"). `completion_prefix` iterates by character to avoid panics on multi-byte content. Error masking in `parse_sysml_collect_errors` uses character boundaries so multi-byte lines no longer produce invalid UTF-8.
- **Parse error messages:** Low-level Pest messages are mapped to clearer user-facing text (e.g. "expected metadata_annotation" -> "unexpected token; perhaps missing an attribute or expression"); original message is appended for debugging. Additional mappings for package, member, name, identifier, import, expressions, literals, parentheses, etc.

### Changed

- Removed unused `line_char_to_byte_offset` from kerml-parser (server already has equivalent logic). Extended `improve_pest_error_message` with more grammar-rule mappings.

### Added

- Unit tests for multi-byte edge cases: `position_to_byte_offset`, `word_at_position`, `completion_prefix`, and `parse_sysml_collect_errors` with UTF-8 in the error region.

## [0.2.0] - 2025-03-06

### Added

- Calc def result expressions: parser now supports bare result expressions (e.g. `capacity / currentDraw`) without a final semicolon, per SysML v2 7.19.2.
- Full validation suite test: parses all `.sysml` files in SysML-v2-Release `sysml/src/validation`; test is `#[ignore]`d (run with `cargo test -p kerml-parser -- --ignored`); CI runs it with `--include-ignored`. Per-file summary (pkgs, members, lines) logged when running the suite.

### Fixed

- "position" no longer incorrectly marked as keyword in semantic tokens; it is a contextual keyword only and valid as an identifier (e.g. `out position : String`).
- Shared reserved keyword list: single source of truth in `language::RESERVED_KEYWORDS` for semantic token fallback and goto-definition/rename suppression; eliminates discrepancies between keyword lists.

## [0.1.0] - 2026-03-05

### Added

- LSP over stdio: text sync, diagnostics, hover, completion, go to definition, find references, rename, document symbols, workspace symbol search, code actions, formatting.
- Workspace-aware indexing for `.sysml` and `.kerml` files (workspace folders and root URI).
- VS Code extension with SysML/KerML syntax highlighting and language configuration.
- Parser aligned with the [SysML v2 Release](https://github.com/Systems-Modeling/SysML-v2-Release) validation suite.

### Known limitations

- Parser is aligned with the SysML v2 Release validation suite; it does not claim full OMG spec compliance.
- Some constructs may have incomplete semantic token or outline coverage.

[0.49.0]: https://github.com/elan8/spec42/releases/tag/v0.49.0
[0.48.0]: https://github.com/elan8/spec42/releases/tag/v0.48.0
[0.47.3]: https://github.com/elan8/spec42/releases/tag/v0.47.3
[0.47.2]: https://github.com/elan8/spec42/releases/tag/v0.47.2
[0.47.1]: https://github.com/elan8/spec42/releases/tag/v0.47.1
[0.47.0]: https://github.com/elan8/spec42/releases/tag/v0.47.0
[0.46.1]: https://github.com/elan8/spec42/releases/tag/v0.46.1
[0.46.0]: https://github.com/elan8/spec42/releases/tag/v0.46.0
[0.33.0]: https://github.com/elan8/spec42/releases/tag/v0.33.0
[0.32.0]: https://github.com/elan8/spec42/releases/tag/v0.32.0
[0.31.0]: https://github.com/elan8/spec42/releases/tag/v0.31.0
[0.30.0]: https://github.com/elan8/spec42/releases/tag/v0.30.0
[0.29.1]: https://github.com/elan8/spec42/releases/tag/v0.29.1
[0.29.0]: https://github.com/elan8/spec42/releases/tag/v0.29.0
[0.28.0]: https://github.com/elan8/spec42/releases/tag/v0.28.0
[0.27.1]: https://github.com/elan8/spec42/releases/tag/v0.27.1
[0.27.0]: https://github.com/elan8/spec42/releases/tag/v0.27.0
[0.26.3]: https://github.com/elan8/spec42/releases/tag/v0.26.3
[0.26.2]: https://github.com/elan8/spec42/releases/tag/v0.26.2
[0.26.1]: https://github.com/elan8/spec42/releases/tag/v0.26.1
[0.26.0]: https://github.com/elan8/spec42/releases/tag/v0.26.0
[0.25.0]: https://github.com/elan8/spec42/releases/tag/v0.25.0
[0.24.0]: https://github.com/elan8/spec42/releases/tag/v0.24.0
[0.23.0]: https://github.com/elan8/spec42/releases/tag/v0.23.0
[0.22.0]: https://github.com/elan8/spec42/releases/tag/v0.22.0
[0.21.0]: https://github.com/elan8/spec42/releases/tag/v0.21.0
[0.20.0]: https://github.com/elan8/spec42/releases/tag/v0.20.0
[0.19.0]: https://github.com/elan8/spec42/releases/tag/v0.19.0
[0.18.1]: https://github.com/elan8/spec42/releases/tag/v0.18.1
[0.18.0]: https://github.com/elan8/spec42/releases/tag/v0.18.0
[0.17.0]: https://github.com/elan8/spec42/releases/tag/v0.17.0
[0.15.1]: https://github.com/elan8/spec42/releases/tag/v0.15.1
[0.11.0]: https://github.com/elan8/spec42/releases/tag/v0.11.0
[0.15.0]: https://github.com/elan8/spec42/releases/tag/v0.15.0
[0.14.0]: https://github.com/elan8/spec42/releases/tag/v0.14.0
[0.13.0]: https://github.com/elan8/spec42/releases/tag/v0.13.0
[0.12.0]: https://github.com/elan8/spec42/releases/tag/v0.12.0
[0.10.0]: https://github.com/elan8/spec42/releases/tag/v0.10.0
[0.9.1]: https://github.com/elan8/spec42/releases/tag/v0.9.1
[0.9.0]: https://github.com/elan8/spec42/releases/tag/v0.9.0
[0.8.0]: https://github.com/elan8/spec42/releases/tag/v0.8.0
[0.7.0]: https://github.com/elan8/spec42/releases/tag/v0.7.0
[0.6.0]: https://github.com/elan8/spec42/releases/tag/v0.6.0
[0.5.0]: https://github.com/elan8/spec42/releases/tag/v0.5.0
[0.4.0]: https://github.com/elan8/spec42/releases/tag/v0.4.0
[0.3.0]: https://github.com/elan8/spec42/releases/tag/v0.3.0
[0.2.2]: https://github.com/elan8/spec42/releases/tag/v0.2.2
[0.2.1]: https://github.com/elan8/spec42/releases/tag/v0.2.1
[0.2.0]: https://github.com/elan8/spec42/releases/tag/v0.2.0
[0.1.0]: https://github.com/elan8/spec42/releases/tag/v0.1.0
