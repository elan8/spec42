//! KerML `Type::endFeature` and the positional end redefinitions it implies.
//!
//! `checkFeatureEndRedefinition` (KerML 8.3.3.3.4) pairs the owned end at position `i` of a Type
//! with the `endFeature` at position `i` of each of that Type's direct supertypes. Both sides of the
//! pairing are owned facts: positions come from the canonical owned end collection
//! ([`crate::lower::storage::SemanticModelStorage::owned_end_features`], where a bare connector end
//! occupies its position even though it mints no declaration), and a supertype's `endFeature` is
//! derived here, once, over whatever direct specialization edges the caller has settled.
//!
//! The same derivation serves the published `TypeIndex` and the resolver phases that synthesize the
//! implied redefinitions, so the endFeature a pairing reads and the endFeature a query reports can
//! never be computed two different ways.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::lower::facts::OwnedEndFeature;
use crate::lower::facts::OwnedEndRecord;
use crate::lower::storage::SemanticModelStorage;
use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
use crate::model::ReferenceKind;
use crate::resolve::results::ImpliedRelationship;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionResults;
use crate::resolve::results::ResolutionStatus;

/// What one connector end contributes to its connector's `relatedFeature` collection.
///
/// KerML derives `relatedFeature` from each end's owned ReferenceSubsetting
/// (`ConnectorUtil.getRelatedFeaturesOf` in the Pilot). An end with no ReferenceSubsetting
/// contributes nothing; an end whose ReferenceSubsetting is authored but did not settle to one
/// target does relate a feature, just not one this publication can name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EndRelatedFeature {
    /// The end's ReferenceSubsetting settled to this feature.
    Resolved(DeclarationId),
    /// The end authors a ReferenceSubsetting whose target is unresolved or ambiguous.
    Unsettled,
    /// The end authors no ReferenceSubsetting.
    Absent,
}

impl EndRelatedFeature {
    /// Whether the end contributes a member to `relatedFeature`, known or not.
    pub(crate) fn is_present(self) -> bool {
        !matches!(self, Self::Absent)
    }
}

/// The reference kinds that carry a connector end's ReferenceSubsetting: the `references` /
/// `::>` relationship of a declared end, and the connector-end kinds a bare or named KerML
/// connector end is lowered as.
fn is_reference_subsetting(kind: ReferenceKind) -> bool {
    matches!(
        kind,
        ReferenceKind::References
            | ReferenceKind::ConnectorEnd
            | ReferenceKind::BindSource
            | ReferenceKind::BindTarget
            | ReferenceKind::Succession
            | ReferenceKind::FlowSource
            | ReferenceKind::FlowTarget
            | ReferenceKind::MemberAccessOperand
    )
}

/// The single derivation of the `relatedFeature` member one connector end contributes.
///
/// `outgoing` yields a declared end's authored references in canonical (authored) order; the
/// published model supplies its index, and resolver phases a map built from the same storage.
pub(crate) fn end_related_feature<I>(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
    end: OwnedEndFeature,
    outgoing: impl Fn(DeclarationId) -> I,
) -> EndRelatedFeature
where
    I: IntoIterator<Item = AuthoredReferenceId>,
{
    let reference = match end {
        OwnedEndFeature::Bare(reference) | OwnedEndFeature::Flow { reference, .. } => {
            Some(reference)
        }
        OwnedEndFeature::Declared(declaration) => {
            outgoing(declaration).into_iter().find(|reference| {
                storage
                    .references
                    .get(reference.index())
                    .is_some_and(|reference| is_reference_subsetting(reference.kind))
            })
        }
    };
    match reference.map(|reference| resolution.outcome(reference)) {
        None => EndRelatedFeature::Absent,
        Some(Some(ResolutionStatus::Resolved(target))) => EndRelatedFeature::Resolved(target),
        Some(_) => EndRelatedFeature::Unsettled,
    }
}

