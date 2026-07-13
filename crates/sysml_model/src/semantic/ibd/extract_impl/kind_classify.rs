use super::*;

pub(crate) fn is_part_like(kind: &str) -> bool {
    let k = kind.to_lowercase();
    k.contains("part def") || k == "part" || (k.contains("part") && !k.contains("def"))
}

/// BNF interconnection-element: part usage or part-ref (not definitions).
pub(crate) fn is_interconnection_element_kind(kind: &str) -> bool {
    is_part_like(kind) || is_reference_element_kind(kind)
}

pub(crate) fn is_part_instance_kind(kind: &str) -> bool {
    let k = kind.to_lowercase();
    k == "part" || k.contains("part usage")
}

/// True when the element kind is a SysML definition (not valid as interconnection-element).
pub(crate) fn is_definition_element_kind(kind: &str) -> bool {
    ElementKind::parse(kind.trim()).is_definition()
}

pub(crate) fn is_reference_element_kind(kind: &str) -> bool {
    let k = kind.trim().to_lowercase();
    if k == "ref" {
        return true;
    }
    if k.ends_with("-ref") || k.ends_with(" ref") {
        return true;
    }
    k.split_whitespace().any(|token| token == "ref")
}

/// Normalize graph element_kind to interconnection-view `type` for the diagram renderer.
pub(crate) fn normalize_ibd_element_type(kind: &str) -> String {
    if is_reference_element_kind(kind) {
        return "ref".to_string();
    }
    if is_part_instance_kind(kind) {
        return "part".to_string();
    }
    kind.trim().to_string()
}

pub(crate) fn decorate_ibd_part_attributes(
    element_type: &str,
    attributes: &mut std::collections::HashMap<String, serde_json::Value>,
) {
    attributes.insert(
        "isReference".to_string(),
        serde_json::json!(is_reference_element_kind(element_type)),
    );
    attributes.insert(
        "isDefinition".to_string(),
        serde_json::json!(is_definition_element_kind(element_type)),
    );
}
