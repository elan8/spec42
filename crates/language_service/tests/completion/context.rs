use language_service::{
    completion_edit_shape, detect_completion_context, CompletionContext,
    ATTRIBUTE_TYPE_LOOKUP_KINDS, PART_TYPE_LOOKUP_KINDS, PORT_TYPE_LOOKUP_KINDS,
};
use sysml_model::TextPosition;

#[test]
fn detects_part_type_reference_context() {
    let context = detect_completion_context("    part laptop: La");
    assert_eq!(
        context,
        CompletionContext::TypeReference {
            prefix: "La".to_string(),
            qualifier: None,
            expected_kinds: PART_TYPE_LOOKUP_KINDS,
        }
    );
}

#[test]
fn detects_port_type_reference_context() {
    let context = detect_completion_context("    port control: C");
    assert_eq!(
        context,
        CompletionContext::TypeReference {
            prefix: "C".to_string(),
            qualifier: None,
            expected_kinds: PORT_TYPE_LOOKUP_KINDS,
        }
    );
}

#[test]
fn detects_attribute_type_reference_context() {
    let context = detect_completion_context("    attribute mass: M");
    assert_eq!(
        context,
        CompletionContext::TypeReference {
            prefix: "M".to_string(),
            qualifier: None,
            expected_kinds: ATTRIBUTE_TYPE_LOOKUP_KINDS,
        }
    );
}

#[test]
fn detects_qualified_reference_context() {
    let context = detect_completion_context("    part laptop: Pkg::La");
    assert_eq!(
        context,
        CompletionContext::TypeReference {
            prefix: "La".to_string(),
            qualifier: Some("Pkg".to_string()),
            expected_kinds: PART_TYPE_LOOKUP_KINDS,
        }
    );
}

#[test]
fn detects_member_reference_context() {
    let context = detect_completion_context("    vehicle.eng");
    assert_eq!(
        context,
        CompletionContext::MemberReference {
            prefix: "eng".to_string(),
            receiver: "vehicle".to_string(),
        }
    );
}

#[test]
fn detects_declaration_modifier_context() {
    let context = detect_completion_context("    part ");
    assert_eq!(
        context,
        CompletionContext::DeclarationModifier {
            prefix: String::new(),
            keyword: "part".to_string(),
        }
    );
}

#[test]
fn does_not_treat_comments_as_type_context() {
    let context = detect_completion_context("// part laptop: La");
    assert_eq!(
        context,
        CompletionContext::General {
            prefix: "La".to_string(),
        }
    );
}

#[test]
fn detects_top_level_keyword_context() {
    let context = detect_completion_context("    pa");
    assert_eq!(
        context,
        CompletionContext::TopLevelKeyword {
            prefix: "pa".to_string(),
        }
    );
}

#[test]
fn replacement_range_uses_only_member_prefix() {
    let shape = completion_edit_shape(TextPosition::new(0, 11), "eng");
    assert_eq!(shape.replace_range.start, TextPosition::new(0, 8));
    assert_eq!(shape.replace_range.end, TextPosition::new(0, 11));
}

#[test]
fn replacement_range_uses_only_qualified_suffix() {
    let shape = completion_edit_shape(TextPosition::new(0, 7), "Fo");
    assert_eq!(shape.replace_range.start, TextPosition::new(0, 5));
    assert_eq!(shape.replace_range.end, TextPosition::new(0, 7));
}
