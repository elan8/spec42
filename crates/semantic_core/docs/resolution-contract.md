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

### Typing/specialization library closure

IBD and other graph consumers traverse `Typing` / `Specializes` edges into library part definitions. The loader therefore also seeds closure from type references in workspace and loaded library bodies (part usages, defs, ports, attributes). This is narrower than a full library scan but wider than import-only closure. Controlled by `LibraryClosureOptions::bootstrap_typing_references` (default `true`).

### Parent-qualified sibling imports

Inside package `P`, import `Sibling::*` resolves to `P::Sibling::*` before a top-level `Sibling` lookup. Implemented in `import_namespace_target_candidates`.

### Metadata restrictions

Metadata def restriction features (`annotatedElement`, `baseType`) use reflective KerML/SysML typing targets; normal attribute typing rules do not apply.

- Subset shorthand (`:> feature : Type`) sets `subsetsFeature` on the attribute node.
- Redefine shorthand (`:>> feature : Type`) sets `redefines` and projects the same `subsetsFeature` when the parent is a `metadata def`.
- Diagnostics skip Rule 6 and incompatible typing for `kinds::is_metadata_restriction_attribute`.
- Resolved `SysML::…` targets whose element kind is `metadata def` or `kermlDecl` are allowed via `kinds::is_reflective_sysml_usage_type`.

KerML `SemanticMetadata` from library sources is materialized as `metadata def` with `metaclassRole: "SemanticMetadata"` (see `package_body.rs`).

### Derivation connections

`#derivation connection` produces a `Derivation` edge between requirement endpoints. No structural `Connection` edge is emitted between those endpoints.

### Unit resolution

Unit literals in attribute values (`10 [kV]`) resolve through `UnitRegistry::from_graph`:

1. **Graph index** — ingest `attribute` / `attribute def` nodes with `shortName`, `unitConversion`, `unitValueExpr`, and `UnitPrefix` metadata materialized during graph build (`unit_metadata.rs`, `graph_ingest.rs`).
2. **Derivation** — SI-prefixed symbols (`kV`, `MW`), compound units (`MWh`), and algebraic derived units are computed in `finalize_ingest` from graph-ingested base symbols.
3. **Quantity compatibility** — `incompatible_unit_dimension` compares the attribute quantity type against the unit literal dimension using `is_measurement_unit_compatible` (MeasurementUnit ancestry in the linked graph).

Library closure loads QUDV/SI packages into the semantic graph when imports or unit literals require them; the registry does not re-parse catalog text or read paths from disk.

Workspace models containing unit literals (`[…]` after numeric values) seed the quantity library closure (`Measurement`, `ISQ`, `SI`, `SIPrefixes`, …) even without explicit quantity imports.

## Regression gate

Run before merging changes to `semantic_core`:

```powershell
cargo test -p semantic_core resolution_contract
cargo test -p semantic_core --test p2_diagnostics_semantics
cargo test -p semantic_core --test import_namespace_semantics
cargo test -p semantic_core --test metadata_semantics
cargo test -p semantic_core --test requirement_derivation_semantics
```
