//! The objective redefinitions of SysML `checkRequirementUsageObjectiveRedefinition`.
//!
//! A `RequirementUsage` owned through an `ObjectiveMembership` must redefine the
//! `objectiveRequirement` of every `CaseDefinition` its owning Type directly specializes, and of
//! every `CaseUsage` that is the `featureTarget` of a Feature its owning Type directly specializes
//! (SysML 8.3.21.9). Both sides are owned facts: the objective role is the
//! `DeclarationKind::ObjectiveRequirement` membership role, and a case's `objectiveRequirement` is
//! derived here, once, over the caller's settled direct specialization edges.
//!
//! The same derivation serves phase 4's synthesis of the implied redefinitions (as the Pilot's
//! `RequirementUsageAdapter` adds them) and the redefinition check, so the two can never read a
//! case's objective differently.

use std::collections::BTreeSet;

use crate::lower::storage::SemanticModelStorage;
use crate::model::element_kind::element_kind;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::ReferenceKind;
use crate::resolve::inherited_members::derive_inherited_members;
use crate::resolve::inherited_members::InheritedMember;
use crate::resolve::inherited_members::OwnedMembers;
use crate::resolve::results::ImpliedRelationship;
use crate::resolve::results::ResolutionError;
use crate::ElementKind;

/// Whether `kind` is a SysML `CaseDefinition` (including its analysis, verification and use case
/// specializations).
pub(crate) fn is_case_definition(kind: DeclarationKind) -> bool {
    matches!(
        element_kind(kind),
        ElementKind::CaseDefinition
            | ElementKind::AnalysisCaseDefinition
            | ElementKind::VerificationCaseDefinition
            | ElementKind::UseCaseDefinition
    )
}

/// Whether `kind` is a SysML `CaseUsage` (including its analysis, verification and use case
/// specializations).
pub(crate) fn is_case_usage(kind: DeclarationKind) -> bool {
    matches!(
        element_kind(kind),
        ElementKind::CaseUsage
            | ElementKind::AnalysisCaseUsage
            | ElementKind::VerificationCaseUsage
            | ElementKind::UseCaseUsage
    )
}

/// A case's `objectiveRequirement`, as far as this publication settles it.
pub(crate) type ObjectiveRequirement = InheritedMember;

/// Every declaration's `objectiveRequirement`, indexed by declaration.
///
/// A declaration's owned objective is its `ObjectiveRequirement` member; the rest is the shared
/// [`derive_inherited_members`] derivation.
pub(crate) fn derive_objective_requirements<F>(
    storage: &SemanticModelStorage,
    generals: F,
    authored: &BTreeSet<(DeclarationId, DeclarationId)>,
) -> Result<Vec<ObjectiveRequirement>, ResolutionError>
where
    F: Fn(DeclarationId) -> Vec<DeclarationId>,
{
    let mut owned = OwnedMembers::new(storage.declarations.len());
    for (index, declaration) in storage.declarations.iter().enumerate() {
        if declaration.kind != DeclarationKind::ObjectiveRequirement {
            continue;
        }
        let Some(owner) = declaration.owner else {
            continue;
        };
        let objective = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        owned.insert(owner, objective)?;
    }
    derive_inherited_members(&owned, generals, authored)
}

/// What one objective must redefine for one direct supertype of its owning Type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ObjectiveObligation {
    pub(crate) objective: DeclarationId,
    pub(crate) general: DeclarationId,
    pub(crate) target: ObjectiveRequirement,
}

/// Every objective obligation of the publication, in objective then supertype order.
///
/// A supertype that is a `CaseDefinition` or a `CaseUsage` (a Feature that is its own
/// `featureTarget`) contributes its `objectiveRequirement`; one without an objective, or the
/// objective itself, contributes nothing.
pub(crate) fn objective_obligations<F>(
    storage: &SemanticModelStorage,
    generals: F,
    objectives: &[ObjectiveRequirement],
) -> Result<Vec<ObjectiveObligation>, ResolutionError>
where
    F: Fn(DeclarationId) -> Vec<DeclarationId>,
{
    let mut obligations = Vec::new();
    for (index, declaration) in storage.declarations.iter().enumerate() {
        if declaration.kind != DeclarationKind::ObjectiveRequirement {
            continue;
        }
        let Some(owner) = declaration.owner else {
            continue;
        };
        let objective = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        for general in generals(owner) {
            let general_kind = storage
                .declaration(general)
                .ok_or(ResolutionError::InvalidStorage)?
                .kind;
            if general == owner
                || !(is_case_definition(general_kind) || is_case_usage(general_kind))
            {
                continue;
            }
            let target = objectives
                .get(general.index())
                .copied()
                .ok_or(ResolutionError::InvalidStorage)?;
            if target != ObjectiveRequirement::Absent
                && target != ObjectiveRequirement::Resolved(objective)
            {
                obligations.push(ObjectiveObligation {
                    objective,
                    general,
                    target,
                });
            }
        }
    }
    Ok(obligations)
}

/// Synthesizes the objective Redefinitions over `type_edges`, the settled direct specialization
/// edges `(specific, general)` (authored and implied, including implied library supertypes such as
/// `Cases::Case`). A redefinition an authored edge already states (`authored`) is not restated; an
/// ambiguous target publishes nothing and the check answers it as unresolved.
pub(crate) fn synthesize_objective_redefinitions(
    storage: &SemanticModelStorage,
    type_edges: impl IntoIterator<Item = (DeclarationId, DeclarationId)>,
    authored: &BTreeSet<(DeclarationId, DeclarationId)>,
) -> Result<Vec<ImpliedRelationship>, ResolutionError> {
    if !storage
        .declarations
        .iter()
        .any(|declaration| declaration.kind == DeclarationKind::ObjectiveRequirement)
    {
        return Ok(Vec::new());
    }
    let mut generals = vec![BTreeSet::new(); storage.declarations.len()];
    for (specific, general) in type_edges {
        generals
            .get_mut(specific.index())
            .ok_or(ResolutionError::InvalidStorage)?
            .insert(general);
    }
    let generals_of = |declaration: DeclarationId| {
        generals
            .get(declaration.index())
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default()
    };
    let objectives = derive_objective_requirements(storage, generals_of, authored)?;
    let mut implied = Vec::new();
    for obligation in objective_obligations(storage, generals_of, &objectives)? {
        let ObjectiveRequirement::Resolved(target) = obligation.target else {
            continue;
        };
        if !authored.contains(&(obligation.objective, target)) {
            implied.push(ImpliedRelationship {
                kind: ReferenceKind::Redefinition,
                source: obligation.objective,
                target,
            });
        }
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied)
}
