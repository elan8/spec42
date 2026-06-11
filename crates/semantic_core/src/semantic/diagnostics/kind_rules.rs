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

pub fn allowed_subset_redefine_target_kinds(usage_kind: &str) -> &'static [&'static str] {
    match usage_kind {
        "part" | "port" | "item" | "attribute" | "action" | "state" | "requirement"
        | "use case" | "analysis" | "verification" | "view" | "viewpoint" | "concern" | "actor"
        | "stakeholder" | "flow" | "allocation" | "interface" => {
            allowed_typing_target_kinds(usage_kind)
        }
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
