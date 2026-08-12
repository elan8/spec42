# Canonical Semantic Resolution Design

Status: implementation handoff
Audience: maintainers of `sysml_model`, `workspace`, `sysml_diagnostics`, `language_service`,
`lsp_server`, and the unified semantic cache
Answers: `RESOLUTION_LAYER_INVESTIGATION.md`
Target base: `upstream/main` (`09ef5c4e` at the time of investigation)

## 1. Decision summary

Spec42 will have one semantic resolution engine. It will resolve every authored reference against
one stable whole-model input, publish one immutable `SemanticModel`, and expose that model through
one read-only query surface. Sequential construction, parallel construction, document replacement,
and later cache decoding may prepare the input differently; none of them will implement resolution
or mutate an already published model.

The deciding requirement is not merely that references can be resolved. Spec42 publishes a model
that diagnostics, validation, navigation, evaluation, visualization, generators, and cache/export
surfaces query repeatedly. Those consumers need fast, deterministic access to one coherent set of
semantic answers. They must not trigger context-dependent resolution work or observe different
answers according to query order or cache warmth. This requirement favors resolving the complete
admitted model once, freezing the resulting semantic publication, and serving downstream queries
from indexes over that publication.

This is a clean migration:

- Delete the URI-scoped relationship resolver, its frontier calculation, its public entry points,
  and its tests and benchmarks in the same upstream change.
- Do not retain deprecated aliases, compatibility wrappers, dual-read paths, or a feature flag for
  the old resolver.
- Keep parallel parsing and document graph construction, but express sequential/parallel work as a
  construction strategy passed to one build service. Resolve only after the deterministic merge
  barrier.
- Keep document replacement as a workspace operation that captures a new immutable source snapshot
  and builds a new `SemanticModel`. Delete in-place graph patching from the published-model API.
  Readers continue using the prior coherent model until the replacement is atomically installed.
- Do not plan dependency-scoped semantic resolution as part of this architecture. Full-model
  incremental resolution for SysML is a research problem without an established precedent and may
  be impractical given recursive scope, imports, inheritance, typing, and redefinition.
- Delete public name-resolution helpers that accept caller-selected element-kind allowlists.
  Reference kind, scope origin, and metamodel target domain are owned by the resolution layer.
- Keep authored reference facts distinct from their derived resolution outcomes. A resolved edge
  is a projection of the canonical outcome, not another authority.
- Keep unresolved, ambiguous, unsupported, and failed states explicit. Ambiguity retains all
  candidates in canonical order; non-convergence is a failed publication, never a warning followed
  by apparent success.
- Keep the parser document as the source-fidelity owner. An immutable `ParsedDocument` retains its
  source storage, parser arena, recovery nodes, spans, and parser-local typed IDs. Semantic
  compilation borrows that document and constructs the new semantic IR; it does not wholesale-copy
  or remap the parser arena into a second syntax store.
- Do not resume persistent semantic-graph caching until this migration lands and the cache branch
  incorporates the new resolution state and semantic contract version.

The full-resolution policy is deliberate, not an interim approximation. The sibling
`sysml-compiler` reached the same
correctness-first conclusion: its LSP reparses from content cache where possible, then rebuilds and
resolves one whole `ModelContext`. It does not cache project semantic analysis.

### 1.1 Why whole-model batch resolution is the chosen architecture

Resolution is not a set of independent string lookups. Its semantic products form a dependency
cycle:

```text
reference target
    -> lexical/effective scope
        -> owned + inherited + imported membership
            -> resolved specialization / typing / subsetting
                -> effective names and redefinitions
                    -> lexical/effective scope
```

Imports introduce another recursive branch because an imported namespace's effective exports may
depend on its imports and inherited memberships. Qualified feature navigation may depend on the
resolved type of an earlier segment. Subject and other effective relationships are derived from
resolved source relationships. A correct engine must therefore solve a mutually dependent semantic
system to a coherent fixed point; processing every authored reference once is insufficient.

There are three broad execution models:

| Model | Strength | Cost/risk for Spec42 |
|---|---|---|
| Pilot-style lazy contextual resolution | Avoids computing unused answers and can reuse framework object/proxy caches | Dependencies are implicit in recursive calls and cache entries; invalidation, negative lookup, cycles, query-order independence, and cross-surface equivalence are harder to establish |
| Resolve while mutating the semantic graph | Makes resolved edges immediately visible to existing graph consumers | Later queries can observe earlier mutations; indexes become stale; partial/failing solves contaminate the input; repeated materialize/reindex/solve phases appear |
| Whole-model batch resolution over stable input | Makes dependencies and convergence explicit and produces one queryable result for all consumers | Pays the cost of solving all admitted references for each semantic publication and requires that the solver be engineered to make this fast |

The OMG Pilot's lazy scope provider remains the semantic reference for lookup behavior. Its
execution model is optimized for an interactive EMF/Xtext object graph in which references are
resolved when requested. That can be efficient when only a small portion of a model is queried,
but it moves correctness obligations into proxy state, recursive query order, memoization, and
fine-grained invalidation. Reproducing that lifecycle across Spec42's CLI, LSP, generators,
parallel builds, incremental edits, and persistent cache would create multiple opportunities for
the same reference to acquire different answers.

Spec42's dominant workload is different: after construction it exposes a semantic model to many
downstream consumers, and those consumers commonly traverse large parts of it. Deferring
resolution would make every consumer participate in resolver lifetime, error, cache, and
invalidation policy. It would also make query latency unpredictable and allow a model's observable
completeness to depend on which queries had happened to run.

The selected design therefore performs one batch solve for the complete admitted model. The
solver may be internally demand-driven and memoized while evaluating a pass, but it must force all
published authored reference sites before declaring convergence. Successful completion produces an
immutable `ResolutionState` and immutable query indexes. Downstream consumers use
`ResolutionView`; they do not invoke the working solver.

The publication boundary is:

```text
stable structural graph + configuration
              |
              v
       private ResolutionDb
       mutable working slots
              |
        converge or fail
              |
              v
immutable SemanticModel {
    structural graph + ResolutionState + disposable query indexes
}
              |
              v
       all downstream consumers
```

Here, `stable structural graph` means the new immutable semantic IR owned by the publication. It
does **not** mean retaining the existing mutable `SemanticGraph` and treating a clone or read-only
borrow of it as the new architecture. The immutable IR is constructed directly from parser-owned
typed syntax into compact node, string, qualified-reference, and authored-fact tables. Its node and
fact identities are opaque dense domains owned by that publication.

The existing `SemanticGraph` is migration debt. It may remain temporarily behind isolated adapters
for semantic families that have not yet been ported, but it is not extended with new facts,
indexes, linkers, caches, lookup helpers, or resolution behavior. A migrated fact family is removed
from the mutable graph path in the same change that makes the immutable IR authoritative; there is
no dual read, fallback, parity arbitration, or post-construction join between parser facts and
legacy graph nodes. In particular, the new builder never scans the old graph by URI, name, kind, or
source range to rediscover semantic ownership. It receives the owning opaque node identity when it
constructs the immutable node and records authored facts at that point.

This is also the reason graph mutation is forbidden during resolution. The graph is a prerequisite
of every resolution query and must keep one identity throughout the solve. Mutating it would:

- make results depend on traversal or thread scheduling because later requests could see facts
  published by earlier requests in the same pass;
- invalidate name, parent, adjacency, inheritance, and import indexes while they are being read;
- expose half-resolved states in which graph edges, indexes, diagnostics, and completeness refer to
  different semantic moments;
- overwrite the distinction between an authored reference and its resolved, unresolved, or
  ambiguous outcome;
- make cancellation, non-convergence, and out-of-order completion difficult to discard without
  reconstructing the previous graph;
- force graph-facing consumers into materialize/rebuild-index/solve-again cycles;
- make persistent cache identity dishonest because there would be no single immutable input or
  atomic settled output to hash and validate;
- prevent isolated parallel evaluation, since workers would contend on shared semantic truth.

`ResolutionDb` is allowed to mutate private working arrays while solving. Those arrays are not a
published semantic model and are discarded on failure. Only a complete `ResolutionState` crosses
the publication barrier. Resolved relationship adjacency and other indexes are derived from that
state and can be rebuilt; they do not compete with it as authorities.

