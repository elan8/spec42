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
        code: "unsupported_package_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_part_definition_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_part_usage_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_attribute_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_requirement_definition_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_port_definition_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_port_usage_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_action_definition_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_action_usage_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_state_definition_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_connection_definition_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_interface_definition_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_view_definition_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_constraint_definition_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_calc_definition_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_rendering_definition_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_occurrence_definition_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_analysis_case_definition_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_case_definition_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_verification_case_definition_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_use_case_definition_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_reference_usage_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_relationship_body_member",
        severity: "warning",
        meaning: "This member is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_parser_construct",
        severity: "warning",
        meaning: "This construct is parsed but not modelled by the semantic publication.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_type_reference",
        severity: "warning",
        meaning: "A type name on a usage or feature does not resolve to a known definition in the workspace or libraries.",
        typical_fix: "Add or import the missing definition, fix the qualified name, or configure library paths / standard library.",
        editor_quick_fixes: Some(&[
            "add_import",
            "create_definition_for_unresolved_type",
        ]),
    },
    DiagnosticCatalogEntry {
        code: "unresolved_specializes_reference",
        severity: "warning",
        meaning: "A specializes target does not resolve to a known definition.",
        typical_fix: "Correct the specializes clause or add the base definition.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_import_target",
        severity: "warning",
        meaning: "An import statement targets a package or namespace that cannot be found.",
        typical_fix: "Fix the import path, add the defining file to the workspace, or index the library root.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_reference",
        severity: "warning",
        meaning: "This reference does not resolve.",
        typical_fix: "Follow the diagnostic message; use spec42 check for the exact range and related locations.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_filtered_import",
        severity: "warning",
        meaning: "A parser-recognized filtered namespace import has no implemented semantic expansion.",
        typical_fix: "Use an unfiltered import or remove the filter until filtered imports are supported.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unsupported_reference",
        severity: "warning",
        meaning: "This reference form is parsed but not semantically supported.",
        typical_fix: "Rewrite the member using a construct the semantic publication models, or track the gap; the parser accepted it but no semantic fact is published for it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "non_converged_resolution",
        severity: "warning",
        meaning: "Resolution did not converge, so this reference has no settled outcome.",
        typical_fix: "Follow the diagnostic message; use spec42 check for the exact range and related locations.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "ambiguous_import_target",
        severity: "warning",
        meaning: "An import target resolves to more than one semantic element.",
        typical_fix: "Qualify the target further or remove the conflicting declarations.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "ambiguous_reference",
        severity: "error",
        meaning: "This reference names several elements, so it identifies none of them.",
        typical_fix: "Follow the diagnostic message; use spec42 check for the exact range and related locations.",
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
        code: "specialization_cycle",
        severity: "error",
        meaning: "A specialization, subsetting, or redefinition chain contains a cycle.",
        typical_fix: "Break the cyclic specializes/subsets/redefines chain.",
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
        code: "subsetting_type_incompatible",
        severity: "error",
        meaning: "A subsetting feature type is not conformant with the subsetted feature.",
        typical_fix: "Use the subsetted feature type or a type that specializes it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "flow_payload_type_not_occurrence",
        severity: "error",
        meaning: "A flow payload is typed by a value type rather than an occurrence type.",
        typical_fix: "Use a part, item, or occurrence definition for the payload type.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "incomplete_connection_like_end_pair",
        severity: "warning",
        meaning: "A connection, flow, or allocation definition declares only one direct end.",
        typical_fix: "Declare a second end, or specialize a definition that supplies the inherited ends.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invalid_binary_connection_like_end_count",
        severity: "warning",
        meaning: "A flow or allocation definition declares more than its required two direct ends.",
        typical_fix: "Keep exactly two direct ends, or model an n-ary relationship as a general connection definition.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "end_feature_invalid_restrictions",
        severity: "warning",
        meaning: "An end feature is derived, abstract, or composite.",
        typical_fix: "Remove the incompatible modifier from the end feature.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invalid_variation_member_kind",
        severity: "warning",
        meaning: "A typed variant member has a different usage kind from its variation.",
        typical_fix: "Declare variants using the variation's usage kind.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "redefinition_featuring_type_incompatible",
        severity: "error",
        meaning: "A feature redefines another feature from an unrelated featuring type.",
        typical_fix: "Place the redefining feature on the same type as the target or a specializing type.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "redefinition_end_mismatch",
        severity: "warning",
        meaning: "A feature redefines an end feature but is not itself declared as an end.",
        typical_fix: "Declare the redefining feature as an end, or redefine a non-end feature.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "redefinition_direction_mismatch",
        severity: "warning",
        meaning: "A redefining feature has a different explicit direction from its redefined feature.",
        typical_fix: "Align the declared feature direction with the redefined feature.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "subsetting_uniqueness_mismatch",
        severity: "warning",
        meaning: "A non-unique feature subsets a feature explicitly declared unique.",
        typical_fix: "Remove `nonunique` or subset a non-unique feature.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "single_type_relationship_operand",
        severity: "error",
        meaning: "A type owns exactly one unions, intersects or differences operand; KerML requires zero or at least two.",
        typical_fix: "Name a second operand, or drop the clause and state the specialization directly.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "attribute_value_type_mismatch",
        severity: "error",
        meaning: "An authored value has a type unrelated to the feature it is bound to.",
        typical_fix: "Assign a value whose type is the feature's, or one of its subtypes.",
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
        code: "unknown_unit_symbol",
        severity: "warning",
        meaning: "A value unit suffix names no unit in the admitted measurement libraries.",
        typical_fix: "Use a unit declared by an admitted library, or fix the unit symbol.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "ambiguous_unit_symbol",
        severity: "warning",
        meaning: "A value unit suffix names several admitted units, so it identifies none of them.",
        typical_fix: "Qualify the unit with the package that declares the intended one.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "incompatible_unit_dimension",
        severity: "warning",
        meaning: "A recognized unit suffix has a quantity dimension incompatible with the attribute type.",
        typical_fix: "Use a unit whose dimension matches the declared quantity value type.",
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
        code: "view_filter_non_boolean",
        severity: "warning",
        meaning: "A view body filter expression must evaluate to Boolean.",
        typical_fix: "Rewrite the filter to a Boolean expression.",
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
        code: "invalid_import_filter",
        severity: "warning",
        meaning: "An import filter expression is not Boolean-valued.",
        typical_fix: "Rewrite the filter as a Boolean condition over visible metadata or properties.",
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
        code: "connection_endpoint_not_port",
        severity: "warning",
        meaning: "A connection endpoint is not a port-like feature.",
        typical_fix: "Connect port usages or adjust the connection statement.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "port_type_mismatch",
        severity: "warning",
        meaning: "Connected ports have incompatible port definitions or types.",
        typical_fix: "Use compatible port types or an interface that connects them.",
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
        code: "connection_context_invalid",
        severity: "warning",
        meaning: "Connection endpoints are not connectable in the containing structural context.",
        typical_fix: "Connect compatible port or structural features within the same context.",
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
        code: "binding_connector_incompatible",
        severity: "warning",
        meaning: "Binding connector ends have incompatible value or type semantics.",
        typical_fix: "Bind features with compatible declared types.",
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
        code: "transition_guard_non_boolean",
        severity: "warning",
        meaning: "A state transition guard expression must evaluate to Boolean.",
        typical_fix: "Rewrite the guard to a Boolean expression (for example a comparison or logical operator).",
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
        code: "missing_final_state",
        severity: "information",
        meaning: "Modeling guidance: a state definition has state usages but no finality indicator (`final`/`final state` or a transition to `done` per SysML 7.18.3).",
        typical_fix: "Add a transition to `done` from a terminal state, or an explicit `final` marker if your tooling uses that extension.",
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
        code: "accept_payload_incompatible",
        severity: "warning",
        meaning: "An accept action payload type resolves to an incompatible definition kind.",
        typical_fix: "Type the accept payload with an action definition or compatible item type.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "duplicate_role_member",
        severity: "warning",
        meaning: "A requirement, case, viewpoint, or view declares more than one member for a role that permits only one.",
        typical_fix: "Keep one subject, objective, or rendering member in the owning declaration.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "subject_member_not_first",
        severity: "warning",
        meaning: "A subject role member appears after another input role member.",
        typical_fix: "Move the `subject` declaration before actor or stakeholder role members.",
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
        code: "use_case_include_invalid_target",
        severity: "warning",
        meaning: "An include use case target does not resolve to a use case definition or usage.",
        typical_fix: "Include an in-scope use case definition or usage.",
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
        code: "view_type_non_standard",
        severity: "warning",
        meaning: "This view is typed by a definition outside the SysML standard view catalog.",
        typical_fix: "Follow the diagnostic message; use spec42 check for the exact range and related locations.",
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
        code: "viewpoint_rep_language_unresolved",
        severity: "warning",
        meaning: "A textual representation on a viewpoint or frame is missing a language identifier.",
        typical_fix: "Add `rep ... language \"...\"` with a valid language name.",
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
        code: "invalid_multiplicity",
        severity: "warning",
        meaning: "A multiplicity clause is not valid for the usage.",
        typical_fix: "Fix multiplicity syntax or bounds.",
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
        code: "analysis_evaluation_unresolved",
        severity: "warning",
        meaning: "An analysis expression could not be evaluated.",
        typical_fix: "Check referenced values, operators, and expression syntax supported by Spec42.",
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
];

