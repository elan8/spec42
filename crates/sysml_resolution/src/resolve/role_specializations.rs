//! Library-anchored role specializations: the canonical fact family behind the conditional
//! `check*Specialization` constraints whose condition is a metamodel role of the specializing
//! Feature and whose required general is one exact standard-library feature.
//!
//! The Pilot satisfies these through `getDefaultSupertype` / `getSubactionType` keys of its
//! `ImplicitGeneralizationMap` (`StateUsageImpl` `exclusiveState`/`substate`, `TransitionUsageImpl`
//! `stateTransition`/`actionTransition`). The role is an owned semantic fact of the lowered and
//! resolved model -- effective composition, the owning type's metaclass and `isParallel`, the
//! owning membership's role, and a transition's resolved `source` -- never a name or rendered
//! text. This module owns, once:
//!
//! - which role maps to which library feature ([`LibrarySpecializationRole::anchor_path`]);
//! - which declarations play each role in a publication ([`library_specialization_role_occupants`]);
//! - the implied specialization edges that satisfy those checks
//!   ([`synthesize_library_role_specializations`]).
//!
//! The specialization-check query consumes the same occupants and verifies the published edge
//! set, so synthesis and validation can never disagree about what a role is. The anchor paths
//! mirror the pinned OCL bodies whose digests the constraint manifest records.

use std::collections::BTreeMap;

use sysml_contract::ElementKind;

use crate::lower::storage::SemanticModelStorage;
use crate::model::element_kind::element_kind;
use crate::model::element_kind::membership_role_with_trigger;
use crate::model::DeclarationId;
use crate::model::MembershipKind;
use crate::model::ReferenceKind;
use crate::resolve::implied::LibrarySpecializationAnchor;
use crate::resolve::results::ImpliedRelationship;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionStatus;
use crate::specialization_query::SpecializationCheckKind;
use crate::MembershipRole;

/// A metamodel role whose occupant must specialize one exact standard-library feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LibrarySpecializationRole {
    /// SysML 8.3.18.6 `checkStateUsageExclusiveStateSpecialization`: a substate usage of a
    /// non-parallel StateDefinition or StateUsage (`isSubstateUsage(false)`).
    ExclusiveState,
    /// SysML 8.3.18.6 `checkStateUsageSubstateSpecialization`: a substate usage of a parallel
    /// StateDefinition or StateUsage (`isSubstateUsage(true)`).
    Substate,
    /// SysML 8.3.18.9 `checkTransitionUsageStateSpecialization`: a composite TransitionUsage owned
    /// by a StateDefinition or StateUsage whose `source` is a StateUsage.
    StateTransition,
    /// SysML 8.3.18.9 `checkTransitionUsageActionSpecialization`: a composite TransitionUsage
    /// owned by an ActionDefinition or ActionUsage whose `source` is not a StateUsage.
    ActionTransition,
    /// KerML 8.3.3.3.4 `checkFeatureSuboccurrenceSpecialization`: a composite Feature with an
    /// owned typing by a Class whose owning type is a Class or a Feature typed by a Class.
    FeatureSuboccurrence,
    /// KerML 8.3.3.3.4 `checkFeaturePortionSpecialization`: a portion Feature with an owned
    /// typing by a Class whose owning type is a Class or a Feature typed by a Class.
    FeaturePortion,
    /// KerML 8.3.3.3.4 `checkFeatureSubobjectSpecialization`: a composite Feature with an owned
    /// typing by a Structure whose owning type is a Structure or a Feature typed by a Structure.
    FeatureSubobject,
    /// SysML 8.3.9.4 `checkOccurrenceUsageSuboccurrenceSpecialization`: a composite
    /// OccurrenceUsage whose owning type is a Class, an OccurrenceUsage, or a Feature typed by a
    /// Class.
    OccurrenceUsageSuboccurrence,
}

impl LibrarySpecializationRole {
    pub(crate) const ALL: [Self; 8] = [
        Self::ExclusiveState,
        Self::Substate,
        Self::StateTransition,
        Self::ActionTransition,
        Self::FeatureSuboccurrence,
        Self::FeaturePortion,
        Self::FeatureSubobject,
        Self::OccurrenceUsageSuboccurrence,
    ];

