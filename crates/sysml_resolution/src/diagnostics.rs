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

use crate::{PublicationCompleteness, SymbolId, TextRange};

pub use sysml_contract::{DiagnosticCategory, DiagnosticOrigin, DiagnosticSeverity};

/// The resolution-owned diagnostics of one publication, with the phase that produced them.
///
/// `completeness` travels with the diagnostics rather than being a separate lookup because the two
/// are only meaningful together: an empty slice from a complete publication means the model is
/// clean, while an empty slice from a non-converged one means nothing of the sort.
///
/// Only workspace-authored documents are reported. Library and standard-library sources are
/// admitted to the same semantic system, but their diagnostics are not the authoring surface this
/// contract describes, and reporting them would make every workspace inherit the library's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublishedDiagnostics<'m> {
    pub completeness: PublicationCompleteness,
    /// Canonically ordered: by document identity, then by range, then by code. The order is a
    /// property of the publication, not of traversal, storage, or scheduling.
    ///
    /// A slice of the settled sequence, never a copy of it: asking a publication for its
    /// diagnostics costs a bounds check, not one allocated message, document identity and related
    /// site per entry.
    diagnostics: &'m [Diagnostic],
}

impl<'m> PublishedDiagnostics<'m> {
    pub(crate) fn new(
        completeness: PublicationCompleteness,
        diagnostics: &'m [Diagnostic],
    ) -> Self {
        Self {
            completeness,
            diagnostics,
        }
    }

    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&'m Diagnostic> {
        self.diagnostics.get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &'m Diagnostic> + 'm {
        self.diagnostics.iter()
    }
}

impl<'m> IntoIterator for PublishedDiagnostics<'m> {
    type Item = &'m Diagnostic;
    type IntoIter = std::slice::Iter<'m, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.iter()
    }
}

/// One published diagnostic.
///
/// `code` is both the stable public identifier consumers key on and the typed outcome: which
/// failure this is -- unresolved, ambiguous, an unsupported reference, an unsupported construct,
/// a non-converged solve -- is decided by matching it, never by reading text. [`DiagnosticCode`]
/// also owns its neutral [`DiagnosticCategory`], so every surface reads the same classification
/// without grouping codes, messages, or rendered output itself.
///
/// `message` is owner-produced. It exists so a host renders one sentence rather than inventing its
/// own from the code, and it is never a semantic input: no consumer may recover a fact from it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub(crate) code: DiagnosticCode,
    pub(crate) severity: DiagnosticSeverity,
    pub(crate) origin: DiagnosticOrigin,
    /// A stable owner-produced sentence. Presentation only.
    pub(crate) message: Box<str>,
    /// Machine-readable context owned by the rule that produced this diagnostic.
    ///
    /// Consumers use this for actions; presentation text remains presentation-only.
    pub(crate) payload: Option<DiagnosticPayload>,
    /// The element the diagnostic is about, where one exists.
    ///
    /// Absent for a parse error, an unsupported construct, and any other diagnostic whose subject
    /// is a span rather than a declaration this publication named.
    pub(crate) subject: Option<SymbolId>,
    /// Where the diagnostic is reported. This is the authored site, not a definition it names.
    pub(crate) location: DiagnosticLocation,
    /// Further sites that explain the diagnostic, in canonical order.
    ///
    /// Ambiguity reports every candidate here. An empty slice means the diagnostic has no related
    /// site, never that the related sites were unavailable.
    pub(crate) related: Box<[RelatedLocation]>,
}

impl Diagnostic {
    /// The stable public identifier and typed outcome of this diagnostic.
    pub fn code(&self) -> &DiagnosticCode {
        &self.code
    }

    pub fn severity(&self) -> DiagnosticSeverity {
        self.severity
    }

    pub fn origin(&self) -> DiagnosticOrigin {
        self.origin
    }

