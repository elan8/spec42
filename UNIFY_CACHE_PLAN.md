# Unified Global Cache Design

Status: implementation handoff  
Audience: maintainers of `workspace`, `sysml_model`, `lsp_server`, `server`, and other
Spec42 hosts  
Replaces: the existing runtime parse cache and library graph cache

## 1. Decision summary

Spec42 will have one process-independent cache for semantic construction. Every production
surface that builds a model will use the same protocol-neutral build service and the same cache
keys. The cache will contain several typed artifact kinds, but they will share one root, format
envelope, deterministic key-to-path mapping, disk budget, eviction policy, configuration contract,
and management interface.

This is a clean replacement, not an extension of the current design:

- Use BLAKE3 for repository-owned source, manifest, and cache identities.
- Use one canonical postcard-plus-zstd representation for every cache artifact. Remove untyped
  JSON values from the semantic graph before graph caching rather than introducing another codec.
- Remove the existing runtime parse-cache and library-graph-cache implementations and all of
  their call sites in the same cutover.
- Do not read, migrate, dual-write, or retain format compatibility with their SHA-256-keyed
  entries. Old directories are inert disposable files after the cutover.
- Remove `spec42 libraries clear-graph-cache`; replace it with the global `spec42 cache`
  commands defined below.
- Keep installed standard-library and KPAR source trees in the data directory. They are managed
  inputs, not derived cache entries, even though their current commands call them caches.
- Do not cache Wasmtime compilation, plugin execution, plugin output, render output, transport
  DTOs, diagnostics, or generator output. Plugins merely consume the semantic graph. Their work
  and runtime are outside this design.

The cache is always a disposable accelerator. A disabled, missing, corrupt, full, raced, or
unwritable cache must produce the same semantic publication and diagnostics as a cold build.

## Current state and motivation

One-shot commands such as generation, checking, diagram export, model summary, and diagnostic
explanation construct a `HostWorkspaceSnapshot` through `Spec42Engine`. That path currently calls
`IncrementalWorkspace::load`, reparsing and rebuilding the workspace and admitted libraries for
every process invocation. Profiling a repeated generator workload showed most time in semantic
graph construction, allocation, and hashing rather than new command-specific work.

Two independent global disk caches exist today, but neither forms a shared architecture:

- `workspace::parse_cache` stores only `RootNamespace`, keyed by SHA-256 source content and encoded
  with postcard. It is used by selected incremental/LSP paths, not the normal host snapshot load.
  Because diagnostics are not stored, an editor-recovery cache hit currently returns an empty
  parse-error list even when the cold recovery parse reported errors. Strict and recovery callers
  also do not share one explicit parse-outcome contract.
- `workspace::library_graph_cache` stores a JSON-serialized library `SemanticGraph`, keyed by
  SHA-256 path/configuration data and guarded by a second full-tree content fingerprint. Only LSP
  startup reads and writes it. A lookup must walk and hash mutable library trees again, and its only
  serialization round-trip test uses an empty graph.

The LSP and host providers also choose library closure differently. The host provider adds a fixed
set of implied semantic package seeds; LSP startup uses default closure options and a separate
`SPEC42_LIBRARY_FULL_SCAN` switch. Consequently, identical source/configuration presented through
different surfaces is not currently guaranteed to admit the same library set or share a graph
entry.

An in-memory view cache exists, and standard/KPAR libraries are materialized under the data
directory, but neither is a persistent semantic construction artifact. The library catalog's
current SHA-256 `content_hash` commits paths and configured versions rather than all source bytes,
so it cannot prove freshness for a local override whose contents change in place.

These gaps are why merely wiring the two old caches into more call sites is rejected. The owning
build sequence, source identity, closure policy, publication contract, and store policy must be
unified together.

## 2. Goals and success criteria

### 2.1 Goals

1. Reuse parsing and semantic construction across separate invocations and across every Spec42
   host, not only LSP startup.
2. Make cache identity dependency-complete. A hit proves that every semantic input is identical;
   filesystem metadata is never freshness proof.
3. Give CLI, LSP, HTTP/MCP, generator/conformance hosts, and workspace sessions identical
   semantic results for identical inputs and build policy.
4. Preserve atomic publication. A cache hit is an input to the normal publication protocol, not
   permission to bypass revision or cancellation checks.
