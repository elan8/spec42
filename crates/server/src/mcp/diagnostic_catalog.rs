use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticCatalogEntry {
    pub code: &'static str,
    pub severity: &'static str,
    pub meaning: &'static str,
    pub typical_fix: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor_quick_fixes: Option<&'static [&'static str]>,
}

const CATALOG: &[DiagnosticCatalogEntry] = &[
    DiagnosticCatalogEntry {
        code: "unresolved_type_reference",
        severity: "warning",
        meaning: "A type name on a usage or feature does not resolve to a known definition in the workspace or libraries.",
        typical_fix: "Add or import the missing definition, fix the qualified name, or configure library paths / standard library.",
        editor_quick_fixes: Some(&[
            "create_matching_part_def",
            "create_definition_for_unresolved_type",
        ]),
    },
    DiagnosticCatalogEntry {
        code: "unresolved_ref_type_reference",
        severity: "warning",
        meaning: "A type referenced after `ref` does not resolve.",
        typical_fix: "Ensure the referenced type exists and is visible via imports or namespace.",
        editor_quick_fixes: Some(&["create_definition_for_unresolved_type"]),
    },
    DiagnosticCatalogEntry {
        code: "unresolved_import_target",
        severity: "warning",
        meaning: "An import statement targets a package or namespace that cannot be found.",
        typical_fix: "Fix the import path, add the defining file to the workspace, or index the library root.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_specializes_reference",
        severity: "warning",
        meaning: "A specializes target does not resolve to a known definition.",
        typical_fix: "Correct the specializes clause or add the base definition.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "implicit_redefinition_without_operator",
        severity: "error",
        meaning: "An inherited feature is redefined without an explicit redefinition operator (`:>>` / `redefines`).",
        typical_fix: "Add an explicit redefines clause on the redefining feature.",
        editor_quick_fixes: Some(&["explicit_redefinition_quick_fix"]),
    },
    DiagnosticCatalogEntry {
        code: "inherited_attribute_value_type_mismatch",
        severity: "error",
        meaning: "A redefining attribute value is not compatible with the inherited attribute typing.",
        typical_fix: "Align value expression type with the inherited attribute or adjust typing.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "untyped_part_usage",
        severity: "information",
        meaning: "A part usage has no typing clause (`: Type`).",
        typical_fix: "Add a type if the usage should be typed; otherwise this may be intentional.",
        editor_quick_fixes: Some(&["create_matching_part_def"]),
    },
    DiagnosticCatalogEntry {
        code: "missing_library_context",
        severity: "information",
        meaning: "The document imports library symbols but no SysML library paths are configured.",
        typical_fix: "Configure spec42.libraryPaths / --library-path or install the standard library.",
        editor_quick_fixes: Some(&[
            "manage_custom_libraries",
            "show_standard_library_info",
        ]),
    },
    DiagnosticCatalogEntry {
        code: "port_type_mismatch",
        severity: "warning",
        meaning: "Connected ports have incompatible port definitions or types.",
        typical_fix: "Use compatible port types or an interface that connects them.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "connection_endpoint_not_port",
        severity: "warning",
        meaning: "A connection endpoint is not a port-like feature.",
        typical_fix: "Connect port usages or adjust the connection statement.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unconnected_port",
        severity: "information",
        meaning: "A port is not connected in the current structural context.",
        typical_fix: "Add a connection or mark the port as intentionally unused.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "duplicate_connection",
        severity: "information",
        meaning: "The same connection appears more than once.",
        typical_fix: "Remove duplicate connect/bind statements.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "ambiguous_connection_endpoint",
        severity: "warning",
        meaning: "A connection endpoint expression resolves to more than one candidate.",
        typical_fix: "Use a fully qualified endpoint path to disambiguate the connection end.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invalid_multiplicity",
        severity: "warning",
        meaning: "A multiplicity clause is not valid for the usage.",
        typical_fix: "Fix multiplicity syntax or bounds.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invalid_redefines_reference",
        severity: "warning",
        meaning: "A redefines target does not resolve or is not redefinable.",
        typical_fix: "Point redefines at an existing inherited or visible feature.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_pending_relationship",
        severity: "error",
        meaning: "A cross-document relationship could not be resolved after indexing.",
        typical_fix: "Ensure both ends exist, imports are correct, and workspace indexing completed.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_allocate_source",
        severity: "warning",
        meaning: "The source of an allocate relationship does not resolve.",
        typical_fix: "Use a resolvable usage or definition as the allocate source.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_allocate_target",
        severity: "warning",
        meaning: "The target of an allocate relationship does not resolve.",
        typical_fix: "Use a resolvable usage or definition as the allocate target.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "allocate_endpoint_prefers_usage",
        severity: "warning",
        meaning: "Allocate should reference a usage rather than a bare definition.",
        typical_fix: "Point the allocate end at a part usage (or other usage) instead of a definition.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_viewpoint_conformance_target",
        severity: "warning",
        meaning: "A viewpoint conformance target does not resolve.",
        typical_fix: "Import the viewpoint/view package or fix the qualified target name.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "viewpoint_conformance_invalid_target_kind",
        severity: "warning",
        meaning: "The target of viewpoint conformance is not a viewpoint element.",
        typical_fix: "Reference a viewpoint definition or usage as required by the conformance statement.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "analysis_constraint_failed",
        severity: "warning",
        meaning: "An analysis constraint evaluated to false.",
        typical_fix: "Adjust the model or constraint expression so the analysis passes.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "objective_binding_unresolved",
        severity: "warning",
        meaning: "An analysis or verification objective binding could not be resolved.",
        typical_fix: "Ensure subject/objective identifiers match defined case elements.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_pending_expression_relationship",
        severity: "error",
        meaning: "An expression-based relationship could not be resolved after graph construction.",
        typical_fix: "Check both endpoint expressions, imports, and qualified feature paths.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "allocation_type_not_allocation_def",
        severity: "warning",
        meaning: "An allocation usage is typed by something other than an allocation definition.",
        typical_fix: "Type the allocation with an allocation definition or remove the incompatible type.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invalid_allocation_endpoints",
        severity: "warning",
        meaning: "An allocation usage declares only one endpoint of an allocate-to pair.",
        typical_fix: "Declare both source and target endpoints, or remove the incomplete allocate clause.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_satisfy_source",
        severity: "warning",
        meaning: "The source of a satisfy relationship does not resolve.",
        typical_fix: "Use a resolvable requirement usage or qualified requirement path as the satisfy source.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_satisfy_target",
        severity: "warning",
        meaning: "The target of a satisfy relationship does not resolve.",
        typical_fix: "Use a resolvable design feature or qualified feature path after `by`.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "satisfy_endpoint_prefers_usage",
        severity: "warning",
        meaning: "A satisfy endpoint resolves to a definition where a usage or feature path is expected.",
        typical_fix: "Point the endpoint at the concrete usage that satisfies the requirement.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "analysis_evaluation_incomplete",
        severity: "information",
        meaning: "An analysis expression depends on declared values that are not assigned.",
        typical_fix: "Assign the required values or accept that the analysis cannot be fully evaluated yet.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "analysis_evaluation_unresolved",
        severity: "warning",
        meaning: "An analysis expression could not be evaluated.",
        typical_fix: "Check referenced values, operators, and expression syntax supported by Spec42.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invalid_verdict_value",
        severity: "warning",
        meaning: "A verdict value is outside the SysML verdict domain.",
        typical_fix: "Use one of: pass, fail, inconclusive, or error.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "ambiguous_name_reference",
        severity: "warning",
        meaning: "A simple name resolves to multiple visible members in the current scope.",
        typical_fix: "Use a qualified name or disambiguate imports.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invalid_qualified_name_segment",
        severity: "warning",
        meaning: "An intermediate segment in a qualified name is not a namespace.",
        typical_fix: "Correct the qualified path so each prefix resolves to a namespace.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "duplicate_namespace_member",
        severity: "warning",
        meaning: "The same member name is declared more than once in one namespace.",
        typical_fix: "Rename or remove the duplicate member.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "import_kind_mismatch",
        severity: "warning",
        meaning: "An import statement uses the wrong import kind for its target.",
        typical_fix: "Use a namespace import for packages and a membership import for individual members.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invalid_recursive_import",
        severity: "warning",
        meaning: "A recursive import does not target a namespace.",
        typical_fix: "Point recursive imports at a package or namespace element.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invalid_import_filter",
        severity: "warning",
        meaning: "An import filter expression is not Boolean-valued.",
        typical_fix: "Rewrite the filter as a Boolean condition over visible metadata or properties.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "incompatible_type_kind",
        severity: "warning",
        meaning: "A usage is typed by a definition of an incompatible kind.",
        typical_fix: "Use a compatible definition kind for the usage (for example part def for part).",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "incompatible_specializes_kind",
        severity: "warning",
        meaning: "A definition specializes another definition of an incompatible kind.",
        typical_fix: "Specialize a compatible base definition for this element kind.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "incompatible_subset_redefine_kind",
        severity: "warning",
        meaning: "A subsetting or redefinition target is not compatible with the redefining feature kind.",
        typical_fix: "Subset or redefine a compatible inherited feature.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_redefines_target",
        severity: "warning",
        meaning: "A redefines target does not resolve to an inherited or visible feature.",
        typical_fix: "Point redefines at an existing inherited feature name.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "redefinition_multiplicity_widened",
        severity: "error",
        meaning: "A redefining feature loosens inherited multiplicity bounds.",
        typical_fix: "Keep multiplicity within inherited bounds or use explicit subsetting rules.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "redefinition_type_incompatible",
        severity: "error",
        meaning: "A redefining feature type or value is not conformant with the inherited feature.",
        typical_fix: "Align the redefinition type/value with the inherited feature.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "specialization_cycle",
        severity: "error",
        meaning: "A specialization, subsetting, or redefinition chain contains a cycle.",
        typical_fix: "Break the cyclic specializes/subsets/redefines chain.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_connection_segment",
        severity: "warning",
        meaning: "A connection endpoint feature chain contains an unresolved segment.",
        typical_fix: "Fix the first unresolved segment in the endpoint path or add the missing feature.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "connection_context_invalid",
        severity: "warning",
        meaning: "Connection endpoints are not connectable in the containing structural context.",
        typical_fix: "Connect compatible port or structural features within the same context.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "binding_connector_incompatible",
        severity: "warning",
        meaning: "Binding connector ends have incompatible value or type semantics.",
        typical_fix: "Bind features with compatible declared types.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "interface_end_invalid",
        severity: "warning",
        meaning: "An interface end does not map to a compatible port or feature.",
        typical_fix: "Declare a valid port type on each interface end.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "flow_direction_incompatible",
        severity: "warning",
        meaning: "Connected port features have incompatible flow directions.",
        typical_fix: "Align in/out directions or use conjugated port pairing.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "flow_item_type_incompatible",
        severity: "warning",
        meaning: "Connected port features carry incompatible item or parameter types.",
        typical_fix: "Use ports whose feature types match across the connection.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "conjugated_port_inconsistent",
        severity: "warning",
        meaning: "Both connected ports have the same conjugation; one should be conjugated and one not.",
        typical_fix: "Conjugate one port (~) or use matching non-conjugated types.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "attribute_value_type_mismatch",
        severity: "error",
        meaning: "An attribute value is incompatible with its declared type.",
        typical_fix: "Assign a value compatible with the attribute type.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invalid_enumeration_value",
        severity: "error",
        meaning: "An enum-typed attribute uses a literal that is not declared on the enum.",
        typical_fix: "Use a declared enumeration value (for example EnumName::member).",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "incompatible_unit_dimension",
        severity: "warning",
        meaning: "A value unit suffix is not compatible with indexed quantity/unit catalogs.",
        typical_fix: "Use a unit from the loaded quantity libraries or fix the unit symbol.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "non_boolean_expression",
        severity: "warning",
        meaning: "A constraint, assert, guard, or filter expression must evaluate to Boolean.",
        typical_fix: "Rewrite the expression to produce a Boolean result.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "calculation_binding_mismatch",
        severity: "warning",
        meaning: "A calculation invocation does not match declared parameter count or binding.",
        typical_fix: "Provide arguments matching the calculation definition parameters.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "perform_target_invalid_kind",
        severity: "warning",
        meaning: "A perform relationship targets an element that is not an action definition or usage.",
        typical_fix: "Point perform at an action definition or action usage in scope.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "transition_endpoint_invalid_state",
        severity: "warning",
        meaning: "A transition source or target does not resolve to a state usage.",
        typical_fix: "Use state usages for both transition endpoints.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "transition_endpoint_invalid_context",
        severity: "warning",
        meaning: "Transition endpoints belong to different state definition contexts.",
        typical_fix: "Keep transition source and target within the same state definition.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "initial_state_invalid_target",
        severity: "warning",
        meaning: "An initial transition targets an element that is not a state usage.",
        typical_fix: "Point the initial transition at a state usage in the same composite.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "succession_endpoint_invalid",
        severity: "warning",
        meaning: "A behavior succession connects endpoints that are not action-like.",
        typical_fix: "Connect perform steps, actions, or merges in the behavior flow.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "satisfy_invalid_endpoint_kind",
        severity: "warning",
        meaning: "A satisfy relationship has incompatible requirement or use-case endpoint kinds.",
        typical_fix: "Satisfy requirements with requirements and use cases with use cases.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "verified_requirement_invalid_target",
        severity: "warning",
        meaning: "A verification case references a requirement that does not resolve.",
        typical_fix: "Reference an in-scope requirement definition or usage.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "assignment_target_unresolved",
        severity: "warning",
        meaning: "An assignment action target does not resolve to an assignable feature.",
        typical_fix: "Assign to a declared feature visible in the verification context.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "use_case_include_invalid_target",
        severity: "warning",
        meaning: "An include use case target does not resolve to a use case definition or usage.",
        typical_fix: "Include an in-scope use case definition or usage.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "view_expose_empty",
        severity: "information",
        meaning: "A view declares a body but exposes no members.",
        typical_fix: "Add expose members or remove the empty view body.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "view_rendering_invalid_target",
        severity: "warning",
        meaning: "A view rendering member does not resolve to a rendering definition or usage.",
        typical_fix: "Type the rendering member with a valid rendering definition.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "metadata_annotation_unresolved",
        severity: "warning",
        meaning: "A metadata annotation does not resolve to a metadata definition.",
        typical_fix: "Declare or import the metadata definition and annotate with its type.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "transition_guard_non_boolean",
        severity: "warning",
        meaning: "A state transition guard expression must evaluate to Boolean.",
        typical_fix: "Rewrite the guard to a Boolean expression (for example a comparison or logical operator).",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "multiple_initial_states",
        severity: "warning",
        meaning: "Modeling guidance: a state definition declares more than one unguarded initial transition (SysML 7.18.2 allows multiple guarded conditionals).",
        typical_fix: "Keep a single unguarded `then` or `first` initial transition; use guarded `if ... then` for conditional entry.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "missing_initial_state",
        severity: "information",
        meaning: "Modeling guidance: a state definition has state usages but no initial transition (including guarded entry successions).",
        typical_fix: "Add a `then` or `first` transition from entry to designate how execution enters the state machine.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "multiple_final_states",
        severity: "warning",
        meaning: "Modeling guidance: a state definition declares more than one explicit `final`/`final state` marker (not counting `then done` transitions per SysML 7.18.3).",
        typical_fix: "Keep a single explicit `final` marker, or express finality with transitions to `done`.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "missing_final_state",
        severity: "information",
        meaning: "Modeling guidance: a state definition has state usages but no finality indicator (`final`/`final state` or a transition to `done` per SysML 7.18.3).",
        typical_fix: "Add a transition to `done` from a terminal state, or an explicit `final` marker if your tooling uses that extension.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "accept_payload_incompatible",
        severity: "warning",
        meaning: "An accept action payload type resolves to an incompatible definition kind.",
        typical_fix: "Type the accept payload with an action definition or compatible item type.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "send_payload_incompatible",
        severity: "warning",
        meaning: "A send action payload type resolves to an incompatible definition kind.",
        typical_fix: "Type the send payload with an action definition or compatible item type.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "assignment_value_incompatible",
        severity: "warning",
        meaning: "A verification assignment assigns a value incompatible with the target feature type.",
        typical_fix: "Assign a literal or expression that matches the declared attribute type.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "view_filter_non_boolean",
        severity: "warning",
        meaning: "A view body filter expression must evaluate to Boolean.",
        typical_fix: "Rewrite the filter to a Boolean expression.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "requirement_constraint_invalid_membership",
        severity: "warning",
        meaning: "A require constraint on a requirement has invalid parameter membership or an empty expression.",
        typical_fix: "Declare constraint parameters with `in`/`out`/`inout` and a type, and provide a constraint expression.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "verification_case_invalid_shape",
        severity: "warning",
        meaning: "A verification case has an invalid combination of objectives, verdicts, and then-actions.",
        typical_fix: "Declare one verdict/return, pair then-actions with a verdict, and add objectives when verifying requirements.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "case_objective_binding_cardinality",
        severity: "warning",
        meaning: "Case objectives expect a single subject or analysis result but the case declares the wrong count.",
        typical_fix: "Declare exactly one subject or return ref matching the objective binding kind.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "case_subject_missing",
        severity: "warning",
        meaning: "A verification or analysis case has subject-bound objectives but no declared subject.",
        typical_fix: "Add a `subject` clause to the case definition.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "viewpoint_reference_unresolved",
        severity: "warning",
        meaning: "A viewpoint frame, concern, or import target does not resolve in the workspace.",
        typical_fix: "Fix the reference or import the defining package.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "viewpoint_rep_language_unresolved",
        severity: "warning",
        meaning: "A textual representation on a viewpoint or frame is missing a language identifier.",
        typical_fix: "Add `rep ... language \"...\"` with a valid language name.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "metadata_keyword_unresolved",
        severity: "warning",
        meaning: "A user-defined declaration keyword does not resolve to a metadata definition.",
        typical_fix: "Declare the metadata definition for the keyword or use a built-in declaration form.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "metadata_keyword_collision",
        severity: "warning",
        meaning: "The same metadata definition short name is declared more than once in one document.",
        typical_fix: "Rename or remove the duplicate metadata definition.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "semantic_diagnostic",
        severity: "warning",
        meaning: "A semantic rule reported by the graph builder (see message for detail).",
        typical_fix: "Follow the diagnostic message; use spec42_check for exact range.",
        editor_quick_fixes: None,
    },
];

