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

pub fn is_part_like(element_kind: &ElementKind) -> bool {
    matches!(
        element_kind,
        ElementKind::Part
            | ElementKind::PartDef
            | ElementKind::ItemDef
            | ElementKind::OccurrenceDef
    ) || matches!(element_kind, ElementKind::Unknown(s) if s.contains("part"))
}

pub fn is_port_like(element_kind: &ElementKind) -> bool {
    matches!(element_kind, ElementKind::Port | ElementKind::PortDef)
        || matches!(element_kind, ElementKind::Unknown(s) if s.contains("port"))
}

pub fn is_requirement(element_kind: &ElementKind) -> bool {
    matches!(
        element_kind,
        ElementKind::Requirement | ElementKind::RequirementDef
    )
}

pub fn is_metadata_restriction_attribute(node: &SemanticNode) -> bool {
    node.attributes.contains_key("subsetsFeature") || is_known_metadata_redefine(node)
}

/// Feature names that may appear in metadata def restriction shorthand (`:>` / `:>>`).
pub const METADATA_RESTRICTION_FEATURE_NAMES: &[&str] = &["annotatedElement", "baseType"];

pub fn is_known_metadata_redefine(node: &SemanticNode) -> bool {
    node.attributes
        .get("redefines")
        .and_then(|value| value.as_str())
        .is_some_and(|feature| METADATA_RESTRICTION_FEATURE_NAMES.contains(&feature))
}

pub fn is_reflective_sysml_usage_type(type_ref: &str, target: &SemanticNode) -> bool {
    type_ref.contains("SysML::")
        && matches!(
            target.element_kind,
            ElementKind::MetadataDef | ElementKind::KermlDecl
        )
}

