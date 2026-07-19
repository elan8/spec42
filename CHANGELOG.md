# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.44.14] - 2026-07-19

- **S42-004 slice: every explicit comma-separated typing/specialization target is now
  materialized, not just the first.** SysML v2 allows a feature or definition to be typed/
  specialized by multiple comma-separated targets (`attribute reading : Weight, Height;` is
  equivalent to `attribute reading defined by Weight defined by Height;`; `part def Combined
  specializes BaseA, BaseB;` is two independent `Subclassification` relationships). The parser
  already retained this losslessly in `TypingRelationship.target: Vec<Node<RelationshipTarget>>`
  (used by `AttributeDef`/`AttributeUsage.typing` and by `specializes` on every `xxxDef` struct),
  but every spec42 consumer discarded everything after the first target -- `ast_util::typing_target`/
  `typing_target_display` called `.first()`, and `TypeReferenceTarget for TypingRelationship`/
  `for Node<TypingRelationship>` did the same when a builder passed the whole relationship struct
  directly into `add_typing_edge_if_exists`/`add_specializes_edge_if_exists`. New
  `ast_util::typing_targets` returns every target; `graph_builder::mod::insert_def_specialization_attr`/
  `wire_def_specialization_edge` (the shared choke point for ~13 `xxxDef` materializers routed
  through `package_body/materialize.rs`, `calc_constraint_def.rs`, `view_def.rs`) and every direct
  `AttributeDef`/`AttributeUsage` typing consumer (`attribute_body.rs`, `usage_builders.rs`,
  `part_def.rs`, `package_body/materialize.rs`, `occurrence_body.rs`, `port_def.rs`,
  `requirement_body.rs`, `verification.rs`, `analysis_case.rs`) now loop over every target instead
  of resolving one. Deliberately out of scope (unchanged from S42-004's existing deferral, now
  documented explicitly): multi-target typing for usage kinds other than `AttributeDef`/
  `AttributeUsage` (their AST only ever retained a flattened `type_name: Option<String>`, so
  recovering multiple targets needs a parser AST change -- the actual "large, multi-release" part
  of S42-004); implied/standard-library default typing; the `is_implied` flag (exists on both the
  parser AST and the projection layer already, but nothing ever sets it `true`, so wiring it
  through today would be a no-op); end-owning-type/cross-feature/feature-chain collections. No
  `PROJECTION_SCHEMA_VERSION` bump: more edges of the already-existing `RelationshipKind::Typing`/
  `Specializes` kinds, no new fields. Regression coverage:
  `crates/workspace/tests/snapshot_single_build.rs`'s new
  `snapshot_materializes_every_target_of_a_multi_typed_attribute_and_specializes_clause`.

## [0.44.13] - 2026-07-19

- **`of X` payload feature on named flows now materializes as a real child node (SysML v2
  8.2.2.16 `PayloadFeature`).** An audit of this session's work against the OMG spec text found
  that `of qty : Payload[1..3]` (a *named* payload feature) was a hard parser failure -- the
  whole flow statement dropped into error recovery -- because `FlowUsage::payload` was parsed as
  a bare `Expression` (`optional_payload`, `sysml-v2-parser/src/parser/flow.rs`), which only
  happened to work for the bare-type-reference form (`of Payload`). Per spec, `of X` is a real
  owned `FeatureMembership` (`PayloadFeature : Feature = Identification? PayloadFeatureSpecializationPart
  ValuePart?`), not a value reference. Fixed upstream in `sysml-v2-parser` 0.43.0: `FlowUsage::payload`
  is now `Option<Node<PayloadFeature>>` (`name`/`type_name`/`multiplicity`), parsed by a new
  `payload_feature` combinator accepting typing and multiplicity in either order (mirroring
  `feature_usage_header`'s existing either-order handling, but retaining the multiplicity instead
  of discarding it). New `ElementKind::FlowPayload` (kind-string `"flow payload"`).
  `materialize_flow_usage` (`graph_builder/flow_usage.rs`) now materializes the payload as a real
  child node of the flow for every *named* flow with a payload -- unconditionally on
  `flow.payload.is_some()`, not gated on whether a name/multiplicity was actually written, since
  `Identification?` only makes the *name* optional, not the feature itself; a bare `of Payload`
  gets a synthetic `"_payload"` name, matching the existing `"_assign"`/`"_terminate"` convention
  for unnamed control-flow children. Unnamed flows (`flow of Payload from a to b;`) have no node
  to own the feature and keep only the existing edge-level `payload_type_id` scalar (deliberate
  scope limit, not a regression -- avoids a second synthetic-containment precedent in the same
  session). The new child node is additive: `add_flow_edge_if_both_exist`'s existing
  `payload_type_id` resolution is unchanged (simplified to read the now-plain `type_name` field
  instead of extracting it from an expression), so `FlowStatementDetail.payload_type_id` still
  resolves for both named and unnamed flows. No `PROJECTION_SCHEMA_VERSION` bump: new enum
  variant + additive node only, same class as the `Else`/`ConjugatedPortDefinition` additions.
  Also fixed in the same audit: `TextualRepresentation` was missing its
  `representedElement`/`documentedElement`-style back-reference (KerML `AnnotatingElement`) --
  see `babel42-v2`'s changelog for that fix, which lives entirely on that side (DTO +
  `openapi.json` only, no spec42 change needed). Regression coverage:
  `crates/workspace/tests/snapshot_single_build.rs`'s extended
  `snapshot_projects_flow_detail_with_payload_and_succession_kind` (bare `of Payload` still gets
  a synthetic-name child node) and new
  `snapshot_materializes_named_flow_payload_feature_with_multiplicity` (name/type/multiplicity
  all correct, `payload_type_id` regression guard, real `Typing` edge to the resolved type).

## [0.44.12] - 2026-07-19

- **S42-009: conjugated port definitions synthesized (KerML 8.3.12.2-8.3.12.4, SysML v2
  8.2.2.12/8.4.8.1-8.4.8.2).** `port p : ~P;` previously discarded the `~` during type
  resolution -- `p`'s typing edge pointed straight at `P`. Per spec, `~P` denotes an implicit
  `ConjugatedPortDefinition`, nested as a real `ownedMember` of `P` (not a sibling, not
  ownerless -- confirmed directly against the OMG spec text, correcting an earlier draft of
  this work), linked back via a `PortConjugation` relationship (a `Conjugation`, not a
  `Typing`/`Specializes` edge). `materialize_port_def`
  (`graph_builder/package_body/materialize.rs`) now eagerly materializes this conjugate
  alongside every non-conjugated port def -- matching the spec's own eager-parsing semantics,
  not lazily on first `~`-typed usage -- so usage-time resolution
  (`add_typing_edge_for_node`, `relationships.rs`) is a pure lookup (resolve `P`, then find its
  already-materialized conjugate child), never node creation, eliminating an entire class of
  find-or-create raciness. New `ElementKind::ConjugatedPortDefinition` and
  `RelationshipKind::PortConjugation`; both added to the relevant typing-target allowlists
  (`kinds.rs`) to avoid a spurious `incompatible_type_kind` diagnostic regression.
  `effective_port_features`/`port_definition_qualified_name` (`diagnostics/helpers.rs`) now
  follow the conjugate's `PortConjugation` edge to find the real `PortDef` for feature/name
  lookups, fixing a silent regression the conjugate redirect would otherwise have introduced in
  `connection_conformance.rs`'s `port_compatibility_mismatch` check (previously untested for
  conjugated ports). Two independent typing-resolution engines needed the same conjugation-aware
  fix -- `add_typing_edge_for_node` (`relationships.rs`, the immediate per-document and deferred
  `link_workspace_relationships` paths) and `resolve_typing_edge_cross_document_inner`
  (`relationships/cross_document.rs`, the parallel full-build and scoped-incremental paths) --
  discovered by a test failure showing both a correct and an incorrect typing edge coexisting
  from the same usage; fixing only one silently left the other producing the pre-fix (direct-to-
  `P`) edge. No `PROJECTION_SCHEMA_VERSION` bump: new enum variants only, same class as the
  `Else`/`Terminate`/`While`/`If` additions. Deliberately out of scope this round: the
  conjugate's own features (the in/out-reversed mirror of `P`'s owned features) are not
  independently materialized as child nodes -- comparable in size to the already-deferred S42-004
  implied-typing work. Regression coverage:
  `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_materializes_conjugated_port_definition_eagerly` and
  `snapshot_conjugated_port_structural_mismatch_uses_feature_check_not_fallback`.

## [0.44.11] - 2026-07-18

- **Flow payload resolves to a real type; `TextualRepresentation` is addressable.**
  `FlowStatementDetail` (the `Flow`/`SuccessionFlow` edge detail attached to a resolved
  `succession flow dataFlow of Payload from a to b;`) gains `payload_type_id: Option<String>`:
  `of Payload` previously stayed raw debug text (`payload_expression`) with no resolved
  reference, unlike `source`/`target`, whose edge endpoints were already resolved to real
  feature IDs. `add_flow_edge_if_both_exist` (`graph_builder/flow_usage.rs`) now resolves the
  payload the same way `add_typing_edge_if_exists` resolves any type reference (it names a
  type, not a feature path, so it goes through `resolve_type_target_in_workspace` +
  `TYPING_TARGET_KINDS`, not the feature-path resolver used for `from`/`to`). The
  graph-builder layer stores the resolved qualified name; `workspace`'s `project_host_semantic_model`
  translates it into the same semantic-ID space as `source_id`/`target_id` before exposing it,
  mirroring the existing two-step handoff. New `ElementKind::TextualRepresentation`
  (kind-string `"textualRep"`, unchanged) gives `rep <name>? language "..." { ... }` nodes —
  already fully materialized by `materialize_textual_rep`/`requirement_body.rs` with
  `language`/`text` attributes, just previously falling into `Unknown("textualRep")` — a
  distinct, addressable classification instead. `PROJECTION_SCHEMA_VERSION` bumped `12` → `13`:
  new `FlowStatementDetail` field, same class as the 0.44.9 `isPortion`/`portionKind` addition.
  Regression coverage: `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_projects_flow_detail_with_payload_and_succession_kind` (extended) and new
  `snapshot_materializes_textual_representation_as_addressable_node`.
- **S42-009 (conjugated ports) status clarified.** `isConjugated` was already fully wired and
  tested (a `port p : ~P` usage already reports `isConjugated: true` via `HostFeatureProperties`).
  The remaining gap — synthesizing implicit `ConjugatedPortDefinition`/`ConjugatedPortTyping`
  elements per the KerML metamodel, so `p`'s typing points at the conjugated definition rather
  than directly at `P` — is deliberately deferred: it would be the first "implicit node not
  present in source text" pattern in this codebase, a genuine architectural decision on the
  same footing as S42-004/S42-005, not a small fix. See
  spec42-systems-modeling-api-gaps.md S42-009 for the updated status.

## [0.44.10] - 2026-07-18

- **`if`'s `else` branch materialized; `add_for_loop` nesting bug fixed.** `add_if_stmt`
  (`crates/sysml_model/src/semantic/graph_builder/action.rs`) previously flagged `hasElse` but
  never walked the else body (deferred in 0.44.5). It now materializes a nested `"else"`-kind
  child node under the `if` node when an else branch is present, whose own children are the else
  body's elements -- mirroring the existing `for`/`while` body-wrapper pattern rather than
  flattening then/else children into one list. New `ElementKind::Else` (kind-string `"else"`). No
  `PROJECTION_SCHEMA_VERSION` bump: new `ElementKind` value plus the existing generic node/attrs
  shape, same class as the 0.44.5 `Terminate`/`While`/`If` additions. Separately, `add_for_loop`
  was passing the outer `container_prefix` instead of the loop's own qualified name as the
  `container_prefix` for its body recursion -- the only control-flow body materializer with this
  bug (`while`/`if` already passed their own qualified name correctly) -- so nested actions inside
  a `for` loop did not have qualified names reflecting the loop's nesting. Fixed to pass
  `Some(qualified.as_str())`. Regression coverage: `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_materializes_terminate_while_and_if_control_nodes`, extended with a `for` loop fixture
  asserting nested qualified-name scoping and an `else` branch fixture asserting the new `"else"`
  node, its `parent`, and its child.

## [0.44.9] - 2026-07-18

- **`isPortion`/`portionKind` added to declared feature properties (S42-008).** `DeclaredFeatureProperties`
  and `HostFeatureProperties` gain `is_portion: bool` and `portion_kind: Option<String>`.
  `occurrence_usage_feature_properties` (`ast_util.rs`) now populates both from
  `OccurrenceUsage::portion_kind` (already set by the `snapshot`/`timeslice`/`then timeslice`
  parser forms, unchanged this round) — previously only the raw `"portionKind"` debug attribute
  existed, with no slot in the typed feature-properties system Babel42's `isPortion`/`portionKind`
  DTO fields actually read. All other `*_feature_properties` builders (`part`/`attribute`/`port`/
  `item`/`definition`) explicitly set `is_portion: false, portion_kind: None`, since only
  occurrences have portion semantics. `PROJECTION_SCHEMA_VERSION` bumped `11` → `12`: new
  `HostFeatureProperties` fields, same class as the 0.44.0 `content_expression_id` addition.
  Regression coverage: `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_populates_occurrence_def_and_usage_facts`, extended with `is_portion`/`portion_kind`
  assertions for `snapshot`/`timeslice`/plain `occurrence` usages.
- **`isReadOnly`/`isSufficient`/`isVariable`/`mayTimeVary` confirmed out of scope.** Exhaustive
  search of `sysml-v2-parser`'s grammar found no `readonly`, `variable`, `sufficient`, or
  time-varying keyword production anywhere — `readonly`/`variable` were already documented as
  deliberately out of scope (not SysML textual keywords, "Modifier completeness audit"); this
  extends the same finding to `sufficient` and time-varying, none of which have a textual BNF
  production either. Babel42's hardcoded `null` for these four properties is therefore not a gap
  to close via keyword wiring — see spec42-systems-modeling-api-gaps.md S42-008 for the
  reclassification.

## [0.44.8] - 2026-07-18