    /// The structural path of the library feature this role's occupant specializes.
    pub(crate) fn anchor_path(self) -> &'static [&'static str] {
        match self {
            Self::ExclusiveState => &["States", "StateAction", "exclusiveStates"],
            Self::Substate => &["States", "StateAction", "substates"],
            Self::StateTransition => &["States", "StateAction", "stateTransitions"],
            Self::ActionTransition => &["Actions", "Action", "decisionTransitions"],
            Self::FeatureSuboccurrence | Self::OccurrenceUsageSuboccurrence => {
                &["Occurrences", "Occurrence", "suboccurrences"]
            }
            Self::FeaturePortion => &["Occurrences", "Occurrence", "portions"],
            Self::FeatureSubobject => &["Objects", "Object", "subobjects"],
        }
    }

    /// The exact specialization check this role's library specialization satisfies.
    pub(crate) fn check(self) -> SpecializationCheckKind {
        match self {
            Self::ExclusiveState => SpecializationCheckKind::StateUsageExclusiveState,
            Self::Substate => SpecializationCheckKind::StateUsageSubstate,
            Self::StateTransition => SpecializationCheckKind::TransitionUsageState,
            Self::ActionTransition => SpecializationCheckKind::TransitionUsageAction,
            Self::FeatureSuboccurrence => SpecializationCheckKind::FeatureSuboccurrence,
            Self::FeaturePortion => SpecializationCheckKind::FeaturePortion,
            Self::FeatureSubobject => SpecializationCheckKind::FeatureSubobject,
            Self::OccurrenceUsageSuboccurrence => {
                SpecializationCheckKind::OccurrenceUsageSuboccurrence
            }
        }
    }

    /// The root library packages the role anchors live in, so the library closure admits them.
    pub(crate) fn anchor_packages() -> impl Iterator<Item = &'static str> {
        Self::ALL
            .into_iter()
            .filter_map(|role| role.anchor_path().first().copied())
    }
}

/// The settled library feature of every role, resolved once per publication.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct LibrarySpecializationRoleAnchors {
    by_role: BTreeMap<LibrarySpecializationRole, LibrarySpecializationAnchor>,
}

impl LibrarySpecializationRoleAnchors {
    pub(crate) fn resolve(names: &crate::resolve::implied::LibraryAnchorNames<'_>) -> Self {
        Self {
            by_role: LibrarySpecializationRole::ALL
                .into_iter()
                .map(|role| (role, names.resolve_path(role.anchor_path())))
                .collect(),
        }
    }

    pub(crate) fn anchor(&self, role: LibrarySpecializationRole) -> &LibrarySpecializationAnchor {
        self.by_role
            .get(&role)
            .unwrap_or(&LibrarySpecializationAnchor::Missing)
    }
}

/// One declaration playing a library-anchored specialization role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct LibrarySpecializationRoleOccupant {
    pub(crate) source: DeclarationId,
    pub(crate) role: LibrarySpecializationRole,
}

/// The role occupants of one publication and the checks whose occupancy is not settled.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct LibrarySpecializationRoleOccupants {
    pub(crate) occupants: Vec<LibrarySpecializationRoleOccupant>,
    /// A TransitionUsage whose role depends on a `source` that did not resolve to one
    /// declaration: both transition checks are unsettled rather than guessed.
    pub(crate) transitions_unsettled: bool,
    /// A Feature whose own or owning feature's owned typing did not settle: the Feature
    /// category checks are unsettled rather than guessed.
    pub(crate) features_unsettled: bool,
}

