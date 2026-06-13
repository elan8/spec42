//! Canonical element-kind predicates and resolution allowlists for semantic_core.

use crate::SemanticNode;

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
    "actor_def",
    "occurrence_def",
    "interface",
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

pub const TYPING_TARGET_KINDS: &[&str] = &[
    "part def",
    "port def",
    "interface",
    "item def",
    "attribute def",
    "action def",
    "actor def",
    "occurrence def",
    "flow def",
    "allocation def",
    "state def",
    "requirement def",
    "requirement",
    "use case def",
    "concern def",
    "analysis def",
    "verification def",
    "view def",
    "viewpoint def",
    "rendering def",
    "metadata def",
    "enum def",
    "alias",
    "kermlDecl",
];

pub const SPECIALIZES_TARGET_KINDS: &[&str] = &[
    "part def",
    "port def",
    "interface",
    "item def",
    "attribute def",
    "action def",
    "actor def",
    "occurrence def",
    "flow def",
    "allocation def",
    "state def",
    "requirement def",
    "use case def",
    "concern def",
    "enum def",
    "alias",
    "kermlDecl",
    "individual def",
    "connection def",
    "metadata def",
    "constraint def",
    "calc def",
    "case def",
    "analysis def",
    "verification def",
    "view def",
    "viewpoint def",
    "rendering def",
];

/// Allowed resolved kinds for Rule 6 unresolved-type diagnostics (typing + definitional targets).
pub const RULE6_ALLOWED_KINDS: &[&str] = &[
    "part def",
    "port def",
    "interface",
    "item def",
    "attribute def",
    "action def",
    "actor def",
    "occurrence def",
    "flow def",
    "allocation def",
    "state def",
    "requirement def",
    "use case def",
    "concern def",
    "analysis def",
    "verification def",
    "enum def",
    "alias",
    "kermlDecl",
    "view def",
    "viewpoint def",
    "metadata def",
    "rendering def",
];

pub const SUBJECT_TYPE_TARGET_KINDS: &[&str] = &[
    "part def",
    "port def",
    "interface",
    "item def",
    "attribute def",
    "requirement def",
    "action def",
    "actor def",
    "occurrence def",
    "flow def",
    "allocation def",
    "state def",
    "use case def",
    "concern def",
    "analysis def",
];

pub const VERIFIED_REQUIREMENT_TARGET_KINDS: &[&str] = &["requirement def", "requirement"];

pub fn allowed_for_role(role: ResolutionRole) -> &'static [&'static str] {
    match role {
        ResolutionRole::Typing => TYPING_TARGET_KINDS,
        ResolutionRole::Specializes => SPECIALIZES_TARGET_KINDS,
        ResolutionRole::NameLookupRule6 => RULE6_ALLOWED_KINDS,
        ResolutionRole::Subject => SUBJECT_TYPE_TARGET_KINDS,
        ResolutionRole::VerifiedRequirement => VERIFIED_REQUIREMENT_TARGET_KINDS,
        ResolutionRole::AnnotatedElement => ANNOTATED_ELEMENT_TARGET_KINDS,
    }
}

pub const ANNOTATED_ELEMENT_TARGET_KINDS: &[&str] = &[
    "part def",
    "part",
    "port def",
    "port",
    "action def",
    "action",
    "state def",
    "state",
    "requirement def",
    "requirement",
    "use case def",
    "use case",
    "concern def",
    "concern",
    "item def",
    "item",
    "interface",
    "metadata def",
    "metadata usage",
    "constraint def",
    "constraint",
    "package",
];

pub fn element_kind_allowed(element_kind: &str, allowed_kinds: &[&str]) -> bool {
    allowed_kinds.contains(&element_kind)
}

pub fn is_namespace(element_kind: &str) -> bool {
    matches!(
        element_kind,
        "package"
            | "requirement def"
            | "requirement"
            | "use case def"
            | "use case"
            | "analysis def"
            | "analysis"
            | "verification def"
            | "verification"
            | "concern def"
            | "concern"
    )
}

pub fn is_part_like(element_kind: &str) -> bool {
    matches!(element_kind, "part" | "part def" | "item def" | "occurrence def")
        || element_kind.contains("part")
}

pub fn is_port_like(element_kind: &str) -> bool {
    element_kind.contains("port")
}

pub fn is_requirement(element_kind: &str) -> bool {
    matches!(element_kind, "requirement" | "requirement def")
}

pub fn is_metadata_restriction_attribute(node: &SemanticNode) -> bool {
    node.attributes.contains_key("subsetsFeature")
}

pub fn is_kerml_metadata_supertype(target: &SemanticNode) -> bool {
    target.element_kind == "kermlDecl" && target.name == "SemanticMetadata"
}

pub fn is_semantic_metadata_base_type_redefine(owner: &SemanticNode, node: &SemanticNode) -> bool {
    node.name == "baseType"
        && node
            .attributes
            .get("redefines")
            .and_then(|value| value.as_str())
            == Some("baseType")
        && owner.element_kind == "metadata def"
        && owner
            .attributes
            .get("specializes")
            .and_then(|value| value.as_str())
            .is_some_and(|value| value.contains("SemanticMetadata"))
}

pub fn is_compatible_kind(target_kind: &str, allowed: &[&str]) -> bool {
    allowed.contains(&target_kind)
}

pub fn is_compatible_specializes_target(def_kind: &str, target: &SemanticNode) -> bool {
    if is_compatible_kind(&target.element_kind, allowed_specializes_target_kinds(def_kind)) {
        return true;
    }
    def_kind == "metadata def" && is_kerml_metadata_supertype(target)
}

