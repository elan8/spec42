//! The publication's typed diagnostic contract.
//!
//! A diagnostic is a semantic fact about one publication, not a rendering of one. A consumer of
//! this contract -- the canonical S-expression adapter, the LSP host, the CLI validation report --
//! reads these values rather than recovering a code, severity, or outcome by parsing presentation
//! text, and never re-derives a rule the publication already settled.
//!
//! # Scope
//!
//! This contract is the complete production validation surface. Every family a host reports is
//! decided here, from facts settled before the publication becomes visible:
//!
//! - parser errors, carried through with the parser's own code;
//! - constructs this publication does not model ([`DiagnosticCode::UnsupportedPackageMember`] and
//!   the other `Unsupported*Member` codes);
//! - the settled outcome of every authored reference -- unresolved, ambiguous, unsupported, or
//!   non-converged;
//! - typed feature conformance: metaclass-family compatibility for typing, specialization and the
//!   subsetting family, specialization cycles, multiplicity and type conformance under
//!   redefinition and subsetting, KerML type-relationship cardinality, and the structural feature
//!   rules;
//! - expression conformance: the type of an authored value against the feature it is bound or
//!   assigned to, the resolution and dimension of an authored unit token, constraint bodies and
//!   view filters that settle to a non-Boolean constant, and calculation invocations that leave
//!   parameters unbound;
//! - name-collision, connection, behavior, requirement/case, view and inherited-value
//!   conformance ([`DiagnosticCode::DuplicateNamespaceMember`] onwards).
//!
//! # States stay distinguishable
//!
//! "No diagnostic" is the absence of an entry; an unresolved prerequisite, an ambiguous one, a rule
//! this publication does not support, a non-converged solve, and parser recovery are each their own
//! [`DiagnosticCode`], which is a closed enum so a consumer matches them exhaustively instead of
//! parsing a string. Publication completeness is a separate fact on [`PublishedDiagnostics`],
//! because a publication can be complete and still carry warnings.
//!
//! A rule whose operands are not settled reports nothing rather than reporting a failure: an
//! indeterminate conformance answer, an unresolved reference a later rule would have judged, and an
//! evaluation that did not run are each already their own published state.

use crate::{PublicationCompleteness, SymbolIdentity, TextRange};

/// The resolution-owned diagnostics of one publication, with the phase that produced them.
///
/// `completeness` travels with the diagnostics rather than being a separate lookup because the two
/// are only meaningful together: an empty slice from a complete publication means the model is
/// clean, while an empty slice from a non-converged one means nothing of the sort.
///
/// Only workspace-authored documents are reported. Library and standard-library sources are
/// admitted to the same semantic system, but their diagnostics are not the authoring surface this
/// contract describes, and reporting them would make every workspace inherit the library's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedDiagnostics {
    pub completeness: PublicationCompleteness,
    /// Canonically ordered: by document identity, then by range, then by code. The order is a
    /// property of the publication, not of traversal, storage, or scheduling.
    pub diagnostics: Box<[Diagnostic]>,
}

/// One published diagnostic.
///
/// `code` is both the stable public identifier consumers key on and the typed outcome: which
/// failure this is -- unresolved, ambiguous, an unsupported reference, an unsupported construct,
/// a non-converged solve -- is decided by matching it, never by reading text.
///
/// `message` is owner-produced. It exists so a host renders one sentence rather than inventing its
/// own from the code, and it is never a semantic input: no consumer may recover a fact from it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: DiagnosticSeverity,
    pub origin: DiagnosticOrigin,
    /// A stable owner-produced sentence. Presentation only.
    pub message: Box<str>,
    /// The element the diagnostic is about, where one exists.
    ///
    /// Absent for a parse error, an unsupported construct, and any other diagnostic whose subject
    /// is a span rather than a declaration this publication named.
    pub subject: Option<SymbolIdentity>,
    /// Where the diagnostic is reported. This is the authored site, not a definition it names.
    pub location: DiagnosticLocation,
    /// Further sites that explain the diagnostic, in canonical order.
    ///
    /// Ambiguity reports every candidate here. An empty slice means the diagnostic has no related
    /// site, never that the related sites were unavailable.
    pub related: Box<[RelatedLocation]>,
}

