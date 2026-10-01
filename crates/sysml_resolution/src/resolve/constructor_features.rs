//! The result-feature redefinitions of KerML `checkConstructorExpressionResultFeatureRedefinition`
//! (KerML 8.3.4.8.3).
//!
//! Each owned Feature of a ConstructorExpression's result -- one per authored argument -- must
//! redefine exactly one public `feature` of the expression's `instantiatedType`. A named argument
//! (`new P(x = a)`) owns the authored Redefinition of the feature it names; a positional argument
//! redefines the public feature at its position (the Pilot's
//! `FeatureAdapter.getConstructorRelevantFeatures`). A Type's public features are derived here,
//! once, for the instantiated types that need them; synthesis and check read the same derivation.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::lower::storage::SemanticModelStorage;
use crate::model::DeclarationId;
use crate::model::MembershipKind;
use crate::model::ReferenceKind;
use crate::model::Visibility;
use crate::resolve::results::ConstructorExpressionProjection;
use crate::resolve::results::ImpliedRelationship;
use crate::resolve::results::ResolutionError;

/// KerML `Type::feature` of each of `roots`, restricted to members of public (or default, which is
/// public) FeatureMembership: the Type's owned features in authored order, then the features of
/// each direct supertype in the order `generals` yields them, less any feature another one in the
/// collection redefines (inherited memberships exclude redefined features). A supertype on a
/// specialization cycle with the Type contributes nothing.
///
/// `redefines` yields the settled (authored and implied) redefinition targets of a Feature.
pub(crate) fn public_features<G, R>(
    storage: &SemanticModelStorage,
    roots: impl IntoIterator<Item = DeclarationId>,
    generals: G,
    redefines: R,
) -> Result<BTreeMap<DeclarationId, Vec<DeclarationId>>, ResolutionError>
where
    G: Fn(DeclarationId) -> Vec<DeclarationId>,
    R: Fn(DeclarationId) -> Vec<DeclarationId>,
{
    let mut owned: BTreeMap<DeclarationId, Vec<DeclarationId>> = BTreeMap::new();
    for membership in storage.memberships.iter() {
        if membership.kind != MembershipKind::Feature
            || !matches!(
                membership.visibility,
                Visibility::Default | Visibility::Public
            )
        {
            continue;
        }
        let Some(owner) = storage
            .declaration(membership.member)
            .ok_or(ResolutionError::InvalidStorage)?
            .owner
        else {
            continue;
        };
        owned.entry(owner).or_default().push(membership.member);
    }
    for members in owned.values_mut() {
        members.sort_unstable();
        members.dedup();
    }
    let mut rows: BTreeMap<DeclarationId, Vec<DeclarationId>> = BTreeMap::new();
    let mut in_progress = BTreeSet::new();
    let mut stack = Vec::new();
    for root in roots {
        stack.push((root, false));
        while let Some((node, expanded)) = stack.pop() {
            if !expanded {
                if rows.contains_key(&node) || !in_progress.insert(node) {
                    continue;
                }
                stack.push((node, true));
                for general in generals(node).into_iter().rev() {
                    if !rows.contains_key(&general) && !in_progress.contains(&general) {
                        stack.push((general, false));
                    }
                }
                continue;
            }
            let mut row = owned.get(&node).cloned().unwrap_or_default();
            for general in generals(node) {
                if let Some(inherited) = rows.get(&general) {
                    for member in inherited {
                        if !row.contains(member) {
                            row.push(*member);
                        }
                    }
                }
            }
            let redefined = row
                .iter()
                .flat_map(|member| redefines(*member))
                .collect::<BTreeSet<_>>();
            row.retain(|member| !redefined.contains(member));
            in_progress.remove(&node);
            rows.insert(node, row);
        }
    }
    Ok(rows)
}

/// One positional argument Feature of a constructor result and the public feature of the
/// instantiated type at its position, if there is one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ConstructorArgumentObligation {
    pub(crate) argument: DeclarationId,
    pub(crate) target: Option<DeclarationId>,
}

