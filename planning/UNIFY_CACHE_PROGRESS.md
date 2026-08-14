# Unified cache implementation status

Status: paused before semantic-graph cache enablement

This tracker records only current state. [UNIFY_CACHE_PLAN.md](UNIFY_CACHE_PLAN.md) owns the design,
and [ROUNDTRIP_SEMGRAPH_PREREQS.md](ROUNDTRIP_SEMGRAPH_PREREQS.md) owns the graph enablement gate.

## Active blockers

| Blocker | Required outcome | Status |
|---|---|---|
| Resolution | Every full, incremental, and parallel surface uses the canonical resolver | active design/cutover work |
| B2 | Rebuild and validate lookup and containment indexes from canonical records | not started |
| B5 | Define `SemanticGraphRecordV1`; do not serialize runtime graph state directly | not started |
| B6 | Rehydrate graph hits with their required source and syntax inputs | not started |
| B8 | Produce canonical, byte-stable graph encoding | not started |
| B9 | Remove the remaining attribute bag and its plumbing | in progress |
| B10 | Bound hostile nesting during decode and destruction | in progress |

Persistent `LibrarySemanticGraph` and `WorkspaceSemanticGraph` artifacts remain disabled until all
round-trip blockers pass. A cache miss, corrupt entry, or unsupported incremental case must use the
canonical full path without changing observable semantics.

## Remaining B9 work

The remaining semantic residue must become typed facts before the JSON attribute map is deleted:

- `analysisResultCount`, `analysisResultMode`, `returnType`, and `analysisResultType` need a typed
  derived-analysis representation, distinct from authored analysis facts.
- `renderingType`, `hasSubject`, and `objectiveCount` need classification at their owning semantic
  layer.
- `inheritedAnalysisResult` and `isRedefinition` have no readers and should be deleted.
- Remove transitional dual writes, `serde_json::Value`, and the vestigial `attrs`/`HashMap`
  plumbing only after all consumers use the typed owners.

## Resume order

1. Complete the canonical resolution cutover and its full/incremental/parallel parity tests.
2. Finish B9 and delete the attribute bag.
3. Implement B5 and B8, with B2 index reconstruction and invariant validation.
4. Implement B6 workspace rehydration.
5. Route production surfaces through the canonical build service.
6. Add cache management/observability, remove legacy paths, and run parity, corruption,
   concurrency, cold/warm, and performance gates.

## Known semantic follow-up

Redefinition semantically entails subsetting. The current implied exactly-one multiplicity check
suppresses its default only for an explicit `Subsetting` edge, so redefining features can receive a
spurious implied `1..1`. Fix this at the relationship owner with a fixture survey and regression
coverage; do not encode a cache-specific workaround.
