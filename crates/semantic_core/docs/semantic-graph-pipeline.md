# Semantic Graph Pipeline

`semantic_core` builds and links the workspace graph in explicit stages:

1. **Materialize** per document (`RootNamespace` + `uri`) into graph nodes and local attributes.
2. **Merge** workspace and library documents (library symbols skip workspace package collisions).
3. **Link** workspace relationships from the merged `SemanticGraph`.
4. **Prepare** analysis evaluation context (typing caches for expressions).
5. **Resolve pending** relationship queues (cross-file typing/specializes/connections).

## Orchestrator

Entry point: [`build_and_link_graph`](../../src/semantic/pipeline.rs) (also exposed as `build_semantic_graph_from_documents`).

```text
build_graph_from_doc (per document)
  → merge / merge_skip_existing_qualified_names
  → link_workspace_relationships
  → prepare_analysis_evaluation_context
  → resolve_workspace_pending_relationships (up to 8 passes)
```

## Stage 1: Materialize

- Input: parser AST (`RootNamespace`) and source URI.
- Output: semantic nodes (`part`, `part def`, `analysis`, `attribute`, etc.) with attributes such as `partType`, `subjectType`, `analysisType`, and value literals.
- Scope: syntax-driven only. No cross-document traversal through AST helpers.
- Per-document pending edges may be partially resolved via `resolve_pending_relationships_for_uri` during materialize.

## Stage 2: Link

- Input: merged `SemanticGraph` containing all document nodes.
- Output: `Typing` and `Specializes` edges resolved with workspace context and imports.
- Entry point: `link_workspace_relationships`.
- Resolution path: `resolve_type_target_in_workspace` delegates to `resolve_type_reference_targets` with role allowlists from `kinds.rs`.

`NodeId.uri` is provenance (origin file), not relationship scope.

## Stage 3: Pending resolve

- Input: merged graph with pending relationship queues from materialize/link.
- Output: resolved cross-document edges; remaining items stay in pending queues for diagnostics.
- Entry point: `resolve_workspace_pending_relationships`.
- Pending typing/specializes use full `resolve_type_reference_targets` (not import-only fallback).

## Stage 4: Evaluate / Project

- Evaluation and diagnostics resolve member paths through graph relationships (`resolve_member_via_type`).
- Projection consumes semantic attributes and relationship edges from the merged graph.
- Typed usage mirroring is intentionally not used; consumers should traverse typing targets.

## Legacy cross-document API

`resolve_cross_document_edges_for_uri` remains exported for the Spec42 kernel incremental update path. Prefer `build_and_link_graph` for full workspace builds.

## Resolution contract

See [resolution-contract.md](resolution-contract.md) for golden behavior and resolver entry points.
