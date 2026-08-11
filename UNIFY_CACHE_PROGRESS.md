# Unified Cache Implementation Tracker

Working branch: `integration/unified-cache`

Tracks execution of `UNIFY_CACHE_PLAN.md` §12 and the `ROUNDTRIP_SEMGRAPH_PREREQS.md` §8
enablement gate. This file records ownership and status only; the two design documents remain
authoritative for intent.

## Test baseline

Recorded on `integration/unified-cache` at `df9ea0b5` (docs-only vs `main`):

- `cargo test -p workspace --test snapshot_single_build`: 22 failed, 1 passed, 1 ignored.
- Every other workspace test target: green.

Those 22 failures are pre-existing on `main` and unrelated to this work. They are the accepted
baseline: a change is green if it introduces no failure outside that set. They are tracked
separately and must not be "fixed" by weakening assertions in passing.

## Phase status

| # | Plan step | Status | Owner |
|---|-----------|--------|-------|
| 1 | Round-trip prerequisites, incl. attribute-bag removal | in progress | see B9 table |
| 2 | Typed BLAKE3 identities, exact source snapshots, metadata v2 | done | merged |
| 3 | Lock-free sharded store, postcard/zstd envelope, capacity, management API | done | merged |
| 4 | Parse, library-index, closure, library-graph, workspace-graph artifacts | not started | blocked on 2, 3 |
| 5 | `SemanticBuildService`, route every production surface through it | not started | blocked on 4 |
| 6 | CLI management and observability | not started | blocked on 5 |
| 7 | Remove legacy caches, SHA-based metadata, legacy graph command | not started | blocked on 5 |
| 8 | Parity, corruption, concurrency, performance gates | not started | blocked on 7 |

Step 3 is deliberately started in parallel with step 1: the physical store depends on neither the
semantic model nor the attribute-bag removal. Steps 2 and 4 onward are genuinely sequential
because they consume the digest types and the graph record.

## B9: semantic-node attribute bag removal

`SemanticNode.attributes: HashMap<String, serde_json::Value>` at
`crates/sysml_model/src/semantic/model.rs:1914` must be removed with no deprecated map, no
unknown-key escape hatch, no compatibility accessor, and no dual-read path. Postcard cannot encode
`serde_json::Value`, so this blocks every graph cache artifact.

Chunks are sized so that parallel agents do not contend for the same keys. Several chunks touch
the same files; ownership is per key, not per file.

| Chunk | Keys | Status | Branch |
|-------|------|--------|--------|
| A | `shortName` | merged | `feat/b9-shortname-units` |
| B | unit prefix/conversion/value-expression metadata | merged | `feat/b9-shortname-units` |
| C | state machine: `source`, `target`, `isInitial`, `targetIsDone`, `stateName`, `finalStateCount` | merged | `feat/b9-state-facts` |
| D | source fidelity: `doc`, `body`, `text`, `language`, `keyword` (hover use) | merged | `feat/b9-source-fidelity` |
| E | analysis and expression: `value`, `defaultValue`, `lhs`, `rhs`, `condition`, `isThen`, `analysis*`, `objectiveBoundTo`, `originRange` | merged | `feat/b9-analysis-expression` |
| F | relationship endpoints and semantic classification keys | merged |  |
| F2 | the `*Type` classification family (37 keys) | in progress | `feat/b9-type-family` |
| G | presentation cutover and field deletion: `generalView*` rollups, remaining `lsp_server`/`server`/`generator_api`/`workspace` consumers, then delete the field | not started | |

Chunk C additionally deleted `stateName` outright as a redundant duplicate of the node's own
name, and replaced the `finalStateCount`/`doneTransitionCount` visit counters with a derivation
over the graph rather than storing a counter as a fact.

Chunk D reclassified `metaclassRole`: it has no documentation or hover consumer at all, and its
only two readers make a genuine semantic classification decision, so it moved to chunk F.

Chunk E deleted `originRange`, `analysisKind`, `analysisParams`, `analysisReturn` and
`parameters` outright: an exhaustive search found no reader for any of them. A key with no
reader is deleted, not migrated. It also introduced
`semantic/model_projection.rs::project_expression_text_attributes`, which re-projects typed facts
onto a *boundary DTO's* JSON map at transport construction sites only. Nothing is written back
onto `SemanticNode`; that projection is the precedent for how later chunks may serve presentation
consumers without reintroducing a second semantic authority.

### Two cross-document resolution engines

B1 established typed edge construction ownership and made whole, parallel, incremental and
decoded builds agree on it, by tagging ownership structurally at the single edge-insertion choke
point rather than trusting whichever pass added the edge.

