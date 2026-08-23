//! What a diagram projection presents, named rather than described.
//!
//! Only the closed vocabulary lives here: which of SysML's eight view kinds a projection is, which
//! compartment a member is presented in and whether that membership is direct or inherited, and
//! what role a vertex plays in a state machine. The projected scene itself -- its elements, edges
//! and incompleteness reasons -- carries model identities and authored text, so it stays with the
//! authority that computes it.

/// One of the eight view kinds SysML's standard view definitions declare.
///
/// A closed set fixed by the specification, not an open registry: a consumer matching on it is
/// exhaustive today and stays exhaustive until OMG adds a view definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagramViewKind {
    General,
    Interconnection,
    ActionFlow,
    StateTransition,
    Sequence,
    Browser,
    Grid,
    Geometry,
}

impl DiagramViewKind {
    /// Every view kind, in specification order, so a catalog has one fixed order.
    pub const ALL: [Self; 8] = [
        Self::General,
        Self::Interconnection,
        Self::ActionFlow,
        Self::StateTransition,
        Self::Sequence,
        Self::Browser,
        Self::Grid,
        Self::Geometry,
    ];

    /// A stable kebab-case identifier, for catalog keys and snapshot output.
    pub const fn id(self) -> &'static str {
        match self {
            Self::General => "general-view",
            Self::Interconnection => "interconnection-view",
            Self::ActionFlow => "action-flow-view",
            Self::StateTransition => "state-transition-view",
            Self::Sequence => "sequence-view",
            Self::Browser => "browser-view",
            Self::Grid => "grid-view",
            Self::Geometry => "geometry-view",
        }
    }
}

/// Which compartment of a presented element a member appears in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagramCompartmentKind {
    Attributes,
    Parts,
    Ports,
    Items,
    Constraints,
    Requirements,
    Actions,
    States,
    Calculations,
    Connections,
    Interfaces,
    Occurrences,
}

/// Whether a compartment's members are declared on the element or inherited into it.
///
/// Kept beside the compartment rather than folded into it: a viewer that dims inherited members
/// needs the distinction, and a compartment that mixed the two could not supply it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagramCompartmentProvenance {
    Direct,
    Inherited,
}

/// The role one vertex plays in a projected state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagramStateVertexKind {
    Initial,
    State,
    Final,
}
