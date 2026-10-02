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
        meaning: "A definition or KerML classifier specializes one of an incompatible kind: a SysML definition of an unrelated family, or a KerML metaclass the classifier rules forbid (a data type specializing a class or association, a class specializing a data type or, unless it is one, an association, a structure specializing a behavior, or a behavior specializing a structure).",
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
        meaning: "A specialization, subsetting, or redefinition cycle is entirely closed and does not include Base::Anything. KerML treats other cycles as shared extent.",
        typical_fix: "Add an escape specialization (typically of Anything) or break the closed cycle.",
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
        code: "variation_owns_feature_membership",
        severity: "warning",
        meaning: "A variation definition or usage owns an ordinary feature; its owned members must all be variants (SysML validateDefinitionVariationOwnedFeatureMembership, validateUsageVariationOwnedFeatureMembership).",
        typical_fix: "Declare the member with `variant`, or move it out of the variation.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "variation_specializes_variation",
        severity: "warning",
        meaning: "A variation definition or usage specializes (types, subclassifies, subsets or redefines) another variation (SysML validateDefinitionVariationSpecialization, validateUsageVariationSpecialization).",
        typical_fix: "Specialize a non-variation definition or usage, or remove the `variation` prefix.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "variant_outside_variation",
        severity: "warning",
        meaning: "A `variant` member is owned by a definition or usage that is not a variation (SysML validateVariantMembershipOwningNamespace).",
        typical_fix: "Add the `variation` prefix to the owning definition or usage, or declare the member without `variant`.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "expose_invalid_owner",
        severity: "warning",
        meaning: "An `expose` is owned by something other than a view usage (SysML validateExposeOwningNamespace).",
        typical_fix: "Move the `expose` into the body of a `view` usage.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "verification_membership_invalid_owner",
        severity: "warning",
        meaning: "A `verify` requirement member is not owned by the objective of a verification case definition or usage (SysML validateRequirementVerificationMembershipOwningType).",
        typical_fix: "Move the `verify` member into the `objective` of a `verification` definition or usage.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "requirement_constraint_invalid_owner",
        severity: "warning",
        meaning: "An `assume` or `require` constraint member is owned by something other than a requirement definition or usage (SysML validateRequirementConstraintMembershipOwningType).",
        typical_fix: "Move the constraint member into a requirement, or declare it as an ordinary constraint.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "state_duplicate_subaction_kind",
        severity: "warning",
        meaning: "A state definition or usage owns more than one `entry`, `do` or `exit` action (SysML validateStateDefinitionStateSubactionKind, validateStateUsageStateSubactionKind).",
        typical_fix: "Keep a single `entry`, `do` and `exit` action per state; combine their behavior into one action.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "assert_target_invalid_kind",
        severity: "warning",
        meaning: "An `assert <name>;` member references a feature that is not a constraint usage (SysML validateAssertConstraintUsageReference).",
        typical_fix: "Reference a `constraint` usage, or declare the asserted constraint inline with `assert constraint { ... }`.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "event_occurrence_reference_not_occurrence",
        severity: "warning",
        meaning: "An `event occurrence` references a feature that is not an occurrence usage (SysML validateEventOccurrenceUsageReference).",
        typical_fix: "Reference an occurrence usage (an `occurrence`, `item`, `part`, `action`, ...) as the event.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "cross_subsetting_crossing_feature_invalid",
        severity: "warning",
        meaning: "A `crosses` relationship is owned by a feature that is not an end feature of a type with at least two end features (KerML validateCrossSubsettingCrossingFeature).",
        typical_fix: "Declare `crosses` only on an `end` feature of an association or connector with two or more ends.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "cross_subsetting_crossed_feature_invalid",
        severity: "warning",
        meaning: "The crossed feature of an end feature is not a two-feature chain through the opposite end (KerML validateCrossSubsettingCrossedFeature).",
        typical_fix: "Cross a chain `otherEnd.feature` that starts with the opposite end feature.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "transition_trigger_source_not_state",
        severity: "warning",
        meaning: "A transition with an `accept` trigger has a source that is not a state usage (SysML validateTransitionUsageTriggerActions).",
        typical_fix: "Remove the trigger, or make the transition's source a state usage.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "return_parameter_membership_invalid_owner",
        severity: "warning",
        meaning: "A `return` parameter is owned by something other than a function or an expression (KerML validateReturnParameterMembershipOwningType).",
        typical_fix: "Move the `return` parameter into a function, expression, calculation, constraint, requirement or case.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "function_result_parameter_count",
        severity: "warning",
        meaning: "A function owns more than one `return` parameter (KerML validateFunctionResultParameterMembership).",
        typical_fix: "Keep a single `return` parameter; declare the others as `out` parameters or features.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "expression_result_parameter_count",
        severity: "warning",
        meaning: "An expression owns more than one `return` parameter (KerML validateExpressionResultParameterMembership).",
        typical_fix: "Keep a single `return` parameter; declare the others as `out` parameters or features.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "feature_reference_referent_not_feature",
        severity: "warning",
        meaning: "A name used as a value in an expression references an element that is not a feature, such as a type or package (KerML validateFeatureReferenceExpressionReferentIsFeature).",
        typical_fix: "Reference a feature; to talk about a type or other element itself, use `T.metadata` or `meta`.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "trigger_when_argument_not_boolean",
        severity: "warning",
        meaning: "The argument of an `accept when` trigger is not a Boolean condition (SysML validateTriggerInvocationExpressionWhenArgument).",
        typical_fix: "Write a Boolean expression, or reference a Boolean feature, after `when`.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "trigger_at_argument_not_time_instant",
        severity: "warning",
        meaning: "The argument of an `accept at` trigger is not a time instant value (SysML validateTriggerInvocationExpressionAtArgument).",
        typical_fix: "Reference a `Time::TimeInstantValue` after `at`; use `after` for a duration.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "trigger_after_argument_not_duration",
        severity: "warning",
        meaning: "The argument of an `accept after` trigger is not a duration value (SysML validateTriggerInvocationExpressionAfterArgument).",
        typical_fix: "Reference an `ISQBase::DurationValue`, or write a quantity in a duration unit, after `after`.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invocation_argument_redefines_no_parameter",
        severity: "warning",
        meaning: "A named invocation argument names a feature of the invoked behavior that is not one of its input parameters (KerML validateInvocationExpressionParameterRedefinition).",
        typical_fix: "Name an `in` or `inout` parameter of the invoked behavior.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invocation_duplicate_parameter_redefinition",
        severity: "warning",
        meaning: "Two arguments of one invocation bind the same parameter (KerML validateInvocationExpressionNoDuplicateParameterRedefinition).",
        typical_fix: "Bind each parameter at most once.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "constructor_duplicate_feature_redefinition",
        severity: "warning",
        meaning: "Two arguments of one `new T(...)` initialise the same feature of T (KerML validateConstructorExpressionNoDuplicateFeatureRedefinition).",
        typical_fix: "Initialise each feature at most once.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invocation_instantiated_type_not_behavior",
        severity: "warning",
        meaning: "An invocation `F(...)` invokes something that is neither a behavior nor a feature typed by a behavior (KerML validateInvocationExpressionInstantiatedType).",
        typical_fix: "Invoke a function, calculation or other behavior; construct a structure or data value with `new T(...)`.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "metadata_type_not_metaclass",
        severity: "warning",
        meaning: "A metadata feature is not typed by exactly one metaclass (KerML validateMetadataFeatureMetaclass).",
        typical_fix: "Type the metadata feature by a single `metaclass` (or `metadata def`).",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "metadata_metaclass_abstract",
        severity: "warning",
        meaning: "A metadata feature is typed by an abstract metaclass (KerML validateMetadataFeatureMetaclassNotAbstract).",
        typical_fix: "Type the metadata feature by a concrete metaclass.",
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
        code: "assignment_target_not_time_varying",
        severity: "warning",
        meaning: "An assignment's referent resolves to a feature that cannot have time-varying values: its featureTarget has isVariable = false, for example a feature not owned by an occurrence (SysML validateAssignmentActionUsage).",
        typical_fix: "Assign to a feature of an occurrence definition or usage (which is variable unless it is a portion, a link participant, or a composite action), or stop assigning to this feature.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "subsetting_constant_mismatch",
        severity: "warning",
        meaning: "A variable feature that is not constant subsets or redefines a constant feature, so it could change values the subsetted feature fixes (KerML validateSubsettingConstantConformance).",
        typical_fix: "Declare the subsetting feature constant as well, or subset a feature that is not constant.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "subsetting_target_not_accessible",
        severity: "warning",
        meaning: "A feature subsets (or references) a feature whose featuring types none of the subsetting feature's featuring types specialize, so the subsetted feature has no values in the subsetting feature's context (KerML validateSubsettingFeaturingTypes).",
        typical_fix: "Subset a feature of the owning type or one of its supertypes, or specialize the type that features the subsetted feature.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "type_relationship_operand_is_self",
        severity: "error",
        meaning: "A type names itself as an operand of its own `unions`, `intersects` or `differences` relationship (KerML validateTypeUnioningTypesNotSelf, validateTypeIntersectingTypesNotSelf, validateTypeDifferencingTypesNotSelf).",
        typical_fix: "Remove the type from its own operand list.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "top_level_import_not_private",
        severity: "warning",
        meaning: "An import directly in a root namespace (outside every package) is declared public or protected; such imports must be private (KerML validateImportTopLevelVisibility).",
        typical_fix: "Declare the import `private`, drop the visibility keyword (imports default to private), or move it into a package.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "feature_chaining_single_operand",
        severity: "warning",
        meaning: "A `chains` clause names exactly one chaining feature; a feature chain needs none or at least two (KerML validateFeatureChainingFeatureNotOne).",
        typical_fix: "Chain at least two features with `.` (`chains a.b`), or subset or redefine the single feature instead.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "feature_chaining_includes_self",
        severity: "warning",
        meaning: "A feature's `chains` clause resolves one of its chaining features to the feature itself (KerML validateFeatureChainingFeaturesNotSelf).",
        typical_fix: "Chain through other features; a feature cannot be part of its own chain.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "feature_chaining_not_featured_within_previous",
        severity: "warning",
        meaning: "A dotted feature chain names a chaining feature whose featuring types the previous chaining feature does not conform to, typically one reached through an import (KerML validateFeatureChainingFeatureConformance).",
        typical_fix: "Chain through a feature of the previous feature's type.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "feature_value_overrides_non_default",
        severity: "warning",
        meaning: "A feature with a value redefines (directly or indirectly) a feature whose value is bound with `=` or `:=` rather than given as a `default`, so the bound value cannot be overridden (KerML validateFeatureValueOverriding).",
        typical_fix: "Declare the redefined feature's value as `default`, or drop the overriding value.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "feature_multiple_reference_subsettings",
        severity: "warning",
        meaning: "A feature has more than one `references` (`::>`) clause, but it may own at most one ReferenceSubsetting (KerML validateFeatureOwnedReferenceSubsetting).",
        typical_fix: "Keep one `references` clause and use `subsets` for the others.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "feature_multiple_cross_subsettings",
        severity: "warning",
        meaning: "A feature has more than one `crosses` (`=>`) clause, but it may own at most one CrossSubsetting (KerML validateFeatureOwnedCrossSubsetting).",
        typical_fix: "Keep a single `crosses` clause.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "variable_feature_owner_not_occurrence",
        severity: "warning",
        meaning: "A KerML feature declared `var` (or `const`, which implies variable) has no owning type, or its owning type does not specialize Occurrences::Occurrence (KerML validateFeatureIsVariable).",
        typical_fix: "Move the feature into a class, behavior or other occurrence type, or drop the `var`/`const` prefix.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "portion_feature_is_variable",
        severity: "warning",
        meaning: "A KerML feature is declared both `portion` and variable (`var` or `const`); portions of an occurrence do not vary over its lifetime (KerML validateFeaturePortionNotVariable).",
        typical_fix: "Remove either the `portion` prefix or the `var`/`const` prefix.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "portion_owner_not_occurrence",
        severity: "warning",
        meaning: "A `snapshot` or `timeslice` occurrence usage is not owned by an occurrence definition or occurrence usage (SysML validateOccurrenceUsagePortionKind).",
        typical_fix: "Declare the snapshot or timeslice inside an occurrence, item, part, action or other occurrence definition or usage.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "occurrence_multiple_individual_definitions",
        severity: "warning",
        meaning: "An occurrence usage has more than one individual occurrence definition among its types, including types inherited through subsetting or redefinition (SysML validateOccurrenceUsageIndividualDefinition).",
        typical_fix: "Type the usage by at most one `individual` occurrence definition.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "individual_usage_without_individual_definition",
        severity: "warning",
        meaning: "An `individual` occurrence usage has no individual occurrence definition among its settled types (SysML validateOccurrenceUsageIndividualUsage).",
        typical_fix: "Type the usage by an `individual` occurrence definition, or drop the `individual` prefix.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "initial_value_feature_not_variable",
        severity: "warning",
        meaning: "A feature has an initial value (`:=`) but is not variable, so it has no initial time to bind the value at (KerML validateFeatureValueIsInitial).",
        typical_fix: "Use a bound value (`=`) instead, or make the feature variable (a KerML `var` feature, or a SysML usage owned by an occurrence).",
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
        code: "specialization_specific_conjugated",
        severity: "error",
        meaning: "A conjugated type is the specific type of a specialization (KerML validateSpecializationSpecificNotConjugated).",
        typical_fix: "Remove the specialization, or specialize the original type instead of its conjugate.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "exhibit_target_invalid_kind",
        severity: "warning",
        meaning: "An exhibit state usage references a feature that is not a state usage (SysML validateExhibitStateUsageReference).",
        typical_fix: "Point exhibit at a state usage in scope.",
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
        code: "association_end_type_not_one",
        severity: "warning",
        meaning: "An owned end feature of a KerML association does not have exactly one type, after redundant supertypes are removed (KerML validateAssociationEndTypes).",
        typical_fix: "Type the end feature with exactly one type, e.g. `end feature target : Thing;`.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "association_related_types_insufficient",
        severity: "warning",
        meaning: "A concrete KerML association relates fewer than two types through its end features, owned or inherited (KerML validateAssociationRelatedTypes).",
        typical_fix: "Declare at least two typed end features, or mark the association `abstract`.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "binary_association_end_count",
        severity: "warning",
        meaning: "A KerML association with more than two owned end features specializes Links::BinaryLink; reported at each end after the second (KerML validateAssociationBinarySpecialization).",
        typical_fix: "Remove the extra end features, or specialize Links::Link instead of Links::BinaryLink.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "binary_connector_end_count",
        severity: "warning",
        meaning: "A KerML connector with more than two owned ends specializes Links::BinaryLink; reported at each end after the second (KerML validateConnectorBinarySpecialization).",
        typical_fix: "Remove the extra ends, or type the connector by an association that is not binary.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "binding_connector_not_binary",
        severity: "warning",
        meaning: "A KerML binding connector does not relate exactly two features through its ends' reference subsettings (KerML validateBindingConnectorIsBinary).",
        typical_fix: "Bind exactly two features, e.g. `binding of a = b;`.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "connector_related_features_insufficient",
        severity: "warning",
        meaning: "A concrete KerML connector relates fewer than two features through its ends' reference subsettings (KerML validateConnectorRelatedFeatures).",
        typical_fix: "Give the connector at least two ends that reference features, e.g. `end feature e ::> a;`, or mark it `abstract`.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "end_feature_multiplicity_not_one",
        severity: "warning",
        meaning: "An end feature authors a literal multiplicity other than 1..1 (KerML validateFeatureEndMultiplicity).",
        typical_fix: "Change the end feature's multiplicity to `[1]`, or remove it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "flow_multiple_payload_features",
        severity: "warning",
        meaning: "A flow authors more than one `of` payload clause, so it owns more than one PayloadFeature (KerML validateFlowPayloadFeature).",
        typical_fix: "Keep a single payload clause, e.g. `flow of Thing from a to b;`.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "end_feature_has_direction",
        severity: "warning",
        meaning: "An end feature declares an in/out/inout direction (KerML validateFeatureEndNoDirection).",
        typical_fix: "Remove the direction from the end feature.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "metadata_body_feature_invalid",
        severity: "warning",
        meaning: "A feature declared in a metadata feature body does not redefine a feature of the annotating metaclass (KerML validateMetadataFeatureBody).",
        typical_fix: "Write the body member as a redefinition of a metaclass feature, e.g. `name = value;`.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "metadata_annotated_element_incompatible",
        severity: "warning",
        meaning: "A metadata feature annotates an element whose abstract-syntax metaclass is not admitted by the types of its metaclass's `annotatedElement` features (KerML validateMetadataFeatureAnnotatedElement).",
        typical_fix: "Annotate an element of an admitted metaclass, or widen the metaclass's `annotatedElement` redefinition.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "port_owned_usage_composite",
        severity: "warning",
        meaning: "A port definition owns a composite non-port usage (SysML validatePortDefinitionOwnedUsagesNotComposite).",
        typical_fix: "Declare the member as a referential usage with `ref`, or move it out of the port definition.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "port_nested_usage_composite",
        severity: "warning",
        meaning: "A port usage nests a composite non-port usage (SysML validatePortUsageNestedUsagesNotComposite).",
        typical_fix: "Declare the nested member as a referential usage with `ref`, or move it out of the port usage.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "control_node_incoming_multiplicity",
        severity: "warning",
        meaning: "An incoming succession to a control node (decide, merge, fork, join) authors a target end multiplicity other than 1..1 (SysML validateControlNodeIncomingSuccessions).",
        typical_fix: "Change the succession's target end multiplicity to `[1]`, or remove it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "control_node_outgoing_multiplicity",
        severity: "warning",
        meaning: "An outgoing succession from a control node (decide, merge, fork, join) authors a source end multiplicity other than 1..1 (SysML validateControlNodeOutgoingSuccessions).",
        typical_fix: "Change the succession's source end multiplicity to `[1]`, or remove it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "decision_node_multiple_incoming",
        severity: "warning",
        meaning: "A decision node has more than one incoming succession; reported at each succession after the first (SysML validateDecisionNodeIncomingSuccessions).",
        typical_fix: "Route the extra incoming successions through a merge node placed before the decision.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "decision_node_outgoing_multiplicity",
        severity: "warning",
        meaning: "An outgoing succession from a decision node authors a target end multiplicity other than 0..1 (SysML validateDecisionNodeOutgoingSuccessions).",
        typical_fix: "Change the succession's target end multiplicity to `[0..1]`, or remove it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "fork_node_multiple_incoming",
        severity: "warning",
        meaning: "A fork node has more than one incoming succession; reported at each succession after the first (SysML validateForkNodeIncomingSuccessions).",
        typical_fix: "Join or merge the extra incoming successions before the fork.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "join_node_multiple_outgoing",
        severity: "warning",
        meaning: "A join node has more than one outgoing succession; reported at each succession after the first (SysML validateJoinNodeOutgoingSuccessions).",
        typical_fix: "Follow the join with a fork node to fan out to several successors.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "merge_node_incoming_multiplicity",
        severity: "warning",
        meaning: "An incoming succession to a merge node authors a source end multiplicity other than 0..1 (SysML validateMergeNodeIncomingSuccessions).",
        typical_fix: "Change the succession's source end multiplicity to `[0..1]`, or remove it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "merge_node_multiple_outgoing",
        severity: "warning",
        meaning: "A merge node has more than one outgoing succession; reported at each succession after the first (SysML validateMergeNodeOutgoingSuccessions).",
        typical_fix: "Follow the merge with a fork or decision node to reach several successors.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "parallel_state_substate_transition",
        severity: "warning",
        meaning: "A parallel state definition or usage owns a transition or succession between its substates (SysML validateStateDefinitionParallelSubactions / validateStateUsageParallelSubactions).",
        typical_fix: "Remove the transition, or drop the `parallel` modifier from the state.",
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
        code: "view_expose_unresolved",
        severity: "warning",
        meaning: "A view exposes a target that does not resolve, so the view shows nothing for it.",
        typical_fix: "Fix the exposed name, or import the package that declares it.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "view_expose_empty",
        severity: "information",
        meaning: "A view declares members but exposes nothing, so it renders nothing.",
        typical_fix: "Add an `expose` member naming the elements the view should show.",
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
        meaning: "A multiplicity's literal upper bound is below its lower bound, so it admits no cardinality at all.",
        typical_fix: "Fix multiplicity syntax or bounds.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "multiplicity_bound_invalid",
        severity: "warning",
        meaning: "A multiplicity bound evaluates to a negative number, but bounds must be natural numbers (KerML validateMultiplicityRangeBoundResultTypes).",
        typical_fix: "Use a non-negative integer or `*` as the bound.",
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
    DiagnosticCatalogEntry {
        code: "missing_library_anchor",
        severity: "information",
        meaning: "The required standard-library anchor is not available in this publication.",
        typical_fix: "Admit the pinned standard library that provides the required language anchor.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "ambiguous_library_anchor",
        severity: "warning",
        meaning: "The required standard-library anchor is ambiguous in this publication.",
        typical_fix: "Ensure the admitted standard library provides exactly one required language anchor.",
        editor_quick_fixes: None,
    },
];

/// Diagnostics that reflect modeling/tooling guidance rather than normative SysML constraints.
const MODELING_GUIDANCE_CODES: &[&str] = &[
    "missing_final_state",
    "missing_initial_state",
    "missing_library_context",
    "multiple_final_states",
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