While doing so it confirmed that whole-graph linking and the scoped/incremental resolver are two
independently implemented resolution engines, to the point that fixes have had to be applied to
both separately. B1 only required identical *ownership*, which is now guaranteed regardless of
which engine produced an edge. Unifying the two algorithms is a larger separate concern and is
not yet scheduled; it is a standing risk to the plan's requirement that full, incremental, cached
and parallel paths be observably equivalent.

### Chunk F remaining work

Chunk F (in progress, `feat/b9-relationship-classification`) migrated the fully "genuine semantic
classification" reads: `endType` (new `DeclaredSemanticFacts::interface_end_type` plus
`declared_end_reference()`, kept out of `relationships.typing` because folding it in made
unrelated unresolved-typing diagnostics double-fire), `metaclassRole` (new
`DeclaredSemanticFacts::metaclass_role: Option<KermlMetaclassRole>`), the `keyword` metadata-view
semantic use (new `DeclaredSemanticFacts::modeled_keyword`, distinct from `SourceTextFacts::keyword`
which stays hover-only), and `refTarget` (new `DeclaredRelationshipFacts::reference_target` under
`RelationshipKind::Reference`). `subjectRef` had no reader anywhere and was deleted outright. The
guardrail's `RELATIONSHIP_PROJECTION_KEYS` now also covers `endType`, `metaclassRole`, `refTarget`,
`keyword`. Still open for chunk F: `redefines`/`subsetsFeature`/`referencesFeature`/
`crossesFeature`/`specializes` are already dual-written to `DeclaredRelationshipFacts` by every
SysML producer but still have presentation consumers
(`lsp_server/src/views/feature_inspector.rs`, `lsp_server/src/views/model.rs`,
`language_service/src/presentation_hover.rs`) reading the legacy attribute map directly; the
`*Type` classification family is entirely unmigrated. A gap-closing attempt to also populate
`relationships.typing` for `attribute`/`attribute def` typing (to retire
`lsp_server/src/lsp_runtime/symbols.rs`'s `attributeType`/`dataType`/`type` fallback) was reverted
after it changed a semantic-graph golden fixture: `link_workspace_relationships_pass` republishes
edges from every populated `relationships.typing` entry using workspace-wide resolution, which
resolved an edge the corpus fixture expects to stay unresolved. That interaction has now been
settled normatively — see below.

## Finding: type-reference resolution ignores KerML scoping

Investigated against the OMG pilot implementation. This is a semantic defect in Spec42, not a
cache or attribute-bag question, and it is not specific to attributes.

**Attribute typing is an ordinary `FeatureTyping`.** `AttributeUsage`, `PartUsage`, `PortUsage`
and `ItemUsage` all route through one shared grammar chain ending at
`org.omg.sysml.xtext/.../SysML.xtext:437-444`; there is no attribute-specific typing rule.
`AttributeUsage.java:32` extends `Usage`, and `Feature.java:549` (`getOwnedTyping`) and `:418`
(`getType`) are inherited uniformly. The DataType restriction is enforced *after* resolution by a
separate validator (`SysMLValidator.xtend:559-562`, `checkAttributeUsage`), and the pilot's own
expectation test `AttributeUsage_invalid.sysml.xt:37-61` shows `attribute a : A;` with `A` a
`part def` resolving successfully and only then failing validation. So recording attribute typing
in `relationships.typing` is correct, and the untyped-string treatment is the artifact.

**But `link_workspace_relationships_pass` resolves type references workspace-wide, where the
specification requires namespace-containment walking with visibility and import filtering.** The
pilot routes `FeatureTyping` through `KerMLScopeProvider.xtend:96-97` to
`scope_nonExpressionNamespace`, which descends `scope_Namespace → scopeFor` (`:137-173`), walking
up `NamespaceUtil.getParentNamespaceOf` and only falling back to the global index at the namespace
root (`:166-167`). `SysMLScopeProvider.xtend:55-78` inherits this for attributes unchanged.

This affects **every** typing kind, not attributes specifically. The golden fixture that flipped
was exposing a pre-existing scoping gap; it is evidence for the scoping bug, not against recording
attribute typing.

Consequences: the attribute-typing rewire and the `symbols.rs` fallback are blocked on fixing the
scoping, not on a design decision about attributes. The fix belongs to the resolution pass and
must be scoped to all typing kinds. It is not yet scheduled, and it should be sequenced against
B3, which is auditing the same layer for ordering determinism. Until then the corpus fixture must
not be changed.

This matters for the cache beyond correctness: the plan requires full, incremental, cached and
parallel paths to be observably equivalent, which is only meaningful once resolution itself is
well defined. Together with B1's finding that whole-graph and incremental linking are two
independently implemented resolution engines, the resolution layer is the least settled part of
this work.

### Resolving normative language questions

Several findings in this effort are not cache questions at all but SysML/KerML semantics
questions: whether attribute typing is an ordinary `FeatureTyping`, what scope a type reference
resolves in, and whether a golden fixture encodes correct behaviour or an implementation artifact.

Do not settle these by reading Spec42's own code, and do not settle them by picking whichever
answer keeps the fixtures green. The authoritative sources are the specification and the OMG
pilot implementation, available locally at
`/Users/luke/Documents/GitHub/SysML-v2-Pilot-Implementation` — in particular `sysml.library/`
(the normative model library), `org.omg.sysml/` (metamodel and derived-property implementations),
the Xtext grammars, and the `*.xpect.tests` expectation suites, which frequently encode exactly
the edge cases in dispute.

When a question of this kind blocks work, investigate there and record the answer with citations,
distinguishing what the specification states normatively from what the pilot implementation
merely happens to do. A fixture may then be changed as a deliberate, cited correction — never as
a quiet edit.

### Agent worktree hygiene

Each agent worktree builds its own `target/`, at roughly 20 GB apiece. Seven concurrent
worktrees exhausted the disk mid-merge. Agents must run `cargo clean` as their final action after
committing and verifying, and merged worktrees should be removed promptly.

### Transitional dual population

Chunk D leaves its producers writing *both* the typed `SourceTextFacts` and the legacy attribute
entries for keys whose remaining consumers have not migrated yet. Every read it owns goes through
the typed fact, so there is no dual-read path, but the legacy write is a second writer and must
not outlive the migration.

Chunk G is responsible for deleting those writes along with the field. A chunk that removes the
last consumer of a key must also remove that key's legacy write; the attribute bag is not
considered gone while any producer still populates it.

Chunk E is the largest and is best split into a producer-side pass in `sysml_model` followed by a
consumer-side pass in `sysml_diagnostics`. Chunks E and F both touch `graph_builder/action.rs`,
`analysis_case.rs`, and `usage_builders.rs`, so they are sequenced rather than run concurrently.
Chunk G must be last: it deletes the field and therefore depends on every other chunk.

### Key classification rule

Each key is classified before migration:

- **Semantic** — read to make a construction, name-resolution, typing, evaluation, analysis, or
  diagnostic decision. Becomes a canonical typed declared/effective/evaluated fact at the earliest
  layer holding all prerequisites.
- **Source fidelity** — source spelling, documentation text, ranges. Belongs to the AST or a typed
  source fact. May be projected to JSON at a transport or render boundary, never read back into a
  semantic decision.
- **Presentation only** — currently written back onto the node purely to be projected later.
  Becomes a typed projection result owned outside the semantic graph.

A key with both a semantic and a presentation consumer is split into a typed semantic fact plus a
separately derived projection. It is not carried as JSON to serve both.

## Remaining round-trip blockers

Tracked against `ROUNDTRIP_SEMGRAPH_PREREQS.md` §8. Persistent `LibrarySemanticGraph` and
`WorkspaceSemanticGraph` artifacts stay disabled until every row passes.

| Blocker | Summary | Status |
|---------|---------|--------|
| B1 | Typed edge construction ownership; rebuild cross-document ownership from it | done |
| B2 | Omit lookup/containment indexes from the record; rebuild and validate them | not started |
| B3 | Complete source roles and canonical resolution precedence | done |
| B4 | `SemanticPublication` identity, phase, completeness | done |
| B5 | `SemanticGraphRecordV1` replaces direct runtime serde | not started |
| B6 | Graph hit rehydrates sources and ASTs; no concealed missing input | not started |
| B7 | Typed `GraphInvariantError` and single cache-import validator | in progress |
| B8 | Canonical, byte-stable encoding | not started |
| B9 | Attribute bag removed | in progress |
| B10 | Decode bounds; no stack overflow on hostile nesting | in progress (store layer) |
| B11 | `GraphStateFingerprint` plus query and post-edit differential suites | not started |

## Semantic defects found while doing this work

These are pre-existing correctness problems surfaced by the cache effort, not caused by it. They
are listed separately because they are not cache work and outlive it.

| Defect | Status |
|--------|--------|
| Cross-document edge ownership was not reconstructible; a stale edge could survive an edit beside its replacement | fixed by B1 |
| Whole-graph linking and the scoped/incremental resolver are two independently implemented resolution engines | open, unscheduled |
| Type-reference resolution is workspace-wide where KerML requires containment, visibility and import scoping; affects all typing kinds | open, unscheduled |
| 22 pre-existing `snapshot_single_build` failures on `main` | open, out of scope for this effort |
