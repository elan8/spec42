//! Canonical element-kind predicates and resolution allowlists for sysml_model.

use crate::{ElementKind, SemanticNode};

/// Why a qualified or simple name is being resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionRole {
    Typing,
    Specializes,
    NameLookupRule6,
    Subject,
    VerifiedRequirement,
    AnnotatedElement,
}

/// Canonical `#kind` suffix spellings used by `qualified_name_for_node` disambiguation.
pub const DISAMBIGUATION_SUFFIXES: &[&str] = &[
    "part_def",
    "port_def",
    "action_def",
    "state_def",
    "view_def",
    "viewpoint_def",
    "viewpoint",
    "flow_def",
    "allocation_def",
    "requirement_def",
    "use_case_def",
    "attribute_def",
    "enum_def",
    "item_def",
    "occurrence_def",
    "interface",
    "interface_def",
    "concern_def",
    "alias",
    "kermlDecl",
    "individual_def",
    "connection_def",
    "metadata_def",
    "constraint_def",
    "calc_def",
    "case_def",
    "analysis_def",
    "verification_def",
    "rendering_def",
];

pub const TYPING_TARGET_KINDS: &[ElementKind] = &[
    ElementKind::PartDef,
    ElementKind::PortDef,
    ElementKind::InterfaceDef,
    ElementKind::ItemDef,
    ElementKind::AttributeDef,
    ElementKind::ActionDef,
    ElementKind::OccurrenceDef,
    ElementKind::FlowDef,
    ElementKind::AllocationDef,
    ElementKind::StateDef,
    ElementKind::RequirementDef,
    ElementKind::Requirement,
    ElementKind::UseCaseDef,
    ElementKind::ConcernDef,
    ElementKind::AnalysisDef,
    ElementKind::VerificationDef,
    ElementKind::ViewDef,
    ElementKind::ViewpointDef,
    ElementKind::RenderingDef,
    ElementKind::MetadataDef,
    ElementKind::EnumDef,
    ElementKind::Alias,
    ElementKind::KermlDecl,
    ElementKind::ConstraintDef,
    ElementKind::CalcDef,
    ElementKind::CaseDef,
    ElementKind::ConjugatedPortDefinition,
    ElementKind::ConnectionDef,
];

pub const SPECIALIZES_TARGET_KINDS: &[ElementKind] = &[
    ElementKind::PartDef,
    ElementKind::PortDef,
    ElementKind::InterfaceDef,
    ElementKind::ItemDef,
    ElementKind::AttributeDef,
    ElementKind::ActionDef,
    ElementKind::OccurrenceDef,
    ElementKind::FlowDef,
    ElementKind::AllocationDef,
    ElementKind::StateDef,
    ElementKind::RequirementDef,
    ElementKind::UseCaseDef,
    ElementKind::ConcernDef,
    ElementKind::EnumDef,
    ElementKind::Alias,
    ElementKind::KermlDecl,
    ElementKind::IndividualDef,
    ElementKind::ConnectionDef,
    ElementKind::MetadataDef,
    ElementKind::ConstraintDef,
    ElementKind::CalcDef,
    ElementKind::CaseDef,
    ElementKind::AnalysisDef,
    ElementKind::VerificationDef,
    ElementKind::ViewDef,
    ElementKind::ViewpointDef,
    ElementKind::RenderingDef,
];

/// Allowed resolved kinds for Rule 6 unresolved-type diagnostics (typing + definitional targets).
pub const RULE6_ALLOWED_KINDS: &[ElementKind] = &[
    ElementKind::PartDef,
    ElementKind::PortDef,
    ElementKind::InterfaceDef,
    ElementKind::ItemDef,
    ElementKind::AttributeDef,
    ElementKind::ActionDef,
    ElementKind::OccurrenceDef,
    ElementKind::FlowDef,
    ElementKind::AllocationDef,
    ElementKind::StateDef,
    ElementKind::RequirementDef,
    ElementKind::UseCaseDef,
    ElementKind::ConcernDef,
    ElementKind::AnalysisDef,
    ElementKind::VerificationDef,
    ElementKind::EnumDef,
    ElementKind::Alias,
    ElementKind::KermlDecl,
    ElementKind::ViewDef,
    ElementKind::ViewpointDef,
    ElementKind::MetadataDef,
    ElementKind::RenderingDef,
    ElementKind::ConnectionDef,
];

