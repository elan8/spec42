# Diagnostic checks roadmap

This document inventories the diagnostic checks currently surfaced by Spec42 and
captures candidate generic SysML v2 checks to add next. Domain-specific checks
are intentionally out of scope here.

Primary local reference used for the candidate list:
`C:\Git\elan8-monorepo\library\SysML_v2.txt`.

## Current diagnostic pipeline

Diagnostics are currently assembled in `crates/kernel/src/analysis/diagnostics_core.rs`:

1. Syntax diagnostics from `sysml-v2-parser::parse_with_diagnostics`.
2. A lightweight textual `untyped_part_usage` hint.
3. Semantic graph checks from `semantic_core::collect_diagnostics_from_graph`, only when there is no parser error.
4. A `missing_library_context` hint when unresolved references likely come from missing library configuration.
5. LSP/CLI postprocessing, including startup suppression of transient unresolved reference diagnostics.

The semantic checks are mostly implemented in
`crates/semantic_core/src/semantic/diagnostics/engine_impl.rs`.

## Current checks

### Parser / syntax

Owned by `sysml-v2-parser` and surfaced as source `sysml`.

- Generic parse errors and warnings with parser-owned codes/messages.
- Concrete syntax coverage depends on the parser grammar and recovery behavior.
- Known limitation: some invalid partial inputs can parse partially and may not always produce useful diagnostics.

### Lightweight text checks

- `untyped_part_usage`: reports a `part name;` usage with no declared type. This is informational and currently implemented as a textual heuristic.
- `missing_library_context`: reports unresolved library-like references when no library paths are configured or indexed.

### Name and reference resolution

- `unresolved_type_reference`: usage or feature type reference does not resolve.
- `unresolved_ref_type_reference`: `ref` type reference does not resolve.
- `unresolved_import_target`: import target does not resolve to a known namespace/member.
- `unresolved_specializes_reference`: specializes target does not resolve (includes `analysis def`, `verification def`, `metadata def`, and other case kinds via `SPECIALIZES_TARGET_KINDS`).
- `unresolved_pending_relationship`: deferred cross-document relationship did not resolve after graph construction.
- `unresolved_pending_expression_relationship`: deferred expression relationship did not resolve after graph construction.

### Connections, ports, and multiplicity

- `connection_endpoint_not_port`: connect endpoint is not port-like, except for supported part-like connection cases.
- `port_type_mismatch`: connected ports have incompatible types.
- `unconnected_port`: declared port usage is not connected.
- `duplicate_connection`: duplicate textual connection endpoints.
- `invalid_multiplicity`: malformed or inconsistent multiplicity bounds.

### Redefinition and value typing

- `invalid_redefines_reference`: empty or self-referential redefines target.
- `implicit_redefinition_without_operator`: inherited feature receives a value without explicit `:>>` / redefinition.
- `inherited_attribute_value_type_mismatch`: inherited enum-typed attribute is assigned an incompatible string literal.

### Relationships and viewpoints

- `unresolved_allocate_source`
- `unresolved_allocate_target`
- `allocate_endpoint_prefers_usage`
- `allocation_type_not_allocation_def`
- `invalid_allocation_endpoints`
- `unresolved_satisfy_source`
- `unresolved_satisfy_target`
- `satisfy_endpoint_prefers_usage`
- `unresolved_viewpoint_conformance_target`
- `viewpoint_conformance_invalid_target_kind`

Some relationship diagnostics are emitted by graph-builder diagnostic nodes and then refined or suppressed after workspace resolution.

### Analysis, constraints, and cases

- `analysis_constraint_failed`
- `analysis_evaluation_incomplete`
- `analysis_evaluation_unresolved`
- `invalid_verdict_value`
- `objective_binding_unresolved`

**Expression evaluation (June 2026):** `semantic_core::evaluation` now propagates typed
`analysis` / `verification` usage context after workspace linking, aggregates `assert
constraint` bodies onto owners, evaluates `require constraint` on requirement defs and
usages (including `[MW]`-style unit literals via the embedded engineering unit catalog),
and rolls up `sum(child.feature)` across sibling `part` usages when no explicit
collection binding exists. `constraint def` / `calc def` remain templates (invocation
only). KerML `if` / `let` / lambda in constraints remain deferred.

### Catalog coverage