/// The owned Features of each projected constructor result, in authored order.
pub(crate) fn constructor_arguments(
    storage: &SemanticModelStorage,
    projections: &[ConstructorExpressionProjection],
) -> Result<BTreeMap<DeclarationId, Vec<DeclarationId>>, ResolutionError> {
    let results = projections
        .iter()
        .map(|projection| projection.result)
        .collect::<BTreeSet<_>>();
    let mut arguments: BTreeMap<DeclarationId, Vec<DeclarationId>> = BTreeMap::new();
    for (index, declaration) in storage.declarations.iter().enumerate() {
        let Some(owner) = declaration.owner.filter(|owner| results.contains(owner)) else {
            continue;
        };
        let argument = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        if storage
            .declaration_facts(argument)
            .is_some_and(|facts| facts.instantiation_argument.is_some())
        {
            arguments.entry(owner).or_default().push(argument);
        }
    }
    Ok(arguments)
}

/// The positional obligation of every positional argument of every projected constructor.
pub(crate) fn constructor_argument_obligations<G, R>(
    storage: &SemanticModelStorage,
    projections: &[ConstructorExpressionProjection],
    generals: G,
    redefines: R,
) -> Result<Vec<ConstructorArgumentObligation>, ResolutionError>
where
    G: Fn(DeclarationId) -> Vec<DeclarationId>,
    R: Fn(DeclarationId) -> Vec<DeclarationId>,
{
    let arguments = constructor_arguments(storage, projections)?;
    let features = public_features(
        storage,
        projections
            .iter()
            .map(|projection| projection.instantiated_type),
        generals,
        redefines,
    )?;
    let mut obligations = Vec::new();
    for projection in projections {
        let Some(arguments) = arguments.get(&projection.result) else {
            continue;
        };
        let features = features
            .get(&projection.instantiated_type)
            .map(Vec::as_slice)
            .unwrap_or_default();
        for (position, argument) in arguments.iter().copied().enumerate() {
            let named = storage
                .declaration_facts(argument)
                .and_then(|facts| facts.instantiation_argument)
                .is_some_and(|argument| argument.named);
            if named {
                continue;
            }
            obligations.push(ConstructorArgumentObligation {
                argument,
                target: features.get(position).copied(),
            });
        }
    }
    obligations.sort_unstable();
    Ok(obligations)
}

/// Synthesizes the positional constructor-argument Redefinitions over the settled direct
/// specialization edges and redefinitions.
pub(crate) fn synthesize_constructor_argument_redefinitions(
    storage: &SemanticModelStorage,
    projections: &[ConstructorExpressionProjection],
    type_edges: impl IntoIterator<Item = (DeclarationId, DeclarationId)>,
    redefinitions: &BTreeSet<(DeclarationId, DeclarationId)>,
) -> Result<Vec<ImpliedRelationship>, ResolutionError> {
    if projections.is_empty() {
        return Ok(Vec::new());
    }
    let mut generals: BTreeMap<DeclarationId, BTreeSet<DeclarationId>> = BTreeMap::new();
    for (specific, general) in type_edges {
        generals.entry(specific).or_default().insert(general);
    }
    let generals_of = |declaration: DeclarationId| {
        generals
            .get(&declaration)
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default()
    };
    let redefines = |feature: DeclarationId| {
        redefinitions
            .range((feature, DeclarationId(0))..=(feature, DeclarationId(u32::MAX)))
            .map(|(_, target)| *target)
            .collect()
    };
    let mut implied = Vec::new();
    for obligation in
        constructor_argument_obligations(storage, projections, generals_of, redefines)?
    {
        if let Some(target) = obligation.target {
            implied.push(ImpliedRelationship {
                kind: ReferenceKind::Redefinition,
                source: obligation.argument,
                target,
            });
        }
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied)
}
