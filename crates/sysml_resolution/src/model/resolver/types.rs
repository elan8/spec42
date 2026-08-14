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
    /// `Subsetting` and `Redefinition`: how one feature specializes another, ignoring typing.
    ///
    /// This is the chain an untyped feature inherits its type along, so it is the scope effective
    /// typing walks. Following a `FeatureTyping` edge instead would cross from the feature to its
    /// type and start collecting the type's own supertypes as if they were the feature's types.
    FeatureSpecialization,
}

impl SpecializationScope {
    /// Every published scope, widest first, so rendered output has one fixed order.
    pub(crate) const ALL: [Self; 3] = [
        Self::AnySpecialization,
        Self::Subclassification,
        Self::FeatureSpecialization,
    ];

    fn bit(self) -> u8 {
        match self {
            Self::AnySpecialization => 1 << 0,
            Self::Subclassification => 1 << 1,
            Self::FeatureSpecialization => 1 << 2,
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
        ReferenceKind::Subsetting | ReferenceKind::Redefinition => Some(
            SpecializationScope::AnySpecialization.bit()
                | SpecializationScope::FeatureSpecialization.bit(),
        ),
        ReferenceKind::FeatureTyping => Some(SpecializationScope::AnySpecialization.bit()),
        _ => None,
    }
}

/// Whether a fact was authored or synthesized by the resolver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum FactProvenance {
    Authored,
    Implied,
}

/// Where one of a feature's effective types came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum EffectiveTypeSource {
    /// The feature carries this typing itself.
    Direct,
    /// The feature inherits this typing from a feature it subsets or redefines.
    Inherited(DeclarationId),
}

/// Contiguous per-declaration rows over one shared entry table.
///
/// The same shape the element-fact index uses: a per-declaration question reads one slice instead
/// of scanning a whole table.
#[derive(Debug)]
struct Rows<T> {
    ranges: Box<[(u32, u32)]>,
    entries: Box<[T]>,
}

impl<T> Default for Rows<T> {
    fn default() -> Self {
        Self {
            ranges: Box::default(),
            entries: Box::default(),
        }
    }
}

impl<T: Ord> Rows<T> {
    /// Builds rows from `(owner, entry)` pairs. Entries are sorted and deduplicated within each
    /// row, so output never depends on the order facts were discovered in.
    fn build(count: usize, mut pairs: Vec<(DeclarationId, T)>) -> Result<Self, ResolutionError> {
        pairs.sort();
        pairs.dedup();
        let mut ranges = Vec::with_capacity(count);
        let mut entries = Vec::with_capacity(pairs.len());
        let mut remaining = pairs.into_iter().peekable();
        for index in 0..count {
            let start = u32::try_from(entries.len()).map_err(|_| ResolutionError::Capacity)?;
            while remaining
                .peek()
                .is_some_and(|(owner, _)| owner.index() == index)
            {
                let (_, entry) = remaining.next().ok_or(ResolutionError::InvalidStorage)?;
                entries.push(entry);
            }
            let end = u32::try_from(entries.len()).map_err(|_| ResolutionError::Capacity)?;
            ranges.push((start, end));
        }
        // A pair whose owner is outside the declaration domain would otherwise be dropped without
        // a trace, leaving a fact that exists but can never be read.
        if remaining.next().is_some() {
            return Err(ResolutionError::InvalidStorage);
        }
        Ok(Self {
            ranges: ranges.into_boxed_slice(),
            entries: entries.into_boxed_slice(),
        })
    }