`crates/server/src/mcp/diagnostic_catalog.rs` is the current user-facing catalog for `spec42 explain-diagnostic` and the MCP diagnostic explanation surface.

P0 coverage now includes:

- catalog entries for all known emitted Spec42 semantic/sysml diagnostic codes
- a uniqueness test for catalog codes
- a guard test that checks known emitted codes have catalog entries with current emitted severities

## Candidate generic SysML v2 checks

The references below point to sections in `SysML_v2.txt` rather than quoting normative text.

### P0: make existing diagnostics more reliable

- Done: diagnostic catalog completeness for known emitted Spec42 codes, with guard tests.
- Done: severity alignment between the catalog and current emitted severities.
- Done: parser diagnostic audit fixture for common malformed inputs, requiring stable codes and LSP ranges.
- Done: duplicate/cascade suppression now includes unresolved allocate/satisfy/viewpoint relationship diagnostics after parser errors.
- Done: range quality audit for file-backed sources; unresolved type/import/relation diagnostics prefer the offending reference token rather than the containing declaration when source text is available. Memory/custom URI sources still fall back to semantic node ranges unless token ranges are provided by the graph builder.

### P1: names, namespaces, imports

Spec areas: 7.5, 8.3.5, 8.4.2.

- Done: `ambiguous_name_reference` — unqualified reference is ambiguous only when type/specializes resolution fails.
- Done: `duplicate_namespace_member` — duplicate visible member names in a namespace (counts by `(name, element_kind)` so subject vs `then action` reuse is allowed).
- Done: `invalid_qualified_name_segment` — intermediate qualified-name segment is not a namespace.
- Done: `import_kind_mismatch` — namespace vs membership import target mismatch.
- Done: `invalid_recursive_import` — recursive import targets a non-namespace.
- Removed: `visibility_violation` — previously warned on all `private import …::*`; private wildcard imports are valid for internal use in SysML v2.
- Done: `invalid_import_filter` — import filter expression is not Boolean-valued.

### P1: typing, specialization, subsetting, redefinition

Spec areas: 7.6, 8.2.2.6, 8.4.2.

- Done: `incompatible_type_kind` — usage typed by incompatible definition kind.
- Done: `incompatible_specializes_kind` — definition specializes incompatible base kind.
- Done: `incompatible_subset_redefine_kind` — subset/redefine target kind mismatch.
- Done: `unresolved_redefines_target` — redefines target does not resolve on specializing owners.
- Done: `redefinition_multiplicity_widened` — redefinition loosens inherited multiplicity.
- Done: `redefinition_type_incompatible` — redefinition type/value not conformant with inherited feature.
- Done: `specialization_cycle` — specializes chain contains a cycle.

### P1: expressions, values, units, and multiplicity

Spec areas: 7.7, 7.8, 7.19, 7.20, 8.4.3, 8.4.4, 8.4.15, 8.4.16, 9.2.

- Done: `attribute_value_type_mismatch` — scalar literal incompatible with declared attribute type.
- Done: `invalid_enumeration_value` — string literal not declared on enum type.
- Done: `unknown_unit_symbol` — unit suffix not in indexed quantity/unit catalogs.
- Done: `incompatible_unit_dimension` — recognized unit suffix incompatible with attribute quantity type dimension.
- Done: `non_boolean_expression` — constraint/assert/filter not Boolean-typed (when evaluation reports it).
- Done: `calculation_binding_mismatch` — invocation arity below declared parameter count (when graph captures arg count).
- Done: `redefinition_multiplicity_widened` — multiplicity conformance across redefinition (see typing section).

### P1: ports, connections, interfaces, and flows

Spec areas: 7.12-7.16, 8.3.12-8.3.16, 8.4.8-8.4.12.

- Done: `ambiguous_connection_endpoint` — cataloged (emitted by graph builder).
- Done: `unresolved_connection_segment` — first unresolved segment in pending connect endpoint chains.
- Done: `connection_context_invalid` — endpoints not connectable in structural context.
- Done: `binding_connector_incompatible` — binding ends with incompatible value types.
- Done: `interface_end_invalid` — interface end missing/empty port type.
- Done: `flow_direction_incompatible` — port feature direction mismatch (extends port compatibility).
- Done: `flow_item_type_incompatible` — incompatible port definition pairing.
- Done: `conjugated_port_inconsistent` — both connected ports share conjugation.

### P2: actions, states, and behavior

