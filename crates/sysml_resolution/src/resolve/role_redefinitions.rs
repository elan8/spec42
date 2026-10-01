//! Library-anchored role redefinitions: the canonical fact family behind the redefinition checks
//! whose required target is one exact standard-library feature.
//!
//! Several normative `check*Redefinition` constraints say "the Feature playing metamodel role R
//! must redefine library feature L" (`redefinesFromLibrary`). The role is an owned semantic fact
//! of the lowered model (a declaration kind, a membership role, or an expression projection),
//! never a name or rendered text. This module owns, once:
//!
//! - which role maps to which library feature ([`LibraryRedefinitionRole::anchor_path`]);
//! - which declarations play each role in a publication ([`library_role_occupants`]);
//! - the implied `Redefinition` edges that satisfy those checks, as the Pilot's adapters add them
//!   ([`synthesize_library_role_redefinitions`]).
//!
//! The redefinition-check query consumes the same occupants and verifies the published edge set,
//! so synthesis and validation can never disagree about what a role is.
//!
//! The anchor paths mirror the pinned OCL bodies (`redefinesFromLibrary('...')`) whose digests
//! the constraint manifest records; a changed body fails the manifest's exact-body match before
//! this table could drift silently.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::lower::storage::SemanticModelStorage;
use crate::model::element_kind::membership_role;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::ReferenceKind;
use crate::redefinition_query::RedefinitionCheckKind;
use crate::resolve::implied::resolve_library_anchor_path;
use crate::resolve::implied::LibrarySpecializationAnchor;
use crate::resolve::results::ImpliedRelationship;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionResults;
use crate::resolve::results::ResolutionStatus;
use crate::MembershipRole;
use crate::StateSubactionKind;

/// A metamodel role whose occupant must redefine one exact standard-library feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LibraryRedefinitionRole {
    /// SysML 8.3.17.9 `checkForLoopActionUsageVarRedefinition`: the `loopVariable` of a
    /// `ForLoopActionUsage`.
    ForLoopVariable,
    /// KerML 8.3.4.8.4 `checkFeatureChainExpressionTargetRedefinition`: the source-target
    /// feature (first owned feature of the first input parameter) of a `FeatureChainExpression`.
    FeatureChainSourceTarget,
    /// SysML 8.3.17.4 `checkActionUsageStateActionRedefinition`, `entry` kind.
    StateEntryAction,
    /// SysML 8.3.17.4 `checkActionUsageStateActionRedefinition`, `do` kind.
    StateDoAction,
    /// SysML 8.3.17.4 `checkActionUsageStateActionRedefinition`, `exit` kind.
    StateExitAction,
    /// SysML 8.3.17.5 `checkAssignmentActionUsageStartingAtRedefinition`: the first owned
    /// Feature of an `AssignmentActionUsage`'s target parameter.
    AssignmentStartingAt,
    /// SysML 8.3.17.5 `checkAssignmentActionUsageAccessedFeatureRedefinition`: the first owned
    /// Feature of the `startingAt` Feature.
    AssignmentAccessedFeature,
}

impl LibraryRedefinitionRole {
    pub(crate) const ALL: [Self; 7] = [
        Self::ForLoopVariable,
        Self::FeatureChainSourceTarget,
        Self::StateEntryAction,
        Self::StateDoAction,
        Self::StateExitAction,
        Self::AssignmentStartingAt,
        Self::AssignmentAccessedFeature,
    ];