5. Bound the global store to 5 GiB and prune by least-recent use to 4 GiB.
6. Make misses and fallbacks observable and testable without turning them into user-facing
   semantic failures.

### 2.2 Acceptance criteria

- Every production semantic-build entry point delegates to the shared build service. Direct use
  of the pure `sysml_model` builders remains allowed in owning-layer tests and benchmarks.
- Cold, warm, parallel, incremental, and cache-disabled paths are semantically equivalent.
- Changing any source byte, URI, source role, library order, closure option, standard-library
  classification, parser contract, semantic algorithm contract, or evaluation policy makes an
  incompatible artifact unreachable.
- A same-size, same-mtime source replacement cannot hit an old artifact.
- A corrupted or adversarial entry is rejected before publication and followed by a canonical
  rebuild.
- A representative warm one-shot semantic build takes no more than 20% of the cold semantic-build
  time, excluding unavoidable source acquisition and explicitly uncached projections. Record the
  actual cold/warm figures; do not weaken correctness to meet the target.

## 3. Scope

### 3.1 In scope

- Exact source-content hashing and immutable source manifests.
- Strict and editor-recovery parse outcomes.
- Library package/dependency indexes and resolved library closures.
- Settled library semantic graphs.
- Settled full-workspace semantic graphs.
- Shared cache configuration, storage, concurrency, telemetry, pruning, and CLI management.
- Migration of repository-owned source provenance from SHA-256 strings to typed BLAKE3 digests.
- The semantic-graph publication and round-trip prerequisites in
  `ROUNDTRIP_SEMGRAPH_PREREQS.md`.

### 3.2 Out of scope

- Wasmtime configuration or compiled-module caching.
- Plugin/generator execution and plugin/generator output.
- KPAR archive checksum rules. Existing SHA-256 KPAR interchange checks remain unchanged.
- Externally observable plugin model/module digest contracts. They remain SHA-256 even when their
  implementation consumes the new source metadata.
- Persisting validation reports, render snapshots, prepared views, IBDs, diagrams, symbols,
  hovers, or protocol response objects.
- Replacing snapshot-local query, shape, import, or view memoization.
- Relocating a serialized semantic graph to new URIs. Graph artifacts are path-bound in v1.
- Trusting mtimes, sizes, inode/file IDs, or a prior catalog object as proof of freshness.

## 4. Architectural boundaries

### 4.1 Pure semantic layer

`sysml_model` continues to own parsing adapters, graph construction, linking, evaluation, typed
facts, and semantic queries. Its low-level builders remain deterministic functions of explicit
documents and options. They do not discover an OS cache directory, open a database, or decide
host policy.

`sysml_model` must expose a protocol-neutral graph record/import boundary described in the
round-trip prerequisite document. Runtime `SemanticGraphData` serde is not the persistence
contract.

Before that boundary is introduced, remove `SemanticNode.attributes: HashMap<String,
serde_json::Value>`. Every value that affects semantic construction, resolution, evaluation, or a
typed query moves to its owning declared/effective fact. Source-fidelity and presentation-only
data remains in the AST or an explicitly typed presentation fact and is projected into JSON only
at transport/render boundaries. There is no generic attribute bag in the semantic model at all
and no compatibility fallback that reads old attribute keys.

### 4.2 Canonical build service

Add one `SemanticBuildService` in the `workspace` layer. It owns the semantic construction
sequence and is the only production component allowed to orchestrate persistent artifact reuse.
It receives explicit providers rather than depending on a transport:

```text
SemanticBuildService
  + CacheStore
  + SourceProvider
  + LibraryPolicy
  + SemanticBuildOptions
  + Cancellation/owner publication guard
    -> SemanticWorkspaceSnapshot
```

The service performs source acquisition, manifest construction, library closure, parse lookup,
graph lookup or construction, graph validation, and final publication. Hosts adapt its immutable
`SemanticWorkspaceSnapshot` into language, validation, rendering, projection, or plugin inputs.

The LSP may retain its in-memory incremental engine for live edits, but startup, full rebuilds,
and settled graph persistence must pass through this service. It must not retain a private cache
or private closure implementation. The CLI/HTTP/MCP snapshot path, generator/conformance hosts,
and workspace sessions use the same service.

### 4.3 Canonical publication

Introduce a protocol-neutral publication returned by the service:

```rust
pub struct SemanticWorkspaceSnapshot {
    pub publication: SemanticPublication,
    pub sources: SourceManifest,
    pub graph: SemanticGraph,
    pub parsed_documents: Vec<WorkspaceParsedDocument>,
}

pub struct SemanticPublication {
    pub root_digest: RootDigest,
    pub phase: SemanticPhase,
    pub completeness: SemanticCompleteness,
    pub semantic_contract: SemanticContractVersion,
}
```

`SemanticPhase` distinguishes at least parsed, structurally linked, and settled/evaluated.
`SemanticCompleteness` distinguishes complete input from editor recovery, unsupported, partial,
cancelled, and failed construction. Only a complete settled/evaluated publication is eligible for
a persistent semantic-graph entry.

Pending semantic relationships are not evidence of an incomplete build: a settled graph may
correctly publish an explicit unresolved or ambiguous result.

## 5. Source acquisition and identity

### 5.1 Immutable source snapshot

Filesystem providers read each admitted file as bytes exactly once. From those bytes they:

1. Compute the BLAKE3 content digest.
2. Validate/decode UTF-8.
3. Construct the `SysmlDocument` content used by parsing and graph construction.

Do not hash one read and parse a later read. In-memory/editor providers hash the UTF-8 bytes of the
captured immutable buffer. A provider failure remains a provider failure; it is not hidden by a
cache hit.

Use distinct newtypes so digest domains cannot be mixed accidentally:

```rust
pub struct Blake3Digest([u8; 32]);
pub struct ContentDigest(Blake3Digest);
pub struct RootDigest(Blake3Digest);
pub struct ArtifactKey(Blake3Digest);
```

The stable text form is `blake3:<64 lowercase hex digits>`. Binary formats store the 32 bytes.
No digest type accepts an unprefixed legacy SHA-256 string.

### 5.2 Source manifest

Each source entry includes:

- normalized URI;
- path hint as provenance, not identity unless a consuming policy explicitly depends on it;
- `Workspace`, `StandardLibrary`, `Library`, or `External` source role;
- exact `ContentDigest`;
- byte length;
- ordered library-root slot and relative path when applicable.

Sort workspace entries by normalized URI. Preserve configured library-root precedence explicitly;
do not sort away a precedence rule. Compute leaves and roots with domain-separated,
length-prefixed canonical encoding rather than string concatenation. The root commits every
entry, role, identity, digest, and ordering policy.

Mutable local library roots must be scanned and every admitted source byte hashed. Managed
embedded libraries may use a pinned, previously verified content manifest whose root digest
transitively commits every installed file. A version string or install directory alone is not
sufficient.

### 5.3 Repository-owned metadata migration

Make the following breaking changes together:

- `SysmlDocument.sha256: Option<String>` becomes
  `content_digest: Option<ContentDigest>`.
- `HostArtifactMetadata.document_hashes` becomes
  `document_digests: BTreeMap<String, ContentDigest>`.
- `LibraryCatalog.content_hash` becomes `root_digest: RootDigest` and is computed from verified
  content, not only path and configured version.
- Increment `ARTIFACT_METADATA_VERSION` and update owning schemas, fixtures, comparison code,
  documentation, and generated bindings. Reject old metadata rather than defaulting or aliasing.

KPAR SHA-256 checksums and the generator protocol's SHA-256 digest fields are separate externally
owned/observable contracts and do not change in this work.

## 6. Artifact model and keys

### 6.1 Common key construction

Every artifact declares an ordered `ArtifactIdentity` containing:

- artifact kind and kind schema version;
- cache envelope version;
- parser AST/diagnostic contract where applicable;
- semantic algorithm contract where applicable;
- exact prerequisite digests;
- normalized options and policy versions;
- source identities and roles whenever output embeds or depends on them.

The `ArtifactKey` is BLAKE3 over a domain-separated canonical encoding of that identity. Artifact
versions are explicit repository-owned constants. `CARGO_PKG_VERSION` may be included as a
conservative invalidator, but it is not a substitute for parser, closure, or semantic contract
versions, particularly during development.

### 6.2 Parse outcome

Key inputs:

- raw `ContentDigest`;
- parse mode (`StrictSemantic` or `EditorRecovery`);
- parser package/AST version;
- parse diagnostic schema and parse algorithm version;
- relevant parser options.

