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

pub(crate) fn derive_effective_types_from_edges(
    count: usize,
    edges: impl IntoIterator<Item = (DeclarationId, DeclarationId, ReferenceKind)>,
) -> Result<EffectiveTypes, ResolutionError> {
    let mut direct = vec![std::collections::BTreeSet::new(); count];
    let mut generals = vec![std::collections::BTreeSet::new(); count];
    for (source, target, kind) in edges {
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
    let mut dependants = vec![Vec::new(); count];
    for (source, source_generals) in generals.iter().enumerate() {
        let source = DeclarationId::from_index(source).map_err(|_| ResolutionError::Capacity)?;
        for general in source_generals {
            let Some(row) = dependants.get_mut(general.index()) else {
                return Err(ResolutionError::InvalidStorage);
            };
            row.push(source);
        }
    }
    for row in &mut dependants {
        row.sort_unstable();
        row.dedup();
    }

    // Propagate only newly discovered facts along the reverse Subsetting/Redefinition graph.
    // Each finite `(declaration, type, provenance)` fact enters the queue once, so cycles converge
    // without cloning every declaration's complete row on every depth pass.
    let mut queue = std::collections::VecDeque::new();
    for (source, row) in rows.iter().enumerate() {
        let source = DeclarationId::from_index(source).map_err(|_| ResolutionError::Capacity)?;
        queue.extend(row.iter().map(|fact| (source, *fact)));
    }
    while let Some((general, (target, origin))) = queue.pop_front() {
        let inherited = (
            target,
            EffectiveTypeSource::Inherited(match origin {
                EffectiveTypeSource::Direct => general,
                EffectiveTypeSource::Inherited(from) => from,
            }),
        );
        let Some(general_dependants) = dependants.get(general.index()) else {
            return Err(ResolutionError::InvalidStorage);
        };
        for source in general_dependants {
            let Some(row) = rows.get_mut(source.index()) else {
                return Err(ResolutionError::InvalidStorage);
            };
            if row.insert(inherited) {
                queue.push_back((*source, inherited));
            }
        }
    }
    Ok(EffectiveTypes {
        rows: rows
            .into_iter()
            .map(|row| row.into_iter().collect::<Vec<_>>().into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    })
}

/// Derives effective typing once from direct FeatureTyping and the transitive
/// Subsetting/Redefinition closure.
///
/// The shared worklist derivation is also used while those relationships settle, so published and
/// intermediate member scopes cannot disagree about effective typing.
pub(crate) fn derive_effective_types(
    storage: &SemanticModelStorage,
    resolution: &ResolutionResults,
) -> Result<EffectiveTypes, ResolutionError> {
    let mut edges = Vec::new();
    for (index, reference) in storage.references.iter().enumerate() {
        let id = AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        if let Some(ResolutionStatus::Resolved(target)) = resolution.outcome(id) {
            edges.push((reference.source, target, reference.kind));
        }
    }
    for relationship in resolution.implied_relationships.iter() {
        edges.push((relationship.source, relationship.target, relationship.kind));
    }
    derive_effective_types_from_edges(storage.declarations.len(), edges)
}
