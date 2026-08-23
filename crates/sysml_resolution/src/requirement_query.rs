//! Exact Systems::Requirements derived-property query vocabulary.
//!
//! The manifest identifies each full pinned property. Resolver implementations consume only
//! canonical direct feature membership roles and documentation records; they do not reparse
//! requirement syntax or infer a role from a member name.

use crate::SymbolIdentity;

pub use spec42_constraint_manifest::RequirementDerivedFactKind;
pub use sysml_contract::{RequirementDerivedFactCollection, RequirementDerivedFactPrerequisite};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequirementDerivedFactOutcome {
    Elements(Box<[SymbolIdentity]>),
    Text(Box<[Box<str>]>),
    Unsupported {
        prerequisite: RequirementDerivedFactPrerequisite,
    },
}

/// The collection one manifest-published rule kind selects.
///
/// A free fn rather than a method: the mapping is between the pinned manifest's rule table and the
/// contract vocabulary, and the contract crate does not depend on the manifest.
pub const fn requirement_collection_from_kind(
    kind: RequirementDerivedFactKind,
) -> RequirementDerivedFactCollection {
    use RequirementDerivedFactCollection as Collection;
    match kind {
        RequirementDerivedFactKind::DefinitionActorParameter => {
            Collection::DefinitionActorParameter
        }
        RequirementDerivedFactKind::DefinitionSubjectParameter => {
            Collection::DefinitionSubjectParameter
        }
        RequirementDerivedFactKind::DefinitionText => Collection::DefinitionText,
        RequirementDerivedFactKind::DefinitionRequiredConstraint => {
            Collection::DefinitionRequiredConstraint
        }
        RequirementDerivedFactKind::DefinitionAssumedConstraint => {
            Collection::DefinitionAssumedConstraint
        }
        RequirementDerivedFactKind::DefinitionFramedConcern => Collection::DefinitionFramedConcern,
        RequirementDerivedFactKind::UsageActorParameter => Collection::UsageActorParameter,
        RequirementDerivedFactKind::UsageSubjectParameter => Collection::UsageSubjectParameter,
        RequirementDerivedFactKind::UsageText => Collection::UsageText,
        RequirementDerivedFactKind::UsageRequiredConstraint => Collection::UsageRequiredConstraint,
        RequirementDerivedFactKind::UsageAssumedConstraint => Collection::UsageAssumedConstraint,
        RequirementDerivedFactKind::UsageFramedConcern => Collection::UsageFramedConcern,
    }
}
