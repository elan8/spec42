# Semantic query boundary

`sysml_query` is the boundary foundation for the supported consumer facade over the semantic-model
implementation. It owns the opaque published handle and exposes cohesive build, resolution,
navigation, diagnostics, and debug services. It does not expose a graph, semantic node, resolver
state, resolution-fact collection, index handle, generic attribute map, or constructor from partial
semantic state. This first vertical slice does not yet move every consumer or physical query-index
representation into this crate.

The standalone snapshot pipeline now selects only the `immutable-resolution` facade feature. Its
transitive dependency closure is `sysml_query -> sysml_resolution -> parser-next` and cannot reach
`sysml_model` or `sysml_diagnostics`; unsupported syntax and recovery remain explicit incomplete
publications rather than falling back. `workspace_session` stores only `Arc<PublishedModel>` and
validates replacement identity and completeness through the existing typed publication service
while the remaining production consumers migrate.

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

The `legacy-model` facade feature still depends on `sysml_diagnostics` for production consumers that
have not migrated. The immutable snapshot path instead receives parser, canonicalization, and
resolution diagnostics from `sysml_resolution` and streams their canonical S-expression without
exposing storage. Presentation-neutral typed diagnostics remain the intended shared contract for
future CLI, LSP, Markdown, and HTML adapters.

The next ownership step is a dependency-complete, owner-scoped construction seed from
`sysml_model`. It must carry only the canonical typed inputs needed to build indexes, not expose a
graph, generic facts collection, or index handle. `sysml_query` can then build and privately own
`SemanticQueryIndexes` before publishing `PublishedModel`, after which the model-owned temporary
index and its forwarding methods are deleted. The manifest gate, rather than compatibility
wrappers, restricts that implementation seam to `sysml_query`.

The dependency-complete inventory for replacing the production workspace, server, and LSP graph
publication is maintained in [PRODUCTION_CUTOVER.md](PRODUCTION_CUTOVER.md). This tranche does not
claim that production cutover.
