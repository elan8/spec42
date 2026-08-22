# Production semantic-query follow-ups

The mutable semantic graph and its production consumers are removed. A workspace owns one
dependency-complete `PublishedModel`; validation, editor services, generators, and hosts consume
that same immutable identity through typed queries. This document now contains only missing
capabilities that have an active product need.

| Typed service to add | Required owned result | Disabled consumer until it exists |
|---|---|---|
| Ambiguous-name edit candidates | Canonically ordered candidates with identity, provenance, and authored replacement range | Qualify-name quick fix |
| Importable definitions | Candidates plus owning package and authored import insertion contract | Add-import quick fix |
| Behavior invocation relationships | Typed `perform` endpoints, source ranges, ambiguity, and ordering | Call hierarchy and monikers |
| Bounded structural summary | Purpose-built elements/relationships selected and ordered by the publication | Structural `model-summary`; the command currently reports validation only |
| Diagram render product | Typed view selection and versioned render artifact exposed through generator queries | Built-in diagrams remain removed; replacement belongs in a generator plugin |
| Immutable comparison | Stable-identity typed fact differences with explicit completeness | Workspace semantic comparison |

Full publication rebuilds are the supported path. Immutable incremental construction is not active
work until measurements justify it and its full/cold, supersession, and ordering equivalence can be
proved. Graph patching and graph caches are not compatibility surfaces.

## Diagnostic families this cutover knowingly dropped

Every diagnostic a host reports is settled by `sysml_resolution` and read through
`PublishedModel::diagnostics()`. The legacy graph engine is deleted, and the rules below went with
it **without a replacement**: a model that used to receive these checks no longer does. This is a
knowing, breaking reduction in validation coverage, not a set of normative corrections, and it is
the cost of removing the second engine in one step rather than running both.

Each row names the owning fact the publication would need. None of them requires a new rule --
the rules are small once the fact exists -- so each is a lowering task rather than a design one.

| Rule | Missing owning fact |
|---|---|
| View expose filtering (`view_expose_empty_result`) | A view's `filter` conditions are lowered and its expose targets now are too, but what a filter *admits* is not published, so "the filters remove everything the view exposes" cannot be decided. `view_expose_unresolved` and `view_expose_empty` are owned and reported. |
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
