//! Publication-borrowed effective identification and its naming provenance.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectiveName<'a> {
    Resolved(&'a str),
    Absent,
    Unresolved,
    NonConverged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectiveNameProvenance {
    /// The element's own authored identification, or none.
    Declared,
    /// The first redefined feature supplies both naming components (KerML 8.3.3.3.4).
    FirstRedefinition,
    /// The action a perform usage performs supplies the identification.
    PerformedAction,
    /// The original port definition supplies the conjugated identification.
    OriginalPortDefinition,
}

/// Semantic identification, separate from the syntax-owned authored names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EffectiveIdentification<'a> {
    pub name: EffectiveName<'a>,
    pub short_name: EffectiveName<'a>,
    pub provenance: EffectiveNameProvenance,
}