/// Diagnostics that reflect modeling/tooling guidance rather than normative SysML constraints.
const MODELING_GUIDANCE_CODES: &[&str] = &[
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
    "view_expose_empty_result",
    "view_expose_unresolved",
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

    /// The catalog documents exactly the codes the publication can report.
    ///
    /// Both directions: a new code that nothing documents, and a documented code nothing can
    /// report, are the two ways a hand-written table drifts from the owner that decides it.
    #[test]
    fn the_catalog_documents_exactly_the_published_codes() {
        let published = sysml_query::resolved_slice::DiagnosticCode::SEMANTIC
            .iter()
            .map(|code| code.as_str())
            .collect::<HashSet<_>>();
        let documented = all_codes().into_iter().collect::<HashSet<_>>();
        let undocumented = published.difference(&documented).collect::<Vec<_>>();
        let unreportable = documented.difference(&published).collect::<Vec<_>>();
        assert!(
            undocumented.is_empty(),
            "codes the publication reports with no catalog entry: {undocumented:?}"
        );
        assert!(
            unreportable.is_empty(),
            "catalog entries for codes nothing reports: {unreportable:?}"
        );
    }
    #[test]
    fn alignment_classifies_state_cardinality_as_modeling_guidance() {
        assert_eq!(super::alignment("missing_final_state"), "modeling_guidance");
        assert_eq!(
            super::alignment("transition_guard_non_boolean"),
            "spec_constraint"
        );
    }
}
