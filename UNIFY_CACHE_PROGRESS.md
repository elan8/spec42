# Unified Cache Implementation Tracker

Working branch: `integration/unified-cache`

Tracks execution of `UNIFY_CACHE_PLAN.md` §12 and the `ROUNDTRIP_SEMGRAPH_PREREQS.md` §8
enablement gate. This file records ownership and status only; the two design documents remain
authoritative for intent.

---

## Conclusion of this work phase

**This phase is stopping short of the cutover, deliberately. The unified cache cannot be completed
as specified until the semantic resolution layer is settled.** That is a real blocker, not a
scheduling problem, and the honest thing is to stop at a coherent boundary and say so rather than
build the remaining layers on an unresolved foundation.

### What was delivered

Everything below is merged to `integration/unified-cache`, pushed, and verified at the recorded
test baseline with `cargo clippy --workspace --all-targets -- -D warnings` clean.

- **Plan step 3, complete.** The whole physical cache layer: the sharded `.s42c` object store,
  the manually decoded envelope, canonical postcard-plus-zstd payloads, lock-free atomic
  publication, typed miss reasons, the LRU capacity policy, and the `CacheStore` API — with tests
  covering corruption, truncation, bit flips, key mismatch, decompression bombs, concurrent
  writers, interrupted writes, unwritable roots, and prune convergence.
- **Plan step 2, complete.** Typed BLAKE3 identities in their own `source_identity` crate,
  `SourceManifest` with a root digest that commits every entry, role, digest and the ordering
  policy, and the breaking migration of repository-owned metadata off SHA-256 strings.
- **Round-trip prerequisites B1, B3, B4 and B7, complete.** Typed edge construction ownership;
  canonical source roles and resolution precedence; the `SemanticPublication` phase, completeness
  and identity contract; and the single cache-import invariant validator, whose failures carry no
  diagnostic code or range and so can never surface as though the user's model were at fault.
- **B9 substantially advanced.** All keys are migrated or deleted except a small entangled
  residue. 24 redundant `*Type` projections were retired once it was established that
  `add_typing_edge_if_exists` records the declared target into `relationships.typing` *before*
  attempting resolution, which proves those producers duplicated an existing typed fact. What
  remains is the residue listed below and the final field deletion (chunk G).

The store is genuinely usable infrastructure. It is not wired to any call site, which is correct:
nothing should consume it until the artifacts it stores are trustworthy.

### Why it stops here

`UNIFY_CACHE_PLAN.md` §2.2 requires that cold, warm, parallel, incremental and cache-disabled
paths be semantically equivalent, and `AGENTS.md` requires the same. That is the cache's central
correctness claim — everything else is an optimization detail.

While implementing the prerequisites, three pre-existing semantic defects surfaced. One is fixed.
The other two mean **the equivalence guarantee is not currently true, independently of caching**:

1. Whole-graph linking and the scoped/incremental resolver are two independently implemented
   resolution engines. They are known to be different code; whether they produce different results
   has never been established.
2. Type-reference resolution is workspace-wide, where KerML requires scoping by namespace
   containment with visibility and import filtering. This affects every typing kind.

A cache is a disposable accelerator and must never be what makes an inconsistency permanent.
Persisting and redistributing graphs produced by one of two divergent engines, under a resolution
rule that does not match the specification, would do exactly that — and would make the divergence
harder to find, because it would no longer reproduce from source alone.

The remaining plan steps are all downstream of this. Step 4 defines the graph artifacts, step 5
routes every production surface through one build service, and step 8 is the parity gate. Each of
those either bakes in the current resolution behaviour or is meant to prove an equivalence that is
not yet true.

### What must happen before this resumes

`RESOLUTION_LAYER_INVESTIGATION.md` is the kickoff brief for the blocking work. It carries the
established findings and citations so the design activity does not re-derive them. Its most
important output for the cache is the answer to one question: **what does a reference's resolution
actually depend on?** Once resolution depends on enclosing namespace, visibility and imports, the
set of edits that can invalidate it is much larger than a name match — and if resolution depends
on inputs the cache key does not commit, the cache is unsound. That question must be answered
before any artifact key covering a graph can be declared complete.

`ROUNDTRIP_SEMGRAPH_PREREQS.md` §8 already forbids enabling persistent graph artifacts until its
gate passes. That gate is not met, and this work does not enable them.

### Resuming cleanly

The remaining work is independently resumable in this order. None of it is blocked on anything
except where noted.

1. Finish B9: the entangled key residue, then chunk G to delete the field. This unblocks a real
   postcard round-trip test, which is currently impossible. Note that part of the residue is
   itself blocked on the resolution work — see below — so chunk G cannot fully complete until
   that lands.