pub fn is_kerml_metadata_supertype(target: &SemanticNode) -> bool {
    if target
        .attributes
        .get("metaclassRole")
        .and_then(|value| value.as_str())
        == Some("SemanticMetadata")
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

pub fn is_semantic_metadata_base_type_redefine(owner: &SemanticNode, node: &SemanticNode) -> bool {
    node.name == "baseType"
        && node
            .attributes
            .get("redefines")
            .and_then(|value| value.as_str())
            == Some("baseType")
        && owner.element_kind == ElementKind::MetadataDef
        && owner
            .attributes
            .get("specializes")
            .and_then(|value| value.as_str())
            .is_some_and(|value| value.contains("SemanticMetadata"))
}

pub fn is_compatible_kind(target_kind: &ElementKind, allowed: &[ElementKind]) -> bool {
    allowed.contains(target_kind)
}

pub fn is_compatible_specializes_target(def_kind: &ElementKind, target: &SemanticNode) -> bool {
    if is_compatible_kind(
        &target.element_kind,
        allowed_specializes_target_kinds(def_kind),
    ) {
        return true;
    }
    *def_kind == ElementKind::MetadataDef && is_kerml_metadata_supertype(target)
}

/// Per-usage typing compatibility (diagnostics layer).
pub fn allowed_typing_target_kinds(usage_kind: &ElementKind) -> &'static [ElementKind] {
    match usage_kind {
        ElementKind::Part => &[
            ElementKind::PartDef,
            ElementKind::ItemDef,
            ElementKind::OccurrenceDef,
        ],
        ElementKind::Port => &[ElementKind::PortDef],
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

pub fn allowed_specializes_target_kinds(def_kind: &ElementKind) -> &'static [ElementKind] {
    match def_kind {
        ElementKind::PartDef => &[ElementKind::PartDef, ElementKind::OccurrenceDef],
        ElementKind::PortDef => &[ElementKind::PortDef],
        ElementKind::ItemDef => &[ElementKind::ItemDef],
        ElementKind::AttributeDef => &[ElementKind::AttributeDef],
        ElementKind::ActionDef => &[ElementKind::ActionDef],
        ElementKind::StateDef => &[ElementKind::StateDef],
        ElementKind::RequirementDef => &[ElementKind::RequirementDef],
        ElementKind::UseCaseDef => &[ElementKind::UseCaseDef],
        ElementKind::AnalysisDef => &[ElementKind::AnalysisDef],
        ElementKind::VerificationDef => &[ElementKind::VerificationDef],
        ElementKind::ViewDef => &[ElementKind::ViewDef],
        ElementKind::ViewpointDef => &[ElementKind::ViewpointDef],
        ElementKind::ConcernDef => &[ElementKind::ConcernDef],
        ElementKind::FlowDef => &[ElementKind::FlowDef],
        ElementKind::AllocationDef => &[ElementKind::AllocationDef],
        ElementKind::EnumDef => &[ElementKind::EnumDef],
        ElementKind::MetadataDef => &[ElementKind::MetadataDef],
        ElementKind::RenderingDef => &[ElementKind::RenderingDef],
        ElementKind::InterfaceDef => &[ElementKind::InterfaceDef],
        _ => &[],
    }
}

const PART_SUBSET_TARGETS: &[ElementKind] = &[
    ElementKind::Part,
    ElementKind::PartDef,
    ElementKind::ItemDef,
    ElementKind::OccurrenceDef,
];
const PORT_SUBSET_TARGETS: &[ElementKind] = &[ElementKind::Port, ElementKind::PortDef];
const ITEM_SUBSET_TARGETS: &[ElementKind] = &[
    ElementKind::Item,
    ElementKind::ItemDef,
    ElementKind::PartDef,
];
const ATTRIBUTE_SUBSET_TARGETS: &[ElementKind] = &[
    ElementKind::Attribute,
    ElementKind::AttributeDef,
    ElementKind::EnumDef,
];
const ACTION_SUBSET_TARGETS: &[ElementKind] = &[ElementKind::Action, ElementKind::ActionDef];
const STATE_SUBSET_TARGETS: &[ElementKind] = &[ElementKind::State, ElementKind::StateDef];
const REQUIREMENT_SUBSET_TARGETS: &[ElementKind] =
    &[ElementKind::Requirement, ElementKind::RequirementDef];
const USE_CASE_SUBSET_TARGETS: &[ElementKind] = &[ElementKind::UseCase, ElementKind::UseCaseDef];
const ANALYSIS_SUBSET_TARGETS: &[ElementKind] = &[ElementKind::Analysis, ElementKind::AnalysisDef];
const VERIFICATION_SUBSET_TARGETS: &[ElementKind] =
    &[ElementKind::Verification, ElementKind::VerificationDef];
const VIEW_SUBSET_TARGETS: &[ElementKind] = &[ElementKind::View, ElementKind::ViewDef];
const VIEWPOINT_SUBSET_TARGETS: &[ElementKind] =
    &[ElementKind::Viewpoint, ElementKind::ViewpointDef];
const CONCERN_SUBSET_TARGETS: &[ElementKind] = &[ElementKind::Concern, ElementKind::ConcernDef];
const ACTOR_SUBSET_TARGETS: &[ElementKind] = &[
    ElementKind::Actor,
    ElementKind::Part,
    ElementKind::PartDef,
    ElementKind::ItemDef,
    ElementKind::OccurrenceDef,
];
const FLOW_SUBSET_TARGETS: &[ElementKind] = &[ElementKind::Flow, ElementKind::FlowDef];
const ALLOCATION_SUBSET_TARGETS: &[ElementKind] =
    &[ElementKind::Allocation, ElementKind::AllocationDef];
const INTERFACE_SUBSET_TARGETS: &[ElementKind] =
    &[ElementKind::Interface, ElementKind::InterfaceDef];

pub fn allowed_subset_redefine_target_kinds(usage_kind: &ElementKind) -> &'static [ElementKind] {
    match usage_kind {
        ElementKind::Part => PART_SUBSET_TARGETS,
        ElementKind::Port => PORT_SUBSET_TARGETS,
        ElementKind::Item => ITEM_SUBSET_TARGETS,
        ElementKind::Attribute => ATTRIBUTE_SUBSET_TARGETS,
        ElementKind::Action => ACTION_SUBSET_TARGETS,
        ElementKind::State => STATE_SUBSET_TARGETS,
        ElementKind::Requirement => REQUIREMENT_SUBSET_TARGETS,
        ElementKind::UseCase => USE_CASE_SUBSET_TARGETS,
        ElementKind::Analysis => ANALYSIS_SUBSET_TARGETS,
        ElementKind::Verification => VERIFICATION_SUBSET_TARGETS,
        ElementKind::View => VIEW_SUBSET_TARGETS,
        ElementKind::Viewpoint => VIEWPOINT_SUBSET_TARGETS,
        ElementKind::Concern => CONCERN_SUBSET_TARGETS,
        ElementKind::Actor | ElementKind::Stakeholder => ACTOR_SUBSET_TARGETS,
        ElementKind::Flow => FLOW_SUBSET_TARGETS,
        ElementKind::Allocation => ALLOCATION_SUBSET_TARGETS,
        ElementKind::Interface => INTERFACE_SUBSET_TARGETS,
        _ => &[],
    }
}

pub fn expected_typing_definition_label(usage_kind: &ElementKind) -> String {
    match usage_kind {
        ElementKind::Actor | ElementKind::Stakeholder => "part or item".to_string(),
        _ => usage_kind.as_str().trim_end_matches(" def").to_string(),
    }
}