Spec areas: 7.17-7.18, 8.3.17-8.3.18, 8.4.13-8.4.14.

- Done: `perform_target_invalid_kind` — perform relationship target must be action-like.
- Done: `accept_payload_incompatible` / `send_payload_incompatible` — when payload type is on the graph and resolves to an incompatible kind.
- Done: `assignment_target_unresolved` — verification assign lhs must resolve.
- Done: `assignment_value_incompatible` — verification assign rhs literal incompatible with resolved lhs scalar type.
- Done: `transition_endpoint_invalid_state` — transition endpoints must resolve to state usages.
- Done: `transition_endpoint_invalid_context` — transition endpoints must share a state-def context.
- Done: `transition_guard_non_boolean` — transition guard expression must be Boolean-valued.
- Done: `initial_state_invalid_target` — initial transition target must be a state usage.
- Done: `multiple_initial_states` / `missing_initial_state` — initial transition cardinality on state definitions.
- Done: `multiple_final_states` — more than one `final` state in a state definition (parser 0.19.0 + graph `final state` nodes).
- Done: `missing_final_state` — state definition with state usages but no `final` state.
- Done: transition `accept_payload_incompatible` when typed `Transition.accept` payload is on the graph.
- Done: transition/view/import filter Boolean checks use AST `conditionIsBoolean` when graph-built (fallback string heuristic).
- Done: `succession_endpoint_invalid` — behavior flow endpoints must be action-like.

### P2: requirements, cases, verification, use cases

Spec areas: 7.21-7.25, 8.3.21-8.3.25, 8.4.17-8.4.21.

- Done: subject typing via extended `declared_type_ref` + `incompatible_type_kind` for `subject` nodes.
- Done: `requirement_constraint_invalid_membership` — require constraint parameter direction/type and expression body.
- Done: `satisfy_invalid_endpoint_kind` — requirement/use-case satisfy kind compatibility (view→viewpoint still uses viewpoint conformance checks).
- Done: `verified_requirement_invalid_target` — verification verified-requirement membership resolution.
- Done: `verification_case_invalid_shape` — multiple verdict/return clauses, or verified requirements without objective. Does **not** warn on `then action` without explicit `return` (valid per SysML v2 `CaseBody`; `ResultExpressionMember` is optional — see S42-LIM-003 / robot-vacuum showcase).
- Done: `use_case_include_invalid_target` — include use case target resolution and kind validation.
- Done: `case_subject_missing` / `case_objective_binding_cardinality` — subject and analysis-result binding cardinality.

### P2: views, viewpoints, renderings, metadata

Spec areas: 7.26-7.27, 8.3.26, 8.4.22-8.4.23.

- Done: `view_rendering_invalid_target` — view rendering membership target kind validation.
- Done: `view_expose_empty` — view body without expose members.
- Done: `view_expose_unresolved` — expose target/feature chain does not resolve.
- Done: `view_expose_empty_result` — reserved for filters removing all exposed elements (catalog entry; emit when filter-aware evaluation is wired in diagnostics).
- Done: `view_filter_non_boolean` — view body filter expression Boolean validation.
- Done: `viewpoint_reference_unresolved` / `viewpoint_rep_language_unresolved` — viewpoint frame/import/stakeholder/purpose targets and missing rep language.
- Done: `metadata_annotation_unresolved` — metadata annotation target resolution when untyped.
- Done: metadata/rendering/subject typing via extended `kind_rules` + `incompatible_type_kind`.
- Done: `metadata_keyword_collision` — duplicate metadata def short names in one document.
- Done: `metadata_keyword_unresolved` — `#keyword` usages on the graph (`metadata keyword` nodes) and user-defined `feature decl` / `classifier decl` keywords.

## Robot vacuum showcase regressions (June 2026)

External validation report: `C:\Git\sysml-robot-vacuum-cleaner\docs\SPEC42_VALIDATION_REPORT.md`.

Tracked limitations (`S42-LIM-*`) addressed in this cycle:

- `S42-LIM-001`: cross-package `verify requirement` resolves via imports (`requirement_body.rs`, `relationships.rs`).
- `S42-LIM-002`: `then done` succession accepts `verdict` endpoints (`behavior_conformance.rs`).
- `S42-LIM-004`: `VerdictKind::pass` evaluates to `analysisEvaluationStatus = ok` (`evaluation/mod.rs`).
- `S42-LIM-007`: named `transition … first source then target` no longer counts as initial (`sysml-v2-parser` + `state.rs`).
- `S42-LIM-008`: cyclic state machines suppress `missing_final_state` guidance (`behavior_conformance.rs`).
- `S42-LIM-009`: bundled `MonetaryUnits` indexed for `[EUR]` (`evaluation/units.rs`).
- `S42-LIM-010`: remove implicit redefines heuristic false positives (`kind_compatibility.rs`).

Optional env-gated integration baseline: `SYSML_ROBOT_VACUUM_DIR` → `robot_vacuum_baseline.rs`.

Done: `S42-LIM-005` — generic `FlowUsage` in parser and semantic graph (`flow` / `message` / `succession flow` in structure-usage bodies including part def/usage, package, occurrence def, action, use case).

## Suggested implementation order

1. Normalize diagnostic metadata first: catalog completeness, severity alignment, tests that emitted codes are cataloged.
2. Improve reference diagnostics: ambiguous names, invalid qualified-name segments, better unresolved ranges.
3. Add kind-compatibility checks for typing/specialization/redefinition. These will make everyday modeling mistakes much more visible.
4. Deepen connection/interface/flow checks because they directly improve the visualizer and structural modeling workflow.
5. Add expression/value/unit checks incrementally as the expression model becomes stronger.
6. Add behavior/case/view/metadata checks after the core name/type system is dependable.

## Notes

- Keep checks generic SysML v2 unless a domain library explicitly owns the rule.
- Prefer checks over the semantic graph when possible, but use parser AST/ranges for token-precise diagnostics.
- Each new diagnostic should include a fixture, LSP/CLI coverage, catalog entry, and quick-fix metadata when the fix is mechanical.

## Spec alignment notes (June 2026)

Fixes applied against `SysML_v2.txt` after drone-example and vacuum-corpus audits:

### Kind compatibility (`incompatible_type_kind`)

| Usage kind | Spec rule | Fix |
|---|---|---|
| `actor`, `stakeholder` | `ActorUsage` / `StakeholderUsage` are `PartUsage`; typed by part or item definitions (§7.11.2, §7.21.2, §7.22.2) | Allow `part def`, `item def`, `occurrence def`; removed non-spec `actor def` target |
| `part` | Part usages may use item definitions (§7.11.2) | Added `item def` |
| `item` | Item usages may use part definitions (part defs are item defs) | Added `part def` |
| `subject`, `ref` | `ReferenceUsage` may reference any Classifier (§8.3.6.3) | Skip kind check when typing resolves |

### Boolean filter expressions

| Code | Spec rule | Fix |
|---|---|---|
| `view_filter_non_boolean`, `invalid_import_filter` | Filter conditions are Boolean-valued; `@SysML::Metaclass` classification is valid (§7.5.4, §7.26.2) | Treat `@`-prefixed refs and logical combinations as Boolean; dedupe view vs package filter ownership |

### Regression coverage

- `crates/semantic_core/tests/drone_diagnostics.rs` — drone actor + view filter fixtures
- `crates/semantic_core/tests/p1_diagnostics.rs` — actor/part/subject typing
- `crates/semantic_core/tests/p2_diagnostics_semantics.rs` — metaclass filter expressions

### Remaining limitations (deferred)

- KerML classifier targets in kind tables (`kermlDecl`) — add when library typing is indexed
- Internal graph kind `"actor def"` from legacy package-body path — not a SysML v2 construct; quarantined from typing checks
- Full KerML `OwnedExpression` (`if`, `let`, lambda) — incremental tranches only; `@Metaclass` / `istype` / `hastype` / `as` covered in parser **0.23.0** with Spec42 `exprClass` + AST boolean walk

### Shipped in parser 0.23.0 (no longer deferred)

- `Expression::Classification` for `@SysML::Metaclass` filters; `TypeCheck` for `istype` / `hastype` / `as`
- Typed `stakeholder name : Type` in requirement/viewpoint bodies
- Filter/guard Boolean checks use graph `conditionIsBoolean` + `exprClass` from AST walk (not string re-parse)

### Catalog audit (other codes)

Spot-checked against spec examples and the MBSE vacuum corpus. No additional systematic false positives found beyond the fixes above. Connection, port, flow, import-resolution, and state-machine hint diagnostics remain as documented in `MBSE-VACUUM-CHECK-ANALYSIS.md`.
