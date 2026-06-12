//! Kind-compatibility tables for P1 typing, specialization, and redefinition checks.

/// Part-like usage kinds (ActorUsage / StakeholderUsage are PartUsages per SysML §7.21–7.22).
const PART_LIKE_TYPING_TARGETS: &[&str] = &["part def", "item def", "occurrence def"];

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
        "actor" | "stakeholder" => PART_LIKE_TYPING_TARGETS,
        "flow" => &["flow def"],
        "allocation" => &["allocation def"],
        "interface" => &["interface"],
        "metadata usage" => &["metadata def"],
        "metadata keyword" => &["metadata def"],
        "rendering" => &["rendering def"],
        "view rendering" => &["rendering def", "rendering"],
        "perform" => &["action def", "action"],
        "subject" => &[
            "part def",
            "port def",
            "interface",
            "item def",
            "attribute def",
            "requirement def",
            "action def",
            "occurrence def",
            "flow def",
            "allocation def",
            "state def",
            "use case def",
            "concern def",
        ],
        "verified requirement" => &["requirement def", "requirement"],
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

/// Subsetting/redefinition resolves to inherited *features* (usage kinds) as well as typed definitions.
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

pub fn is_compatible_kind(target_kind: &str, allowed: &[&str]) -> bool {
    allowed.contains(&target_kind)
}

pub fn expected_typing_definition_label(usage_kind: &str) -> String {
    match usage_kind {
        "actor" | "stakeholder" => "part or item".to_string(),
        _ => usage_kind.trim_end_matches(" def").to_string(),
    }
}