    /// The owner-produced sentence, borrowed from the settled diagnostic.
    ///
    /// Presentation only, and never a semantic input: no consumer may recover a fact from it.
    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn payload(&self) -> Option<&DiagnosticPayload> {
        self.payload.as_ref()
    }

    /// The element the diagnostic is about, where one exists.
    pub fn subject(&self) -> Option<SymbolId> {
        self.subject
    }

    /// Where the diagnostic is reported: the authored site, not a definition it names.
    pub fn location(&self) -> &DiagnosticLocation {
        &self.location
    }

    /// Further sites that explain the diagnostic, in canonical order.
    pub fn related(&self) -> impl Iterator<Item = &RelatedLocation> + '_ {
        self.related.iter()
    }

    /// The number of related sites, without walking them.
    pub fn related_len(&self) -> usize {
        self.related.len()
    }

    /// The neutral category settled by the diagnostic's owning code declaration.
    ///
    /// Keeping this as an accessor prevents a second, mutable category store from drifting away
    /// from the code declaration while presenting one typed diagnostic contract to consumers.
    pub fn category(&self) -> DiagnosticCategory {
        self.code.category()
    }
}

/// Typed context a consumer may act on without parsing diagnostic prose.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticPayload {
    /// The authored path whose resolution failed.
    UnresolvedReference { authored_target: Box<str> },
}

impl DiagnosticPayload {
    pub fn unresolved_reference_target(&self) -> &str {
        match self {
            Self::UnresolvedReference { authored_target } => authored_target,
        }
    }
}

/// A document identity and range inside it.
///
/// Deliberately not [`crate::SourceLocation`]: that type also carries an
/// [`crate::OccurrenceRole`], and a parse error or an unsupported construct is neither a
/// declaration nor a reference. Reusing it would require inventing a role.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticLocation {
    pub(crate) document: Box<str>,
    pub(crate) range: TextRange,
}

impl DiagnosticLocation {
    /// The document identity, borrowed from the settled diagnostic.
    pub fn document(&self) -> &str {
        &self.document
    }

    pub fn range(&self) -> TextRange {
        self.range
    }
}

/// One explanatory site of a diagnostic, with the owner's own note about why it is related.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelatedLocation {
    pub(crate) location: DiagnosticLocation,
    /// A stable owner-produced sentence. Presentation only.
    pub(crate) message: Box<str>,
}

impl RelatedLocation {
    pub fn location(&self) -> &DiagnosticLocation {
        &self.location
    }

