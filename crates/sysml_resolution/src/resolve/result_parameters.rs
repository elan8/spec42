//! The result redefinitions of KerML `checkFeatureResultRedefinition` (KerML 8.3.3.3.4).
//!
//! A Feature that is the `result` of its owning Function or Expression must redefine the `result`
//! of every direct supertype of that owner that is itself a Function or Expression. A result is
//! the `out` parameter a Type owns through a `ReturnParameterMembership` -- an authored `return`,
//! or the result Feature lowering mints for each value and instantiation Expression -- or, for a
//! Type that owns none, the result it inherits (the Pilot's `TypeUtil.getResultParameterOf`).
//!
//! Both sides are owned facts derived here, once, over the caller's settled direct specialization
//! edges: phase 4 synthesizes the implied redefinitions from them (as the Pilot's
//! `FeatureAdapter.getParameterRelevantFeatures` adds them) and the redefinition check reads the
//! same derivation, so the two can never disagree about a Type's result.

use std::collections::BTreeSet;

use crate::lower::storage::SemanticModelStorage;
use crate::model::element_kind::element_kind;
use crate::model::element_kind::membership_role_with_trigger;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::ReferenceKind;
use crate::resolve::inherited_members::derive_inherited_members;
use crate::resolve::inherited_members::InheritedMember;
use crate::resolve::inherited_members::OwnedMembers;
use crate::resolve::results::ImpliedRelationship;
use crate::resolve::results::ResolutionError;
use crate::ElementKind;
use crate::MembershipRole;

/// Whether `kind` is a KerML `Function` or `Expression` (including every SysML calculation,
/// constraint, requirement and case definition and usage).
pub(crate) fn is_function_or_expression(kind: DeclarationKind) -> bool {
    let kind = element_kind(kind);
    kind.conforms_to(ElementKind::Function) || kind.conforms_to(ElementKind::Expression)
}

/// The result parameter each Function or Expression owns, indexed by owner.
///
/// An owned Feature whose effective membership role is `ReturnParameter`, and the result Feature
/// lowering mints for a value or instantiation Expression (`DeclarationFacts::expression_result`).
pub(crate) fn owned_result_parameters(
    storage: &SemanticModelStorage,
) -> Result<OwnedMembers, ResolutionError> {
    let mut owned = OwnedMembers::new(storage.declarations.len());
    for membership in storage.memberships.iter() {
        let declaration = storage
            .declaration(membership.member)
            .ok_or(ResolutionError::InvalidStorage)?;
        let facts = storage
            .declaration_facts(membership.member)
            .ok_or(ResolutionError::InvalidStorage)?;
        let role = membership
            .role
            .or_else(|| membership_role_with_trigger(declaration.kind, facts.is_trigger_action));
        if role != Some(MembershipRole::ReturnParameter) {
            continue;
        }
        if let Some(owner) = declaration.owner {
            owned.insert(owner, membership.member)?;
        }
    }
    for (index, facts) in storage.declaration_facts.iter().enumerate() {
        if let Some(result) = facts.expression_result {
            let expression =
                DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            owned.insert(expression, result)?;
        }
    }
    Ok(owned)
}

/// One `checkFeatureResultRedefinition` obligation: `result`, owned by `owner`, must redefine the
/// result of `general`, a direct supertype of `owner`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResultObligation {
    pub(crate) owner: DeclarationId,
    pub(crate) result: DeclarationId,
    pub(crate) general: DeclarationId,
    pub(crate) target: InheritedMember,
}

/// Every declaration's result and the publication's result obligations, in owner then supertype
/// order.
///
/// A Function or Expression that owns a result is paired with each direct supertype that is a
/// Function or Expression and has a result other than this one. A supertype with no result
/// contributes nothing; one whose result is ambiguous is an obligation with an unsettled target.
pub(crate) fn result_obligations<F>(
    storage: &SemanticModelStorage,
    generals: F,
    authored: &BTreeSet<(DeclarationId, DeclarationId)>,
) -> Result<Vec<ResultObligation>, ResolutionError>
where
    F: Fn(DeclarationId) -> Vec<DeclarationId>,
{
    let owned = owned_result_parameters(storage)?;
    let results = derive_inherited_members(&owned, &generals, authored)?;
    let is_function_or_expression_declaration = |declaration: DeclarationId| {
        storage
            .declaration(declaration)
            .map(|declaration| is_function_or_expression(declaration.kind))
            .ok_or(ResolutionError::InvalidStorage)
    };
    let mut obligations = Vec::new();
    for (owner, result) in owned.iter() {
        if !is_function_or_expression_declaration(owner)? {
            continue;
        }
        for general in generals(owner) {
            if general == owner || !is_function_or_expression_declaration(general)? {
                continue;
            }
            let target = results
                .get(general.index())
                .copied()
                .ok_or(ResolutionError::InvalidStorage)?;
            if target != InheritedMember::Absent && target != InheritedMember::Resolved(result) {
                obligations.push(ResultObligation {
                    owner,
                    result,
                    general,
                    target,
                });
            }
        }
    }
    Ok(obligations)
}

/// Synthesizes the result Redefinitions over `type_edges`, the settled direct specialization edges
/// `(specific, general)` (authored and implied, including implied library supertypes such as
/// `Performances::Evaluation` and the FeatureTyping of an invocation by its Function). A
/// redefinition an authored edge already states (`authored`) is not restated; an ambiguous
/// target publishes nothing and the check answers it as unresolved.
pub(crate) fn synthesize_result_redefinitions(
    storage: &SemanticModelStorage,
    type_edges: impl IntoIterator<Item = (DeclarationId, DeclarationId)>,
    authored: &BTreeSet<(DeclarationId, DeclarationId)>,
) -> Result<Vec<ImpliedRelationship>, ResolutionError> {
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
    let mut implied = Vec::new();
    for obligation in result_obligations(storage, generals_of, authored)? {
        let InheritedMember::Resolved(target) = obligation.target else {
            continue;
        };
        if !authored.contains(&(obligation.result, target)) {
            implied.push(ImpliedRelationship {
                kind: ReferenceKind::Redefinition,
                source: obligation.result,
                target,
            });
        }
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied)
}