/// The owned ends of `owner`, from an owned end collection sorted by owner.
pub(crate) fn owned_ends_of(owned: &[OwnedEndRecord], owner: DeclarationId) -> &[OwnedEndRecord] {
    let start = owned.partition_point(|record| record.owner < owner);
    let end = owned.partition_point(|record| record.owner <= owner);
    &owned[start..end]
}

/// Derives every declaration's KerML `Type::endFeature`, indexed by declaration: the shared
/// positional derivation ([`derive_positional_features`]) over the owned end collection.
pub(crate) fn derive_end_features<F, I>(
    count: usize,
    owned: &[OwnedEndRecord],
    generals: F,
) -> Result<Vec<Vec<OwnedEndFeature>>, ResolutionError>
where
    F: Fn(DeclarationId) -> I,
    I: DoubleEndedIterator<Item = DeclarationId>,
{
    derive_positional_features(
        count,
        |declaration| {
            owned_ends_of(owned, declaration)
                .iter()
                .map(|record| record.end)
                .collect()
        },
        generals,
    )
}

/// Derives, for every declaration, an ordered positional Feature collection of a Type -- KerML
/// `Type::endFeature`, or the directed parameters a Behavior or Step has -- indexed by
/// declaration.
///
/// A row is the declaration's owned members in authored order (`owned`), then, for each direct
/// supertype in the order `generals` yields them, that supertype's row past the owned count: the
/// owned members redefine the supertype's members at the same positions, so only the rest are
/// inherited. A member inherited along two paths is listed once. A supertype still being derived is
/// on a specialization cycle with the declaration and contributes nothing, so the derivation is
/// one deterministic pass rather than a fixed point.
pub(crate) fn derive_positional_features<T, O, F, I>(
    count: usize,
    owned: O,
    generals: F,
) -> Result<Vec<Vec<T>>, ResolutionError>
where
    T: Copy + PartialEq,
    O: Fn(DeclarationId) -> Vec<T>,
    F: Fn(DeclarationId) -> I,
    I: DoubleEndedIterator<Item = DeclarationId>,
{
    const UNVISITED: u8 = 0;
    const IN_PROGRESS: u8 = 1;
    const DONE: u8 = 2;
    let mut state = vec![UNVISITED; count];
    let mut rows: Vec<Vec<T>> = (0..count).map(|_| Vec::new()).collect();
    let mut stack = Vec::new();
    for root in 0..count {
        if state[root] != UNVISITED {
            continue;
        }
        stack.push((root, false));
        while let Some((node, expanded)) = stack.pop() {
            let declaration =
                DeclarationId::from_index(node).map_err(|_| ResolutionError::Capacity)?;
            if !expanded {
                if state[node] != UNVISITED {
                    continue;
                }
                state[node] = IN_PROGRESS;
                stack.push((node, true));
                for general in generals(declaration).rev() {
                    if state.get(general.index()) == Some(&UNVISITED) {
                        stack.push((general.index(), false));
                    }
                }
                continue;
            }
            let mut row = owned(declaration);
            let owned_count = row.len();
            for general in generals(declaration) {
                if state.get(general.index()) != Some(&DONE) {
                    continue;
                }
                let inherited = rows
                    .get(general.index())
                    .ok_or(ResolutionError::InvalidStorage)?;
                for member in inherited.iter().skip(owned_count) {
                    if !row.contains(member) {
                        row.push(*member);
                    }
                }
            }
            rows[node] = row;
            state[node] = DONE;
        }
    }
    Ok(rows)
}

/// One `checkFeatureEndRedefinition` obligation: the owned end at `position` of `owner` must
/// redefine `target`, the end at the same position of `general`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct PositionalEndObligation {
    pub(crate) owner: DeclarationId,
    pub(crate) position: usize,
    pub(crate) general: DeclarationId,
    pub(crate) source: OwnedEndFeature,
    pub(crate) target: OwnedEndFeature,
}

