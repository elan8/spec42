use crate::{RelationshipProvenance, SourceLocation, SymbolIdentity};
pub use spec42_constraint_manifest::BindingConnectorCheckKind;

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

/// Why a named binding-connector validation could not be evaluated from canonical facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingConnectorValidationPrerequisite {
    /// Lowering has not yet published the `FeatureReferenceExpression.targetFeature` and
    /// expression-result facts which the exact rule relates through a binding connector.
    FeatureReferenceExpressionTargetAndResult,
    FeatureValueEndpointFacts,
    ExpressionResultEndpointFacts,
    FunctionResultEndpointFacts,
    InvocationExpressionBehaviorEndpointFacts,
    AcceptActionUsageReceiverEndpointFacts,
    TransitionUsageSourceEndpointFacts,
    TransitionUsageSuccessionEndpointFacts,
    SatisfyRequirementUsageEndpointFacts,
    /// The exact pinned OCL body is `TBD`, so OMG has not supplied an evaluable predicate.
    NormativeSpecificationTbd,
    /// The selected typed rule is not present in the manifest-derived resolver table.
    ///
    /// This protects the query boundary from treating an enum value as evidence that the pinned
    /// manifest actually publishes a corresponding normative contract.
    RuleNotPublished,
}

/// The explicit result of asking for one binding-connector validation.
///
/// This is separate from a connector endpoint's resolution state. An endpoint may be settled
/// while the named validation remains unsupported because the rule's own semantic inputs do not
/// exist in the publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingConnectorValidationOutcome {
    /// The full named predicate held over canonical paired binding and endpoint facts.
    Satisfied,
    /// The full named predicate was evaluable and did not hold.
    Violated,
    /// A required paired connector endpoint is unresolved or ambiguous.
    Unresolved,
    Unsupported {
        prerequisite: BindingConnectorValidationPrerequisite,
    },
}