/// A document identity and range inside it.
///
/// Deliberately not [`crate::SourceLocation`]: that type also carries an
/// [`crate::OccurrenceRole`], and a parse error or an unsupported construct is neither a
/// declaration nor a reference. Reusing it would require inventing a role.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticLocation {
    pub document: Box<str>,
    pub range: TextRange,
}

/// One explanatory site of a diagnostic, with the owner's own note about why it is related.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelatedLocation {
    pub location: DiagnosticLocation,
    /// A stable owner-produced sentence. Presentation only.
    pub message: Box<str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    /// A fact worth surfacing that is not a fault: an unconnected port, a state machine with no
    /// finality indicator, a workspace with no library context.
    Information,
}

impl DiagnosticSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Information => "information",
        }
    }
}

/// Which owner decided the diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticOrigin {
    /// Reported by the parser contract and carried through unchanged.
    Parser,
    /// Decided by semantic construction or resolution.
    Semantic,
}

impl DiagnosticOrigin {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Parser => "parser",
            Self::Semantic => "semantic",
        }
    }
}

/// The stable public identifier of a diagnostic.
///
/// Codes are public behavior: consumers key suppression, documentation, and tests on them. The
/// mapping to text below is exhaustive and is the only place a code string is produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticCode {
    /// A code owned by the parser contract, mirrored verbatim.
    ///
    /// The parser may report an error without a code; the publication substitutes `parse_error`
    /// rather than omitting the field, so every diagnostic has one.
    Parser(Box<str>),

    UnsupportedPackageMember,
    UnsupportedPartDefinitionMember,
    UnsupportedPartUsageMember,
    UnsupportedAttributeMember,
    UnsupportedRequirementDefinitionMember,
    UnsupportedPortDefinitionMember,
    UnsupportedPortUsageMember,
    UnsupportedActionDefinitionMember,
    UnsupportedActionUsageMember,
    UnsupportedStateDefinitionMember,
    UnsupportedConnectionDefinitionMember,
    UnsupportedInterfaceDefinitionMember,
    UnsupportedViewDefinitionMember,
    UnsupportedConstraintDefinitionMember,
    UnsupportedCalcDefinitionMember,
    UnsupportedRenderingDefinitionMember,
    UnsupportedOccurrenceDefinitionMember,
    UnsupportedAnalysisCaseDefinitionMember,
    UnsupportedCaseDefinitionMember,
    UnsupportedVerificationCaseDefinitionMember,
    UnsupportedUseCaseDefinitionMember,
    UnsupportedReferenceUsageMember,
    UnsupportedRelationshipBodyMember,
    UnsupportedParserConstruct,

    UnresolvedTypeReference,
    UnresolvedSpecializesReference,
    UnresolvedImportTarget,
    UnresolvedReference,
    UnsupportedFilteredImport,
    UnsupportedReference,
    NonConvergedResolution,
    AmbiguousImportTarget,
    AmbiguousReference,

    /// A usage is typed by a definition of an incompatible metaclass family.
    IncompatibleTypeKind,
    /// A definition specializes a definition of an incompatible metaclass family.
    IncompatibleSpecializationKind,
    /// A usage subsets or redefines a feature of an incompatible metaclass family.
    IncompatibleSubsettingKind,
    /// A declaration reaches itself through specialization.
    SpecializationCycle,
    /// A redefining feature admits values its redefined feature's multiplicity excludes.
    RedefinitionMultiplicityWidened,
    /// A redefining feature's types do not conform to the redefined feature's.
    RedefinitionTypeIncompatible,
    /// A subsetting feature's types do not conform to the subsetted feature's.
    SubsettingTypeIncompatible,
    /// A flow payload is typed by something that is not an occurrence.
    FlowPayloadTypeNotOccurrence,
    /// A binary connection-like declaration has one end where it needs two.
    IncompleteConnectionLikeEndPair,
    /// A binary connection-like declaration has more than two ends.
    InvalidBinaryConnectionLikeEndCount,
    /// An end feature is derived, abstract or composite.
    EndFeatureInvalidRestrictions,
    /// A variant member's metaclass family is not the variation's.
    InvalidVariationMemberKind,
    /// A redefining feature is featured by a type unrelated to the redefined feature's.
    RedefinitionFeaturingTypeIncompatible,
    /// A feature redefines an end feature without being one.
    RedefinitionEndMismatch,
    /// A redefining feature's direction does not conform to the redefined feature's.
    RedefinitionDirectionMismatch,
    /// A non-unique feature subsets a unique one.
    SubsettingUniquenessMismatch,
    /// A type owns exactly one `unions`, `intersects` or `differences` operand.
    ///
    /// KerML requires zero or at least two: a union, intersection or difference of one type is
    /// that type, so a single operand states a generalization the author did not write.
    SingleTypeRelationshipOperand,

    /// A feature's authored value has a type unrelated to the feature's own.
    AttributeValueTypeIncompatible,
    /// An assignment's value has a type unrelated to the feature it assigns to.
    AssignmentValueIncompatible,
    /// A unit token names no unit in the admitted measurement catalog.
    UnknownUnitSymbol,
    /// A unit token names several admitted units, so it identifies none of them.
    AmbiguousUnitSymbol,
    /// A quantity value is measured in a unit whose dimension its feature's type does not admit.
    IncompatibleUnitDimension,
    /// A constraint's expression evaluates to something other than a Boolean.
    NonBooleanConstraintExpression,
    /// A view filter's condition evaluates to something other than a Boolean.
    NonBooleanViewFilter,
    /// A calculation invocation supplies fewer arguments than the callee has parameters to bind.
    CalculationArgumentsIncomplete,

    /// A package-level import filter settles to a non-Boolean constant.
    InvalidImportFilter,

    // --- Namespace identity ------------------------------------------------------------------
    /// A namespace declares two members whose names resolution cannot tell apart.
    DuplicateNamespaceMember,

    // --- Connection conformance ---------------------------------------------------------------
    /// A connector end resolves to something that is not a port.
    ConnectionEndpointNotPort,
    /// Two connected ports are typed by unrelated definitions.
    PortTypeMismatch,
    /// A declared port takes part in no connection.
    UnconnectedPort,
    /// Two connectors relate the same pair of ends.
    DuplicateConnection,
    /// Connected elements are neither ports nor a pair of structural parts.
    ConnectionContextInvalid,
    /// An interface end declares no port type.
    InterfaceEndInvalid,
    /// A binding connector binds two features with unrelated types.
    BindingConnectorIncompatible,

    // --- Behavior conformance -----------------------------------------------------------------
    /// A `perform` names something that is not an action.
    PerformTargetInvalidKind,
    /// A transition endpoint resolves to something that is not a state.
    TransitionEndpointInvalidState,
    /// A transition's ends belong to different state definitions.
    TransitionEndpointInvalidContext,
    /// An initial-state marker names something that is not a state.
    InitialStateInvalidTarget,
    /// A succession relates endpoints that are not actions.
    SuccessionEndpointInvalid,
    /// A transition guard settles to a non-Boolean constant.
    TransitionGuardNonBoolean,
    /// A state definition owns states but declares no initial transition.
    MissingInitialState,
    /// A state definition owns states but declares no finality indicator.
    MissingFinalState,
    /// A state definition declares more than one explicit final state.
    MultipleFinalStates,
    /// An `accept` payload is typed by something that cannot be an action payload.
    AcceptPayloadIncompatible,

    // --- Requirement and case conformance ------------------------------------------------------
    /// A declaration owns more than one member of a role that admits one.
    DuplicateRoleMember,
    /// A subject member is preceded by another input role member.
    SubjectMemberNotFirst,
    /// A satisfy relationship relates endpoints of incompatible kinds.
    SatisfyInvalidEndpointKind,
    /// A `verify` target does not resolve to a requirement.
    VerifiedRequirementInvalidTarget,
    /// An `include` target does not resolve to a use case.
    UseCaseIncludeInvalidTarget,

    // --- View conformance ----------------------------------------------------------------------
    /// A view satisfies something that is not a viewpoint.
    ViewpointConformanceInvalidTargetKind,
    /// A view usage is typed by a definition outside the SysML standard view catalog.
    ViewTypeNonStandard,
    /// A rendering member is typed by something that is not a rendering definition.
    ViewRenderingInvalidTarget,
    /// A textual representation declares no language identifier.
    ViewpointRepLanguageUnresolved,

    // --- Allocation ----------------------------------------------------------------------------
    /// An `allocate` statement declares only one of its two endpoints.
    InvalidAllocationEndpoints,

    // --- Inherited values ----------------------------------------------------------------------
    /// A feature overrides an inherited member without writing `:>>`.
    ImplicitRedefinitionWithoutOperator,
    /// A feature's authored value is a string where the member it inherits is enumerated.
    InheritedAttributeValueTypeMismatch,

    // --- Multiplicity ---------------------------------------------------------------------------
    /// A declared multiplicity states bounds that admit nothing.
    InvalidMultiplicity,

    // --- Analysis ------------------------------------------------------------------------------
    /// An analysis constraint settled to false.
    AnalysisConstraintFailed,
    /// An analysis constraint could not be evaluated.
    AnalysisEvaluationUnresolved,

    // --- Authoring hints -------------------------------------------------------------------------
    /// A part usage declares no type.
    UntypedPartUsage,
    /// A workspace document imports names it cannot resolve and no library source was admitted.
    MissingLibraryContext,
}

