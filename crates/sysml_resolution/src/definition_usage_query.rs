//! Exact Systems::DefinitionAndUsage derived-property query contracts.
//!
//! The manifest selects one closed normative property.  The resolver then consumes only the
//! publication's canonical direct owner, feature-membership, declaration-kind, and modifier
//! facts.  It never reconstructs a broader inherited `feature` collection from syntax or names.

use crate::{MembershipId, SymbolId};

pub use spec42_constraint_manifest::DefinitionUsageDerivedKind;
pub use sysml_contract::DefinitionUsageDerivedPrerequisite;

/// The published value shape of one exact Definition/Usage derivation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefinitionUsageDerivedOutcome {
    /// A final element-valued collection. The declarations retain their canonical identities and
    /// are deterministically ordered by the owner-defined symbol projection.
    Elements(Box<[SymbolId]>),
    /// A relationship-valued collection. Each handle addresses the canonical Membership record;
    /// it is not the identity of the member element in another guise.
    Memberships(Box<[MembershipId]>),
    /// A scalar property whose complete canonical modifier facts are available.
    Boolean(bool),
    /// The precise first fact family that is not currently a published canonical input.
    Unsupported {
        prerequisite: DefinitionUsageDerivedPrerequisite,
    },
}