/// Diagnostics that reflect modeling/tooling guidance rather than normative SysML constraints.
const MODELING_GUIDANCE_CODES: &[&str] = &[
    "analysis_evaluation_incomplete",
    "duplicate_connection",
    "missing_final_state",
    "missing_initial_state",
    "missing_library_context",
    "multiple_final_states",
    "multiple_initial_states",
    "semantic_diagnostic",
    "unconnected_port",
    "untyped_part_usage",
    "view_expose_empty",
];

/// Whether a diagnostic code reflects a normative SysML constraint or modeling/tooling guidance.
pub fn alignment(code: &str) -> &'static str {
    if MODELING_GUIDANCE_CODES.contains(&code) {
        "modeling_guidance"
    } else {
        "spec_constraint"
    }
}

pub fn lookup(code: &str) -> Option<&'static DiagnosticCatalogEntry> {
    CATALOG.iter().find(|entry| entry.code == code)
}

pub fn all_codes() -> Vec<&'static str> {
    CATALOG.iter().map(|e| e.code).collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{all_codes, lookup, CATALOG};

    const EMITTED_DIAGNOSTIC_CODES: &[(&str, &str)] = &[
        ("allocate_endpoint_prefers_usage", "warning"),
        ("ambiguous_connection_endpoint", "warning"),
        ("ambiguous_name_reference", "warning"),
        ("allocation_type_not_allocation_def", "warning"),
        ("attribute_value_type_mismatch", "error"),
        ("binding_connector_incompatible", "warning"),
        ("calculation_binding_mismatch", "warning"),
        ("conjugated_port_inconsistent", "warning"),
        ("connection_context_invalid", "warning"),
        ("analysis_constraint_failed", "warning"),
        ("analysis_evaluation_incomplete", "information"),
        ("analysis_evaluation_unresolved", "warning"),
        ("assignment_target_unresolved", "warning"),
        ("connection_endpoint_not_port", "warning"),
        ("duplicate_connection", "information"),
        ("duplicate_namespace_member", "warning"),
        ("flow_direction_incompatible", "warning"),
        ("flow_item_type_incompatible", "warning"),
        ("implicit_redefinition_without_operator", "error"),
        ("initial_state_invalid_target", "warning"),
        ("import_kind_mismatch", "warning"),
        ("incompatible_specializes_kind", "warning"),
        ("incompatible_subset_redefine_kind", "warning"),
        ("incompatible_type_kind", "warning"),
        ("incompatible_unit_dimension", "warning"),
        ("inherited_attribute_value_type_mismatch", "error"),
        ("interface_end_invalid", "warning"),
        ("invalid_enumeration_value", "error"),
        ("invalid_import_filter", "warning"),
        ("invalid_recursive_import", "warning"),
        ("invalid_allocation_endpoints", "warning"),
        ("invalid_multiplicity", "warning"),
        ("invalid_redefines_reference", "warning"),
        ("invalid_qualified_name_segment", "warning"),
        ("invalid_verdict_value", "warning"),
        ("metadata_annotation_unresolved", "warning"),
        ("metadata_keyword_collision", "warning"),
        ("metadata_keyword_unresolved", "warning"),
        ("missing_final_state", "information"),
        ("missing_initial_state", "information"),
        ("missing_library_context", "information"),
        ("multiple_final_states", "warning"),
        ("multiple_initial_states", "warning"),
        ("non_boolean_expression", "warning"),
        ("objective_binding_unresolved", "warning"),
        ("perform_target_invalid_kind", "warning"),
        ("port_type_mismatch", "warning"),
        ("requirement_constraint_invalid_membership", "warning"),
        ("redefinition_multiplicity_widened", "error"),
        ("redefinition_type_incompatible", "error"),
        ("satisfy_endpoint_prefers_usage", "warning"),
        ("satisfy_invalid_endpoint_kind", "warning"),
        ("send_payload_incompatible", "warning"),
        ("semantic_diagnostic", "warning"),
        ("specialization_cycle", "error"),
        ("succession_endpoint_invalid", "warning"),
        ("transition_endpoint_invalid_context", "warning"),
        ("transition_endpoint_invalid_state", "warning"),
        ("transition_guard_non_boolean", "warning"),
        ("accept_payload_incompatible", "warning"),
        ("assignment_value_incompatible", "warning"),
        ("case_objective_binding_cardinality", "warning"),
        ("case_subject_missing", "warning"),
        ("verification_case_invalid_shape", "warning"),
        ("view_filter_non_boolean", "warning"),
        ("viewpoint_reference_unresolved", "warning"),
        ("viewpoint_rep_language_unresolved", "warning"),
        ("unconnected_port", "information"),
        ("unresolved_allocate_source", "warning"),
        ("unresolved_connection_segment", "warning"),
        ("unresolved_allocate_target", "warning"),
        ("unresolved_import_target", "warning"),
        ("unresolved_redefines_target", "warning"),
        ("unresolved_pending_expression_relationship", "error"),
        ("unresolved_pending_relationship", "error"),
        ("unresolved_ref_type_reference", "warning"),
        ("unresolved_satisfy_source", "warning"),
        ("unresolved_satisfy_target", "warning"),
        ("unresolved_specializes_reference", "warning"),
        ("unresolved_type_reference", "warning"),
        ("unresolved_viewpoint_conformance_target", "warning"),
        ("untyped_part_usage", "information"),
        ("use_case_include_invalid_target", "warning"),
        ("verified_requirement_invalid_target", "warning"),
        ("view_expose_empty", "information"),
        ("view_rendering_invalid_target", "warning"),
        ("viewpoint_conformance_invalid_target_kind", "warning"),
    ];

    #[test]
    fn lookup_returns_entry_for_known_code() {
        let entry = lookup("unresolved_type_reference").expect("catalog entry");
        assert_eq!(entry.code, "unresolved_type_reference");
        assert_eq!(entry.severity, "warning");
    }

    #[test]
    fn lookup_returns_none_for_unknown_code() {
        assert!(lookup("not_a_real_diagnostic_code").is_none());
    }

    #[test]
    fn all_codes_includes_common_semantic_codes() {
        let codes = all_codes();
        assert!(codes.contains(&"unresolved_type_reference"));
        assert!(codes.contains(&"missing_library_context"));
    }

    #[test]
    fn catalog_codes_are_unique() {
        let mut seen = HashSet::new();
        for entry in CATALOG {
            assert!(
                seen.insert(entry.code),
                "duplicate catalog code {}",
                entry.code
            );
        }
    }

    #[test]
    fn catalog_covers_known_emitted_diagnostics_with_current_severity() {
        for (code, expected_severity) in EMITTED_DIAGNOSTIC_CODES {
            let entry = lookup(code).unwrap_or_else(|| panic!("missing catalog entry for {code}"));
            assert_eq!(
                entry.severity, *expected_severity,
                "catalog severity for {code} must match current emitted severity"
            );
        }
    }

    #[test]
    fn alignment_classifies_state_cardinality_as_modeling_guidance() {
        assert_eq!(super::alignment("missing_final_state"), "modeling_guidance");
        assert_eq!(super::alignment("transition_guard_non_boolean"), "spec_constraint");
    }
}
