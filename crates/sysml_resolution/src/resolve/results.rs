//! Phase 3: the settled outcome of name resolution.

use crate::lower::facts::TransitionFeatureRole;
use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
use crate::model::NameId;
use crate::model::ReferenceKind;
use crate::resolve::implied::LibrarySpecializationAnchor;
use crate::resolve::implied::LibrarySpecializationAnchorFacts;
use crate::resolve::names::CandidateRange;
use crate::resolve::names::NameIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolutionError {
    Capacity,
    InvalidStorage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolutionStatus {
    Resolved(DeclarationId),
    Unresolved,
    Ambiguous(CandidateRange),
    Unsupported,
    NonConverged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SolverStatus {
    Converged,
    NonConverged,
}

/// One component of KerML `Feature::effectiveName`. Authored names are retained on the syntax-owned
/// declaration/fact records; this phase-3 value records only the settled semantic result and keeps
/// absence, an unresolved first Redefinition, and a cyclic naming chain distinct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EffectiveNameOutcome {
    Resolved(NameId),
    Absent,
    Unresolved,
    NonConverged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct EffectiveNameFacts {
    pub(crate) name: EffectiveNameOutcome,
    pub(crate) short_name: EffectiveNameOutcome,
    /// True only when both components come from the first redefined Feature rather than authored
    /// identification. This preserves provenance without replacing the authored declaration name.
    pub(crate) derived_from_redefinition: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct ResolutionWork {
    pub(crate) passes: u32,
    pub(crate) import_evaluations: u64,
    pub(crate) downstream_evaluations: u64,
    pub(crate) indexed_name_lookups: u64,
    pub(crate) direct_index_entries: u64,
    pub(crate) effective_index_entries: u64,
}

/// A resolver-synthesized relationship fact that has no authored reference site. The narrow slice
/// currently covered here is same-name inherited-member redefinition against an immediate
/// (directly specialized) parent's own directly owned feature. Multi-level/diamond inherited
/// redefinition is intentionally out of scope: an ambiguous or absent immediate-parent match is
/// left unresolved rather than guessed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ImpliedRelationship {
    pub(crate) kind: ReferenceKind,
    pub(crate) source: DeclarationId,
    pub(crate) target: DeclarationId,
}

/// A settled relationship with an authored declaration site and two resolved authored endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AuthoredRelationship {
    pub(crate) kind: ReferenceKind,
    pub(crate) source: DeclarationId,
    pub(crate) target: DeclarationId,
    pub(crate) declaration: crate::model::AuthoredReferenceId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SemanticMetadataProjection {
    pub(crate) annotation: DeclarationId,
    pub(crate) annotated_element: DeclarationId,
    pub(crate) syntax_element: DeclarationId,
    pub(crate) specialization_target: DeclarationId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum SemanticMetadataProjectionStatus {
    #[default]
    Complete,
    Unresolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum ExpressionArgumentProjectionStatus {
    #[default]
    Complete,
    Unresolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum ConstructorExpressionProjectionStatus {
    #[default]
    Complete,
    Unresolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum ConstructorExpressionSpecializationStatus {
    #[default]
    Complete,
    Unresolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum FeatureChainExpressionSpecializationStatus {
    Complete,
    #[default]
    Unresolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum FeatureReferenceExpressionSpecializationStatus {
    Complete,
    #[default]
    Unresolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum InvocationExpressionProjectionStatus {
    Complete,
    #[default]
    Unresolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InvocationInstantiatedTypeKind {
    Function,
    FeatureTypedByFunction,
    NonFunctionType,
    NonFunctionFeature,
}

impl InvocationInstantiatedTypeKind {
    pub(crate) const fn is_function(self) -> bool {
        matches!(self, Self::Function | Self::FeatureTypedByFunction)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FeatureChainExpressionProjection {
    pub(crate) expression: DeclarationId,
    pub(crate) result: DeclarationId,
    pub(crate) input_parameter: DeclarationId,
    pub(crate) source_target: DeclarationId,
    pub(crate) target_feature: DeclarationId,
    pub(crate) subsetting_chain: DeclarationId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FeatureReferenceExpressionProjection {
    pub(crate) expression: DeclarationId,
    pub(crate) result: DeclarationId,
    pub(crate) referent: DeclarationId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ConstructorExpressionProjection {
    pub(crate) expression: DeclarationId,
    pub(crate) result: DeclarationId,
    pub(crate) instantiated_type: DeclarationId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct InvocationExpressionProjection {
    pub(crate) expression: DeclarationId,
    pub(crate) result: DeclarationId,
    pub(crate) instantiated_type: DeclarationId,
    pub(crate) instantiated_type_kind: InvocationInstantiatedTypeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SuccessionEndpointSubsettingKind {
    DecisionOutgoing,
    MergeIncoming,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum SuccessionEndpointSubsettingStatus {
    #[default]
    Complete,
    Unresolved,
}

/// The resolved endpoint and library feature that jointly define one normative `subsetsChain`.
/// The endpoint is retained explicitly because the implied Subsetting edge alone cannot represent
/// whether the library feature is contextualized by the source DecisionNode or target MergeNode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SuccessionEndpointSubsettingProjection {
    pub(crate) succession: DeclarationId,
    pub(crate) endpoint: DeclarationId,
    pub(crate) subsetting_target: DeclarationId,
    pub(crate) kind: SuccessionEndpointSubsettingKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TransitionPayloadSubsettingProjection {
    pub(crate) transition: DeclarationId,
    pub(crate) transition_payload_parameter: DeclarationId,
    pub(crate) trigger_action: DeclarationId,
    pub(crate) trigger_payload_parameter: DeclarationId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum TransitionPayloadSubsettingStatus {
    #[default]
    Complete,
    Unresolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TransitionSuccessionSourceProjection {
    pub(crate) transition: DeclarationId,
    pub(crate) succession: DeclarationId,
    pub(crate) transition_source: Option<DeclarationId>,
    pub(crate) succession_source: Option<DeclarationId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum TransitionSuccessionSourceStatus {
    #[default]
    Complete,
    Unresolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TransitionFeatureSpecializationProjection {
    pub(crate) transition: DeclarationId,
    pub(crate) feature: DeclarationId,
    pub(crate) role: TransitionFeatureRole,
    pub(crate) library_anchor: DeclarationId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum TransitionFeatureSpecializationStatus {
    #[default]
    Complete,
    Unresolved,
}

#[derive(Debug)]
pub(crate) struct ResolutionResults {
    pub(crate) outcomes: Box<[ResolutionStatus]>,
    pub(crate) ambiguous_candidates: Box<[DeclarationId]>,
    pub(crate) inherited_names: NameIndex,
    pub(crate) effective_names: Box<[EffectiveNameFacts]>,
    pub(crate) solver_status: SolverStatus,
    pub(crate) implied_relationships: Box<[ImpliedRelationship]>,
    pub(crate) authored_relationships: Box<[AuthoredRelationship]>,
    pub(crate) library_specialization_anchors: LibrarySpecializationAnchorFacts,
    pub(crate) semantic_metadata_projections: Box<[SemanticMetadataProjection]>,
    pub(crate) semantic_metadata_projection_status: SemanticMetadataProjectionStatus,
    pub(crate) select_expression_projection_status: ExpressionArgumentProjectionStatus,
    pub(crate) index_expression_projection_status: ExpressionArgumentProjectionStatus,
    pub(crate) index_expression_array_anchor: Option<LibrarySpecializationAnchor>,
    pub(crate) constructor_expression_projection_status: ConstructorExpressionProjectionStatus,
    pub(crate) constructor_expression_projections: Box<[ConstructorExpressionProjection]>,
    pub(crate) constructor_expression_specialization_status:
        ConstructorExpressionSpecializationStatus,
    pub(crate) constructor_expression_anchor: Option<LibrarySpecializationAnchor>,
    pub(crate) feature_chain_expression_specialization_status:
        FeatureChainExpressionSpecializationStatus,
    pub(crate) feature_chain_expression_projections: Box<[FeatureChainExpressionProjection]>,
    pub(crate) feature_reference_expression_status: FeatureReferenceExpressionSpecializationStatus,
    pub(crate) feature_reference_expression_projections:
        Box<[FeatureReferenceExpressionProjection]>,
    pub(crate) invocation_expression_projection_status: InvocationExpressionProjectionStatus,
    pub(crate) invocation_expression_projections: Box<[InvocationExpressionProjection]>,
    pub(crate) succession_endpoint_subsetting_projections:
        Box<[SuccessionEndpointSubsettingProjection]>,
    pub(crate) decision_outgoing_subsetting_status: SuccessionEndpointSubsettingStatus,
    pub(crate) merge_incoming_subsetting_status: SuccessionEndpointSubsettingStatus,
    pub(crate) transition_payload_subsetting_projections:
        Box<[TransitionPayloadSubsettingProjection]>,
    pub(crate) transition_payload_subsetting_status: TransitionPayloadSubsettingStatus,
    pub(crate) transition_succession_source_projections:
        Box<[TransitionSuccessionSourceProjection]>,
    pub(crate) transition_succession_source_status: TransitionSuccessionSourceStatus,
    pub(crate) transition_feature_specialization_projections:
        Box<[TransitionFeatureSpecializationProjection]>,
    pub(crate) transition_feature_specialization_status: TransitionFeatureSpecializationStatus,
    #[cfg(test)]
    pub(crate) work: ResolutionWork,
}

impl ResolutionResults {
    pub(crate) fn outcome(&self, id: AuthoredReferenceId) -> Option<ResolutionStatus> {
        self.outcomes.get(id.index()).copied()
    }

    pub(crate) fn settle_authored_relationships(
        self,
        authored_relationships: Box<[AuthoredRelationship]>,
    ) -> Self {
        Self {
            authored_relationships,
            ..self
        }
    }

    pub(crate) fn ambiguous_candidates(&self, range: CandidateRange) -> &[DeclarationId] {
        range.slice(&self.ambiguous_candidates).unwrap_or_default()
    }

    /// The phase-4 settle: implied relationships and library anchors arrive together, by moving
    /// the phase-3 value into a new one rather than writing back into the solver's product. No
    /// reader can observe the pre-synthesis state, because it no longer exists once this returns.
    pub(crate) fn settle(
        self,
        implied_relationships: Box<[ImpliedRelationship]>,
        library_specialization_anchors: LibrarySpecializationAnchorFacts,
    ) -> Self {
        Self {
            implied_relationships,
            library_specialization_anchors,
            ..self
        }
    }

    pub(crate) fn settle_semantic_metadata(
        self,
        implied_relationships: Box<[ImpliedRelationship]>,
        semantic_metadata_projections: Box<[SemanticMetadataProjection]>,
        semantic_metadata_projection_status: SemanticMetadataProjectionStatus,
    ) -> Self {
        Self {
            implied_relationships,
            semantic_metadata_projections,
            semantic_metadata_projection_status,
            ..self
        }
    }

    pub(crate) fn settle_expression_arguments(
        self,
        implied_relationships: Box<[ImpliedRelationship]>,
        select_expression_projection_status: ExpressionArgumentProjectionStatus,
        index_expression_projection_status: ExpressionArgumentProjectionStatus,
        index_expression_array_anchor: Option<LibrarySpecializationAnchor>,
    ) -> Self {
        Self {
            implied_relationships,
            select_expression_projection_status,
            index_expression_projection_status,
            index_expression_array_anchor,
            ..self
        }
    }

    pub(crate) fn settle_constructor_expressions(
        self,
        implied_relationships: Box<[ImpliedRelationship]>,
        constructor_expression_projections: Box<[ConstructorExpressionProjection]>,
        constructor_expression_projection_status: ConstructorExpressionProjectionStatus,
        constructor_expression_specialization_status: ConstructorExpressionSpecializationStatus,
        constructor_expression_anchor: Option<LibrarySpecializationAnchor>,
    ) -> Self {
        Self {
            implied_relationships,
            constructor_expression_projections,
            constructor_expression_projection_status,
            constructor_expression_specialization_status,
            constructor_expression_anchor,
            ..self
        }
    }

    pub(crate) fn settle_feature_chain_expressions(
        self,
        implied_relationships: Box<[ImpliedRelationship]>,
        feature_chain_expression_projections: Box<[FeatureChainExpressionProjection]>,
        feature_chain_expression_specialization_status: FeatureChainExpressionSpecializationStatus,
    ) -> Self {
        Self {
            implied_relationships,
            feature_chain_expression_projections,
            feature_chain_expression_specialization_status,
            ..self
        }
    }

    pub(crate) fn settle_feature_reference_expressions(
        self,
        implied_relationships: Box<[ImpliedRelationship]>,
        feature_reference_expression_projections: Box<[FeatureReferenceExpressionProjection]>,
        feature_reference_expression_status: FeatureReferenceExpressionSpecializationStatus,
    ) -> Self {
        Self {
            implied_relationships,
            feature_reference_expression_projections,
            feature_reference_expression_status,
            ..self
        }
    }

    pub(crate) fn settle_invocation_expressions(
        self,
        implied_relationships: Box<[ImpliedRelationship]>,
        invocation_expression_projections: Box<[InvocationExpressionProjection]>,
        invocation_expression_projection_status: InvocationExpressionProjectionStatus,
    ) -> Self {
        Self {
            implied_relationships,
            invocation_expression_projections,
            invocation_expression_projection_status,
            ..self
        }
    }

    pub(crate) fn settle_succession_endpoint_subsettings(
        self,
        implied_relationships: Box<[ImpliedRelationship]>,
        projections: Box<[SuccessionEndpointSubsettingProjection]>,
        decision_status: SuccessionEndpointSubsettingStatus,
        merge_status: SuccessionEndpointSubsettingStatus,
    ) -> Self {
        Self {
            implied_relationships,
            succession_endpoint_subsetting_projections: projections,
            decision_outgoing_subsetting_status: decision_status,
            merge_incoming_subsetting_status: merge_status,
            ..self
        }
    }

    pub(crate) fn settle_transition_payload_subsettings(
        self,
        implied_relationships: Box<[ImpliedRelationship]>,
        projections: Box<[TransitionPayloadSubsettingProjection]>,
        status: TransitionPayloadSubsettingStatus,
    ) -> Self {
        Self {
            implied_relationships,
            transition_payload_subsetting_projections: projections,
            transition_payload_subsetting_status: status,
            ..self
        }
    }

    pub(crate) fn settle_transition_succession_sources(
        self,
        projections: Box<[TransitionSuccessionSourceProjection]>,
        status: TransitionSuccessionSourceStatus,
    ) -> Self {
        Self {
            transition_succession_source_projections: projections,
            transition_succession_source_status: status,
            ..self
        }
    }

    pub(crate) fn settle_transition_feature_specializations(
        self,
        implied_relationships: Box<[ImpliedRelationship]>,
        projections: Box<[TransitionFeatureSpecializationProjection]>,
        status: TransitionFeatureSpecializationStatus,
    ) -> Self {
        Self {
            implied_relationships,
            transition_feature_specialization_projections: projections,
            transition_feature_specialization_status: status,
            ..self
        }
    }

    pub(crate) fn library_specialization_anchor(
        &self,
        rule_id: &str,
    ) -> Option<&LibrarySpecializationAnchor> {
        self.library_specialization_anchors.outcome(rule_id)
    }
}
