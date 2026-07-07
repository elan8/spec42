//! Canonical substring-based element-kind classifiers.
//!
//! These match loosely against a raw `element_type` string (or `ElementKind::as_str()`) rather
//! than exhaustively matching [`crate::ElementKind`] variants via [`crate::ElementKind::parse`],
//! because graph `element_type` strings aren't guaranteed to be one of `parse`'s fixed canonical
//! spellings (compound/varied phrasings show up across extraction, projection, and reference
//! resolution call sites). Previously reimplemented independently in `view_projection.rs`,
//! `model_projection.rs`, and `reference_resolution.rs`; consolidated here so all three (and any
//! future caller) agree on what counts as "part-like"/"port-like"/etc.

pub(crate) fn is_part_like(element_type: &str) -> bool {
    element_type.to_lowercase().contains("part")
}

pub(crate) fn is_action_like(element_type: &str) -> bool {
    element_type.to_lowercase().contains("action")
}

pub(crate) fn is_port_like(element_type: &str) -> bool {
    element_type.to_lowercase().contains("port")
}

pub(crate) fn is_attribute_like(element_type: &str) -> bool {
    let lower = element_type.to_lowercase();
    lower.contains("attribute") || lower.contains("property")
}

pub(crate) fn is_parameter_like(element_type: &str) -> bool {
    element_type.to_lowercase().contains("parameter")
}
