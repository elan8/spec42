# Semantic Graph Round-Trip Prerequisites

Status: blocking design and implementation checklist  
Audience: maintainers of the semantic graph, incremental pipeline, and unified cache  
Related design: `UNIFY_CACHE_PLAN.md`

## 1. Purpose

The unified cache design needs to persist settled semantic graphs. The current
`SemanticGraph: Serialize + Deserialize` implementation is not proof that a graph can safely be
resumed. It proves only that some runtime fields can pass through a serde format. The only cache
round-trip test currently exercises an empty graph.

This document defines what “round-trip” must mean, records the blockers found in the current code,
and specifies the changes and tests required before either a library or workspace semantic graph
may be accepted from the global cache.

This is not a compatibility project. The existing library graph JSON is not a supported input to
the new design, and the new decoder will not attempt to upgrade it.

## 2. Required round-trip guarantees

There are four different guarantees. Passing an earlier level does not imply a later one.

### 2.1 Codec round-trip

A representative postcard record can be decoded without loss or panic. This covers URLs, typed
graph facts, recursive declared expressions, and finite evaluated numeric values. Untyped JSON
attribute values are deliberately not part of the record contract.

Codec round-trip alone says nothing about graph coherence or behavior.

### 2.2 Exact graph-state round-trip

Every authoritative part of a published graph survives:

- node identity, kind, authored/effective names, URI, range, parent, and typed facts;
- all declared semantic facts and their source ranges/provenance;
- edges, endpoint identities, relationship kind, authored/implied provenance, construction owner,
  and connect/flow detail;
- standard-library/source-origin classification;
- pending relationship records and explicit unresolved/ambiguous outcomes;
- effective facts and derived relationship resolution outcomes;
- evaluation facts and evaluation publication state;
- semantic phase, completeness, input root identity, and semantic contract version.

Disposable indexes and memoized queries are excluded, but their complete prerequisites must be
present so they can be derived again.

### 2.3 Query and incremental round-trip

After decoding and rebuilding derived state:

- every public semantic query returns the same value and contractual ordering;
- diagnostics and semantic projections remain equivalent;
- a sequence of incremental changes produces the same publication after each change as a full
  rebuild of the same source set;
- construction order, serialization order, cache warmth, and thread schedule do not affect the
  result.

This is the minimum guarantee for reusing a decoded graph in `IncrementalWorkspace::from_parts` or
as a cached library base graph.

### 2.4 Workspace round-trip

A complete semantic workspace also needs current text, parsed ASTs, source roles, path hints,
library policy, and exact source identity. `SemanticGraph` alone cannot reconstruct
`InMemoryWorkspace`, `HostWorkspaceSnapshot`, syntax diagnostics, editor navigation, or render
inputs.

The full cache artifact therefore wraps the graph record in a source manifest and publication.
Current source text comes from the immutable provider snapshot, and ASTs are rehydrated through
parse artifacts. Presentation and transport projections are rebuilt from that canonical state.

## 3. Current serialized and skipped state

`SemanticGraph` delegates serde directly to `SemanticGraphData`. The current serialized fields are:

- the `StableGraph<SemanticNode, SemanticEdge>`;
- `nodes_by_uri`;
- `node_ids_by_qualified_name`;
- `standard_library_uris`;
- pending expression and ordinary relationships;
- effective facts;
- derived relationship resolutions;
- evaluation facts and `evaluation_publication`.

The current skipped fields are:

- `node_index_by_id`;
- `children_by_parent_id`;
- the build-local `pending_declared_membership_facts` handoff;
- import lookup, general query, and shape caches;
- static document dependency targets and reverse dependents;
- `cross_document_edges_by_source_uri`.

After deserialize, `rebuild_derived_indexes()` reconstructs node indices, containment children, and
the static dependency index. It does not rebuild every skipped field whose behavior matters after
mutation.

## 4. Blocking correctness gaps

### B1. Cross-document edge ownership is lost

`cross_document_edges_by_source_uri` records the exact Typing, Specializes, and Subject edges last
added by the scoped cross-document resolver. Before re-resolving an unchanged dependent document,
`remove_recorded_cross_document_edges_for_uri()` relies on this map to delete its prior edges.

The map is skipped by serde and remains empty after `rebuild_derived_indexes()`, despite the field
comment saying it is rebuilt. Static dependency reconstruction is not the same thing: it identifies
which documents might need refresh, not which existing edges that refresh owns.

A cache-loaded graph can therefore answer current queries correctly and still become wrong after
an edit:

1. Load graph containing an edge from unchanged dependent B to target A.
2. Edit A so B belongs to the relationship frontier.
3. Refresh B. Its ownership map is empty, so the prior edge is not removed.
4. If the new target differs or becomes unresolved, the old edge can survive beside the new state.

This gap also exposes a broader full-build/incremental issue: whole-graph linking does not create
the same ownership state as building documents through the scoped patch path. Serialization merely
makes the missing ownership obvious.

Required resolution:

- Give every edge a typed construction owner separate from semantic provenance. At minimum,
  distinguish document construction, workspace cross-document linking, derivation linking,
  pending-expression resolution, and universal implied-rule construction.
- Preserve that owner in the graph record.
- Rebuild `cross_document_edges_by_source_uri` from edge owner and source identity rather than
  treating the skipped map as stored truth.
- Make whole, parallel, merge-from-base, incremental, and decoded builds establish identical
  ownership.
- Add a narrow regression that full-builds or decodes A+B, edits A away and back, and compares B's
  Typing, Specializes, and Subject edges with a full rebuild after each step.

Do not solve this by serializing the skipped map as a second authority.

### B2. Serialized lookup indexes duplicate graph truth

`nodes_by_uri` and `node_ids_by_qualified_name` are serialized while
`node_index_by_id` and `children_by_parent_id` are rebuilt. There is no decoder validation that:

- each indexed ID exists exactly once;
- each node appears under its own URI and qualified name;
- no real or short-name alias is missing or dangling;
- a duplicate `NodeId` did not overwrite an earlier node-index entry;
- vector ordering agrees with the resolution policy.

This makes cache bytes, rather than nodes and typed source facts, a competing authority for lookup
behavior.

Required resolution:

- Omit all four lookup/containment indexes from the canonical record.
- Rebuild URI membership, canonical qualified names, short-name aliases, node indices, and child
  indexes through one function from decoded nodes and source-origin policy.
- Reject duplicate `NodeId`s before insertion.
- Validate that every parent is valid under the artifact's containment policy and that containment
  cycles are rejected.
- Compare the rebuilt indexes against a canonical full build in tests, but never persist them as
  required truth.

### B3. Source role and lookup precedence are incomplete

The graph preserves `standard_library_uris`, but it does not retain the complete
Workspace/StandardLibrary/Library/External classification. Some current precedence is encoded
indirectly by merge order, skipped library duplicates, and order within qualified-name vectors.

Rebuilding lookup vectors in an arbitrary or lexicographic order can therefore change a
first-match consumer. Preserving the old vector merely preserves an accidental ordering dependency.

Required resolution:

- Add the complete normalized source-origin map to the semantic publication/graph construction
  contract.
- Define canonical source precedence and duplicate namespace behavior in one resolution owner.
- Audit resolution code for `.first()`, `.next()`, or unsorted iteration over potentially multiple
  candidates. Return a unique target, explicit ambiguity, or deterministically ordered candidates;
  never let insertion order decide semantic meaning.
- Rebuild qualified-name and alias vectors using that policy.
- Test workspace/library shadowing, duplicated package namespaces, standard-library admission,
  and document-order reversal.

### B4. The graph has no complete publication contract

`evaluation_publication` distinguishes evaluation `NotRun` from `Complete`, but the graph has no
general identity, phase, or completeness marker. A decoded graph cannot prove:

- which exact source root produced it;
- whether parsing used strict success or editor recovery;
- whether linking, effective-fact construction, pending resolution, and evaluation crossed their
  barriers;
- whether work was partial, cancelled, unsupported, or superseded;
- which semantic algorithm contract produced the facts.

Required resolution:

- Add `SemanticPublication { root_digest, phase, completeness, semantic_contract }` to the
  workspace graph artifact contract.
- Make phase transitions explicit and monotonic during construction.
- Require complete settled/evaluated publication for persistent graph storage.
- Keep unresolved and ambiguous semantic facts explicit; do not mistake them for incomplete
  construction.
- Verify the current owner identity again immediately before publication and storage.

### B5. Direct runtime serde is not a cache schema

`SemanticGraphData` and many nested types use derived serde with additive `#[serde(default)]`
fields. That is convenient for JSON evolution but dangerous for a cache: an old payload can decode
with new semantic facts silently absent. Runtime field layout and petgraph's serde representation
also become accidental disk contracts.

Required resolution:

- Stop serializing `SemanticGraph`/`SemanticGraphData` directly for persistent reuse.
- Define an explicit `SemanticGraphRecordV1` and graph-artifact schema version.
- Check the envelope version and exact artifact schema before invoking the record decoder.
- Do not default absent cache-record fields. A required field missing under the current schema is
  corruption.
