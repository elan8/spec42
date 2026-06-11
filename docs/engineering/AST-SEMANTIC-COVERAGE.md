# AST semantic coverage matrix

Maps **sysml-v2-parser** body/member enums to Spec42 surfaces. This is a **prioritization** tool, not a commitment to 100% AST-to-graph mapping. Parser version: **0.25.1** (structured `@` / `#` metadata bodies, `metadata def` body shorthand `:>` / `:>>`, `about` targets, `meta` cast expressions).

| Parser surface | Graph (`semantic_core`) | Symbols / hover | Semantic tokens | Priority |
|----------------|-------------------------|-----------------|-----------------|----------|
| `Expression` (filters, guards, constraints) | Yes — `exprClass`, `conditionIsBoolean` from AST walk (0.23.0) | N/A | N/A | P0 |
| `PartUsageBody::Brace` | Yes | Partial | Partial | P0 |
| `PortBody::Brace` / `PortBodyElement` | Yes (2026-06-03) | Partial | Yes (nested ports) | P0 |
| `PortDefBodyElement` (+ `InOutDecl`, `Error`) | Yes | Partial | Yes (`InOutDecl` span) | P0 |
| `PartDefBodyElement` (connect, ref, interface, …) | Yes | Partial | Mostly | P0 |
| Import / qualified package id | Yes | Yes | Namespace | P0 |
| `ref` + `Reference` edges (part def + usage) | Yes | Yes | Partial | P0 |
| `InterfaceDefBodyElement` | Yes | Partial | Partial | P1 |
| `ActionDefBody` / `ActionUsageBody` | Yes | Partial | Yes | P1 |
| `StateDefBody` (`Transition.accept`, `is_initial`, `FinalState`) | Yes (0.19.0) | Partial | Partial | P0 |
| `RequirementDefBody` (`Stakeholder`, `Purpose`, `TextualRep`, `#keyword`) | Yes | Partial | Yes | P1 |
| `MetadataKeywordUsage` in part/state/requirement/action/use-case bodies | Yes (0.25.0) | Partial | Partial | P1 |
| `MetadataAnnotation` / `#keyword` brace bodies (`AttributeBody`) | Yes (0.25.0) | Partial | Partial | P1 |
| `metadata` / `@` `about` targets + `RelationshipKind::Annotation` | Yes (0.25.0) | N/A | N/A | P1 |
| `SemanticMetadata` restrictions (`baseType`, `annotatedElement`) | Yes (0.25.1) | N/A | N/A | P1 |
| `ActionUsage.send` / `PayloadClause` accept | Yes (0.19.0) | Partial | Partial | P1 |
| `RequirementDefBody` / constraint bodies | Yes | Partial | Mostly | P1 |
| `AttributeBody` on `metadata def` / `metadata` usage | Yes | Partial | Def span | P1 |
| `AttributeBody` on `item def` / `individual def` / `metadata def` | Yes | Partial | Mostly | P1 |
| `AttributeBody` / `DefinitionBody` (flow, occurrence, other defs) | Yes | Partial | Mostly | P1 |
| View `expose` feature chains (§7.6.6) | Yes (0.18.0 parser + view eval) | N/A | N/A | P0 |
| `Error` / `Other` | Ignored | Ignored | Ignored | N/A |
| `OpaqueMember` | Minimal node | Partial | Ignored | P1 |
| Doc / annotation members | Ignored | Ignored | Ignored | WONTFIX 1.0 |

## Library content

The OMG standard library and Elan8 domain libraries ship **inside the Spec42 server binary** and materialize under the data directory on first use. Domain-library fixtures (for example `RequirementMetadata` in systems-engineering examples) resolve from the bundled tree without a git submodule checkout.

## Policy

1. **Compile:** exhaustive `match` on body-element enums; no-op `Error` / doc / opaque members.
2. **Graph:** implement when a **shipped workflow** needs it (LSP navigation, `spec42 check`, IBD/general/action/state/sequence views).
3. **Tokens:** extend `sysml_semantic_tokens` `ast_ranges` for editor-visible identifiers in tested fixtures.

## Recent changes (metadata §7.27, parser 0.25.1)

- **Metadata def body shorthand** — `:> annotatedElement : Type;` and `:>> baseType = expr meta Type;` parse via `metadata_body` / `metadata_binding` in sysml-v2-parser; projected as `attribute` children with `subsetsFeature`, `redefines`, and `attributeType` in [`attribute_body.rs`](../../crates/semantic_core/src/semantic/graph_builder/attribute_body.rs); covered by [`metadata_semantics.rs`](../../crates/semantic_core/tests/metadata_semantics.rs) (`requirement_metadata_def_shorthand_projects_restriction_attributes`).
- **Inline metadata bodies** — `@` / `#` annotations walk `AttributeBody` members via [`metadata_def.rs`](../../crates/semantic_core/src/semantic/graph_builder/metadata_def.rs) and [`metadata_keyword.rs`](../../crates/semantic_core/src/semantic/graph_builder/metadata_keyword.rs); covered by [`metadata_semantics.rs`](../../crates/semantic_core/tests/metadata_semantics.rs).
- **`about` targets** — package-level and inline metadata store `aboutTargets` and emit `RelationshipKind::Annotation` edges in [`relationships.rs`](../../crates/semantic_core/src/semantic/relationships.rs).
- **Diagnostics** — `metadata_about_unresolved`, `metadata_binding_missing`, `metadata_binding_unknown`, `metadata_annotated_element_incompatible` in [`view_metadata_conformance.rs`](../../crates/semantic_core/src/semantic/diagnostics/checks/view_metadata_conformance.rs).
- **Action / use-case `#keyword`** — wired in [`action.rs`](../../crates/semantic_core/src/semantic/graph_builder/action.rs) and [`use_case.rs`](../../crates/semantic_core/src/semantic/graph_builder/use_case.rs).