/// Every declaration of the publication that plays a library-anchored specialization role, in
/// identity order. `outcomes` are the settled authored-reference outcomes, indexed like
/// `storage.references`; a transition's `source` is its resolved `transitionSource` reference.
pub(crate) fn library_specialization_role_occupants(
    storage: &SemanticModelStorage,
    outcomes: &[ResolutionStatus],
) -> Result<LibrarySpecializationRoleOccupants, ResolutionError> {
    let count = storage.declarations.len();
    let variant_members = crate::resolve::usage_composition::variant_member_flags(storage)?;
    let mut state_subaction_members = vec![false; count];
    for membership in storage.memberships.iter() {
        let (Some(declaration), Some(facts)) = (
            storage.declaration(membership.member),
            storage.declaration_facts(membership.member),
        ) else {
            return Err(ResolutionError::InvalidStorage);
        };
        let role = membership
            .role
            .or_else(|| membership_role_with_trigger(declaration.kind, facts.is_trigger_action));
        if matches!(role, Some(MembershipRole::StateSubaction(_))) {
            *state_subaction_members
                .get_mut(membership.member.index())
                .ok_or(ResolutionError::InvalidStorage)? = true;
        }
    }
    // `None`: no `transitionSource`; `Some(None)`: a source that did not settle to one target.
    let mut transition_sources: Vec<Option<Option<DeclarationId>>> = vec![None; count];
    for (index, reference) in storage.references.iter().enumerate() {
        if reference.kind != ReferenceKind::TransitionSource {
            continue;
        }
        let slot = transition_sources
            .get_mut(reference.source.index())
            .ok_or(ResolutionError::InvalidStorage)?;
        let target = match outcomes.get(index) {
            Some(ResolutionStatus::Resolved(target)) => Some(*target),
            _ => None,
        };
        *slot = Some(if slot.is_some() { None } else { target });
    }

    // The settled targets of each declaration's owned FeatureTyping (`ownedTyping.type`); an
    // unsettled typing makes the declaration's type category unknown.
    let mut owned_types: Vec<Vec<DeclarationId>> = vec![Vec::new(); count];
    let mut typing_unsettled = vec![false; count];
    for (index, reference) in storage.references.iter().enumerate() {
        if reference.kind != ReferenceKind::FeatureTyping {
            continue;
        }
        let slot = reference.source.index();
        match outcomes.get(index) {
            Some(ResolutionStatus::Resolved(target)) => owned_types
                .get_mut(slot)
                .ok_or(ResolutionError::InvalidStorage)?
                .push(*target),
            _ => {
                *typing_unsettled
                    .get_mut(slot)
                    .ok_or(ResolutionError::InvalidStorage)? = true
            }
        }
    }

    // KerML `Feature::owningType` is the owning type of the Feature's owning FeatureMembership;
    // a Feature owned through any other membership has none.
    let mut feature_members = vec![false; count];
    for membership in storage.memberships.iter() {
        if membership.kind == MembershipKind::Feature {
            *feature_members
                .get_mut(membership.member.index())
                .ok_or(ResolutionError::InvalidStorage)? = true;
        }
    }

    let kind_of = |id: DeclarationId| storage.declaration(id).map(|d| element_kind(d.kind));
    let conforms = |id: DeclarationId, general: ElementKind| {
        kind_of(id).is_some_and(|kind| kind.conforms_to(general))
    };
    let mut result = LibrarySpecializationRoleOccupants::default();
    for (index, declaration) in storage.declarations.iter().enumerate() {
        let source = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        let kind = element_kind(declaration.kind);
        if kind.conforms_to(ElementKind::Feature) && feature_members[index] {
            feature_category_occupants(
                storage,
                &variant_members,
                &owned_types,
                &typing_unsettled,
                source,
                &mut result,
            )?;
        }
        let is_state = kind.conforms_to(ElementKind::StateUsage);
        let is_transition = kind.conforms_to(ElementKind::TransitionUsage);
        if !is_state && !is_transition {
            continue;
        }
        let Some(owner) = declaration.owner else {
            continue;
        };
        if crate::resolve::usage_composition::usage_is_composite(storage, &variant_members, source)
            != Some(true)
        {
            continue;
        }
        let owned_by_state = conforms(owner, ElementKind::StateDefinition)
            || conforms(owner, ElementKind::StateUsage);
        if is_state {
            // `isSubstateUsage(isParallel)`: not the member of a StateSubactionMembership.
            if !owned_by_state || state_subaction_members[index] {
                continue;
            }
            let parallel = storage
                .declaration_facts(owner)
                .ok_or(ResolutionError::InvalidStorage)?
                .modifiers
                .parallel;
            result.occupants.push(LibrarySpecializationRoleOccupant {
                source,
                role: if parallel {
                    LibrarySpecializationRole::Substate
                } else {
                    LibrarySpecializationRole::ExclusiveState
                },
            });
            continue;
        }
        let owned_by_action = conforms(owner, ElementKind::ActionDefinition)
            || conforms(owner, ElementKind::ActionUsage);
        if !owned_by_state && !owned_by_action {
            continue;
        }
        let source_is_state = match transition_sources[index] {
            Some(Some(target)) => conforms(target, ElementKind::StateUsage),
            // A transition without an authored source, or whose source did not settle, has no
            // `source` this publication can classify.
            Some(None) | None => {
                result.transitions_unsettled = true;
                continue;
            }
        };
        if owned_by_state && source_is_state {
            result.occupants.push(LibrarySpecializationRoleOccupant {
                source,
                role: LibrarySpecializationRole::StateTransition,
            });
        }
        if owned_by_action && !source_is_state {
            result.occupants.push(LibrarySpecializationRoleOccupant {
                source,
                role: LibrarySpecializationRole::ActionTransition,
            });
        }
    }
    result.occupants.sort_unstable();
    result.occupants.dedup();
    Ok(result)
}

