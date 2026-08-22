//! Rule-scoped outcomes for exact normative redefinition checks.
//!
//! The resolver already owns authored and implied `Redefinition` relationship facts. The 15
//! predicates represented here additionally require role-specific membership, endpoint,
//! direction, or identity facts. Until those facts are published, callers receive the first
//! missing prerequisite rather than a relationship inferred from a declaration name or a rendered
//! semantic projection.

pub use spec42_constraint_manifest::RedefinitionCheckKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedefinitionCheckPrerequisite {
    RuleNotPublished,
    EndFeaturePositionAndInheritedEnds,
    FlowEndOrdinalAndLibraryAnchors,
    CrossFeatureAndSubsettingEndpoints,
    ParameterDirectionAndInheritedPosition,
    FunctionOrExpressionResult,
    ConstructorResultAndInstantiatedTypeFeatures,
    FeatureChainSourceTarget,
    FeatureChainSourceTargetAndLibraryAnchor,
    StateSubactionMembershipAndKind,
    AssignmentActionInputParameterEndpoints,
    ForLoopVariableProjection,
    ObjectiveMembershipAndCaseObjective,
    ViewRenderingMembership,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedefinitionCheckOutcome {
    Satisfied,
    Violated,
    Unresolved,
    Unsupported {
        prerequisite: RedefinitionCheckPrerequisite,
    },
}