## Recent changes (graph depth P4a)

- **Parser 0.20.0** — flow/allocation `DefinitionBody` members parse as `OccurrenceMember`; Spec42 bump unlocks semantics and token tests.
- **DefinitionBody semantics** — flow def, flow usage, and allocation def inner `attribute`/`part` members materialize on graph; covered by [`definition_body_semantics.rs`](../../crates/semantic_core/tests/definition_body_semantics.rs).
- **Semantic tokens** — `FlowDef`/`FlowUsage`/`AllocationDef`/`OccurrenceDef` recurse `DefinitionBody` in [`ast_ranges.rs`](../../crates/sysml_semantic_tokens/src/ast_ranges.rs); covered by [`flow_def_tokens.rs`](../../crates/sysml_semantic_tokens/tests/flow_def_tokens.rs).
- **General View** — `require constraint` child nodes filtered from `generalViewGraph` while `requirementConstraints` inline on owner; unit test in [`model_projection.rs`](../../crates/semantic_core/src/semantic/model_projection.rs).

## Recent changes (graph depth P3)

- **DefinitionBody** — [`definition_body.rs`](../../crates/semantic_core/src/semantic/graph_builder/definition_body.rs) and [`occurrence_body.rs`](../../crates/semantic_core/src/semantic/graph_builder/occurrence_body.rs) walk occurrence-level members on `occurrence def`, `flow def`, and `flow usage` shells.
- **PartDefBody** — `EnumerationUsage`, `ItemUsage` `AttributeBody`, `OpaqueMember`, and `OccurrenceUsage` brace bodies projected in [`part_def.rs`](../../crates/semantic_core/src/semantic/graph_builder/part_def.rs); covered by [`part_def_body_semantics.rs`](../../crates/semantic_core/tests/part_def_body_semantics.rs) and [`definition_body_semantics.rs`](../../crates/semantic_core/tests/definition_body_semantics.rs).
- **Semantic tokens** — `ItemDef`/`IndividualDef`/`MetadataDef` inner attributes and expanded `PartDefBodyElement` coverage in [`ast_ranges.rs`](../../crates/sysml_semantic_tokens/src/ast_ranges.rs); requirement `RequirementActorDecl`, `TextualRep` `language_span`, and `#keyword` members; covered by [`part_def_tokens.rs`](../../crates/sysml_semantic_tokens/tests/part_def_tokens.rs).
- **Hover / symbols** — signatures and `SymbolKind` for `require constraint`, `enumeration`, `opaque member`, `individual def`, `stakeholder`, `purpose`, and `verified requirement` in [`hover.rs`](../../crates/kernel/src/semantic/presentation/hover.rs) and [`symbol_entries.rs`](../../crates/kernel/src/semantic/presentation/symbol_entries.rs).

## Recent changes (graph depth P2)

- **Action-flow view** — `enrich_activity_diagrams_from_graph` merges graph action children and `Flow`/`Perform` edges into activity diagrams; covered by [`activity_graph_semantics.rs`](../../crates/semantic_core/tests/activity_graph_semantics.rs).
- **Semantic tokens** — `ActionDefBody` and `RequirementDefBody` recurse in [`ast_ranges.rs`](../../crates/sysml_semantic_tokens/src/ast_ranges.rs); covered by [`action_definitions.rs`](../../crates/sysml_semantic_tokens/tests/action_definitions.rs).
- **Item / individual defs** — shared [`attribute_body.rs`](../../crates/semantic_core/src/semantic/graph_builder/attribute_body.rs) walks inner attributes; covered by [`item_def_body_semantics.rs`](../../crates/semantic_core/tests/item_def_body_semantics.rs).
- **Require constraint nodes** — `require constraint` projected as child nodes while retaining `analysisConstraints` for diagnostics.

## Recent changes (graph depth P1)

