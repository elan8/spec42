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
        let mut direct: Vec<Vec<(DeclarationId, u8)>> = vec![Vec::new(); count];
        let mut record = |source: DeclarationId,
                          target: DeclarationId,
                          scopes: u8|
         -> Result<(), ResolutionError> {
            direct
                .get_mut(source.index())
                .ok_or(ResolutionError::InvalidStorage)?
                .push((target, scopes));
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

        for row in &mut direct {
            merge_scopes(row);
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

    /// Whether `specific` reaches `general` through specialization within `scope`.
    ///
    /// Strict: the transitive relation only. Reflexivity is applied by the query service, which
    /// owns the published conformance contract.
    pub(crate) fn reaches(
        &self,
        specific: DeclarationId,
        general: DeclarationId,
        scope: SpecializationScope,
    ) -> bool {
        let entries = self.entries(specific);
        entries
            .binary_search_by_key(&general, |(ancestor, _)| *ancestor)
            .is_ok_and(|found| entries[found].1 & scope.bit() != 0)
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
    /// Direct supertypes per declaration: the specialization edges it declares, tagged with the
    /// scopes each edge belongs to. The transitive answer lives in the closure; this is the one
    /// hop a type hierarchy view expands at a time.
    supertypes: Rows<(DeclarationId, u8)>,
    /// Direct specializers per declaration: the reverse of the direct specialization edges, tagged
    /// with the scopes each edge belongs to.
    ///
    /// Reverse rather than derived on demand: without it, "what specializes this?" is a scan of
    /// every reference in the model, which is the shape of query a publication is supposed to
    /// index away.
    subtypes: Rows<(DeclarationId, u8)>,
    /// The types a feature has, directly or inherited along its subsetting/redefinition chain.
    effective_types: Rows<(DeclarationId, EffectiveTypeSource)>,
    /// Effective featuring types, derived solely from settled TypeFeaturing and FeatureChaining
    /// relationships. This replaces the former ownership-only shortcut so authored and implied
    /// facts have one canonical representation.
    featuring: Rows<(DeclarationId, FactProvenance)>,
    /// A `var` FeatureMembership requires the owner's canonical `snapshots` feature. Lowering
    /// does not publish that prerequisite yet, so callers receive an explicit incomplete outcome
    /// instead of the ordinary non-variable owning-type implication.
    variable_featuring_requires_snapshots: Box<[bool]>,
    /// The KerML type-relationship operands each type owns, in authored order.
    ///
    /// Deliberately outside [`SpecializationClosure`]. `Unioning`, `Intersecting`, `Differencing`
    /// and `Disjoining` are direct kinds of `Relationship`, not of `Specialization`, so folding
    /// their targets into the closure would put a union's operands into its `supertypes` and make
    /// a set-membership statement answer a generalization question. The set semantics they do
    /// entail are derived by the conformance query from this table, with their own provenance.
    set_operands: Rows<(u32, SetOperator, DeclarationId)>,
    /// How many positional connector ends each declaration authors, indexed by ordinal.
    authored_ends: Box<[u32]>,
    /// How many positional ends each declaration effectively has.
    ///
    /// A connection-like declaration that specializes another inherits its ends, and ends it
    /// authors itself redefine the inherited ones positionally rather than adding to them. The
    /// effective count is therefore the larger of what it authors and the most any of its
    /// specialization ancestors authors -- read straight off the transitive closure, so no separate
    /// fixed point is needed.
    ///
    /// Kept beside `authored_ends` rather than replacing it: a rule that asks "did the author write
    /// one end" and a rule that asks "does this declaration have two ends" are different questions,
    /// and the legacy check could only answer the first, which is why it fell silent whenever an
    /// ancestor declared any end at all.
    effective_ends: Box<[u32]>,
}

/// A KerML type-relationship operator, as a set operation over what its operands classify.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum SetOperator {
    /// The owner classifies exactly what any operand classifies.
    Union,
    /// The owner classifies exactly what every operand classifies.
    Intersection,
    /// The owner classifies what the first operand classifies, less everything the rest classify.
    Difference,
    /// The owner and its operands share no instances. Carries no inclusion entailment.
    Disjoint,
}

impl TypeIndex {
    pub(crate) fn build(
        storage: &SemanticModelStorage,
        resolution: &ResolutionResults,
    ) -> Result<Self, ResolutionError> {
        let count = storage.declarations.len();
        let specialization = SpecializationClosure::build(storage, resolution)?;

        let mut direct_types = Vec::new();
        let mut direct_featuring = Vec::new();
        let mut chaining = Vec::new();
        let mut supertypes = Vec::new();
        let mut subtypes = Vec::new();
        let mut edge = |source: DeclarationId,
                        target: DeclarationId,
                        kind: ReferenceKind,
                        provenance: FactProvenance| {
            if let Some(scopes) = edge_scopes(kind) {
                supertypes.push((source, (target, scopes)));
                subtypes.push((target, (source, scopes)));
            }
            if kind == ReferenceKind::FeatureTyping {
                direct_types.push((source, (target, provenance)));
            }
            if kind == ReferenceKind::TypeFeaturing {
                direct_featuring.push((source, (target, provenance)));
            }
            if kind == ReferenceKind::FeatureChaining {
                chaining.push((source, (target, provenance)));
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
        let direct_featuring = Rows::build(count, direct_featuring)?;
        let chaining = Rows::build(count, chaining)?;
        let supertypes = Rows::build(count, supertypes)?;
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

        // Ordinals are per `(source, kind)` and assigned in authored order, so sorting the row by
        // `(ordinal, operator)` recovers the authored sequence -- which `Difference` needs, since
        // its first operand is the type being reduced and the rest are exclusions.
        let mut set_operands = Vec::new();
        for (index, reference) in storage.references.iter().enumerate() {
            let Some(operator) = set_operator(reference.kind) else {
                continue;
            };
            let id =
                AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            // Only a settled single target contributes an operand: an unresolved or ambiguous one
            // would make the operand set silently smaller than the author wrote, and a set
            // entailment computed over a short set is not a weaker answer but a wrong one.
            if let Some(ResolutionStatus::Resolved(target)) = resolution.outcome(id) {
                set_operands.push((reference.source, (reference.ordinal, operator, target)));
            }
        }
        let set_operands = Rows::build(count, set_operands)?;

        let mut authored_ends = vec![0u32; count];
        for (index, facts) in storage.declaration_facts.iter().enumerate() {
            if facts.positional_end.is_none() {
                continue;
            }
            let Some(owner) = storage
                .declarations
                .get(index)
                .and_then(|declaration| declaration.owner)
            else {
                continue;
            };
            let slot = authored_ends
                .get_mut(owner.index())
                .ok_or(ResolutionError::InvalidStorage)?;
            *slot = slot.checked_add(1).ok_or(ResolutionError::Capacity)?;
        }
        let mut effective_ends = authored_ends.clone();
        for index in 0..count {
            let declaration =
                DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            for (ancestor, _) in specialization.entries(declaration) {
                let inherited = authored_ends
                    .get(ancestor.index())
                    .copied()
                    .unwrap_or_default();
                let slot = effective_ends
                    .get_mut(index)
                    .ok_or(ResolutionError::InvalidStorage)?;
                *slot = (*slot).max(inherited);
            }
        }

        // `deriveFeatureFeaturingType` is an owner-side fixed point over canonical facts: direct
        // TypeFeaturing targets are retained with their provenance, while a chaining feature
        // contributes the already-derived featuring types of its first/only resolved target. Each
        // pass reads a complete prior vector and publishes a new one, so traversal order cannot
        // affect the answer. A path adds at most one declaration per hop; `count` passes therefore
        // cover every simple path, including cycles with a reachable direct fact.
        let mut featuring_rows = Vec::with_capacity(count);
        for index in 0..count {
            let declaration =
                DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            featuring_rows.push(
                direct_featuring
                    .row(declaration)
                    .iter()
                    .copied()
                    .collect::<std::collections::BTreeSet<_>>(),
            );
        }
        for _ in 0..count {
            let mut next = featuring_rows.clone();
            for (index, next_row) in next.iter_mut().enumerate() {
                let source =
                    DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
                for (target, _) in chaining.row(source) {
                    let Some(target_row) = featuring_rows.get(target.index()) else {
                        return Err(ResolutionError::InvalidStorage);
                    };
                    next_row.extend(
                        target_row
                            .iter()
                            .map(|(featuring, _)| (*featuring, FactProvenance::Implied)),
                    );
                }
            }
            if next == featuring_rows {
                break;
            }
            featuring_rows = next;
        }
        let mut featuring = Vec::new();
        for (index, targets) in featuring_rows.into_iter().enumerate() {
            let source = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            featuring.extend(targets.into_iter().map(|target| (source, target)));
        }
        let featuring = Rows::build(count, featuring)?;
        let mut variable_featuring_requires_snapshots = vec![false; count];
        for membership in storage.memberships.iter() {
            if membership.kind != MembershipKind::Feature {
                continue;
            }
            let Some(facts) = storage.declaration_facts(membership.member) else {
                return Err(ResolutionError::InvalidStorage);
            };
            if facts.modifiers.var {
                let slot = variable_featuring_requires_snapshots
                    .get_mut(membership.member.index())
                    .ok_or(ResolutionError::InvalidStorage)?;
                *slot = true;
            }
        }

        Ok(Self {
            specialization,
            direct_types,
            supertypes,
            subtypes,
            effective_types,
            featuring,
            variable_featuring_requires_snapshots: variable_featuring_requires_snapshots
                .into_boxed_slice(),
            set_operands,
            authored_ends: authored_ends.into_boxed_slice(),
            effective_ends: effective_ends.into_boxed_slice(),
        })
    }

    /// The single featuring type of `declaration`, if its canonical fact row has one target.
    /// Call [`Self::featuring_types`] when a consumer can represent every authored/derived target.
    pub(crate) fn featuring_type(&self, declaration: DeclarationId) -> Option<DeclarationId> {
        let row = self.featuring.row(declaration);
        (row.len() == 1).then(|| row[0].0)
    }

    /// Every canonical effective featuring type, with its provenance.
    pub(crate) fn featuring_types(
        &self,
        declaration: DeclarationId,
    ) -> &[(DeclarationId, FactProvenance)] {
        self.featuring.row(declaration)
    }

    pub(crate) fn featuring_requires_snapshots(&self, declaration: DeclarationId) -> bool {
        self.variable_featuring_requires_snapshots
            .get(declaration.index())
            .copied()
            .unwrap_or(false)
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

    /// Declarations `declaration` directly specializes, with the scopes of each edge.
    pub(crate) fn supertypes(&self, declaration: DeclarationId) -> &[(DeclarationId, u8)] {
        self.supertypes.row(declaration)
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

    /// The type-relationship operands `declaration` owns, in authored order.
    pub(crate) fn set_operands(
        &self,
        declaration: DeclarationId,
    ) -> &[(u32, SetOperator, DeclarationId)] {
        self.set_operands.row(declaration)
    }

    /// The operands of one operator on `declaration`, in authored order.
    ///
    /// Authored order is the row order because ordinals are assigned in it; `Difference` reads the
    /// first entry as the reduced type and the remainder as exclusions, and a second `differences`
    /// clause continues that same list rather than starting a new one.
    pub(crate) fn operands_of(
        &self,
        declaration: DeclarationId,
        operator: SetOperator,
    ) -> impl Iterator<Item = DeclarationId> + '_ {
        self.set_operands(declaration)
            .iter()
            .filter(move |(_, candidate, _)| *candidate == operator)
            .map(|(_, _, target)| *target)
    }
}

impl TypeIndex {
    /// How many positional connector ends `declaration` authors itself.
    pub(crate) fn authored_ends(&self, declaration: DeclarationId) -> u32 {
        self.authored_ends
            .get(declaration.index())
            .copied()
            .unwrap_or_default()
    }

    /// How many positional ends `declaration` has, counting those it inherits.
    pub(crate) fn effective_ends(&self, declaration: DeclarationId) -> u32 {
        self.effective_ends
            .get(declaration.index())
            .copied()
            .unwrap_or_default()
    }
}

/// The set operation one reference kind states, or `None` if it states none.
pub(crate) fn set_operator(kind: ReferenceKind) -> Option<SetOperator> {
    match kind {
        ReferenceKind::Unioning => Some(SetOperator::Union),
        ReferenceKind::Intersecting => Some(SetOperator::Intersection),
        ReferenceKind::Differencing => Some(SetOperator::Difference),
        ReferenceKind::Disjoining => Some(SetOperator::Disjoint),
        _ => None,
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
///
/// Rows are sorted vectors rather than maps, and each pass rebuilds the next row from scratch
/// rather than copying the previous one forward. Carrying a per-declaration map across passes cost
/// a deep clone of every ancestor set on every pass -- for the standard library that was the
/// single most expensive step of the whole publication barrier.
fn saturate(
    direct: &[Vec<(DeclarationId, u8)>],
    count: usize,
) -> Result<Vec<Vec<(DeclarationId, u8)>>, ResolutionError> {
    let mut closure = direct.to_vec();
    let mut next: Vec<Vec<(DeclarationId, u8)>> = vec![Vec::new(); count];
    let pass_limit = count.checked_add(1).ok_or(ResolutionError::Capacity)?;
    for _ in 0..pass_limit {
        let mut changed = false;
        for index in 0..count {
            let row = &mut next[index];
            row.clear();
            for (parent, parent_scopes) in &direct[index] {
                row.push((*parent, *parent_scopes));
                for (ancestor, ancestor_scopes) in &closure[parent.index()] {
                    let scopes = parent_scopes & ancestor_scopes;
                    if scopes != 0 {
                        row.push((*ancestor, scopes));
                    }
                }
            }
            merge_scopes(row);
            if *row != closure[index] {
                changed = true;
            }
        }
        std::mem::swap(&mut closure, &mut next);
        if !changed {
            break;
        }
    }
    Ok(closure)
}

/// Sorts one row and folds repeated ancestors into a single entry carrying every scope that
/// reaches it.
fn merge_scopes(row: &mut Vec<(DeclarationId, u8)>) {
    row.sort_unstable();
    let mut written = 0usize;
    for read in 0..row.len() {
        if written > 0 && row[written - 1].0 == row[read].0 {
            row[written - 1].1 |= row[read].1;
            continue;
        }
        row[written] = row[read];
        written += 1;
    }
    row.truncate(written);
}
