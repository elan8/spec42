//! Normative SysML v2 standard view types (§9.2.20, Table 34).
//!
//! Spec42 implements exactly these eight view definitions — one renderer each — and
//! does not treat legacy or vendor-specific names (RequirementView, CaseView, …) as standard.

/// OMG standard view type name paired with Spec42 renderer id.
pub const STANDARD_VIEW_TYPES: &[(&str, &str)] = &[
    ("ActionFlowView", "action-flow-view"),
    ("BrowserView", "browser-view"),
    ("GeneralView", "general-view"),
    ("GeometryView", "geometry-view"),
    ("GridView", "grid-view"),
    ("InterconnectionView", "interconnection-view"),
    ("SequenceView", "sequence-view"),
    ("StateTransitionView", "state-transition-view"),
];

pub fn normalize_view_type_name(view_type: &str) -> String {
    view_type
        .split("::")
        .last()
        .unwrap_or(view_type)
        .replace([' ', '_'], "")
        .to_lowercase()
}

pub fn is_standard_view_type(view_type: &str) -> bool {
    is_standard_view_type_normalized(&normalize_view_type_name(view_type))
}

pub fn is_standard_view_type_normalized(normalized: &str) -> bool {
    STANDARD_VIEW_TYPES
        .iter()
        .any(|(name, _)| normalize_view_type_name(name) == normalized)
}

pub fn renderer_for_standard_view_type(view_type: &str) -> Option<&'static str> {
    let normalized = normalize_view_type_name(view_type);
    STANDARD_VIEW_TYPES
        .iter()
        .find(|(name, _)| normalize_view_type_name(name) == normalized)
        .map(|(_, renderer)| *renderer)
}

/// Explicit `: SomeView` on a usage that is not a local `view def` and not a standard type.
pub fn is_non_standard_explicit_view_type(type_ref: &str) -> bool {
    let normalized = normalize_view_type_name(type_ref);
    if is_standard_view_type_normalized(&normalized) {
        return false;
    }
    normalized.ends_with("view")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_exactly_eight_standard_view_types() {
        assert_eq!(STANDARD_VIEW_TYPES.len(), 8);
        let renderers: Vec<_> = STANDARD_VIEW_TYPES.iter().map(|(_, r)| *r).collect();
        assert_eq!(renderers.len(), renderers.iter().collect::<std::collections::HashSet<_>>().len());
    }

    #[test]
    fn rejects_legacy_non_standard_view_type_names() {
        assert!(!is_standard_view_type("RequirementView"));
        assert!(!is_standard_view_type("CaseView"));
        assert!(!is_standard_view_type("StructureView"));
        assert!(is_non_standard_explicit_view_type("RequirementView"));
    }

    #[test]
    fn accepts_all_table_34_view_types() {
        for (view_type, renderer) in STANDARD_VIEW_TYPES {
            assert!(is_standard_view_type(view_type));
            assert_eq!(renderer_for_standard_view_type(view_type), Some(*renderer));
        }
    }
}
