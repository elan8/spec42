//! The published type-query contract.
//!
//! Direct types, supertypes, subtypes, effective types, featuring types and conformance, answered
//! from settled facts. Consumers get typed answers; they never get the closure, the scope bitset,
//! or any other storage this crate uses to produce them.

use crate::inspection::{MultiplicityFacts, RelationshipProvenance};
use crate::SymbolIdentity;
pub use spec42_constraint_manifest::TypeDerivedFactKind;
pub use spec42_constraint_manifest::TypeFeaturingCheckKind;

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

/// A future canonical value of one exact Type derived-fact query.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDerivedFactValue {
    Feature(SymbolIdentity),
    FeatureMembership { member: SymbolIdentity },
    Multiplicity(MultiplicityFacts),
    Conjugator { original_type: SymbolIdentity },
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

/// A typed result for exact Type derivations that are not yet executable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDerivedFactOutcome {
    Values(Box<[TypeDerivedFactValue]>),
    Unsupported {
        prerequisite: TypeDerivedFactPrerequisite,
    },
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

/// One type a feature declares.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeReference {
    pub symbol: SymbolIdentity,
    pub provenance: RelationshipProvenance,
}

/// The authoritative direct typing of one requirement usage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequirementUsageTyping {
    Missing,
    Resolved(TypeReference),
    Ambiguous(Box<[SymbolIdentity]>),
    Unresolved,
    Unsupported,
}

/// Where one of a feature's effective types came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectiveTypeOrigin {
    /// The feature declares this typing itself.
    Direct,
    /// The feature inherits it from a feature it subsets or redefines.
    Inherited(SymbolIdentity),
}

/// One type a feature has, directly or by inheritance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveType {
    pub symbol: SymbolIdentity,
    pub origin: EffectiveTypeOrigin,
}

/// Why a conformance question has no settled answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConformanceObstacle {
    /// The specific side reaches itself through specialization. Its hierarchy is malformed, so no
    /// answer derived from it is trustworthy -- reporting one would launder a modelling error into
    /// a semantic fact.
    CyclicSpecialization,
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
