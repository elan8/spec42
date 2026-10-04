use crate::{RelationshipProvenance, SourceLocation, SymbolId};
pub use spec42_constraint_manifest::BindingConnectorCheckKind;
pub use sysml_contract::{
    BindingConnectorValidationOutcome, BindingConnectorValidationPrerequisite, SatisfyPolarity,
};

/// The settled target of one directional end of an authored satisfy relationship.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SatisfyEndpoint {
    Resolved(SymbolId),
    /// A feature chain (`satisfy R by a.b.c`), every hop resolved. In SysML v2 the chain as a
    /// whole is the satisfying feature (8.3.21.10, `satisfyingFeature`; the `by` operand is an
    /// `OwnedFeatureChain`, 8.2.2.21.2), which is not the declaration its last hop names.
    FeatureChain {
        /// The feature each authored segment resolves to, in order: `path[0]` is the root
        /// (`a`), the last entry the feature the chain ends at (`c`).
        path: Box<[SymbolId]>,
        /// The authored segments, `::`-joined like every published authored path (a connector's
        /// `ConnectorEndpoint::FeatureChain::authored` too): `a::b::c` for `a.b.c`.
        authored: Box<str>,
    },
    Ambiguous(Box<[SymbolId]>),
    Unresolved,
    Unsupported,
}

/// One authoritative `satisfy <requirement> by <element>` statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SatisfyRelationship {
    /// Stable identity of the anonymous relationship usage, preserving duplicates.
    pub identity: SymbolId,
    /// The `satisfy` operand: the requirement being satisfied.
    pub requirement: SatisfyEndpoint,
    /// The `by` operand: the element claimed to satisfy the requirement.
    pub satisfying_element: SatisfyEndpoint,
    pub polarity: SatisfyPolarity,
    pub provenance: RelationshipProvenance,
    pub location: SourceLocation,
}

/// The settled target of one end of a requirement derivation connection.
///
/// The same outcome shape as [`SatisfyEndpoint`], kept a distinct type (as [`BindingEndpoint`]
/// is) so a derivation end cannot be mistaken for a satisfy claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DerivationEndpoint {
    Resolved(SymbolId),
    /// A feature chain end (`end #derive ::> spec.engineMass`), every hop resolved, in order.
    FeatureChain {
        path: Box<[SymbolId]>,
        /// The authored segments, `::`-joined like every published authored path.
        authored: Box<str>,
    },
    Ambiguous(Box<[SymbolId]>),
    Unresolved,
    Unsupported,
}

/// One requirement derivation (the `RequirementDerivation` domain library): a connection usage
/// that is a `DerivationConnections::Derivation`, whether by `#derivation` metadata or by
/// typing. Its ends are classified by what they specialize: `original` ends conform to
/// `DerivationConnections::originalRequirements`, `derived` ends to `derivedRequirements`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivationRelationship {
    /// The derivation connection usage. Distinct connections stay distinct.
    pub identity: SymbolId,
    /// The original requirement ends. `Derivation` requires exactly one; more or none is a
    /// modelling error the consumer can see here.
    pub original: Box<[DerivationEndpoint]>,
    /// The derived requirement ends, in authored order (`Derivation` requires at least one).
    pub derived: Box<[DerivationEndpoint]>,
    /// Ends that specialize neither, e.g. a bare positional end: reported, not dropped.
    pub unclassified: Box<[DerivationEndpoint]>,
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
    Resolved(SymbolId),
    Ambiguous(Box<[SymbolId]>),
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
    pub identity: SymbolId,
    pub source: BindingEndpoint,
    pub target: BindingEndpoint,
    pub provenance: RelationshipProvenance,
    pub location: SourceLocation,
}
