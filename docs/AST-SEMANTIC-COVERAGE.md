# AST semantic coverage matrix

Maps **sysml-v2-parser** body/member enums to Spec42 surfaces. This is a **prioritization** tool, not a commitment to 100% AST-to-graph mapping. Parser version: **v0.15.0**.

| Parser surface | Graph (`semantic_core`) | Symbols / hover | Semantic tokens | Priority |
|----------------|-------------------------|-----------------|-----------------|----------|
| `PackageBodyElement` (core defs/usages) | Yes | Partial | Partial | P0 |
| `PartUsageBody::Brace` | Yes | Partial | Partial | P0 |
| `PortBody::Brace` / `PortBodyElement` | Yes (2026-06-03) | Partial | Yes (nested ports) | P0 |
| `PortDefBodyElement` (+ `InOutDecl`, `Error`) | Yes | Partial | Yes (`InOutDecl` span) | P0 |
| `PartDefBodyElement` (connect, ref, interface, …) | Mostly | Partial | Partial | P0 |
| Import / qualified package id | Yes | Yes | Namespace | P0 |
| `ref` + `Reference` edges (part def + usage) | Yes | Yes | Partial | P0 |
| `InterfaceDefBodyElement` | Partial | Partial | Partial | P1 |
| `ActionDefBody` / `ActionUsageBody` | Partial | Partial | Brace stub | P1 |
| `StateDefBody` | Partial | Partial | Partial | P1 |
| `RequirementDefBody` / constraint bodies | Partial | Partial | Partial | P1 |
| `AttributeBody` / `DefinitionBody` (item, flow, metadata defs) | Shell node only | Name only | Def span | Defer |
| `Error` / `Other` / `OpaqueMember` | Ignored | Ignored | Ignored | N/A |
| Doc / annotation members | Ignored | Ignored | Ignored | WONTFIX 1.0 |

## Policy

1. **Compile:** exhaustive `match` on body-element enums; no-op `Error` / doc / opaque members.
2. **Graph:** implement when a **shipped workflow** needs it (LSP navigation, `spec42 check`, IBD/general/action/state/sequence views).
3. **Tokens:** extend `sysml_semantic_tokens` `ast_ranges` for editor-visible identifiers in tested fixtures.

## Recent changes (v0.15.0 follow-through)

- Nested `PortBody` members are walked in [`port_def.rs`](../crates/semantic_core/src/semantic/graph_builder/port_def.rs) and covered by [`nested_port_body_semantics.rs`](../crates/semantic_core/tests/nested_port_body_semantics.rs).
- Token ranges recurse port bodies in [`ast_ranges.rs`](../crates/sysml_semantic_tokens/src/ast_ranges.rs).

## Backlog (P1+)

- Deeper `AttributeBodyElement` inside `item def` / `metadata def` when validation fixtures require inner attributes in the graph.
- `DefinitionBodyElement` for occurrence/rendering/flow families when general view projection needs compartment detail beyond the definition shell.
