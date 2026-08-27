# AGENTS.md

These principles apply across the repository. More local instructions may add constraints for a
specific area, but must not weaken them.

They are normative for new and modified code. Existing violations are debt, not precedent: do not
copy or extend them, but do not broaden an unrelated change solely to remove them.

## Work from authoritative context

- Understand the relevant design, implementation, and tests before changing code. Executable
  contracts, manifests, and tests establish current behavior; design documents establish intent.
  The root `design.md` is the authoritative design document for the system's architecture.
  If they disagree, do not guess. Resolve the conflict when it is in scope and otherwise report it.
- Fix the owning abstraction, not only the visible symptom. Do not expand a change to clean up
  unrelated legacy debt or pre-existing failures.
- Preserve unrelated work. Do not use destructive operations merely to obtain a clean tree.

## One semantic system

- For identical model inputs and configuration, every surface must use the same semantic results.
  Presentation, protocol shape, feature availability, and explicitly contracted reporting policy
  may differ; semantic meaning may not.
- Every semantic fact category has one authoritative owner and representation for a model state.
  Syntax trees own source fidelity; semantic facts and typed query results own meaning. Indexes,
  projections, DTOs, caches, labels, and rendered output must not become competing truth stores.
- Give every derived fact one canonical derivation owner/API at the earliest semantic layer with all
  prerequisites. Eager computation, lazy computation, recomputation, and memoization are performance
  choices; downstream consumers always use the canonical result rather than reimplementing it.
- Preserve provenance. Authored, normalized/effective, implied, inherited, and evaluated facts must
  remain distinguishable. Never overwrite an authored fact with its effective value or make an
  implied relationship appear explicitly declared.
- After semantic construction, semantic decisions consume owned semantic facts. Source and syntax
  inspection is valid for syntax-fidelity features and documented parser/editor recovery, but it
  must not recreate facts owned by the semantic system.
- When a consumer lacks information, extend the owning fact, result, or contract and its construction
  path. Do not infer it from names, display text, serialized output, incidental data structure, or
  another consumer's behavior.
- Workarounds and fallbacks that guess, substitute, or suppress semantic facts are forbidden.
  Documented parser recovery, compatibility decoding, reporting policy, and harmless presentation
  defaults may degrade explicitly, but their output must not masquerade as a resolved semantic fact.
- Keep unresolved, ambiguous, unsupported, partial, cancelled, and failed states explicit. Where
  externally observable, use a typed result/status or stable diagnostic and test that it cannot be
  mistaken for success.
- Prefer typed facts, relationships, identities, spans, and exhaustive mappings over stringly typed
  dispatch or synchronized ad hoc tables.
- Hosts, transports, editors, renderers, generators, and other consumers adapt explicit semantic
  inputs and outputs. They do not maintain private semantic implementations.

## Phases, publication, and determinism

- Make pipeline phases and ownership boundaries explicit. A phase consumes stable input, has a
  clear writer for the facts it owns, and publishes only at a defined barrier.
- Publish coherent model states atomically. Source revisions, syntax/recovery state, semantic facts,
  indexes, diagnostics, projections, and completeness must agree. Readers may keep using an older
  coherent state; they must not observe a half-applied one.
- An intentionally incomplete publication must identify its phase and completeness in its contract.
  Later enrichment that changes facts is a new publication and makes prior dependent artifacts stale.
- Every publication has a dependency-complete, owner-scoped identity. Asynchronous work captures the
  identity of its immutable inputs and commits only after the owner atomically verifies that identity
  is still current. Every invalidating operation changes it; independent owners never arbitrate work
  with a shared token.
- Parallel work reads stable inputs and produces isolated results. Merge at an explicit barrier with
  deterministic conflict and ordering rules; avoid shared semantic mutation whose result depends on
  scheduling.
- Semantic results and contractually ordered output must not depend on thread scheduling, traversal
  order, unordered-map iteration, cache warmth, or accidental insertion order. Canonicalize at the
  layer that owns ordering.