- Do not provide a decoder for the legacy JSON payload.
- Prefer removing the public runtime graph serde implementation if no non-cache contract still
  requires it, preventing a second persistence path from reappearing.

### B6. A graph-only hit cannot reconstruct a workspace

`HostWorkspaceSnapshot` and `InMemoryWorkspace` need parsed documents and current content in
addition to a graph. Syntax diagnostics intentionally examine documents that strict semantic
parsing excluded. Rendering and navigation also consume AST/source-fidelity data.

Required resolution:

- Wrap the graph record in the exact `SourceManifest` and `SemanticPublication`.
- Capture current source text from the provider even on a graph hit; a cache must not conceal a
  missing or unreadable input.
- Rehydrate each AST by content digest and parse mode. Parse cold if its independent entry is
  missing.
- Preserve structured parse diagnostics in parse artifacts so warm recovery cannot erase them.
- Construct host/language snapshots from the same graph, source snapshot, and AST set.

### B7. No comprehensive invariant validator exists

Current deserialization trusts most graph content and then populates maps. At minimum, a cache
decoder must reject:

- duplicate node IDs;
- edges with absent endpoints;
- invalid parent relationships or containment cycles;
- source-origin entries that disagree with node URIs;
- standard-library URIs absent from the admitted source set;
- dangling keys/targets in effective facts, evaluation facts, derived resolutions, or edge detail;
- build-local pending membership facts at publication;
- evaluation facts inconsistent with `evaluation_publication`;
- non-finite evaluated real numbers;
- invalid ranges or URL normalization where those contracts require them;
- graph/resource counts beyond the active host limits.

Required resolution:

- Add a typed `GraphInvariantError` and a single cache-import validator in the semantic owner.
- Separate invariant validation from user model diagnostics. Invalid cache state is a cache miss,
  not a source diagnostic.
- Use artifact-specific containment rules where a deliberately extracted library subgraph may
  omit a workspace parent; do not weaken full-workspace invariants globally.

### B8. Serialization is not canonical

The runtime graph contains `HashMap`, `HashSet`, and `StableGraph` insertion/index state. Direct
serde output can vary with construction and hash iteration order even when semantic meaning is
equal. Node-index holes created by incremental deletion are also runtime history, not model facts.

Required resolution:

- Encode nodes as a vector sorted by canonical `NodeId` ordering.
- Encode edges by source ID, target ID, relationship kind, provenance, construction owner, and
  canonical edge-detail encoding; preserve legitimate parallel-edge multiplicity explicitly.
- Encode maps/sets as sorted vectors of key/value records.
- Canonically order pending relationships, effective/evaluation fact records, source origins, and
  standard-library URIs.
- Reconstruct fresh petgraph indices on decode.
- Require byte-identical encoding for the same `SemanticGraphRecordV1` within schema v1, while
  making no byte-compatibility promise across schema versions.

### B9. The untyped semantic-node attribute bag blocks the ideal format

`SemanticNode.attributes` is a `HashMap<String, serde_json::Value>`. It is not merely a codec
inconvenience: semantic construction, name/relationship resolution, units, analysis, state/view
logic, language-service presentation, and projections still read keys from it. Persisting the bag
would make string keys and JSON shapes a second semantic contract, while selecting a
self-describing graph codec solely to carry it would make that debt permanent.

Postcard is the selected format for every unified-cache artifact. It intentionally does not
support `serde_json::Value`'s `deserialize_any` contract. The model must be corrected before graph
caching rather than adding MessagePack, a graph-only codec, or a JSON fallback.

Required resolution:

- Inventory every producer and consumer of `SemanticNode.attributes` and classify each value as
  semantic, source-fidelity, or presentation-only.
- Move every semantic value to one canonical typed declared/effective/evaluated fact at the
  earliest layer with all prerequisites. This includes aliases, import/reference details, unit
  prefixes/conversions, analysis and requirement facts, relationship endpoint facts, and any
  state/view property used to make a semantic decision.
- Keep source spelling, documentation text, and other syntax-fidelity data in the AST or a typed
  source/presentation fact. Renderers and transports may project those typed values to JSON at the
  boundary; they may not feed that JSON back into semantic decisions.
- Replace projection-time JSON aggregates stored back onto semantic nodes with typed projection
  results owned outside the semantic graph.
- Remove the `attributes` field from `SemanticNode` and `SemanticNodeRecord`. Do not retain a
  deprecated map, unknown-key escape hatch, compatibility accessor, or dual-read path.