/// The KerML Feature category roles (`suboccurrence`, `portion`, `subobject`) and SysML's
/// OccurrenceUsage suboccurrence role of one Feature, read from its effective composition, its
/// `portion` fact, its owned typings, and its owning type's metaclass or owned typings, as the
/// Pilot's `FeatureAdapter.isSuboccurrence`/`isPortion`/`isSubobject` and
/// `OccurrenceUsageAdapter.isSuboccurrence` do over `ownedTyping`.
fn feature_category_occupants(
    storage: &SemanticModelStorage,
    variant_members: &[bool],
    owned_types: &[Vec<DeclarationId>],
    typing_unsettled: &[bool],
    source: DeclarationId,
    result: &mut LibrarySpecializationRoleOccupants,
) -> Result<(), ResolutionError> {
    let declaration = storage
        .declaration(source)
        .ok_or(ResolutionError::InvalidStorage)?;
    let facts = storage
        .declaration_facts(source)
        .ok_or(ResolutionError::InvalidStorage)?;
    let Some(owner) = crate::resolve::usage_composition::owning_type(storage, source) else {
        return Ok(());
    };
    let composite =
        crate::resolve::usage_composition::feature_is_composite(storage, variant_members, source);
    let portion = facts.modifiers.portion || facts.portion_kind.is_some();
    let is_occurrence_usage =
        element_kind(declaration.kind).conforms_to(ElementKind::OccurrenceUsage);
    if !composite && !portion {
        return Ok(());
    }
    let typed_by = |feature: DeclarationId, general: ElementKind| {
        owned_types.get(feature.index()).is_some_and(|types| {
            types.iter().any(|ty| {
                storage
                    .declaration(*ty)
                    .is_some_and(|ty| element_kind(ty.kind).conforms_to(general))
            })
        })
    };
    let owner_kind = element_kind(
        storage
            .declaration(owner)
            .ok_or(ResolutionError::InvalidStorage)?
            .kind,
    );
    let owner_is_feature = owner_kind.conforms_to(ElementKind::Feature);
    // Whether the category of `feature`'s or its owner's typing is not yet known.
    let unsettled = |feature: DeclarationId| {
        typing_unsettled
            .get(feature.index())
            .copied()
            .unwrap_or(true)
    };
    if unsettled(source) || (owner_is_feature && unsettled(owner)) {
        result.features_unsettled = true;
        return Ok(());
    }
    let owner_is = |general: ElementKind| {
        owner_kind.conforms_to(general) || (owner_is_feature && typed_by(owner, general))
    };
    let typed_by_class = typed_by(source, ElementKind::Class);
    let mut push = |role| {
        result
            .occupants
            .push(LibrarySpecializationRoleOccupant { source, role })
    };
    // `Objects::Object::subobjects` subsets `suboccurrences`, so a subobject already satisfies the
    // suboccurrence checks; like the Pilot's `getDefaultSupertype`, it gets only the narrower edge.
    let subobject =
        composite && typed_by(source, ElementKind::Structure) && owner_is(ElementKind::Structure);
    if subobject {
        push(LibrarySpecializationRole::FeatureSubobject);
        if portion && typed_by_class && owner_is(ElementKind::Class) {
            push(LibrarySpecializationRole::FeaturePortion);
        }
        return Ok(());
    }
    if composite && typed_by_class && owner_is(ElementKind::Class) {
        push(LibrarySpecializationRole::FeatureSuboccurrence);
    }
    if portion && typed_by_class && owner_is(ElementKind::Class) {
        push(LibrarySpecializationRole::FeaturePortion);
    }
    if composite
        && is_occurrence_usage
        && (owner_is(ElementKind::Class) || owner_kind.conforms_to(ElementKind::OccurrenceUsage))
    {
        push(LibrarySpecializationRole::OccurrenceUsageSuboccurrence);
    }
    Ok(())
}