/// Per-usage typing compatibility (diagnostics layer).
pub fn allowed_typing_target_kinds(usage_kind: &str) -> &'static [&'static str] {
    match usage_kind {
        "part" => &["part def", "item def", "occurrence def"],
        "port" => &["port def"],
        "item" => &["item def", "part def"],
        "attribute" => &["attribute def", "enum def"],
        "action" => &["action def"],
        "state" => &["state def"],
        "requirement" => &["requirement def"],
        "use case" => &["use case def"],
        "analysis" => &["analysis def"],
        "verification" => &["verification def"],
        "view" => &["view def"],
        "viewpoint" => &["viewpoint def"],
        "concern" => &["concern def"],
        "actor" | "stakeholder" => &["part def", "item def", "occurrence def"],
        "flow" => &["flow def"],
        "allocation" => &["allocation def"],
        "interface" => &["interface"],
        "metadata usage" => &["metadata def"],
        "metadata keyword" => &["metadata def"],
        "rendering" => &["rendering def"],
        "view rendering" => &["rendering def", "rendering"],
        "perform" => &["action def", "action"],
        "subject" => SUBJECT_TYPE_TARGET_KINDS,
        "verified requirement" => VERIFIED_REQUIREMENT_TARGET_KINDS,
        "include use case" => &["use case def", "use case"],
        "ref" => &[
            "part def",
            "port def",
            "item def",
            "attribute def",
            "action def",
            "state def",
            "requirement def",
            "use case def",
            "analysis def",
            "verification def",
            "view def",
            "viewpoint def",
            "concern def",
            "flow def",
            "allocation def",
            "interface",
            "enum def",
            "occurrence def",
        ],
        _ => &[],
    }
}

pub fn allowed_specializes_target_kinds(def_kind: &str) -> &'static [&'static str] {
    match def_kind {
        "part def" => &["part def", "occurrence def"],
        "port def" => &["port def"],
        "item def" => &["item def"],
        "attribute def" => &["attribute def"],
        "action def" => &["action def"],
        "state def" => &["state def"],
        "requirement def" => &["requirement def"],
        "use case def" => &["use case def"],
        "analysis def" => &["analysis def"],
        "verification def" => &["verification def"],
        "view def" => &["view def"],
        "viewpoint def" => &["viewpoint def"],
        "concern def" => &["concern def"],
        "flow def" => &["flow def"],
        "allocation def" => &["allocation def"],
        "enum def" => &["enum def"],
        "metadata def" => &["metadata def"],
        "rendering def" => &["rendering def"],
        "interface" => &["interface"],
        _ => &[],
    }
}

const PART_SUBSET_TARGETS: &[&str] = &["part", "part def", "item def", "occurrence def"];
const PORT_SUBSET_TARGETS: &[&str] = &["port", "port def"];
const ITEM_SUBSET_TARGETS: &[&str] = &["item", "item def", "part def"];
const ATTRIBUTE_SUBSET_TARGETS: &[&str] = &["attribute", "attribute def", "enum def"];
const ACTION_SUBSET_TARGETS: &[&str] = &["action", "action def"];
const STATE_SUBSET_TARGETS: &[&str] = &["state", "state def"];
const REQUIREMENT_SUBSET_TARGETS: &[&str] = &["requirement", "requirement def"];
const USE_CASE_SUBSET_TARGETS: &[&str] = &["use case", "use case def"];
const ANALYSIS_SUBSET_TARGETS: &[&str] = &["analysis", "analysis def"];
const VERIFICATION_SUBSET_TARGETS: &[&str] = &["verification", "verification def"];
const VIEW_SUBSET_TARGETS: &[&str] = &["view", "view def"];
const VIEWPOINT_SUBSET_TARGETS: &[&str] = &["viewpoint", "viewpoint def"];
const CONCERN_SUBSET_TARGETS: &[&str] = &["concern", "concern def"];
const ACTOR_SUBSET_TARGETS: &[&str] = &["actor", "part", "part def", "item def", "occurrence def"];
const FLOW_SUBSET_TARGETS: &[&str] = &["flow", "flow def"];
const ALLOCATION_SUBSET_TARGETS: &[&str] = &["allocation", "allocation def"];
const INTERFACE_SUBSET_TARGETS: &[&str] = &["interface"];

pub fn allowed_subset_redefine_target_kinds(usage_kind: &str) -> &'static [&'static str] {
    match usage_kind {
        "part" => PART_SUBSET_TARGETS,
        "port" => PORT_SUBSET_TARGETS,
        "item" => ITEM_SUBSET_TARGETS,
        "attribute" => ATTRIBUTE_SUBSET_TARGETS,
        "action" => ACTION_SUBSET_TARGETS,
        "state" => STATE_SUBSET_TARGETS,
        "requirement" => REQUIREMENT_SUBSET_TARGETS,
        "use case" => USE_CASE_SUBSET_TARGETS,
        "analysis" => ANALYSIS_SUBSET_TARGETS,
        "verification" => VERIFICATION_SUBSET_TARGETS,
        "view" => VIEW_SUBSET_TARGETS,
        "viewpoint" => VIEWPOINT_SUBSET_TARGETS,
        "concern" => CONCERN_SUBSET_TARGETS,
        "actor" | "stakeholder" => ACTOR_SUBSET_TARGETS,
        "flow" => FLOW_SUBSET_TARGETS,
        "allocation" => ALLOCATION_SUBSET_TARGETS,
        "interface" => INTERFACE_SUBSET_TARGETS,
        _ => &[],
    }
}

pub fn expected_typing_definition_label(usage_kind: &str) -> String {
    match usage_kind {
        "actor" | "stakeholder" => "part or item".to_string(),
        _ => usage_kind.trim_end_matches(" def").to_string(),
    }
}
