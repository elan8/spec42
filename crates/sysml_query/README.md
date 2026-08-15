# Semantic query boundary

`sysml_query` is the boundary foundation for the supported consumer facade over the semantic-model
implementation. It owns the opaque published handle and exposes cohesive build, resolution,
navigation, diagnostics, and debug services. It does not expose a graph, semantic node, resolver
state, resolution-fact collection, index handle, generic attribute map, or constructor from partial
semantic state. The navigation/edit vertical slice now exposes typed definition/declaration
targets, reverse references, collision-aware rename ranges, and effective visible-member
candidates from the parser-owned immutable publication. Language-service and LSP adapters consume
these services without graph or textual-lookup fallbacks. Other cutover rows and some physical
query-index representations remain transitional.

`PublishedModel::types()` answers direct types, effective types with their origin, direct and
transitive supertypes, direct subtypes, featuring types, and conformance. Conformance is an
explicit `Conforms`/`DoesNotConform`/`Indeterminate` outcome rather than a boolean, so an
unresolved or cyclic hierarchy cannot be reported as a type violation. `all_supertypes` is
reflexive, matching the OMG Pilot's `allSupertypes`, and the specialization scope is a query
parameter so one closure answers both the Pilot's all-subkinds reading and the narrower
classifier-only one. The producers behind it are published at the barrier; the legacy graph's
`specializes_transitively`/`feature_typing_conforms` and their consumers migrate with the
diagnostics and views slices.

Publications admit the whole configured library set. `LibraryStratum::build` parses and solves a
library once so later publications reuse it, which is what makes admitting the standard library
affordable on a rebuild-per-keystroke host; see `planning/RESOLUTION_LAYER_DESIGN.md` §5.5.1 for
the conditions under which that reuse is discarded.

The facade has one dependency. Its transitive closure is
`sysml_query -> sysml_resolution -> parser-next` and cannot reach `sysml_model` or
`sysml_diagnostics`; unsupported syntax and recovery remain explicit incomplete publications rather
than falling back. There is no feature to select, so no consumer can opt back in. `workspace_session`
stores only `Arc<PublishedModel>` and validates replacement identity and completeness through the
typed publication service while the remaining production consumers migrate.

The normal `sysml_query` test gate enforces the boundary in three ways:

- Cargo metadata verifies the facade's own dependency set is exactly `sysml_resolution` and that it
  declares no features, verifies designated consumers depend on `sysml_query` rather than
  `sysml_model`, and rejects any change to direct implementation dependencies outside the recorded,
  shrink-only migration inventory. Removing a dependency must remove its inventory entry in the same
  change.
- A `syn`-based public-API inspection rejects raw storage types, aliases, and public glob exports;
  it also verifies the model publication has no public graph/node/state/index escape hatch.
- Compiler-fail documentation tests prove consumers cannot import raw state/index types, call an
  implementation view, or access the opaque handle's private field.

No crate may depend directly on `sysml_model` in the finished architecture. `language_service`,
`lsp_server`, `server`, `sysml_diagnostics`, and `workspace` are named migration debt in an exact
transitional inventory; the metadata gate permits no additions or stale entries. Each is removed
when its complete vertical slice migrates—partial wrappers are not retained as a compatibility
surface.

Diagnostics are published as typed values. `PublishedModel::diagnostics()` returns the codes,
severities, ranges, and related locations `sysml_resolution` settled at the publication barrier, and
the canonical S-expression is one adapter over them rather than their only representation. That is
the shared contract CLI, LSP, Markdown, and HTML adapters consume; none of them recovers a fact by
parsing presentation text.

The dependency-complete inventory for replacing the production workspace, server, and LSP graph
publication is maintained in [PRODUCTION_CUTOVER.md](PRODUCTION_CUTOVER.md). This tranche does not
claim that production cutover.