/// Every positional end obligation of `owner`, in position then supertype order.
///
/// `generals` yields `owner`'s direct supertypes; a Type that specializes itself pairs nothing,
/// since an end never redefines itself. `end_features` reads a supertype's `endFeature` row.
pub(crate) fn positional_end_obligations<G, E>(
    owned: &[OwnedEndRecord],
    owner: DeclarationId,
    generals: G,
    end_features: E,
) -> Vec<PositionalEndObligation>
where
    G: Fn(DeclarationId) -> Vec<DeclarationId>,
    E: Fn(DeclarationId) -> Vec<OwnedEndFeature>,
{
    let owner_generals = generals(owner);
    let mut obligations = Vec::new();
    for (position, record) in owned_ends_of(owned, owner).iter().enumerate() {
        for general in owner_generals.iter().copied() {
            if general == owner {
                continue;
            }
            if let Some(target) = end_features(general).get(position).copied() {
                if target != record.end {
                    obligations.push(PositionalEndObligation {
                        owner,
                        position,
                        general,
                        source: record.end,
                        target,
                    });
                }
            }
        }
    }
    obligations
}

/// Synthesizes the positional end Redefinitions implied by `checkFeatureEndRedefinition`.
///
/// `type_edges` are the settled direct specialization edges, `(specific, general)`: authored ones
/// and every implied one known at the calling phase, including implied library supertypes such
/// as `Links::BinaryLink`, whose `source`/`target` ends are thereby redefined. Matching the Pilot
/// (`FeatureAdapter.addRedefinitions`), the pairing is implied whether or not the end also authors
/// a redefinition; only a pairing an authored edge already states is omitted (`authored`, as
/// `(source, target)`), so it keeps its authored provenance. A pairing with a bare end on either
/// side has no declaration to relate and publishes nothing; the redefinition check answers it as
/// unresolved.
pub(crate) fn synthesize_positional_end_redefinitions(
    count: usize,
    owned: &[OwnedEndRecord],
    type_edges: impl IntoIterator<Item = (DeclarationId, DeclarationId)>,
    authored: &BTreeSet<(DeclarationId, DeclarationId)>,
) -> Result<Vec<ImpliedRelationship>, ResolutionError> {
    if owned.is_empty() {
        return Ok(Vec::new());
    }
    let mut generals = vec![BTreeSet::new(); count];
    for (specific, general) in type_edges {
        generals
            .get_mut(specific.index())
            .ok_or(ResolutionError::InvalidStorage)?
            .insert(general);
    }
    let generals = generals
        .into_iter()
        .map(|set| set.into_iter().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let generals_of = |declaration: DeclarationId| {
        generals
            .get(declaration.index())
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
            .copied()
    };
    let end_features = derive_end_features(count, owned, generals_of)?;
    let mut owners = owned.iter().map(|record| record.owner).collect::<Vec<_>>();
    owners.dedup();
    let mut implied = Vec::new();
    for owner in owners {
        for obligation in positional_end_obligations(
            owned,
            owner,
            |declaration| generals_of(declaration).collect(),
            |general| {
                end_features
                    .get(general.index())
                    .cloned()
                    .unwrap_or_default()
            },
        ) {
            let (OwnedEndFeature::Declared(source), OwnedEndFeature::Declared(target)) =
                (obligation.source, obligation.target)
            else {
                continue;
            };
            if !authored.contains(&(source, target)) {
                implied.push(ImpliedRelationship {
                    kind: ReferenceKind::Redefinition,
                    source,
                    target,
                });
            }
        }
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied)
}

/// KerML `Feature::crossFeature` of one Feature, as far as this publication states it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CrossFeature {
    /// The settled cross feature: the Feature's owned cross feature (`end crossing [1] feature
    /// e`), which the implied CrossSubsetting the Pilot adds (`addCrossingSpecialization`) makes
    /// its `crossFeature`, or the second chaining feature of its authored `crosses a.b`.
    Resolved(DeclarationId),
    /// The Feature authors a CrossSubsetting (`crosses a.b`) whose second chaining feature did
    /// not settle to a declaration.
    Unpublished,
    /// The Feature has no cross feature: no owned cross feature and no owned CrossSubsetting, or
    /// one whose crossed feature has fewer than two chaining features.
    Absent,
}

