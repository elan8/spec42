//! Closed exact Systems::Actions derived-property query vocabulary.

use crate::SymbolIdentity;
pub use spec42_constraint_manifest::ActionDerivedFactKind;
pub use sysml_contract::{ActionDerivedFactCollection, ActionDerivedFactPrerequisite};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionDerivedFactOutcome {
    Values(Box<[SymbolIdentity]>),
    Unsupported {
        prerequisite: ActionDerivedFactPrerequisite,
    },
}
