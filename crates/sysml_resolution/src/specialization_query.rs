//! Rule-scoped outcomes for complete non-library specialization checks.
//!
//! `Specialization` edges are already canonical resolver facts, but the checks represented here
//! select them through additional roles (result, variation owner, transition endpoint, library
//! anchor, and so on).  The query deliberately reports the first unpublished role family instead
//! of treating any authored or implied edge as evidence that the richer predicate holds.

pub use spec42_constraint_manifest::SpecializationCheckKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecializationCheckPrerequisite {
    RuleNotPublished,
    CrossFeatureProjection,
    FeatureTypingMetaclassAndLibraryAnchor,
    OwnedCrossFeatureOwnerTypes,
    FeatureModifiersOwnerTypingAndLibraryAnchor,
    FeatureValueEvaluationResults,
    SemanticMetadataProjection,
    ConnectorAssociationProjectionAndLibraryAnchor,
    StepOwnershipTypingAndLibraryAnchor,
    ExpressionArgumentResult,
    ExpressionResultAndInstantiatedType,
    LibraryAnchorAndImpliedSpecialization,
    FeatureChainSourceTargetAndSubsetting,
    FeatureReferenceReferentAndResult,
    InvocationInstantiatedTypeAndResult,
    InvocationInstantiatedType,
    SuccessionEndpointAndSubsetting,
    StateSubactionKindAndLibraryAnchor,
    TransitionOwnerSourceAndLibraryAnchor,
    TransitionTriggerPayloadEndpoints,
    TransitionSuccessionSource,
    TransitionFeatureRolesAndLibraryAnchors,
    UseCaseOwnerAndLibraryAnchor,
    UsageVariationOwner,
    IndividualMultiplicityAndLibraryAnchor,
    OccurrenceOwnerTypingAndLibraryAnchor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecializationCheckOutcome {
    Satisfied,
    Violated,
    Unresolved,
    Unsupported {
        prerequisite: SpecializationCheckPrerequisite,
    },
}
