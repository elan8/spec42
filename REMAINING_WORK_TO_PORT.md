# Remaining Work to Port

This file tracks useful compatibility work that is deliberately deferred from the current pull
request. It is repository-neutral: the intended end state must be understandable and maintainable
from Spec42's own contracts, tests, and normative references.

The current pull request is scope-frozen. Only work already in progress should land there. New
items below should be taken in separate, focused pull requests and removed from this file when their
owning tests are active.

## Completion contract

Every item must follow these rules:

- Add or extend the authoritative typed fact at the earliest semantic layer that has all required
  inputs. Consumers must not reconstruct it from source text, display names, debug output, or
  generic attribute maps.
- Keep authored, effective, implied, inherited, and evaluated provenance distinct.
- Publish unresolved, ambiguous, unsupported, partial, and not-run outcomes explicitly.
- Prove deterministic full/incremental equivalence and invalidation where the result can cross a
  snapshot or cache boundary.
- Add the narrow owning-layer regression first, then affected diagnostics, host projections,
  generators, LSP surfaces, and canonical semantic-graph snapshots.
- Keep every skip executable and give it a concrete capability or fixture reason. Remove the skip
  when the capability lands.

## Typed builder facts

Several grammar productions are addressable but do not yet publish enough typed declared facts for
later semantic rules:

- Interface usages and enumeration usages need parser-backed `DeclaredFeatureProperties`.
- Objective, assert-constraint, and require-constraint builders need typed feature properties
  instead of legacy attributes.
- Performed actions, `perform` steps, and view-rendering usages need typed feature properties.
- Requirement/case constraint members, assignments, filters, and transition subfeatures still have
  consumers that depend on generic attribute payloads. Introduce typed facts before migrating those
  consumers.
- Opaque KerML fallback declarations need structured parser output before their relationship
  endpoints, members, or declaration roles can enter semantic construction. Do not parse their
  retained text in Spec42 as a substitute.

Activate the ignored ownership regressions in
`crates/sysml_model/tests/implied_semantic_facts.rs` as the first acceptance gate.

## Implied relationships

The graph-owned relationship-provenance foundation should be extended with named, typed rules for:

- context-dependent subsettings for nested parts, items, occurrences, ports, connections, actions,
  states, calculations, cases, requirements, views, renderings, constraints, and performed actions;
- requirement assumptions, required constraints, concerns, included use cases, snapshots,
  time-slices, exclusive states, and requirement verifications;
- same-name inherited-member redefinitions;
- positional connector-end and behavior-parameter redefinitions;
- role-based subject, objective, entry, do, and exit redefinitions;
- association-structure connector subsetting overrides;
- implied type-featuring relationships for features nested below other features;
- implied parameter directions where the declared syntax leaves them unspecified;
- binding relationships to stable expression-result identities, including satisfaction bindings;
  and
- deterministic pruning of redundant implied relationships without removing authored evidence.

Each rule needs a closed rule identity, explicit prerequisite-resolution status, cycle/convergence
handling, authored-equivalent behavior, and full/incremental/merge/removal parity. All resolved
relationships must enter the canonical graph edge store rather than a parallel host-only list.

## Structural and type validation

The following checks remain incomplete because their canonical prerequisites are incomplete or
only partially projected:

- connector end referential constraints and inherited end closure;
- exact binary-end rules for connection-like, allocation, interface, and flow definitions/usages;
- flow payload occurrence/type conformance;
- redefinition featuring-type conformance;
- computed multiplicity-bound validation beyond directly declared literal bounds;
- cross-category namespace-member distinguishability;
- membership owning-type and composite restrictions where builders do not yet publish typed
  membership/feature facts; and
- canonical classification of sequence roles from library/profile identity.

Diagnostics must add stable catalog codes, exact ranges, severity and ordering tests, related
information where useful, and conformance-metadata updates. Do not infer these rules from node names
or diagnostic messages.

## Expressions and units

- Publish unit declarations, dimensions, conversions, and unit-reference resolution as typed graph
  facts. Unit-bearing evaluation must remain explicitly unsupported until those facts exist.
- Complete typed expression coverage for conditional, collection, selection, indexing, cast,
  classification, constructor, extent, and metadata-access forms as parser facts permit.
- Replace remaining expression-related semantic reads of generic attributes in filters,
  requirements/cases, assignments, and transitions.
- Preserve distinct malformed, unresolved, ambiguous, unsupported, type-error, cycle, and
  division-by-zero outcomes through diagnostics and all presentation boundaries.

## Imports, exposes, and visibility

- Implement filtered-import expansion from typed filter expressions. Until then the typed
  unsupported result and diagnostic must remain active.
- Give Expose its complete graph-owned scope/expansion result if any consumer still relies on a
  separate resolver; do not treat Expose as Import or inherit Import visibility defaults.
- Extend namespace classification only from typed declaration roles. The opaque KerML declaration
  bucket must never be treated wholesale as a namespace.
- Add broader re-export, recursive-cycle, ambiguity, invalidation, and cached/full/incremental
  parity matrices as new import shapes become supported.

## Cache and host rebuild parity

- Add a cold-build/store/warm-load regression that compares universal standard-library
  relationship identity, resolution status, and authored/implied provenance before and after the
  library-graph cache boundary. The existing cache-key tests prove that standard-library root
  classification changes cache identity; this drill must prove the loaded semantic result itself
  is equivalent and that a generic-library classification cannot reuse the entry.
- Add an LSP rebuild regression covering cold startup and live relinking with the same standard and
  generic library roots. It must prove that only configured standard-library roots can satisfy
  standard-library prerequisites and that both publication paths expose identical relationship
  status and provenance.

## Compatibility corpus

- Keep all discovered fixtures accounted for by the harness.
- Replace semantic-graph skip metadata as structured parser or semantic facts become available.
- The single non-UTF-8 fixture needs an explicit byte-input parser/recovery contract before it can
  be exercised like the UTF-8 corpus; it must remain a named skip until then.
- Golden regeneration remains a deliberate maintenance command. The normal test path must assert
  exact semantic graph, diagnostic, and formatter output and must reject stale or unexplained skip
  metadata.

## Explicit non-goals

The following are not deferred compatibility work for this effort and should not be added to this
file as implementation tasks:

- RTM generation;
- runtime or scripting integration; and
- JSON model interchange.