pub const SUBJECT_TYPE_TARGET_KINDS: &[ElementKind] = &[
    ElementKind::PartDef,
    ElementKind::PortDef,
    ElementKind::InterfaceDef,
    ElementKind::ItemDef,
    ElementKind::AttributeDef,
    ElementKind::RequirementDef,
    ElementKind::ActionDef,
    ElementKind::OccurrenceDef,
    ElementKind::FlowDef,
    ElementKind::AllocationDef,
    ElementKind::StateDef,
    ElementKind::UseCaseDef,
    ElementKind::ConcernDef,
    ElementKind::AnalysisDef,
];

pub const VERIFIED_REQUIREMENT_TARGET_KINDS: &[ElementKind] =
    &[ElementKind::RequirementDef, ElementKind::Requirement];

pub fn allowed_for_role(role: ResolutionRole) -> &'static [ElementKind] {
    match role {
        ResolutionRole::Typing => TYPING_TARGET_KINDS,
        ResolutionRole::Specializes => SPECIALIZES_TARGET_KINDS,
        ResolutionRole::NameLookupRule6 => RULE6_ALLOWED_KINDS,
        ResolutionRole::Subject => SUBJECT_TYPE_TARGET_KINDS,
        ResolutionRole::VerifiedRequirement => VERIFIED_REQUIREMENT_TARGET_KINDS,
        ResolutionRole::AnnotatedElement => ANNOTATED_ELEMENT_TARGET_KINDS,
    }
}

pub const ANNOTATED_ELEMENT_TARGET_KINDS: &[ElementKind] = &[
    ElementKind::PartDef,
    ElementKind::Part,
    ElementKind::PortDef,
    ElementKind::Port,
    ElementKind::ActionDef,
    ElementKind::Action,
    ElementKind::StateDef,
    ElementKind::State,
    ElementKind::RequirementDef,
    ElementKind::Requirement,
    ElementKind::UseCaseDef,
    ElementKind::UseCase,
    ElementKind::ConcernDef,
    ElementKind::Concern,
    ElementKind::ItemDef,
    ElementKind::Item,
    ElementKind::InterfaceDef,
    ElementKind::Interface,
    ElementKind::MetadataDef,
    ElementKind::MetadataUsage,
    ElementKind::ConstraintDef,
    ElementKind::Constraint,
    ElementKind::Package,
];

pub fn element_kind_allowed(element_kind: &ElementKind, allowed_kinds: &[ElementKind]) -> bool {
    allowed_kinds.contains(element_kind)
}

pub fn is_namespace(element_kind: &ElementKind) -> bool {
    matches!(
        element_kind,
        ElementKind::Package
            | ElementKind::ClassifierDecl
            | ElementKind::RequirementDef
            | ElementKind::Requirement
            | ElementKind::UseCaseDef
            | ElementKind::UseCase
            | ElementKind::AnalysisDef
            | ElementKind::Analysis
            | ElementKind::VerificationDef
            | ElementKind::Verification
            | ElementKind::ConcernDef
            | ElementKind::Concern
    )
}

/// Whether two direct members must have distinct identifiers in `owner`'s namespace.
///
/// Definitions declared directly in a package participate in one classifier namespace even
/// when their concrete metaclasses differ (for example, a `part def` and an `action def`).
/// Other modeled namespace members retain the existing kind-specific distinguishability rule:
/// role members and behavioral features may use the same name when their kinds differ.
pub fn namespace_member_names_must_be_distinguishable(
    owner: &ElementKind,
    left: &ElementKind,
    right: &ElementKind,
) -> bool {
    left == right
        || (*owner == ElementKind::Package && left.is_definition() && right.is_definition())
}

pub fn is_part_like(element_kind: &ElementKind) -> bool {
    matches!(
        element_kind,
        ElementKind::Part
            | ElementKind::PartDef
            | ElementKind::ItemDef
            | ElementKind::OccurrenceDef
    ) || matches!(element_kind, ElementKind::Unknown(s) if s.contains("part"))
}