/// Synthesizes the implied library specialization of every role occupant, as the Pilot's
/// `addDefaultGeneralType` adds the role's default supertype. The library's own declaration of
/// an anchor (`abstract state exclusiveStates` in the non-parallel `States::StateAction`) is the
/// anchor, and no Feature specializes itself. A role whose anchor is missing or ambiguous yields
/// no edge; the check reports it unresolved.
pub(crate) fn synthesize_library_role_specializations(
    storage: &SemanticModelStorage,
    outcomes: &[ResolutionStatus],
    implied_so_far: &[ImpliedRelationship],
    anchors: &LibrarySpecializationRoleAnchors,
) -> Result<Vec<ImpliedRelationship>, ResolutionError> {
    let occupants = library_specialization_role_occupants(storage, outcomes)?;
    if occupants.occupants.is_empty() {
        return Ok(Vec::new());
    }
    // The settled generalization edges an occupant already has, authored or implied so far: a
    // role whose anchor is already a general of its occupant ("directly or indirectly") needs no
    // further edge, and one whose anchor already specializes its occupant (a library feature that
    // generalizes the anchor itself) would make a cycle.
    let mut generals: Vec<Vec<DeclarationId>> = vec![Vec::new(); storage.declarations.len()];
    for (index, reference) in storage.references.iter().enumerate() {
        if !reference.kind.is_specialization() {
            continue;
        }
        if let Some(ResolutionStatus::Resolved(target)) = outcomes.get(index) {
            generals
                .get_mut(reference.source.index())
                .ok_or(ResolutionError::InvalidStorage)?
                .push(*target);
        }
    }
    for relationship in implied_so_far {
        if relationship.kind.is_specialization() {
            generals
                .get_mut(relationship.source.index())
                .ok_or(ResolutionError::InvalidStorage)?
                .push(relationship.target);
        }
    }
    // Per anchor, once: every declaration that already reaches it (reverse traversal) and every
    // declaration it reaches, so each occupant is answered in constant time.
    let count = storage.declarations.len();
    let mut specifics: Vec<Vec<DeclarationId>> = vec![Vec::new(); count];
    for (index, targets) in generals.iter().enumerate() {
        let source = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        for target in targets {
            specifics
                .get_mut(target.index())
                .ok_or(ResolutionError::InvalidStorage)?
                .push(source);
        }
    }
    let closure = |start: DeclarationId, edges: &[Vec<DeclarationId>]| {
        let mut seen = vec![false; count];
        let mut pending = vec![start];
        while let Some(current) = pending.pop() {
            if let Some(slot) = seen.get_mut(current.index()) {
                if !*slot {
                    *slot = true;
                    if let Some(next) = edges.get(current.index()) {
                        pending.extend(next.iter().copied());
                    }
                }
            }
        }
        seen
    };
    let mut anchor_reach = std::collections::BTreeMap::new();
    for role in LibrarySpecializationRole::ALL {
        if let LibrarySpecializationAnchor::Resolved(anchor) = anchors.anchor(role) {
            anchor_reach
                .entry(*anchor)
                .or_insert_with(|| (closure(*anchor, &specifics), closure(*anchor, &generals)));
        }
    }
    let mut implied = Vec::new();
    for occupant in occupants.occupants {
        let LibrarySpecializationAnchor::Resolved(anchor) = anchors.anchor(occupant.role) else {
            continue;
        };
        let Some((reaching, reached)) = anchor_reach.get(anchor) else {
            continue;
        };
        if reaching[occupant.source.index()] || reached[occupant.source.index()] {
            continue;
        }
        implied.push(ImpliedRelationship {
            kind: ReferenceKind::Subsetting,
            source: occupant.source,
            target: *anchor,
        });
    }
    implied.sort_by_key(|relationship| (relationship.source.0, relationship.target.0));
    implied.dedup();
    Ok(implied)
}
