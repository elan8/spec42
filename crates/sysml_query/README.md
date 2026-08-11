# Semantic query boundary

`sysml_query` is the boundary foundation for the supported consumer facade over the semantic-model
implementation. It owns the opaque published handle and exposes cohesive build, resolution,
navigation, diagnostics, and debug services. It does not expose a graph, semantic node, resolver
state, resolution-fact collection, index handle, generic attribute map, or constructor from partial
semantic state. This first vertical slice does not yet move every consumer or physical query-index
representation into this crate.

The standalone snapshot pipeline is the first complete migrated consumer. Its manifest has no
direct `sysml_model` or `sysml_diagnostics` dependency, so Rust cannot name either implementation
crate through the transitive facade dependency.

The normal `sysml_query` test gate enforces the boundary in three ways:

- Cargo metadata verifies designated consumers depend on `sysml_query`, not `sysml_model`, and
  rejects any change to direct implementation dependencies outside the recorded, shrink-only
  migration inventory. Removing a dependency must remove its inventory entry in the same change.
- A `syn`-based public-API inspection rejects raw storage types, aliases, and public glob exports;
  it also verifies the model publication has no public graph/node/state/index escape hatch.
- Compiler-fail documentation tests prove consumers cannot import raw state/index types, call an
  implementation view, or access the opaque handle's private field.

Only `sysml_query` is an implementation owner allowed to depend directly on `sysml_model` in the
finished architecture. `language_service`, `lsp_server`, `server`, `sysml_diagnostics`, and
`workspace` are named migration debt in an exact transitional inventory; the metadata gate permits
no additions or stale entries. Each is removed when its complete vertical slice migrates—partial
wrappers are not retained as a compatibility surface.

The current `sysml_query -> sysml_diagnostics` dependency exists only to keep canonical diagnostic
calculation and S-expression rendering behind the facade for the migrated snapshot runner.
`sysml_diagnostics` remains in the transitional inventory because it still consumes model-owned
diagnostic projections; this direction is not the finished ownership model and must disappear when
diagnostic services migrate to the facade owner.

The next ownership step is a dependency-complete, owner-scoped construction seed from
`sysml_model`. It must carry only the canonical typed inputs needed to build indexes, not expose a
graph, generic facts collection, or index handle. `sysml_query` can then build and privately own
`SemanticQueryIndexes` before publishing `PublishedModel`, after which the model-owned temporary
index and its forwarding methods are deleted. The manifest gate, rather than compatibility
wrappers, restricts that implementation seam to `sysml_query`.
