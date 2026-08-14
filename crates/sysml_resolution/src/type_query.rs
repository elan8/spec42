//! The published type-query contract.
//!
//! Direct types, supertypes, subtypes, effective types, featuring types and conformance, answered
//! from settled facts. Consumers get typed answers; they never get the closure, the scope bitset,
//! or any other storage this crate uses to produce them.

use crate::inspection::RelationshipProvenance;
use crate::SymbolIdentity;

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