- Iterative semantic enrichment runs to convergence or an explicit safety bound. Hitting a bound
  leaves an explicit unresolved or unsupported state; it never silently yields a settled result.
- At the same declared phase and model identity, full, incremental, cached, and parallel paths must
  be observably equivalent. An optimization is incomplete until this equivalence is established.

## Derived facts and caches

- Caches are disposable accelerators, never authorities. A missing, corrupt, or unwritable cache may
  cost time but must not change semantic behavior.
- State every prerequisite of a cached value before adding it. Its key or invalidation rule covers
  all semantic inputs, configuration, normalization policy, algorithm/schema/contract versions, and
  query options that can affect the result. If the dependency set cannot be stated, do not cache it.
- Persistent semantic cache identity includes a collision-resistant content digest, a verified root
  digest that transitively commits all inputs, or an immutable pinned snapshot identity. Paths,
  sizes, timestamps, and object presence may reject an entry quickly but are not proof of freshness.
- Snapshot-local data may rely on snapshot identity. Cross-snapshot reuse requires a
  dependency-complete content/version identity; a matching URI, object ID, shape, or entry alone is
  insufficient evidence of validity.
- A prerequisite change atomically invalidates dependent data or makes old entries unreachable.
  Mutation and invalidation are one correctness operation even when physical eviction is deferred.
- Do not publish work for a superseded identity or store partial, cancelled, recovery-produced, or
  failed work as success. Intentional negative caches require the same explicit contract and complete
  key as positive results.
- A cache failure or unsupported incremental case may use the canonical uncached/full path only when
  observable semantics are identical. Make the fallback reason explicit and testable; it must not
  invent a result or hide a canonical-path error.
- New or modified memoization requires cold/warm parity. Cross-snapshot reuse also requires
  stale-input/invalidation coverage; serialized or external caches require corruption rejection;
  asynchronous reuse requires supersession and out-of-order-completion coverage. Snapshot-local
  memoization instead proves that its prerequisites and lifetime belong entirely to that snapshot.

## Boundaries and contracts

- Keep semantic logic independent of transport, editor, UI, process, and filesystem policy. Supply
  external data and effects through explicit providers or adapters, and keep consumers thin.
- Translate protocol-specific types at the boundary. Core contracts use neutral types and do not
  leak a particular host's DTOs, errors, lifecycle, or runtime assumptions inward.
- Each identity domain has one owning normalization policy. Downstream consumers reuse its normalized
  identity while retaining original provenance; display paths and names are not substitutes for
  identity. Identities are unique within their declared scope, collision-resistant or
  collision-detected, and stable across the changes promised by their contract.
- Declare each repository-owned contract value once and derive its bindings, tables, manifests,
  documentation, and compatibility checks. Mirror externally owned contracts only with an
  authoritative conformance or drift check.
- Do not hand-edit derived artifacts. Change their authoritative input and regenerate them. Committed
  generated output is byte-reproducible from pinned inputs, free of clocks, absolute paths, and
  environment-dependent data, and checked as a complete set for missing or orphan files.
- A deliberate wire or observable semantic contract change updates the authoritative declaration,
  generated artifacts, tests, and consumers together. Update a version or compatibility token when
  the contract defines one and its versioning policy requires it.
- Rendering and presentation may lay out, style, alias, or format semantic projections, but must not
  rediscover model semantics.
- Preserve local-first, reproducible operation. Repository build scripts and tests must not perform
  implicit ad hoc network fetches or download model libraries. Dependency installation is explicit
  and lockfile-governed; model, schema, and tool refreshes are explicit and pinned.

## Evidence and verification

- The standalone snapshot tool is the primary end-to-end integration test for the compiler
  pipeline; regenerate and review its checked-in snapshots for pipeline behavior changes.
