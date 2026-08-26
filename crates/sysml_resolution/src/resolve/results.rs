//! Phase 3: the settled outcome of name resolution.

use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
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
pub(crate) struct ConstructorExpressionProjection {
    pub(crate) expression: DeclarationId,
    pub(crate) result: DeclarationId,
    pub(crate) instantiated_type: DeclarationId,
}

#[derive(Debug)]
pub(crate) struct ResolutionResults {
    pub(crate) outcomes: Box<[ResolutionStatus]>,
    pub(crate) ambiguous_candidates: Box<[DeclarationId]>,
    pub(crate) inherited_names: NameIndex,
    pub(crate) solver_status: SolverStatus,
    pub(crate) implied_relationships: Box<[ImpliedRelationship]>,
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
    #[cfg(test)]
    pub(crate) work: ResolutionWork,
}

impl ResolutionResults {
    pub(crate) fn outcome(&self, id: AuthoredReferenceId) -> Option<ResolutionStatus> {
        self.outcomes.get(id.index()).copied()
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

    pub(crate) fn library_specialization_anchor(
        &self,
        rule_id: &str,
    ) -> Option<&LibrarySpecializationAnchor> {
        self.library_specialization_anchors.outcome(rule_id)
    }
}