impl DiagnosticCode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Parser(code) => code,
            Self::UnsupportedPackageMember => "unsupported_package_member",
            Self::UnsupportedPartDefinitionMember => "unsupported_part_definition_member",
            Self::UnsupportedPartUsageMember => "unsupported_part_usage_member",
            Self::UnsupportedAttributeMember => "unsupported_attribute_member",
            Self::UnsupportedRequirementDefinitionMember => {
                "unsupported_requirement_definition_member"
            }
            Self::UnsupportedPortDefinitionMember => "unsupported_port_definition_member",
            Self::UnsupportedPortUsageMember => "unsupported_port_usage_member",
            Self::UnsupportedActionDefinitionMember => "unsupported_action_definition_member",
            Self::UnsupportedActionUsageMember => "unsupported_action_usage_member",
            Self::UnsupportedStateDefinitionMember => "unsupported_state_definition_member",
            Self::UnsupportedConnectionDefinitionMember => {
                "unsupported_connection_definition_member"
            }
            Self::UnsupportedInterfaceDefinitionMember => "unsupported_interface_definition_member",
            Self::UnsupportedViewDefinitionMember => "unsupported_view_definition_member",
            Self::UnsupportedConstraintDefinitionMember => {
                "unsupported_constraint_definition_member"
            }
            Self::UnsupportedCalcDefinitionMember => "unsupported_calc_definition_member",
            Self::UnsupportedRenderingDefinitionMember => "unsupported_rendering_definition_member",
            Self::UnsupportedOccurrenceDefinitionMember => {
                "unsupported_occurrence_definition_member"
            }
            Self::UnsupportedAnalysisCaseDefinitionMember => {
                "unsupported_analysis_case_definition_member"
            }
            Self::UnsupportedCaseDefinitionMember => "unsupported_case_definition_member",
            Self::UnsupportedVerificationCaseDefinitionMember => {
                "unsupported_verification_case_definition_member"
            }
            Self::UnsupportedUseCaseDefinitionMember => "unsupported_use_case_definition_member",
            Self::UnsupportedReferenceUsageMember => "unsupported_reference_usage_member",
            Self::UnsupportedRelationshipBodyMember => "unsupported_relationship_body_member",
            Self::UnsupportedParserConstruct => "unsupported_parser_construct",
            Self::UnresolvedTypeReference => "unresolved_type_reference",
            Self::UnresolvedSpecializesReference => "unresolved_specializes_reference",
            Self::UnresolvedImportTarget => "unresolved_import_target",
            Self::UnresolvedReference => "unresolved_reference",
            Self::UnsupportedFilteredImport => "unsupported_filtered_import",
            Self::UnsupportedReference => "unsupported_reference",
            Self::NonConvergedResolution => "non_converged_resolution",
            Self::AmbiguousImportTarget => "ambiguous_import_target",
            Self::AmbiguousReference => "ambiguous_reference",
            Self::IncompatibleTypeKind => "incompatible_type_kind",
            Self::IncompatibleSpecializationKind => "incompatible_specializes_kind",
            Self::IncompatibleSubsettingKind => "incompatible_subset_redefine_kind",
            Self::SpecializationCycle => "specialization_cycle",
            Self::RedefinitionMultiplicityWidened => "redefinition_multiplicity_widened",
            Self::RedefinitionTypeIncompatible => "redefinition_type_incompatible",
            Self::SubsettingTypeIncompatible => "subsetting_type_incompatible",
            Self::FlowPayloadTypeNotOccurrence => "flow_payload_type_not_occurrence",
            Self::IncompleteConnectionLikeEndPair => "incomplete_connection_like_end_pair",
            Self::InvalidBinaryConnectionLikeEndCount => "invalid_binary_connection_like_end_count",
            Self::EndFeatureInvalidRestrictions => "end_feature_invalid_restrictions",
            Self::InvalidVariationMemberKind => "invalid_variation_member_kind",
            Self::RedefinitionFeaturingTypeIncompatible => {
                "redefinition_featuring_type_incompatible"
            }
            Self::RedefinitionEndMismatch => "redefinition_end_mismatch",
            Self::RedefinitionDirectionMismatch => "redefinition_direction_mismatch",
            Self::SubsettingUniquenessMismatch => "subsetting_uniqueness_mismatch",
            Self::SingleTypeRelationshipOperand => "single_type_relationship_operand",
            Self::AttributeValueTypeIncompatible => "attribute_value_type_mismatch",
            Self::AssignmentValueIncompatible => "assignment_value_incompatible",
            Self::UnknownUnitSymbol => "unknown_unit_symbol",
            Self::AmbiguousUnitSymbol => "ambiguous_unit_symbol",
            Self::IncompatibleUnitDimension => "incompatible_unit_dimension",
            Self::NonBooleanConstraintExpression => "non_boolean_expression",
            Self::NonBooleanViewFilter => "view_filter_non_boolean",
            Self::CalculationArgumentsIncomplete => "calculation_binding_mismatch",
            Self::InvalidImportFilter => "invalid_import_filter",
            Self::DuplicateNamespaceMember => "duplicate_namespace_member",
            Self::ConnectionEndpointNotPort => "connection_endpoint_not_port",
            Self::PortTypeMismatch => "port_type_mismatch",
            Self::UnconnectedPort => "unconnected_port",
            Self::DuplicateConnection => "duplicate_connection",
            Self::ConnectionContextInvalid => "connection_context_invalid",
            Self::InterfaceEndInvalid => "interface_end_invalid",
            Self::BindingConnectorIncompatible => "binding_connector_incompatible",
            Self::PerformTargetInvalidKind => "perform_target_invalid_kind",
            Self::TransitionEndpointInvalidState => "transition_endpoint_invalid_state",
            Self::TransitionEndpointInvalidContext => "transition_endpoint_invalid_context",
            Self::InitialStateInvalidTarget => "initial_state_invalid_target",
            Self::SuccessionEndpointInvalid => "succession_endpoint_invalid",
            Self::TransitionGuardNonBoolean => "transition_guard_non_boolean",
            Self::MissingInitialState => "missing_initial_state",
            Self::MissingFinalState => "missing_final_state",
            Self::MultipleFinalStates => "multiple_final_states",
            Self::AcceptPayloadIncompatible => "accept_payload_incompatible",
            Self::DuplicateRoleMember => "duplicate_role_member",
            Self::SubjectMemberNotFirst => "subject_member_not_first",
            Self::SatisfyInvalidEndpointKind => "satisfy_invalid_endpoint_kind",
            Self::VerifiedRequirementInvalidTarget => "verified_requirement_invalid_target",
            Self::UseCaseIncludeInvalidTarget => "use_case_include_invalid_target",
            Self::ViewpointConformanceInvalidTargetKind => {
                "viewpoint_conformance_invalid_target_kind"
            }
            Self::ViewTypeNonStandard => "view_type_non_standard",
            Self::ViewRenderingInvalidTarget => "view_rendering_invalid_target",
            Self::ViewpointRepLanguageUnresolved => "viewpoint_rep_language_unresolved",
            Self::InvalidAllocationEndpoints => "invalid_allocation_endpoints",
            Self::ImplicitRedefinitionWithoutOperator => "implicit_redefinition_without_operator",
            Self::InheritedAttributeValueTypeMismatch => "inherited_attribute_value_type_mismatch",
            Self::InvalidMultiplicity => "invalid_multiplicity",
            Self::AnalysisConstraintFailed => "analysis_constraint_failed",
            Self::AnalysisEvaluationUnresolved => "analysis_evaluation_unresolved",
            Self::UntypedPartUsage => "untyped_part_usage",
            Self::MissingLibraryContext => "missing_library_context",
        }
    }
}

