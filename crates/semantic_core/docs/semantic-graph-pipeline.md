# Semantic Graph Pipeline

`semantic_core` builds and links the workspace graph in two explicit stages:

1. Materialize per document (`RootNamespace` + `uri`) into graph nodes and local attributes.
2. Link workspace relationships from the merged `SemanticGraph` only.

## Stage 1: Materialize

- Input: parser AST (`RootNamespace`) and source URI.
- Output: semantic nodes (`part`, `part def`, `analysis`, `attribute`, etc.) with attributes such as `partType`, `subjectType`, `analysisType`, and value literals.
- Scope: syntax-driven only. No cross-document traversal through AST helpers.

## Stage 2: Link

- Input: merged `SemanticGraph` containing all document nodes.
- Output: `Typing` and `Specializes` edges resolved with workspace context and imports.
- Entry point: `link_workspace_relationships`.
- Resolution path: `resolve_type_target_in_workspace` delegates to `resolve_type_reference_targets`.

`NodeId.uri` is provenance (origin file), not relationship scope.

## Stage 3: Evaluate / Project

- Evaluation and diagnostics resolve member paths through graph relationships (`resolve_member_via_type`).
- Projection consumes semantic attributes and relationship edges from the merged graph.
- Typed usage mirroring is intentionally not used; consumers should traverse typing targets.