- Add a compile/source guardrail preventing semantic crates from introducing `serde_json::Value`
  into graph-owned model facts. Boundary DTO modules remain explicitly allowed.
- Encode every cache artifact as a canonical typed postcard record compressed with zstd. The v1
  envelope has no codec-negotiation field and no alternate decoder.
- Round-trip evaluated real values and reject non-finite values during record validation.
- Benchmark postcard encode/decode, compressed size, and peak allocation against the old JSON
  cache only as engineering evidence; do not retain the old representation.

### B10. Recursive expressions and corrupt input need bounds

`DeclaredExpression` can be arbitrarily deep and has a custom iterative `Drop` to avoid stack
overflow. Derived serde decoding can still recurse or allocate from malicious/corrupt length
prefixes. A cache is local but not trusted semantic input: crashes, partial writes, disk faults, and
another local process can alter it.

Required resolution:

- Validate compressed length before allocation.
- Stream decompression with a hard uncompressed limit derived from host resource limits.
- Bound nodes, edges, typed fact collections, string bytes, pending records, expression nodes,
  expression nesting, and all collection lengths.
- Use an iterative expression record decoder or otherwise demonstrate that the selected decoder
  rejects excessive nesting without stack overflow.
- Catch no panic as a successful miss. Decoder APIs return typed failure; outer cache plumbing may
  add a final panic-containment boundary only as defense in depth.

### B11. Existing semantic equivalence oracle is intentionally incomplete

`SemanticGraph::to_semantic_sexpr()` is a valuable canonical semantic projection. It covers
containment, resolved/pending relationships, selected declared/effective facts, derived resolution,
and evaluation facts. It intentionally excludes source ranges, document paths, query caches, and
the current untyped display attributes that this plan requires removing. It also does not by itself
prove publication or incremental operational state.

Required resolution:

- Keep the S-expression as the human-readable semantic parity oracle.
- Add a test-only canonical `GraphStateFingerprint` covering every authoritative record field,
  including URI, ranges, typed facts, source roles, publication, evaluation publication, and
  construction ownership.
- Add public-query and post-edit differential suites. Neither the fingerprint nor S-expression
  alone proves observable behavior.

## 5. Canonical record design

The cache-facing record is protocol-neutral and contains no petgraph indices, mutexes, `Arc`s, or
host DTOs:

```rust
pub struct SemanticGraphRecordV1 {
    pub nodes: Vec<SemanticNodeRecord>, // typed facts; no generic JSON attribute map
    pub edges: Vec<SemanticEdgeRecord>,
    pub source_origins: Vec<DocumentOriginRecord>,
    pub standard_library_uris: Vec<Url>,
    pub pending_relationships: Vec<PendingRelationship>,
    pub pending_expression_relationships: Vec<PendingExpressionRelationship>,
    pub effective_facts: Vec<(NodeId, EffectiveSemanticFacts)>,
    pub derived_relationship_resolutions:
        Vec<(NodeId, DerivedRelationshipResolution)>,
    pub evaluation_facts: Vec<(NodeId, NodeEvaluationFacts)>,
    pub evaluation_publication: EvaluationPublicationState,
}

pub struct SemanticEdgeRecord {
    pub source: NodeId,
    pub target: NodeId,
    pub edge: SemanticEdge,
}
```

The enclosing library/workspace artifact supplies schema version, source manifest, publication,
and artifact-specific limits. The record does not contain `nodes_by_uri`, qualified-name indexes,
containment indexes, static dependency indexes, cross-document ownership maps, query indexes,
shape caches, or import caches.

Export flow:

1. Verify complete publication eligibility in the enclosing snapshot.
2. Assert no build-local membership handoff remains.
3. Validate runtime graph invariants.
4. Project authoritative values into canonical sorted records.
5. Encode, compress, checksum, and atomically store.

Import flow:

1. Validate cache envelope, schema, key, sizes, and payload checksum.
2. Decode under resource limits.
3. Validate record uniqueness, references, source origins, publication, and numeric constraints.
4. Create nodes and edges in canonical order.
5. Rebuild every derived index, including cross-document ownership and static dependencies.
6. Validate the reconstructed runtime graph again.
7. Compare the enclosing publication root with the current immutable source manifest.
8. Return a candidate to the normal owner publication check.

No step substitutes a guessed/default fact when record data is absent or inconsistent.

## 6. Determinism and ordering requirements

The record and runtime graph must make these rules explicit:

- `NodeId` canonical order is normalized URI string, then qualified name.
- Document order follows normalized URI for workspace/external sources and explicit configured root
  precedence plus relative path for libraries.