Payload:

- typed status: success, recovered with diagnostics, or expected syntax failure;
- AST when the mode produces one;
- complete structured parser diagnostics, including stable ranges and codes where available.

Do not cache a panic, cancellation, resource-limit abort, or partial result. Cache an expected
negative parse outcome only under the same complete key as a successful result. This fixes the
current warm-path loss of editor parse diagnostics and prevents strict/editor modes from sharing
an incompatible AST outcome.

Parse artifacts omit URI and are portable across files and relocated checkouts because parser
ranges are content-relative.

### 6.3 Library package index

Key inputs:

- ordered library-root content manifests expressed as root slot, relative path, and content
  digest, without absolute install paths;
- package-index schema/algorithm version;
- parser-independent lexical indexing policy.

Payload:

- package/import/type-reference facts required by closure resolution;
- relative file identities and their content digests;
- explicit malformed/unsupported index status.

The index is portable when identical library contents move to a different absolute root. It must
not become a second semantic parser: facts used only after parsing remain owned by the semantic
pipeline.

### 6.4 Library closure

Key inputs:

- the complete workspace source root, conservatively committing all workspace text;
- ordered library-index keys;
- one canonical `LibraryClosurePolicy`, including bootstrap flags, implied package seeds,
  full-scan mode, source roles, standard-library roots, root precedence, and closure algorithm
  version.

Payload:

- deterministically ordered selected library files by root slot and relative path;
- workspace-declared/shadowed packages that influenced selection;
- the selected-files root digest.

The current CLI/host implied-package list and LSP default closure options must be replaced by this
single policy. A closure entry may over-invalidate on unrelated workspace content, but it may not
omit an input because a heuristic signature happened to remain unchanged.

### 6.5 Library semantic graph

Key inputs:

- selected library files with absolute normalized URIs, content digests, and source roles;
- workspace shadow set and merge policy;
- standard-library classification;
- closure key and semantic contract versions;
- complete settled evaluation policy.

Payload:

- a complete path-bound library `SemanticGraphRecordV1` plus its publication and source subset
  manifest.

Absolute URIs are deliberately part of the identity because graph `NodeId`s embed them. Reuse
after relocating a library occurs at the parse/index layers, not the graph layer.

### 6.6 Workspace semantic graph

Key inputs:

- complete source root and every admitted document URI, digest, and role;
- resolved library-closure and library-graph keys;
- merge/shadowing, normalization, semantic, and evaluation policies;
- parser/graph/record contract versions.

Payload:

- a complete path-bound `SemanticGraphRecordV1` and `SemanticPublication`.

Do not embed presentation artifacts or duplicate raw source text/ASTs. On a graph hit, obtain the
current text from the immutable source snapshot and fetch each AST from the parse artifact cache.
If an AST entry is absent or corrupt, parse that source normally. The graph candidate remains
valid only if its publication root exactly matches the current source manifest.

Do not persist structurally linked but unevaluated LSP waves. A debounced complete LSP publication
may be stored after the owner verifies it is still current.

## 7. Physical store

### 7.1 Layout

Use one stable root:

```text
<platform-cache>/spec42/cache/
  objects/<key-hex[0..2]>/<key-hex[2..4]>/<artifact-key>.s42c
  tmp/
```

There is exactly one object location for an `ArtifactKey`. The first two bytes of the hexadecimal
key form two directory levels, keeping directory fan-out bounded without introducing an index or
key-to-location lookup table. Artifact kind is already domain-separated into the key and is also
recorded in the verified envelope for inspection and status reporting.

Do not put the current envelope or artifact version in the root directory name. Those versions
already participate in keys and headers, so incompatible entries are unreachable and removable by
normal maintenance. If a future change replaces the physical key-to-path/sharding contract itself,
that change may deliberately choose a new root and treat the old root as disposable; the initial
design does not pre-create that migration mechanism.

The default platform cache root comes from the existing platform-directory abstraction. A shared
`CacheConfig` supports an explicit root for embedders/tests and `SPEC42_CACHE_DIR` for processes.
Create cache directories and files with user-only permissions where the platform supports it.
Cached ASTs and graphs can contain model identifiers and literals even though raw source files are
not stored verbatim.

### 7.2 Object envelope and format

