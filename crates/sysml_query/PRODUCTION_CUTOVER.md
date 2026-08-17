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
| Source admission and publication | URI, content, source kind, parser recovery, semantic contract version, evaluation policy, standard-library and external-library inputs | Opaque complete/recovery publication, dependency-complete identity, document identities and source ranges, structural counts for resource limits | `workspace::snapshot::{build,update}`, `workspace::IncrementalWorkspace`, `lsp_server::workspace::{services,rebuild,library_closure}` |
| Document diagnostics | Publication identity, document identity, strict/reporting options, library context | Canonically ordered typed diagnostics with code, severity, primary range and related locations; unresolved, ambiguous, unsupported and recovery remain distinct | **Producer partial — see the family table below.** `PublishedModel::diagnostics()` answers the resolution-owned families, the two feature-conformance families and expression conformance from facts settled at the publication barrier. The remaining conformance families are not in it, so it is not yet a drop-in replacement. Legacy consumers are `workspace::snapshot::facts::collect_host_validation_report`, `lsp_server::analysis`, `lsp_server::lsp_runtime::diagnostics`, and `server::generation`. |
| Element inspection and symbols | Element identity or source position, document scope, authored/effective selection | Typed kind, name, range, ownership, visibility, multiplicity, documentation, effective type/value and provenance; no generic attribute map or all-node iterator | `language_service::{hover,symbol,completion}`, `lsp_server::{language,lsp_runtime::symbols,views::feature_inspector}`. Generator search and inspection are migrated. |
| Navigation and edits | Document identity, position, lookup role, optional selected symbol identity | Definition/declaration targets, references, rename ranges and visible-member candidates with canonical ordering and explicit outcomes | **Migrated.** `language_service::{definition,references,rename,completion}` and the LSP definition/reference/highlight/rename/completion adapters consume one immutable `PublishedModel`; graph lookup and textual reference scanning are deleted from these paths. |
| Relationship queries | Source/target identity, closed relationship kind, authored/effective selection | Direct typed relationships with endpoint identities, provenance and source ranges; authored reference outcome remains separately addressable | `workspace::snapshot::facts`, `lsp_server::views::{model,feature_inspector}`. Generator model queries are migrated. |
| Type queries | Element identity, specialization scope, admitted libraries | Direct types, effective types with their origin, direct and transitive supertypes, direct subtypes, featuring type, positional connector-end counts, KerML type-relationship operands, and conformance as an explicit `Conforms`/`DoesNotConform`/`Indeterminate` outcome | **Producer landed; generator and the feature-conformance families migrated; other consumers pending.** `PublishedModel::types()` answers these from settled facts. Remaining legacy consumers are `sysml_diagnostics::checks::view_metadata_conformance`, `lsp_server::{lsp_runtime::features,views::feature_inspector}`, and `workspace::snapshot::facts`. |
| Evaluation and units | Element or expression identity, evaluation policy, unit catalog committed by the publication | Typed evaluated value/status/unit/dimension and explicit not-run/unsupported/failure states | **Producer landed; hover migrated; the remaining consumers are node-keyed.** `PublishedModel::evaluation()` answers value, state, authored unit tokens with their resolution, and the measurement reference a feature's type requires. Hover reads it for both the evaluated value and the unit literal, and the graph-derived unit catalog those paths used is deleted. `lsp_server::views::feature_inspector` and `lsp_server::lsp_runtime::symbols` still read `SemanticGraph::evaluation_facts_for`: they address elements by graph node rather than by published identity, and the inspector also renders the analysis/verification evaluation, which this row does not publish. `workspace::snapshot::facts` and `lsp_server::workspace::services` project the same graph facts. |
| Workspace projection | Target document identities, normalized workspace/library identities, projection contract version | Purpose-built immutable host projection with canonical element/relationship ordering and provenance; not a reusable general semantic inventory | `HostWorkspaceSnapshot::semantic_projection`, workspace comparison, server validation output |
| Views and diagrams | Selected renderer/view identity, target/workspace scope, library identities, rendering options | Typed view catalog and immutable render plan/payload for general, interconnection, activity, sequence and state views; cache keys include publication identity and options | `workspace::{build_view_catalog,render_view,view_cache}`, `HostWorkspaceSnapshot::prepare_view`, `lsp_server::views`, `server::diagrams` |
| Publication metrics | Publication identity and completed phase counters | Document/byte/node/reference/relationship/index counts and phase completeness, with timings supplied by the host rather than semantic snapshots | workspace resource limits, LSP model statistics and performance reporting |

## Diagnostic families

`sysml_diagnostics` runs seven check modules over the mutable graph. The kind-compatibility,
structural-feature-conformance and expression-conformance families are gone from it: the immutable
publication owns them, and their modules, helpers, kind tables and graph-derived unit catalog are
deleted rather than bridged.
`tests/architecture.rs::the_cutover_inventory_names_every_legacy_diagnostic_check_family` keeps this
table exhaustive. Migrating a consumer to
`PublishedModel::diagnostics()` before its families land silently stops the public codes they own
from being reported, so this table gates that move.

| `sysml_diagnostics` check family | Immutable owner | State |
|---|---|---|
| `builder_diagnostics` (parser errors, unmodelled constructs) | `sysml_resolution` publication barrier | **Published.** Parser code, severity and range carried through unchanged; unmodelled constructs are the `Unsupported*Member` codes. |
| `name_resolution`, `import_resolution` | `sysml_resolution` publication barrier | **Published** as the authored-reference outcome codes, with ambiguity candidates as related locations. |
| `view_metadata_conformance` | none yet | Needs published metadata annotation bindings and view expose/rendering targets. Overlaps the views row and should move with it. |
| `behavior_conformance` | none yet | Not yet analysed. |
| `connection_conformance` | none yet | Not yet analysed. |
| `import_conformance` | none yet | Not yet analysed. |
| `requirement_case_conformance` | none yet | Not yet analysed. |

`sysml_diagnostics::engine_impl` also reports `inherited_attribute_value_type_mismatch` from an
inline rule outside the check modules. It is not the migrated value rule and does not overlap it:
the publication judges an authored value against the feature's *effective* types, and this rule
covers the case where a usage's value is bound to a member it inherits without the publication
deriving a redefinition for it, so the two never report the same declaration. It moves with the
inline engine rules rather than with this family.

The production barrier must publish sources, semantic answers, diagnostics, projections and view
products for the same identity. `HostWorkspaceSnapshot::semantic_graph()` and
`semantic_graph_arc()`, LSP `DocumentStore::{semantic_graph,semantic_graph_mut}`, incremental graph
patching, and model/view helpers accepting `&SemanticGraph` are deletion targets, not compatibility
surfaces. During migration there must not be a `PublishedModel` beside a mutable graph with callers
free to choose between them; each vertical slice moves its complete producer and consumers, then
deletes the old access path.

`sysml_diagnostics` remains only because `workspace`, `lsp_server`, and `server` still evaluate its
checks over the mutable graph. It is not reachable from `sysml_query`, so it is now an unmigrated
consumer of the graph rather than part of the facade's contract. The family table above is the
complete inventory of what it still owns.

The facade's one module is still named `resolved_slice`, which distinguished it from the legacy
facade that no longer exists. Renaming it is a mechanical change across every consumer and is
deliberately not bundled with a dependency cut.
