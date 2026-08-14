# Semantic graph round-trip prerequisites

Status: blocks persistent semantic-graph cache artifacts

Related design: [UNIFY_CACHE_PLAN.md](UNIFY_CACHE_PLAN.md)

## 1. Purpose

This plan contains only work still required before a semantic graph may be persisted and reused.
Completed prerequisites are removed; their contracts are owned by implementation, tests, and
repository policy.

## 2. Required round-trip guarantees

### 2.1 Codec round-trip

A supported graph must encode and decode through the cache record without panic, alternate codecs,
legacy fallback, or direct runtime-graph serde.

### 2.2 Exact graph-state round-trip

Cold and decoded graphs must agree on every authoritative fact: node identity, kind, names, URI,
range, parent, typed authored/effective/evaluated facts, edges and provenance/owner, source roles,
publication state, and evaluation state. Runtime indexes and memoized queries are rebuilt.

### 2.3 Query and incremental round-trip

Cold and decoded graphs must return identical public query results before and after replacement,
removal, rename-away/restore, and dependency-frontier refresh. Unresolved, ambiguous, partial,
recovery, cancelled, and failed states remain explicit.

### 2.4 Workspace round-trip

A graph hit reconstructs a coherent workspace with current source text, ASTs, source roles, path
hints, diagnostics, graph, and publication identity. It may not conceal unreadable source input.

## 3. Current blocking gaps

### B2. Rebuild lookup indexes from graph truth

- Omit URI, qualified-name, node-index, and containment indexes from the record.
- Rebuild URI membership, canonical names, aliases, node indexes, and children through one owner.
- Reject duplicate IDs, invalid parents, containment cycles, missing aliases, and dangling targets.
- Verify rebuilt indexes against a canonical cold build.

### B5. Define an explicit cache schema

- Define `SemanticGraphRecordV1` and an exact artifact schema version.
- Reject envelope/schema mismatches and absent required fields.
- Provide no legacy JSON decoder or alternate runtime-graph persistence path.
- Remove public runtime graph serde if no non-cache contract requires it.

### B6. Rehydrate the complete workspace

- Bind the graph record to its `SourceManifest` and `SemanticPublication`.
- Read current source text even on a graph hit.
- Rehydrate each AST by content digest and parse mode, parsing cold on an independent miss.
- Preserve structured parser diagnostics in parse artifacts.
- Construct every host snapshot from the same graph, source snapshot, and AST set.

### B8. Make encoding canonical

- Sort nodes by canonical `NodeId`.
- Sort edges by source, target, kind, provenance, construction owner, and detail while preserving
  legitimate parallel-edge multiplicity.
- Encode maps, sets, pending relationships, facts, source origins, and library URIs as sorted
  records.
- Reconstruct fresh runtime indices and require byte-identical encoding for identical v1 records.

### B9. Remove the semantic-node attribute bag

`SemanticNode.attributes: HashMap<String, serde_json::Value>` remains a competing semantic store
and cannot be represented by the selected postcard format.

- Move semantic values to canonical typed authored, effective, derived, or evaluated facts.
- Keep syntax fidelity in AST/source facts and presentation values in boundary projections.
- Remove the field and construction plumbing without a compatibility map or dual-read path.
- Guard graph-owned semantic modules against `serde_json::Value`; boundary DTOs remain allowed.
- Round-trip evaluated real values and reject non-finite values.

Key-level work is tracked in [UNIFY_CACHE_PROGRESS.md](UNIFY_CACHE_PROGRESS.md).

### B10. Bound corrupt input

- Validate compressed length before allocation and stream decompression under a hard output limit.
- Bound nodes, edges, facts, strings, pending records, expression nodes/depth, and collections.
- Decode recursive expressions iteratively, or prove excessive nesting is safely rejected.
- Return typed failures; panic containment may exist only as defense in depth.

## 5. Canonical record design

`SemanticGraphRecordV1` is path-bound and explicit. It contains schema/semantic contract versions,
publication, source origins, library identities, canonical nodes and edges, and authoritative typed
facts. It excludes runtime indexes, caches, traversal state, and build-local handoff state.

Export flow:

1. Verify publication eligibility and current owner identity.
2. Assert no build-local handoff remains and validate runtime invariants.
3. Project canonical records, encode, compress, checksum, and atomically store.

Import flow:

1. Validate envelope, schema, key, lengths, and checksum.
2. Decode under limits and validate uniqueness and references.
3. Construct nodes and edges canonically, rebuild indexes, and validate the runtime graph.
4. Verify the publication root against the current source manifest.
5. Return a candidate to the normal owner publication check.

## 6. Determinism and ordering requirements

- Use normalized URI then qualified name for canonical `NodeId` ordering.
- Canonicalize unordered inputs at their owner before encoding.
- Preserve explicitly ordered semantic collections.
- Reject duplicate identities or ambiguous ordering; never use first-wins behavior.
- Exclude timestamps, access metadata, cache paths, and graph-index allocation from payloads.

## 7. Required test matrix

### 7.1 Exact record corpus

Cover empty, single/cross-document, library, expression-heavy, evaluated, and recovery graphs with
exact field assertions and byte-stable encoding checks.

### 7.2 Observable parity

Compare graph fingerprints, semantic S-expressions, public queries, diagnostics, navigation,
projections, and evaluation between cold and decoded state.

### 7.3 Incremental resume parity

Compare replacement, removal, rename-away/restore, import changes, and forward/reverse edit order
with a cold rebuild after every step.

### 7.4 Library-base parity

Cold and decoded library bases must produce identical workspace semantics, precedence, and
ambiguity outcomes.

### 7.5 Corruption and limits

Reject truncation, bit flips, checksum/key/schema mismatch, duplicate IDs, dangling references,
containment cycles, invalid publication, oversized data, excessive expression depth,
decompression bombs, and non-finite numbers as typed misses.

## 8. Enablement gate

Persistent graph artifacts remain disabled until:

1. B2, B5, B6, B8, B9, and B10 are complete and removed from this plan.
2. Exact, observable, incremental-resume, and library-base parity pass.
3. Every decode/import failure is an explicit miss with canonical cold fallback.
4. No production surface can select runtime serde or an alternate graph codec.