- **Action bodies** — `ActionDefBody` and `ActionUsageBody` walked in [`action.rs`](../../crates/semantic_core/src/semantic/graph_builder/action.rs): nested `action` usages, `then action` chains (`Perform` / `Flow`), `assign`, `ref`, `state usage`, `for` loops; covered by [`action_body_semantics.rs`](../../crates/semantic_core/tests/action_body_semantics.rs).
- **Interface bodies** — `interface end` nodes carry `portType` alongside `endType`; end-typing post-pass wires `Connection` edges on plain `interface def` builds; covered by [`interface_body_semantics.rs`](../../crates/semantic_core/tests/interface_body_semantics.rs).
- **Requirement bodies** — `verify` members and `subject` declarations emit `verified requirement` nodes and `Subject` edges in [`requirement_body.rs`](../../crates/semantic_core/src/semantic/graph_builder/requirement_body.rs); covered by [`requirement_body_semantics.rs`](../../crates/semantic_core/tests/requirement_body_semantics.rs).

## Recent changes (0.19.0 follow-through)

- `Transition.accept` / `TransitionAccept`, `is_initial`, and `FinalState` projected in [`state.rs`](../../crates/semantic_core/src/semantic/graph_builder/state.rs); `PayloadClause` on actions via [`payload.rs`](../../crates/semantic_core/src/semantic/graph_builder/payload.rs).
- `MetadataKeywordUsage` (`#keyword`) on part/state/requirement/verification bodies via [`metadata_keyword.rs`](../../crates/semantic_core/src/semantic/graph_builder/metadata_keyword.rs).
- Viewpoint `stakeholder` / `purpose` / `TextualRep` in [`requirement_body.rs`](../../crates/semantic_core/src/semantic/graph_builder/requirement_body.rs); `languageSpan` on `textualRep` nodes for precise diagnostics.
- Verification-local `AttributeDef` in [`verification.rs`](../../crates/semantic_core/src/semantic/graph_builder/verification.rs) (mirrors analysis case).
- Parser-wave fixtures under [`tests/fixtures/parser_wave/`](../../crates/semantic_core/tests/fixtures/parser_wave/) and integration tests in [`p2_diagnostics_semantics.rs`](../../crates/semantic_core/tests/p2_diagnostics_semantics.rs).

## Prior release (0.18.0 follow-through)

- `metadata def` and package-level `metadata` usage bodies walk `AttributeBodyElement` in [`metadata_def.rs`](../../crates/semantic_core/src/semantic/graph_builder/metadata_def.rs); covered by [`metadata_semantics.rs`](../../crates/semantic_core/tests/metadata_semantics.rs).
- Parser 0.18.0 accepts dot feature chains in `expose` targets; view evaluation resolves normalized chains in [`explicit_views.rs`](../../crates/semantic_core/src/semantic/explicit_views.rs); covered by [`expose_feature_chains.rs`](../../crates/semantic_core/tests/expose_feature_chains.rs).
- `unresolved_specializes_reference` (RULE7) includes case and metadata definition kinds via shared [`SPECIALIZES_TARGET_KINDS`](../../crates/semantic_core/src/semantic/relationships.rs); analysis def `:>` regression in kernel integration diagnostics.

## Prior release (0.17.0 follow-through)

- `AttributeUsage` / `PortUsage` usage-header operators (`:>`, `::>`, `=>`) are stored on graph nodes as `subsetsFeature`, `referencesFeature`, and `crossesFeature` in [`part_def.rs`](../../crates/semantic_core/src/semantic/graph_builder/part_def.rs), [`part_usage.rs`](../../crates/semantic_core/src/semantic/graph_builder/part_usage.rs), and [`port_def.rs`](../../crates/semantic_core/src/semantic/graph_builder/port_def.rs).

## Prior release (0.16.0 follow-through)

- `RequirementActorDecl` in requirement bodies is wired in [`requirement_body.rs`](../../crates/semantic_core/src/semantic/graph_builder/requirement_body.rs) (distinct from use-case `ActorDecl`).
- `EnumerationUsage` in part def bodies projects enumeration shell nodes and inner `AttributeBody` members (P3).
- Nested `PortBody` members are walked in [`port_def.rs`](../../crates/semantic_core/src/semantic/graph_builder/port_def.rs) and covered by [`nested_port_body_semantics.rs`](../../crates/semantic_core/tests/nested_port_body_semantics.rs).
- Token ranges recurse port bodies in [`ast_ranges.rs`](../../crates/sysml_semantic_tokens/src/ast_ranges.rs).

## Recent changes (graph depth P4b–P8)

- **RenderingDef** body walks filter/rendering members; **OccurrenceUsage** brace bodies recurse occurrence members.
- **Case bodies** wire `SubjectRef`, `FirstSuccession`, `ThenUseCaseUsage`, `RefRedefinition`, and actor redefinition via shared `use_case` helpers.
- **MetadataAnnotation** in action and requirement bodies projects `metadata usage` nodes (`metadata_def::add_metadata_annotation_node`).
- Parser **0.20.1**: `AttributeDef.value_span`, literal-with-unit spans; Spec42 stores `valueIsBoolean` / `rhsIsBoolean` on verification/analysis nodes.

## Backlog (P1+)

- `DefinitionBodyElement` for occurrence/rendering families when general view projection needs compartment detail beyond the definition shell.
- Remaining parser `advance_to_closing_brace` sites in action ref bodies, in-out defaults, and satisfy connect bodies.