- **`expose` materializes as a real Import.** `annotate_view_usage_body`
  (`crates/sysml_model/src/semantic/graph_builder/view_def.rs`) previously only recorded
  `hasExpose`/`exposeTargets` text attributes on the owning view node -- no addressable element
  or relationship existed for an `expose` member at all. `expose` is normatively an Import per
  `ExposeMember`'s own BNF doc comment; new `materialize_expose_member` (using
  `sysml-v2-parser` 0.42.0's `ExposeMember::is_import_all`/`::is_recursive`) creates a real
  `"import"`-kind child node with the exact same attribute shape `materialize_import` already
  uses for ordinary `import` statements (`importTarget`/`importAll`/`recursive`). This reuses
  the existing `membership_kind`/`membership_relationship_metaclass` pipeline entirely --
  `HostMembershipKind::Import` already classifies further into
  `HostRelationshipMetaclass::NamespaceImport`/`::MembershipImport` based on the `importAll`
  attribute -- so `expose` publishes as a concrete `NamespaceImport`/`MembershipImport`
  relationship with zero new classification logic. The old `hasExpose`/`exposeTargets` summary
  attributes are left in place alongside the new node.
- **`filter`'s condition is now a real addressable `Expression`.** Both `add_view_filter_node`
  and `build_filter_member` (`view_def.rs`) previously only stored a debug-text `"condition"`
  attribute. Both now also set `declared_facts.own_expression` (mirroring
  `TransitionGuard`'s 0.44.0 `content_expression_id` fix, S42-003), so `filter`'s condition is
  the real parsed `Expression`, not rendered text.
- No `PROJECTION_SCHEMA_VERSION` bump: both changes reuse existing `HostSemanticProjection`
  shape (generic node/attrs, and the pre-existing `content_expression_id` mechanism). Regression
  coverage: `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_classifies_expose_as_namespace_or_membership_import` and
  `snapshot_projects_filter_condition_as_an_addressable_expression`.
- **Deliberately still open:** `filter`'s own node kind (`ElementKind::Filter`) and a `view
  rendering` member (`ElementKind::ViewRendering`) remain without their own confirmed Systems
  Modeling API concrete resource shape at the Babel42 layer (tracked as a Babel42-side follow-up,
  not a Spec42 gap); `expose`'s optional bracket-filter form (`expose X [ expr ];`) still skips
  its filter expression's content entirely, same as before this round.

## [0.44.7] - 2026-07-18

- **`OccurrenceDef.isAbstract` fix.** 0.44.6 set a raw `"isAbstract"` attribute on `occurrence
  def` nodes, but Babel42's `isAbstract` DTO field only reads
  `node.facts.feature_properties.is_abstract` or a `"definitionPrefix"` string attribute (the
  `PartDef`-only convention) — a raw attribute alone was silently dropped.
  `materialize_occurrence_def` now builds `DeclaredFeatureProperties` directly (`OccurrenceDef`
  has a plain `is_abstract: bool`, not the `DefinitionPrefix` enum
  `definition_feature_properties` expects) and attaches it the same way `PartDef` does. Caught
  by the downstream Babel42 contract test before this shipped anywhere.

## [0.44.6] - 2026-07-18

- **Occurrence definition/usage facts enriched.** `materialize_occurrence_def`
  (`crates/sysml_model/src/semantic/graph_builder/package_body/materialize.rs`)
  never set `isAbstract`, unlike its `requirement def`/`case def` siblings.
  `materialize_occurrence_usage`
  (`crates/sysml_model/src/semantic/graph_builder/usage_builders.rs`) never
  called `attach_feature_properties` at all (so `is_individual` was silently
  discarded) and never surfaced `portionKind`/`isThen`/the `subsets`/
  `redefines`/`references`/`crosses` subsetting-clause targets already
  parsed onto `OccurrenceUsage` — all four AST fields existed, unlike
  `AttributeUsage`'s identical shape, which already projects them. Both
  gaps closed: `occurrence def` now sets `isAbstract`; `occurrence`/
  `individual`/`snapshot`/`timeslice`/`then timeslice` usages now attach
  `occurrence_usage_feature_properties` (new `ast_util.rs` helper, mirroring
  `item_usage_feature_properties`, populating `is_individual` from
  `OccurrenceUsage::is_individual`) and the same four subsetting-clause
  attributes `AttributeUsage` already sets. No `PROJECTION_SCHEMA_VERSION`
  bump: existing `HostFeatureProperties`/attribute shape, no new fields.
  Regression coverage: `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_populates_occurrence_def_and_usage_facts`.

## [0.44.5] - 2026-07-18

- **`terminate`/`while`/`if` control nodes materialized.** `ActionDefBodyElement::TerminateStmt`/
  `::WhileStmt`/`::IfStmt` and their `ActionUsageBodyElement` counterparts previously matched a
  silent no-op arm in both `build_from_action_def_body` and `build_from_action_usage_body`
  (`crates/sysml_model/src/semantic/graph_builder/action.rs`) and were dropped from the graph
  entirely, unlike their sibling control statements (`perform`/`merge`/`decide`/`join`/`fork`/
  `for`/`assign`, all already materialized). New `ElementKind::Terminate`/`::While`/`::If`
  (kind-strings `"terminate"`/`"while"`/`"if"`) plus `add_terminate_stmt`/`add_while_stmt`/
  `add_if_stmt` close the gap: `terminate` records an optional `terminateTarget` debug-text
  attribute (mirroring `merge`/`decide`/`join`/`fork`'s existing text-only target pattern);
  `while` records a `whileCondition` debug-text attribute and walks its body as children via the
  existing `build_from_action_def_body`; `if` records an `ifCondition` attribute plus a
  `hasElse: bool` flag and walks only its `then` branch as children this round. The `else` branch
  body, when present, is flagged by `hasElse` but not yet walked -- materializing it needs a
  design decision on how to scope a second child branch, deferred as a follow-up. No
  `PROJECTION_SCHEMA_VERSION` bump: new `ElementKind` values plus the existing generic node/attrs
  shape, same class as the pre-existing `merge`/`decide`/`join`/`fork` additions. Regression
  coverage: `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_materializes_terminate_while_and_if_control_nodes`.

## [0.44.4] - 2026-07-18

- **`ConcernDefinition`/`ConcernUsage` classification.** `sysml-v2-parser` 0.41.0 added
  `ConcernUsage::is_definition` (previously the parser matched the optional `concern def` keyword
  but discarded whether it was present, since `concern_usage` handles both textual forms via one
  AST struct -- no separate `ConcernDef` node exists, unlike `case`/`case def`).
  `materialize_concern_usage` (`crates/sysml_model/src/semantic/graph_builder/package_body/materialize.rs`)
  now branches on that flag to materialize kind-string `"concern def"` (`ElementKind::ConcernDef`)
  or `"concern"` (`ElementKind::Concern`) instead of always tagging `"concern"`. `ElementKind::ConcernDef`
  was already in `is_definition()` and the typing/specializes allowlists (`kinds.rs`), so
  `membership_kind()`'s generic `kind.is_definition() => OwningMembership` rule and the existing
  `ElementKind::Concern => FeatureMembership` arm now classify both forms correctly with no
  further change. `concern` usages are only legal at package level (no `PartDefBodyElement`
  variant exists), so there is no nested-in-`part def` dispatch gap to fix here, unlike the
  `case`/`use case`/`analysis`/`verification` family. No `PROJECTION_SCHEMA_VERSION` bump: pure
  classification, same class as the 0.44.1 `CaseUsage` fix. Regression coverage:
  `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_classifies_concern_def_separately_from_concern_usage`.

## [0.44.3] - 2026-07-17

- **Nested `use case`/`analysis`/`verification` def and usage support in `part def` bodies.**
  `PartDefBodyElement::UseCaseDef`/`::UseCaseUsage`, `::AnalysisCaseDef`/`::AnalysisCaseUsage`,
  and `::VerificationCaseDef`/`::VerificationCaseUsage` were already parsed correctly
  (`sysml-v2-parser`'s `part_def_body_element`), and the package-level
  `PackageBodyElement` counterparts already dispatched to the same materializers with full
  body-walking and typing-edge resolution -- but `graph_builder/part_def.rs`'s `PDBE` match had
  no arm at all for any of these six variants, unlike the sibling `PDBE::CalcUsage`/`PDBE::CaseDef`
  arms. All six fell to the `_ => {}` catch-all and were silently dropped from the graph when
  nested inside a `part def { ... }` body -- the exact same bug class already fixed once for
  `case`/`case def` in 0.44.1. Fixed by adding the six missing dispatch arms, reusing the existing
  `materialize_use_case_def`/`_usage`, `materialize_analysis_case_def`/`_usage`, and
  `materialize_verification_case_def`/`_usage` builders the package-level dispatch already calls
  (widened from `pub(super)` to `pub(crate)`, same as the `case` builders were in 0.44.1). No
  `PROJECTION_SCHEMA_VERSION` bump: this is dispatch-coverage only, not a `HostSemanticProjection`
  shape change. Regression coverage: `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_materializes_use_case_analysis_and_verification_nested_in_part_def`.

## [0.44.2] - 2026-07-17

- **`ConstraintUsage` classification, package-level.** `sysml-v2-parser` 0.40.0 added a distinct
  `ConstraintUsage` AST node (previously bare `constraint c : X;` folded into `ConstraintDef` at
  parse time, since no usage-side node existed to classify it separately). `ElementKind` already
  had a `"constraint"` string arm (`ElementKind::Constraint`) from an earlier round, but nothing
  ever constructed it -- `graph_builder/calc_constraint_def.rs` gains `build_constraint_usage`
  (materializing kind-string `"constraint"`, mirroring `build_constraint_def`'s metadata
  extraction over the shared `ConstraintDefBody` type, plus a typing edge when `type_name` is
  present), dispatched from `PackageBodyElement::ConstraintUsage` in
  `graph_builder/package_body/mod.rs`. No `PROJECTION_SCHEMA_VERSION` bump: this adds
  graph-builder dispatch coverage and consumes a new (but backward-compatible) parser AST node,
  not a `HostSemanticProjection` shape change. Regression coverage:
  `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_materializes_bare_constraint_usage_and_resolves_its_typing`, covering both the simple
  typed form and the real `Systems Library/Constraints.sysml` `constraintChecks` shape (`abstract`
  + typing + trailing multiplicity + `nonunique` + subsetting, all `def`-less).
  Nested-in-another-body `constraint` members (e.g. `Requirements.sysml`'s
  `RequirementConstraintCheck::assumptions`) remain out of scope -- the parser change only covers
  package-level dispatch; see `sysml-v2-parser` CHANGELOG 0.40.0 for the parser-side detail and
  `babel42-v2`'s `spec42-systems-modeling-api-gaps.md` for the API-level tracking.

## [0.44.1] - 2026-07-17

- **`CaseUsage` classification and nested `case`/`case def` support (S42-003 follow-up).**
  `ElementKind` had a `"case def"` string arm but no `"case"` arm, so a bare `case c : X;`
  usage — even at package level, where it already materialized — fell to
  `ElementKind::Unknown("case")` and could never become a concrete API resource. Added
  `ElementKind::Case` plus its `Display`/`FromStr` arms
  (`crates/sysml_model/src/semantic/model.rs`). Separately, `case`/`case def` nested inside a
  `part def { ... }` body had no dispatch arm at all in
  `crates/sysml_model/src/semantic/graph_builder/part_def.rs` (unlike the sibling
  `PartDefBodyElement::CalcUsage` arm), so it was silently dropped from the graph entirely at
  both the definition and usage level — not merely misclassified. Fixed by adding
  `PartDefBodyElement::CaseDef`/`::CaseUsage` dispatch arms that reuse the existing
  `materialize_case_def`/`materialize_case_usage` builders package-level dispatch already
  calls, widening those two functions from `pub(super)` to `pub(crate)` so the sibling
  `graph_builder::part_def` module can reach them through `package_body`'s existing
  `pub(crate) use materialize::*` re-export instead of duplicating the body-walking logic.
  No `PROJECTION_SCHEMA_VERSION` bump: this changes graph-builder dispatch coverage and
  `ElementKind` classification, not `HostSemanticProjection`'s shape. Regression coverage:
  `crates/workspace/tests/snapshot_single_build.rs`'s
  `snapshot_materializes_case_def_and_case_usage_nested_in_part_def`.

## [0.44.0] - 2026-07-17

- **`TransitionFeatureMembership` baseline (S42-003)** — a `transition` statement's `accept`
  trigger, `if` guard, and `do` effect now materialize as real, addressable child graph nodes
  (`ElementKind::TransitionTrigger`/`TransitionGuard`/`TransitionEffect`) owned via a new
  `HostMembershipKind::TransitionFeatureMembership`, mirroring how every other owned node gets a
  `Membership` automatically via `membership_kind()` (`crates/workspace/src/snapshot/facts.rs`).
  Trigger and effect use a simplified/uniform attribute representation this round (reusing the
  same rendering already used on the parent `transition` node), not full
  `AcceptActionUsage`/`ActionUsage` typing (deferred). Guard is different: its content is the
  real `Node<Expression>` from the parser AST, so it's projected losslessly via the existing
  `ast_util::declared_expression()` converter (unmodified) into a genuine addressable
  `Expression`, referenced from the new `HostElementFacts::content_expression_id` field
  (`crates/sysml_model/src/semantic/graph_builder/state.rs`,
  `crates/workspace/src/snapshot/facts.rs`).
  **Design note — a `FeatureValue`-reuse approach was considered and rejected:** an earlier draft
  planned to attach guard/trigger/effect content by reusing `HostFeatureValue`
  (`DeclaredFeatureValueKind`) to avoid a schema bump, but `HostFeatureValue` normatively means
  "this feature's value is X" (a KerML value-assignment) — none of guard/trigger/effect are
  value-assignments, so that would have shipped an incorrect `@type: FeatureValue` label on the
  consuming API. `content_expression_id` instead represents "this element's substance is X",
  a distinct, correctly-scoped concept.
- **Fix: the `transition` node's own membership was `OwningMembership`, should be
  `FeatureMembership`** — `ElementKind::Transition` was missing from `membership_kind()`'s
  `FeatureMembership` arm (documented as a known gap in 0.43.2). Fixed alongside the above since
  both required touching the same function.
- All existing flattened string attributes on the `transition` node (`source`, `target`,
  `guardExpression`, `conditionIsBoolean`, `exprClass`, `effectExpression`, `actionKind`,
  `payloadType`, `acceptType`, `acceptName`, …) are unchanged — purely additive; zero changes
  required in `behavior_conformance.rs` or `state_views/graph_extractor.rs`.
- **`PROJECTION_SCHEMA_VERSION` bump: 10 → 11.** Unlike 0.43.1/0.43.2's pure enum widening (no
  bump needed, since `Option<HostMembershipKind>` and `HostFeatureValue.kind: String` were
  already open wire fields), this release adds a genuinely new field
  (`HostElementFacts::content_expression_id`) to a core wire struct — closer in kind to the
  `feature_properties: Option<HostFeatureProperties>` addition that bumped v5→v6, so bumped for
  consistency and to force reprojection of cached artifacts.
  **Deferred, not in scope:** `transitionLinkSource`/`payload` `ParameterMembership`s, `Succession`
  as an addressable owned `Feature`, `BindingConnector`s, and full `AcceptActionUsage`/
  `ActionUsage` typing of trigger/effect remain open — see spec42-systems-modeling-api-gaps.md
  (babel42-v2) S42-003.

## [0.43.2] - 2026-07-17

- **Specialized memberships: Parameter/ViewRendering (S42-003)** — `HostMembershipKind` gains
  `ParameterMembership` and `ViewRenderingMembership` variants. `view rendering` member nodes
  previously fell into the generic `FeatureMembership` bucket. `membership_kind()`
  (`crates/workspace/src/snapshot/facts.rs`) now also takes the owner's `ElementKind`:
  `InOutDecl` (`in`/`out`/`inout`) is shared grammar between genuine Behavior parameters
  (Action/Calc definition and usage bodies — KerML 8.3.19.2 `ParameterMembership`) and Port/
  PortDef directed features, which reuse the same production but are ordinary
  `FeatureMembership`, not parameters. Both previously defaulted to `OwningMembership` since
  `InOutParameter` wasn't dispatched at all. No `PROJECTION_SCHEMA_VERSION` bump: this only
  changes which value some node kinds receive, not the wire shape.
  **Known remaining gap:** `TransitionFeatureMembership` (trigger/guard/effect of a
  `TransitionUsage`) is not yet implementable — Spec42 flattens a transition's guard/effect into
  string attributes (`guardExpression`/`effectExpression`) on the `transition` node rather than
  materializing them as addressable child features, so there is no node to classify as
  `TransitionFeatureMembership` yet. The `transition` node's own membership within its owning
  state is also still unclassified (falls to `OwningMembership`, should be `FeatureMembership`)
  and is deliberately left alone in this release rather than mixed in with the correct fix.

## [0.43.1] - 2026-07-16

- **Specialized memberships: Subject/Stakeholder/Objective (S42-003)** — `HostMembershipKind`
  gains `SubjectMembership`, `StakeholderMembership`, and `ObjectiveMembership` variants,
  alongside the existing `ActorMembership`/`VariantMembership`. `subject`/`stakeholder` member
  nodes previously fell into the generic `FeatureMembership` bucket in `membership_kind()`
  (`crates/workspace/src/snapshot/facts.rs`); `objective` member nodes weren't handled at all and
  silently defaulted to `OwningMembership` — a real misclassification, not a deliberate choice.
  No `PROJECTION_SCHEMA_VERSION` bump: `membership_kind: Option<HostMembershipKind>` already
  exists on the wire shape, this only changes which value three node kinds receive.

## [0.43.0] - 2026-07-16

- **Enumeration vertical slice** — requires `sysml-v2-parser` **0.39.0**, which gives each
  enumerated value inside `enum def { ... }` a real spanned AST node (`EnumeratedValue`) instead
  of a bare `String`.
  - New `ElementKind::EnumeratedValue` (`"enumerated value"`) — `materialize_enum_def` now walks
    `EnumerationBody::Brace { values }` and materializes each enumerated value as its own
    addressable child node, owned by the enclosing `EnumDef`. Initializer expressions (`= expr`)
    and inline bodies (`active { ... }`) remain discarded; only name + span are retained, matching
    the parser's own scope for this construct.
  - New `ElementKind::Enumeration` (`"enumeration"`) — the kind-string an `enum status : Status;`
    usage node was already materialized with by `graph_builder/part_def.rs`'s nested-in-`part def`
    handler, but which had no `ElementKind::parse` mapping (fell to `Unknown("enumeration")`) and
    no package-level dispatch at all (`PackageBodyElement::EnumerationUsage` was a no-op).
    Package-level `enum status : Status;` now materializes and resolves its typing edge the same
    way the nested form already did (`EnumDef` was already a valid typing target).
  - No `PROJECTION_SCHEMA_VERSION` bump: `ElementKind` serializes as a plain string
    (`#[serde(into = "String", from = "String")]`), so new kind variants are additive at the wire
    level, same as the Requirements/Behavior slice's new definition/usage kinds in 0.42.0.

## [0.42.1] - 2026-07-16

- **Fix: `ConstraintDef`/`CalcDef`/`CaseDef` typing resolution** — added the three to
  `TYPING_TARGET_KINDS` (`crates/sysml_model/src/semantic/kinds.rs`), matching what
  `SPECIALIZES_TARGET_KINDS` already allowed. A nested `calc`/`case` usage's Typing edge to its
  definition previously never resolved.
- **Fix: `materialize_case_usage` never wired a typing edge** — unlike its sibling
  analysis/verification/use-case usage builders, it never called `add_typing_edge_if_exists` even
  though `CaseUsage.type_name` is captured by the parser. Fixed alongside the allowlist change
  above, since the allowlist fix alone wasn't sufficient to make `case` usage typing resolve.
- No `PROJECTION_SCHEMA_VERSION` bump — both fixes only improve resolution fidelity of an
  already-projected fact; `HostSemanticProjection`'s shape is unchanged.

## [0.42.0] - 2026-07-16

- **Host projection schema v10** — `Satisfy` and `Subject` relationships classify as
  their own `HostRelationshipMetaclass` instead of the generic `Relationship` fallback.
  Both were already real, resolved graph edges; only the metaclass classification was
  missing.
- **Bug fix: `case`/`case def` bodies were silently dropped** — `materialize_case_def`/
  `materialize_case_usage` created the case node but never walked its body, unlike the
  sibling `use_case`/`analysis_case`/`verification_case` builders. `subject`/`actor`/
  `objective`/`include` members inside a `case`/`case def` are now materialized the same
  way they already were for `use case`/`analysis`/`verification case`.

## [0.41.0] - 2026-07-16

- **Host projection schema v9** — Addressable `HostConnectorEnd` facts for resolved
  `Connection`/`Interface` connect statements, and a `FlowStatementDetail` (payload,
  source/target expression text) plus a distinct `SuccessionFlow` relationship kind for
  `flow`/`succession flow` usages. Requires `sysml-v2-parser` **0.38.0**, which fixes a
  parser bug (PAR-007) where a package-level `connection`/`interface` usage typed with an
  inline `connect ... to ...` clause was misclassified as a definition with the clause
  silently discarded.
- **Connector ends** — `HostSemanticProjection.connector_ends` projects the binary
  `from`/`to` ends of a resolved `connect` statement as addressable, explicitly-ordered
  facts (`end_index` 0/1), derived from the relationship's already-resolved
  `source_id`/`target_id`. N-ary `connect (a, b, c, ...)` ends beyond the binary pair are
  parsed (`ConnectStmt::extra_ends`) but not yet resolved to feature IDs anywhere in the
  graph builder, so they are not projected yet — tracked as a follow-up.
- **Flow detail** — `HostSemanticModelRelationship.flow` carries payload and source/target
  expression text for `Flow`/`SuccessionFlow` edges, mirroring `Connection`'s existing
  `connect` field. `RelationshipKind::SuccessionFlow` distinguishes `succession flow` from
  plain `flow`/`message` (previously all three collapsed into one generic `Flow` edge kind).

## [0.40.0] - 2026-07-16

- **Host projection schema v8** — Addressable Documentation elements with Annotation
  edges, Import/Alias as NamespaceImport / MembershipImport / AliasMembership
  relationships (with visibility), first-class `element_type` for ReferenceUsage,
  and AttributeUsage multiplicity projection. Requires `sysml-v2-parser` **0.37.0**.
- **Documentation** — `doc /* … */` materializes as `ElementKind::Documentation`
  children while keeping `HostElementFacts.documentation` text on the annotated element.
- **Import / Alias relationships** — Containment memberships for import/alias nodes
  use dedicated metaclasses instead of generic Membership.
- **ReferenceUsage** — `ElementKind::Ref` projects `facts.element_type = "ReferenceUsage"`.
- **AttributeUsage multiplicity** — Structured multiplicity ranges from parser 0.37
  project as addressable `HostMultiplicity` facts.

## [0.39.0] - 2026-07-16

- **Host projection schema v7** — Extends v6 feature properties with composite/reference
  ownership and conjugation, adds documentation/short-name facts, library-element flagging,
  richer membership kinds, and relationship metaclasses for Subsetting, Redefinition,
  Subclassification, and Annotation.
- **RefDecl ownership** — Package-level and nested `ref` declarations materialize as
  `ElementKind::Ref` with `is_reference=true` / `is_composite=false` and retain typed
  feature values when present. Ordinary part/item/port/attribute usages default to
  composite ownership.
- **Membership taxonomy** — Containment memberships distinguish FeatureMembership (usages),
  OwningMembership (definitions/packages), plus Import, Alias, VariantMembership, and
  ActorMembership.
- **Relationship family** — Graph edges for `subsets`/`redefines` resolve to Subsetting /
  Redefinition; def `specializes` projects as Subclassification; Annotation edges keep an
  Annotation metaclass.
- **Library / conjugation / names** — `is_library_element` uses the library URI set;
  conjugated port typing (`~Type`) sets `is_conjugated`; `doc` and `shortName` attributes
  lift into typed `HostElementFacts` fields.

## [0.38.0] - 2026-07-16

- **Host projection schema v6** — Projected elements now carry typed
  `feature_properties` on `HostElementFacts` for explicitly declared modifiers:
  direction, abstract/variation, individual, derived, constant, end, ordered,
  and unique. These facts come from the parser AST via `DeclaredFeatureProperties`
  and are separate from the legacy display attribute map.
- **Feature property materialization** — Part, attribute, port, and item usages,
  part definitions, attribute definitions, and directed port parameters retain
  declaration modifiers as semantic facts instead of dropping them after parse.

## [0.37.0] - 2026-07-16

- **Typed host projection v5** — Added API-oriented element facts for declared
  and effective names, ownership, owning membership, and library status. These
  facts are separate from the legacy display-oriented attribute map.
- **Addressable ownership** — Parent containment now records an explicit
  membership form (`OwningMembership` or `FeatureMembership`) and gives both
  the child and relationship the same stable membership identity.
- **Deterministic relationship identities** — Relationship IDs no longer depend
  on graph enumeration order. Explicit connection facts use their source range
  and endpoint expressions as an identity discriminator.

## [0.36.0] - 2026-07-16

- **Parser dependency** — Bumped `sysml-v2-parser` to **0.36.0** and aligned
  Spec42 with typed relationship targets, `Membership`, import visibility, and
  typed `FeatureValue` AST nodes.
- **Host projection** — Projects declared feature values and their structural
  expression trees as addressable `HostFeatureValue` and `HostExpression`
  facts, preserving bind/default/initial semantics without parser debug text.

## [0.35.0] - 2026-07-08

### Added

- **`sysml-v2-parser` upgraded 0.28.0 → 0.32.0**, unlocking new SysML v2 syntax and fixing several evaluation/diagnostic gaps along the way:
  - **0.29.0**: state `do`/`exit` actions, `decide`/`join`/`fork` control nodes, negated `assert`, structured `Transition.effect` (enum, not raw string), structured `AssignStmt.lhs`/`ForLoop.range`. `ElementKind::Decide`/`Join`/`Fork` added; `graph_builder::action` materializes them like `merge` already was; `graph_builder::state` now handles `StateDefBodyElement::Do`/`::Exit` (previously only `Entry`); `AssertConstraintMember.is_negated` captured as an `isNegated` attribute. Deliberately not wired up: `Satisfy.is_negated` and `ConnectStmt.extra_ends` (n-ary connect) — both additive fields that default safely, so existing behavior is unaffected.
  - **0.31.0**: connection ends redefined via `::>` pointing at a nested feature path (not a type name) no longer false-flag an unresolved-type-reference diagnostic (`ElementKind::InterfaceEnd` with a dotted `type_ref` is now recognized and skipped). Reusable requirement/analysis/verification/use-case templates (`is_parametric_definition`) no longer incorrectly report "incomplete" for unassigned values while still surfacing real `failed_constraint` violations. Boolean-equality expressions (`s.flag == true`) inside `require constraint` bodies now evaluate correctly instead of falling through to the numeric/quantity comparison path.
  - **0.32.0**: `variant` members inside a `variation part def` body (`variant part manual : ManualTransmission;`, and the `attribute`/`item`/`port` equivalents) now materialize with their full declared-type node shape (attributes, typing edge, body recursion) via new `materialize_item_usage`/`materialize_variant_usage`, instead of always collapsing to an untyped stub. Requirement usages now carry an `isAbstract` attribute. Expanded reserved-keyword hover coverage.
- **VS Code: `sysml.showVisualizer`/`sysml.showVisualizerActive` editor-title commands** — an icon button in the editor tab toolbar (for `.sysml`/`.kerml` files) that reveals the SysML Visualizer panel, addressing low discoverability since the visualizer moved to the secondary sidebar in 0.34.0. The icon dims to signal the panel is already open, tracked via a `webviewView.onDidChangeVisibility` listener (previously the `sysml.visualizerOpen` context key only updated on first-resolve/dispose, so it went stale after switching tabs without closing the panel).
- **`workspace_session` crate** — actor-model concurrency wrapper (`SessionActor` + lock-free `SnapshotHandle` reads) for embedder-owned session state, enabling non-blocking reads while background work runs.
- **Incremental Workspace Engine** (`crates/workspace/src/incremental.rs`) — standalone engine supporting full-load and incremental-patch graph operations, with `WorkspaceUpdateMetrics` for perf visibility; `build_view_catalog`/`render_view` render a single view without building a full snapshot; `validate_workspace` computes diagnostics directly without one either.

### Changed

- **LSP responsiveness** — hover/completion requests no longer block on in-flight background relink or startup indexing, via the new `workspace_session` actor and a synchronous `WorkspaceSession` state machine (`SessionLifecycle`/`RelinkToken`) in the `workspace` crate. `SemanticCoordinator` (`lsp_server`) has been removed entirely, its responsibilities folded into `WorkspaceSession` (~657 lines removed).
- **Parallel semantic graph building** — workspace and library documents are now parsed/linked in parallel via `rayon` (`build_and_link_graph_parallel`); `lsp_server`'s worker dispatch also moved off hand-rolled `std::thread::spawn` onto rayon, which additionally fixes worker-thread panics that were previously silently swallowed.
- **Semantic graph builder consistency audit** — centralized `part`/`attribute`/`occurrence`/`requirement` usage materialization (previously reimplemented independently per containing context, causing attribute/subtree drift depending on where a usage appeared); specialization edges now wired before body recursion (fixes inherited-member resolution ordering); spec-correct effective-name resolution for unnamed redefining usages (SysML v2 §7.6.5); `package_body.rs`'s dispatcher split into 34 named functions.
- **`ElementKind` type-safety migration** — string-based element-kind/type checks replaced with the `ElementKind` enum across `sysml_model` and `language_service` (completion, import resolution, diagnostics, hover, references); `ElementKind::is_definition()` added as the canonical, compiler-checked replacement for scattered `.ends_with(" def")`/`.contains("_def")` string-suffix heuristics.
- **General-view layout** — ELK graph now nests members under real per-package containers (`elk.hierarchyHandling: INCLUDE_CHILDREN`) instead of laying out every node in one flat tree, and spacing constants were tightened; diagrams with real package structure are meaningfully narrower and less tangled. A single package with many same-depth siblings can still lay out as one very wide row (tracked as a known issue, see below).
- **ELK layout engine version pinned** across the CLI/headless renderer and the VS Code webview build — the webview's `node_modules` had silently drifted to elkjs 0.8.2 while the CLI used the intended 0.11.1; both now resolve the same pinned version with a build-time assertion that fails loudly on drift.
- **Internal view-pipeline refactor (8 phases)** — general-view/interconnection-view/behavior-view rendering code reorganized (ELK option unification, DTO builder dedup, duplicate Rust "prepared view" preparers deleted for 6 of 7 view kinds, exposed-id filter dedup, dead-code deletion); no intended behavior change, verified via before/after CLI export diffing across all 8 baseline view kinds.
- **VS Code settings cleanup** — Removed eight display-only / legacy configuration properties (`spec42.standardLibrary.*`, `spec42.domainLibraries.*`) that were not user-configurable; removed `spec42.codeLens.enabled` (code lens disabled for now) and `spec42.startup.workspaceIndexing` (only `background` mode is tested); fixed encoding corruption in `spec42.debug` description. Sync scripts updated accordingly.

### Fixed

- **Interconnection View: several real connector/rendering bugs found and fixed against a real-world workspace** (`sysml-robot-vacuum-cleaner`):
  - Bare own-port connector endpoints (e.g. `connect leftMotor.phaseIn to phaseLeftIn;`) were silently dropped — a 2-segment relative member chain was indistinguishable from an already-qualified reference and went unprefixed, so the whole connector failed to match.
  - A view exposing a definition-nested member (declared directly inside a `part def` body) resolved **zero connectors** whenever the definition and instance paths used independently-named top-level packages — the def→instance path translation only recognized a narrow `Architecture`/`architecture` naming convention.
  - Sibling subtypes sharing a common ancestor could cross-contaminate def→instance path mappings when a `part def` (not a usage) leaked into the mapping seed, producing orphan nodes and empty parent containers.
  - A package that only ever contains other packages/definitions (e.g. a top-level `PhysicalArchitecture` wrapping a `part def`) rendered as an empty, disconnected box — two independent container-building mechanisms never linked a root-level definition container to its real owning package container.
  - Click-to-source did nothing — scene/prepared-view DTOs weren't carrying `uri`/`range` through to the final payload the renderer consumes, despite the upstream data having them.
  - Node bodies weren't showing multiplicity, port direction, or `redefines`/`subsets` annotations even though the data existed.
- **General-view node bodies rendered completely empty** (header-only, no Attributes/Parts/Ports compartments) whenever the view had a kind-narrowing `filter` clause (e.g. `filter @SysML::PartUsage;`) — the filter ran before compartment-folding needed its input nodes.
- **Duplicate edges during graph linking** — `add_semantic_edge_once` now deduplicates edges that could previously be added more than once via `rebuild_all_document_links`/`rebuild_semantic_graph_staged`.
- **Stale analysis evaluation context after a full workspace load** — inherited context wasn't being reapplied before expression evaluation on some code paths; fixed via `prepare_analysis_evaluation_context`.
- **Evaluated attributes not computed on some incremental-update paths** — fixed via new `finalize_and_evaluate`/`patch_graph_for_document`, replacing `finalize_workspace_graph`.
- **Windows drive-letter normalization** — `workspace::snapshot::discovery::path_to_file_url` now lowercases the drive letter in `file://` URIs (`file:///C:/…` → `file:///c:/…`), matching the normalization applied by `FileSystemDocumentProvider`; previously target URIs and document URIs could differ in case, causing `project_host_semantic_model` to return an empty projection on Windows.
- **`HostSemanticProjection` excludes diagnostic pseudo-nodes** — `ElementKind::Diagnostic` nodes (internal builder markers for unresolvable connect/allocate endpoints) are now filtered out of `HostSemanticProjection`; they are already emitted as first-class `SemanticDiagnostic` entries via `HostValidationReport`.
- **Serde defaults for `HostSemanticModelNode.attributes` and `HostSemanticModelRelationship.connect`** — Added `#[serde(default)]` so older persisted projection artifacts without these fields deserialize correctly instead of failing with a missing-field error.
- **Backward-compatible deserialization for renamed element and relationship kinds** — `ElementKind::parse` now accepts `"requirement usage"` (→ `Requirement`) and `"verification case"` (→ `Verification`); `RelationshipKind::Subject` accepts `"verification"` via `#[serde(alias)]` and `from_persisted_type`; canonical serialization is unchanged.
- **Document URI normalization at ingestion** — `workspace::snapshot::build::enrich_document_hashes` now normalizes each document's URI (drive-letter case) via `language_service::uri::normalize_uri` before the semantic graph is built, complementing the existing `path_to_file_url` canonicalization. Previously the graph itself could be indexed under a differently-cased URI than the canonicalized `target_urls`, causing `project_host_semantic_model` and `collect_host_validation_report` (parse-error diagnostics) to silently return empty results.
- **`interface def` no longer collapses into the same kind as `interface` usage** — added `ElementKind::InterfaceDef`; `interface def Foo { ... }` now produces `ElementKind::InterfaceDef` nodes instead of being mislabeled as the bare `interface` usage kind, matching the part/port/action def-vs-usage pattern. Typing/specializes/subject allowlists in `kinds.rs` updated accordingly.
- **`entry` actions in state bodies no longer become `ElementKind::Unknown`** — `graph_builder::state` now builds entry-action nodes as `ElementKind::Action` (with a `compartment: "entry"` attribute) instead of the unrecognized literal `"entry"`; `state_views::graph_extractor::compartment_action` updated to match on the new attribute.
- **`ElementKind::ActorDef` removed (was never valid SysML v2 syntax)** — the official grammar only defines a bare `actor Foo;` usage (`ActorUsage : PartUsage`), no `actor def` definition form exists. `graph_builder::package_body`'s `PBE::Actor` handler was mislabeling this usage as `"actor def"`; it now builds `ElementKind::Actor` like the equivalent handling elsewhere (`use_case.rs`, `requirement_body.rs`).
- **Flaky diagnostics test removed** — `did_change_republishs_peer_diagnostics_after_debounce` deleted; peer-file diagnostic republish timing is non-deterministic under CI load and the behavior is covered by workspace integration tests.

### Known Issues

- **Interconnection-view and action-flow-view layout is non-deterministic run-to-run** — re-exporting the same unchanged model can produce different node coordinates each time (same node/edge content). Byte-diff-based regression checking is unreliable for these 2 of 8 view kinds specifically; not a new issue in this release.
- **Scoped/incremental IBD builds can resolve a different (but still valid) root than a full-workspace build** for some views in workspaces that use SysML variant-selection — a parity gap, not a correctness bug, but scoped and full-workspace exports of the same view can disagree.

## [0.34.0] - 2026-06-30

### Added

- **SysML v2 Quick Reference panel** — New Help section in the Spec42 sidebar with a "SysML v2 Quick Reference" entry that opens a rich editor tab covering definitions, usages, relationship symbols, annotations, and views — verified against the OMG SysML v2 Language Specification.
- **Spec42 documentation site** — VitePress site published at `https://elan8.github.io/spec42/` via GitHub Pages, with getting-started guide, example walkthroughs, visualizer reference, library guide, and SysML v2 quick reference. Automated deployment via `.github/workflows/docs.yml`.
- **Help sidebar panel** — Sidebar section with quick actions: open visualizer, open recommended example, open quick reference, link to Spec42 docs, link to OMG SysML v2 spec.
- **Drone interconnection performance smoke report** — Nightly CI enforces latency budgets against the in-repo drone example (`drone_interconnection_performance_smoke_report`).

### Changed

- **Visualizer moved to secondary sidebar** — SysML Visualizer is now a persistent sidebar panel (`WebviewViewProvider`) in the secondary sidebar instead of a transient editor panel; removed the editor-title toolbar button.
- **Semantic token accuracy** — Improved token classification for state definitions, flow definitions, port definitions, action definitions, and part definitions; new golden-token regression suite; better declaration-range resolution in `ast_util.rs`.
- **LSP document processing** — New `workspace/coordinator.rs`; cleaner semantic lifecycle management with explicit `SemanticLifecycle` states; improved relink scheduling and reduced redundant re-processing.
- **Library graph handling** — Enhanced caching and concurrency in LSP server library graph loading.
- **Diagram legend accuracy** — Specialization edge now shows a correct hollow triangle; containment shows a proper diamond; removed the incorrect "Hierarchy" entry.
- **Nightly CI cleanup** — `large-workspace-performance` job (tiny fixture, no budget signal) replaced by `performance` (drone interconnection, blocking); `interconnection-visualization-performance` job (always skipped without external fixture) removed; stdlib bundle caching added to `sysml-release-validation` job.
- **Library sidebar** — Simplified styling and layout of the library panel.
- **Release surface alignment** — Rust workspace, `spec42` server, VS Code extension, Zed extension, and GitHub Action examples aligned at `0.34.0`.

### Fixed

- **Visualizer open state tracking** — `visualizerOpen` in the extension debug state now correctly reflects whether the visualizer view has been resolved, rather than always returning `true` after activation.
- **`BaseVisualizationPanelController.dispose()`** — Restore state is now cleared on dispose in all code paths (previously only cleared when going through `VisualizationPanel.dispose()`).
- **Flaky diagnostics test** — `did_change_republishs_peer_diagnostics_after_debounce` now uses barrier-based polling with a 5-second deadline instead of a fixed `sleep(250ms)`, eliminating race conditions on loaded CI runners.

## [0.33.0] - 2026-06-29

### Added

- **`workspace` crate** — Protocol-neutral embedding for workspace build, snapshot loading, semantic comparison, incremental updates, cancellation, and resource limits (ADR 0003).
- **Semantic comparison** — `compare_snapshots` returns a versioned `SemanticComparisonReport` for element, relationship, diagnostic, and view catalog diffs with identity-preservation assessment.
- **Artifact metadata** — `HostArtifactMetadata` records schema versions, engine version, library catalog hash, and per-document source hashes.
- **TypeScript bindings (`sysml_model`)** — `ts-rs` integration exports shared DTOs (including unified `Position` and `Range`) for the diagram renderer and extension surfaces.
- **Component view module** — Part and port expansion for semantic graph queries.
- **Library parse caching** — LSP server caches library graph parses across workspace sessions.
- **`WorkspaceRenderCache`** — LSP visualization warm-cache integration for `sysml/visualization` requests.
- **Incremental snapshot updates** — Experimental incremental workspace updates with parity, fallback, and benchmark tests.
- **Robot vacuum showcase** — Integration tests and performance analysis documentation for the release-gating fixture.
- **Performance guardrails** — Nightly budget enforcement via `scripts/check-perf-budgets.mjs` on large-workspace and drone-interconnection fixtures.
- **`edges_for_uri`** — Semantic graph query method for URI-scoped edge lookup with improved serialization.
- **Parent-child relationship handling** — Semantic graph evaluation now models parent-child relationships explicitly.

### Changed

- **Crate restructuring** — `semantic_core` renamed to `sysml_model`, `kernel` renamed to `lsp_server`, and embedding concerns extracted into the new `workspace` crate.
- **Parser dependency** — Bumped `sysml-v2-parser` to **0.28.0** on [crates.io](https://crates.io/crates/sysml-v2-parser); graph builders updated for new AST body/member enums.
- **DTO unification** — Neutral `TextPosition`/`TextRange` span types and unified `Position`/`Range` DTOs across `sysml_model`.
- **LSP server** — Refactored document handling for improved concurrency; library graph caching; streamlined diagnostics and visualization data paths.
- **Semantic graph** — `Arc`-based memory management, query performance improvements, and stale-cache invalidation fixes.
- **IBD payload merging** — Refactored merge path with enhanced workspace instance mapping.
- **Element kind handling** — Refactored predicates for improved type safety and consistency.
- **CI workflows** — Updated for crate renaming; diagram renderer performance tracking in CI.
- **Documentation** — Engineering docs, Sequence View notes, and architecture references updated for current crate names.
- **Release surface alignment** — Rust workspace, `spec42` server, VS Code extension, Zed extension, and GitHub Action examples aligned at `0.33.0`.

### Fixed

- **Stale semantic graph cache** — Cache invalidation after graph mutations restores correct query results.
- **LSP integration test** — Diagnostic handling for loose-file workspaces.
- **MCP tools test** — Additional error-message assertion for tool failure paths.

## [0.32.0] - 2026-06-22

### Added

- **`language_service` crate** — Shared language-service layer extracted from the kernel; integrated into LSP runtime and core tests.
- **`preparedView` pipeline** — Rust builds render-ready `preparedView` DTOs for interconnection and standard views; LSP can emit slim interconnection payloads that omit duplicate `ibd`, `graph`, and `interconnectionScene` fields when `preparedView` is present.
- **Headless SVG export** — Shared `@spec42/diagram-renderer` headless path for CLI/API diagram export and VS Code export smoke tests, aligned with the webview renderer.
- **Standard views** — Browser, grid, and geometry view preparers in `semantic_core` and the shared diagram renderer.
- **Projection hints** — View projection hints and rendering resolution for defined SysML views; standard-view defaults and swim-lane handling in activity diagrams.
- **IBD build scoping** — `ViewExposedPackages` scope for interconnection builds reduces work on large workspaces; scoped-vs-full parity and performance smoke tests.
- **Legacy ELK SVG (test-only)** — Rust `legacy_elk_svg` module retains `interconnectionScene` ELK parity probes; production export uses headless `preparedView`.
- **Diagram export quality analysis** — Engineering notes and integration coverage for export/readability invariants.

### Changed

- **Interconnection scene schema version 2** — Canonical interconnection scenes and prepared views use schema version 2; integration tests poll `preparedView` instead of legacy `interconnectionScene`/`ibd` counts.
- **Visualization workspace cache** — Model Explorer and visualizer share workspace render snapshots; slim `preparedView` interconnection responses are cached on warm `sysml/visualization` requests.
- **Parser dependency** — Bumped `sysml-v2-parser` to **0.25.6** on [crates.io](https://crates.io/crates/sysml-v2-parser).
- **Library closure resolution** — Further hardening of import-scoped library loading and workspace-wins merge behavior.
- **CI and agent surfaces** — Core vs agent/API/MCP test split in CI; ignored integration tests documented for MCP/API workflows.
- **Release surface alignment** — Rust workspace, `spec42` server, VS Code extension, Zed extension, and GitHub Action examples aligned at `0.32.0`.

### Fixed

- **Slim interconnection cache** — Visualization response cache accepts `preparedView`-only interconnection payloads (restores warm-cache behavior after slim-payload refactor).
- **IBD unit tests** — Correct `extract_impl` test imports for `merge_ibd_payloads` and related IBD helpers after module layout changes.
- **Diagram renderer types** — TypeScript `PreparedView` passthrough casts in shared prepare/headless export paths.

## [0.31.0] - 2026-06-15

### Added

- **KPAR crate (`crates/kpar`)** — Read, validate, and materialize KerML Project Archives (`.project.json`, `.meta.json`, SHA-256 checksums, textual `.sysml`/`.kerml` sources). `kpar-pack` binary packs Elan8 domain libraries for release; `extract_archive_subset` lives in `legacy.rs` for zip subset helpers used in tests.
- **Domain libraries via KPAR** — Bundled domain libraries ship as `elan8-domain-libraries-{version}.kpar`. `config/domain-libraries.json` pins `format`, `version`, and release `artifact`. Runtime materializes the embedded KPAR on first use; managed install path no longer uses a `tree/` subdirectory when `contentPath` is empty.
- **Standard library via OMG KPAR** — Bundled stdlib embeds the OMG `sysml.library.kpar` archives from the pinned SysML v2 Release tag. Multiple KPAR subroots are materialized and mounted for semantic indexing (`stdlib_roots` in environment resolution).
- **Domain library release automation** — GitHub Action on [elan8/sysml-domain-libraries](https://github.com/elan8/sysml-domain-libraries) (`release-kpar.yml`) builds and publishes the KPAR asset plus `SHA256SUMS.txt` on `v*` tags.
- **KPAR embed smoke test** — `crates/server/tests/kpar_stdlib_embed_smoke.rs` verifies embedded OMG KPAR stdlib materialization and `ScalarValues::Real` import resolution via `spec42 check`.

### Changed

- **Library distribution format** — Standard and domain libraries use KPAR only. Tree-format zip extraction and repack fallbacks removed from `build.rs`, `stdlib.rs`, `domain_libraries.rs`, and fetch scripts.
- **Stdlib fetch (KPAR-only)** — `scripts/fetch-stdlib-bundle.sh` sparse-checkouts only `sysml.library.kpar/` from the pinned OMG release tag (not the full release zip and not `master`). All embed KPAR inputs live under repo-root `.cache/` (stdlib subdirectory + domain `.kpar` file).
- **Domain library fetch** — `scripts/fetch-domain-libraries-bundle.sh` downloads the release `.kpar` asset or packs locally from `SPEC42_DOMAIN_LIBRARIES_SOURCE_DIR` / sibling `../sysml-domain-libraries`.
- **Build embed pipeline** — `build.rs` embeds `bundled-sysml-kpar/*.kpar` for the OMG standard library and raw `.kpar` bytes for domain libraries. Maintainer override: `SPEC42_STDLIB_KPAR_DIR` (replaces `SPEC42_STDLIB_BUNDLE_ZIP`).
- **Library path resolution** — `spec42 doctor` and environment resolution expose multiple stdlib roots when using the KPAR distribution; VS Code library settings sync `format: kpar` from `config/*.json`.
- **CI and release workflows** — `ci.yml`, `release.yml`, and `nightly.yml` cache embed KPAR inputs under `.cache/` (`SPEC42_STDLIB_KPAR_DIR` and `SPEC42_DOMAIN_LIBRARIES_BUNDLE_ZIP`). OMG ratchet jobs in `nightly.yml` still clone the full `SysML-v2-Release` checkout separately.
- **Local build cache** — `.cache/` at the repository root is gitignored; KPAR build inputs are never committed.

### Removed

- **Tree-format library bundles** — No support for textual `sysml.library/` trees or git-archive zip repack as embed input. Installation from bytes requires KPAR archives.
- **`SPEC42_STDLIB_BUNDLE_ZIP`** — Replaced by `SPEC42_STDLIB_KPAR_DIR` pointing at a directory of OMG `.kpar` files.

### Added

- **Library import closure tests** — Kernel `validate_paths` integration test for webshop-like workspaces with duplicate domain-library packages; semantic_core tests for import-scoped closure, conditional unit catalogs, and workspace-wins graph merge.
- **Semantic resolution contract** — New [resolution-contract.md](crates/semantic_core/docs/resolution-contract.md) documenting name-resolution pipeline stages and resolver entry points; `kinds.rs` canonical element-kind predicates; refactored graph pipeline for materialization, merging, linking, and pending resolution.
- **Flow usage projection** — Dedicated `flow_usage` module for consistent flow edge emission across body elements.
- **Metadata OMG 14c compliance** — Shorthand metadata redefine features, `subsetsFeature` projection for redefine shorthand, and nightly OMG 14c metadata compliance check.
- **Import namespace resolution** — `import_namespace_target_candidates` for accurate all/membership import target generation.
- **Requirement satisfaction diagnostics** — Checks for requirement satisfaction by parts per SysML specifications.
- **Robot vacuum showcase test** — Integration test validating diagnostics on the showcase model.

### Changed

- **Parser dependency** — Bumped `sysml-v2-parser` to **0.25.4** on [crates.io](https://crates.io/crates/sysml-v2-parser); removed local patch configuration. Flow handling refactored through the new `flow_usage` module.
- **Verification case diagnostics** — Then-action return semantics aligned with SysML v2 (verification cases with then-actions and no explicit return are valid); removed outdated then-action count checks in requirement case conformance.
- **Semantic metadata graph building** — Improved parent ID handling for semantic metadata definitions in the graph builder.
- **Release surface alignment** — Rust workspace, `spec42` server, VS Code extension, Zed extension, and GitHub Action examples aligned at `0.31.0`.

### Fixed

- **Library import closure** — Library files enter the semantic graph only through transitive `import` closure; workspace-declared packages satisfy imports without loading same-named library copies (fixes ambiguous `view_expose_unresolved` and empty views when domain libraries include examples such as webshop). Unit catalogs (`ScalarValues`, `ISQ`, `QUDV`, etc.) load only when the closure needs them. Graph merge prefers workspace declarations when duplicate package names slip through (`spec42 check`, LSP, and filesystem provider).
- **Import resolution** — Enhanced namespace import target resolution for all and membership imports.
- **Cyclic state definitions** — Diagnostic checks for cyclic state definition graphs.
- **Verdict evaluations** — Improved verdict evaluation diagnostics.
- **Monetary units** — Correct resolution of monetary units from indexed catalogs during evaluation.

## [0.30.0] - 2026-06-12

### Added

- **Canonical interconnection pipeline** — Language server emits `interconnectionScene` (schema version 1) for Interconnection views; shared `@spec42/diagram-renderer` prepares and lays out scenes with ELK; VS Code and export paths consume the same pipeline.
- **IBD and interconnection scope** — Merged workspace IBD, scoped interconnection filtering, package container groups, and diagram export improvements for internal block diagrams.
- **Visualization performance** — Lazy single-view projection, workspace visualization artifact cache (shared by Model Explorer and visualizer), per-diagram response cache, and slimmer interconnection LSP payloads. Documented in [docs/engineering/POWER-SYSTEMS-PERFORMANCE-ANALYSIS.md](docs/engineering/POWER-SYSTEMS-PERFORMANCE-ANALYSIS.md) with `powersystems_system_context_performance_report` integration test.
- **Domain libraries bundled like stdlib** — Removed the `domain-libraries` git submodule. Elan8 domain libraries are embedded in the Spec42 server binary, materialized under the data directory on first use, and shown in the VS Code Library dashboard next to the standard library. Local dev auto-detects a sibling `../sysml-domain-libraries` checkout or uses `SPEC42_DOMAIN_LIBRARIES_SOURCE_DIR`.
- **Workspace lifecycle feedback** — Clear indexing/ready states for semantic queries; visualizer and Model Explorer respect lifecycle before building diagrams.
- **Analysis typing and evaluation** — Richer analysis usage typing, inherited analysis results, and conformance-matrix updates for metadata and quantities.
- **Diagnostics** — Engineering-units validation, SysML kind-compatibility checks, and improved reference resolution messaging.

### Changed

- **Parser dependency** — Bumped `sysml-v2-parser` to **0.25.2** on [crates.io](https://crates.io/crates/sysml-v2-parser): metadata in constraint bodies, structured recovery for state `ref` and part-usage bind/ref bodies, case-body `ref :>>` and verdict spans, expression classification, and related graph/diagnostic follow-through since 0.29.1.
- **Interconnection visualization** — Legacy interconnection webview paths removed; layout, routing, and drawing consolidated in the shared renderer.
- **LSP perf logging** — `backend:sysmlVisualizationRequest` includes `cacheHit`, `ibdMs`, `viewEvalMs`, and `sceneMs`.
- **Release surface alignment** — Rust workspace, `spec42` server, VS Code extension, Zed extension, and GitHub Action examples aligned at `0.30.0`.

### Fixed

- **Visualization response cache** — Do not cache or serve incomplete `interconnection-view` responses (avoids stale empty scenes in integration tests and after fast view switches).
- **Extension build** — Resolve `elkjs` when bundling `diagram-renderer` into the VS Code extension host.
- **Interconnection integration tests** — Poll until `interconnectionScene` is ready before asserting drone/grid diagram invariants.

## [0.29.1] - 2026-06-09

### Added

- **Graph depth P4b–P8 (semantic depth)** — RenderingDef and OccurrenceUsage body walks; shared case-body wiring (`SubjectRef`, `FirstSuccession`, `ThenUseCaseUsage`, `RefRedefinition`); `MetadataAnnotation` in action/requirement bodies; typed graph facts (`valueIsBoolean`, `rhsIsBoolean`); state/interface/payload semantic tokens; hover signatures for view rendering, ref redefinition, filters, and verdicts.
- **Graph depth P1** — Action/interface/requirement body projection: `ActionDefBody` and `ActionUsageBody` walked (`then action`, nested actions, assign/ref/state/for); interface ends expose `portType` with end-typing `Connection` wiring; requirement `verify` and `subject` members emit graph nodes and `Subject` edges. Integration tests: `action_body_semantics`, `interface_body_semantics`, `requirement_body_semantics`.
- **Graph depth P2** — Action-flow graph enrichment for activity diagrams; semantic token recursion for action/requirement bodies; `item def`/`individual def` `AttributeBody` projection; `require constraint` child graph nodes. Integration tests: `activity_graph_semantics`, `item_def_body_semantics`, `action_definitions`.
- **Graph depth P3** — `definition_body`/`occurrence_body` walkers for occurrence and flow definitions; `PartDefBody` completion (`enum`/`item` usages, opaque members, occurrence brace bodies); semantic token expansion for part/item/metadata inner identifiers and requirement body gaps; hover and symbol kinds for P3 node kinds. Integration tests: `definition_body_semantics`, `part_def_body_semantics`, `part_def_tokens`.
- **Graph depth P4a** — Flow/allocation `DefinitionBody` semantics and semantic tokens; General View filters `require constraint` child nodes while preserving inline constraint text. Integration tests: `definition_body_semantics`, `flow_def_tokens`, `model_projection`.

### Changed

- **Parser dependency** — Bumped `sysml-v2-parser` to **0.20.1** on [crates.io](https://crates.io/crates/sysml-v2-parser/0.20.1) (flow/allocation definition bodies, `MetadataAnnotation` in action bodies, `AttributeDef.value_span`, literal-with-unit spans).
- **Release surface alignment** — Rust workspace, `spec42` server, VS Code extension, Zed extension, and GitHub Action examples aligned at `0.29.1`.

## [0.29.0] - 2026-06-08

### Added

- **Parser 0.19.0 graph projection** - `PayloadClause` on actions/transitions, `FinalState`, `MetadataKeywordUsage`, viewpoint `stakeholder`/`purpose`/`TextualRep`; parser-wave fixtures and extended `p2_diagnostics_semantics` coverage.
- **`missing_final_state`** - Information diagnostic when a state definition has state usages but no `final` state.
- **AST-based Boolean classification** - Transition guards and view/import filters set `conditionIsBoolean` at graph-build time (replaces string heuristics when available).
- **Verification case `attribute def`** - Local attribute definitions in verification bodies are projected on the semantic graph.

### Changed

- **Parser dependency** - Bumped `sysml-v2-parser` to **0.19.0** on [crates.io](https://crates.io/crates/sysml-v2-parser) (`PayloadClause`, `TransitionAccept`, `FinalState`, metadata keywords, viewpoint members).
- **`viewpoint_rep_language_unresolved`** - Diagnostic range prefers parser `language_span` on `textualRep` nodes when present.
- **Release surface alignment** - Rust workspace, `spec42` server, VS Code extension, Zed extension, and GitHub Action examples aligned at `0.29.0`.

## [0.28.0] - 2026-06-08

### Added

- **Read-only HTTP API** - `spec42 api serve` exposes workspace validation, doctor, model summary, and diagram export over loopback HTTP; documented in [docs/api/README.md](docs/api/README.md) and [docs/adr/0001-read-only-systems-modeling-http-api.md](docs/adr/0001-read-only-systems-modeling-http-api.md).
- **P2 semantic diagnostics** - Behavior, expression, requirement/case, and view/metadata conformance checks (for example `transition_guard_non_boolean`, `accept_payload_incompatible`, `assignment_value_incompatible`, `view_filter_non_boolean`, `verification_case_invalid_shape`, `viewpoint_reference_unresolved`, `metadata_keyword_collision`); catalog entries and integration tests in `p2_diagnostics_semantics`.
- **Diagnostic pipeline modularization** - Dedicated collectors for behavior, requirement/case, and view/metadata conformance; shared Boolean filter helpers; graph-builder facts for transition guards, view filters, viewpoint bodies, send/accept payloads, and case shape counters.
- **Unit suffix handling** - Quantity/unit suffix validation and richer hover for attributed values with units.
- **Inherited analysis results** - Analysis-case objectives can inherit `analysis result` bindings from specializing definitions.
- **Documentation index** - [docs/README.md](docs/README.md) with user, engineering, architecture, reference, and archive folders; legacy top-level doc paths updated.
- **CodeQL** - `.github/workflows/codeql.yml` for security analysis of the Rust codebase.

### Changed

- **Parser dependency** - Bumped `sysml-v2-parser` to **0.18.0** (metadata definition bodies, `expose` feature chains, port body depth, and related graph-builder follow-through).
- **GitHub Action Marketplace name** - Renamed action listing to `Spec42 SysML Check` so the Marketplace `name` does not collide with the GitHub user `spec42`.
- **CI workflows** - Refactored standard-library bundle handling, integration-test timeouts, performance metrics logging, and Tool42-driven Rust test orchestration.
- **Release surface alignment** - Rust workspace, `spec42` server, VS Code extension, and Zed extension versions aligned at `0.28.0`.

### Fixed

- **Standard library test configuration** - Corrected stdlib path resolution and error handling in integration tests.
- **Model Explorer selection sync** - Retry path for editor-to-explorer selection under concurrent refresh (carried forward from post-0.27.1 work).

## [0.27.1] - 2026-06-05

### Added

- **Diagnostic roadmap** - Added [docs/engineering/DIAGNOSTIC-CHECKS-ROADMAP.md](docs/engineering/DIAGNOSTIC-CHECKS-ROADMAP.md) to document status and rollout sequencing for diagnostic quality checks.
- **Regression coverage** - Expanded integration and unit tests for unresolved-reference diagnostics, rename/reference behavior, and startup/update flow gating.

### Changed

- **Semantic diagnostics** - Refined unresolved relationship and reference diagnostics, including clearer endpoint messaging and post-processing behavior.
- **Reference and rename resilience** - Improved reference resolution and rename flows to handle broader document/editing scenarios with better stability.
- **Model Explorer sync reliability** - Added a retry path for debug source-to-explorer selection sync to reduce CI flakiness under concurrent refresh activity.
- **Release surface alignment** - Rust workspace, `spec42` server, VS Code extension, and Zed extension versions aligned at `0.27.1`.

## [0.27.0] - 2026-06-04

### Added

- **AI assistant integration** - [docs/AI-ASSISTANTS.md](docs/AI-ASSISTANTS.md), [`.github/copilot-instructions.md`](.github/copilot-instructions.md), and [docs/examples/mcp-vscode.json](docs/examples/mcp-vscode.json) for Copilot/Cursor MCP setup.
- **MCP tools** - `spec42_doctor`, `spec42_model_summary`, and `spec42_explain_diagnostic` on `spec42-mcp`; `spec42_check` gains optional `include_semantic_model`.
- **MCP test coverage** - Handler error-path tests, in-process `rmcp` protocol smoke (`mcp_protocol`), subprocess `spec42-mcp` smoke (`mcp_binary`), and unit tests for the diagnostic code catalog; CI builds `spec42-mcp` explicitly.
- **VS Code Language Model Tools** - Four Copilot Agent tools (`spec42_check`, `spec42_doctor`, `spec42_model_summary`, `spec42_explain_diagnostic`) via bundled `spec42` CLI; extension `engines.vscode` ^1.99.0.
- **CLI agent commands** - `spec42 explain-diagnostic` and `spec42 model-summary` (JSON parity with MCP); integration tests in `cli_ai_tools`.
- **Semantic relink** - LSP/code-action path to relink symbols when the semantic graph has moved ahead of editor state.
- **Unresolved type quick fix** - Quick fix to create a matching type definition for unresolved type references.
- **Library view quick fixes** - Quick fixes and clearer library status handling in the library webview.
- **Model Explorer selection sync** - Editor selection and Model Explorer tree stay aligned when navigating structure from either side.
- **Visualization commands** - Additional palette commands for opening and refreshing visualizer views.

### Changed

- **Parser dependency** - Bumped `sysml-v2-parser` to **0.17.0** on [crates.io](https://crates.io/crates/sysml-v2-parser). Structured view/part bodies, `implies` in expressions, and usage-header `:>` / `::>` / `=>` on attribute and port usages (see parser `CHANGELOG.md`).
- **Graph builder for parser 0.17.0** - Maps `AttributeUsage` and `PortUsage` `subsets` / `references` / `crosses` into semantic node attributes.
- **Definition insertion** - Snippet/insertion logic and indentation handling for new definitions in SysML bodies.
- **Hover documentation** - Richer hover text for semantic elements.
- **README and extension metadata** - Onboarding and marketplace-facing description updates for AI assistant and workspace workflows.
- **Release surface alignment** - Rust workspace, `spec42` server, VS Code extension, and Zed extension versions aligned at `0.27.0`.

### Fixed

- **Action smoke CI** - Corrected actionlint invocation in `.github/workflows/action-smoke.yml`.

## [0.26.3] - 2026-06-04

### Fixed

- **GitHub Action version resolution** - Install step now uses `github.action_ref` when `inputs.version` is omitted. Composite actions do not expose `GITHUB_ACTION_REF` to `run` scripts, which caused `No Spec42 version was provided and GITHUB_ACTION_REF is empty` in downstream workflows such as [sysml-examples](https://github.com/elan8/sysml-examples).
- **GitHub Action release ref validation** - Reject branch-style refs (for example `main` → `vmain`) with a clear error; only semver tags like `v0.26.3` download release archives.
- **GitHub Action SARIF upload on forks** - SARIF upload runs with `continue-on-error` so fork PRs still fail only on validation results, not Code Scanning upload restrictions.

### Added

- **Action smoke CI** - `.github/workflows/action-smoke.yml` runs actionlint and exercises the composite Action against a published release binary.

### Changed

- **Release surface alignment** - Rust workspace, `spec42` server, VS Code extension, and Zed extension versions aligned at `0.26.3`.

## [0.26.2] - 2026-06-04

### Fixed

- **GitHub Action manifest** - Quoted the `format` input description in `action.yml` so YAML parsers accept the colon in `spec42 check: text, ...` (fixes `Mapping values are not allowed in this context` when loading `elan8/spec42@v0.26.1`).

### Changed

- **Release surface alignment** - Rust workspace, `spec42` server, VS Code extension, and Zed extension versions aligned at `0.26.2`.

## [0.26.1] - 2026-06-04

### Added

- **GitHub Action** - Composite Action (`action.yml`) that downloads the matching Spec42 release binary for the runner OS, runs `spec42 doctor`, validates models with `spec42 check`, and uploads SARIF to GitHub Code Scanning by default. Documented in [docs/GITHUB-ACTION.md](docs/GITHUB-ACTION.md).

### Changed

- **Release surface alignment** - Rust workspace, `spec42` server, VS Code extension, and Zed extension versions aligned at `0.26.1`.

## [0.26.0] - 2026-06-03

### Removed

- **Software Architecture add-on** - Removed the experimental Rust workspace analyzer (`software-architecture` crate), `software/*` LSP RPCs, dedicated software visualizer views (`software-module-view`, `software-dependency-view`), legacy `generalView.ts` / `graphBuilders.ts` extension renderer path, and the Spec42 Add-ons sidebar UI with related settings and commands.

### Added

- **Workspace import graph** - [`workspace/import_graph.rs`](crates/kernel/src/workspace/import_graph.rs) detects workspace files that import a changed package so `didChange` can republish importer diagnostics immediately (debounced full-workspace republish remains as a backstop).
- **Import/namespace regression coverage** - Added semantic-core tests for recursive namespace import (`::**`) so nested members remain resolvable in cross-document typing and ref scenarios.
- **Import and ref test matrix** - Extended tests for membership imports (`import Pkg::*`), qualified package declarations, part-def/part-usage ref parity, and multi-file `perform_check` workspaces.
- **Nested port semantics tests** - Integration tests for nested port bodies in port definitions and part usages.
- **CLI check workspace-root smoke coverage** - Added server smoke coverage that validates `perform_check` behavior with an explicit `workspace_root`, matching common `spec42 check` workspace invocations.
- **Semantic index ready notification** - LSP clients receive `spec42/semanticIndexReady` after workspace indexing so startup diagnostics filtering and Model Explorer can align with graph readiness.
- **Docs** - [AST-SEMANTIC-COVERAGE.md](docs/engineering/AST-SEMANTIC-COVERAGE.md) prioritization matrix and [LEGACY-RENDERER-SUNSET.md](docs/archive/LEGACY-RENDERER-SUNSET.md) removal plan for legacy webview renderers.

### Changed

- **Parser dependency** - Bumped `sysml-v2-parser` to **0.16.0** on [crates.io](https://crates.io/crates/sysml-v2-parser) (no local path / git pin required for CI). Brings requirement-body `actor` declarations, `enum` usages in part bodies, diagnostic taxonomy and cascade suppression, and fewer false positives on spec-aligned models (see parser `CHANGELOG.md`).
- **Graph builder for parser 0.16.0** - Handles `RequirementActorDecl` in requirement bodies and `EnumerationUsage` in part def/usage bodies; drops kernel priority for removed `missing_statement_separator_between_members` parse code.
- **Examples sidebar** - Lists example workspaces from a single canonical `examples/` root (repository submodule) instead of scanning both `vscode/examples` and `../examples`; hides dot-prefixed folders such as `.github`.
- **Workspace-first extension behavior** - Default workspace indexing is `background`; Model Explorer shows indexing status until the workspace model is ready and no longer falls back to the active file tree when a workspace folder is open. Status bar and **Validate Model** summarize diagnostics across workspace SysML/KerML files. `sysml/model` graph requests from the extension use `workspaceVisualization` when a workspace is open.
- **Cross-file diagnostic refresh** - After `didChange`, importer files are republished in the same handler once the semantic graph is updated; a debounced workspace pass still runs after idle typing.
- **Untyped part usage severity** - `untyped_part_usage` is **Information** (was Warning) to match SysML typing optionalities on usages.
- **Analysis diagnostics on requirements** - Requirement definitions are no longer skipped for analysis status collection (only `constraint def` / `calc def` remain excluded).
- **Nested port bodies in semantic graph** - Port usage bodies (`PortBody::Brace`) are now walked in the graph builder (port def, part def, and part usage paths) so nested ports appear in the workspace graph and views.
- **Semantic tokens for ports** - Token range collection recurses nested port bodies and includes `InOutDecl` members in port definitions.
- **Shared renderer default alignment** - Webview `htmlBuilder` fallback for `useSharedRenderer` matches `package.json` default (`true`).
- **Ref assignment graph parity** - `ref` assignments inside `part def` bodies now emit `reference` edges the same way as `part usage` bodies, reducing reliance on identifier-only fallbacks in downstream models.
- **Type disambiguation for view symbols** - Import/type resolution now includes view/viewpoint suffix disambiguation paths, improving nested namespace resolution for viewpoint conformance and view typing.
- **Release surface alignment** - Rust workspace, `spec42` server, VS Code extension, and Zed extension versions aligned at `0.26.0`.

### Fixed

- **Flaky cross-file diagnostic integration test** - `did_change_republishs_peer_diagnostics_after_debounce` now waits for the latest peer `publishDiagnostics` instead of racing debounce timing against `hover`.
- **Standard SysML view types in examples** - Spec42 standard view types (`InterconnectionView`, `SequenceView`, `StateTransitionView`, `ActionFlowView`, and related) are recognized for type diagnostics like `GeneralView`, so `examples/webshop/Views.sysml` no longer reports spurious `unresolved_type_reference` warnings.
- **Cross-file allocate/satisfy endpoints** - Imported simple names such as `CheckoutService` in `allocate CheckoutService to CommerceCluster` resolve workspace-wide, so `unresolved_allocate_source` is no longer reported when the target exists in another file.
- **Model Explorer diagnostic nodes** - Internal builder diagnostic nodes (for example `unresolved_allocate_source`) are excluded from workspace/document graph payloads and tree building, so they no longer appear as spurious children in Model Explorer.
- **Allocate/satisfy diagnostic clarity** - Relationship endpoint diagnostics now explain when a name resolves to a type definition instead of a usage (`allocate_endpoint_prefers_usage`, `satisfy_endpoint_prefers_usage`), suggest concrete usage paths, and hint at case-mismatched part usages when applicable.
- **Part connection and port compatibility diagnostics** - Clearer semantic messages for allocation typing and homonymous port definitions.

## [0.25.0] - 2026-06-02

### Added

- **`diagram_core` crate** - Added a shared Rust diagram-core layer for reusable view/model shaping and renderer-facing contracts used by visualization pipelines.
- **`sysml_semantic_tokens` crate** - Introduced a dedicated semantic-token crate with parser-aware range extraction, keyword classification, and focused token tests.
- **Shared diagram renderer package** - Added `shared/diagram-renderer` with normalized graph preparation, node-notation handling, theme support, and view-specific renderer modules (general/IBD/action/state/sequence).
- **Pending-relationship diagnostics** - Added explicit semantic diagnostics for unresolved pending relationships to improve cross-document and late-resolution feedback.
- **Viewpoint conformance coverage** - Added viewpoint conformance fixtures and semantic tests to validate conformance graph and diagnostic behavior.
- **General/interconnection notation audit artifacts** - Added BNF/sign-off and notation-inventory docs plus generation tooling to track renderer/spec parity.

### Changed

- **Parser dependency upgrade** - Bumped `sysml-v2-parser` to git tag `v0.14.0` (`https://github.com/elan8/sysml-v2-parser`) and switched back from local path wiring to the tagged GitHub source. This brings qualified package identification support and `ref part` assignment parsing improvements.
- **Semantic graph/evaluation architecture** - Expanded semantic graph construction, relationship resolution, and analysis evaluation paths (including unit-aware handling) for stronger cross-file behavior and more consistent graph-first outputs.
- **Visualization/webview pipeline** - Refactored webview orchestration with render scheduling, quiescence/gating, shared-renderer adapters, and updated UI control/state flows for more predictable refresh behavior.
- **Workspace/library closure handling** - Improved document/workspace services and library-closure flows to better preserve cross-document linkage and indexing consistency.
- **Release surface alignment** - Updated Rust workspace/server, VS Code extension, and Zed extension versions in lockstep for `0.25.0`.

### Fixed

- **Reference and endpoint resolution edge cases** - Improved handling for typed/member-chain and pending-expression endpoint resolution so semantic links and diagnostics are less sensitive to declaration order and container context.
- **IBD/interconnection rendering consistency** - Fixed multiple interconnection/IBD shaping and endpoint-mirroring issues that could lead to missing or unstable rendered structure.
- **Verification/subject relationship propagation** - Corrected subject and verified-requirement relationship wiring so verification semantics are represented consistently in the semantic graph.

## [0.24.0] - 2026-05-15

### Added

- **`semantic_core` crate** - Extracted reusable semantic engine (graph building, resolution, evaluation, diagnostics, and graph-first visualization) for use by `kernel`, future hosts, and services without LSP coupling.
- **MCP server** - Added `spec42-mcp` stdio server with `spec42_check` tool exposing the same validation pipeline as the CLI for AI assistant workflows.
- **`software-architecture` crate** - Promoted software-architecture support from an in-tree plugin to a dedicated crate with custom RPC provider wiring.
- **Graph-first diagnostics engine** - Added neutral diagnostics collection in `semantic_core` with kernel/LSP adapter integration.
- **Semantic core architecture guide** - Added `docs/architecture/SEMANTIC_CORE_ARCHITECTURE.md` documenting module layout and graph-first visualization/diagnostics flows.
- **Content submodule helper** - Added `scripts/update-content-submodules.ps1` for updating `domain-libraries` and `examples` submodules.

### Changed

- **Parser dependency upgrade** - Updated `sysml-v2-parser` integration to `v0.10.0` and aligned graph-builder paths.
- **Visualization and DTO architecture** - Moved graph-first visualization, sequence extraction, IBD/interconnection projection, and shared DTOs into `semantic_core`; slimmed `kernel` to host/runtime concerns.
- **Sequence view extraction** - Reworked sequence-diagram extraction to operate on the semantic graph rather than legacy view-layer paths.
- **Release and CI packaging** - Extended release workflow and validation pipelines for `spec42-mcp`, recursive submodule checkout, and updated binary staging.
- **Default logging** - Reduced default runtime logging noise for quieter editor sessions.

### Fixed

- **IBD composite scope** - Pruned IBD payloads to retain unconnected parts that belong under the same composite structure.
- **Build and duplication guardrails** - Removed duplicate semantic implementations and added dependency/debt guardrail tests to keep crate boundaries stable.

## [0.23.0] - 2026-05-05

### Added

- **Domain libraries as submodules** - Added `domain-libraries` and `examples` as tracked submodules to keep large model content versioned independently while preserving repository reproducibility.
- **Brace-based folding fallback** - Added LSP folding support that can derive foldable regions from brace structure when AST-driven folding ranges are unavailable.
- **Incomplete analysis diagnostics** - Added semantic-diagnostic support for incomplete analysis expressions so partially authored models surface clearer feedback during edit loops.

### Changed

- **SysML release alignment** - Updated bundled SysML release references from `2026-02` to `2026-03`.
- **Parser dependency upgrade** - Updated `sysml-v2-parser` integration to `v0.9.0` and aligned dependent surfaces.
- **Environment resolution behavior** - Enhanced environment/config/data-directory resolution to better support custom installation layouts.
- **Submodule/content tracking** - Replaced committed in-repo domain-library/example snapshots with submodule pointers to current upstream content.

### Fixed

- **Semantic evaluation status accuracy** - Corrected semantic evaluation status reporting so type errors are reflected accurately in analysis results.

## [0.22.0] - 2026-04-30

### Added

- **Semantic validation expansion** - Added deeper analysis diagnostics for verification semantics, allocation conformance, objective bindings, invalid verdict values, and analysis-constraint expressions.
- **Expression evaluation improvements** - Expanded semantic expression evaluation for qualified references, local attributes, unit-aware values, requirement bodies, and analysis cases.

### Changed

- **Kernel crate and validation architecture** - Reorganized the former core package into the `crates/kernel` workspace layout, split validation and visualization internals into smaller modules, and removed the deprecated `semantic_model` compatibility layer.
- **Parser/dependency alignment** - Updated the SysML v2 parser integration and dependency setup to track the newer parser surface used by current semantic and visualization paths.
- **Release/package staging** - Added VS Code package staging support and strengthened package-layout verification for release artifacts.
- **Documentation and onboarding** - Refreshed the root, VS Code, Zed, example, and domain-library READMEs to better explain `spec42`, guide first-time users, and promote example-driven evaluation.

### Fixed

- **Analysis diagnostic accuracy** - Improved semantic graph construction around verification, package bodies, requirement bodies, analysis cases, and expression references so diagnostics are more precise and less dependent on incidental graph shape.
- **Visualization model shaping** - Hardened visualization extraction/projection behavior for semantic analysis data and current view rendering paths.

## [0.21.0] - 2026-04-24

### Added

- **Sequence View rendering pipeline** - Added end-to-end sequence-diagram extraction/projection/rendering support in the backend and VS Code visualizer, including lifelines, messages, activations, and fragment handling.
- **Software Architecture add-on workflows** - Added software workspace analysis/project-view APIs plus VS Code add-on UX for running analysis and opening dedicated software architecture visualizations.
- **Expanded domain library coverage** - Added broad new domain-library content across software, electronics, communication, and robotics layers with structured rule sets and updated documentation.
- **Webshop end-to-end examples** - Added a richer webshop sample set (`architecture`, `behavior`, `requirements`, `views`) to validate explicit-view projections and cross-file visualization behavior.

### Changed

- **Parser/runtime alignment to `sysml-v2-parser` 0.7.0** - Updated parser dependency and adapted semantic-model/view extraction paths to newer AST behavior.
- **Visualizer panel/update lifecycle hardening** - Refined panel restore/update timing, content hashing, and refresh flow across startup, restore, and workspace contexts.
- **Workspace visualization model shaping** - Improved explicit-view projection, model parameter parsing, and package/container grouping behavior for more consistent rendered diagrams.

### Fixed

- **Webshop visualization regressions** - Fixed empty/missing structure/interconnection/state/action results in cross-file explicit-view scenarios (including projection and IBD scope fallback behavior).
- **`satisfy` diagnostic false positives** - Fixed typed-member-chain resolution and diagnostic suppression for references like `instance.member` across documents.
- **Action Flow UX issues** - Fixed edge-label behavior for structural flow markers and improved click-to-source reliability/disambiguation.
- **Startup empty visualizer race** - Fixed restore/startup timing cases where the visualizer pane could stay empty with no listed views until manual reopen.

## [0.20.0] - 2026-04-21

### Added

- **Expression evaluation in semantic model** - Added a new evaluation module with unit-aware expression handling and integration coverage for referenced attributes and unit expression behavior.
- **Inlay hints support** - Added inlay-hint capability wiring in the language server surface and related test coverage.
- **Expanded visualization data surface** - Added a dedicated SysML visualization endpoint and richer DTO/model payload support for container groups and connector endpoint relationships.

### Changed

- **Parser and semantic alignment** - Upgraded `sysml-v2-parser` (through `0.6.0`) and updated semantic projection paths for part definitions/usages, activity flows, and state-machine extraction.
- **Visualizer rendering/UX architecture** - Refined view state handling, text measurement/truncation, rendering flow, and loading behavior to improve responsiveness and diagram readability.
- **Action/State view release defaults** - Promoted Action Flow and State Transition views to default-enabled visualizer views and removed experimental labeling/toggle configuration from the extension surface.
- **Workspace and explorer interaction cleanup** - Simplified model-explorer/selection synchronization paths and tightened visualizer payload handling across workspace scenarios.

### Fixed

- **Hover/type resolution robustness** - Improved type resolution behavior in hover and related semantic lookup paths for more reliable editor feedback.

## [0.19.0] - 2026-04-16

### Changed

- **`sysml-v2-parser` upgrade and host alignment** - Updated Spec42 to the newer `sysml-v2-parser` releases through `0.4.0`, aligned semantic/expression handling with the newer AST shapes, and added a shared `spec42_core::sysml_v2` re-export so hosts can use the pinned parser surface consistently.
- **Semantic graph and endpoint resolution** - Expanded semantic graph construction and member resolution around package bodies, interface definitions, part usages, and connection endpoint lookup so port-like members and local references resolve more reliably.

### Fixed

- **Port compatibility diagnostics** - Replaced raw port type-name comparison with SysML v2 style feature-compatibility checks, reducing false `port_type_mismatch` warnings when differently named ports are directionally and structurally compatible.
- **Semantic diagnostic false positives** - Reduced noisy diagnostics around `redefines`, forward-resolvable `satisfy` references, delegated/redefined ports, and syntax-only missing-semicolon guesses so valid models are less likely to receive misleading warnings.

## [0.18.1] - 2026-04-10

### Fixed

- **General View workspace loading hang** - Fixed an infinite traversal in General View scene construction that could leave the visualizer stuck on "Parsing SysML model" for larger workspace visualizations such as `apollo-11-sysml-v2`.
- **Workspace package visibility in VS Code visualizer** - Fixed workspace General View package selection so the visualizer uses workspace-scoped model metadata instead of collapsing back to the currently open file, restoring visibility of packages outside the active document.

## [0.18.0] - 2026-04-10

### Added

- **Bundled SysML standard library** - The official `sysml.library` tree from the SysML v2 Release is repacked at build time, embedded in the `spec42` binary, and materialized into the spec42 data directory when needed (metadata records `repo: "embedded"`).
- **Default library paths in LSP** - `Spec42Config` carries default library search paths from the resolved environment; the server merges host defaults with paths from `initialize` / `didChangeConfiguration` so clients can extend discovery without replacing host layout.
- **`spec42 stdlib` CLI** - Subcommands to show status, print the resolved path, and `clear-cache` to remove materialized standard-library files (re-created from the embedded copy on next use). Legacy `stdlib install` / remove flows were removed in favor of the bundled workflow.
- **Import scope checks** - The semantic model applies import-scope rules so diagnostics and analysis respect SysML v2 import visibility beyond earlier membership/import resolution.

### Changed

- **Standard library resolution order** - After explicit flags, env, config, and a valid managed install under the data directory, resolution prefers **materializing the embedded archive** before falling back to the legacy VS Code `globalStorage` path, so upgrades pick the bundled release without manual cleanup.
- **VS Code Library view** - The extension no longer downloads or manages the standard library from the UI; the Library view shows the bundled release inline and defers materialization to the server.
- **`sysml-v2-parser` upgrade** - Bumped the parser dependency and adjusted semantic graph, token range, and graph-builder paths (including state machines, part definitions/usages, requirement bodies, package bodies) for compatibility with newer AST shapes and Apollo-oriented parsing behavior.
- **Document symbols** - Normalized document symbol extraction in the semantic layer for more consistent outlines and navigation.
- **Semantic graph and import resolution** - Expanded import-resolution and relationship handling (including cross-document typing tests) and refreshed validation/diagnostics integration.

## [0.17.0] - 2026-04-09

### Added

- **Visible parser declaration support** - Added first-class support for `FeatureDecl` and `ClassifierDecl` so these package-level declarations now appear in document symbols, named-element extraction, semantic graph output, feature inspector responses, and General View diagrams.

### Changed

- **Published parser dependency** - Switched Spec42 from the old git-pinned parser dependency to the published `sysml-v2-parser` crate `0.1.0`.
- **Parser compatibility handling** - Updated semantic graph, symbol, and diagnostic/token code paths to tolerate newly introduced parser AST variants without breaking existing behavior.

## [0.16.0] - 2026-04-08

### Added

- **SysML v2 import semantics in semantic resolution** - Added shared import-membership resolution for membership imports, namespace imports, `public` re-export chains, and recursive `::**` imports across semantic linking.

### Changed

- **Import-aware editor navigation** - Updated hover and go-to-definition to resolve symbols through spec-valid import chains instead of relying only on direct local typing edges or symbol-table heuristics.
- **Import-aware semantic diagnostics** - Updated unresolved-type diagnostics to respect SysML v2 import visibility rules so valid `public import` chains resolve cleanly while private-only chains continue to warn.
- **Semantic-model backend architecture** - Introduced a reusable import-resolution layer with semantic-graph caching so future import-aware features can share one standards-aligned backend path.

## [0.15.1] - 2026-04-07

### Fixed

- **Imported library type resolution** - Resolved cross-document typing for unqualified type references introduced via `import` declarations, including KerML modeled declarations from library sources.
- **Loose-file diagnostics after library refresh** - Rebuilt semantic links for non-library documents after initialization/configuration library scans so files outside workspace roots stop reporting stale unresolved-type diagnostics.

## [0.15.0] - 2026-04-07

### Added

- **Initial Zed extension** - Added a Zed extension for SysML v2 with Tree-sitter-based language support, LSP wiring to `spec42`, and automatic download of the matching `spec42` release binary when no configured or `PATH` binary is available.
- **Zed CI and release packaging** - Added CI coverage for building the Zed extension and release packaging that publishes a Zed extension source bundle alongside the existing server archives and VS Code extension artifact.

### Changed

- **Explorer-first sidebar UX** - Upgraded the Model Explorer with richer semantic summaries/tooltips and retired the separate Feature Inspector view from the VS Code extension surface.

## [0.14.0] - 2026-04-02

### Added

- **Visualizer canvas interactions** - Added richer visualizer canvas handling, including zoom/pan improvements for diagram exploration.

### Changed

- **Frontend visualization architecture cleanup** - Removed legacy sequence view/debug-label paths, tightened enabled-view handling, and refined visualizer initialization/render flow.
- **Documentation alignment** - Updated root/development/extension docs to reflect frontend-rendered diagrams and current Spec42 feature/configuration status.
- **Configuration surface reduction** - Removed unused visualization settings and semantic-token dump setting from the VS Code extension contribution surface.
- **Model Explorer navigation UX** - Updated tree selection navigation to better reuse existing editors when opening source locations.

## [0.13.0] - 2026-04-01

### Added

- **State transition visualization** - Added a new state-transition view for visualizing state machines with states and transitions, including SVG export and dedicated test fixtures.
- **Parsing performance benchmarking** - Added Criterion benchmarks (including a `parse_scan` benchmark) and documented how to run them.

### Changed

- **SysML parser + semantic model enhancements** - Updated the `sysml-v2-parser` dependency and expanded action-definition handling in the semantic model (including parameters, perform steps, and richer connection extraction for activity diagrams).
- **Indexing performance** - Improved startup and workspace indexing performance, including graph lookup caching and additional indexing/stdlib parsing optimizations.
- **Repository hygiene** - Removed outdated output/reference files that no longer reflect the current project structure.

### Fixed

- **Visualizer UI** - Disabled the layout-direction button for the state-transition view to avoid presenting a non-applicable control.

## [0.12.0] - 2026-03-31

### Changed

- **CI reliability** - Removed integration tests that depended on an external `C:\Git\sysml-examples\office\office.sysml` fixture path.
- **VS Code extension tests** - Removed a brittle interconnection SVG route-analysis test that depended on webview render/export timing and exact SVG geometry.

## [0.11.0] - 2026-03-30

### Added

- **Requirements relationship visibility controls** - Added `subject` relationship support in the semantic graph and General View, plus a General View toggle to show/hide requirement-related nodes and edges (`subject`, `satisfy`, `verify`) with default ON behavior.
- **Workspace visualization coverage** - Added/expanded integration coverage for multi-file workspace visualization flows and all-package graph expectations.
- **Production troubleshooting switch** - Added `spec42.logging.verbose` so production installs stay quieter by default while enabling deeper runtime logs on demand.

### Changed

- **spec42-core modularization (technical debt reduction)** - Split large hotspot modules into smaller focused units without changing external behavior:
  - `semantic_model/mod.rs` extracted hover/signature and symbol-entry logic.
  - `sysml_model.rs` extracted request-param parsing and graph projection helpers.
  - `semantic_checks.rs` extracted rule helper functions.
  - `lsp_server.rs` began service/helper extraction for qualified symbol lookup flows.
  - `semantic_model/graph_builder.rs` extracted requirement-subject edge helper logic.
- **General + Interconnection visualization data shaping** - Improved workspace/model graph handling (including synthetic-node stripping and root/selection behavior) to better align rendered views with real package/model structure.
- **Startup and indexing performance** - Improved startup traceability and introduced parallel parsing for scanned entries to reduce indexing bottlenecks in larger workspaces.
- **Library UX updates** - Refined standard/custom library management UX and model-explorer package statistics behavior.

### Fixed

- **Click-to-source regression in General View** - Restored reliable node click behavior and source navigation in workspace visualization scenarios.
- **Diagnostic quality improvements** - Reduced invalid semantic diagnostics around declared type references and built-in type handling (for example `String`), with better diagnostic anchoring.
- **Visualizer command icon consistency** - Updated the visualizer entry icon to `open-preview` for clearer VS Code affordance.

## [0.10.0] - 2026-03-27

### Changed

- **macOS Apple Silicon startup compatibility** - Added `darwin-arm64` server artifact build and VSIX packaging so the language server starts natively on Apple Silicon without architecture mismatch spawn failures.
- **Release packaging validation** - Updated package layout verification to require bundled `darwin-arm64` server binaries in both staged layout checks and VSIX content checks.

## [0.9.1] - 2026-03-27

### Added

- **Untyped part quick fix** - Added a Quick Fix for untyped part usage (for example `part display;`) that can create a matching `part def Display { }` and rewrite usage to `part display : Display;`.
- **Quick-fix diagnostics coverage** - Added unit and integration tests for the new untyped-part diagnostics and code-action flow.

### Changed

- **Parser update** - Updated `sysml-v2-parser` and aligned parser references across `spec42-core` and `server`.
- **Model Explorer auto-refresh** - Added debounced Model Explorer refresh/reload on SysML/KerML save and file-system changes, including workspace-aware re-scan on create/delete.
- **Diagnostic clarity** - Suppressed cryptic empty-type unresolved warnings by ignoring empty declared type references in semantic typing checks.

## [0.9.0] - 2026-03-27

### Added

- **Semantic diagnostics hardening** - Added new semantic diagnostics for invalid multiplicity intervals (`invalid_multiplicity`) and unresolved declared type references (`unresolved_type_reference`), plus guardrails for invalid/self-referential `redefines` metadata (`invalid_redefines_reference`) when available in the semantic graph.
- **Requirements slice hardening** - Added requirement usage typing resolution to requirement definitions (including cross-file linking) and added explicit satisfy-resolution diagnostics (`unresolved_satisfy_source`, `unresolved_satisfy_target`).
- **Edit-loop diagnostics coverage** - Added integration tests covering invalid intermediate edits that later become valid, and semantic diagnostics for unresolved type references.
- **Tiered CI validation workflow** - Added an informational `.github/workflows/full-validation.yml` workflow that runs SysML release workspace validation on PR/schedule/manual dispatch.
- **Requirements slice fixtures/tests** - Added focused requirement fixtures and deterministic integration tests for same-file/cross-file requirement typing and unresolved satisfy diagnostics.

### Changed

- **Required CI fast path** - Updated `.github/workflows/ci.yml` to explicitly run required fast Rust checks (`cargo test --workspace`, `cargo clippy --workspace --all-targets`).
- **CI requirements visibility** - Added explicit requirements-slice integration test invocations to the fast CI workflow for clearer release gating signal.
- **Developer guidance** - Expanded `DEVELOPMENT.md` with semantic diagnostics pipeline/codes and clarified fast-vs-full CI validation strategy.
- **Technical debt hardening** - Added release preflight checks and SHA256 release asset checksums in `.github/workflows/release.yml`, and expanded required extension CI to include the interconnection test suite.
- **Docs and metadata alignment** - Updated `DEVELOPMENT.md` local validation steps to match current repo workflows and aligned extension experimental-view wording in `vscode/package.json`.
- **Parser reproducibility** - Pinned `sysml-v2-parser` in `spec42-core/Cargo.toml` and documented parser update policy in `DEVELOPMENT.md`.
- **VS Code extension packaging** - Updated extension packaging to include runtime dependencies required at activation time and tightened packaging excludes to avoid shipping unnecessary development/test artifacts in the VSIX.

## [0.8.0] - 2026-03-27

### Added

- **Library experience and discoverability** - Added richer library search DTOs and surfaced stronger library status/search flows in the VS Code extension, with improved support for workspace + managed library usage.
- **LSP feature coverage** - Added type hierarchy support and completed additional remaining LSP capabilities, with integration-test coverage updates to keep behavior stable.

### Changed

- **Server architecture and diagnostics** - Refactored key parts of the LSP server and model-explorer context handling, improved code lens generation, and integrated tracing-based logging to make troubleshooting and diagnostics clearer.
- **Parser and dependency updates** - Updated parser/dependency stack and related tests to improve reliability and maintainability.
- **Documentation and marketplace metadata** - Refreshed README badges/screenshots/instructions and corrected the VS Code Marketplace badge link.

## [0.7.0] - 2026-03-25

### Changed

- **Diagram quality and UX** - Improved diagram behavior across General View and Interconnection View, including stronger connector handling for typed/interface-based connections, backend-root-driven interconnection filtering, fit-to-window defaults, and related visualizer usability updates.

## [0.6.0] - 2026-03-23

### Changed

- **Interconnection visualizer layout and routing** - Improved IBD layout quality and connector routing reliability for denser models, including clearer root selection behavior and more readable exported diagrams.
- **Diagnostics quality** - Improved diagnostics stability and clarity in the VS Code workflow, with better behavior during active edit/update cycles.

## [0.5.0] - 2026-03-13

### Added

- **Interconnection View release enablement** - Promoted `interconnection-view` from experimental opt-in to a release-enabled default view with CI-backed export coverage.
- **Multi-workspace VS Code test fixtures** - Added stronger fixture coverage for single-file, multi-file, large-workspace, and timer-based visualization scenarios.

### Changed

- **Visualization UX** - Improved interconnection root selection, root summaries, port direction legend support, and clearer no-connector guidance in the visualizer.
- **Source-to-diagram behavior** - Editor-driven diagram selection now reveals and centers matching elements more reliably when available.
- **VS Code test infrastructure** - Standardized cross-platform test workspace `serverPath` configuration so Windows and Linux test hosts boot the language server consistently.

### Fixed

- **Crash and edit-loop robustness** - Hardened incremental edit handling, malformed range handling, repeated open/edit/close lifecycles, and semantic token refresh after edits.
- **VS Code integration stability** - Fixed startup and restart issues in test and CI environments, and made extension-host tests deterministic again.
- **Definition and visualization regressions** - Stabilized go-to-definition test coverage and visualizer export checks across representative fixtures.

## [0.4.0] - 2026-03-12

### Changed

- **New sysml-v2-parser** - Switched to a new sysml-v2-parser dependency for improved parsing and alignment with SysML v2.

## [0.3.0] - 2026-03-10

### Added

- **General View diagram** - New diagram view showing the model structure with element hierarchy, attributes, ports, and parts. Nodes use standard SysML-style compartments.

## [0.2.2] - 2026-03-06

### Changed

- VS Code extension display name updated to "SysML v2 Language Support" to meet marketplace requirements and reduce name confusion with other language server extensions.

## [0.2.1] - 2026-03-06

### Fixed

- **UTF-8 / multi-byte handling:** `position_to_byte_offset` now correctly converts LSP character indices to byte offsets (e.g. for "cafe"). `completion_prefix` iterates by character to avoid panics on multi-byte content. Error masking in `parse_sysml_collect_errors` uses character boundaries so multi-byte lines no longer produce invalid UTF-8.
- **Parse error messages:** Low-level Pest messages are mapped to clearer user-facing text (e.g. "expected metadata_annotation" -> "unexpected token; perhaps missing an attribute or expression"); original message is appended for debugging. Additional mappings for package, member, name, identifier, import, expressions, literals, parentheses, etc.

### Changed

- Removed unused `line_char_to_byte_offset` from kerml-parser (server already has equivalent logic). Extended `improve_pest_error_message` with more grammar-rule mappings.

### Added

- Unit tests for multi-byte edge cases: `position_to_byte_offset`, `word_at_position`, `completion_prefix`, and `parse_sysml_collect_errors` with UTF-8 in the error region.

## [0.2.0] - 2025-03-06

### Added

- Calc def result expressions: parser now supports bare result expressions (e.g. `capacity / currentDraw`) without a final semicolon, per SysML v2 7.19.2.
- Full validation suite test: parses all `.sysml` files in SysML-v2-Release `sysml/src/validation`; test is `#[ignore]`d (run with `cargo test -p kerml-parser -- --ignored`); CI runs it with `--include-ignored`. Per-file summary (pkgs, members, lines) logged when running the suite.

### Fixed

- "position" no longer incorrectly marked as keyword in semantic tokens; it is a contextual keyword only and valid as an identifier (e.g. `out position : String`).
- Shared reserved keyword list: single source of truth in `language::RESERVED_KEYWORDS` for semantic token fallback and goto-definition/rename suppression; eliminates discrepancies between keyword lists.

## [0.1.0] - 2026-03-05

### Added

- LSP over stdio: text sync, diagnostics, hover, completion, go to definition, find references, rename, document symbols, workspace symbol search, code actions, formatting.
- Workspace-aware indexing for `.sysml` and `.kerml` files (workspace folders and root URI).
- VS Code extension with SysML/KerML syntax highlighting and language configuration.
- Parser aligned with the [SysML v2 Release](https://github.com/Systems-Modeling/SysML-v2-Release) validation suite.

### Known limitations

- Parser is aligned with the SysML v2 Release validation suite; it does not claim full OMG spec compliance.
- Some constructs may have incomplete semantic token or outline coverage.

[0.33.0]: https://github.com/elan8/spec42/releases/tag/v0.33.0
[0.32.0]: https://github.com/elan8/spec42/releases/tag/v0.32.0
[0.31.0]: https://github.com/elan8/spec42/releases/tag/v0.31.0
[0.30.0]: https://github.com/elan8/spec42/releases/tag/v0.30.0
[0.29.1]: https://github.com/elan8/spec42/releases/tag/v0.29.1
[0.29.0]: https://github.com/elan8/spec42/releases/tag/v0.29.0
[0.28.0]: https://github.com/elan8/spec42/releases/tag/v0.28.0
[0.27.1]: https://github.com/elan8/spec42/releases/tag/v0.27.1
[0.27.0]: https://github.com/elan8/spec42/releases/tag/v0.27.0
[0.26.3]: https://github.com/elan8/spec42/releases/tag/v0.26.3
[0.26.2]: https://github.com/elan8/spec42/releases/tag/v0.26.2
[0.26.1]: https://github.com/elan8/spec42/releases/tag/v0.26.1
[0.26.0]: https://github.com/elan8/spec42/releases/tag/v0.26.0
[0.25.0]: https://github.com/elan8/spec42/releases/tag/v0.25.0
[0.24.0]: https://github.com/elan8/spec42/releases/tag/v0.24.0
[0.23.0]: https://github.com/elan8/spec42/releases/tag/v0.23.0
[0.22.0]: https://github.com/elan8/spec42/releases/tag/v0.22.0
[0.21.0]: https://github.com/elan8/spec42/releases/tag/v0.21.0
[0.20.0]: https://github.com/elan8/spec42/releases/tag/v0.20.0
[0.19.0]: https://github.com/elan8/spec42/releases/tag/v0.19.0
[0.18.1]: https://github.com/elan8/spec42/releases/tag/v0.18.1
[0.18.0]: https://github.com/elan8/spec42/releases/tag/v0.18.0
[0.17.0]: https://github.com/elan8/spec42/releases/tag/v0.17.0
[0.15.1]: https://github.com/elan8/spec42/releases/tag/v0.15.1
[0.11.0]: https://github.com/elan8/spec42/releases/tag/v0.11.0
[0.15.0]: https://github.com/elan8/spec42/releases/tag/v0.15.0
[0.14.0]: https://github.com/elan8/spec42/releases/tag/v0.14.0
[0.13.0]: https://github.com/elan8/spec42/releases/tag/v0.13.0
[0.12.0]: https://github.com/elan8/spec42/releases/tag/v0.12.0
[0.10.0]: https://github.com/elan8/spec42/releases/tag/v0.10.0
[0.9.1]: https://github.com/elan8/spec42/releases/tag/v0.9.1
[0.9.0]: https://github.com/elan8/spec42/releases/tag/v0.9.0
[0.8.0]: https://github.com/elan8/spec42/releases/tag/v0.8.0
[0.7.0]: https://github.com/elan8/spec42/releases/tag/v0.7.0
[0.6.0]: https://github.com/elan8/spec42/releases/tag/v0.6.0
[0.5.0]: https://github.com/elan8/spec42/releases/tag/v0.5.0
[0.4.0]: https://github.com/elan8/spec42/releases/tag/v0.4.0
[0.3.0]: https://github.com/elan8/spec42/releases/tag/v0.3.0
[0.2.2]: https://github.com/elan8/spec42/releases/tag/v0.2.2
[0.2.1]: https://github.com/elan8/spec42/releases/tag/v0.2.1
[0.2.0]: https://github.com/elan8/spec42/releases/tag/v0.2.0
[0.1.0]: https://github.com/elan8/spec42/releases/tag/v0.1.0
