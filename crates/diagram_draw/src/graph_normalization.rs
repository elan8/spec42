/// Port of `normalizeEdgeKind` in `graph-normalization.ts`.
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
    if kind.contains("binding-connection")
        || kind.contains("binding connection")
        || kind.contains("binding-connector")
        || kind.contains("binding connector")
    {
        return "bind".to_string();
    }
    if kind.contains("connection") || kind == "connect" || kind.contains("connector") {
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
    if kind == "typing" || kind == "defined_by" || kind == "defined by" || kind == "definition" {
        return "typing".to_string();
    }
    if kind == "dependency" || kind.contains("depend") || kind.contains("binary-dependency") {
        return "dependency".to_string();
    }
    if kind == "usage" || kind == "usage-relationship" {
        return "usage".to_string();
    }
    if kind.contains("redefin") {
        return "redefinition".to_string();
    }
    if kind == "specializes" || kind == "specialization" {
        return "specializes".to_string();
    }
    if kind == "bind" || kind == "binding" {
        return "bind".to_string();
    }
    if kind == "allocate" || kind == "allocation" {
        return "allocate".to_string();
    }
    if kind == "transition" {
        return "transition".to_string();
    }
    if kind == "composition" {
        return "composition".to_string();
    }
    if kind == "hierarchy"
        || kind == "contains"
        || kind == "owns"
        || kind == "ownership"
        || kind == "containment"
    {
        return "hierarchy".to_string();
    }
    // `type.replace(/[^a-z0-9_-]+/g, "_")`: collapse each maximal run of disallowed characters
    // into a single underscore (not per-character, and no trimming of leading/trailing runs).
    let mut sanitized = String::new();
    let mut in_run = false;
    for c in kind.chars() {
        if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
            sanitized.push(c);
            in_run = false;
        } else if !in_run {
            sanitized.push('_');
            in_run = true;
        }
    }
    if sanitized.is_empty() {
        "relationship".to_string()
    } else {
        sanitized
    }
}