impl DiagnosticCode {
    /// The owner's sentence for this code.
    ///
    /// One sentence per code, stated once. A host renders this beside [`DiagnosticCode::as_str`]
    /// rather than composing its own text from the code, and no consumer may read a fact back out
    /// of it: everything a rule decided is already a typed field.
    ///
    /// Rules whose subject is not visible at the reported range -- a namespace that declares a
    /// duplicate member, a state definition that declares two initial transitions -- build a
    /// message naming it instead of using this default.
    pub fn describe(&self) -> &str {
        match self {
            Self::Parser(_) => "The parser reported an error here.",
            Self::UnsupportedPackageMember
            | Self::UnsupportedPartDefinitionMember
            | Self::UnsupportedPartUsageMember
            | Self::UnsupportedAttributeMember
            | Self::UnsupportedRequirementDefinitionMember
            | Self::UnsupportedPortDefinitionMember
            | Self::UnsupportedPortUsageMember
            | Self::UnsupportedActionDefinitionMember
            | Self::UnsupportedActionUsageMember
            | Self::UnsupportedStateDefinitionMember
            | Self::UnsupportedConnectionDefinitionMember
            | Self::UnsupportedInterfaceDefinitionMember
            | Self::UnsupportedViewDefinitionMember
            | Self::UnsupportedConstraintDefinitionMember
            | Self::UnsupportedCalcDefinitionMember
            | Self::UnsupportedRenderingDefinitionMember
            | Self::UnsupportedOccurrenceDefinitionMember
            | Self::UnsupportedAnalysisCaseDefinitionMember
            | Self::UnsupportedCaseDefinitionMember
            | Self::UnsupportedVerificationCaseDefinitionMember
            | Self::UnsupportedUseCaseDefinitionMember
            | Self::UnsupportedReferenceUsageMember
            | Self::UnsupportedRelationshipBodyMember => {
                "This member is parsed but not modelled by the semantic publication."
            }
            Self::UnsupportedParserConstruct => {
                "This construct is parsed but not modelled by the semantic publication."
            }
            Self::UnresolvedTypeReference => "This type reference does not resolve.",
            Self::UnresolvedSpecializesReference => "This specialization target does not resolve.",
            Self::UnresolvedImportTarget => "This import target does not resolve.",
            Self::UnresolvedReference => "This reference does not resolve.",
            Self::UnsupportedFilteredImport => {
                "Filtered namespace imports are parsed but not semantically supported."
            }
            Self::UnsupportedReference => {
                "This reference form is parsed but not semantically supported."
            }
            Self::NonConvergedResolution => {
                "Resolution did not converge, so this reference has no settled outcome."
            }
            Self::AmbiguousImportTarget => {
                "This import target names several elements, so it identifies none of them."
            }
            Self::AmbiguousReference => {
                "This reference names several elements, so it identifies none of them."
            }
            Self::IncompatibleTypeKind => {
                "This usage is typed by a definition of an incompatible kind."
            }
            Self::IncompatibleSpecializationKind => {
                "This definition specializes a definition of an incompatible kind."
            }
            Self::IncompatibleSubsettingKind => {
                "This usage subsets or redefines a feature of an incompatible kind."
            }
            Self::SpecializationCycle => "This declaration reaches itself through specialization.",
            Self::RedefinitionMultiplicityWidened => {
                "This redefinition admits values the redefined feature's multiplicity excludes."
            }
            Self::RedefinitionTypeIncompatible => {
                "This redefinition's types do not conform to the redefined feature's."
            }
            Self::SubsettingTypeIncompatible => {
                "This feature's types do not conform to the feature it subsets."
            }
            Self::FlowPayloadTypeNotOccurrence => "A flow payload must be typed by an occurrence.",
            Self::IncompleteConnectionLikeEndPair => {
                "This connection-like definition declares one end where it needs at least two."
            }
            Self::InvalidBinaryConnectionLikeEndCount => {
                "This binary connection-like definition declares more than two ends."
            }
            Self::EndFeatureInvalidRestrictions => {
                "An end feature must not be derived, abstract or composite."
            }
            Self::InvalidVariationMemberKind => {
                "A variant member must use the variation's own kind."
            }
            Self::RedefinitionFeaturingTypeIncompatible => {
                "A redefinition must be introduced by the redefined feature's featuring type or a \
                 specialization of it."
            }
            Self::RedefinitionEndMismatch => "Redefining an end feature requires an end feature.",
            Self::RedefinitionDirectionMismatch => {
                "This redefinition's direction does not conform to the redefined feature's."
            }
            Self::SubsettingUniquenessMismatch => {
                "A non-unique feature cannot subset a unique one."
            }
            Self::SingleTypeRelationshipOperand => {
                "A union, intersection or difference needs zero or at least two operands."
            }
            Self::AttributeValueTypeIncompatible => {
                "This value's type is unrelated to the feature it is bound to."
            }
            Self::AssignmentValueIncompatible => {
                "This value's type is unrelated to the feature it is assigned to."
            }
            Self::UnknownUnitSymbol => {
                "This unit token names no unit in the admitted measurement catalog."
            }
            Self::AmbiguousUnitSymbol => {
                "This unit token names several admitted units, so it identifies none of them."
            }
            Self::IncompatibleUnitDimension => {
                "This value's unit has a dimension the feature's type does not admit."
            }
            Self::NonBooleanConstraintExpression => {
                "A constraint expression must evaluate to a Boolean."
            }
            Self::NonBooleanViewFilter => "A view filter condition must evaluate to a Boolean.",
            Self::CalculationArgumentsIncomplete => {
                "This invocation supplies fewer arguments than the calculation has parameters."
            }
            Self::InvalidImportFilter => "An import filter condition must evaluate to a Boolean.",
            Self::DuplicateNamespaceMember => {
                "A namespace must not declare two members resolution cannot tell apart."
            }
            Self::ConnectionEndpointNotPort => "This connector end does not resolve to a port.",
            Self::PortTypeMismatch => "These connected ports are typed by unrelated definitions.",
            Self::UnconnectedPort => "This port takes part in no connection.",
            Self::DuplicateConnection => "This connector repeats an existing pair of ends.",
            Self::ConnectionContextInvalid => {
                "Connected elements must be ports, or a pair of structural parts."
            }
            Self::InterfaceEndInvalid => "An interface end must declare a port type.",
            Self::BindingConnectorIncompatible => {
                "This binding connector binds two features with unrelated types."
            }
            Self::PerformTargetInvalidKind => {
                "A performed behavior must resolve to an action definition or usage."
            }
            Self::TransitionEndpointInvalidState => {
                "A transition endpoint must resolve to a state."
            }
            Self::TransitionEndpointInvalidContext => {
                "A transition's source and target must belong to the same state definition."
            }
            Self::InitialStateInvalidTarget => "An initial-state marker must name a state.",
            Self::SuccessionEndpointInvalid => {
                "A succession must relate action definitions or usages."
            }
            Self::TransitionGuardNonBoolean => "A transition guard must evaluate to a Boolean.",
            Self::MissingInitialState => {
                "This state definition owns states but declares no initial transition."
            }
            Self::MissingFinalState => {
                "This state definition owns states but declares no finality indicator."
            }
            Self::MultipleFinalStates => {
                "A state definition declares more than one explicit final state."
            }
            Self::AcceptPayloadIncompatible => {
                "An accept payload must be typed by an item, part, attribute or occurrence."
            }
            Self::DuplicateRoleMember => {
                "This declaration owns more than one member of a role that admits one."
            }
            Self::SubjectMemberNotFirst => {
                "A subject member must precede the other input role members."
            }
            Self::SatisfyInvalidEndpointKind => {
                "This satisfy relationship relates endpoints of incompatible kinds."
            }
            Self::VerifiedRequirementInvalidTarget => {
                "A verify target must resolve to a requirement definition or usage."
            }
            Self::UseCaseIncludeInvalidTarget => {
                "An include target must resolve to a use case definition or usage."
            }
            Self::ViewpointConformanceInvalidTargetKind => {
                "A view must satisfy a viewpoint definition or usage."
            }
            Self::ViewTypeNonStandard => {
                "This view is typed by a definition outside the SysML standard view catalog."
            }
            Self::ViewRenderingInvalidTarget => {
                "A rendering member must be typed by a rendering definition or usage."
            }
            Self::ViewpointRepLanguageUnresolved => {
                "A textual representation must declare a language identifier."
            }
            Self::InvalidAllocationEndpoints => {
                "An allocate statement must declare both a source and a target."
            }
            Self::ImplicitRedefinitionWithoutOperator => {
                "This feature overrides an inherited member without the explicit ':>>' operator."
            }
            Self::InheritedAttributeValueTypeMismatch => {
                "This value is a string literal where the inherited member is enumerated."
            }
            Self::InvalidMultiplicity => "This multiplicity states bounds that admit nothing.",
            Self::AnalysisConstraintFailed => "This analysis constraint evaluated to false.",
            Self::AnalysisEvaluationUnresolved => {
                "This analysis expression could not be evaluated."
            }
            Self::UntypedPartUsage => "This part usage declares no type.",
            Self::MissingLibraryContext => {
                "This document imports names that do not resolve and no library source was \
                 admitted to the publication."
            }
        }
    }
}

/// The code a parser error carries when the parser reports none.
pub(crate) const UNCODED_PARSE_ERROR: &str = "parse_error";
