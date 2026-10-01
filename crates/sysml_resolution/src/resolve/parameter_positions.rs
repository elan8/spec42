//! The positional parameter redefinitions of KerML `checkFeatureParameterRedefinition`
//! (KerML 8.3.3.3.4).
//!
//! A parameter of a Behavior or Step -- an owned Feature with a direction that is not its owner's
//! result -- redefines the parameter at the same position of every direct supertype of its owner
//! that is itself a Behavior or Step. A supertype's parameters at each position are its own
//! parameters, then those it inherits past them (the Pilot's `TypeUtil.getAllParametersOf`, i.e.
//! `Type::directedFeature` less the result), derived here, once, by the same positional derivation
//! as `Type::endFeature`. A parameter of an InvocationExpression that names the parameter it binds
//! (`f(x = a)`) owns that authored redefinition instead, and is excluded, as the OCL excludes it.
//!
//! Phase 4 synthesizes the implied redefinitions from these obligations (as the Pilot's
//! `FeatureAdapter.getRelevantParameters` pairs them) and the redefinition check reads the same
//! derivation, so the two can never disagree about a position.

use std::collections::BTreeSet;

use crate::lower::facts::InstantiationForm;
use crate::lower::facts::UnloweredExpression;
use crate::lower::storage::SemanticModelStorage;
use crate::model::element_kind::effective_direction;
use crate::model::element_kind::element_kind;
use crate::model::element_kind::membership_role_with_trigger;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::MembershipKind;
use crate::model::ReferenceKind;
use crate::resolve::end_features::derive_positional_features;
use crate::resolve::result_parameters::owned_result_parameters;
use crate::resolve::results::ImpliedRelationship;
use crate::resolve::results::ResolutionError;
use crate::ElementKind;
use crate::MembershipRole;

/// Whether `kind` is a KerML `Behavior` or `Step` (including every SysML action, calculation,
/// constraint, requirement, case and state definition and usage, and every Expression).
pub(crate) fn is_behavior_or_step(kind: DeclarationKind) -> bool {
    let kind = element_kind(kind);
    kind.conforms_to(ElementKind::Behavior) || kind.conforms_to(ElementKind::Step)
}

/// Whether the SysML grammar gives a usage of `kind` parameters lowering does not mint: the
/// payload, sender and receiver of `send`; the payload and receiver of `accept`; the
/// `replacementValues` of `assign`; the condition and clause parameters of `if`, `while`/`loop`
/// and `for`; the transition's input parameters; the occurrence a `terminate` terminates.
pub(crate) fn has_unlowered_grammar_parameters(kind: DeclarationKind) -> bool {
    matches!(
        element_kind(kind),
        ElementKind::SendActionUsage
            | ElementKind::AcceptActionUsage
            | ElementKind::AssignmentActionUsage
            | ElementKind::IfActionUsage
            | ElementKind::WhileLoopActionUsage
            | ElementKind::ForLoopActionUsage
            | ElementKind::TransitionUsage
            | ElementKind::TerminateActionUsage
    )
}

/// Whether some grammar-defined parameter of this publication is not lowered, so the positional
/// obligations it would carry are not facts: an expression lowering does not represent with its
/// parameters, or a usage whose metaclass has [`has_unlowered_grammar_parameters`].
pub(crate) fn grammar_parameters_are_unlowered(storage: &SemanticModelStorage) -> bool {
    storage.unlowered_expressions.iter().any(|site| {
        matches!(
            site.kind,
            UnloweredExpression::Element | UnloweredExpression::Parameters
        )
    }) || storage
        .declarations
        .iter()
        .any(|declaration| has_unlowered_grammar_parameters(declaration.kind))
}