/// KerML 8.3.3.3.4 `Feature::crossFeature` of `feature`:
///
/// ```text
/// crossFeature = if ownedCrossSubsetting = null then null else
///     let chainingFeatures = ownedCrossSubsetting.crossedFeature.chainingFeature in
///     if chainingFeatures->size() < 2 then null else chainingFeatures->at(2)
/// ```
///
/// An owned cross feature's CrossSubsetting is implied, not authored, so its projection fact
/// answers directly. Otherwise the first authored `crosses` is the owned CrossSubsetting (the
/// Pilot's `getOwnedCrossSubsetting`; a second one is `validateFeatureOwnedCrossSubsetting`'s
/// concern) and its chaining features are the canonical per-hop outcomes `member_access_paths`
/// captured at the last segment of each `.`-separated chaining feature of its dotted reference.
pub(crate) fn cross_feature_of(
    storage: &SemanticModelStorage,
    member_access_paths: &BTreeMap<AuthoredReferenceId, Box<[ResolutionStatus]>>,
    feature: DeclarationId,
) -> Result<CrossFeature, ResolutionError> {
    let facts = storage
        .declaration_facts(feature)
        .ok_or(ResolutionError::InvalidStorage)?;
    if let Some(projection) = facts.cross_feature_projection {
        return Ok(CrossFeature::Resolved(projection.cross_feature));
    }
    let Some((index, reference)) = storage
        .references
        .iter()
        .enumerate()
        .find(|(_, reference)| {
            reference.source == feature
                && reference.kind == ReferenceKind::Crosses
                && !reference.flags.implied
        })
    else {
        return Ok(CrossFeature::Absent);
    };
    if !reference.flags.dotted || reference.chaining_feature_ends.len() < 2 {
        return Ok(CrossFeature::Absent);
    }
    let id = AuthoredReferenceId(u32::try_from(index).map_err(|_| ResolutionError::Capacity)?);
    let second = member_access_paths.get(&id).and_then(|path| {
        reference
            .chaining_feature_ends
            .get(1)
            .and_then(|end| path.get(*end as usize).copied())
    });
    Ok(match second {
        Some(ResolutionStatus::Resolved(cross)) => CrossFeature::Resolved(cross),
        _ => CrossFeature::Unpublished,
    })
}

/// One owned cross feature together with the end Feature that owns it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct OwnedCrossFeature {
    pub(crate) end: DeclarationId,
    pub(crate) cross_feature: DeclarationId,
}

/// Every owned cross feature of the publication, in end identity order.
pub(crate) fn owned_cross_features(
    storage: &SemanticModelStorage,
) -> Result<Vec<OwnedCrossFeature>, ResolutionError> {
    let mut owned = Vec::new();
    for (index, facts) in storage.declaration_facts.iter().enumerate() {
        if let Some(projection) = facts.cross_feature_projection {
            owned.push(OwnedCrossFeature {
                end: DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?,
                cross_feature: projection.owned_cross_feature,
            });
        }
    }
    Ok(owned)
}

