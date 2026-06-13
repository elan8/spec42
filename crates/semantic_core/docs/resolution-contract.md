# Semantic resolution contract

This document defines how Spec42 resolves names in the merged semantic graph. Contract tests live in `tests/resolution_contract.rs`.

## Pipeline

1. **Materialize** — `build_graph_from_doc` per URI (syntax only).
2. **Merge** — workspace + library documents.
3. **Link** — `link_workspace_relationships` (typing, specializes, derivation wiring).
4. **Evaluate prep** — `prepare_analysis_evaluation_context`.
5. **Pending resolve** — `resolve_workspace_pending_relationships` (expression + qualified pending edges).
6. **Diagnostics** — `collect_diagnostics_from_graph`.

Entry point: `build_and_link_graph` in `semantic/pipeline.rs`.

## Resolver entry points

| Intent | Function | Allowed kinds | Used by |
| --- | --- | --- | --- |
| Type reference | `resolve_type_reference_targets` | `ResolutionRole::Typing` / `Specializes` | Link, diagnostics Rule 6/7 |
| Expression endpoint | `resolve_expression_endpoint_strict` | graph node lookup | Pending expression edges, graph build |
| Import target (diag) | `import_target_resolves` | namespace + export walk | Import diagnostics |
| Simple import name | `resolve_imported_node_ids_for_simple_name` | per caller | Pending QN fallback |
| Member / inherit | `resolve_member_via_type` | feature kinds | Evaluation, redefines |

Unified role allowlists: `kinds::allowed_for_role(ResolutionRole::…)`.

## Deliberate policies

### SysML library bootstrap

When workspace content contains `SysML::` qualified references, or imports `sysml` / `sysml::*`, the library closure seeds the `SysML` package (and optionally full stdlib slice). See `library_loader.rs`.

### Parent-qualified sibling imports

Inside package `P`, import `Sibling::*` resolves to `P::Sibling::*` before a top-level `Sibling` lookup. Implemented in `import_namespace_target_candidates`.

### Metadata restrictions

Attributes with `subsetsFeature` (metadata restriction shorthand) use reflective KerML/SysML typing targets; normal attribute typing rules do not apply.

### Derivation connections

`#derivation connection` produces a `Derivation` edge between requirement endpoints. No structural `Connection` edge is emitted between those endpoints.

## Regression gate

Run before merging changes to `semantic_core`:

```powershell
cargo test -p semantic_core resolution_contract
cargo test -p semantic_core --test p2_diagnostics_semantics
cargo test -p semantic_core --test import_namespace_semantics
cargo test -p semantic_core --test metadata_semantics
cargo test -p semantic_core --test requirement_derivation_semantics
```
