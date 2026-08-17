# Production semantic-query cutover inventory

This is the dependency-complete inventory for replacing the mutable `SemanticGraph` publication
used by `HostWorkspaceSnapshot`, `server`, and `lsp_server`. It records required typed services,
not proposed raw graph access. A production consumer is removed from the transitional Cargo
inventory only when every row it uses is served by one coherent `PublishedModel` identity.

`sysml_query` itself is out. Its normal dependency closure is exactly `sysml_resolution`, which
`tests/architecture.rs::facade_depends_only_on_the_immutable_resolution_owner` pins, so no consumer
of the facade can reach the mutable graph. The crates below reach it directly, by their own
manifests, and are the remaining work.

| Typed service | Required inputs committed by the publication | Required result contract | Current production owners to migrate |
|---|---|---|---|
| Source admission and publication | URI, content, source kind, parser recovery, semantic contract version, evaluation policy, standard-library and external-library inputs | Opaque complete/recovery publication, dependency-complete identity, document identities and source ranges, structural counts for resource limits | `workspace::snapshot::{build,update}` publish alongside the graph and keep the publication on the snapshot; `workspace::IncrementalWorkspace` and `lsp_server::workspace::{services,rebuild,library_closure}` still build the graph for the unmigrated rows below |
| Element inspection and symbols | Element identity or source position, document scope, authored/effective selection | Typed kind, name, range, ownership, visibility, multiplicity, documentation, effective type/value and provenance; no generic attribute map or all-node iterator | **Producer landed as `element_details`.** One cohesive answer per element: inspection, relationship families, effective typing, inherited features with their declaring type, metadata bindings, both relationship directions, and both evaluation channels. `lsp_server::views::feature_inspector` consumes it and its wire contract carries typed fields rather than an attribute map. Remaining legacy consumers are `language_service::{symbol,presentation_hover,code_actions}` and `lsp_server::language`, which render from `SemanticNode`. |
| Navigation and edits | Document identity, position, lookup role, optional selected symbol identity | Definition/declaration targets, references, rename ranges and visible-member candidates with canonical ordering and explicit outcomes | **Migrated.** `language_service::{definition,references,rename,completion}` and the LSP definition/reference/highlight/rename/completion adapters consume one immutable `PublishedModel`; graph lookup and textual reference scanning are deleted from these paths. |
| Relationship queries | Source/target identity, closed relationship kind, authored/effective selection | Direct typed relationships with endpoint identities, provenance and source ranges; authored reference outcome remains separately addressable | **Producer landed; the inspector is migrated.** `element_details` publishes each authored family as a closed `RelationshipOutcome` with its settled targets and its ambiguous candidates kept apart, plus both relationship directions with authored/implied provenance. Remaining legacy consumers are `workspace::snapshot::facts`'s host projection and `lsp_server::views::model`. Generator model queries are migrated. |
| Type queries | Element identity, specialization scope, admitted libraries | Direct types, effective types with their origin, direct and transitive supertypes, direct subtypes, featuring type, positional connector-end counts, KerML type-relationship operands, and conformance as an explicit `Conforms`/`DoesNotConform`/`Indeterminate` outcome | **Producer landed; generator, every diagnostic family, the feature inspector and type hierarchy migrated.** `PublishedModel::types()` answers these from settled facts. The remaining legacy consumer is `workspace::snapshot::facts`'s host projection. |
| Evaluation and units | Element or expression identity, evaluation policy, unit catalog committed by the publication | Typed evaluated value/status/unit/dimension, an explicit verdict channel, and explicit not-run/unsupported/failure states | **Producer landed; hover, the feature inspector and the analysis diagnostics migrated.** `PublishedModel::evaluation()` answers value, state, authored unit tokens with their resolution, and the measurement reference a feature's type requires. The remaining legacy consumer is `workspace::snapshot::facts`, which reads `SemanticGraph::evaluation_facts_for` by graph node for the host projection. |
| Workspace projection | Target document identities, normalized workspace/library identities, projection contract version | Purpose-built immutable host projection with canonical element/relationship ordering and provenance; not a reusable general semantic inventory | `HostWorkspaceSnapshot::semantic_projection`, workspace comparison, server validation output |
| Views and diagrams | Selected renderer/view identity, target/workspace scope, library identities, rendering options | Typed view catalog and immutable render plan/payload for general, interconnection, activity, sequence and state views; cache keys include publication identity and options | `workspace::{build_view_catalog,render_view,view_cache}`, `HostWorkspaceSnapshot::prepare_view`, `lsp_server::views`, `server::diagrams` |
| Publication metrics | Publication identity and completed phase counters | Document/byte/node/reference/relationship/index counts and phase completeness, with timings supplied by the host rather than semantic snapshots | workspace resource limits, LSP model statistics and performance reporting |