- Semantic compiler behavior belongs in the standalone snapshot corpus, not in Rust tests that
  render a model and search its text. New or modified semantic coverage must use authored typed
  snapshot expectations (`EXPECTED SEMANTICS` or `EXPECTED DIAGNOSTICS`) when the snapshot tool
  exposes the relevant contract; otherwise it must use the canonical generated snapshot projection
  until that typed expectation API is added. Never assert semantic identities, facts,
  relationships, outcomes, diagnostics, or provenance with `contains`, substring matching, regexes,
  or hand-parsed rendered S-expressions.
- When touching an existing semantic-level unit or integration test that string-matches rendered
  compiler output, migrate that coverage into the snapshot corpus and remove the anti-pattern; do
  not extend it with another assertion. Rust unit tests remain appropriate for owning-layer
  algorithms, typed data structures, parser mechanics, snapshot Markdown mechanics, and renderers
  when the tested value is itself the textual contract, but their assertions must consume the
  owning typed API rather than rediscover semantics from presentation text.
- Specification and conformance claims require traceable normative evidence plus executable tests.
  Unsupported coverage stays visible; it does not disappear behind omission or optimistic labels.
- Encode each researched KerML or SysML validation rule in the snapshot corpus with conforming and
  violating source examples, explicit specification document and clause metadata, and an authored
  `EXPECTED DIAGNOSTICS` assertion. If the canonical compiler does not yet satisfy that assertion,
  use a typed `blocked_by` issue with a concrete owner and category so the case remains visible as
  `BLOCKED`; remove the blocker when it becomes stale. The snapshot-tool README owns the fixture
  syntax, blocker semantics, and evidence requirements.
- For behavior changes and bug fixes, add the narrowest regression test at the owning layer, then
  verify affected consumers. Test both sides of the rule as appropriate: accepted/rejected,
  resolved/unresolved, same/cross source, full/incremental, cold/warm, or current/superseded. A pure
  refactor may rely on existing owning-layer coverage that already pins behavior.
- Diagnostics are public behavior. Test stable codes, precise source locations, severity, ordering,
  and related information where relevant. Intentional contract changes may update them with stated
  rationale and consumer coverage; never weaken assertions merely to hide a regression.
- Prefer runnable tests. A skip must state its concrete reason, such as a missing capability,
  optional external fixture, platform constraint, or deliberately expensive local drill. Conformance
  and migration corpus tooling accounts for discovered, exercised, skipped, and malformed cases.
- Run focused checks while iterating, then the broader checks appropriate to the affected area. Fix
  failures caused by the change; record unrelated failures and continue independent checks where
  practical.
- Before opening or updating a pull request, run `scripts/minici.sh`, the repository's local
  pre-push entry point for the platform-independent Rust-core CI checks. If its documented local
  prerequisites are unavailable, report that explicitly and do not claim that all checks pass;
  distinguish the script's coverage from the CI-only packaging and cross-platform jobs in
  `.github/workflows/ci.yml`.
- Establish correctness and equivalence before optimizing. Measure representative workloads and
  preserve deterministic correctness under both cold and warm conditions.
- Review architectural changes adversarially. Look for a second source of truth, downstream
  re-derivation, hidden recovery, incomplete cache identity, stale publication, nondeterministic
  ordering, boundary leakage, generated-artifact drift, and missing negative or parity coverage.

## Documentation lifecycle

- Planning documents contain only active decisions, blockers, and remaining work. Remove an item
  when it is completed; do not retain completion summaries, execution diaries, resolved-item
  indexes, historical context sections, or superseded plans.
- Git commit messages own implementation history and completed-work rationale. `CHANGELOG.md` may
  carry one succinct user-visible summary when the change is notable; it is not a substitute for a
  planning archive.
- Move reusable lessons into this file as enduring repository policy, or into the authoritative
  design document when they are lasting properties of the system. Do not preserve them in a
  completed investigation or progress tracker.
- Delete a planning or investigation document when it has no live decisions or work. Update or
  remove inbound references in the same change so completed documents do not survive as accidental
  authorities.