The performance assumption is explicit: a whole-model solve must be fast enough for ordinary CLI
and editor publication. That assumption is plausible because a batch solver can use compact typed
slots, prebuilt ownership/name/edge indexes, canonical work ordering, worklists that revisit only
changed families within a solve, and isolated parallel evaluation followed by deterministic merge.
It also avoids repeating equivalent lazy queries independently in diagnostics, navigation,
validation, and generators.

The parser owns source bytes, authored spelling, token spans, qualified-reference segments, and
parser-local IDs inside each immutable `ParsedDocument`. A compound syntax identity is therefore
`(DocumentId, ParserLocalId)`, where `DocumentId` is the immutable source-snapshot document identity
and `ParserLocalId` is meaningful only in that document's arena. A parser-local integer is never
used without its document. This compound identity is the one consumed by source-fidelity,
diagnostics-range, and syntax-aware services.

Semantic compilation owns a separate typed string side table for semantic entities only. It borrows
parser segments and provenance while constructing dense model-owned declaration and semantic-
reference IDs. It interns normalized cross-document semantic names and other lookup keys exactly
once in the publication, while retaining the parser identity for authored provenance. It does not
copy every parser node, source string, or arena entry into the semantic model, and it never asks a
consumer to reconstruct a parser reference from display text. Renderers, protocol adapters, and
stable external identity projections cross this boundary explicitly through typed APIs.

The publication string table stores each distinct UTF-8 byte sequence once in one contiguous byte
arena. A dense span table maps opaque `TextId` values to byte ranges, and a private hash index stores
only `TextId` values; collision checks resolve an ID through the arena and compare borrowed bytes.
The index never owns a second string key, facts never retain pointers into a growable buffer, and
hash iteration never assigns IDs or orders output. Construction can therefore reallocate safely:
typed IDs remain stable, while `&str` borrows exist only for the duration of an immutable table
borrow. Freezing converts the byte and span stores to immutable slices and retains or rebuilds the
lookup index only where a supported service requires text-to-ID lookup.

The semantic compiler may use private per-document scratch buffers during sequential or parallel
construction, but those buffers are not a second parser representation and are never published as
another document representation. Compilation visits immutable `ParsedDocument` values in canonical source order,
assigns dense model-owned IDs only to semantic declarations, memberships, authored references, and
other semantic entities, then discards its scratch state at the publication barrier. Parser-local
IDs remain valid only through their owning `(DocumentId, ParsedDocument)` pair. Cache encoding of
parser results retains the parser's source/arena envelope; semantic cache encoding retains the
compiled semantic publication. Neither format serializes disposable hash buckets, random seeds, or
capacity.

Every compact identity domain is represented by a distinct opaque newtype: string symbols, node
ordinals, authored-reference ordinals, fact slots, and similar indexes are never aliases for
`usize` or interchangeable integer fields. Their numeric representation and conversions remain
private to the owning table. Semantic code obtains an ID only through that owner's typed API, so
the compiler rejects cross-domain lookup and arbitrary integer construction.

The resolver's working representation is compiled once from those source-faithful structural
facts. Hot paths use dense node/reference ordinals, interned string identities, compact slot arrays,
and indexed adjacency or membership ranges. They do not use owned `String`, `Url`, or compound
public `NodeId` values as per-query map keys and do not clone semantic nodes or rebuild whole-model
maps while resolving a reference. Conversion back to stable public identities happens at the
publication boundary. The string table is owned immutable publication data; lookup indexes over
its IDs remain private disposable accelerators and never become semantic authorities.

Construction borrows immutable parser documents. An exhaustive parser-to-semantic compiler
classifies every encountered AST family as modeled, unsupported, or recovery-produced while
constructing semantic entities. An absent family is recorded as absent; an unclassified family is
an input-construction failure. Unsupported and recovery facts retain `(DocumentId, ParserLocalId)`
and exact parser provenance, but do not mint settled semantic IDs. There is no finished document
fragment abstraction that can become a second source of truth, and no generic builder method may
assert completeness for an unvisited family.

Qualified names and feature chains are segmented by the parser while typed token boundaries are
still available. Semantic compilation consumes the parser's borrowed segment view, preserving
separator semantics, absolute scope, authored spelling, and exact ranges through the compound
document/parser-local identity. For a semantic reference, it creates a compact model-owned path
record or segment range and interns only the normalized semantic names required by cross-document
lookup. It never reconstructs grammar by splitting or joining text, and never allocates a
per-lookup segment vector. This follows the sibling compiler's token-driven canonicalization model
without duplicating the parser's source arena. Unsupported parser forms remain explicit and cannot
publish a partial semantic path.

The sibling implementation provides useful performance evidence for this representation: its typed
`u32` node domains, flat extra-data arenas, dense resolution slots, ordinal parent arrays, and
two-pass CSR edge indexes remove allocation-heavy generic resolution machinery. Spec42 adopts those
structural techniques while retaining distinct opaque ID types, exhaustive ambiguous outcomes, and
dependency-complete publication identity. It does not adopt the sibling's duplicate string storage,
fixed segment or supertype limits, generic packed fields, mutable resolver pointer, raw edge-index
escape, same-pass recursive memoization, or nondeterministic cache serialization.

Before solving, the frozen authored IR is compiled once into dense parent and authored-reference
tables, canonical per-scope bindings, typed relationship inputs, and import ranges. Solver outcomes
are indexed directly by authored-reference ordinal; variable-size candidate and effective-membership
sets live in flat arenas and are referenced by typed ranges. Fixed-point families read the same
complete previous-pass state and write reusable next-pass buffers, which are swapped only at the pass
barrier. Published outgoing and incoming relationships use purpose-built typed CSR indexes. This
keeps local queries proportional to their result size without making an index authoritative.

Performance is nevertheless a measured acceptance criterion, not an article of faith. The
whole-model solver is the correctness oracle. Representative CLI and LSP benchmarks must record
construction, solve, and downstream query time separately. If solving is too slow, optimization
stays inside this one engine: improve indexes and algorithms, reuse immutable standard-library
products with complete identity, and reuse content-addressed parser documents before semantic
compilation and the solve. Performance pressure must not reintroduce scoped resolution or
cross-model reuse of settled resolution outcomes.

Fine-grained incremental semantic resolution is intentionally outside the chosen architecture. It
would require proving complete invalidation across positive and negative lookup, imports,
inheritance, typing, redefinition, effective membership, and every recursive cycle. There is no
evidence yet that the affected closure is materially smaller than the whole model often enough to
repay the dependency tracking, memory, and correctness cost. This design therefore solves the
correct whole-model case and makes that solver fast.

## 2. Scope and non-goals

### 2.1 In scope

- Scope computation and qualified-name traversal for:
  - `FeatureTyping`, including conjugated-port typing and subject usages;
  - general `Specialization`;
  - `Subsetting`, `Redefinition`, `ReferenceSubsetting`, and `CrossSubsetting`;
  - verified-requirement references from which case-level `Subject` relationships are derived;
  - namespace and membership imports.
- Migration of every upstream caller of `resolve_type_reference_targets`,
  `resolve_type_target_in_workspace`, and equivalent fallbacks. The current inventory includes
  analysis typing, derivation ends, flow payload typing, metadata/annotation conformance,
  requirement/use-case conformance, enum checks, name diagnostics, and interactive navigation in
  addition to the minimum relationship families above.
- One typed result contract for those reference families.
- One whole-model resolution phase used by sequential, parallel, replacement, and later decoded
  construction paths.
- Migration of diagnostics, navigation, hover, evaluation, projections, and other consumers away
  from ad hoc name lookup.
- Evidence and boundaries separating safe pre-semantic reuse from speculative incremental
  semantic resolution.
- Removal of the remaining `symbols.rs` attribute-type presentation fallback after canonical
  typing facts are proven sufficient.

### 2.2 Non-goals

- No persistent semantic cache is enabled by this change.
- No dependency-scoped or cross-model semantic-resolution reuse is implemented or planned by this
  design.
- No source compatibility is promised for the deleted scoped or caller-filtered resolution APIs.
- Expression/member-chain resolution is not silently folded into plain relationship lookup.
  Its first lexical segment must eventually use the same namespace-scope owner, but its
  type-directed continuation remains a distinct typed query.
- This work does not canonicalize `nodes_by_uri`, change presentation ordering, or rework B1 edge
  construction ownership, B3 `NodeId` ordering, or B4 publication identity.
- Corpus fixtures are not updated merely to preserve current output. Every changed fixture needs a
  Pilot/spec citation and an explicit correction classification.

## 3. Established evidence

### 3.1 Spec42 has two real engines, not two adapters