- Candidate ordering is defined by resolution policy, not petgraph index or vector insertion.
- Children and edges exposed through an ordered contract are sorted at their semantic owner.
- Ambiguous outcomes retain all candidates in canonical order.
- Merge-from-base and full build apply the same workspace/library shadowing rule before lookup
  indexes are published.
- Parallel producers emit isolated results and merge at a deterministic barrier.

If an existing query intentionally promises source order, preserve an explicit source-order field;
do not infer source order from graph insertion history.

## 7. Required test matrix

### 7.1 Exact record corpus

Build and round-trip nontrivial fixtures containing:

- every `ElementKind` and `RelationshipKind` currently constructible;
- authored and implied relationship provenance plus every construction owner;
- connect and flow edge details;
- regular and short names, duplicate package namespaces, and anonymous/effective names;
- multiplicities, feature properties, feature values, nested expressions, and source ranges;
- unresolved and ambiguous pending relationships;
- standard-library implied relationship outcomes, including missing and ambiguous prerequisites;
- evaluation success, false verdict, unresolved, malformed, cycle, not-applicable, and empty
  complete evaluation publication;
- every typed fact introduced while removing the semantic-node attribute bag.

For each fixture, assert canonical encoded bytes are stable, exact graph fingerprints match after
decode, and encode(decode(bytes)) reproduces the same bytes.

### 7.2 Observable parity

For cold and decoded graphs compare:

- semantic S-expression;
- node/relationship queries, containment, imports, type resolution, inherited members, standard
  library facts, units, and evaluation queries;
- symbol/navigation inputs;
- owning-layer diagnostics with codes, ranges, severities, related information, and ordering;
- host semantic projection and representative render inputs.

Run fixtures in forward/reverse document order and sequential/parallel construction modes.

### 7.3 Incremental resume parity

Start each scenario from both a fresh graph and a decoded full-build graph, then compare with a full
rebuild after every operation:

- edit one document without changing dependencies;
- add and delete a document;
- rename a referenced type away and restore it;
- change a target while leaving the dependent document untouched;
- exercise cross-file Typing, Specializes, and Subject edges;
- rewire a derivation connection;
- add/remove an import and wildcard import;
- change workspace/library shadowing and duplicate namespace membership;
- change a standard-library source classification;
- run multiple edits with out-of-order async completion and reject superseded commits.

The rename-away/restore test must begin from a normal full-build graph, not only a graph assembled
one document at a time, so it covers the current ownership gap.

### 7.4 Library-base parity

- Build a library graph, encode/decode it, merge workspace documents onto it, and compare with a
  from-scratch combined build.
- Cover workspace shadowing, shared package namespaces, standard versus ordinary library roots,
  changed closure selection, and missing cached parse artifacts.
- Ensure extraction does not leave dangling parents, facts, source origins, dependency entries, or
  cross-document ownership.

### 7.5 Corruption and limits

Reject and cold-rebuild for:

- wrong envelope or record version;
- truncated or trailing-invalid data;
- compressed/uncompressed length mismatch;
- payload checksum or expected-key mismatch;
- duplicate node IDs;
- dangling edge, parent, effective fact, evaluation fact, or derived resolution;
- inconsistent source origin or standard-library membership;
- invalid publication transition/state;
- non-finite evaluated real;
- excessive graph counts, strings, JSON nesting, expression depth, or decompressed size.

No corruption case may publish a partial graph or emit a model diagnostic that implies the source
is at fault.

## 8. Enablement gate

Persistent `LibrarySemanticGraph` and `WorkspaceSemanticGraph` artifacts remain disabled until all
of the following are true:

1. Edge construction ownership is explicit and reconstructible.
2. Complete source roles and canonical resolution precedence are owned by the semantic system.
3. General publication identity/phase/completeness exists.
4. `SemanticGraphRecordV1` replaces direct runtime serde for cache persistence.
5. The invariant validator rejects every malformed-state category above.
6. The semantic-node attribute bag is gone and canonical postcard/decompression limits pass the
   full corpus and adversarial tests.
7. Exact, observable, incremental-resume, and library-base parity pass for cold and decoded state.
8. The global cache treats all decode/import failures as explicit misses and exercises the
   canonical uncached path.

Parse, library-index, and closure artifacts may be developed while these blockers are being fixed,
but the clean user-visible cache cutover described in `UNIFY_CACHE_PLAN.md` occurs only when the
graph gates pass. Do not retain the old JSON graph cache as a fallback.
