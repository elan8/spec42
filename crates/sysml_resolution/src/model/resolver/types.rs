//! Published type facts: the specialization closure this publication settled.
//!
//! KerML makes `Subclassification`, `Subsetting`, `Redefinition` and `FeatureTyping` all subkinds
//! of `Specialization`, and the OMG Pilot's `Type::supertypes` is exactly `ownedSpecialization
//! .general`, so one closure spans all four. The sibling compiler instead walks subclassification
//! alone and answers feature-level conformance through separate rules; both readings are served
//! here from one index by tagging each entry with the scopes whose paths reach it, rather than by
//! building a second closure store or making a consumer re-traverse.
//!
//! The closure is eager. It is a barrier product computed once from settled outcomes, so a query
//! is a lookup rather than a traversal, an answer cannot depend on the order queries arrive in,
//! and the publication keeps its "the only interior mutation is source line indexing" invariant.

use super::*;

/// Which specialization edges a path may use.
///
/// A closed set: each variant is a published query contract, not a caller-assembled edge filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpecializationScope {
    /// Every `Specialization` subkind, as the Pilot's `Type::supertypes` does.
    AnySpecialization,
    /// `Subclassification` alone: generalization between classifiers, ignoring the feature-level
    /// subkinds and typing.
    Subclassification,
}

impl SpecializationScope {
    /// Every published scope, widest first, so rendered output has one fixed order.
    pub(crate) const ALL: [Self; 2] = [Self::AnySpecialization, Self::Subclassification];

    fn bit(self) -> u8 {
        match self {
            Self::AnySpecialization => 1 << 0,
            Self::Subclassification => 1 << 1,
        }
    }
}

/// The scopes one edge participates in.
///
/// Every specialization edge is in `AnySpecialization`; only a `Subclassification` edge is also in
/// the narrower scope. A path's scopes are the intersection of its edges' scopes, which is what
/// makes one tagged closure answer both readings: a path through a `Subsetting` edge stops being
/// a subclassification path at that edge and never regains it.
fn edge_scopes(kind: ReferenceKind) -> Option<u8> {
    match kind {
        ReferenceKind::Subclassification => Some(
            SpecializationScope::AnySpecialization.bit()
                | SpecializationScope::Subclassification.bit(),
        ),
        ReferenceKind::Subsetting | ReferenceKind::Redefinition | ReferenceKind::FeatureTyping => {
            Some(SpecializationScope::AnySpecialization.bit())
        }
        _ => None,
    }
}

/// Per-declaration transitive specialization ancestors.
///
/// Strict: a declaration is never its own ancestor here. Reflexivity belongs to the query contract
/// (the Pilot's `allSupertypes` is `OrderedSet{self}->closure(...)`), and keeping it out of storage
/// is what lets a cyclic declaration be reported as cyclic instead of as conforming to itself
/// through its cycle.
#[derive(Debug, Default)]
pub(crate) struct SpecializationClosure {
    /// Contiguous range into `ancestors` per declaration, indexed by declaration ordinal.
    ranges: Box<[(u32, u32)]>,
    /// `(ancestor, scopes)` sorted by ancestor within each declaration's range, so a conformance
    /// question is a binary search rather than a scan.
    ancestors: Box<[(DeclarationId, u8)]>,
    /// Declarations that reach themselves through specialization. Their ancestor sets are still
    /// published; the flag is what lets a query report the cycle rather than answer from it.
    cyclic: Box<[bool]>,
}