## Diagnostic facts the publication does not yet own

Every diagnostic a host reports is settled by `sysml_resolution` and read through
`PublishedModel::diagnostics()`. The families below are the rules the legacy engine ran that this
publication cannot state, so they are absent rather than approximated. Each names the missing
owning fact.

| Rule | Missing owning fact |
|---|---|
| View expose targets (`view_expose_unresolved`, `view_expose_empty`, `view_expose_empty_result`) | `ExposeMember` is not lowered, so a view's expose targets and their resolution are not published. It shares `ImportTarget`'s shape, so it lowers like an import once the view row moves. |
| Metadata `about` targets and body bindings (`metadata_about_unresolved`, `metadata_binding_missing`, `metadata_binding_unknown`, `metadata_annotated_element_incompatible`) | An annotation's `about` clause and its body's feature-value overrides are not lowered; only the annotation's own target reference is. |
| User-defined keyword resolution (`metadata_keyword_unresolved`, `metadata_keyword_collision`) | The `#keyword` prefix a declaration carries is not published as a fact. |
| Case objective and verdict shape (`objective_binding_unresolved`, `invalid_verdict_value`, `verification_case_invalid_shape`, `case_subject_missing`, `case_objective_binding_cardinality`, `requirement_constraint_invalid_membership`) | An `objective`'s binding kind, a `verdict`'s authored token, and a case's analysis-result cardinality are not lowered; an objective's body lowers as an ordinary requirement usage. |
| Initial-transition cardinality (`multiple_initial_states`) | The pinned parser gives a bare `then <state>;` initial marker and a `then <state>;` continuation the same shape, so counting them would report a continuation as a second initial transition. |
| Allocation usage typing (`allocation_type_not_allocation_def`) | There is no allocation-usage declaration kind, so there is no declaration whose typing to judge. |
| Import kind and recursion conformance (`import_kind_mismatch`, `invalid_recursive_import`, `invalid_import_target`) | A non-namespace or recursive import target settles as unresolved or unsupported before a kind question arises, so the rule has no reachable input. |

Codes the publication deliberately consolidated rather than dropped, because it settles every
authored reference the same way: `unresolved_ref_type_reference`, `unresolved_satisfy_source`,
`unresolved_satisfy_target`, `unresolved_allocate_source`, `unresolved_allocate_target`,
`unresolved_viewpoint_conformance_target`, `unresolved_connection_segment`,
`assignment_target_unresolved` and `invalid_qualified_name_segment` are reported as
`unresolved_reference`/`unresolved_type_reference` at the same range;
`ambiguous_name_reference` as `ambiguous_reference`; `conjugated_port_inconsistent` and
`flow_item_type_incompatible` as `port_type_mismatch` or `flow_direction_incompatible`, whichever
the settled conjugation and directions decide. `unresolved_pending_relationship` and
`unresolved_pending_expression_relationship` described the graph's own pending queues and have no
counterpart: every authored reference in a publication has a settled outcome.

`crates/server/src/diagnostic_catalog.rs` documents exactly `DiagnosticCode::SEMANTIC`, checked
both ways by `the_catalog_documents_exactly_the_published_codes`.

## Remaining graph consumers

The production barrier must publish sources, semantic answers, diagnostics, projections and view
products for the same identity. `HostWorkspaceSnapshot::semantic_graph()` and
`semantic_graph_arc()`, LSP `DocumentStore::{semantic_graph,semantic_graph_mut}`, incremental graph
patching, and model/view helpers accepting `&SemanticGraph` are deletion targets, not compatibility
surfaces. During migration there must not be a `PublishedModel` beside a mutable graph with callers
free to choose between them; each vertical slice moves its complete producer and consumers, then
deletes the old access path.

LSP call hierarchy and monikers (`lsp_server::lsp_runtime::features`) stay on the graph. They are a
different semantic product -- `perform` relationships between behaviours -- that no published row
answers, so they are inventoried here rather than served by widening a type or relationship query
into a general graph traversal.

The facade's one module is still named `resolved_slice`, which distinguished it from the legacy
facade that no longer exists. Renaming it is a mechanical change across every consumer and is
deliberately not bundled with a dependency cut.
