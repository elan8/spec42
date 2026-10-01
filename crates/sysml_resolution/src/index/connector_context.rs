//! KerML `checkConnectorTypeFeaturing` (8.3.4.5.3): the implied context featuring type of a
//! Connector that has no owning Type.
//!
//! A Connector owned by a Type is featured by it through its FeatureMembership. A Connector owned
//! by a Namespace that is not a Type instead gets, as the Pilot's
//! `ConnectorAdapter.addContextFeaturingType` / `ConnectorUtil.getContextTypeFor` give it, the
//! innermost featuring type its related features share (KerML `Connector::defaultFeaturingType`),
//! so that every related feature is featured within it.
//!
//! The derivation consumes owned facts only: the canonical `endFeature` collection and each end's
//! `relatedFeature` member ([`crate::resolve::end_features::end_related_feature`]), the settled
//! featuring rows, and the specialization closure of a prerequisite [`TypeIndex`]. It lives with
//! the type index it reads; the pipeline runs it at the final resolution sub-barrier and settles
//! its TypeFeaturing edges with the other implied relationships, since nothing it reads depends
//! on a Connector's own featuring type.

use std::collections::BTreeMap;

use crate::index::types::ScopeBits;
use crate::index::types::TypeIndex;
use crate::lower::storage::SemanticModelStorage;
use crate::model::element_kind::element_kind;
use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
use crate::model::ReferenceKind;
use crate::resolve::end_features::end_related_feature;
use crate::resolve::end_features::EndRelatedFeature;
use crate::resolve::implied::is_owning_type_kind;
use crate::resolve::results::ImpliedRelationship;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionResults;
use sysml_contract::ElementKind;

/// Whether a declaration is a KerML `Connector` (including its SysML usage forms).
fn is_connector(kind: crate::model::DeclarationKind) -> bool {
    matches!(
        element_kind(kind),
        ElementKind::Connector
            | ElementKind::BindingConnector
            | ElementKind::ConnectionUsage
            | ElementKind::InterfaceUsage
            | ElementKind::AllocationUsage
            | ElementKind::FlowConnectionUsage
            | ElementKind::SuccessionFlowUsage
            | ElementKind::SuccessionAsUsage
            | ElementKind::BindingConnectorAsUsage
    )
}

/// The Connectors this rule applies to: no owning Type and no authored TypeFeaturing.
pub(crate) fn context_featuring_candidates(
    storage: &SemanticModelStorage,
) -> Result<Vec<DeclarationId>, ResolutionError> {
    let authored = storage
        .references
        .iter()
        .filter(|reference| reference.kind == ReferenceKind::TypeFeaturing)
        .map(|reference| reference.source)
        .collect::<std::collections::BTreeSet<_>>();
    let mut candidates = Vec::new();
    for (index, declaration) in storage.declarations.iter().enumerate() {
        if !is_connector(declaration.kind) {
            continue;
        }
        let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        let owned_by_type = match declaration.owner {
            Some(owner) => is_owning_type_kind(
                storage
                    .declaration(owner)
                    .ok_or(ResolutionError::InvalidStorage)?
                    .kind,
            ),
            None => false,
        };
        if !owned_by_type && !authored.contains(&id) {
            candidates.push(id);
        }
    }
    Ok(candidates)
}

/// Synthesizes the context TypeFeaturing of each candidate Connector.
///
/// A Connector with an unsettled related feature, or whose common featuring types cannot be
/// compared by specialization alone (see [`is_compatible`]), receives no context featuring type:
/// the derivation never guesses one.
pub(crate) fn synthesize_connector_context_featurings(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
    types: &TypeIndex,
    candidates: &[DeclarationId],
) -> Result<Box<[ImpliedRelationship]>, ResolutionError> {
    let mut outgoing = BTreeMap::<DeclarationId, Vec<AuthoredReferenceId>>::new();
    for (index, reference) in storage.references.iter().enumerate() {
        outgoing
            .entry(reference.source)
            .or_default()
            .push(AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?);
    }
    let mut implied = Vec::new();
    for connector in candidates {
        let mut related = Vec::new();
        let mut settled = true;
        for end in types.end_features(*connector) {
            match end_related_feature(storage, resolution, end, |declaration| {
                outgoing.get(&declaration).cloned().unwrap_or_default()
            }) {
                EndRelatedFeature::Resolved(feature) => related.push(feature),
                EndRelatedFeature::Unsettled => settled = false,
                EndRelatedFeature::Absent => {}
            }
        }
        if !settled {
            continue;
        }
        if let Some(Some(context)) = context_type(types, &related) {
            implied.push(ImpliedRelationship {
                kind: ReferenceKind::TypeFeaturing,
                source: *connector,
                target: context,
            });
        }
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied.into_boxed_slice())
}

