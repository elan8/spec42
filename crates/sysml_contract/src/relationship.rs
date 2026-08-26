//! How one element stands to another, and what a check of that standing settled on.
//!
//! Two families. The first says what a relationship *is*: whether it was authored or implied,
//! whether a family of targets resolved, and whether one type conforms to another. The second
//! says what a rule *found*: satisfied, violated, or unsettled -- and when unsettled, which
//! prerequisite fact the authority has not published yet, so an unsupported answer names its own
//! gap instead of being indistinguishable from a violation.

/// Whether a relationship was authored or derived by the resolver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RelationshipProvenance {
    /// Written in the source.
    Authored,
    /// Synthesized by the resolver from a rule, such as an implied redefinition.
    Implied,
}

/// What resolution settled for one authored relationship family of an element.
///
/// A family is every authored reference of one kind group -- the typings, the specializations, the
/// subsettings. Its outcome is the *least settled* outcome among them, so a family where one
/// reference resolved and another did not is [`RelationshipOutcome::Partial`] rather than
/// resolved. The precedence, applied in this order, is:
///
/// 1. no authored reference at all is [`RelationshipOutcome::NotApplicable`];
/// 2. any ambiguous reference makes the family [`RelationshipOutcome::Ambiguous`];
/// 3. otherwise any unsupported reference makes it [`RelationshipOutcome::Unsupported`];
/// 4. otherwise a mix of settled and unsettled references is [`RelationshipOutcome::Partial`];
/// 5. otherwise the family is wholly [`RelationshipOutcome::Resolved`] or wholly
///    [`RelationshipOutcome::Unresolved`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RelationshipOutcome {
    /// The element authors no relationship of this family, so there is nothing to resolve.
    NotApplicable,
    /// Every authored reference in the family settled on a target.
    Resolved,
    /// Some authored references settled and others did not.
    Partial,
    /// The family is authored and no reference in it settled on a target.
    Unresolved,
    /// At least one reference has several candidates, and none of them was chosen.
    Ambiguous,
    /// At least one reference is written in a form outside the supported resolution slice.
    Unsupported,
}

impl RelationshipOutcome {
    /// A stable kebab-case name, for debug rendering and snapshot output.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not-applicable",
            Self::Resolved => "resolved",
            Self::Partial => "partial",
            Self::Unresolved => "unresolved",
            Self::Ambiguous => "ambiguous",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Whether one type conforms to another.
///
/// Deliberately not a `bool`. A conformance question over an unresolved, ambiguous or cyclic
/// hierarchy has no answer, and collapsing that into `false` is what makes the legacy check report
/// a type incompatibility on top of the unresolved-reference diagnostic that caused it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Conformance {
    Conforms,
    DoesNotConform,
    Indeterminate(ConformanceObstacle),
}

/// Why a conformance question has no settled answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConformanceObstacle {
    /// The specific side reaches itself through specialization. Its hierarchy is malformed, so no
    /// answer derived from it is trustworthy -- reporting one would launder a modelling error into
    /// a semantic fact.
    CyclicSpecialization,
}

/// The two halves of KerML §8.4.3.4's subsetting rule, kept apart.
///
/// A consumer that reports a violation needs to say which half failed: "the subsetting feature's
/// type does not conform" and "the subsetting feature is featured by an unrelated type" are
/// different errors with different fixes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubsettingConformance {
    /// Whether the subsetting feature's featuring type conforms to the subsetted feature's.
    pub featuring: Conformance,
    /// Whether the subsetting feature's types conform to the subsetted feature's.
    pub types: Conformance,
}

/// Which specialization edges a supertype or conformance question may follow.
///
/// KerML makes `Subclassification`, `Subsetting`, `Redefinition` and `FeatureTyping` all subkinds
/// of `Specialization`, and the OMG Pilot's `Type::supertypes` spans all of them. A consumer that
/// wants the narrower classifier-only reading asks for it here rather than filtering an answer it
/// was already given, because filtering after the fact cannot tell a path that used only
/// subclassification from one that happened to end at a classifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpecializationScope {
    /// Every `Specialization` subkind. The Pilot's reading.
    AnySpecialization,
    /// `Subclassification` alone: generalization between classifiers.
    Subclassification,
    /// `Subsetting` and `Redefinition`: how one feature specializes another.
    FeatureSpecialization,
}

/// Why an exact TypeFeaturing check cannot be decided from the canonical publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeFeaturingCheckPrerequisite {
    /// The selected typed rule has no generated manifest binding.
    RuleNotPublished,
    /// The declaration has no published FeatureMembership applicability fact.
    FeatureMembershipFacts,
    /// A variable feature's required snapshots fact remains intentionally unpublished.
    VariableFeatureSnapshots,
}

/// A rule-scoped outcome for a closed exact TypeFeaturing check.
///
/// This is not a general OCL evaluator. Each variant is produced only by the canonical fact
/// family named by the manifest contract and keeps lack of a prerequisite distinct from a false
/// predicate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeFeaturingCheckOutcome {
    Satisfied,
    Violated,
    Unresolved,
    Unsupported {
        prerequisite: TypeFeaturingCheckPrerequisite,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecializationCheckPrerequisite {
    RuleNotPublished,
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

/// Whether the authored statement asserts or negates satisfaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SatisfyPolarity {
    Satisfied,
    NotSatisfied,
}

/// Why a named binding-connector validation could not be evaluated from canonical facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingConnectorValidationPrerequisite {
    /// Lowering has not yet published the `FeatureReferenceExpression.targetFeature` and
    /// expression-result facts which the exact rule relates through a binding connector.
    FeatureReferenceExpressionTargetAndResult,
    FeatureValueEndpointFacts,
    ExpressionResultEndpointFacts,
    FunctionResultEndpointFacts,
    InvocationExpressionBehaviorEndpointFacts,
    AcceptActionUsageReceiverEndpointFacts,
    TransitionUsageSourceEndpointFacts,
    TransitionUsageSuccessionEndpointFacts,
    SatisfyRequirementUsageEndpointFacts,
    /// The exact pinned OCL body is `TBD`, so OMG has not supplied an evaluable predicate.
    NormativeSpecificationTbd,
    /// The selected typed rule is not present in the manifest-derived resolver table.
    ///
    /// This protects the query boundary from treating an enum value as evidence that the pinned
    /// manifest actually publishes a corresponding normative contract.
    RuleNotPublished,
}

/// The explicit result of asking for one binding-connector validation.
///
/// This is separate from a connector endpoint's resolution state. An endpoint may be settled
/// while the named validation remains unsupported because the rule's own semantic inputs do not
/// exist in the publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingConnectorValidationOutcome {
    /// The full named predicate held over canonical paired binding and endpoint facts.
    Satisfied,
    /// The full named predicate was evaluable and did not hold.
    Violated,
    /// A required paired connector endpoint is unresolved or ambiguous.
    Unresolved,
    Unsupported {
        prerequisite: BindingConnectorValidationPrerequisite,
    },
}
