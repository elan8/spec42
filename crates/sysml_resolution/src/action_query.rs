//! Closed exact Systems::Actions derived-property query vocabulary.

use crate::SymbolId;
pub use spec42_constraint_manifest::ActionDerivedFactKind;
pub use sysml_contract::{ActionDerivedFactCollection, ActionDerivedFactPrerequisite};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionDerivedFactOutcome {
    Values(Box<[SymbolId]>),
    Unsupported {
        prerequisite: ActionDerivedFactPrerequisite,
    },
}
