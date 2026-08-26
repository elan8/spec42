//! Canonical effective Feature typing derived from settled relationship facts.

use crate::lower::storage::SemanticModelStorage;
use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
use crate::model::ReferenceKind;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionResults;
use crate::resolve::results::ResolutionStatus;

/// Where one of a Feature's effective types came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum EffectiveTypeSource {
    Direct,
    Inherited(DeclarationId),
}

type EffectiveTypeRow = Box<[(DeclarationId, EffectiveTypeSource)]>;

/// The canonical effective-type collection for every declaration in one settled resolution.
pub(crate) struct EffectiveTypes {
    rows: Box<[EffectiveTypeRow]>,
}

impl EffectiveTypes {
    pub(crate) fn row(
        &self,
        declaration: DeclarationId,
    ) -> &[(DeclarationId, EffectiveTypeSource)] {
        self.rows
            .get(declaration.index())
            .map(Box::as_ref)
            .unwrap_or_default()
    }
}

/// Derives effective typing once from direct FeatureTyping and the transitive
/// Subsetting/Redefinition closure.
///
/// Each pass reads a complete prior value and publishes a fresh next value. A declaration count
/// bound is sufficient because every successful pass adds at least one of the finite
/// `(declaration, type, origin)` facts; cycles therefore converge without scheduling dependence.
pub(crate) fn derive_effective_types(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
) -> Result<EffectiveTypes, ResolutionError> {
    let count = storage.declarations.len();
    let mut direct = vec![std::collections::BTreeSet::new(); count];
    let mut generals = vec![std::collections::BTreeSet::new(); count];
    let mut edge = |source: DeclarationId, target: DeclarationId, kind: ReferenceKind| {
        let Some(direct_row) = direct.get_mut(source.index()) else {
            return Err(ResolutionError::InvalidStorage);
        };
        let Some(general_row) = generals.get_mut(source.index()) else {
            return Err(ResolutionError::InvalidStorage);
        };
        match kind {
            ReferenceKind::FeatureTyping => {
                direct_row.insert(target);
            }
            ReferenceKind::Subsetting | ReferenceKind::Redefinition => {
                general_row.insert(target);
            }
            _ => {}
        }
        Ok(())
    };
    for (index, reference) in storage.references.iter().enumerate() {
        let id = AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        if let Some(ResolutionStatus::Resolved(target)) = resolution.outcome(id) {
            edge(reference.source, target, reference.kind)?;
        }
    }
    for relationship in resolution.implied_relationships.iter() {
        edge(relationship.source, relationship.target, relationship.kind)?;
    }

    let mut rows = direct
        .iter()
        .map(|targets| {
            targets
                .iter()
                .map(|target| (*target, EffectiveTypeSource::Direct))
                .collect::<std::collections::BTreeSet<_>>()
        })
        .collect::<Vec<_>>();
    for _ in 0..count {
        let mut next = rows.clone();
        for (source, source_generals) in generals.iter().enumerate() {
            for general in source_generals {
                let Some(inherited) = rows.get(general.index()) else {
                    return Err(ResolutionError::InvalidStorage);
                };
                let Some(next_row) = next.get_mut(source) else {
                    return Err(ResolutionError::InvalidStorage);
                };
                next_row.extend(inherited.iter().map(|(target, origin)| {
                    (
                        *target,
                        EffectiveTypeSource::Inherited(match origin {
                            EffectiveTypeSource::Direct => *general,
                            EffectiveTypeSource::Inherited(from) => *from,
                        }),
                    )
                }));
            }
        }
        if next == rows {
            break;
        }
        rows = next;
    }
    Ok(EffectiveTypes {
        rows: rows
            .into_iter()
            .map(|row| row.into_iter().collect::<Vec<_>>().into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    })
}