2. B11 (graph state fingerprint) — not blocked, and needed regardless of how the resolution work
   lands. B7's validator is in place and is the natural backing for
   `workspace::cache::api::CacheArtifact::validate_invariants`, which is not yet wired to it.
3. B5, B8, B2, B6 (the graph record, canonical encoding, index rebuild, workspace rehydration) —
   blocked on B9 completing, because the record cannot be defined while the untyped attribute bag
   exists.
4. Plan steps 4 onward — blocked on the resolution work above, not merely on the prerequisites.

---

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
| F2 | the `*Type` classification family | merged; entangled residue remains | `feat/b9-type-family` |
| G (partial) | chunk D dual write (`doc`/`text`/`language`/`body`/hover `keyword`), the relationship-target family (`redefines`/`subsetsFeature`/`referencesFeature`/`crossesFeature`/`specializes`), retired dead keys | merged | `feat/b9-chunk-g-partial` |
| G (remaining) | `generalView*` rollups (owned by a DTO, not `SemanticNode`, see below), the blocked `*Type` residue, then delete the field | not started | |

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

### Chunk G (partial): what was retired

`feat/b9-chunk-g-partial` retired every remaining `SemanticNode.attributes` key that was not
blocked by the resolution-scoping defect, driven by a fresh exhaustive sweep of every
`attrs.insert`/`attributes.insert` producer and every `.attributes.get`/`.attributes[...]` reader
in the repository:

