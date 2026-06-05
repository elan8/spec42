//! Kind-compatibility tables for P1 typing, specialization, and redefinition checks.

pub fn allowed_typing_target_kinds(usage_kind: &str) -> &'static [&'static str] {
    match usage_kind {
        "part" => &["part def", "occurrence def"],
        "port" => &["port def"],
        "item" => &["item def"],
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
        "actor" => &["actor def"],
        "flow" => &["flow def"],
        "allocation" => &["allocation def"],
        "interface" => &["interface"],
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
            "actor def",
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
        "actor def" => &["actor def"],
        "flow def" => &["flow def"],
        "allocation def" => &["allocation def"],
        "enum def" => &["enum def"],
        "metadata def" => &["metadata def"],
        "interface" => &["interface"],
        _ => &[],
    }
}

pub fn allowed_subset_redefine_target_kinds(usage_kind: &str) -> &'static [&'static str] {
    match usage_kind {
        "part" | "port" | "item" | "attribute" | "action" | "state" | "requirement"
        | "use case" | "analysis" | "verification" | "view" | "viewpoint" | "concern" | "actor"
        | "flow" | "allocation" | "interface" => allowed_typing_target_kinds(usage_kind),
        _ => &[],
    }
}

pub fn is_compatible_kind(target_kind: &str, allowed: &[&str]) -> bool {
    allowed.contains(&target_kind)
}