    /// A stable owner-produced sentence. Presentation only.
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Declares every code this publication itself can report, once.
///
/// The enum, the stable code string, neutral category, owner's sentence, and exhaustive list a
/// consumer enumerates all come from this one table, so a new code cannot be added to one and
/// forgotten in another. `Parser` is deliberately outside it: the parser owns its code and text,
/// while the boundary maps its typed category.
macro_rules! semantic_diagnostic_codes {
    ($( $category:ident { $( $(#[$meta:meta])* $variant:ident => $code:literal, $describe:expr; )* } )*) => {
        /// The stable public identifier of a diagnostic.
        ///
        /// Codes are public behavior: consumers key suppression, documentation, and tests on them.
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum DiagnosticCode {
            /// A code owned by the parser contract, mirrored verbatim.
            ///
            /// The parser may report an error without a code; the publication substitutes
            /// `parse_error` rather than omitting the field, so every diagnostic has one.
            Parser {
                code: Box<str>,
                category: DiagnosticCategory,
            },
            $( $( $(#[$meta])* $variant, )* )*
        }

        impl DiagnosticCode {
            /// Every code the publication itself can report, in declaration order.
            ///
            /// A consumer that documents or classifies codes enumerates this rather than keeping
            /// its own list, which is what makes drift a compile or test failure instead of a
            /// silently undocumented diagnostic.
            pub const SEMANTIC: &'static [DiagnosticCode] = &[ $( $( DiagnosticCode::$variant, )* )* ];

            /// The stable code string. The only place one is produced.
            pub fn as_str(&self) -> &str {
                match self {
                    Self::Parser { code, .. } => code,
                    $( $( Self::$variant => $code, )* )*
                }
            }

            /// The neutral outcome category published with this diagnostic.
            pub fn category(&self) -> DiagnosticCategory {
                match self {
                    Self::Parser { category, .. } => *category,
                    $( $( Self::$variant => DiagnosticCategory::$category, )* )*
                }
            }

            /// The owner's sentence for this code.
            ///
            /// A host renders this beside [`DiagnosticCode::as_str`] rather than composing its own
            /// text, and no consumer may read a fact back out of it: everything a rule decided is
            /// already a typed field. Rules whose subject is not visible at the reported range
            /// build a message naming it instead of using this default.
            pub fn describe(&self) -> &str {
                match self {
                    Self::Parser { .. } => "The parser reported an error here.",
                    $( $( Self::$variant => $describe, )* )*
                }
            }
        }
    };
}

semantic_diagnostic_codes! {

    UnsupportedSemantics {
    UnsupportedPackageMember => "unsupported_package_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedPartDefinitionMember => "unsupported_part_definition_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedPartUsageMember => "unsupported_part_usage_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedAttributeMember => "unsupported_attribute_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedRequirementDefinitionMember => "unsupported_requirement_definition_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedPortDefinitionMember => "unsupported_port_definition_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedPortUsageMember => "unsupported_port_usage_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedActionDefinitionMember => "unsupported_action_definition_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedActionUsageMember => "unsupported_action_usage_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedStateDefinitionMember => "unsupported_state_definition_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedConnectionDefinitionMember => "unsupported_connection_definition_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedInterfaceDefinitionMember => "unsupported_interface_definition_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedViewDefinitionMember => "unsupported_view_definition_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedConstraintDefinitionMember => "unsupported_constraint_definition_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedCalcDefinitionMember => "unsupported_calc_definition_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedRenderingDefinitionMember => "unsupported_rendering_definition_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedOccurrenceDefinitionMember => "unsupported_occurrence_definition_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedAnalysisCaseDefinitionMember => "unsupported_analysis_case_definition_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedCaseDefinitionMember => "unsupported_case_definition_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedVerificationCaseDefinitionMember => "unsupported_verification_case_definition_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedUseCaseDefinitionMember => "unsupported_use_case_definition_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedReferenceUsageMember => "unsupported_reference_usage_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedRelationshipBodyMember => "unsupported_relationship_body_member",
        "This member is parsed but not modelled by the semantic publication.";
    UnsupportedParserConstruct => "unsupported_parser_construct",
        "This construct is parsed but not modelled by the semantic publication.";
    }

    Unresolved {
    UnresolvedTypeReference => "unresolved_type_reference",
        "This type reference does not resolve.";
    UnresolvedSpecializesReference => "unresolved_specializes_reference",
        "This specialization target does not resolve.";
    UnresolvedImportTarget => "unresolved_import_target",
        "This import target does not resolve.";
    UnresolvedReference => "unresolved_reference",
        "This reference does not resolve.";
    }
    UnsupportedSemantics {
    UnsupportedFilteredImport => "unsupported_filtered_import",
        "Filtered namespace imports are parsed but not semantically supported.";
    UnsupportedReference => "unsupported_reference",
        "This reference form is parsed but not semantically supported.";
    }
    NonConverged {
    NonConvergedResolution => "non_converged_resolution",
        "Resolution did not converge, so this reference has no settled outcome.";
    }
    Ambiguous {
    AmbiguousImportTarget => "ambiguous_import_target",
        "This import target names several elements, so it identifies none of them.";
    AmbiguousReference => "ambiguous_reference",
        "This reference names several elements, so it identifies none of them.";
    }

    Validation {
    /// A usage is typed by a definition of an incompatible metaclass family.
    IncompatibleTypeKind => "incompatible_type_kind",
        "This usage is typed by a definition of an incompatible kind.";
    /// A definition specializes a definition of an incompatible metaclass family.
    IncompatibleSpecializationKind => "incompatible_specializes_kind",
        "This definition specializes a definition of an incompatible kind.";
    /// A usage subsets or redefines a feature of an incompatible metaclass family.
    IncompatibleSubsettingKind => "incompatible_subset_redefine_kind",
        "This usage subsets or redefines a feature of an incompatible kind.";
    /// A declaration reaches itself through specialization.
    SpecializationCycle => "specialization_cycle",
        "This declaration reaches itself through specialization.";
    /// A redefining feature admits values its redefined feature's multiplicity excludes.
    RedefinitionMultiplicityWidened => "redefinition_multiplicity_widened",
        "This redefinition admits values the redefined feature's multiplicity excludes.";
    /// A redefining feature's types do not conform to the redefined feature's.
    RedefinitionTypeIncompatible => "redefinition_type_incompatible",
        "This redefinition's types do not conform to the redefined feature's.";
    /// A subsetting feature's types do not conform to the subsetted feature's.
    SubsettingTypeIncompatible => "subsetting_type_incompatible",
        "This feature's types do not conform to the feature it subsets.";
    /// A flow payload is typed by something that is not an occurrence.
    FlowPayloadTypeNotOccurrence => "flow_payload_type_not_occurrence",
        "A flow payload must be typed by an occurrence.";
    /// A binary connection-like declaration has one end where it needs two.
    IncompleteConnectionLikeEndPair => "incomplete_connection_like_end_pair",
        "This connection-like definition declares one end where it needs at least two.";
    /// A binary connection-like declaration has more than two ends.
    InvalidBinaryConnectionLikeEndCount => "invalid_binary_connection_like_end_count",
        "This binary connection-like definition declares more than two ends.";
    /// An end feature is derived, abstract, composite or portion.
    EndFeatureInvalidRestrictions => "end_feature_invalid_restrictions",
        "An end feature must not be derived, abstract, composite or portion.";
    /// An end feature has an authored direction.
    EndFeatureHasDirection => "end_feature_has_direction",
        "An end feature must not have a direction.";
    /// A feature owned by a metadata feature does not redefine a feature of the metaclass.
    MetadataBodyFeatureInvalid => "metadata_body_feature_invalid",
        "A feature owned by a metadata feature must redefine a feature of its metaclass.";
    /// A non-port usage owned by a port definition is composite.
    PortOwnedUsageComposite => "port_owned_usage_composite",
        "A usage owned by a port definition must not be composite unless it is a port.";
    /// A non-port usage nested in a port usage is composite.
    PortNestedUsageComposite => "port_nested_usage_composite",
        "A usage nested in a port usage must not be composite unless it is a port.";
    /// A parallel state owns a transition or succession between its substates.
    ParallelStateSubstateTransition => "parallel_state_substate_transition",
        "A parallel state must not own transitions or successions between its substates.";
    /// A variant member's metaclass family is not the variation's.
    InvalidVariationMemberKind => "invalid_variation_member_kind",
        "A variant member must use the variation's own kind.";
    /// A redefining feature is featured by a type unrelated to the redefined feature's.
    RedefinitionFeaturingTypeIncompatible => "redefinition_featuring_type_incompatible",
        "A redefinition must be introduced by the redefined feature's featuring type or a \
                         specialization of it.";
    /// A feature redefines an end feature without being one.
    RedefinitionEndMismatch => "redefinition_end_mismatch",
        "Redefining an end feature requires an end feature.";
    /// A redefining feature's direction does not conform to the redefined feature's.
    RedefinitionDirectionMismatch => "redefinition_direction_mismatch",
        "This redefinition's direction does not conform to the redefined feature's.";
    /// A non-unique feature subsets a unique one.
    SubsettingUniquenessMismatch => "subsetting_uniqueness_mismatch",
        "A non-unique feature cannot subset a unique one.";
    /// A type owns exactly one `unions`, `intersects` or `differences` operand.
    ///
    /// KerML requires zero or at least two: a union, intersection or difference of one type is
    /// that type, so a single operand states a generalization the author did not write.
    SingleTypeRelationshipOperand => "single_type_relationship_operand",
        "A union, intersection or difference needs zero or at least two operands.";

    /// A feature's authored value has a type unrelated to the feature's own.
    AttributeValueTypeIncompatible => "attribute_value_type_mismatch",
        "This value's type is unrelated to the feature it is bound to.";
    /// An assignment's value has a type unrelated to the feature it assigns to.
    AssignmentValueIncompatible => "assignment_value_incompatible",
        "This value's type is unrelated to the feature it is assigned to.";
    /// A unit token names no unit in the admitted measurement catalog.
    UnknownUnitSymbol => "unknown_unit_symbol",
        "This unit token names no unit in the admitted measurement catalog.";
    /// A unit token names several admitted units, so it identifies none of them.
    AmbiguousUnitSymbol => "ambiguous_unit_symbol",
        "This unit token names several admitted units, so it identifies none of them.";
    /// A quantity value is measured in a unit whose dimension its feature's type does not admit.
    IncompatibleUnitDimension => "incompatible_unit_dimension",
        "This value's unit has a dimension the feature's type does not admit.";
    /// A constraint's expression evaluates to something other than a Boolean.
    NonBooleanConstraintExpression => "non_boolean_expression",
        "A constraint expression must evaluate to a Boolean.";
    /// A view filter's condition evaluates to something other than a Boolean.
    NonBooleanViewFilter => "view_filter_non_boolean",
        "A view filter condition must evaluate to a Boolean.";
    /// A calculation invocation supplies fewer arguments than the callee has parameters to bind.
    CalculationArgumentsIncomplete => "calculation_binding_mismatch",
        "This invocation supplies fewer arguments than the calculation has parameters.";

    /// A package-level import filter settles to a non-Boolean constant.
    InvalidImportFilter => "invalid_import_filter",
        "An import filter condition must evaluate to a Boolean.";

    // --- Namespace identity ------------------------------------------------------------------
    /// A namespace declares two members whose names resolution cannot tell apart.
    DuplicateNamespaceMember => "duplicate_namespace_member",
        "A namespace must not declare two members resolution cannot tell apart.";

    // --- Connection conformance ---------------------------------------------------------------
    /// A connector end resolves to something that is not a port.
    ConnectionEndpointNotPort => "connection_endpoint_not_port",
        "This connector end does not resolve to a port.";
    /// Two connected ports are typed by unrelated definitions.
    PortTypeMismatch => "port_type_mismatch",
        "These connected ports are typed by unrelated definitions.";
    /// Two connected ports mirror the same direction, so nothing can flow between them.
    FlowDirectionIncompatible => "flow_direction_incompatible",
        "These connected ports mirror the same direction, so nothing can flow between them.";
    }
    Advisory {
    /// A declared port takes part in no connection.
    UnconnectedPort => "unconnected_port",
        "This port takes part in no connection.";
    }
    Validation {
    /// Connected elements are neither ports nor a pair of structural parts.
    ConnectionContextInvalid => "connection_context_invalid",
        "Connected elements must be ports, or a pair of structural parts.";
    /// An interface end declares no port type.
    InterfaceEndInvalid => "interface_end_invalid",
        "An interface end must declare a port type.";
    /// A binding connector binds two features with unrelated types.
    BindingConnectorIncompatible => "binding_connector_incompatible",
        "This binding connector binds two features with unrelated types.";

    // --- Behavior conformance -----------------------------------------------------------------
    /// A `perform` names something that is not an action.
    PerformTargetInvalidKind => "perform_target_invalid_kind",
        "A performed behavior must resolve to an action definition or usage.";
    /// A transition endpoint resolves to something that is not a state.
    TransitionEndpointInvalidState => "transition_endpoint_invalid_state",
        "A transition endpoint must resolve to a state.";
    /// A transition's ends belong to different state definitions.
    TransitionEndpointInvalidContext => "transition_endpoint_invalid_context",
        "A transition's source and target must belong to the same state definition.";
    /// An initial-state marker names something that is not a state.
    InitialStateInvalidTarget => "initial_state_invalid_target",
        "An initial-state marker must name a state.";
    /// A succession relates endpoints that are not actions.
    SuccessionEndpointInvalid => "succession_endpoint_invalid",
        "A succession must relate action definitions or usages.";
    /// A transition guard settles to a non-Boolean constant.
    TransitionGuardNonBoolean => "transition_guard_non_boolean",
        "A transition guard must evaluate to a Boolean.";
    }
    Advisory {
    /// A state definition owns states but declares no initial transition.
    MissingInitialState => "missing_initial_state",
        "This state definition owns states but declares no initial transition.";
    /// A state definition owns states but declares no finality indicator.
    MissingFinalState => "missing_final_state",
        "This state definition owns states but declares no finality indicator.";
    }
    Validation {
    /// A state definition declares more than one explicit final state.
    MultipleFinalStates => "multiple_final_states",
        "A state definition declares more than one explicit final state.";
    /// An `accept` payload is typed by something that cannot be an action payload.
    AcceptPayloadIncompatible => "accept_payload_incompatible",
        "An accept payload must be typed by an item, part, attribute or occurrence.";

    // --- Requirement and case conformance ------------------------------------------------------
    /// A declaration owns more than one member of a role that admits one.
    DuplicateRoleMember => "duplicate_role_member",
        "This declaration owns more than one member of a role that admits one.";
    /// A subject member is preceded by another input role member.
    SubjectMemberNotFirst => "subject_member_not_first",
        "A subject member must precede the other input role members.";
    /// A satisfy relationship relates endpoints of incompatible kinds.
    SatisfyInvalidEndpointKind => "satisfy_invalid_endpoint_kind",
        "This satisfy relationship relates endpoints of incompatible kinds.";
    /// A `verify` target does not resolve to a requirement.
    VerifiedRequirementInvalidTarget => "verified_requirement_invalid_target",
        "A verify target must resolve to a requirement definition or usage.";
    /// An `include` target does not resolve to a use case.
    UseCaseIncludeInvalidTarget => "use_case_include_invalid_target",
        "An include target must resolve to a use case definition or usage.";

    // --- View conformance ----------------------------------------------------------------------
    /// A view satisfies something that is not a viewpoint.
    ViewpointConformanceInvalidTargetKind => "viewpoint_conformance_invalid_target_kind",
        "A view must satisfy a viewpoint definition or usage.";
    /// A view usage is typed by a definition outside the SysML standard view catalog.
    ViewTypeNonStandard => "view_type_non_standard",
        "This view is typed by a definition outside the SysML standard view catalog.";
    }
    Unresolved {
    /// A view exposes a target that resolves to nothing.
    ViewExposeUnresolved => "view_expose_unresolved",
        "This expose target does not resolve, so the view shows nothing for it.";
    }
    Validation {
    /// A view declares members but exposes nothing.
    ViewExposeEmpty => "view_expose_empty",
        "This view declares a body but exposes no members.";
    /// A rendering member is typed by something that is not a rendering definition.
    ViewRenderingInvalidTarget => "view_rendering_invalid_target",
        "A rendering member must be typed by a rendering definition or usage.";
    }
    Unresolved {
    /// A textual representation declares no language identifier.
    ViewpointRepLanguageUnresolved => "viewpoint_rep_language_unresolved",
        "A textual representation must declare a language identifier.";
    }
    Validation {

    // --- Allocation ----------------------------------------------------------------------------
    /// An `allocate` statement declares only one of its two endpoints.
    InvalidAllocationEndpoints => "invalid_allocation_endpoints",
        "An allocate statement must declare both a source and a target.";

    // --- Inherited values ----------------------------------------------------------------------
    /// A feature overrides an inherited member without writing `:>>`.
    ImplicitRedefinitionWithoutOperator => "implicit_redefinition_without_operator",
        "This feature overrides an inherited member without the explicit ':>>' operator.";
    /// A feature's authored value is a string where the member it inherits is enumerated.
    InheritedAttributeValueTypeMismatch => "inherited_attribute_value_type_mismatch",
        "This value is a string literal where the inherited member is enumerated.";

    // --- Multiplicity ---------------------------------------------------------------------------
    /// A declared multiplicity states bounds that admit nothing.
    InvalidMultiplicity => "invalid_multiplicity",
        "This multiplicity states bounds that admit nothing.";

    // --- Analysis ------------------------------------------------------------------------------
    /// An analysis constraint settled to false.
    AnalysisConstraintFailed => "analysis_constraint_failed",
        "This analysis constraint evaluated to false.";
    }
    Unresolved {
    /// An analysis constraint could not be evaluated.
    AnalysisEvaluationUnresolved => "analysis_evaluation_unresolved",
        "This analysis expression could not be evaluated.";
    }

    Advisory {
    // --- Authoring hints -------------------------------------------------------------------------
    /// A part usage declares no type.
    UntypedPartUsage => "untyped_part_usage",
        "This part usage declares no type.";
    }
    MissingContext {
    /// A workspace document imports names it cannot resolve without its standard-library baseline.
    MissingLibraryContext => "missing_library_context",
        "This document imports names that do not resolve and no standard-library baseline \
                         was admitted to the publication.";
    /// A required canonical library element was not admitted to the publication.
    MissingLibraryAnchor => "missing_library_anchor",
        "The required standard-library anchor is not available in this publication.";
    }
    Ambiguous {
    /// Several library elements match one required canonical anchor.
    AmbiguousLibraryAnchor => "ambiguous_library_anchor",
        "The required standard-library anchor is ambiguous in this publication.";
    }
}

/// The code a parser error carries when the parser reports none.
pub(crate) const UNCODED_PARSE_ERROR: &str = "parse_error";

#[cfg(test)]
mod tests {
    use super::{DiagnosticCategory, DiagnosticCode};

    #[test]
    fn semantic_codes_publish_owner_declared_categories() {
        assert_eq!(
            DiagnosticCode::UnsupportedParserConstruct.category(),
            DiagnosticCategory::UnsupportedSemantics
        );
        assert_eq!(
            DiagnosticCode::UnresolvedReference.category(),
            DiagnosticCategory::Unresolved
        );
        assert_eq!(
            DiagnosticCode::AmbiguousReference.category(),
            DiagnosticCategory::Ambiguous
        );
        assert_eq!(
            DiagnosticCode::NonConvergedResolution.category(),
            DiagnosticCategory::NonConverged
        );
        assert_eq!(
            DiagnosticCode::MissingLibraryAnchor.category(),
            DiagnosticCategory::MissingContext
        );
        assert_eq!(
            DiagnosticCode::AmbiguousLibraryAnchor.category(),
            DiagnosticCategory::Ambiguous
        );
        assert_eq!(
            DiagnosticCode::UntypedPartUsage.category(),
            DiagnosticCategory::Advisory
        );
    }

    #[test]
    fn parser_category_is_part_of_the_published_code_contract() {
        let diagnostic = DiagnosticCode::Parser {
            code: "parser_code".into(),
            category: DiagnosticCategory::UnsupportedSyntax,
        };
        assert_eq!(diagnostic.as_str(), "parser_code");
        assert_eq!(diagnostic.category(), DiagnosticCategory::UnsupportedSyntax);
    }
}