    /// The structural path of the library feature this role's occupant redefines, outermost
    /// first. Segments are decoded names, so `ControlFunctions::'.'` is the segment `.`.
    pub(crate) fn anchor_path(self) -> &'static [&'static str] {
        match self {
            Self::ForLoopVariable => &["Actions", "ForLoopAction", "var"],
            Self::FeatureChainSourceTarget => &["ControlFunctions", ".", "source", "target"],
            Self::StateEntryAction => &["States", "StateAction", "entryAction"],
            Self::StateDoAction => &["States", "StateAction", "doAction"],
            Self::StateExitAction => &["States", "StateAction", "exitAction"],
            // The OCL names `AssignmentAction::target::startingAt`, which the library declares
            // once, on the `onOccurrence` parameter `AssignmentAction::target` redefines; the
            // Pilot's ImplicitGeneralizationMap names the declaring feature, as here.
            Self::AssignmentStartingAt => &[
                "FeatureReferencingPerformances",
                "FeatureAccessPerformance",
                "onOccurrence",
                "startingAt",
            ],
            Self::AssignmentAccessedFeature => &[
                "FeatureReferencingPerformances",
                "FeatureAccessPerformance",
                "onOccurrence",
                "startingAt",
                "accessedFeature",
            ],
        }
    }

    /// The exact redefinition check this role's library redefinition satisfies.
    pub(crate) fn check(self) -> RedefinitionCheckKind {
        match self {
            Self::ForLoopVariable => RedefinitionCheckKind::ForLoopActionUsageVar,
            Self::FeatureChainSourceTarget => RedefinitionCheckKind::FeatureChainExpressionTarget,
            Self::StateEntryAction | Self::StateDoAction | Self::StateExitAction => {
                RedefinitionCheckKind::ActionUsageStateAction
            }
            Self::AssignmentStartingAt => RedefinitionCheckKind::AssignmentActionUsageStartingAt,
            Self::AssignmentAccessedFeature => {
                RedefinitionCheckKind::AssignmentActionUsageAccessedFeature
            }
        }
    }

    /// The root library packages the role anchors live in, so the library closure admits them.
    pub(crate) fn anchor_packages() -> impl Iterator<Item = &'static str> {
        Self::ALL
            .into_iter()
            .filter_map(|role| role.anchor_path().first().copied())
    }
}

/// The settled library feature of every role, resolved once per publication.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct LibraryRoleAnchors {
    pub(crate) by_role: BTreeMap<LibraryRedefinitionRole, LibrarySpecializationAnchor>,
}

impl LibraryRoleAnchors {
    pub(crate) fn resolve(storage: &SemanticModelStorage) -> Self {
        Self {
            by_role: LibraryRedefinitionRole::ALL
                .into_iter()
                .map(|role| {
                    (
                        role,
                        resolve_library_anchor_path(storage, role.anchor_path()),
                    )
                })
                .collect(),
        }
    }

    pub(crate) fn anchor(&self, role: LibraryRedefinitionRole) -> &LibrarySpecializationAnchor {
        self.by_role
            .get(&role)
            .unwrap_or(&LibrarySpecializationAnchor::Missing)
    }
}

/// One declaration playing a library-anchored role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct LibraryRoleOccupant {
    pub(crate) source: DeclarationId,
    pub(crate) role: LibraryRedefinitionRole,
}

