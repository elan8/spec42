//! Exact Systems::DefinitionAndUsage derived-property query contracts.
//!
//! The manifest selects one closed normative property.  The resolver then consumes only the
//! publication's canonical direct owner, feature-membership, declaration-kind, and modifier
//! facts.  It never reconstructs a broader inherited `feature` collection from syntax or names.

use crate::SymbolIdentity;

pub use spec42_constraint_manifest::DefinitionUsageDerivedKind;

/// The published value shape of one exact Definition/Usage derivation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefinitionUsageDerivedOutcome {
    /// A final element-valued collection. The declarations retain their canonical identities and
    /// are deterministically ordered by the owner-defined symbol projection.
    Elements(Box<[SymbolIdentity]>),
    /// A scalar property whose complete canonical modifier facts are available.
    Boolean(bool),
    /// The precise first fact family that is not currently a published canonical input.
    Unsupported {
        prerequisite: DefinitionUsageDerivedPrerequisite,
    },
}

/// The first canonical fact owner still needed by an exact Definition/Usage derivation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefinitionUsageDerivedPrerequisite {
    /// The selected generated rule was not published by the loaded manifest.
    RuleNotPublished,
    /// `feature`/`directedFeature` requires the effective inherited FeatureMembership closure;
    /// direct owner membership alone is deliberately not substituted.
    EffectiveFeatureMembershipClosure,
    /// `variantMembership` is an OMG relationship identity, which compact declaration-aligned
    /// storage does not currently publish as a queryable fact.
    VariantMembershipIdentity,
    /// `mayTimeVary` needs the effective library-specialization and portion predicates as one
    /// canonical fact family; direct modifiers or graph edges alone do not decide it.
    EffectiveOccurrenceTimeVariationFacts,
}