The whole resolver in `semantic/relationships.rs` walks the whole graph and independently resolves
typing, specialization, the subsetting family, and subjects. The URI-scoped resolver in
`semantic/relationships/cross_document.rs` duplicates typing, specialization, subject, and
conjugated-port behavior. Parallel full construction calls the URI-scoped resolver for every URI;
document replacement can call either the whole or scoped path.

Both paths ultimately depend on `resolve_type_reference_targets`, whose current behavior is not a
lexical scope algorithm:

- it gathers container-prefixed candidates;
- simple names may scan the complete source URI by suffix;
- when no import is in scope, it may accept a unique graph-wide same-name member;
- it returns `Vec<NodeId>`, after which several consumers select `.next()` and discard ambiguity.

Diagnostics and language services also call this helper or reproduce graph-wide fallback logic.
Consequently, merely making the two linkers call one existing helper would not correct the scope
defect or establish one semantic owner.

### 3.2 Exploratory differential harness

A throw-away Rust executable under `/tmp/spec42-resolution-probe` loaded each Markdown
compatibility fixture, extracted its source documents, and compared `to_semantic_sexpr()` after:

1. sequential whole construction;
2. parallel full construction;
3. whole document replacement in forward order;
4. whole document replacement in reverse order;
5. scoped document replacement in forward order;
6. scoped document replacement in reverse order.

The prototype changed no repository file and is not a deliverable. Its result was:

| Measurement | Result |
|---|---:|
| Parallel-full differences from sequential whole | 0 |
| Forward whole-replacement differences | 0 |
| Reverse whole-replacement differences | 0 |
| Forward scoped-replacement differences | 71 |
| Reverse scoped-replacement differences | 72 |
| Total mode/fixture scoped divergences | 143 |

The divergences include ordinary single-document examples, so they are not explained by cross-file
availability alone. They prove that the scoped engine is observably different and insertion-order
sensitive. This is sufficient to reject it as a semantic path.

The probe compared complete semantic S-expressions, so it is a coarse alarm rather than the final
oracle. It did **not** implement the corrected Pilot scope rules and therefore does not quantify
the behavior changes caused by that correction. The implementation sequence below requires a
reference-level differential harness and a temporary corrected resolver prototype before the
behavior cutover. The resulting fixture-by-fixture classification is a landing gate, not an open
implementation choice.

### 3.3 Attribute typing is already authored correctly

