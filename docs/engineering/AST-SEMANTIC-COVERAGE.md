# AST semantic coverage matrix

Maps **sysml-v2-parser** body/member enums to Spec42 surfaces. This is a **prioritization** tool, not a commitment to 100% AST-to-graph mapping. Parser version: **0.19.0** ([crates.io](https://crates.io/crates/sysml-v2-parser)).

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
| `StateDefBody` (`Transition.accept`, `is_initial`, `FinalState`) | Yes (0.19.0) | Partial | Partial | P0 |
| `RequirementDefBody` (`Stakeholder`, `Purpose`, `TextualRep`, `#keyword`) | Partial | Partial | Partial | P1 |
| `MetadataKeywordUsage` in part/state/requirement bodies | Yes (0.19.0) | Partial | Partial | P1 |
| `ActionUsage.send` / `PayloadClause` accept | Yes (0.19.0) | Partial | Partial | P1 |
| `RequirementDefBody` / constraint bodies | Partial | Partial | Partial | P1 |
| `AttributeBody` on `metadata def` / `metadata` usage | Partial (inner attributes) | Partial | Def span | P1 |
| `AttributeBody` / `DefinitionBody` (item, flow, other defs) | Shell node only | Name only | Def span | Defer |
| View `expose` feature chains (§7.6.6) | Yes (0.18.0 parser + view eval) | N/A | N/A | P0 |
| `Error` / `Other` / `OpaqueMember` | Ignored | Ignored | Ignored | N/A |
| Doc / annotation members | Ignored | Ignored | Ignored | WONTFIX 1.0 |

## Policy

1. **Compile:** exhaustive `match` on body-element enums; no-op `Error` / doc / opaque members.
2. **Graph:** implement when a **shipped workflow** needs it (LSP navigation, `spec42 check`, IBD/general/action/state/sequence views).
3. **Tokens:** extend `sysml_semantic_tokens` `ast_ranges` for editor-visible identifiers in tested fixtures.

## Recent changes (0.19.0 follow-through)

- `Transition.accept` / `TransitionAccept`, `is_initial`, and `FinalState` projected in [`state.rs`](../../crates/semantic_core/src/semantic/graph_builder/state.rs); `PayloadClause` on actions via [`payload.rs`](../../crates/semantic_core/src/semantic/graph_builder/payload.rs).
- `MetadataKeywordUsage` (`#keyword`) on part/state/requirement/verification bodies via [`metadata_keyword.rs`](../../crates/semantic_core/src/semantic/graph_builder/metadata_keyword.rs).
- Viewpoint `stakeholder` / `purpose` / `TextualRep` in [`requirement_body.rs`](../../crates/semantic_core/src/semantic/graph_builder/requirement_body.rs).
- Parser-wave fixtures under [`tests/fixtures/parser_wave/`](../../crates/semantic_core/tests/fixtures/parser_wave/) and integration tests in [`p2_diagnostics_semantics.rs`](../../crates/semantic_core/tests/p2_diagnostics_semantics.rs).

## Prior release (0.18.0 follow-through)

- `metadata def` and package-level `metadata` usage bodies walk `AttributeBodyElement` in [`metadata_def.rs`](../../crates/semantic_core/src/semantic/graph_builder/metadata_def.rs); covered by [`metadata_semantics.rs`](../../crates/semantic_core/tests/metadata_semantics.rs).
- Parser 0.18.0 accepts dot feature chains in `expose` targets; view evaluation resolves normalized chains in [`explicit_views.rs`](../../crates/semantic_core/src/semantic/explicit_views.rs); covered by [`expose_feature_chains.rs`](../../crates/semantic_core/tests/expose_feature_chains.rs).
- `unresolved_specializes_reference` (RULE7) includes case and metadata definition kinds via shared [`SPECIALIZES_TARGET_KINDS`](../../crates/semantic_core/src/semantic/relationships.rs); analysis def `:>` regression in kernel integration diagnostics.

## Prior release (0.17.0 follow-through)

- `AttributeUsage` / `PortUsage` usage-header operators (`:>`, `::>`, `=>`) are stored on graph nodes as `subsetsFeature`, `referencesFeature`, and `crossesFeature` in [`part_def.rs`](../../crates/semantic_core/src/semantic/graph_builder/part_def.rs), [`part_usage.rs`](../../crates/semantic_core/src/semantic/graph_builder/part_usage.rs), and [`port_def.rs`](../../crates/semantic_core/src/semantic/graph_builder/port_def.rs).

## Prior release (0.16.0 follow-through)

- `RequirementActorDecl` in requirement bodies is wired in [`requirement_body.rs`](../../crates/semantic_core/src/semantic/graph_builder/requirement_body.rs) (distinct from use-case `ActorDecl`).
- `EnumerationUsage` in part def/usage bodies is accepted in the graph builder (ignored like other deferred usage members until a workflow needs graph nodes).
- Nested `PortBody` members are walked in [`port_def.rs`](../../crates/semantic_core/src/semantic/graph_builder/port_def.rs) and covered by [`nested_port_body_semantics.rs`](../../crates/semantic_core/tests/nested_port_body_semantics.rs).
- Token ranges recurse port bodies in [`ast_ranges.rs`](../../crates/sysml_semantic_tokens/src/ast_ranges.rs).

## Backlog (P1+)

- Deeper `AttributeBodyElement` inside `item def` and other definition families beyond metadata.
- `DefinitionBodyElement` for occurrence/rendering/flow families when general view projection needs compartment detail beyond the definition shell.