impl SpecializationClosure {
    pub(crate) fn build(
        storage: &SemanticModelStorage,
        resolution: &ResolutionResults,
    ) -> Result<Self, ResolutionError> {
        let count = storage.declarations.len();
        let mut direct: Vec<BTreeMap<DeclarationId, u8>> = vec![BTreeMap::new(); count];
        let mut record = |source: DeclarationId,
                          target: DeclarationId,
                          scopes: u8|
         -> Result<(), ResolutionError> {
            let slot = direct
                .get_mut(source.index())
                .ok_or(ResolutionError::InvalidStorage)?;
            *slot.entry(target).or_insert(0) |= scopes;
            Ok(())
        };

        for (index, reference) in storage.references.iter().enumerate() {
            let Some(scopes) = edge_scopes(reference.kind) else {
                continue;
            };
            let id =
                AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            // Only a settled single target contributes an edge. An ambiguous outcome deliberately
            // contributes none: picking one of its candidates here would publish a conformance
            // answer the resolver refused to publish as a relationship.
            if let Some(ResolutionStatus::Resolved(target)) = resolution.outcome(id) {
                record(reference.source, target, scopes)?;
            }
        }
        // Implied edges count too: the Pilot's `supertypes(excludeImplied = false)` is what
        // `allSupertypes` is defined over.
        for relationship in resolution.implied_relationships.iter() {
            let Some(scopes) = edge_scopes(relationship.kind) else {
                continue;
            };
            record(relationship.source, relationship.target, scopes)?;
        }

        let closure = saturate(&direct, count)?;

        let mut ranges = Vec::with_capacity(count);
        let mut ancestors = Vec::new();
        let mut cyclic = vec![false; count];
        for (index, entry) in closure.into_iter().enumerate() {
            let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            let start = u32::try_from(ancestors.len()).map_err(|_| ResolutionError::Capacity)?;
            for (ancestor, scopes) in entry {
                if ancestor == id {
                    cyclic[index] = true;
                    continue;
                }
                ancestors.push((ancestor, scopes));
            }
            let end = u32::try_from(ancestors.len()).map_err(|_| ResolutionError::Capacity)?;
            ranges.push((start, end));
        }

        Ok(Self {
            ranges: ranges.into_boxed_slice(),
            ancestors: ancestors.into_boxed_slice(),
            cyclic: cyclic.into_boxed_slice(),
        })
    }

    /// Every strict ancestor of `declaration` with the scopes that reach it.
    pub(crate) fn scoped_ancestors(
        &self,
        declaration: DeclarationId,
    ) -> impl Iterator<Item = (DeclarationId, Vec<SpecializationScope>)> + '_ {
        self.entries(declaration).iter().map(|(ancestor, scopes)| {
            (
                *ancestor,
                SpecializationScope::ALL
                    .into_iter()
                    .filter(|scope| scopes & scope.bit() != 0)
                    .collect(),
            )
        })
    }

    /// Whether `declaration` reaches itself through specialization.
    pub(crate) fn is_cyclic(&self, declaration: DeclarationId) -> bool {
        self.cyclic
            .get(declaration.index())
            .copied()
            .unwrap_or(false)
    }

    fn entries(&self, declaration: DeclarationId) -> &[(DeclarationId, u8)] {
        let Some(&(start, end)) = self.ranges.get(declaration.index()) else {
            return &[];
        };
        self.ancestors
            .get(start as usize..end as usize)
            .unwrap_or_default()
    }
}

/// Saturates the direct edges into their transitive closure.
///
/// A bounded fixed point over the immutable previous-pass state, mirroring the ancestor closure
/// the resolver already builds for inherited names: each pass reads the previous complete closure
/// and writes a separate next buffer, swapped only at the pass barrier, so no pass observes its
/// own partial results. Each pass only ever adds pairs, and the pair set is bounded by
/// `declarations^2`, so the loop reaches a fixed point well inside `declarations + 1` passes even
/// when the input contains a specialization cycle.
///
/// A path's scopes are the intersection of its edges' scopes, so a pair already present under a
/// wider scope can still gain a narrower one on a later pass; the loop therefore continues while
/// scope bits change, not only while pairs appear.
fn saturate(
    direct: &[BTreeMap<DeclarationId, u8>],
    count: usize,
) -> Result<Vec<BTreeMap<DeclarationId, u8>>, ResolutionError> {
    let mut closure = direct.to_vec();
    let mut next = closure.clone();
    let pass_limit = count.checked_add(1).ok_or(ResolutionError::Capacity)?;
    for _ in 0..pass_limit {
        let mut changed = false;
        for (index, parents) in direct.iter().enumerate() {
            for (parent, parent_scopes) in parents {
                for (ancestor, ancestor_scopes) in
                    std::iter::once((parent, parent_scopes)).chain(closure[parent.index()].iter())
                {
                    let scopes = parent_scopes & ancestor_scopes;
                    if scopes == 0 {
                        continue;
                    }
                    let slot = next[index].entry(*ancestor).or_insert(0);
                    if *slot | scopes != *slot {
                        *slot |= scopes;
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
        closure.clone_from(&next);
    }
    Ok(next)
}
