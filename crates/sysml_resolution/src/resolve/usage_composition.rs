//! The effective SysML `Usage::isComposite` / `isReference` derivation over lowered facts.
//!
//! It reads only the lowered storage (modifiers, metaclass, ownership and membership roles), so
//! implied-relationship synthesis that runs before the type index exists and the type index's own
//! queries share this one derivation.

use crate::lower::storage::SemanticModelStorage;
use crate::model::DeclarationId;
use crate::resolve::results::ResolutionError;

/// The owning type of a feature: its owner unless that is a namespace, package, import or alias.
pub(crate) fn owning_type(
    storage: &SemanticModelStorage,
    feature: DeclarationId,
) -> Option<DeclarationId> {
    storage.declaration(feature)?.owner.filter(|owner| {
        storage.declaration(*owner).is_some_and(|owner| {
            !matches!(
                owner.kind,
                crate::model::DeclarationKind::Namespace
                    | crate::model::DeclarationKind::Package
                    | crate::model::DeclarationKind::LibraryPackage
                    | crate::model::DeclarationKind::Import
                    | crate::model::DeclarationKind::Alias
            )
        })
    })
}

/// Which declarations are the member of a VariantMembership, by dense declaration index: the one
/// membership fact [`usage_is_reference`] consults to find a variant's expected featuring type.
pub(crate) fn variant_member_flags(
    storage: &SemanticModelStorage,
) -> Result<Box<[bool]>, ResolutionError> {
    let mut variant_members = vec![false; storage.declarations.len()];
    for membership in storage.memberships.iter() {
        let (Some(declaration), Some(facts)) = (
            storage.declaration(membership.member),
            storage.declaration_facts(membership.member),
        ) else {
            return Err(ResolutionError::InvalidStorage);
        };
        let role = membership.role.or_else(|| {
            crate::model::element_kind::membership_role_with_trigger(
                declaration.kind,
                facts.is_trigger_action,
            )
        });
        if role == Some(crate::MembershipRole::Variant) {
            let slot = variant_members
                .get_mut(membership.member.index())
                .ok_or(ResolutionError::InvalidStorage)?;
            *slot = true;
        }
    }
    Ok(variant_members.into_boxed_slice())
}

/// The effective KerML `Feature::isComposite`: a SysML Usage's derived value, and a KerML
/// Feature's authored `composite`.
pub(crate) fn feature_is_composite(
    storage: &SemanticModelStorage,
    variant_members: &[bool],
    feature: DeclarationId,
) -> bool {
    match usage_is_composite(storage, variant_members, feature) {
        Some(composite) => composite,
        None => storage
            .declaration_facts(feature)
            .is_some_and(|facts| facts.modifiers.composite),
    }
}

/// The effective SysML `Usage::isComposite` (`not isReference`); `None` when `usage` is not a
/// Usage. Synthesis that runs before the type index exists reads the same derivation.
pub(crate) fn usage_is_composite(
    storage: &SemanticModelStorage,
    variant_members: &[bool],
    usage: DeclarationId,
) -> Option<bool> {
    usage_is_reference(storage, variant_members, usage).map(|is_reference| !is_reference)
}

/// The effective SysML `Usage::isReference`, documented on `TypeIndex::usage_is_reference`.
pub(crate) fn usage_is_reference(
    storage: &SemanticModelStorage,
    variant_members: &[bool],
    usage: DeclarationId,
) -> Option<bool> {
    use crate::model::element_kind::element_kind;
    use sysml_contract::ElementKind;
    let declaration = storage.declaration(usage)?;
    if !crate::resolve::is_usage_declaration(declaration.kind) {
        return None;
    }
    let facts = storage.declaration_facts(usage)?;
    let kind = element_kind(declaration.kind);
    if facts.modifiers.reference
        || facts.modifiers.event
        || facts.direction.is_some()
        || facts.modifiers.end
        || facts.positional_end.is_some()
        || matches!(
            declaration.kind,
            crate::model::DeclarationKind::ReferenceUsage
                | crate::model::DeclarationKind::DefaultReferenceUsage
        )
        || [
            ElementKind::AttributeUsage,
            ElementKind::ReferenceUsage,
            ElementKind::PerformActionUsage,
            ElementKind::SuccessionAsUsage,
            ElementKind::BindingConnectorAsUsage,
        ]
        .iter()
        .any(|general| kind.conforms_to(*general))
    {
        return Some(true);
    }
    let Some(featuring_type) = expected_featuring_type(storage, variant_members, usage) else {
        return Some(true);
    };
    if kind.conforms_to(ElementKind::PortUsage) {
        let featured_by_port = storage.declaration(featuring_type).is_some_and(|owner| {
            let owner = element_kind(owner.kind);
            owner.conforms_to(ElementKind::PortDefinition)
                || owner.conforms_to(ElementKind::PortUsage)
        });
        return Some(!featured_by_port);
    }
    Some(false)
}

/// The Pilot's `UsageUtil.getExpectedFeaturingTypeOf`: the owning type of a usage owned
/// through a FeatureMembership, or, for a variant (VariantMembership) owned by a variation
/// usage, that usage's own expected featuring type. A variant of a variation definition and a
/// package-owned usage have none.
fn expected_featuring_type(
    storage: &SemanticModelStorage,
    variant_members: &[bool],
    usage: DeclarationId,
) -> Option<DeclarationId> {
    let mut current = usage;
    // Each step moves to a strictly enclosing owner, so the walk is bounded by nesting depth.
    loop {
        let owner = owning_type(storage, current)?;
        if !variant_members
            .get(current.index())
            .copied()
            .unwrap_or(false)
        {
            return Some(owner);
        }
        let (owner_declaration, owner_facts) = (
            storage.declaration(owner)?,
            storage.declaration_facts(owner)?,
        );
        if !crate::resolve::is_usage_declaration(owner_declaration.kind)
            || !owner_facts
                .modifiers
                .effectively_variation(owner_declaration.kind)
        {
            return None;
        }
        current = owner;
    }
}