Every `.s42c` object contains a fixed, manually decoded envelope followed by compressed payload:

- magic `S42C`;
- envelope version;
- artifact kind and artifact schema version;
- expected `ArtifactKey`;
- compressed and uncompressed byte lengths;
- BLAKE3 digest of the uncompressed payload;
- payload bytes.

Envelope v1 defines exactly one payload format: a canonical typed postcard record compressed with
zstd. This applies to parse outcomes, indexes, closures, and semantic graphs. Do not add codec or
compression negotiation to accommodate runtime types. A future format change requires a new
envelope/artifact schema and clean invalidation, not a parallel decoder surface.

Postcard compatibility is a graph enablement gate. All `serde_json::Value` storage must first be
removed from `SemanticNode`, semantic consumers must read typed facts, and cache records must use
sorted collections or canonical vectors. JSON remains permissible only in DTOs assembled at the
transport/presentation boundary after semantic queries have completed.

Decode only after validating magic, exact versions, expected key, compressed length, and configured
resource limits. Stream decompression into a bounded buffer, verify the payload digest, decode,
then run artifact-specific invariants.

### 7.3 Direct lookup and filesystem metadata

The store has no database, manifest, catalog, or advisory control plane. Given an `ArtifactKey`,
lookup derives its two-level path directly and opens that one file. The envelope verifies the key,
kind, schema, lengths, and payload digest before decode.

Filesystem metadata is used only for capacity management:

- compressed file length contributes to the disk budget;
- object modification time is advisory last-access time for LRU;
- on a successfully validated hit, best-effort touch the object at most once per minute per
  process;
- a failed touch does not invalidate the hit or change semantics;
- source-file metadata is never involved in cache freshness, and cache-object metadata is never
  proof that payload contents are valid.

`status`, `prune`, and post-write budget enforcement scan the two-level object tree. They read the
small fixed envelope only when artifact-kind/schema reporting is needed. This O(number of cache
objects) maintenance cost is intentionally preferred to a second persistence subsystem. With a
5 GiB bounded cache and sharded directories, it is acceptable; measure it before introducing any
additional metadata structure.

### 7.4 Lock-free atomicity and concurrency

The store supports any number of concurrent Spec42 processes without a lock service or lock files.
Correctness comes from immutable content-addressed objects, complete-file validation, and atomic
filesystem publication. Two processes may duplicate computation for the same miss; avoiding that
waste is not worth making locking part of the persistent cache contract.

On lookup, open the deterministic final path and validate/decode that open file handle. A concurrent
rename or deletion yields one of three safe results: the reader sees the prior complete file, sees
the new complete file, or fails to open and takes the canonical miss path. It never observes the
temporary file.

On store:

1. Build only from immutable inputs and reject storage for cancelled, failed, partial, or
   superseded work.
2. Encode into a uniquely named file created with `create_new` in the same filesystem as the final
   object, then flush and close the complete file.
3. Publish with a platform abstraction that atomically installs or replaces the final path. A
   reader therefore observes either the old complete object or the new complete object, never a
   partially written destination.
4. If the platform reports that another writer won a no-clobber publication, discard the temporary
   file and validate the winner. If atomic replace is the available primitive, replacement is also
   safe because canonical encoding makes every valid artifact for one key byte-identical.
5. Remove abandoned temporary files by age during maintenance. A race that removes a still-active
   temporary file only causes that optional cache write to fail; it cannot affect semantic output.

A crash before publication leaves only a temporary file. A crash after publication leaves a
complete directly addressable object. Duplicate writers may overwrite or discard identical valid
objects, but cannot publish different semantic prerequisites under one key.

`status`, `prune`, and `clear` intentionally provide filesystem-snapshot, best-effort maintenance
semantics rather than transactions. They tolerate files appearing or disappearing while scanning:

- missing-during-scan is ignored;
- a reader whose file is removed either continues from its open handle where supported or records
  a miss and rebuilds;
- a Windows deletion denied by an open reader is skipped and retried by later maintenance;
- a writer racing with prune may have its new object immediately evicted, which is harmless;
- `clear` removes every object it observes, but a concurrent process may publish another object
  before the command returns. The report states the observed removals rather than claiming a
  transactional empty point.

### 7.5 Corruption and failures

