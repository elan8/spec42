//! Which exact derived property is being asked for, and which canonical fact an answer still waits on.
//!
//! Every family here is a pair. A *collection* enum names one closed, pinned derivation defined by
//! the specification -- `Type::ownedFeature`, `Feature::ownedSubsetting`, `Namespace::ownedMember`
//! -- so a caller selects a derivation by naming it rather than by spelling an OCL body. A
//! *prerequisite* enum names the first canonical fact family the publication does not yet own, so
//! "this derivation has no values" and "this derivation cannot be evaluated yet" stay different
//! answers.
//!
//! The matching `*Outcome` enums do not live here: each of them carries a `Box<[SymbolId]>`
//! of results, which is the authority's storage rather than the contract's vocabulary.

/// One exact derived relationship collection or operand projection defined on KerML `Type`.
///
/// The returned values remain canonical relationship facts. In particular, operand projections
/// preserve authored/implied provenance and unresolved targets rather than reducing to names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeDerivedRelationshipCollection {
    OwnedSpecialization,
    OwnedUnioning,
    OwnedIntersecting,
    OwnedDifferencing,
    OwnedDisjoining,
    UnioningType,
    IntersectingType,
    DifferencingType,
}

/// One exact element-valued derivation defined on KerML `Type`.
///
/// This intentionally exposes final member elements only. `FeatureMembership` remains compact
/// declaration-aligned storage, rather than a fabricated public relationship identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeDerivedElementCollection {
    OwnedFeature,
    OwnedEndFeature,
}

/// One exact Type derivation whose normative result shape is known, but whose fact owner is not
/// yet published by the canonical model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeDerivedFactCollection {
    OwnedFeatureMembership,
    FeatureMembership,
    Feature,
    EndFeature,
    DirectedFeature,
    InheritedMembership,
    InheritedFeature,
    Input,
    Output,
    Multiplicity,
    OwnedConjugator,
}

/// The first canonical fact owner an exact Type derivation needs before it can publish values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeDerivedFactPrerequisite {
    FeatureMembershipIdentity,
    FeatureMembershipIdentityAndInheritedClosure,
    InheritedMembershipClosure,
    MultiplicityIdentity,
    ConjugationRelationshipIdentity,
    RuleNotPublished,
}

/// One closed exact relationship collection derived for a KerML `Feature`.
///
/// Each variant is generated from a complete pinned-XMI derivation body. Complex OCL derivations
/// such as `deriveFeatureType` are intentionally absent until their complete canonical inputs and
/// fixed-point contract are owned by the semantic layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FeatureDerivedRelationshipCollection {
    OwnedFeatureChaining,
    OwnedRedefinition,
    OwnedSubsetting,
    OwnedTyping,
    OwnedTypeFeaturing,
}

/// One element-valued Namespace derivation admitted by the pinned manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NamespaceDerivedElementCollection {
    /// `ownedMembership->selectByKind(OwningMembership).ownedMemberElement`.
    OwnedMember,
    /// `ownedRelationship->selectByKind(Import)`.
    OwnedImport,
}

/// The first canonical fact owner still needed by an exact Definition/Usage derivation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefinitionUsageDerivedPrerequisite {
    /// The selected generated rule was not published by the loaded manifest.
    RuleNotPublished,
    /// `feature`/`directedFeature` requires the effective inherited FeatureMembership closure;
    /// direct owner membership alone is deliberately not substituted.
    EffectiveFeatureMembershipClosure,
}

/// One closed exact `Systems::Actions` derived property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActionDerivedFactCollection {
    ActionDefinitionAction,
    AssignmentValueExpression,
    AssignmentTargetArgument,
    AssignmentReferent,
    ForLoopVariable,
    ForLoopSeqArgument,
    LoopBodyAction,
    TerminateOccurrenceArgument,
    AcceptPayloadArgument,
    AcceptPayloadParameter,
    AcceptReceiverArgument,
    WhileArgument,
    UntilArgument,
    SendSenderArgument,
    SendReceiverArgument,
    SendPayloadArgument,
    IfThenAction,
    IfElseAction,
    IfArgument,
}

/// The first canonical fact owner an exact `Systems::Actions` derivation needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionDerivedFactPrerequisite {
    OrderedInputParameterIdentity,
    OwnedMembershipIdentity,
    OrderedOwnedFeatureIdentity,
    ActionMetaclassIdentity,
    RuleNotPublished,
}

/// One closed exact `Systems::Requirements` derived property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RequirementDerivedFactCollection {
    DefinitionActorParameter,
    DefinitionSubjectParameter,
    DefinitionText,
    DefinitionRequiredConstraint,
    DefinitionAssumedConstraint,
    DefinitionFramedConcern,
    UsageActorParameter,
    UsageSubjectParameter,
    UsageText,
    UsageRequiredConstraint,
    UsageAssumedConstraint,
    UsageFramedConcern,
}

impl RequirementDerivedFactCollection {
    /// Whether this derivation's values are documentation text rather than elements.
    ///
    /// A property of the derivation itself, so a caller knows which arm of the outcome to expect
    /// before it asks.
    pub const fn requires_text(self) -> bool {
        matches!(self, Self::DefinitionText | Self::UsageText)
    }
}

/// The first canonical fact owner an exact `Systems::Requirements` derivation needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequirementDerivedFactPrerequisite {
    RuleNotPublished,
    CanonicalMembershipRole,
    DocumentationRecords,
}
