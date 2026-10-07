//! The `[0..1]` role members a Type owns or inherits.
//!
//! A case's `objectiveRequirement` and a Function's or Expression's `result` are each one member
//! playing a role, owned by the Type or inherited from its direct supertypes. Which one a Type has
//! is derived here, once, over the caller's settled direct specialization edges, so the
//! redefinitions a phase synthesizes and the check that reads them agree on it.

use std::collections::BTreeSet;

use crate::model::DeclarationId;
use crate::resolve::results::ResolutionError;

/// A Type's `[0..1]` role member (a case's `objectiveRequirement`, a Function's or Expression's
/// `result`), as far as this publication settles it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InheritedMember {
    /// The Type owns or inherits exactly this member.
    Resolved(DeclarationId),
    /// The Type inherits more than one distinct member, none of which it owns or redefines, so
    /// which one it has is not settled.
    Ambiguous,
    /// The Type neither owns nor inherits a member.
    Absent,
}

/// The role member each declaration owns, indexed by owner. An owner with more than one is
/// recorded as such: which one plays the role is not settled.
#[derive(Debug, Clone)]
pub(crate) struct OwnedMembers {
    owned: Vec<Option<DeclarationId>>,
    owned_twice: Vec<bool>,
}

impl OwnedMembers {
    pub(crate) fn new(count: usize) -> Self {
        Self {
            owned: vec![None; count],
            owned_twice: vec![false; count],
        }
    }

    /// Records `member` as a role member `owner` owns.
    pub(crate) fn insert(
        &mut self,
        owner: DeclarationId,
        member: DeclarationId,
    ) -> Result<(), ResolutionError> {
        let slot = self
            .owned
            .get_mut(owner.index())
            .ok_or(ResolutionError::InvalidStorage)?;
        if slot.is_some_and(|existing| existing != member) {
            self.owned_twice[owner.index()] = true;
        }
        slot.get_or_insert(member);
        Ok(())
    }

    /// The single member `owner` owns, if it owns exactly one.
    pub(crate) fn owned(&self, owner: DeclarationId) -> Option<DeclarationId> {
        if self
            .owned_twice
            .get(owner.index())
            .copied()
            .unwrap_or(false)
        {
            return None;
        }
        self.owned.get(owner.index()).copied().flatten()
    }

    /// Every owner that owns more than one member, in owner order.
    pub(crate) fn owners_with_several(&self) -> impl Iterator<Item = DeclarationId> + '_ {
        self.owned_twice
            .iter()
            .enumerate()
            .filter(|(_, twice)| **twice)
            .filter_map(|(index, _)| DeclarationId::from_index(index).ok())
    }

    /// Every `(owner, member)` pair, in owner order; an owner with several members is left out.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (DeclarationId, DeclarationId)> + '_ {
        self.owned.iter().enumerate().filter_map(|(index, member)| {
            let owner = DeclarationId::from_index(index).ok()?;
            member.and(self.owned(owner)).map(|member| (owner, member))
        })
    }
}

/// Every declaration's role member, indexed by declaration.
///
/// A declaration that owns a member has that one. Otherwise it inherits the members of its direct
/// supertypes, less any that another inherited member redefines (inherited memberships exclude
/// redefined features): none, one, or several (ambiguous). An owned member redefines its
/// supertypes' members (the obligation the callers imply) and whatever it redefines in
/// `authored`, transitively. A supertype still being derived is on a specialization cycle and
/// contributes nothing, so this is one deterministic pass.
pub(crate) fn derive_inherited_members<F>(
    owned_members: &OwnedMembers,
    generals: F,
    authored: &BTreeSet<(DeclarationId, DeclarationId)>,
) -> Result<Vec<InheritedMember>, ResolutionError>
where
    F: Fn(DeclarationId) -> Vec<DeclarationId>,
{
    let count = owned_members.owned.len();
    derive_inherited_members_from(owned_members, generals, authored, 0..count)
}

