use crate::{RelationshipProvenance, SourceLocation, SymbolIdentity};
pub use spec42_constraint_manifest::BindingConnectorCheckKind;
pub use sysml_contract::{
    BindingConnectorValidationOutcome, BindingConnectorValidationPrerequisite, SatisfyPolarity,
};

/// The settled target of one directional end of an authored satisfy relationship.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SatisfyEndpoint {
    Resolved(SymbolIdentity),
    Ambiguous(Box<[SymbolIdentity]>),
    Unresolved,
    Unsupported,
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

/// The settled target of one directional end of an authored binding connector.
///
/// This is deliberately separate from [`SatisfyEndpoint`]. A binding connector is an equality
/// relationship, not a requirement claim, and publishing a distinct endpoint type prevents a
/// consumer from accidentally treating its left/right pair as a satisfy statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingEndpoint {
    Resolved(SymbolIdentity),
    Ambiguous(Box<[SymbolIdentity]>),
    Unresolved,
    Unsupported,
}

/// One authoritative binding connector with its two paired ends.
///
/// The semantic builder creates this fact at the resolution publication barrier. Consumers read
/// the paired fact rather than independently scanning `BindSource` and `BindTarget` references,
/// which preserves duplicates and makes a partially settled end explicit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingConnector {
    /// Stable identity of the authored binding-connector declaration or anonymous `bind`
    /// statement. Separate authored statements remain separate facts even when their endpoints
    /// are identical.
    pub identity: SymbolIdentity,
    pub source: BindingEndpoint,
    pub target: BindingEndpoint,
    pub provenance: RelationshipProvenance,
    pub location: SourceLocation,
}