The kickoff brief's statement that attribute producers write typing only into the presentation
map is stale relative to the target base. Commit `950bdaec` ("Own relationship targets as declared
semantic facts") made `add_typing_edge_if_exists` record every authored target in
`DeclaredRelationshipFacts::typing` before attempting resolution. Attribute producers use that
path.

Commit `d0414983` changed only the `symbols.rs` reader to consume declared typing facts; it made no
graph-construction change, and the exact compatibility corpus passed unchanged. Its later revert
`b91a20d9` restored the presentation fallback but did not invalidate the evidence. Therefore:

- no attribute-specific relationship kind or producer repair is needed;
- the previously reported fixture flip is not reproducible on the target base;
- the remaining work is resolver correction plus consumer cleanup;
- removing the presentation fallback must still have a focused regression and zero unexplained
  corpus changes.

### 3.4 Pilot-derived scope matrix

The semantic rules below come from the sibling OMG Pilot implementation, not from current Spec42
behavior:

- `KerMLScopeProvider.xtend:86-134` dispatches scope by reference relationship kind.
- `KerMLScopeProvider.xtend:137-173` walks the namespace chain and reaches global scope only at the
  root.
- `KerMLScope.xtend` implements owned, inherited/general, imported, visibility, and redefinition
  behavior.
- `NamespaceUtil.java::getNonExpressionNamespaceFor` skips feature values, instantiation
  expressions, and feature-reference expressions.
- `SysMLScopeProvider.xtend` inherits the KerML rules and adds transition-specific cases.
- `KerML.xtext` and `SysML.xtext` establish the target metaclasses: FeatureTyping and general
  Specialization target `Type`; the subsetting family targets `Feature`; namespace and membership
  imports target `Namespace` and `Membership` respectively.

| Authored reference | Scope origin | Lookup mode | Metamodel domain |
|---|---|---|---|
| FeatureTyping | non-expression namespace | standard | `Type` |
| FeatureTyping of `subject ... : T` | non-expression namespace | standard | `Type` |
| General Specialization | owning namespace | standard | `Type` |
| Subsetting | owning namespace | standard | `Feature` |
| CrossSubsetting | owning namespace | standard | `Feature` |
| Redefinition | owning namespace | inherited-first | `Feature` |
| ReferenceSubsetting | owning namespace | standard | `Feature` |
| Connector-end ReferenceSubsetting | connector's owning namespace | standard | `Feature` |
| Transition ReferenceSubsetting | SysML transition-relative origin | standard | `Feature` |
| NamespaceImport | import-owning namespace | import | `Namespace` |
| MembershipImport | import-owning namespace | import | `Membership` |

A verified requirement is modeled by `RequirementVerificationUsage` with an owned
ReferenceSubsetting. It must resolve using ReferenceSubsetting semantics. The enclosing case's
`Subject` edge is a derived projection of that result, not a separately resolved authored name.

Target-domain filtering and construct validation are different operations. For example,
FeatureTyping resolves within the broad `Type` metaclass domain. The attribute validator may then
reject a resolved part definition as an invalid attribute type. Resolution must not remove an
incompatible inner-scope declaration and fall through to a compatible outer declaration.

## 4. Comparison with the sibling `sysml-compiler`

The sibling repository at `../sysml-compiler` has already performed a resolver replacement. Its
current architecture and migration history are useful evidence, but its implementation is not the
semantic authority for Spec42.

### 4.1 Approach taken there

`src/analyse/canonicalize.zig` owns a whole-project `ModelContext` with this lifecycle:

```text
canonicalizeFile x N -> normalize/implied facts -> freeze SemGraph
    -> build ResolutionDb -> solve -> ResolutionView
    -> diagnostics / validation / evaluation
    -> optional export materialization
```

`src/analyse/resolution/Db.zig` builds immutable parent/name/edge indexes over the frozen graph. It
stores nine derived families in typed dense slot arrays and recomputes them in whole-model passes
until no slot changes. Relationship destinations remain in a resolution overlay during analysis.
Consumers use `src/analyse/resolution/Query.zig::ResolutionView` for resolved targets, effective
names, supertypes, feature types, and scope lookup.

The sibling does not make its canonically ordered fact lists double as its normal query data
structures. `Index.zig` compiles an ordinal-indexed parent array and a namespace/name hash map;
`EdgeIndex.zig` compiles graph edges into CSR adjacency so a node's outgoing edges are a direct
slice; and `TypeQuery.zig` adds snapshot-local memoization for transitive conformance questions.
Those products are disposable views over a frozen model. Building them may scan the model once,
but repeated consumers do not rescan every node, edge, or resolution slot to answer a local query.

Its editor coordinator still rebuilds the unified semantic model after a source edit. Content
caching avoids reparsing unchanged files, but the public cache documentation explicitly says that
canonicalization and import resolution are not cached. A separate `StdlibSummary` serializes only
parent and namespace-name indexes for a pinned, precompiled standard-library graph.

The migration intentionally ended with deletion. Commit `d5b9c6a5` removed the legacy single-file
API, compatibility bridge, `rebuildIndex`, diagnostic refresh, and second-solve lifecycle. Its
commit record notes that the second solve had introduced a false unresolved-name diagnostic.

### 4.2 What Spec42 adopts

- One whole-model context and one resolver owner.
- A stable pre-resolution semantic input.
- Typed derived state held outside authored facts.
- A narrow read-only view used by every semantic consumer.
- A fixed-point solver rather than graph mutation during lookup.
- Correctness-first full semantic rebuilds as the supported execution model.
- Deletion of the legacy path after consumer migration, with no permanent bridge.
- Disposable, purpose-built indexes that can be rebuilt from authoritative graph and resolution
  facts and make the documented downstream queries proportional to their result sets.

### 4.3 What Spec42 must improve

The sibling's `Db.Result` is only `resolved`, `not_found`, or opaque `ambiguous`; it does not retain
ambiguous candidates. Several `ResolutionView` methods return `null` for both absence and
ambiguity, requiring a separate ambiguity query. Spec42 must publish one exhaustive typed outcome
with all candidates.

The sibling uses a fixed limit of 1,000 passes but logs a warning and freezes the database if the
solver still changed on the final pass. Spec42 must return a typed failure and must not publish that
state as settled.

The sibling represents unresolved relationship destinations with generic `element` placeholder
nodes. Spec42 already has source-faithful `DeclaredRelationshipTarget` facts; those remain the
authored representation, and unresolved outcomes remain typed resolution facts.

The sibling's export materializer temporarily unfreezes and rewrites the graph. Spec42 exporters
will project from the coherent semantic graph plus `ResolutionState`; they will not mutate the
published model or trigger another solve.

Finally, the sibling's persistent resolution summary is safe only as a paired, pinned stdlib
artifact. It is not a precedent for reusing mutable workspace semantics without a complete content
root, algorithm version, configuration identity, and invalidation contract.

Spec42 also does not adopt the sibling view's raw `edgeIndex()` escape hatch. A consumer that needs
a new semantic relationship query extends the owning typed view. Exposing a general index would
allow consumers to rebuild semantic classifications and would make its current storage layout a
public contract.

## 5. Canonical semantic model

### 5.1 Phase ownership

The target pipeline is:

```text
immutable sources
    -> parse/recovery results
    -> immutable per-document ParsedDocument values
    -> direct canonicalization into one private SemanticModelBuilder
    -> ResolutionDb.solve() over that builder's stable authored tables
    -> freeze once into one coherent settled semantic publication
    -> diagnostics / evaluation / projections through typed query services
```

Canonicalization walks each parser document and directly creates containment, memberships,
syntax-derived semantic facts, and authored relationship slots in the model builder. There is no
parser-shaped authored-model or document-fragment representation between the parser and semantic
model. The builder must not decide final semantic targets: import traversal, lexical scope,
ambiguity, and relationship target resolution belong exclusively to `ResolutionDb`.

Canonicalization mints every declaration identity and authored-reference slot at the point where
it recognizes the corresponding semantic construct. Later resolution uses those existing slots;
it does not invent publication-visible semantic identities. This preserves a checked
construct-to-slot correspondence for diagnostics and navigation while still allowing the solver
to allocate private temporary work slots that never escape in results.

The finalization phase constructs resolution state in isolation and freezes `SemanticModel` only
after a successful solve. `SemanticModelBuilder`, solver slots, and partial indexes are
build-private types; they cannot be handed to semantic consumers. A failed or
cancelled solve leaves the prior coherent publication active. A new build with no prior
publication returns a failed construction result.

### 5.2 Public contracts

The implementation will introduce equivalent Rust contracts to:

```rust
pub struct SemanticModel {
    identity: SemanticModelIdentity,
    structural_graph: SemanticGraph,
    resolution: ResolutionState,
    evaluation: Option<EvaluationState>,
    query_indexes: SemanticQueryIndexes,
    phase: SemanticPhase,
    completeness: SemanticCompleteness,
}

pub struct AuthoredReferenceId {
    pub source: NodeId,
    pub kind: ReferenceKind,
    pub authored_ordinal: u32,
}

pub enum ReferenceKind {
    FeatureTyping,
    Specialization,
    Subsetting,
    Redefinition,
    ReferenceSubsetting,
    CrossSubsetting,
    NamespaceImport,
    MembershipImport,
}

pub enum ResolutionOutcome {
    Resolved { target: NodeId },
    Unresolved,
    Ambiguous { candidates: Vec<NodeId> },
}

pub struct ResolutionFact {
    pub reference: AuthoredReferenceId,
    pub outcome: ResolutionOutcome,
}
```

`structural_graph` contains authored facts plus any pre-resolution normalized/implied facts, with
their provenance kept distinct. It is stable before `ResolutionDb` starts. Resolution-derived and
evaluation-derived facts do not write into it.

`SemanticModel` fields are private and immutable after construction. It exposes source-fidelity
queries over `structural_graph` and semantic queries through `view()`. Mutation functions accept a
build-private graph/builder, never a published `SemanticModel`. Cache encoding serializes the
authoritative structural graph, `ResolutionState`, and eligible `EvaluationState` as one coherent
record; `SemanticQueryIndexes` are disposable and are rebuilt and validated on decode.

`SemanticModelIdentity` commits the immutable source snapshot and semantic configuration used by
the build. The workspace owner atomically installs a completed asynchronous model only if that
identity is still current. `SemanticPhase` distinguishes resolved from evaluated state;
`SemanticCompleteness` distinguishes complete input from explicit editor recovery and other
supported incomplete input. Recovery never masquerades as complete and is not persistently cached.

`ImmutableSourceSnapshot` is the exact, canonically ordered set of captured documents, bytes, and
content identities used by the build. `SemanticConfiguration` contains every non-source input that
can affect construction, scope, normalization, resolution, validation, or evaluation. Both are
part of `SemanticModelIdentity`; a path, timestamp, or workspace revision counter is not sufficient
identity.

`ReferenceKind` is the resolver input category. It is deliberately not a caller-supplied list of
concrete `ElementKind` values. The resolution module exhaustively maps it to a `ScopeOrigin`,
`LookupMode`, and broad `ReferenceDomain`. Adding a new reference kind requires extending this
mapping and its scope tests.

The enum shown above is the minimum relationship set, not permission to leave other semantic
references as string lookups. Before the old helpers are deleted, the implementation inventory
classifies every call site into exactly one of:

1. an existing authored reference fact resolved by one of these metamodel relationship kinds;
2. a missing authored/derived semantic fact whose owning graph contract must be extended;
3. a downstream validation of an already resolved fact;
4. a non-publishing interactive `LookupRole` query over frozen scope products.

No diagnostic may remain in category 4 for a source-authored semantic reference. If it needs an
answer that is absent from `ResolutionState`, the owning authored fact and construction path are
extended instead of preserving an ad hoc lookup.

`ResolutionState` owns a canonically ordered collection of `ResolutionFact`, derived relationship
facts with provenance, import outcomes, and the fixed-point products that are themselves semantic
facts. Outgoing/incoming adjacency, scope name maps, and lookup tables are private accelerators
rebuilt from those authoritative facts.

Canonical collections are suitable for deterministic serialization, inspection, and rebuilding
indexes; they are not the default downstream query representation. Before publication,
`SemanticQueryIndexes` compiles purpose-built immutable views for authored-reference outcomes,
relationship adjacency by endpoint and kind, ownership, namespace membership, document/range
lookup, direct typing/specialization, and any other query used repeatedly by a supported consumer.
Indexes may use dense ordinal arrays, CSR slices, interval indexes, or maps as appropriate. Their
storage remains private, and query methods return typed outcomes or borrowed typed result slices.

“Disposable” describes authority and lifetime, not necessarily evaluation strategy. Foundational
indexes used by ordinary local queries are populated eagerly before the publication barrier so the
first consumer cannot trigger an unreported whole-model traversal. Rich typed query services may
memoize expensive transitive derivations lazily within one immutable snapshot, provided cold and
warm answers are identical, cache absence cannot change semantics, and cache mutation is hidden
behind the query contract. Lazy memoization is never lazy reference resolution.

A complete scan is permitted while constructing or validating one publication when its cost is
reported as construction work. A `ResolutionView` method, diagnostic category, editor request,
projection, or generator must not linearly scan all nodes, all resolution facts, or all
relationships to answer a source-, node-, reference-, or relationship-local question. Adding such
a consumer requires adding the missing owner-level index or typed projection at the publication
barrier; snapshot-local memoization may then accelerate genuinely transitive queries without
becoming authoritative.

`ResolutionView<'a>` borrows the model's structural graph and owned `ResolutionState`. It provides:

- `outcome(AuthoredReferenceId) -> &ResolutionOutcome`;
- resolved outgoing/incoming relationship iteration by kind;
- local, visible, context, and qualified-name lookup returning typed outcomes;
- direct supertypes, feature types, inherited members, and effective names;
- derived case-subject relationships with explicit provenance.

The downstream query complexity contract is:

- outcome by authored reference: expected `O(1)`;
- outgoing/incoming relationships: `O(number of returned relationships)` after indexed lookup;
- local/context name lookup: expected `O(1)` per visited namespace;
- qualified lookup: `O(number of segments + returned candidates)`, excluding explicitly bounded
  inherited/import closure work already settled into the model;
- direct supertypes, feature types, and effective membership: `O(number of returned facts)`.

Complexity tests instrument visited index entries as well as returned values. Repeating a query
must not increase work in proportion to total model size, and exercising one consumer before
another must not change either result or required semantic work. Wall-clock benchmarks complement
these structural checks but do not replace them.

No `ResolutionView` query runs a convergence pass, mutates memo state needed for correctness, or
changes publication completeness. Harmless allocation-free cache warming is unnecessary because
the publication builds its query indexes before becoming visible.

### 5.2.1 Query crate and enforceable consumer boundary

The implementation packages the published model and all supported semantic query services behind
a `sysml_query` crate. `sysml_query` may depend on `sysml_model` to construct its opaque published
state, but diagnostics, language service, LSP, workspace presentation, generators, and other
semantic consumers do not depend directly on `sysml_model`. Rust does not make transitive
dependencies nameable, so this manifest boundary makes raw graph, resolver, fact-collection, and
index types unavailable to those consumers even when an implementation type must be public for
the owning crate-to-crate construction seam.

`sysml_query` owns the private `SemanticQueryIndexes` representation and exposes cohesive typed
services rather than one generic inventory API. Its public surface may re-export neutral semantic
identities, result enums, provenance, ranges, and immutable model handles required by callers. It
does not re-export `SemanticGraph`, graph nodes, resolver databases, builders, raw index types,
generic attribute maps, complete resolution-fact collections, or constructors that accept a
partly settled model. Consumers cannot request an index handle or iterate general storage and
classify it themselves. A missing consumer operation extends the owning typed query service.

A repository architecture check reads Cargo workspace metadata and fails when a semantic consumer
adds a direct dependency on `sysml_model` or another implementation crate outside an explicit
owner/tool allowlist. Compile-fail contract tests prove that index types, fields, raw collections,
and construction functions are inaccessible through `sysml_query`. A syntax-aware API check also
rejects forbidden public escape-hatch shapes such as raw graph/index accessors; text search alone
is not the authority. These checks run in the normal workspace gate.

This boundary cannot mathematically prove that arbitrary code never combines legitimate typed
answers into a new derived meaning. The query API therefore avoids overly general inventories,
and architectural review still checks for downstream semantic classification. The enforceable
guarantee is that supported consumers cannot access raw semantic storage or resolution machinery
and cannot add such access without a manifest/API gate failure.

The dependency-complete production migration inventory is maintained with the query facade in
`crates/sysml_query/PRODUCTION_CUTOVER.md`. It identifies the typed services that must replace
`HostWorkspaceSnapshot`, server, and LSP graph access before those consumers leave the transitional
allowlist; it does not authorize a dual mutable-graph/`PublishedModel` publication.

There is no public resolver accepting an arbitrary context string, container prefix, or concrete
allowed-kind slice. Diagnostics query the authored reference's canonical outcome. Interactive
language-service lookup uses a separate typed `LookupRole` (for example navigation symbol,
expression first segment, or visible member); the resolution module exhaustively maps that role to
scope origin and broad metamodel domain. It reads the frozen scope products through
`ResolutionView` and cannot create or alter a published `ResolutionFact`.

### 5.3 Outcomes and edges

Each entry in a declared relationship vector has identity `(source, kind, authored ordinal)`. The
ordinal preserves repeated authored clauses even if two clauses resolve to the same target.

- `Resolved` contributes one relationship to the resolved adjacency projection.
- `Unresolved` contributes no resolved relationship but remains an explicit published fact.
- `Ambiguous` contributes no resolved relationship and retains all distinct candidates.

Candidates are deduplicated by `NodeId` and ordered by the B3 `NodeId::Ord` policy. On upstream
before B3 is integrated, the resolver uses the same explicit comparator: normalized URI, then
qualified name. Request evaluation and published facts are ordered by source `NodeId`, reference
kind discriminant, and authored ordinal.

This contract subsumes the current `resolve_subsetting_family_target` gap. The subsetting family
cannot silently select the first qualified-name bucket entry.

Imports reuse `ResolutionOutcome`. Import-only states remain explicit in a wrapper:
`NotApplicable`, `Applicable(ResolutionOutcome)`, and `UnsupportedFiltered`. Import lookup consumes
those canonical results instead of separately resolving the same target.

### 5.4 Subject and provenance

`Subject` is an effective case-level relationship, not a name-resolution mode:

- a subject declaration resolves its ordinary FeatureTyping fact;
- a verified-requirement usage resolves its ordinary ReferenceSubsetting fact;
- one case-subject derivation owner projects the corresponding case-level `Subject` relationship.

The projected relationship records a typed derivation rule. It must not carry authored provenance
or inspect presentation attributes to rediscover its target.

The same ownership rule applies to every derived semantic family. If an analysis-typing,
derivation, implied-relationship, or normalization result can affect scope, imports, inheritance,
or another reference outcome, it is a declared `ResolutionDb` family inside the fixed point. A
post-resolution validator/evaluator may consume resolved facts but cannot publish a fact that would
have changed resolution. Discovering such a feedback edge during the Step 1 inventory requires
moving that derivation into the solver before cutover.

### 5.5 Fixed-point execution

Inheritance, redefinition, imports, feature typing, and effective membership are mutually
dependent. A provisional absence is not a published `Unresolved`: an outer candidate cannot be
accepted while a higher-precedence inherited or imported scope is still incomplete. The working
solver therefore has an internal state that is deliberately richer than `ResolutionOutcome`:

```rust
enum WorkingOutcome {
    Pending { dependencies: Vec<ResolutionDependency> },
    Final(ResolutionOutcome),
}
```

`Pending` never crosses the publication boundary. Each solver family declares the facts it owns,
the prerequisite families it reads, and what constitutes completeness. Candidate discovery may
grow monotonically while a scope is pending; precedence and ambiguity are finalized only after all
higher-precedence candidate sources for that scope are complete. A family that would need to
retract a final answer must instead remain pending or be stratified after the prerequisite family.

`ResolutionDb` evaluates these typed families against one stable authored graph and the previous
complete pass:

1. clear only per-pass computation state;
2. evaluate every family from the same immutable previous-pass state, so requests in one pass
   cannot observe sibling requests' new results;
3. evaluate all authored references in canonical request order;
4. compute effective membership and relationship-derived families without mutating the graph;
5. publish the isolated working pass products in canonical order;
6. stop when no working value changes and every required published reference is final.

The initial working state contains authored/structural facts and `Pending` for every reference
whose answer requires resolution. It does not seed references with graph-wide fallback targets or
results from a prior model identity. Pre-resolved immutable library products may be seeded only
after their complete identity and invariants are verified.

The solver fingerprints the complete ordered working state. An immediately repeated state with no
pending required reference is convergence. A previously seen non-adjacent state is an oscillation.
An unchanged state that still contains required `Pending` values is a dependency deadlock. The
solver also has an explicit `MAX_RESOLUTION_PASSES = 1_000` safety bound. Oscillation, deadlock, or
the safety bound returns `ResolutionFailure::DidNotConverge { passes, changing_families,
pending_references }`. It does not create `SemanticModel`, emit ordinary unresolved diagnostics
from the partial state, or mark the publication settled.

Before implementation cutover, the solver design must classify every mutually recursive family as
monotone-with-completeness or explicitly stratified. If a supported Pilot case requires two
different stable answers from the same authored graph and configuration, this fixed-point design
is falsified and must be revised; canonical iteration order is not permission to choose an
arbitrary fixed point.

## 6. Scope algorithm

For each request, the resolver derives the scope origin from `ReferenceKind` and the authored
source node. It then applies these rules:

1. Normalize the authored qualified name without losing its source spelling or range.
2. Resolve the first segment in the innermost applicable namespace.
3. Within that namespace, consider owned members, then inherited/general members, then imports.
4. Apply membership and import visibility at the boundary where it is observed. Public re-exports
   remain visible; private/protected members do not masquerade as public external results.
5. If that namespace has a binding at the relevant precedence tier, it shadows outer namespaces.
   Multiple distinct candidates at that tier are ambiguous.
6. If it has no binding, walk the parent namespace chain. Consult global roots only after reaching
   the namespace root.
7. Resolve later qualified-name segments within the selected namespace/type using visible lookup.
   Detect import and inheritance cycles by semantic identity.
8. Apply broad metamodel-domain filtering as part of the reference contract, then run
   construct-specific compatibility validation as a later phase.

Absolute names begin at the global root. There is no graph-wide unique-name fallback for a
relative simple name. Source URI and display text are not scope identities.

Conjugated port typing resolves the authored base port definition through ordinary FeatureTyping
scope, then applies the canonical conjugated-port semantic rule. Whole and parallel paths do not
own separate `~P` handling.

## 7. Clean API migration

### 7.1 Replacement build surface

Replace the public build/link/patch family with one protocol-neutral service equivalent to:

```rust
pub enum ConstructionStrategy {
    Sequential,
    Parallel,
}

pub enum EvaluationPolicy {
    ResolvedOnly,
    Evaluate,
}

pub struct SemanticBuildRequest {
    pub sources: ImmutableSourceSnapshot,
    pub construction: ConstructionStrategy,
    pub evaluation: EvaluationPolicy,
    pub configuration: SemanticConfiguration,
}

pub fn build_semantic_model(
    request: SemanticBuildRequest,
) -> Result<SemanticModel, SemanticBuildFailure>;
```

`ConstructionStrategy` controls parsing and semantic compilation from immutable per-document
`ParsedDocument` values. It is not passed to `ResolutionDb` and cannot affect `ResolutionState`.
Both strategies retain the same parser documents, compile semantic entities in canonical source
order, and call the same resolver.

`EvaluationPolicy::ResolvedOnly` may omit expression evaluation, but it never omits semantic
resolution. It publishes a coherent resolved phase with explicit completeness. `Evaluate`
publishes a separate later model including evaluation facts. There is no boolean whose false value
returns a graph with an unspecified mixture of local and cross-document relationships.

An editor replacement captures a new `ImmutableSourceSnapshot`, reuses content-addressed immutable
`ParsedDocument` results where the source and parser contract identity remain valid, and calls this
service. The returned
`SemanticModel` is atomically swapped into the workspace. Readers retain the prior model while the
new one is building. If parsing, resolution, evaluation, or cancellation prevents the requested
phase from completing, no partially mutated replacement is exposed.

### 7.2 Deleted entry points and machinery

Delete in the same upstream landing:

- `build_and_link_graph` and `build_and_link_graph_parallel` as public semantic APIs;
- `link_parsed_documents_parallel` and `link_parsed_documents_parallel_from` as public semantic
  APIs;
- `patch_graph_for_document` and its in-place mutation contract;
- `patch_graph_for_document_scoped`;
- `finalize_and_evaluate`, `finalize_and_evaluate_frontier`, and public finalization of arbitrary
  mutable graphs;
- `add_cross_document_edges_for_uri`;
- `resolve_cross_document_edges_for_uri`;
- relationship-frontier discovery, reverse-frontier state, and scoped refresh code;
- scoped-resolution exports from `sysml_model::lib`;
- scoped-only parity tests and the manual full-vs-frontier benchmark;
- `resolve_type_reference_targets` as a public API;
- `resolve_type_target_in_workspace` and other first-candidate adapters;
- caller-owned `allowed_kinds` resolution parameters;
- diagnostic and language-service graph-wide fallbacks.

Also stop exporting `SemanticGraph`, its mutation methods, and direct graph constructors as a
completed semantic surface. The mutable graph representation and builders become crate-private.
Downstream crates receive `SemanticModel`; source-fidelity queries are methods on that model and
resolved relationship, scope, and type queries are methods on `ResolutionView`. Existing direct
resolved-edge adjacency callers migrate to indexed `ResolutionView` projections, so retaining a
convenient graph method cannot become a second resolved-fact authority.

Do not replace these with deprecated functions that call the whole resolver. A call site must
migrate to `build_semantic_model`, an immutable `SemanticModel`, or `ResolutionView`. Low-level
canonicalization becomes crate-private and consumes retained `ParsedDocument` values directly into
the one `SemanticModelBuilder`; its private construction state cannot be used as a semantic model.
After the cutover, a repository
search for the deleted semantic symbols must return no production or test references.

The word "scoped" remains valid in unrelated presentation features such as diagram view scoping;
the deletion applies specifically to semantic name/relationship resolution.

### 7.3 Structural guard against a third engine

- Scope traversal and `ResolutionDb` constructors are crate-private.
- `SemanticModel` is the only published semantic state and `ResolutionView` is its semantic lookup
  surface.
- Concrete reference-kind/domain mappings are exhaustive and live in the resolution module.
- Document builders can record authored facts but cannot install `ResolutionOutcome` or resolved
  relationship adjacency.
- Diagnostics and hosts receive a settled model/view rather than a mutable graph or raw lookup
  indexes.
- An architecture test compares all retained construction paths at the authored-reference level.
- Code review/CI includes a targeted search that rejects public semantic functions accepting
  arbitrary allowed-kind slices or selecting the first candidate from a resolution result.

## 8. Whole-model rebuild policy, research boundary, and cache implications

### 8.1 Supported semantic rebuild policy

The supported implementation does no scoped semantic reuse. An edit may reuse immutable parser
documents and their source arenas, but semantic compilation assigns a new whole-model set of
model-owned semantic IDs and solves the complete model. The canonical full solve is the semantic
execution model, not a fallback.

For editor latency, syntax/recovery diagnostics may be published under their own explicit source
revision and incomplete phase. If supported recovery input can be resolved coherently, the build
may publish an immutable `SemanticModel` marked with the precise recovery completeness; it cannot
masquerade as complete and is never persistently cached. If the requested semantic phase cannot be
completed under the recovery contract, semantic consumers keep using the previous coherent model.
The workspace policy chooses explicitly between those two observable states; it never exposes a
partly mutated graph. A later evaluated publication for the same source revision is another
immutable model and atomically supersedes the resolved-only one; evaluation never mutates the
visible model in place.

The whole-model rebuild and resolution policy matches the sibling compiler's current semantic
policy. It is the supported architecture, not a temporary fallback awaiting an incremental
resolver. The explicit recovery-completeness contract is Spec42's publication rule and does not
claim equivalence with every sibling editor-recovery behavior.

### 8.2 Incremental semantic resolution is a separate research question

There is no established precedent in this repository, the Pilot, or the sibling compiler for
incrementally publishing a complete SysML semantic model with dependency-complete invalidation.
The Pilot demonstrates lazy contextual query evaluation, not atomic incremental reconstruction of
all semantic outcomes. The sibling compiler deliberately rebuilds and resolves the whole semantic
model.

For SysML, a small edit can have a model-wide semantic closure:

- adding a declaration invalidates negative lookups in inner, outer, inherited, and importing
  scopes;
- import visibility and re-export changes propagate transitively;
- specialization and typing change inherited effective membership;
- redefinition changes effective names and suppression;
- qualified feature lookup can depend on resolved typing;
- those facts participate in recursive cycles and can alter which dependency edges exist.

It is therefore unknown whether a correct dependency graph would usually select substantially less
work than the batch solver. Maintaining that graph may cost more time and memory than solving the
model, while creating a second difficult correctness surface. The cache does not require it:
whole-model cache identity commits the complete source/configuration snapshot and stores one
settled publication.

Safe reuse in this design stops before semantic compilation and resolution:

- content-addressed source acquisition and immutable parser `ParsedDocument` results, including
  their source storage and parser arena;
- parser-local typed identities only with their owning `DocumentId` and parser contract identity;
- pinned standard-library structural or resolved products with complete verified identity;
- semantic indexes rebuilt for the new whole-model input.

Settled workspace `ResolutionState` is not reused across source snapshots.

Any future incremental-resolution proposal starts as an independent research project and must
first produce evidence, not production APIs. At minimum it must:

1. instrument the batch solver's actual positive, negative, import, inheritance, typing, and
   redefinition dependencies;
2. replay representative real editor histories and measure affected strongly connected components
   and invalidation closure sizes;
3. report median, tail, and worst-case affected-reference percentages plus dependency-index memory
   and maintenance cost;
4. demonstrate that the same resolver equations produce byte-equivalent `ResolutionState` to a
   clean batch solve after every edit, including superseded and out-of-order work;
5. show a material end-to-end latency benefit after including dependency tracking and publication
   costs.

If closures commonly approach the whole model, the research result is that incremental semantic
resolution is impractical, and the batch architecture remains final. Until all five conditions are
met, no dependency-trace schema, reverse invalidation index, owner revision API, cache key, or
compatibility hook is added to production.

### 8.3 `SemanticModel` and persistent cache identity

Scope correctness adds no path-derived identity dimension. The cache branch's `RootDigest` already
commits all admitted source bytes, normalized identities, source roles, and library-root ordering;
configuration identity commits scope-affecting policy. The new resolver does require:

- a semantic contract version bump when integrated;
- serialization of structural graph facts and canonical `ResolutionState` as one coherent artifact;
- invariant validation that every outcome references admitted nodes and the matching authored
  reference site;
- rebuilding disposable indexes after decode;
- rejecting partial, failed, non-converged, cancelled, or recovery-produced states as settled
  cache hits.

Cold and decoded publications with the same root/configuration/contract identity must expose the
same outcomes, candidates, relationships, diagnostics, and ordering. The cache never chooses a
resolution engine and never materializes missing outcomes by guessing.

The sibling compiler's parser cache and pinned stdlib index summary do not change this contract:
they do not cache mutable project semantic analysis.

This is also the boundary for the existing [cache work in PR #73](https://github.com/elan8/spec42/pull/73).
That work may cache or reuse immutable source acquisition and parser `ParsedDocument` results when
their content, parser contract, source role, and document identity match. It must not treat a
parser arena, parser-local reference ID, or decoded syntax document as a published semantic model.
Every cache hit still runs semantic compilation into fresh model-owned dense IDs and either runs
the canonical whole-model solve or decodes one complete, identity-validated semantic publication.
PR #73 therefore cannot introduce fragment-arena remapping, a second semantic authority, or a
scoped resolution shortcut; its artifact boundary ends before semantic resolution.

## 9. Diagnostics and consumers

Diagnostics are projections of canonical outcomes:

- `Resolved` produces no unresolved/ambiguous diagnostic. Later compatibility validation may
  diagnose an invalid target kind at the authored target range.
- `Unresolved` produces the existing stable unresolved code at the exact authored range.
- `Ambiguous` produces the stable ambiguity code and related information for every canonically
  ordered candidate.
- a failed solver produces a construction/publication failure, not one unresolved diagnostic per
  reference.

Hover and definition lookup return a unique target only for `Resolved`. If a protocol supports
multiple definition locations, ambiguity may project every canonical candidate; otherwise it
remains explicitly ambiguous rather than selecting one. Symbol classification, conformance checks,
and generators consume resolved relationships through `ResolutionView`.

The remaining `symbols.rs` `attributeType`/`dataType`/`type` fallback is removed only after its
focused declared-typing test and the complete corpus prove parity. No replacement presentation
fallback is added.

## 10. Implementation sequence

The upstream work should be one migration branch and one final semantic cutover, organized into
reviewable commits. Intermediate commits may introduce new types before deletion, but no release or
merge may expose two supported resolution systems.

### 10.1 Minimum ownership-complete cutover closure

The immutable resolver cannot be cut over one convenient reference kind at a time when that kind's
scope depends on facts still owned by the mutable graph. The minimum semantic cutover includes all
facts that participate in scope, effective naming, or type-directed continuation:

- dense declarations, ownership, membership, visibility, declared names, and short names;
- alias targets and alias bindings;
- specialization, typing (including conjugation), subsetting, redefinition,
  reference-subsetting, cross-subsetting, and intersecting or an explicit unsupported outcome;
- namespace, membership, recursive, and expose imports with their typed shape and visibility;
- connection, bind, satisfy, allocate, flow, and succession-flow endpoint pairs;
- dependency, derivation, verified-requirement, case-subject, perform, transition, and initial-state
  inputs whose results affect canonical relationships;
- typed construction-known relationships such as annotation and port conjugation, which must not
  be converted back into textual lookup requests; and
- multiplicity, feature values/properties, and owned expressions required for downstream semantic
  parity even when they are not lexical-lookup inputs.

An exhaustive parser-to-semantic canonicalizer accounts for every AST variant while borrowing the
retained `ParsedDocument` and its arena. Parser fields that lack a typed path, absolute scope,
separator semantics, or exact target provenance publish an explicit unsupported construction fact;
adapters do not split a `String`, inspect a sentinel spelling, use a debug formatter, or assign the
containing range to missing segment spans. Canonicalization assigns model-owned dense IDs only to
entities that enter the semantic model and interns normalized cross-document names; it does not
copy or remap all parser nodes, paths, expressions, or source storage, and it does not first create
a parser-shaped authored IR. In particular, alias,
dependency, derivation, view-body satisfy, intersecting, and conjugated typing cannot disappear
because the previous mutable builder failed to feed them into its canonical fact collection.

The solve order reflects the actual dependency closure:

1. retain the immutable `ParsedDocument` set in canonical `DocumentId` order and canonicalize
   semantic declarations, memberships, bindings, source roles, authored reference slots, and exact
   provenance directly into dense model-owned storage;
2. compile parent/child, local-binding, authored-reference, import, standard-library, and direct
   structural-relationship indexes over those semantic entities;
3. solve the cyclic scope component containing alias binding, specialization, redefinition and
   effective names, inherited membership, imports, exports, visibility, and recursion;
4. solve typing and the remaining simple authored paths against the settled scope products;
5. resolve expression endpoints by lexical first-segment lookup followed by typed feature-chain
   continuation; and
6. derive construction-known and settlement-derived relationships, then build immutable outcome,
   adjacency, diagnostic, navigation, and query indexes at the publication barrier.

No intermediate authored model or semantic publication is placed beside the graph-backed resolver.
Preparatory parser/document and direct-canonicalization infrastructure may land privately, but the
semantic cutover replaces the whole closure above and deletes its graph-based construction,
resolution, and consumer paths together.

### Step 1: evidence harness and blast-radius gate

- Inventory every caller of the old resolution helpers and classify it using the four categories
  in §5.2. The inventory is committed with the design evidence so deletion cannot strand a private
  semantic lookup.
- Add a test-only canonical reference projection containing authored reference identity, outcome,
  candidates, and projected relationship.
- Keep snapshot sections purpose-specific: `SMG` renders semantic identity, typed facts,
  provenance, outcomes, candidates, and relationships without routine element/reference ranges;
  exact observable locations are owned by `DIAGNOSTICS` and `NAVIGATION`. Add an `SMG` span only
  when the span is itself a named semantic fact not covered by either location-sensitive section.
- Compare sequential, parallel, forward/reverse replacement, and the old scoped path before its
  deletion.
- Build the corrected scope resolver as an isolated prototype using the target contracts.
- Run every discovered semantic snapshot and focused multi-document cases.
- Publish a fixture-by-fixture report classifying every output change as Pilot-backed correction,
  existing defect preserved, or regression.
- Stop if any change is unexplained. Do not begin the behavior cutover without this report.

### Step 2: contracts and immutable resolver core

- Add `SemanticModel`, the build-private graph/builder, `AuthoredReferenceId`, `ReferenceKind`,
  `LookupRole`, `ResolutionOutcome`, `ResolutionFact`, `ResolutionState`, `ResolutionDb`, and
  `ResolutionView`.
- Implement the Pilot scope matrix, import traversal, candidate retention, canonical ordering,
  pending/completeness-aware fixed-point families, repeated-state detection, and non-convergence
  failure.
- Integrate `ImportTargetResolution` with the shared outcome contract.
- Add complete pre-publication query indexes and complexity-focused tests proving downstream
  queries do not run the solver.
- Add owning-layer unit tests before migrating consumers.

### Step 3: authored producers and relationship derivation

- Make builders record authored facts only, and close every missing-fact result from the Step 1
  inventory, including analysis, flow, metadata, requirement/use-case, and derivation consumers.
- Route all subsetting-family resolution through `ResolutionDb`.
- Resolve subject typing and verified-requirement ReferenceSubsetting normally; derive case Subject
  relationships from those facts with typed provenance.
- Make conjugated-port handling a single canonical derivation.

### Step 4: consumer migration

- Add `build_semantic_model` and migrate sequential, parallel, workspace replacement, and deferred
  evaluation onto its immutable publication contract.
- Migrate diagnostics, language services, LSP navigation/hover/symbols, evaluation, generators,
  snapshots, and projections to `ResolutionView`.
- Remove direct `.next()` selection, graph-wide name fallbacks, and caller-owned kind filters.
- Change exporters to project resolved relationships without mutating or re-solving the graph.

After the Step 2 interfaces are fixed, diagnostics fixtures, language-service migration, and the
differential harness can proceed in parallel. Graph producers and finalization remain ordered
because consumers depend on their completed contract.

### Step 5: hard deletion

- Delete every old build/link/patch, scoped/frontier, arbitrary-finalization, and name-resolution
  API listed in §7.2.
- Delete obsolete tests and replace them with retained-path/full-rebuild equivalence tests.
- Delete old public name-resolution helpers and remove their re-exports.
- Remove temporary prototype code and any transition-only adapters.
- Update architecture documentation and verify the structural guard searches.

### Step 6: upstream verification and cache handoff

- Run focused resolver, relationship, diagnostics, language-service, LSP, workspace, and corpus
  tests while iterating.
- Run the broader workspace suite and clippy. Preserve the accepted 22 pre-existing
  `snapshot_single_build` failures; do not weaken them.
- Land upstream only after the acceptance criteria below pass.
- Merge the landed upstream change into `integration/unified-cache`, resolve B1/B3/B4 integration
  against the new resolution state, bump the semantic contract, and then resume cache artifact
  design and parity work.

## 11. Verification and acceptance

### 11.1 Required focused scenarios

- nearest lexical namespace shadows outer and global declarations;
- an incompatible inner `Type` still shadows a compatible outer type, followed by validation;
- duplicate candidates at one precedence tier are ambiguous with complete ordered candidates;
- qualified names resolve segment-by-segment with visible lookup;
- public, private, protected, membership, namespace, recursive, and cyclic imports;
- inherited members, diamonds, effective names, explicit and implied redefinitions;
- Redefinition excludes owned first-scope candidates;
- all four subsetting-family kinds retain unresolved and ambiguous outcomes;
- subject declaration typing and verified-requirement ReferenceSubsetting project the same
  case-level semantics on every build path;
- connector-end and transition ReferenceSubsetting origins;
- conjugated port `~P` behavior through the one resolver;
- absolute/global lookup and duplicate roots;
- forward/reverse source order and shuffled internal traversal;
- same-pass request-order shuffling produces the same working-state sequence and final model;
- an outer candidate remains pending until incomplete inherited/imported higher-precedence scopes
  settle;
- a valid cyclic import/inheritance case either converges to the Pilot-backed result or returns the
  explicit unsupported/failure state required by its contract;
- solver convergence, repeated-state detection, safety-bound failure, and prior-publication
  preservation;
- published-model APIs cannot mutate the authored graph, resolution facts, or query indexes;
- outcome, relationship, scope, and type queries perform no solver work and meet the complexity
  contract in §5.2;
- attribute symbol classification from declared typing with no presentation fallback.

### 11.2 Path and publication parity

For every fixture and focused scenario, compare:

- authored reference identities and source ranges;
- `ResolutionOutcome` discriminants;
- complete candidate vectors;
- projected relationship kind, target, and provenance;
- diagnostics, related information, severity, and order;
- navigation/hover results;
- settled phase and completeness.

Sequential and parallel construction strategies, replacement source snapshots, cold builds, and
later decoded models must match. The comparison uses the canonical reference projection, not only
S-expression text.

### 11.3 Performance and downstream-query gate

Before implementation, capture the current whole-build baseline on representative small, medium,
and large workspace/library models. The migration benchmark records these phases separately:

- source acquisition and parsing;
- semantic compilation from retained parser documents into model-owned entities;
- `ResolutionDb` initialization and each solver family/pass;
- query-index construction;
- validation/evaluation;
- representative downstream workloads: diagnostics, hover/definition, model projection, and a
  generator traversal.

Report wall time, peak memory, resolved-reference count, solver passes, changed slots per family,
and downstream query counts. Include cold CLI construction and a burst of editor replacements in
which readers continue using the prior model.

The performance acceptance rule is architectural rather than a guessed machine-specific
millisecond number:

- downstream queries perform no resolution and scale with the documented result/segment counts;
- repeated downstream consumers do not repeat equivalent name-resolution work;
- solver time and memory scale with admitted semantic facts and reported fixed-point work, with no
  unexplained traversal-order or cache-warmth variance;
- the measured editor replacement latency is reviewed before landing and is usable on the chosen
  representative large model.

If the last condition fails, the change does not fall back to the scoped engine. Optimize the one
batch solver or its pre-resolution construction, then repeat the same semantic parity suite. The
benchmark corpus and raw measurements are committed so a concrete product latency budget can be
set from evidence rather than invented in this design.

### 11.4 Landing criteria

- The corrected-scope blast-radius report has no unexplained fixture change.
- All retained build paths produce identical reference projections.
- No old build/link/patch API, scoped semantic resolver, frontier, compatibility wrapper, or
  deleted public symbol remains.
- No public semantic lookup accepts caller-selected concrete kind arrays.
- No consumer re-resolves a published authored reference or chooses its first candidate.
- No semantic consumer can obtain a mutable or partially resolved graph in place of
  `SemanticModel`.
- No downstream crate can construct or inspect a raw graph as an alternative resolved semantic
  surface.
- Every old resolution-helper caller is classified and migrated; the inventory contains no
  unresolved owner.
- Ambiguous candidates and unresolved states survive serialization/projection where observable.
- Non-convergence cannot be published as settled.
- The performance/queryability gate in §11.3 is reported and passes.
- Attribute fallback removal has zero unexplained corpus changes.
- Focused tests, the compatibility corpus, applicable workspace tests, and
  `cargo clippy --workspace --all-targets -- -D warnings` pass at the recorded baseline.

## 12. Risks and falsifiers

| Risk or design claim | Evidence that would falsify it | Required response |
|---|---|---|
| Full resolution is a correct initial replacement | A retained construction path cannot provide the same stable authored input | Fix the construction/publication boundary; do not restore a private resolver |
| One immutable model is sufficient for downstream consumers | A required consumer cannot express its query from authored facts plus `ResolutionState` without invoking the solver | Extend the owning state/view contract; do not give the consumer a resolver or mutable graph |
| Pilot matrix matches the supported grammar | A normative clause or Pilot scope-provider case maps a listed reference differently | Update the exhaustive mapping and add the counterexample fixture before cutover |
| Broad-domain resolution plus later validation is correct | Normative evidence requires target-kind filtering to affect lexical shadowing for a specific reference | Model that rule explicitly in `ReferenceKind`; do not reintroduce caller allowlists |
| Typed outcomes subsume subsetting ambiguity | A subsetting semantic requires multiple simultaneous resolved targets rather than ambiguity | Add a distinct normative outcome/relationship cardinality with evidence |
| Subject is derived from typing/reference subsetting | Metamodel evidence shows an independently authored subject target not represented by either source fact | Add a distinct authored reference kind and provenance; do not infer from attributes |
| Pending/completeness rules define one result | A supported model has two stable results, or a final answer must retract when a prerequisite settles | Re-stratify the semantic equations or revise the solver model; iteration order cannot choose the result |
| Fixed-point families converge within the bound | A valid corpus/model reaches deadlock, oscillation, or 1,000 changing passes | Treat it as a solver failure, capture changing families and pending sites, and redesign the dependency cycle before landing |
| Full rebuild cost is acceptable for the correctness landing | Representative LSP measurements fail the §11.3 usability review | Optimize the single batch solver and pre-resolution reuse while readers retain the old model; do not restore scoped semantics |
| ResolutionState is the sole resolved-fact authority | A consumer or serializer requires a separately mutable resolved-edge graph | Replace it with an indexed projection or extend `ResolutionView`; do not introduce dual truth |
| The reference inventory is complete | A deleted helper still has a semantic caller or a diagnostic performs fresh authored-reference lookup | Extend the owning authored fact and resolver mapping before deletion |
| Current attribute producers are complete | A focused producer test finds authored attribute typing absent from declared facts | Fix the shared typing producer, not an attribute-only resolver |
| Root/config identity is sufficient for later cache reuse | A semantic outcome changes while source root, configuration, and contract version remain equal | Add the missing semantic prerequisite to identity before enabling the cache |

The exact corrected-scope fixture blast radius is intentionally not invented in this document. It
requires the Step 1 prototype. That evidence may alter landing size or reveal missing scope cases,
but it does not alter the architectural decision to have one resolver or the decision to delete the
scoped engine.

## 13. Cache work resumption gate

The unified cache work may resume semantic artifact implementation only after:

1. this upstream resolver migration lands;
2. the cache branch consumes it without restoring a second resolver;
3. the workspace atomically publishes the new `SemanticModel` and verifies its complete identity;
4. the semantic contract version is bumped;
5. cold, parallel, replacement, and decoded reference projections are equal;
6. failed, partial, recovery, cancelled, ambiguous, and unresolved states satisfy their explicit
   publication/cache eligibility contracts.

At that point the full resolver is the uncached oracle. A decoded whole-model cache path is an
optimization of that oracle and must prove observable equivalence before it becomes production.
Incremental semantic resolution is not part of the cache resumption plan.