/// [`derive_inherited_members`], deriving only `roots` and the supertypes they reach.
///
/// A declaration's member depends only on its own supertype closure, so each root's entry equals
/// the full derivation's; entries for declarations no root reaches stay `Absent` and must not be
/// read.
pub(crate) fn derive_inherited_members_from<F>(
    owned_members: &OwnedMembers,
    generals: F,
    authored: &BTreeSet<(DeclarationId, DeclarationId)>,
    roots: impl IntoIterator<Item = usize>,
) -> Result<Vec<InheritedMember>, ResolutionError>
where
    F: Fn(DeclarationId) -> Vec<DeclarationId>,
{
    const UNVISITED: u8 = 0;
    const IN_PROGRESS: u8 = 1;
    const DONE: u8 = 2;
    let count = owned_members.owned.len();
    let owned = &owned_members.owned;
    let owned_twice = &owned_members.owned_twice;
    let mut state = vec![UNVISITED; count];
    let mut members = vec![InheritedMember::Absent; count];
    // The members each owned member redefines, transitively.
    let mut redefines: std::collections::BTreeMap<DeclarationId, BTreeSet<DeclarationId>> =
        std::collections::BTreeMap::new();
    let mut stack = Vec::new();
    for root in roots {
        if state.get(root) != Some(&UNVISITED) {
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
                for general in generals(declaration).into_iter().rev() {
                    if state.get(general.index()) == Some(&UNVISITED) {
                        stack.push((general.index(), false));
                    }
                }
                continue;
            }
            members[node] = if owned_twice[node] {
                // A validation rule reports this (validateCaseDefinitionOnlyOneObjective,
                // validateFunctionResultParameterMembership, ...); which one plays the role is
                // not settled.
                InheritedMember::Ambiguous
            } else if let Some(member) = owned[node] {
                let mut redefined = BTreeSet::new();
                let direct = generals(declaration)
                    .into_iter()
                    .filter(|general| state.get(general.index()) == Some(&DONE))
                    .filter_map(|general| match members[general.index()] {
                        InheritedMember::Resolved(target) => Some(target),
                        InheritedMember::Ambiguous | InheritedMember::Absent => None,
                    })
                    .chain(
                        authored
                            .range((member, DeclarationId(0))..=(member, DeclarationId(u32::MAX)))
                            .map(|(_, target)| *target),
                    )
                    .filter(|target| *target != member)
                    .collect::<Vec<_>>();
                for target in direct {
                    redefined.insert(target);
                    if let Some(transitive) = redefines.get(&target) {
                        redefined.extend(transitive.iter().copied());
                    }
                }
                redefines.insert(member, redefined);
                InheritedMember::Resolved(member)
            } else {
                let mut inherited = BTreeSet::new();
                let mut ambiguous = false;
                for general in generals(declaration) {
                    if state.get(general.index()) != Some(&DONE) {
                        continue;
                    }
                    match members[general.index()] {
                        InheritedMember::Resolved(member) => {
                            inherited.insert(member);
                        }
                        InheritedMember::Ambiguous => ambiguous = true,
                        InheritedMember::Absent => {}
                    }
                }
                let redefined_by_another = inherited
                    .iter()
                    .filter(|candidate| {
                        inherited.iter().any(|other| {
                            other != *candidate
                                && redefines
                                    .get(other)
                                    .is_some_and(|redefined| redefined.contains(candidate))
                        })
                    })
                    .copied()
                    .collect::<Vec<_>>();
                for candidate in redefined_by_another {
                    inherited.remove(&candidate);
                }
                match (ambiguous, inherited.len()) {
                    (false, 0) => InheritedMember::Absent,
                    (false, 1) => InheritedMember::Resolved(
                        *inherited.first().ok_or(ResolutionError::InvalidStorage)?,
                    ),
                    _ => InheritedMember::Ambiguous,
                }
            };
            state[node] = DONE;
        }
    }
    Ok(members)
}