- **Chunk D's transitional dual write** (`doc`, `text`, `language`, `body`, hover-only `keyword`):
  every remaining post-construction reader on `SemanticNode` (`workspace/src/snapshot/facts.rs`,
  `generator_api/src/model.rs`, `lsp_server/src/views/feature_inspector.rs`,
  `sysml_diagnostics/src/checks/view_metadata_conformance.rs`) now reads the typed
  `SourceTextFacts` directly. The producer writes onto `SemanticNode.attributes` were deleted from
  every graph-builder site (`graph_builder/mod.rs`, `action.rs`, `part_def.rs`, `kerml_library.rs`,
  `package_body/materialize.rs`, `requirement_body.rs`, `use_case.rs`). A new
  `model_projection::project_source_text_attributes` boundary-DTO projection (same precedent as
  chunk E's `project_expression_text_attributes`) serves the transport-DTO consumers
  (`GraphNodeDto`/`HostSemanticModelNode`) that still expect these keys in a JSON map, without
  writing anything back onto `SemanticNode`. One unrelated `"doc"` key survives in
  `requirement_body.rs` (a require-constraint child field distinct from source-fidelity doc text,
  called out at chunk E time) and needs no migration.
- **The relationship-target family** (`redefines`, `subsetsFeature`, `referencesFeature`,
  `crossesFeature`, `specializes`): `visualization/scope.rs` now reads
  `declared_facts.relationships.redefinition`/`subsetting` directly; `model_projection/
  general_view_fold.rs` reads them through a new `model_projection::
  project_relationship_target_attributes` boundary-DTO projection (`redefines`/`subsetsFeature`
  only — the other three have no reader left, see below). Every graph-builder producer of
  `redefines`/`subsetsFeature`/`referencesFeature`/`crossesFeature`/`specializes` onto
  `SemanticNode.attributes` was deleted. One real gap surfaced and was fixed: the metadata-def
  restriction shorthand (`:>> annotatedElement : ...`) previously wrote `subsetsFeature` as a
  second key alongside `redefines` for that one gate, encoding that a redefinition of a
  semantic-metadata restriction feature is *also* a subsetting per KerML; `attribute_body.rs` now
  records that dual relationship as a second typed `DeclaredRelationshipFacts::subsetting` target
  explicitly, instead of only the redefinition.
- **Fully dead keys retired outright** (no reader anywhere): `referencesFeature`, `crossesFeature`
  (redundant with `attach_declared_subsetting_family`'s `reference_subsetting`/`cross_subsetting`
  typed facts) and the display-only `specializes` attribute (redundant with
  `wire_def_specialization_edge`'s real per-target edges and
  `DeclaredRelationshipFacts::specializes`; distinct from the `"specializes"` edge-kind/`rel_type`
  string used throughout the graph and DTOs, which is untouched). `referencesFeature`/
  `crossesFeature` were added to `semantic_ownership_guardrails.rs`'s
  `RETIRED_TYPING_PROJECTION_KEYS` (repo-wide ratchet); `specializes` was not, because that string
  is legitimately reused as a `RelationshipKind`/`rel_type` literal outside the attribute map and
  an unscoped literal-string guard would false-positive on every one of those.
- Fourteen tests across `sysml_model`/`workspace` that asserted directly against the legacy
  `SemanticNode.attributes` map for these keys were rewritten to assert against the typed
  `declared_facts.relationships`/`source_text` facts instead (same observable behavior, typed
  source of truth) — see the branch's commits for the full list.
- `semantic_ownership_guardrails.rs`'s `EXCLUDED_MODULES` now also excludes
  `model_projection.rs` (the file, not the `model_projection/` directory, which was already
  excluded): it owns the two new boundary-DTO projection functions above and would otherwise be
  flagged as a "consumer" for writing the very keys it projects.

**Not retired — the `generalView*` projection rollups** (`generalViewDirectAttributes`,
`generalViewDirectParts`, `generalViewDirectPorts`, `generalViewInheritedAttributes`,
`generalViewInheritedParts`) are written by `general_view_fold.rs` onto `GraphNodeDto.attributes`,
**not onto `SemanticNode.attributes`** — `GraphNodeDto` is a boundary transport DTO, populated
from the semantic graph earlier in the same request and never written back. Since B9's field to
remove is `SemanticNode.attributes`, these do not block that deletion and were left alone rather
than spending chunk G's remaining budget on a DTO-internal refactor. They are still the anti-
pattern B9 names in spirit ("projection-time JSON aggregates … Replace … with typed projection
results owned outside the semantic graph"), just one level removed from the blocking field, so a
future pass should still give them a typed `GeneralViewDetails`-shaped result instead of a JSON
map on the DTO.

### Remaining attribute keys and why

After chunk F2 and chunk G (partial), these keys are the entire remaining barrier to deleting
`SemanticNode.attributes`. Every one of them is blocked on the resolution-scoping defect, directly
or by entanglement with a key that is; nothing else is left.

| Key(s) | Producer sites | Remaining readers | Why it remains |
|--------|-----------------|--------------------|-----------------|
| `attributeType`, `dataType`, bare `type` | `graph_builder/{action,analysis_case,attribute_body,occurrence_body,package_body/materialize,part_def,port_def,requirement_body,unit_type_promotion,usage_builders,verification}.rs` | `lsp_server/src/lsp_runtime/symbols.rs`, `sysml_diagnostics/src/checks/behavior_conformance.rs`, tests | Blocked on the resolution-scoping defect. Routing attribute typing into `relationships.typing` sends it through the over-broad workspace-wide `link_workspace_relationships_pass`. |
| `payloadType`, `acceptType` | `graph_builder/{flow_usage,payload}.rs` | `lsp_server/src/lsp_runtime/symbols.rs`, tests | Pairing them would newly populate `relationships.typing` for constructs that do not currently populate it, widening the same defect's blast radius. |
| `partType`, `portType`, `refType`, `parameterType` | `graph_builder/{occurrence_body,usage_builders}.rs` (`partType`); `graph_builder/{interface_def,port_def}.rs` (`portType`); `graph_builder/ref_decl.rs` (`refType`); `graph_builder/{action,calc_constraint_def,port_def}.rs` (`parameterType`) | `lsp_server/src/lsp_runtime/symbols.rs`, tests | Redundant, but entangled with the blocked `attributeType`/`dataType` chain in `detail_type_name`; needs its own typed-DTO rewrite that is meaningless to do before the scoping fix lands (it would just be redundant JSON either way). |
| `renderingType` | `graph_builder/view_def.rs` | `lsp_server/src/lsp_runtime/symbols.rs`, tests | Mixed semantics on view columns; one site carries no typing meaning at all, so it cannot be folded into the blocked typing chain without first disentangling that site. |
| `returnType`, `analysisResultType` | `graph_builder/analysis_case.rs` (`returnType`); `semantic/analysis_typing.rs` (`analysisResultType`) | `sysml_diagnostics/src/checks/behavior_conformance.rs`, tests | Have semantic, non-presentation readers with no safe typed home yet, and sit in the same `detail_type_name`/typing-resolution family as the rows above. |

`baseType`, `relationType` and one `dataType` site (in `general_view_fold.rs`) are not attribute
keys at all — they are unrelated string identifiers/edge-kind labels — and need no migration.

**Consequence: chunk G cannot fully complete until the resolution-scoping defect is fixed.** Every
row above is blocked on it directly or by entanglement. This is the concrete way the resolution
blocker reaches into the cache work — it is not merely a correctness concern held at arm's length,
it physically prevents removing the field that blocks postcard encoding of the graph. Beyond this
table, the only other thing standing between the repository and deleting
`SemanticNode.attributes` is the `generalView*` rollup refactor described above, which is
unblocked and can proceed independently whenever a future chunk has budget for it.

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
| B7 | Typed `GraphInvariantError` and single cache-import validator | done |
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
