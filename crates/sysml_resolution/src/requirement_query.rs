//! Exact Systems::Requirements derived-property query vocabulary.
//!
//! The manifest identifies each full pinned property. Resolver implementations consume only
//! canonical direct feature membership roles and documentation records; they do not reparse
//! requirement syntax or infer a role from a member name.

use crate::SymbolIdentity;

pub use spec42_constraint_manifest::RequirementDerivedFactKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RequirementDerivedFactCollection {
    DefinitionActorParameter,
    DefinitionSubjectParameter,
    DefinitionText,
    DefinitionRequiredConstraint,
    DefinitionAssumedConstraint,
    DefinitionFramedConcern,
    UsageActorParameter,
    UsageSubjectParameter,
    UsageText,
    UsageRequiredConstraint,
    UsageAssumedConstraint,
    UsageFramedConcern,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequirementDerivedFactPrerequisite {
    RuleNotPublished,
    CanonicalMembershipRole,
    DocumentationRecords,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequirementDerivedFactOutcome {
    Elements(Box<[SymbolIdentity]>),
    Text(Box<[Box<str>]>),
    Unsupported {
        prerequisite: RequirementDerivedFactPrerequisite,
    },
}

impl RequirementDerivedFactCollection {
    pub const fn from_kind(kind: RequirementDerivedFactKind) -> Self {
        match kind {
            RequirementDerivedFactKind::DefinitionActorParameter => Self::DefinitionActorParameter,
            RequirementDerivedFactKind::DefinitionSubjectParameter => {
                Self::DefinitionSubjectParameter
            }
            RequirementDerivedFactKind::DefinitionText => Self::DefinitionText,
            RequirementDerivedFactKind::DefinitionRequiredConstraint => {
                Self::DefinitionRequiredConstraint
            }
            RequirementDerivedFactKind::DefinitionAssumedConstraint => {
                Self::DefinitionAssumedConstraint
            }
            RequirementDerivedFactKind::DefinitionFramedConcern => Self::DefinitionFramedConcern,
            RequirementDerivedFactKind::UsageActorParameter => Self::UsageActorParameter,
            RequirementDerivedFactKind::UsageSubjectParameter => Self::UsageSubjectParameter,
            RequirementDerivedFactKind::UsageText => Self::UsageText,
            RequirementDerivedFactKind::UsageRequiredConstraint => Self::UsageRequiredConstraint,
            RequirementDerivedFactKind::UsageAssumedConstraint => Self::UsageAssumedConstraint,
            RequirementDerivedFactKind::UsageFramedConcern => Self::UsageFramedConcern,
        }
    }

    pub const fn requires_text(self) -> bool {
        matches!(self, Self::DefinitionText | Self::UsageText)
    }
}
