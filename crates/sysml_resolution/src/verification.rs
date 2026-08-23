use crate::{RelationshipProvenance, SourceLocation, SymbolIdentity};

pub use sysml_contract::VerificationOutcome;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationRequirement {
    Resolved(SymbolIdentity),
    Ambiguous(Box<[SymbolIdentity]>),
    Unresolved,
    Unsupported,
}

/// One authored requirement-verification membership, directed from its containing case to the
/// requirement named by the membership.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementVerification {
    pub identity: SymbolIdentity,
    pub verification_case: SymbolIdentity,
    pub requirement: VerificationRequirement,
    pub provenance: RelationshipProvenance,
    pub location: SourceLocation,
    pub outcome: VerificationOutcome,
}