/// Every declaration of the publication that plays a library-anchored role, in identity order.
///
/// - A `ForLoopVariable` declaration is the `loopVariable` of the `ForLoop` that owns it.
/// - A feature chain expression's source-target feature is its lowered `source_target`.
/// - An `entry`/`do`/`exit` action is the member of a `StateSubactionMembership`; the kind is the
///   membership role, never the declaration's name.
/// - An assignment's `startingAt` and `accessedFeature` are the Features of its lowered
///   [`crate::lower::facts::AssignmentRecord`].
pub(crate) fn library_role_occupants(
    storage: &SemanticModelStorage,
) -> Result<Vec<LibraryRoleOccupant>, ResolutionError> {
    let mut occupants = Vec::new();
    for (index, declaration) in storage.declarations.iter().enumerate() {
        let source = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        let role = match (declaration.kind, membership_role(declaration.kind)) {
            (DeclarationKind::ForLoopVariable, _) => {
                let owner = declaration
                    .owner
                    .and_then(|owner| storage.declaration(owner))
                    .ok_or(ResolutionError::InvalidStorage)?;
                if owner.kind != DeclarationKind::ForLoop {
                    return Err(ResolutionError::InvalidStorage);
                }
                LibraryRedefinitionRole::ForLoopVariable
            }
            (_, Some(MembershipRole::StateSubaction(kind))) => match kind {
                StateSubactionKind::Entry => LibraryRedefinitionRole::StateEntryAction,
                StateSubactionKind::Do => LibraryRedefinitionRole::StateDoAction,
                StateSubactionKind::Exit => LibraryRedefinitionRole::StateExitAction,
            },
            _ => continue,
        };
        occupants.push(LibraryRoleOccupant { source, role });
    }
    occupants.extend(
        storage
            .feature_chain_expressions
            .iter()
            .map(|chain| LibraryRoleOccupant {
                source: chain.source_target,
                role: LibraryRedefinitionRole::FeatureChainSourceTarget,
            }),
    );
    for assignment in storage.assignments.iter() {
        occupants.push(LibraryRoleOccupant {
            source: assignment.starting_at,
            role: LibraryRedefinitionRole::AssignmentStartingAt,
        });
        occupants.push(LibraryRoleOccupant {
            source: assignment.accessed_feature,
            role: LibraryRedefinitionRole::AssignmentAccessedFeature,
        });
    }
    occupants.sort_unstable();
    occupants.dedup();
    Ok(occupants)
}

/// Synthesizes the implied library redefinition of every role occupant.
///
/// Matching the Pilot (`addImplicitGeneralType`), the edge is implied even when the occupant also
/// authors other redefinitions: a role's normative library redefinition is not a default that an
/// unrelated authored `:>>` replaces. It is omitted only when it would be redundant -- an authored
/// redefinition already settled to the same anchor, which keeps its authored provenance -- or
/// reflexive, as for the library's own declaration of the anchor (`entry action entryAction`).
/// A role whose anchor is missing or ambiguous yields no edge; the check reports it unresolved.
pub(crate) fn synthesize_library_role_redefinitions(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
    anchors: &LibraryRoleAnchors,
) -> Result<Vec<ImpliedRelationship>, ResolutionError> {
    let occupants = library_role_occupants(storage)?;
    if occupants.is_empty() {
        return Ok(Vec::new());
    }
    let mut authored = BTreeSet::new();
    for (index, reference) in storage.references.iter().enumerate() {
        if reference.kind != ReferenceKind::Redefinition {
            continue;
        }
        if let Some(ResolutionStatus::Resolved(target)) = resolution.outcomes.get(index) {
            authored.insert((reference.source, *target));
        }
    }
    let mut implied = Vec::new();
    for occupant in occupants {
        let LibrarySpecializationAnchor::Resolved(anchor) = anchors.anchor(occupant.role) else {
            continue;
        };
        if occupant.source != *anchor && !authored.contains(&(occupant.source, *anchor)) {
            implied.push(ImpliedRelationship {
                kind: ReferenceKind::Redefinition,
                source: occupant.source,
                target: *anchor,
            });
        }
    }
    Ok(implied)
}

/// Synthesizes `checkAssignmentActionUsageReferentRedefinition`: the `accessedFeature` of every
/// assignment redefines the assignment's `referent`, as the Pilot's `FeatureAdapter.
/// addFeatureWriteTypes` adds it. An assignment whose referent did not settle yields no edge; the
/// check reports it unresolved. An authored edge to the same target is not restated.
pub(crate) fn synthesize_assignment_referent_redefinitions(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
) -> Result<Vec<ImpliedRelationship>, ResolutionError> {
    let mut implied = Vec::new();
    for assignment in storage.assignments.iter() {
        let Some(ResolutionStatus::Resolved(referent)) = assignment
            .referent
            .and_then(|referent| resolution.outcome(referent))
        else {
            continue;
        };
        implied.push(ImpliedRelationship {
            kind: ReferenceKind::Redefinition,
            source: assignment.accessed_feature,
            target: referent,
        });
    }
    Ok(implied)
}
