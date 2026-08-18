# workspace

Protocol-neutral source admission and immutable publication ownership for Spec42 hosts.

The workspace layer resolves configured libraries, captures a coherent source set, and publishes
one `Arc<PublishedModel>` for that exact input identity. It does not own semantic rules, maintain a
second semantic representation, or patch publications in place.

## Responsibilities

- Resolve configured library archives and installation roots.
- Adapt filesystem and in-memory document providers into immutable query source documents.
- Build a complete `PublishedModel` through `sysml_query`.
- Publish validation results and explicit completeness for the same source revision.
- Replace a snapshot atomically after edits; existing readers may retain the prior snapshot.
- Enforce cancellation, deadlines, resource limits, and publication identity at the host boundary.

Semantic construction belongs to `sysml_resolution`; consumers access it through typed
`sysml_query` services. Full immutable rebuilds are the correctness path. Incremental graph
patching, persistent semantic graph caches, built-in view products, graph-shaped projections, and
semantic snapshot comparison are intentionally absent.

Specialized products such as diagrams should be generator plugins over typed model queries. A
missing query is extended at the semantic owner rather than reconstructed from syntax, names, or
serialized output.