/// `ConnectorUtil.getContextTypeFor`: `Some(None)` when there is no common featuring type, `None`
/// when compatibility could not be decided.
fn context_type(types: &TypeIndex, related: &[DeclarationId]) -> Option<Option<DeclarationId>> {
    // A related feature that features another related feature is the context.
    for (position, candidate) in related.iter().enumerate() {
        if related.iter().enumerate().any(|(other, feature)| {
            other != position
                && feature != candidate
                && types
                    .featuring_types(*feature)
                    .iter()
                    .any(|(featuring, _)| featuring == candidate)
        }) {
            return Some(Some(*candidate));
        }
    }
    let mut common: Option<Vec<DeclarationId>> = None;
    for feature in related {
        let featuring = all_featuring_types(types, *feature);
        let Some(current) = common.as_mut() else {
            common = Some(featuring);
            continue;
        };
        let mut index = 0;
        while index < current.len() {
            let general = current[index];
            let mut subtype = None;
            for candidate in &featuring {
                if is_compatible(types, *candidate, general)? {
                    subtype = Some(*candidate);
                    break;
                }
            }
            if let Some(subtype) = subtype {
                current[index] = subtype;
            } else {
                let mut conforms = false;
                for candidate in &featuring {
                    if is_compatible(types, general, *candidate)? {
                        conforms = true;
                        break;
                    }
                }
                if !conforms {
                    current.remove(index);
                    continue;
                }
            }
            index += 1;
        }
    }
    Some(common.and_then(|common| common.first().copied()))
}

/// `FeatureUtil.getAllFeaturingTypesOf`: the breadth-first transitive featuring types of a
/// feature, innermost first.
fn all_featuring_types(types: &TypeIndex, feature: DeclarationId) -> Vec<DeclarationId> {
    let mut all = Vec::new();
    let mut frontier = vec![feature];
    while !frontier.is_empty() {
        let mut next = Vec::new();
        for current in frontier {
            for (featuring, _) in types.featuring_types(current) {
                if !all.contains(featuring) {
                    all.push(*featuring);
                    next.push(*featuring);
                }
            }
        }
        frontier = next;
    }
    all
}

/// `TypeUtil.isCompatible(subtype, supertype)` by reflexive specialization.
///
/// The Pilot additionally treats two Features with no owned features that share a redefined
/// feature as compatible when featured within each other. That branch is not derived here, so
/// when its precondition could apply (both Features, neither specializing the other) the answer
/// is `None` and the connector receives no context featuring type rather than a possibly
/// different innermost one.
fn is_compatible(
    types: &TypeIndex,
    subtype: DeclarationId,
    supertype: DeclarationId,
) -> Option<bool> {
    if subtype == supertype
        || types
            .specialization()
            .reaches(subtype, supertype, ScopeBits::AnySpecialization)
    {
        return Some(true);
    }
    if shares_redefinition(types, subtype, supertype) {
        return None;
    }
    Some(false)
}

/// Whether both declarations reach a common feature through Redefinition, the precondition of
/// the Pilot's redefinition-compatibility branch.
fn shares_redefinition(types: &TypeIndex, left: DeclarationId, right: DeclarationId) -> bool {
    let redefined = |declaration: DeclarationId| {
        let mut values = types
            .specialization()
            .entries(declaration)
            .iter()
            .filter(|(_, scopes)| scopes & ScopeBits::Redefinition.bit() != 0)
            .map(|(ancestor, _)| *ancestor)
            .collect::<Vec<_>>();
        values.push(declaration);
        values
    };
    let left = redefined(left);
    redefined(right).iter().any(|value| left.contains(value))
}
