# Semantic query boundary

`sysml_query` is the boundary foundation for the supported consumer facade over the semantic-model
implementation. It owns the opaque published handle and exposes cohesive build, resolution,
navigation, diagnostics, and debug services. It does not expose a graph, semantic node, resolver
state, resolution-fact collection, index handle, generic attribute map, or constructor from partial
semantic state. The navigation/edit vertical slice now exposes typed definition/declaration
targets, reverse references, collision-aware rename ranges, and effective visible-member
candidates from the parser-owned immutable publication. Language-service and LSP adapters consume
these services without graph or textual-lookup fallbacks. Physical query indexes remain private
implementation details.

`PublishedModel::types()` answers direct types, effective types with their origin, direct and
transitive supertypes, direct subtypes, featuring types, and conformance. Conformance is an
explicit `Conforms`/`DoesNotConform`/`Indeterminate` outcome rather than a boolean, so an
unresolved or cyclic hierarchy cannot be reported as a type violation. `all_supertypes` is
reflexive, matching the OMG Pilot's `allSupertypes`, and the specialization scope is a query
parameter so one closure answers both the Pilot's all-subkinds reading and the narrower
classifier-only one. The producers behind it are published at the barrier.

`PublishedModel::element_details()` answers everything about one element at once: its inspection,
what each authored relationship family settled to, the types it has once inheritance is taken into
account, the features it inherits and the type that declares each, its metadata bindings, both
directions of its relationships with authored/implied provenance, and both evaluation channels.
It exists because assembling that view from the individual services made each consumer decide how
they relate to one another, and every consumer decided differently. A family's outcome is a closed
`RelationshipOutcome`, so an empty target list never means both "nothing was authored" and "what
was authored did not resolve", a partly resolved family cannot present as resolved, and an
ambiguous one keeps every candidate without promoting one to a target. The LSP feature inspector is
a projection of this and nothing else.

`PublishedModel::evaluation()` answers one cohesive question per element: what its authored
expression settled to, the unit tokens it wrote with their own ranges and outcomes, and the
measurement reference its type requires. A unit is a declaration -- the `SI::kilogram` an authored
`[kg]` names -- and its dimension is the measurement-reference type it is an instance of, so
comparing dimensions is the same specialization question every other type query asks rather than a
string comparison. Whether the catalog is admitted at all, whether a symbol is unknown, ambiguous,
or a unit expression this engine does not decompose, are each their own outcome, so a workspace
built without the measurement libraries reports that it has none rather than that its units are
wrong. Element inspection projects the same settled evaluation state, so the two cannot disagree.

Publications admit the whole configured library set. `LibraryStratum::build` parses and solves a
library once so later publications reuse it, which is what makes admitting the standard library
affordable on a rebuild-per-keystroke host. Reuse is discarded whenever workspace roots could
change a settled library answer.

The facade has one dependency. Its transitive closure is
`sysml_query -> sysml_resolution -> { sysml-v2-parser, sysml_source -> source_identity }` and cannot
reach `sysml_diagnostics` or any host crate; unsupported syntax and recovery remain explicit
incomplete publications rather than falling back. There is no feature to select, so no consumer can
opt back in. Beyond the publication queries, the facade exposes the source, syntax, library-closure
and publication services a host obtains from one `Services` value; see `design.md`.
`sysml_query::publication::PublicationSession` stores only `Arc<PublishedModel>` and admits a
replacement only when its identity is exactly the one the build was started for.

The normal `sysml_query` test gate enforces the boundary in four ways:

- Cargo metadata verifies the facade's own dependency set is exactly `sysml_resolution` and that it
  declares no features, pins the exact dependency sets of the authority and host crates, and
  verifies every designated consumer depends on `sysml_query` and on no authority crate.
- A `syn`-based public-API inspection rejects parser types, raw storage types, aliases, and public
  glob exports; it also verifies the model publication has no public graph/node/state/index escape
  hatch.
- `tests/syntax_authority.rs` rejects consumers that re-answer syntax questions from text: retired
  helpers, shadowed service queries, parsed trees or caches held outside the authorities, SysML
  file reads, and string probes against reserved keywords, qualified names, operators or braces.
- Compiler-fail documentation tests prove consumers cannot import raw state/index types, call an
  implementation view, or access the opaque handle's private field.

Diagnostics are published as typed values, and this is now the whole validation surface a host
reports. `PublishedModel::diagnostics()` returns the codes, severities, ranges, messages, subject
identities, and related locations `sysml_resolution` settled at the publication barrier;
`for_document` answers one document from a prebuilt index, so the cost is proportional to what is
returned and repeating the query or reordering documents changes nothing. The canonical
S-expression is one adapter over these values rather than their only representation. That is the
shared contract workspace validation, the LSP, the CLI, Markdown, and HTML adapters consume; none
of them recovers a fact by parsing presentation text, and none of them runs a rule of its own.
`sysml_diagnostics` is the neutral rendering shape plus the one explicit reporting policy a host
has -- report only what the parser rejected for a document that does not parse -- and depends on
this facade alone.
