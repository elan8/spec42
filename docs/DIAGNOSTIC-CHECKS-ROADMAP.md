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
- `unresolved_specializes_reference`: specializes target does not resolve.
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
- Remaining: range quality audit; unresolved type/import/relation diagnostics should point at the offending reference token rather than the containing declaration where possible.

### P1: names, namespaces, imports

Spec areas: 7.5, 8.3.5, 8.4.2.

- Ambiguous unqualified name: a reference resolves to multiple visible members.
- Duplicate visible member name in the same namespace, excluding valid alias cases.
- Invalid qualified-name path: an intermediate segment is not a namespace.
- Import kind mismatch: namespace import targets a non-namespace, membership import targets a namespace when a wildcard was likely intended.
- Recursive import sanity: unsupported or malformed recursive import patterns after parsing.
- Private/protected/public visibility violation across namespaces.
- Import filter validation: filter expression must be Boolean and refer to visible metadata/properties.

### P1: typing, specialization, subsetting, redefinition

Spec areas: 7.6, 8.2.2.6, 8.4.2.

- Type kind compatibility: a `part` should type to a part/occurrence-compatible definition, `port` to a port-compatible definition, `item` to item-compatible, etc.
- Definition specialization kind compatibility: definitions should specialize compatible definition kinds.
- Usage subsetting/redefinition kind compatibility: a usage should subset/redefine compatible feature kinds.
- Redefines target resolution: resolve and validate non-empty redefines targets beyond self-reference.
- Redefinition multiplicity conformance: redefining/subsetting feature must not loosen inherited multiplicity.
- Redefinition type conformance: redefining feature type must conform to inherited feature type.
- Cycle detection: direct or indirect specialization/redefinition/subsetting cycles.

### P1: expressions, values, units, and multiplicity

Spec areas: 7.7, 7.8, 7.19, 7.20, 8.4.3, 8.4.4, 8.4.15, 8.4.16, 9.2.

- Attribute value type compatibility for scalar literals, enum values, booleans, strings, numerics, and references.
- Enumeration value validation: enum-typed attributes should use a declared enumeration value.
- Unit compatibility: quantity/unit dimensions should be compatible in assignments and calculations.
- Boolean expression requirement: constraints, asserts, import filters, and guards should evaluate to Boolean.
- Calculation parameter/result binding: invocation arguments should match parameter count, direction, and type.
- Multiplicity lower/upper conformance across subsetting/redefinition, not only local syntax.

### P1: ports, connections, interfaces, and flows

Spec areas: 7.12-7.16, 8.3.12-8.3.16, 8.4.8-8.4.12.

- Connection endpoint resolution through feature chains with precise diagnostics for the first unresolved segment.
- Connection endpoint ownership/context: endpoints should be connectable in the containing structural context.
- Binding connector compatibility: bound ends should have compatible value/type semantics, not just port-like shape.
- Interface end validation: interface ends should map to compatible port/features.
- Flow direction compatibility, including conjugated ports.
- Flow item type compatibility between source and target.
- Conjugated port typing consistency.

### P2: actions, states, and behavior

Spec areas: 7.17-7.18, 8.3.17-8.3.18, 8.4.13-8.4.14.

- Perform action target must resolve to an action definition/usage.
- Send/accept payload compatibility.
- Assignment action target must be assignable and value-compatible.
- Transition source/target must resolve to states in the same state context.
- Guards should be Boolean.
- Initial/final state sanity within a state definition.
- Succession endpoint validation for action and state flows.

### P2: requirements, cases, verification, use cases

Spec areas: 7.21-7.25, 8.3.21-8.3.25, 8.4.17-8.4.21.

- Requirement subject resolution and kind compatibility.
- Requirement constraint membership kind validation.
- `satisfy` source/target kind compatibility beyond unresolved/usage-preferred checks.
- Verification case objective/requirement membership resolution.
- Verification method/result/verdict shape validation.
- Use case include target resolution and kind validation.
- Case subject/objective binding cardinality and type checks.

### P2: views, viewpoints, renderings, metadata

Spec areas: 7.26-7.27, 8.3.26, 8.4.22-8.4.23.

- View rendering membership target kind validation.
- View expose/import filter expression validation.
- Viewpoint concern/stakeholder/language/purpose references should resolve where applicable.
- Metadata annotation target resolution.
- Metadata definition/usage kind compatibility.
- User-defined keyword collision or unresolved metadata definition.

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