/// Canonical `element_type`-string form of [`is_part_like`], for callers holding only a
/// projected `element_type: String` (DTOs) rather than a [`SemanticNode`]/[`ElementKind`].
/// Round-trips losslessly through [`ElementKind::parse`]/[`ElementKind::as_str`] for every
/// known spelling; unrecognized strings fall back to [`ElementKind::Unknown`]'s substring match.
pub fn is_part_like_str(element_type: &str) -> bool {
    is_part_like(&ElementKind::parse(element_type))
}

/// `ConjugatedPortDefinition` (KerML 8.3.12.2, the implicit conjugate materialized alongside
/// every `port def`) is port-like: [`allowed_typing_target_kinds`] already accepts it as a valid
/// typing target for a `port` usage, and `diagnostics::helpers::resolve_typed_port_def` treats it
/// as resolving to a real port definition, not a distinct kind.
pub fn is_port_like(element_kind: &ElementKind) -> bool {
    matches!(
        element_kind,
        ElementKind::Port | ElementKind::PortDef | ElementKind::ConjugatedPortDefinition
    ) || matches!(element_kind, ElementKind::Unknown(s) if s.contains("port"))
}

/// String-based form of [`is_port_like`] — see [`is_part_like_str`].
pub fn is_port_like_str(element_type: &str) -> bool {
    is_port_like(&ElementKind::parse(element_type))
}

pub fn is_requirement(element_kind: &ElementKind) -> bool {
    matches!(
        element_kind,
        ElementKind::Requirement | ElementKind::RequirementDef
    )
}

pub fn is_metadata_restriction_attribute(node: &SemanticNode) -> bool {
    !node.declared_facts.relationships.subsetting.is_empty()
        || node
            .declared_facts
            .relationships
            .redefinition
            .iter()
            .any(|target| METADATA_RESTRICTION_FEATURE_NAMES.contains(&target.reference.as_str()))
}

/// Feature names that may appear in metadata def restriction shorthand (`:>` / `:>>`).
pub const METADATA_RESTRICTION_FEATURE_NAMES: &[&str] = &["annotatedElement", "baseType"];

pub fn is_kerml_metadata_supertype(target: &SemanticNode) -> bool {
    if target.declared_facts.metaclass_role
        == Some(crate::semantic::model::KermlMetaclassRole::SemanticMetadata)
    {
        return true;
    }
    if target.name == "SemanticMetadata"
        && matches!(
            target.element_kind,
            ElementKind::KermlDecl | ElementKind::MetadataDef
        )
    {
        return true;
    }
    target.id.qualified_name.ends_with("::SemanticMetadata")
        && matches!(
            target.element_kind,
            ElementKind::KermlDecl | ElementKind::MetadataDef
        )
}

pub fn is_compatible_kind(target_kind: &ElementKind, allowed: &[ElementKind]) -> bool {
    allowed.contains(target_kind)
}

