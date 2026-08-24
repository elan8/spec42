use crate::{RelationshipProvenance, SourceLocation, SymbolId};

pub use sysml_contract::VerificationOutcome;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationRequirement {
    Resolved(SymbolId),
    Ambiguous(Box<[SymbolId]>),
    Unresolved,
    Unsupported,
}

/// One authored requirement-verification membership, directed from its containing case to the
/// requirement named by the membership.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementVerification {
    pub identity: SymbolId,
    pub verification_case: SymbolId,
    pub requirement: VerificationRequirement,
    pub provenance: RelationshipProvenance,
    pub location: SourceLocation,
    pub outcome: VerificationOutcome,
}