Classify lookup outcomes as typed reasons, including disabled, not found, incompatible version,
key mismatch, checksum failure, truncated, resource limit, decode failure, invariant failure,
and I/O failure.

Every failure is a miss followed by the canonical path. Best-effort deletion of a bad path may race
with another writer and may therefore delete a valid replacement; that is acceptable for a
disposable cache and only causes another miss. Do not return partially decoded data, fill missing
facts with defaults, suppress the canonical-path error, or present corruption as a valid empty
artifact.

## 8. Capacity and lifecycle

- Default maximum: 5 GiB across every artifact kind.
- Prune target: 4 GiB.
- Trigger: after a successful write that observes usage over 5 GiB, and through the explicit
  prune command.
- Accounting: after each successful new-object write, scan object file lengths and prune if the
  observed total exceeds the limit. Concurrent scans may choose overlapping victims or observe
  slightly different totals; missing deletions are ignored. Every completing writer performs a
  convergence pass, so the store returns toward the target without shared mutable accounting. A
  crash between commit and enforcement is repaired by the next write or explicit prune.
- Victim order: oldest object modification time (the advisory last-access time), then artifact key
  for deterministic ties.
- Concurrent readers/writers may make an eviction fail or immediately repopulate an entry; skip the
  failure and continue.
- Treat deletion failure as nonfatal; report that the store remains over budget.
- Version changes make entries unreachable immediately. Maintenance deletes unreachable versions
  through the same budget/prune mechanism rather than an independent startup walker.

The 5 GiB/4 GiB policy belongs to the shared root and is not overridable per process; otherwise two
concurrent processes could apply conflicting lifecycle policy to the same objects. Tests may inject
smaller thresholds only with an isolated temporary root. Setting cache mode to disabled performs
neither reads nor writes.

## 9. Public interfaces and management

The exact module placement may follow repository conventions, but these responsibilities are
required:

```rust
pub struct CacheConfig {
    pub mode: CacheMode,
    pub root: PathBuf,
}

pub enum ArtifactKind {
    ParseOutcome,
    LibraryIndex,
    LibraryClosure,
    LibrarySemanticGraph,
    WorkspaceSemanticGraph,
}

pub enum CacheLookup<T> {
    Hit(T, CacheHitMetadata),
    Miss(CacheMissReason),
}

pub trait CacheStore {
    fn get<T: CacheArtifact>(&self, identity: &T::Identity) -> CacheLookup<T>;
    fn put<T: CacheArtifact>(&self, identity: &T::Identity, value: &T) -> CacheStoreOutcome;
    fn status(&self) -> CacheStatus;
    fn prune(&self) -> CacheMaintenanceReport;
    fn clear(&self) -> CacheMaintenanceReport;
}
```

Expose cache configuration through `EngineBuilder`/host construction so embedded users do not
depend on environment variables. Server CLI flags and LSP configuration translate into that same
type rather than creating separate policy.

Add:

- `spec42 cache status`: scan and report root, enabled mode, budget, total size, entry counts/sizes
  per kind, and corruption/maintenance state; support the repository's normal text/JSON output
  conventions.
- `spec42 cache prune`: enforce the configured target and report reclaimed bytes/counts.
- `spec42 cache clear`: scan and best-effort remove all observed unified derived artifacts, report
  what was removed, and state that concurrent processes may repopulate the cache.

Keep standard-library and KPAR materialization commands distinct and describe them as installed
data management. Remove only the graph-specific legacy command.

## 10. Integration and publication flow

### 10.1 Full build

```text
capture exact sources
  -> build SourceManifest/RootDigest
  -> resolve canonical library indexes and closure
  -> look up complete workspace graph
     -> hit: validate graph record and rehydrate parse artifacts
     -> miss: parse, reuse/build library graph, build/link/evaluate workspace graph
  -> verify publication identity is still current
  -> atomically publish SemanticWorkspaceSnapshot
  -> store only complete, current artifacts
  -> build uncached host projections from the publication
```

The graph lookup may occur only after the complete admitted source set is known. A cache must not
silently change document discovery or make a missing/unreadable input disappear.

### 10.2 Incremental/LSP build

Each workspace owner has its own monotonic revision/publication identity. Async work captures the
immutable source root for that revision. A result—cached or computed—commits only if the owner
atomically confirms that identity is still current. Independent owners never arbitrate using a
shared global generation token.