/// Per-usage typing compatibility (diagnostics layer).
pub fn allowed_typing_target_kinds(usage_kind: &ElementKind) -> &'static [ElementKind] {
    match usage_kind {
        ElementKind::Part => &[
            ElementKind::PartDef,
            ElementKind::ItemDef,
            ElementKind::OccurrenceDef,
        ],
        ElementKind::Port => &[ElementKind::PortDef, ElementKind::ConjugatedPortDefinition],
        ElementKind::Item => &[ElementKind::ItemDef, ElementKind::PartDef],
        ElementKind::Attribute => &[ElementKind::AttributeDef, ElementKind::EnumDef],
        ElementKind::Action => &[ElementKind::ActionDef],
        ElementKind::State => &[ElementKind::StateDef],
        ElementKind::Requirement => &[ElementKind::RequirementDef],
        ElementKind::UseCase => &[ElementKind::UseCaseDef],
        ElementKind::Analysis => &[ElementKind::AnalysisDef],
        ElementKind::Verification => &[ElementKind::VerificationDef],
        ElementKind::View => &[ElementKind::ViewDef],
        ElementKind::Viewpoint => &[ElementKind::ViewpointDef],
        ElementKind::Concern => &[ElementKind::ConcernDef],
        ElementKind::Actor | ElementKind::Stakeholder => &[
            ElementKind::PartDef,
            ElementKind::ItemDef,
            ElementKind::OccurrenceDef,
        ],
        ElementKind::Flow => &[ElementKind::FlowDef],
        ElementKind::Allocation => &[ElementKind::AllocationDef],
        ElementKind::Interface => &[ElementKind::InterfaceDef],
        ElementKind::Connection => &[ElementKind::ConnectionDef],
        ElementKind::MetadataUsage => &[ElementKind::MetadataDef],
        ElementKind::MetadataKeyword => &[ElementKind::MetadataDef],
        ElementKind::Rendering => &[ElementKind::RenderingDef],
        ElementKind::ViewRendering => &[ElementKind::RenderingDef, ElementKind::Rendering],
        ElementKind::Perform => &[ElementKind::ActionDef, ElementKind::Action],
        ElementKind::Subject => SUBJECT_TYPE_TARGET_KINDS,
        ElementKind::VerifiedRequirement => VERIFIED_REQUIREMENT_TARGET_KINDS,
        ElementKind::IncludeUseCase => &[ElementKind::UseCaseDef, ElementKind::UseCase],
        ElementKind::Ref => &[
            ElementKind::PartDef,
            ElementKind::PortDef,
            ElementKind::ItemDef,
            ElementKind::AttributeDef,
            ElementKind::ActionDef,
            ElementKind::StateDef,
            ElementKind::RequirementDef,
            ElementKind::UseCaseDef,
            ElementKind::AnalysisDef,
            ElementKind::VerificationDef,
            ElementKind::ViewDef,
            ElementKind::ViewpointDef,
            ElementKind::ConcernDef,
            ElementKind::FlowDef,
            ElementKind::AllocationDef,
            ElementKind::InterfaceDef,
            ElementKind::EnumDef,
            ElementKind::OccurrenceDef,
        ],
        _ => &[],
    }
}

#[cfg(test)]
mod part_port_classification_tests {
    //! Regression tests pinning `is_part_like`/`is_port_like` for the element kinds that
    //! previously diverged across the (now-removed) duplicate string-based classifiers in
    //! `element_kind_classify.rs` and `ibd/extract_impl/kind_classify.rs`.
    use super::*;

    #[test]
    fn item_def_is_part_like() {
        assert!(is_part_like(&ElementKind::ItemDef));
        assert!(is_part_like_str("item def"));
    }

    #[test]
    fn occurrence_def_is_part_like() {
        assert!(is_part_like(&ElementKind::OccurrenceDef));
        assert!(is_part_like_str("occurrence def"));
    }

    #[test]
    fn conjugated_port_definition_is_port_like() {
        assert!(is_port_like(&ElementKind::ConjugatedPortDefinition));
        assert!(is_port_like_str("conjugated port definition"));
    }

    #[test]
    fn conjugated_port_definition_is_not_part_like() {
        assert!(!is_part_like(&ElementKind::ConjugatedPortDefinition));
    }

    #[test]
    fn item_def_and_occurrence_def_are_not_port_like() {
        assert!(!is_port_like(&ElementKind::ItemDef));
        assert!(!is_port_like(&ElementKind::OccurrenceDef));
    }

    #[test]
    fn plain_part_and_port_kinds_still_classify() {
        assert!(is_part_like(&ElementKind::Part));
        assert!(is_part_like(&ElementKind::PartDef));
        assert!(is_port_like(&ElementKind::Port));
        assert!(is_port_like(&ElementKind::PortDef));
    }

    #[test]
    fn str_wrappers_round_trip_through_parse() {
        assert_eq!(
            is_part_like_str(ElementKind::ItemDef.as_str()),
            is_part_like(&ElementKind::ItemDef)
        );
        assert_eq!(
            is_port_like_str(ElementKind::ConjugatedPortDefinition.as_str()),
            is_port_like(&ElementKind::ConjugatedPortDefinition)
        );
    }

    #[test]
    fn unknown_kind_falls_back_to_substring_match() {
        assert!(is_part_like_str("some future part variant"));
        assert!(is_port_like_str("some future port variant"));
        assert!(!is_part_like_str("totally unrelated kind"));
    }
}