/// Synthesizes the Subsettings implied by `checkFeatureOwnedCrossFeatureRedefinitionSpecialization`
/// (KerML 8.3.3.3.4): an owned cross feature subsets the `crossFeature` of every Feature its end
/// redefines, as the Pilot's `addOwnedCrossFeatureSpecialization` adds them.
///
/// `redefinitions` are the end's settled `(source, target)` Redefinitions, authored and implied
/// (including the positional end redefinitions). A redefined Feature whose cross feature is not a
/// settled fact contributes nothing; the redefinition check answers it as unresolved. A Subsetting
/// an authored relationship already states (`authored_subsettings`) is not restated.
pub(crate) fn synthesize_owned_cross_feature_redefinition_subsettings(
    storage: &SemanticModelStorage,
    member_access_paths: &BTreeMap<AuthoredReferenceId, Box<[ResolutionStatus]>>,
    redefinitions: &BTreeSet<(DeclarationId, DeclarationId)>,
    authored_subsettings: &BTreeSet<(DeclarationId, DeclarationId)>,
) -> Result<Vec<ImpliedRelationship>, ResolutionError> {
    let mut implied = Vec::new();
    for owned in owned_cross_features(storage)? {
        let redefined_by_end = redefinitions
            .range((owned.end, DeclarationId(0))..=(owned.end, DeclarationId(u32::MAX)));
        for (_, redefined) in redefined_by_end {
            let CrossFeature::Resolved(cross) =
                cross_feature_of(storage, member_access_paths, *redefined)?
            else {
                continue;
            };
            if cross != owned.cross_feature
                && !authored_subsettings.contains(&(owned.cross_feature, cross))
            {
                implied.push(ImpliedRelationship {
                    kind: ReferenceKind::Subsetting,
                    source: owned.cross_feature,
                    target: cross,
                });
            }
        }
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::AuthoredReferenceId;

    fn id(index: usize) -> DeclarationId {
        DeclarationId::from_index(index).unwrap()
    }

    fn declared(owner: usize, end: usize) -> OwnedEndRecord {
        OwnedEndRecord {
            owner: id(owner),
            end: OwnedEndFeature::Declared(id(end)),
        }
    }

    fn pairs(implied: &[ImpliedRelationship]) -> Vec<(usize, usize)> {
        implied
            .iter()
            .map(|relationship| {
                assert_eq!(relationship.kind, ReferenceKind::Redefinition);
                (relationship.source.index(), relationship.target.index())
            })
            .collect()
    }

    #[test]
    fn a_bare_end_occupies_its_position() {
        // General 0 owns ends 1 and 2; specific 3 owns a bare end, then declared end 4.
        let owned = [
            declared(0, 1),
            declared(0, 2),
            OwnedEndRecord {
                owner: id(3),
                end: OwnedEndFeature::Bare(AuthoredReferenceId::from_index(0).unwrap()),
            },
            declared(3, 4),
        ];
        let implied =
            synthesize_positional_end_redefinitions(5, &owned, [(id(3), id(0))], &BTreeSet::new())
                .unwrap();
        assert_eq!(pairs(&implied), [(4, 2)]);
    }

    #[test]
    fn a_position_pairs_with_an_inherited_end_of_the_general() {
        // 0 owns ends 1, 2; 3 specializes 0 and owns end 4; 5 specializes 3 and owns ends 6, 7.
        let owned = [
            declared(0, 1),
            declared(0, 2),
            declared(3, 4),
            declared(5, 6),
            declared(5, 7),
        ];
        let implied = synthesize_positional_end_redefinitions(
            8,
            &owned,
            [(id(3), id(0)), (id(5), id(3))],
            &BTreeSet::new(),
        )
        .unwrap();
        assert_eq!(pairs(&implied), [(4, 1), (6, 4), (7, 2)]);
    }

    #[test]
    fn an_authored_pairing_is_not_restated_but_other_authored_targets_do_not_suppress() {
        // 0 owns ends 1, 2; 3 specializes 0 and owns ends 4, 5. End 4 authors `:>> 1`, end 5
        // authors `:>> 1` too: only 4's pairing is already stated.
        let owned = [
            declared(0, 1),
            declared(0, 2),
            declared(3, 4),
            declared(3, 5),
        ];
        let authored = BTreeSet::from([(id(4), id(1)), (id(5), id(1))]);
        let implied =
            synthesize_positional_end_redefinitions(6, &owned, [(id(3), id(0))], &authored)
                .unwrap();
        assert_eq!(pairs(&implied), [(5, 2)]);
    }

    #[test]
    fn a_specialization_cycle_terminates_with_the_owned_ends() {
        let owned = [declared(0, 1), declared(2, 3)];
        let ends = derive_end_features(4, &owned, |declaration| {
            match declaration.index() {
                0 => vec![id(2)],
                2 => vec![id(0)],
                _ => vec![],
            }
            .into_iter()
        })
        .unwrap();
        assert_eq!(ends[0], [OwnedEndFeature::Declared(id(1))]);
        assert_eq!(ends[2], [OwnedEndFeature::Declared(id(3))]);
    }
}
