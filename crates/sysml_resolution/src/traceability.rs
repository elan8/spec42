use crate::{RelationshipProvenance, SourceLocation, SymbolIdentity};

/// The settled target of one directional end of an authored satisfy relationship.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SatisfyEndpoint {
    Resolved(SymbolIdentity),
    Ambiguous(Box<[SymbolIdentity]>),
    Unresolved,
    Unsupported,
}

/// Whether the authored statement asserts or negates satisfaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SatisfyPolarity {
    Satisfied,
    NotSatisfied,
}

/// One authoritative `satisfy <requirement> by <element>` statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SatisfyRelationship {
    /// Stable identity of the anonymous relationship usage, preserving duplicates.
    pub identity: SymbolIdentity,
    /// The `satisfy` operand: the requirement being satisfied.
    pub requirement: SatisfyEndpoint,
    /// The `by` operand: the element claimed to satisfy the requirement.
    pub satisfying_element: SatisfyEndpoint,
    pub polarity: SatisfyPolarity,
    pub provenance: RelationshipProvenance,
    pub location: SourceLocation,
}
