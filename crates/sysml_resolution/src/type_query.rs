//! The published type-query contract.
//!
//! Direct types, supertypes, subtypes, effective types, featuring types and conformance, answered
//! from settled facts. Consumers get typed answers; they never get the closure, the scope bitset,
//! or any other storage this crate uses to produce them.

use crate::inspection::{MultiplicityFacts, RelationshipProvenance};
use crate::SymbolIdentity;
pub use spec42_constraint_manifest::TypeDerivedFactKind;
pub use spec42_constraint_manifest::TypeFeaturingCheckKind;
pub use sysml_contract::{
    Conformance, ConformanceObstacle, SpecializationScope, SubsettingConformance,
    TypeDerivedElementCollection, TypeDerivedFactCollection, TypeDerivedFactPrerequisite,
    TypeDerivedRelationshipCollection, TypeFeaturingCheckOutcome, TypeFeaturingCheckPrerequisite,
};

/// A future canonical value of one exact Type derived-fact query.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDerivedFactValue {
    Feature(SymbolIdentity),
    FeatureMembership { member: SymbolIdentity },
    Multiplicity(MultiplicityFacts),
    Conjugator { original_type: SymbolIdentity },
}

/// A typed result for exact Type derivations that are not yet executable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDerivedFactOutcome {
    Values(Box<[TypeDerivedFactValue]>),
    Unsupported {
        prerequisite: TypeDerivedFactPrerequisite,
    },
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
