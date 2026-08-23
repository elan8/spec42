# Immutable semantic core architecture

Spec42 admits source documents into one immutable `PublishedModel`. The publication owns semantic
facts, diagnostics, identities, ordering, completeness, and evaluation results for an exact input
identity. Hosts share that publication through `Arc`; they do not rebuild it or maintain a second
semantic representation.

## Ownership

Which crate owns what is stated once, in [`design.md`](../../design.md): two authorities (source,
semantic) behind one facade, five services, and the crate map. This document records the
contracts of the semantic core's query products.

Every consumer of one workspace revision receives the same publication handle. Full rebuilds are
the current correctness path. Incremental graph patching and persistent semantic graph caches were
removed; immutable incremental construction may return only after cold/full equivalence and
supersession behavior are established. The syntax service's parse memo is not a semantic cache: it
holds parsed trees keyed by content digest so that a revision is parsed once, and it changes no
semantic answer.

## Identity and qualified references

`SymbolIdentity` is the canonical identity of a declaration inside a publication. Its encoding is
opaque to consumers and commits the normalized document identity plus the declaration's typed
ownership path and occurrence. It is stable across sequential and parallel builds of identical
inputs. Renaming or moving an element, changing its document identity, or changing the versioned
identity contract may deliberately change it. A host must also retain the publication/model digest:
an identity from another publication is not thereby current.

A KerML `qualifiedName` is a readable semantic reference, not an identity. The resolution-owned
`QualifiedElementReference` query accepts a qualified name, an optional normalized document
identity, and an optional expected element kind. Document-scoped lookup distinguishes identical
root/package names authored in different documents. Publication-wide lookup includes workspace,
standard-library, other library, and external source domains and reports multiple matches as typed
ambiguity. Resolution returns the canonical `SymbolIdentity`; unresolved, wrong-kind, recovery,
unsupported, and incomplete publications remain distinct outcomes. Adapters must not reproduce
this lookup by scanning catalog labels.

OMG `Element::elementId` is a separate tool-owned identity domain. An imported API/XMI UUID may be
preserved as provenance or an alias, but it is not interchangeable with `SymbolIdentity`, a
qualified name, a source URI, or a generator handle. Standard/external library documents likewise
retain their admitted source identity and provenance; a matching qualified name does not merge
them with a workspace declaration.

## View selection

Candidate-dependent view conditions are semantic facts owned by `sysml_resolution`. The typed
`view_selection` query applies every owned and inherited view condition to one exposed candidate.
Supported metadata-classification predicates retain their resolved metadata identities, Boolean
`and`/`or` uses three-valued evaluation, and separate conditions are conjunctive. The result is
included, excluded, or explicitly indeterminate with unresolved, ambiguous, or unsupported
predicate evidence. Diagram generators and hosts consume that answer; they do not inspect filter
syntax or metadata display names.

## Deliberately disabled products

- Diagram layout and render caches remain outside the semantic core. The resolution-owned typed
  view catalog and projections are consumed by a repository-owned generator plugin, which emits a
  versioned render artifact without exposing internal publication identities.
- Graph-shaped model DTOs and semantic snapshot comparison are removed. A future comparison product
  must compare typed facts by stable identity.
- Call hierarchy and monikers are disabled until the publication owns typed behavior/`perform`
  relationships.
- `model-summary` is validation-only. A bounded structural summary requires its own typed query;
  hosts must not reconstruct one from display names or serialized output.
- Import and ambiguous-name quick fixes are disabled until typed queries provide candidates,
  provenance, and authored replacement/insertion ranges.

These are unsupported states, not compatibility gaps hidden by a fallback. New semantic capability
belongs first in `sysml_resolution`, then in a typed `sysml_query` contract, and only then in a host.