    fn row(&self, declaration: DeclarationId) -> &[T] {
        let Some(&(start, end)) = self.ranges.get(declaration.index()) else {
            return &[];
        };
        self.entries
            .get(start as usize..end as usize)
            .unwrap_or_default()
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
        self.entries(declaration)
            .iter()
            .map(|(ancestor, scopes)| (*ancestor, scopes_of(*scopes).collect()))
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

/// Every published type fact of one publication.
#[derive(Debug, Default)]
pub(crate) struct TypeIndex {
    specialization: SpecializationClosure,
    /// Resolved `FeatureTyping` targets per feature, with provenance.
    direct_types: Rows<(DeclarationId, FactProvenance)>,
    /// Direct specializers per declaration: the reverse of the direct specialization edges, tagged
    /// with the scopes each edge belongs to.
    ///
    /// Reverse rather than derived on demand: without it, "what specializes this?" is a scan of
    /// every reference in the model, which is the shape of query a publication is supposed to
    /// index away.
    subtypes: Rows<(DeclarationId, u8)>,
    /// The types a feature has, directly or inherited along its subsetting/redefinition chain.
    effective_types: Rows<(DeclarationId, EffectiveTypeSource)>,
}

impl TypeIndex {
    pub(crate) fn build(
        storage: &SemanticModelStorage,
        resolution: &ResolutionResults,
    ) -> Result<Self, ResolutionError> {
        let count = storage.declarations.len();
        let specialization = SpecializationClosure::build(storage, resolution)?;

        let mut direct_types = Vec::new();
        let mut subtypes = Vec::new();
        let mut edge = |source: DeclarationId,
                        target: DeclarationId,
                        kind: ReferenceKind,
                        provenance: FactProvenance| {
            if let Some(scopes) = edge_scopes(kind) {
                subtypes.push((target, (source, scopes)));
            }
            if kind == ReferenceKind::FeatureTyping {
                direct_types.push((source, (target, provenance)));
            }
        };

        for (index, reference) in storage.references.iter().enumerate() {
            let id =
                AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            if let Some(ResolutionStatus::Resolved(target)) = resolution.outcome(id) {
                edge(
                    reference.source,
                    target,
                    reference.kind,
                    FactProvenance::Authored,
                );
            }
        }
        for relationship in resolution.implied_relationships.iter() {
            edge(
                relationship.source,
                relationship.target,
                relationship.kind,
                FactProvenance::Implied,
            );
        }

        let direct_types = Rows::build(count, direct_types)?;
        let subtypes = Rows::build(count, subtypes)?;

        // Effective typing walks the feature-specialization chain and collects what each feature
        // on it is typed by. A feature that declares no typing of its own is not untyped: KerML
        // gives it the typing of the feature it subsets or redefines, which is the rule the legacy
        // conformance check depended on and could only express as "an untyped feature always
        // conforms".
        let mut effective = Vec::new();
        for index in 0..count {
            let declaration =
                DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            for (target, _) in direct_types.row(declaration) {
                effective.push((declaration, (*target, EffectiveTypeSource::Direct)));
            }
            for (ancestor, scopes) in specialization.entries(declaration) {
                if scopes & SpecializationScope::FeatureSpecialization.bit() == 0 {
                    continue;
                }
                for (target, _) in direct_types.row(*ancestor) {
                    effective.push((
                        declaration,
                        (*target, EffectiveTypeSource::Inherited(*ancestor)),
                    ));
                }
            }
        }
        let effective_types = Rows::build(count, effective)?;

        Ok(Self {
            specialization,
            direct_types,
            subtypes,
            effective_types,
        })
    }

    pub(crate) fn specialization(&self) -> &SpecializationClosure {
        &self.specialization
    }

    pub(crate) fn direct_types(
        &self,
        declaration: DeclarationId,
    ) -> &[(DeclarationId, FactProvenance)] {
        self.direct_types.row(declaration)
    }

    /// Declarations that directly specialize `declaration`, with the scopes of each edge.
    pub(crate) fn subtypes(&self, declaration: DeclarationId) -> &[(DeclarationId, u8)] {
        self.subtypes.row(declaration)
    }

    pub(crate) fn effective_types(
        &self,
        declaration: DeclarationId,
    ) -> &[(DeclarationId, EffectiveTypeSource)] {
        self.effective_types.row(declaration)
    }
}

/// The scopes a tagged edge or path belongs to, in the fixed published order.
pub(crate) fn scopes_of(bits: u8) -> impl Iterator<Item = SpecializationScope> {
    SpecializationScope::ALL
        .into_iter()
        .filter(move |scope| bits & scope.bit() != 0)
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
