//! Phase 3: the name and scope indexes the solver looks names up in.

use crate::index::documents::record_visited_index_entries;
use crate::lower::facts::Declaration;
use crate::lower::facts::DeclarationFacts;
use crate::lower::facts::MembershipRecord;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::MembershipKind;
use crate::model::NameId;
use crate::model::ReferenceKind;
use crate::model::Visibility;
use crate::resolve::record_lookup;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionStatus;
use crate::resolve::results::ResolutionWork;
use crate::resolve::DeclarationDomain;
use crate::resolve::ResolutionIndexes;
use crate::resolve::ResolutionReferenceFact;
use crate::MembershipRole;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct NameKey {
    pub(crate) owner: Option<DeclarationId>,
    pub(crate) name: NameId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CandidateRange {
    pub(crate) start: u32,
    pub(crate) len: u32,
}

impl CandidateRange {
    pub(crate) fn from_bounds(start: usize, end: usize) -> Result<Self, ResolutionError> {
        let start = u32::try_from(start).map_err(|_| ResolutionError::Capacity)?;
        let len = u32::try_from(
            end.checked_sub(start as usize)
                .ok_or(ResolutionError::InvalidStorage)?,
        )
        .map_err(|_| ResolutionError::Capacity)?;
        start.checked_add(len).ok_or(ResolutionError::Capacity)?;
        Ok(Self { start, len })
    }

    pub(crate) fn slice<'a, T>(&self, values: &'a [T]) -> Option<&'a [T]> {
        let end = self.start.checked_add(self.len)?;
        values.get(self.start as usize..end as usize)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct NameIndex {
    pub(crate) keys: Box<[NameKey]>,
    pub(crate) ranges: Box<[CandidateRange]>,
    pub(crate) candidates: Box<[DeclarationId]>,
}

/// Effective member enumeration for every lexical scope, including the root scope.
///
/// This is the persistent, frequently-enumerated scope-map family: owned, imported and inherited
/// candidates are merged once at publication. Lookup by a particular name continues to use the
/// canonical origin-specific indexes so shadowing and ambiguity retain their semantic meaning.
#[derive(Debug)]
pub(crate) struct EffectiveScopeIndex {
    pub(crate) ranges: Box<[(u32, u32)]>,
    pub(crate) members: Box<[DeclarationId]>,
}

impl EffectiveScopeIndex {
    pub(crate) fn build(
        declarations: usize,
        direct: &NameIndex,
        imported: &NameIndex,
        inherited: &NameIndex,
    ) -> Result<Self, ResolutionError> {
        let mut ranges = Vec::with_capacity(declarations.saturating_add(1));
        let mut members = Vec::new();
        for slot in 0..=declarations {
            let owner = if slot == 0 {
                None
            } else {
                Some(DeclarationId::from_index(slot - 1).map_err(|_| ResolutionError::Capacity)?)
            };
            let mut scope_members = Vec::new();
            for index in [direct, imported, inherited] {
                for (_, candidates) in index.entries_for_owner(owner) {
                    scope_members.extend_from_slice(candidates);
                }
            }
            scope_members.sort_unstable();
            scope_members.dedup();
            let start = u32::try_from(members.len()).map_err(|_| ResolutionError::Capacity)?;
            members.extend(scope_members);
            let end = u32::try_from(members.len()).map_err(|_| ResolutionError::Capacity)?;
            ranges.push((start, end));
        }
        Ok(Self {
            ranges: ranges.into_boxed_slice(),
            members: members.into_boxed_slice(),
        })
    }

    pub(crate) fn members(&self, owner: Option<DeclarationId>) -> &[DeclarationId] {
        let slot = owner.map_or(0usize, |owner| owner.index().saturating_add(1));
        let Some(&(start, end)) = self.ranges.get(slot) else {
            return &[];
        };
        let members = self
            .members
            .get(start as usize..end as usize)
            .unwrap_or_default();
        record_visited_index_entries(members.len());
        members
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EffectiveVisibility {
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct EffectiveMembership {
    pub(crate) visibility: EffectiveVisibility,
    /// The authored membership kind, retained so inspection need not rescan the record table.
    pub(crate) kind: MembershipKind,
    /// Whether the visibility above was written or defaulted.
    pub(crate) authored: bool,
    pub(crate) role: Option<MembershipRole>,
}

/// Dense, declaration-aligned membership facts used by resolution. Authored `Default` is settled
/// once here, with its owning declaration context, rather than being reinterpreted by each lookup.
#[derive(Debug)]
pub(crate) struct MembershipIndex {
    pub(crate) by_declaration: Box<[EffectiveMembership]>,
}

impl MembershipIndex {
    pub(crate) fn build(
        declarations: &[Declaration],
        memberships: &[MembershipRecord],
    ) -> Result<Self, ResolutionError> {
        let mut by_declaration = vec![None; declarations.len()];
        for membership in memberships {
            let declaration = declarations
                .get(membership.member.index())
                .ok_or(ResolutionError::InvalidStorage)?;
            let slot = by_declaration
                .get_mut(membership.member.index())
                .ok_or(ResolutionError::InvalidStorage)?;
            if slot.is_some()
                || matches!(declaration.kind, DeclarationKind::Import)
                    != matches!(membership.kind, MembershipKind::Import)
            {
                return Err(ResolutionError::InvalidStorage);
            }
            let visibility = match membership.visibility {
                Visibility::Public => EffectiveVisibility::Public,
                Visibility::Private => EffectiveVisibility::Private,
                Visibility::Protected => EffectiveVisibility::Protected,
                Visibility::Default if membership.kind == MembershipKind::Import => {
                    EffectiveVisibility::Private
                }
                Visibility::Default => match declaration.owner {
                    None => EffectiveVisibility::Public,
                    Some(owner)
                        if declarations.get(owner.index()).is_some_and(|owner| {
                            matches!(
                                owner.kind,
                                DeclarationKind::Package | DeclarationKind::LibraryPackage
                            )
                        }) =>
                    {
                        EffectiveVisibility::Public
                    }
                    Some(_) => EffectiveVisibility::Private,
                },
            };
            *slot = Some(EffectiveMembership {
                visibility,
                kind: membership.kind,
                authored: membership.visibility != Visibility::Default,
                role: membership.role,
            });
        }
        let by_declaration = by_declaration
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .ok_or(ResolutionError::InvalidStorage)?;
        Ok(Self {
            by_declaration: by_declaration.into_boxed_slice(),
        })
    }

    pub(crate) fn get(&self, declaration: DeclarationId) -> Option<EffectiveMembership> {
        self.by_declaration.get(declaration.index()).copied()
    }

    pub(crate) fn is_public(&self, declaration: DeclarationId) -> bool {
        self.get(declaration)
            .is_some_and(|membership| membership.visibility == EffectiveVisibility::Public)
    }
}

impl NameIndex {
    pub(crate) fn build(
        mut entries: Vec<(NameKey, DeclarationId)>,
    ) -> Result<Self, ResolutionError> {
        entries.sort_unstable_by_key(name_entry_sort_key);
        entries.dedup();

        let mut keys = Vec::new();
        let mut ranges = Vec::new();
        let mut candidates = Vec::new();
        keys.try_reserve(entries.len())
            .map_err(|_| ResolutionError::Capacity)?;
        ranges
            .try_reserve(entries.len())
            .map_err(|_| ResolutionError::Capacity)?;
        candidates
            .try_reserve(entries.len())
            .map_err(|_| ResolutionError::Capacity)?;

        let mut cursor = 0;
        while cursor < entries.len() {
            let key = entries[cursor].0;
            let start = candidates.len();
            while cursor < entries.len() && entries[cursor].0 == key {
                candidates.push(entries[cursor].1);
                cursor += 1;
            }
            keys.push(key);
            ranges.push(CandidateRange::from_bounds(start, candidates.len())?);
        }

        Ok(Self {
            keys: keys.into_boxed_slice(),
            ranges: ranges.into_boxed_slice(),
            candidates: candidates.into_boxed_slice(),
        })
    }

    pub(crate) fn candidates(
        &self,
        owner: Option<DeclarationId>,
        name: NameId,
    ) -> &[DeclarationId] {
        let key = NameKey { owner, name };
        let Ok(index) = self.keys.binary_search(&key) else {
            return &[];
        };
        let candidates = self.ranges[index]
            .slice(&self.candidates)
            .unwrap_or_default();
        record_visited_index_entries(candidates.len());
        candidates
    }

    pub(crate) fn entries_for_owner(
        &self,
        owner: Option<DeclarationId>,
    ) -> impl Iterator<Item = (NameId, &[DeclarationId])> {
        let start = self.keys.partition_point(|key| key.owner < owner);
        let end = self.keys.partition_point(|key| key.owner <= owner);
        self.keys[start..end]
            .iter()
            .zip(&self.ranges[start..end])
            .filter_map(|(key, range)| {
                range
                    .slice(&self.candidates)
                    .map(|candidates| (key.name, candidates))
            })
    }
}

/// The tuple's canonical `Ord` encoded as one integer comparison.
///
/// `None` sorts before every owner and `Some(u32::MAX)` still fits because the owner occupies 33
/// bits above the two complete 32-bit name/candidate fields. The encoding is injective, so this is
/// purely a cheaper sorting representation rather than a hash or a competing identity policy.
pub(crate) fn name_entry_sort_key((key, candidate): &(NameKey, DeclarationId)) -> u128 {
    let owner = key.owner.map_or(0, |owner| u128::from(owner.0) + 1);
    (owner << 64) | (u128::from(key.name.0) << 32) | u128::from(candidate.0)
}

/// Builds the ancestor-scoped inherited-member lookup index: for each non-cyclic declaration with
/// a non-empty ancestor closure, every name directly owned by any ancestor becomes a candidate for
/// that declaration. `NameIndex::build` sorts and dedups `(owner, name, candidate)` triples, so a
/// member reached through two different ancestor paths to the same target (the diamond case)
/// collapses to one candidate. When multiple ancestors contribute same-named members, the member
/// owned by the most-specific ancestor shadows members owned by its ancestors; members from
/// incomparable ancestors remain distinct candidates and therefore resolve as ambiguous.
pub(crate) fn build_inherited_name_index(
    declarations: &[Declaration],
    direct_names: &NameIndex,
    ancestor_closures: &[Box<[DeclarationId]>],
) -> Result<NameIndex, ResolutionError> {
    build_inherited_name_index_for_scopes(declarations, direct_names, ancestor_closures, None)
}

pub(crate) fn build_inherited_name_index_for_scopes(
    declarations: &[Declaration],
    direct_names: &NameIndex,
    ancestor_closures: &[Box<[DeclarationId]>],
    scope_filter: Option<&std::collections::BTreeSet<DeclarationId>>,
) -> Result<NameIndex, ResolutionError> {
    let mut entries = Vec::new();
    for (index, ancestors) in ancestor_closures.iter().enumerate() {
        if ancestors.is_empty() {
            continue;
        }
        let child = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        if scope_filter.is_some_and(|filter| !filter.contains(&child)) {
            continue;
        }
        for &ancestor in ancestors.iter() {
            for (name, candidates) in direct_names.entries_for_owner(Some(ancestor)) {
                for &candidate in candidates {
                    entries.push((
                        NameKey {
                            owner: Some(child),
                            name,
                        },
                        candidate,
                    ));
                }
            }
        }
    }
    entries.sort_unstable_by_key(name_entry_sort_key);
    let mut visible = Vec::with_capacity(entries.len());
    let mut cursor = 0;
    while cursor < entries.len() {
        let key = entries[cursor].0;
        let end = entries[cursor..]
            .partition_point(|entry| entry.0 == key)
            .checked_add(cursor)
            .ok_or(ResolutionError::Capacity)?;
        for &(entry_key, candidate) in &entries[cursor..end] {
            let candidate_owner = declarations
                .get(candidate.index())
                .ok_or(ResolutionError::InvalidStorage)?
                .owner;
            let shadowed_by_more_specific_owner =
                entries[cursor..end].iter().any(|(_, other_candidate)| {
                    let other_owner = declarations
                        .get(other_candidate.index())
                        .and_then(|declaration| declaration.owner);
                    match (candidate_owner, other_owner) {
                        (Some(candidate_owner), Some(other_owner))
                            if candidate_owner != other_owner =>
                        {
                            ancestor_closures
                                .get(other_owner.index())
                                .is_some_and(|ancestors| {
                                    ancestors.binary_search(&candidate_owner).is_ok()
                                })
                        }
                        _ => false,
                    }
                });
            if !shadowed_by_more_specific_owner {
                visible.push((entry_key, candidate));
            }
        }
        cursor = end;
    }
    NameIndex::build(visible)
}

pub(crate) fn build_direct_name_index(
    declarations: &[Declaration],
    declaration_facts: Option<&[DeclarationFacts]>,
    public_only: Option<&MembershipIndex>,
) -> Result<NameIndex, ResolutionError> {
    let mut entries = Vec::new();
    entries
        .try_reserve(declarations.len())
        .map_err(|_| ResolutionError::Capacity)?;
    for (index, declaration) in declarations.iter().enumerate() {
        let declaration_id =
            DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        if public_only.is_some_and(|memberships| !memberships.is_public(declaration_id)) {
            continue;
        }
        if let Some(name) = declaration.name {
            entries.push((
                NameKey {
                    owner: declaration.owner,
                    name,
                },
                declaration_id,
            ));
        }
        if let Some(short_name) = declaration_facts
            .and_then(|facts| facts.get(index))
            .and_then(|facts| facts.short_name)
        {
            entries.push((
                NameKey {
                    owner: declaration.owner,
                    name: short_name,
                },
                declaration_id,
            ));
        }
    }
    NameIndex::build(entries)
}

pub(crate) fn build_effective_import_indexes<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    memberships: &MembershipIndex,
    references: &[R],
    import_slots: &[usize],
    exported_names: &NameIndex,
    previous_exported_imports: &NameIndex,
    outcomes: &[ResolutionStatus],
) -> Result<(NameIndex, NameIndex), ResolutionError> {
    let mut entries = Vec::new();
    let mut exported_entries = Vec::new();
    for index in import_slots.iter().copied() {
        let reference = references
            .get(index)
            .ok_or(ResolutionError::InvalidStorage)?;
        let ResolutionStatus::Resolved(target) = outcomes[index] else {
            continue;
        };
        let import_owner = declarations
            .get(reference.source().index())
            .ok_or(ResolutionError::InvalidStorage)?
            .owner;
        let import_is_public = memberships.is_public(reference.source());
        match reference.kind() {
            ReferenceKind::NamespaceImport => {
                for (name, candidates) in exported_names.entries_for_owner(Some(target)) {
                    extend_import_entries(
                        &mut entries,
                        &mut exported_entries,
                        import_owner,
                        name,
                        candidates,
                        import_is_public,
                    );
                }
                for (name, candidates) in previous_exported_imports.entries_for_owner(Some(target))
                {
                    extend_import_entries(
                        &mut entries,
                        &mut exported_entries,
                        import_owner,
                        name,
                        candidates,
                        import_is_public,
                    );
                }
            }
            ReferenceKind::MembershipImport => {
                let declaration = declarations
                    .get(target.index())
                    .ok_or(ResolutionError::InvalidStorage)?;
                if memberships.is_public(target) {
                    if let Some(name) = declaration.name {
                        extend_import_entries(
                            &mut entries,
                            &mut exported_entries,
                            import_owner,
                            name,
                            std::slice::from_ref(&target),
                            import_is_public,
                        );
                    }
                }
            }
            ReferenceKind::FilterImport
            | ReferenceKind::ExplicitRelationshipEndpoint
            | ReferenceKind::FeatureTyping
            | ReferenceKind::TypeFeaturing
            | ReferenceKind::FeatureChaining
            | ReferenceKind::Subclassification
            | ReferenceKind::Conjugation
            | ReferenceKind::FeatureInverting
            | ReferenceKind::Subsetting
            | ReferenceKind::Redefinition
            | ReferenceKind::References
            | ReferenceKind::Crosses
            | ReferenceKind::Intersects
            | ReferenceKind::Unioning
            | ReferenceKind::Intersecting
            | ReferenceKind::Differencing
            | ReferenceKind::Disjoining
            | ReferenceKind::AliasBinding
            | ReferenceKind::ConnectorEnd
            | ReferenceKind::Succession
            | ReferenceKind::EntryActionBinding
            | ReferenceKind::DoActionBinding
            | ReferenceKind::ExitActionBinding
            | ReferenceKind::InitialState
            | ReferenceKind::ExpressionOperand
            | ReferenceKind::TransitionSource
            | ReferenceKind::TransitionTarget
            | ReferenceKind::TransitionTrigger
            | ReferenceKind::TransitionEffect
            | ReferenceKind::MetadataAnnotation
            | ReferenceKind::FilterMetadataTest
            | ReferenceKind::SatisfySource
            | ReferenceKind::SatisfyTarget
            | ReferenceKind::AllocateSource
            | ReferenceKind::AllocateTarget
            | ReferenceKind::BindSource
            | ReferenceKind::BindTarget
            | ReferenceKind::IncludeUseCase
            | ReferenceKind::ViewExpose
            | ReferenceKind::MemberAccessOperand
            | ReferenceKind::InvocationCallee
            | ReferenceKind::ThenTarget
            | ReferenceKind::AcceptVia
            | ReferenceKind::SendTarget
            | ReferenceKind::AcceptPayloadType
            | ReferenceKind::TerminateTarget
            | ReferenceKind::FlowSource
            | ReferenceKind::FlowTarget
            | ReferenceKind::TypeCheckTarget
            | ReferenceKind::MetaCastTarget
            | ReferenceKind::StakeholderTarget
            | ReferenceKind::PurposeTarget
            | ReferenceKind::VerifyRequirementTarget
            | ReferenceKind::AssignTarget
            | ReferenceKind::DependencyClient
            | ReferenceKind::DependencySupplier
            | ReferenceKind::PerformParameterTarget
            | ReferenceKind::FlowPayloadType => {}
        }
    }
    Ok((
        NameIndex::build(entries)?,
        NameIndex::build(exported_entries)?,
    ))
}

pub(crate) fn extend_import_entries(
    local: &mut Vec<(NameKey, DeclarationId)>,
    exported: &mut Vec<(NameKey, DeclarationId)>,
    owner: Option<DeclarationId>,
    name: NameId,
    candidates: &[DeclarationId],
    import_is_public: bool,
) {
    let key = NameKey { owner, name };
    local.extend(candidates.iter().copied().map(|candidate| (key, candidate)));
    if import_is_public {
        exported.extend(candidates.iter().copied().map(|candidate| (key, candidate)));
    }
}

/// Walks the enclosing-namespace chain from `owner` outward. At each level, owned members take
/// precedence over inherited (ancestor-scoped) members, which take precedence over imports, per
/// the canonical scope-origin precedence ("owned members, then
/// inherited/general members, then imports"). `inherited_names` is `None` for reference kinds that
/// do not read inherited scope (for example Subclassification itself).
///
/// `domain` is the broad metamodel-domain filter for the reference's final target (section 6 step
/// 8). Per section 11.1 ("an incompatible inner `Type` still shadows a compatible outer type,
/// followed by validation"), domain compatibility never changes *which* precedence tier wins: at
/// each tier, a domain-compatible candidate is preferred when one exists, but a tier that has any
/// same-name binding still shadows lower-precedence tiers even if every candidate at that tier is
/// domain-incompatible. Passing `DeclarationDomain::Any` disables this preference entirely (every
/// candidate matches), which is what callers use when this lookup does not produce the reference's
/// final target (an interior segment of a qualified name).
/// What a lexical lookup is looking for, as opposed to where it looks.
#[derive(Debug, Clone, Copy)]
pub(crate) struct LookupTarget {
    /// The broad metamodel-domain filter for the reference's final target.
    pub(crate) domain: DeclarationDomain,
    /// A declaration that is not in its own scope for this reference, if any.
    pub(crate) excluded: Option<DeclarationId>,
}

pub(crate) fn lookup_lexical_into(
    declarations: &[Declaration],
    indexes: &ResolutionIndexes<'_>,
    mut owner: Option<DeclarationId>,
    name: NameId,
    target: LookupTarget,
    candidates: &mut Vec<DeclarationId>,
    work: &mut ResolutionWork,
) -> Result<(), ResolutionError> {
    let LookupTarget { domain, excluded } = target;
    let select_tier = |raw: &[DeclarationId], out: &mut Vec<DeclarationId>| {
        let compatible = raw
            .iter()
            .copied()
            .filter(|candidate| {
                declarations
                    .get(candidate.index())
                    .is_some_and(|declaration| domain.accepts(declaration.kind))
            })
            .collect::<Vec<_>>();
        if compatible.is_empty() {
            out.extend_from_slice(raw);
        } else {
            out.extend(compatible);
        }
    };
    // Applied before the tier is tested for emptiness, so a tier whose only binding is the
    // excluded declaration does not shadow the tiers below it.
    let visible = |raw: &[DeclarationId], scratch: &mut Vec<DeclarationId>| -> bool {
        let Some(excluded) = excluded else {
            return false;
        };
        if !raw.contains(&excluded) {
            return false;
        }
        scratch.clear();
        scratch.extend(raw.iter().copied().filter(|entry| *entry != excluded));
        true
    };
    let mut filtered = Vec::new();
    loop {
        record_lookup(work)?;
        let mut direct = indexes.direct_names.candidates(owner, name);
        if visible(direct, &mut filtered) {
            direct = &filtered;
        }
        if !direct.is_empty() {
            select_tier(direct, candidates);
            return Ok(());
        }
        if let Some(inherited) = indexes.inherited_names {
            record_lookup(work)?;
            let inherited = inherited.candidates(owner, name);
            if !inherited.is_empty() {
                select_tier(inherited, candidates);
                return Ok(());
            }
        }
        if let Some(imports) = indexes.effective_imports {
            record_lookup(work)?;
            let imported = imports.candidates(owner, name);
            if !imported.is_empty() {
                select_tier(imported, candidates);
                return Ok(());
            }
        }
        let Some(current) = owner else {
            return Ok(());
        };
        owner = declarations
            .get(current.index())
            .ok_or(ResolutionError::InvalidStorage)?
            .owner;
    }
}
