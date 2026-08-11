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
| 2 | Typed BLAKE3 identities, exact source snapshots, metadata v2 | not started | blocked on 3 |
| 3 | Lock-free sharded store, postcard/zstd envelope, capacity, management API | in progress | `feat/cache-store-foundation` |
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
| A | `shortName` | in progress | `feat/b9-shortname-units` |
| B | unit prefix/conversion/value-expression metadata | in progress | `feat/b9-shortname-units` |
| C | state machine: `source`, `target`, `isInitial`, `targetIsDone`, `stateName`, `finalStateCount` | in progress | `feat/b9-state-facts` |
| D | source fidelity: `doc`, `body`, `text`, `language`, `metaclassRole`, `keyword` (hover use) | in progress | `feat/b9-source-fidelity` |
| E | analysis and expression: `value`, `defaultValue`, `lhs`, `rhs`, `condition`, `isThen`, `analysis*`, `objectiveBoundTo`, `originRange` | not started | |
| F | relationship endpoints and type classification: `redefines`, `subsetsFeature`, `referencesFeature`, `crossesFeature`, `specializes`, `endType`, the `*Type` family, `keyword` (metadata-view use) | not started | |
| G | presentation cutover and field deletion: `generalView*` rollups, remaining `lsp_server`/`server`/`generator_api`/`workspace` consumers, then delete the field | not started | |

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
| B1 | Typed edge construction ownership; rebuild cross-document ownership from it | not started |
| B2 | Omit lookup/containment indexes from the record; rebuild and validate them | not started |
| B3 | Complete source roles and canonical resolution precedence | not started |
| B4 | `SemanticPublication` identity, phase, completeness | not started |
| B5 | `SemanticGraphRecordV1` replaces direct runtime serde | not started |
| B6 | Graph hit rehydrates sources and ASTs; no concealed missing input | not started |
| B7 | Typed `GraphInvariantError` and single cache-import validator | not started |
| B8 | Canonical, byte-stable encoding | not started |
| B9 | Attribute bag removed | in progress |
| B10 | Decode bounds; no stack overflow on hostile nesting | in progress (store layer) |
| B11 | `GraphStateFingerprint` plus query and post-edit differential suites | not started |
