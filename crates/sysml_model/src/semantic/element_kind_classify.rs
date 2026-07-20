//! Substring-based element-kind classifiers for kinds not yet covered by
//! [`crate::semantic::kinds`]'s enum-based predicates.
//!
//! `is_part_like`/`is_port_like` used to live here too (substring-matched against a raw
//! `element_type` string / `ElementKind::as_str()`); they were consolidated into
//! [`crate::semantic::kinds::is_part_like`]/[`crate::semantic::kinds::is_port_like`] (plus the
//! `_str` variants for DTO callers) since every `element_type` string in this crate already
//! originates from `ElementKind::as_str()`, so the enum match is exact and no longer needs a
//! substring fallback except for genuinely unrecognized (`Unknown`) kinds — which the enum
//! predicates still handle. See `kinds.rs`'s `part_port_classification_tests` for the resolved
//! ItemDef/OccurrenceDef/ConjugatedPortDefinition edge cases that previously diverged here.

pub(crate) fn is_action_like(element_type: &str) -> bool {
    element_type.to_lowercase().contains("action")
}

pub(crate) fn is_attribute_like(element_type: &str) -> bool {
    let lower = element_type.to_lowercase();
    lower.contains("attribute") || lower.contains("property")
}

pub(crate) fn is_parameter_like(element_type: &str) -> bool {
    element_type.to_lowercase().contains("parameter")
}
