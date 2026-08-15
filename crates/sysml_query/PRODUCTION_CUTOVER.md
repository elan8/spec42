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
| Document diagnostics | Publication identity, document identity, strict/reporting options, library context and unit registry | Canonically ordered typed diagnostics with code, severity, primary range and related locations; unresolved, ambiguous, unsupported and recovery remain distinct | **Producer partial — see the family table below.** `PublishedModel::diagnostics()` answers the resolution-owned families from facts settled at the publication barrier. The conformance families are not in it, so it is not yet a drop-in replacement. Legacy consumers are `workspace::snapshot::facts::collect_host_validation_report`, `lsp_server::analysis`, `lsp_server::lsp_runtime::diagnostics`, and `server::generation`. |
| Element inspection and symbols | Element identity or source position, document scope, authored/effective selection | Typed kind, name, range, ownership, visibility, multiplicity, documentation, effective type/value and provenance; no generic attribute map or all-node iterator | `language_service::{hover,symbol,completion}`, `lsp_server::{language,lsp_runtime::symbols,views::feature_inspector}`. Generator search and inspection are migrated. |
| Navigation and edits | Document identity, position, lookup role, optional selected symbol identity | Definition/declaration targets, references, rename ranges and visible-member candidates with canonical ordering and explicit outcomes | **Migrated.** `language_service::{definition,references,rename,completion}` and the LSP definition/reference/highlight/rename/completion adapters consume one immutable `PublishedModel`; graph lookup and textual reference scanning are deleted from these paths. |
| Relationship queries | Source/target identity, closed relationship kind, authored/effective selection | Direct typed relationships with endpoint identities, provenance and source ranges; authored reference outcome remains separately addressable | `workspace::snapshot::facts`, `lsp_server::views::{model,feature_inspector}`. Generator model queries are migrated. |
| Type queries | Element identity, specialization scope, admitted libraries | Direct types, effective types with their origin, direct and transitive supertypes, direct subtypes, featuring type, and conformance as an explicit `Conforms`/`DoesNotConform`/`Indeterminate` outcome | **Producer landed; generator migrated; other consumers pending.** `PublishedModel::types()` answers these from settled facts. Remaining legacy consumers are `sysml_diagnostics::checks::{kind_compatibility,structural_feature_conformance,view_metadata_conformance}`, `lsp_server::{lsp_runtime::features,views::feature_inspector}`, and `workspace::snapshot::facts`. |
| Evaluation and units | Element or expression identity, evaluation policy, unit catalog committed by the publication | Typed evaluated value/status/unit/dimension and explicit not-run/unsupported/failure states | `workspace::snapshot::facts`, `lsp_server::workspace::services`, hover and feature inspector |
| Workspace projection | Target document identities, normalized workspace/library identities, projection contract version | Purpose-built immutable host projection with canonical element/relationship ordering and provenance; not a reusable general semantic inventory | `HostWorkspaceSnapshot::semantic_projection`, workspace comparison, server validation output |
| Views and diagrams | Selected renderer/view identity, target/workspace scope, library identities, rendering options | Typed view catalog and immutable render plan/payload for general, interconnection, activity, sequence and state views; cache keys include publication identity and options | `workspace::{build_view_catalog,render_view,view_cache}`, `HostWorkspaceSnapshot::prepare_view`, `lsp_server::views`, `server::diagrams` |
| Publication metrics | Publication identity and completed phase counters | Document/byte/node/reference/relationship/index counts and phase completeness, with timings supplied by the host rather than semantic snapshots | workspace resource limits, LSP model statistics and performance reporting |

## Diagnostic families

`sysml_diagnostics` runs eleven check modules over the mutable graph. Three are represented by the
immutable producer; the rest are the remaining diagnostic work.
`tests/architecture.rs::the_cutover_inventory_names_every_legacy_diagnostic_check_family` keeps this
table exhaustive. Migrating a consumer to
`PublishedModel::diagnostics()` before its families land silently stops roughly thirty public codes
from being reported, so this table gates that move.

| `sysml_diagnostics` check family | Immutable owner | State |
|---|---|---|
| `builder_diagnostics` (parser errors, unmodelled constructs) | `sysml_resolution` publication barrier | **Published.** Parser code, severity and range carried through unchanged; unmodelled constructs are the `Unsupported*Member` codes. |
| `name_resolution`, `import_resolution` | `sysml_resolution` publication barrier | **Published** as the authored-reference outcome codes, with ambiguity candidates as related locations. |
| `kind_compatibility` | none yet | Needs published element kinds and specialization/typing conformance per authored relationship. `PublishedModel::types()` supplies the conformance answers; the reporting rule has no owner yet. |
| `structural_feature_conformance` | none yet | Needs redefinition/subsetting conformance, multiplicity, direction, and positional connector-end facts. The legacy check also *writes* implied nodes and edges into the graph, so its implied-relationship half belongs to construction, not to a diagnostic. |
| `view_metadata_conformance` | none yet | Needs published metadata annotation bindings and view expose/rendering targets. Overlaps the views row and should move with it. |
| `behavior_conformance` | none yet | Not yet analysed. |
| `connection_conformance` | none yet | Not yet analysed. |
| `expression_conformance` | none yet | Not yet analysed. |
| `import_conformance` | none yet | Not yet analysed. |
| `requirement_case_conformance` | none yet | Not yet analysed. |

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