Use persistent parse outcomes for any immutable editor buffer. Continue using the snapshot-local
incremental graph for keystrokes. Store a workspace graph only after the settled evaluation barrier
and only if the revision is still current. Cache reuse never substitutes for the owner publication
check.

### 10.3 Surface audit

During implementation, inventory every non-test call to semantic full-build functions. Route
server CLI/HTTP/MCP, LSP startup/rebuild/validation and fallback views, generator/conformance
hosts, language-service construction, and workspace sessions through the service. Add a dependency
or source guardrail test that prevents new production calls from bypassing it.

## 11. Verification plan

### 11.1 Identity and storage

- BLAKE3 known vectors and domain separation between content, root, and artifact keys.
- Root changes for byte, URI, role, root order, closure option, standard-library classification,
  or contract-version changes.
- Same content at another path hits parse/index artifacts but misses graph artifacts.
- Same size/mtime replacement misses.
- Atomic concurrent writers produce one valid object; interrupted writers leave no accepted
  partial entry.
- Spawn concurrent CLI/LSP-style processes performing same-key and different-key reads, builds,
  commits, status, prune, and clear; verify no partial decode, lost invalidation, crash, or
  persistent over-budget state despite duplicate work and racing maintenance.
- Kill writers before and after the atomic publication point; verify valid-winner preservation,
  abandoned temporary-file cleanup, and cold fallback.
- Exercise no-clobber-winner and atomic-replace implementations, including a reader holding the old
  file while a writer publishes the new complete file.
- Truncation, bit flips, key mismatch, invalid envelope, incompatible version, decompression bomb,
  excessive collections/depth, and graph invariant failure all fall back cold.
- Read-only/unwritable cache, missing object, failed access-time touch, and failed LRU deletion do
  not change semantics.
- LRU crosses 5 GiB and converges to at most 4 GiB while readers, writers, and other pruners race.

### 11.2 Semantic parity

- Strict valid and invalid parse outcomes have identical cold/warm ASTs and diagnostics.
- Editor-recovery outcomes retain identical diagnostics on a hit.
- CLI and LSP resolve the same library closure for the same sources and policy.
- Library graph hit/miss and full workspace graph hit/miss match the canonical graph-state
  fingerprint, semantic S-expression, queries, diagnostics, and projections.
- Workspace/library shadowing, standard-library implied relationships, evaluation facts,
  unresolved/ambiguous facts, and duplicate namespaces have cold/warm coverage.
- Full, cached, and incremental sequences cover add/edit/delete/rename-away/restore and
  superseded out-of-order completion.
- Cache-disabled tests execute the same owning code and differ only in reuse metrics.

The semantic graph tests required before enabling graph entries are specified separately in
`ROUNDTRIP_SEMGRAPH_PREREQS.md`.

### 11.3 Performance

Measure at least one small workspace and the representative webshop/generator workload with:

- cold store;
- warm parse only;
- warm library graph;
- warm full workspace graph;
- cache disabled;
- local mutable libraries versus verified managed-library manifests.

Report source scan/hash, lookup, decode/decompress, parse, library closure, graph build/link,
evaluation, and uncached projection time separately. The cache is ready only after parity is proven;
performance results do not waive a correctness gate.

## 12. Implementation sequence and clean cutover

This is one user-visible migration, implemented internally in dependency order:

1. Complete the semantic graph round-trip prerequisites and parity suite, including removal of the
   untyped semantic-node attribute bag and migration of every consumer to canonical typed facts.
2. Add typed BLAKE3 identities, exact source snapshots, and breaking metadata v2.
3. Add the lock-free sharded store, canonical postcard/zstd format, atomic publication,
   filesystem-scan capacity policy, and management API.
4. Add parse, library-index, closure, library-graph, and workspace-graph artifacts.
5. Add `SemanticBuildService` and route every production surface through it.
6. Add CLI management and observability.
7. Remove legacy cache modules, integrations, SHA-based repository metadata, and the graph-specific
   legacy command.
8. Run focused tests, workspace-wide tests, cold/warm parity, corruption drills, concurrency tests,
   and performance benchmarks before enabling the cache by default.

Do not ship a mode that keeps the legacy and unified caches alive together. If the complete cutover
cannot satisfy the correctness gates, keep the new graph artifact disabled in development rather
than retaining a compatibility path.
