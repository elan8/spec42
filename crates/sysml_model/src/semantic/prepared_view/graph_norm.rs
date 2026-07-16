//! Graph normalization helpers aligned with shared/diagram-renderer graph-normalization.ts.

pub fn normalize_edge_kind(relationship_type: &str) -> String {
    let kind = relationship_type.trim().to_lowercase();
    if kind.is_empty() {
        return "relationship".to_string();
    }
    if kind.contains("item_flow")
        || kind.contains("item flow")
        || kind == "flow"
        || kind.contains("flow")
    {
        return "flow".to_string();
    }
    if kind.contains("interface-connection")
        || kind.contains("interface connection")
        || kind.contains("interface")
    {
        return "interface".to_string();
    }
    if kind.contains("binding-connection") || kind.contains("binding connection") {
        return "bind".to_string();
    }
    if kind.contains("connection") || kind == "connect" {
        return "connection".to_string();
    }
    if kind.contains("reference") || kind == "ref" {
        return "reference".to_string();
    }
    if kind.contains("satisfy") {
        return "satisfy".to_string();
    }
    if kind.contains("verify") {
        return "verify".to_string();
    }
    if kind.contains("derivation") || kind.contains("derive") {
        return "derivation".to_string();
    }
    if matches!(
        kind.as_str(),
        "typing" | "defined_by" | "defined by" | "definition"
    ) {
        return "typing".to_string();
    }
    if kind == "dependency" || kind.contains("depend") || kind.contains("binary-dependency") {
        return "dependency".to_string();
    }
    if matches!(kind.as_str(), "usage" | "usage-relationship") {
        return "usage".to_string();
    }
    if kind.contains("redefin") {
        return "redefinition".to_string();
    }
    if matches!(kind.as_str(), "specializes" | "specialization") {
        return "specializes".to_string();
    }
    if matches!(kind.as_str(), "bind" | "binding") {
        return "bind".to_string();
    }
    if matches!(kind.as_str(), "allocate" | "allocation") {
        return "allocate".to_string();
    }
    if kind == "transition" {
        return "transition".to_string();
    }
    if kind == "composition" {
        return "composition".to_string();
    }
    if matches!(
        kind.as_str(),
        "hierarchy" | "contains" | "owns" | "ownership" | "containment"
    ) {
        return "hierarchy".to_string();
    }
    let normalized: String = kind
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect();
    if normalized.is_empty() {
        "relationship".to_string()
    } else {
        normalized
    }
}

fn is_package_element_type(element_type: &str) -> bool {
    let normalized = element_type.trim().to_lowercase();
    normalized.is_empty()
        || normalized == "package"
        || normalized == "library package"
        || normalized.ends_with("_package")
        || normalized.contains("package_def")
}

fn is_non_diagram_semantic_element_type(element_type: &str) -> bool {
    let normalized = element_type.trim().to_lowercase();
    normalized.is_empty()
        || normalized == "import"
        || normalized == "diagnostic"
        || normalized.contains("diagnostic")
}

pub fn is_overview_visual_element_type(element_type: &str) -> bool {
    !is_package_element_type(element_type) && !is_non_diagram_semantic_element_type(element_type)
}

pub fn is_definition_kind(kind: &str) -> bool {
    crate::ElementKind::parse(kind.trim()).is_definition()
}

pub fn is_reference_kind(kind: &str) -> bool {
    let k = kind.trim().to_lowercase();
    if k == "ref" {
        return true;
    }
    if k.ends_with("-ref") || k.ends_with(" ref") {
        return true;
    }
    k.contains("ref") && !k.contains("refine")
}