/// The parameters each Type owns, in authored order, indexed by owner: its owned Features with a
/// direction (authored, or fixed by a `ParameterMembership`) that are not its result.
pub(crate) fn owned_parameters(
    storage: &SemanticModelStorage,
) -> Result<Vec<Vec<DeclarationId>>, ResolutionError> {
    let results = owned_result_parameters(storage)?
        .iter()
        .map(|(_, result)| result)
        .collect::<BTreeSet<_>>();
    let mut roles = vec![None; storage.declarations.len()];
    for membership in storage.memberships.iter() {
        if membership.kind != MembershipKind::Feature {
            continue;
        }
        let declaration = storage
            .declaration(membership.member)
            .ok_or(ResolutionError::InvalidStorage)?;
        let facts = storage
            .declaration_facts(membership.member)
            .ok_or(ResolutionError::InvalidStorage)?;
        *roles
            .get_mut(membership.member.index())
            .ok_or(ResolutionError::InvalidStorage)? = Some(
            membership
                .role
                .or_else(|| membership_role_with_trigger(declaration.kind, facts.is_trigger_action)),
        );
    }
    let mut owned = vec![Vec::new(); storage.declarations.len()];
    for (index, declaration) in storage.declarations.iter().enumerate() {
        let Some(role) = roles[index] else {
            continue;
        };
        let Some(owner) = declaration.owner else {
            continue;
        };
        let member = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        if role == Some(MembershipRole::ReturnParameter) || results.contains(&member) {
            continue;
        }
        let facts = storage
            .declaration_facts(member)
            .ok_or(ResolutionError::InvalidStorage)?;
        if effective_direction(role, facts.direction).is_none() {
            continue;
        }
        owned
            .get_mut(owner.index())
            .ok_or(ResolutionError::InvalidStorage)?
            .push(member);
    }
    Ok(owned)
}

/// One `checkFeatureParameterRedefinition` obligation: the parameter at `position` of `owner`
/// must redefine `target`, the parameter at the same position of `general`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ParameterObligation {
    pub(crate) owner: DeclarationId,
    pub(crate) position: usize,
    pub(crate) general: DeclarationId,
    pub(crate) source: DeclarationId,
    pub(crate) target: DeclarationId,
}

/// Every positional parameter obligation of the publication, in owner, position and supertype
/// order. `generals` yields each declaration's direct supertypes.
pub(crate) fn parameter_obligations<F>(
    storage: &SemanticModelStorage,
    generals: F,
) -> Result<Vec<ParameterObligation>, ResolutionError>
where
    F: Fn(DeclarationId) -> Vec<DeclarationId>,
{
    let owned = owned_parameters(storage)?;
    let rows = derive_positional_features(
        storage.declarations.len(),
        |declaration| owned.get(declaration.index()).cloned().unwrap_or_default(),
        |declaration| generals(declaration).into_iter(),
    )?;
    let is_behavior_or_step_declaration = |declaration: DeclarationId| {
        storage
            .declaration(declaration)
            .map(|declaration| is_behavior_or_step(declaration.kind))
            .ok_or(ResolutionError::InvalidStorage)
    };
    let mut obligations = Vec::new();
    for (index, parameters) in owned.iter().enumerate() {
        if parameters.is_empty() {
            continue;
        }
        let owner = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        if !is_behavior_or_step_declaration(owner)? {
            continue;
        }
        let owner_generals = generals(owner)
            .into_iter()
            .filter(|general| *general != owner)
            .map(|general| Ok((general, is_behavior_or_step_declaration(general)?)))
            .collect::<Result<Vec<_>, ResolutionError>>()?;
        for (position, source) in parameters.iter().copied().enumerate() {
            let named_argument = storage
                .declaration_facts(source)
                .and_then(|facts| facts.instantiation_argument)
                .is_some_and(|argument| {
                    argument.named && argument.form == InstantiationForm::Invocation
                });
            if named_argument {
                continue;
            }
            for (general, behavior_or_step) in owner_generals.iter().copied() {
                if !behavior_or_step {
                    continue;
                }
                let Some(target) = rows
                    .get(general.index())
                    .and_then(|row| row.get(position))
                    .copied()
                else {
                    continue;
                };
                if target != source {
                    obligations.push(ParameterObligation {
                        owner,
                        position,
                        general,
                        source,
                        target,
                    });
                }
            }
        }
    }
    Ok(obligations)
}

/// Synthesizes the positional parameter Redefinitions over `type_edges`, the settled direct
/// specialization edges `(specific, general)` (authored and implied, including the FeatureTyping
/// of an invocation by its callee). A redefinition an authored edge already states (`authored`)
/// is not restated.
pub(crate) fn synthesize_parameter_redefinitions(
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
    for obligation in parameter_obligations(storage, generals_of)? {
        if !authored.contains(&(obligation.source, obligation.target)) {
            implied.push(ImpliedRelationship {
                kind: ReferenceKind::Redefinition,
                source: obligation.source,
                target: obligation.target,
            });
        }
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied)
}
