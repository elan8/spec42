//! Exact derived Feature relationship-collection contracts.
//!
//! These are views over the publication's one canonical relationship store. They deliberately do
//! not materialize a second collection of relationship facts: provenance, unresolved targets, and
//! implied edges remain exactly those published for ordinary element inspection.

/// One closed exact relationship collection derived for a KerML `Feature`.
///
/// Each variant is generated from a complete pinned-XMI derivation body. Complex OCL derivations
/// such as `deriveFeatureType` are intentionally absent until their complete canonical inputs and
/// fixed-point contract are owned by the semantic layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FeatureDerivedRelationshipCollection {
    OwnedFeatureChaining,
    OwnedRedefinition,
    OwnedSubsetting,
    OwnedTyping,
    OwnedTypeFeaturing,
}
