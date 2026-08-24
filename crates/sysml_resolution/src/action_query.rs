//! Closed exact Systems::Actions derived-property query vocabulary.

use crate::SymbolId;
pub use spec42_constraint_manifest::ActionDerivedFactKind;
pub use sysml_contract::{ActionDerivedFactCollection, ActionDerivedFactPrerequisite};

/// Stable identity of one ordered argument expression owned by an action usage.
///
/// Argument expressions are not declarations and therefore must not be represented by a
/// fabricated declaration `SymbolId`. Their identity is the canonical action identity plus the
/// one-based position used by the SysML `ActionUsage::argument(i)` operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActionArgumentId {
    pub action: SymbolId,
    pub position: u32,
}

/// Stable identity of one ordered input parameter owned by an action usage.
///
/// Like [`ActionArgumentId`], this is publication-scoped and uses the one-based position of the
/// normative `ActionUsage::inputParameter(i)` operation. A parameter is not necessarily a named
/// declaration in the surface syntax, so its identity cannot be replaced by a member symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActionInputParameterId {
    pub action: SymbolId,
    pub position: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionDerivedFactOutcome {
    Values(Box<[SymbolId]>),
    Arguments(Box<[ActionArgumentId]>),
    Parameters(Box<[ActionInputParameterId]>),
    Unsupported {
        prerequisite: ActionDerivedFactPrerequisite,
    },
}
