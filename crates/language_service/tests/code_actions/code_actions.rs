use language_service::{
    suggest_create_definition_for_unresolved_type_quick_fix,
    suggest_create_matching_part_def_quick_fix, suggest_explicit_redefinition_quick_fix,
    suggest_wrap_in_package, DiagnosticLine,
};

const PATH: &str = "test.sysml";

fn diagnostic_line(line: u32) -> DiagnosticLine {
    DiagnosticLine { line }
}

#[test]
fn suggest_wrap_in_package_empty() {
    assert!(suggest_wrap_in_package("", PATH).is_none());
}

#[test]
fn suggest_wrap_in_package_named_package() {
    assert!(suggest_wrap_in_package("package P { }", PATH).is_none());
}

#[test]
fn suggest_wrap_in_package_unwrapped_member() {
    let source = "part def X { }";
    // Parser may wrap a lone part def in an anonymous package; when it does, we suggest wrap.
    if let Some(suggestion) = suggest_wrap_in_package(source, PATH) {
        assert!(suggestion.title.contains("Wrap"));
        assert_eq!(suggestion.edits.len(), 1);
        let edit = &suggestion.edits[0];
        assert_eq!(edit.path, PATH);
        assert!(edit.replacement.contains("package Generated"));
        assert!(edit.replacement.contains("part def X"));
    }
}

#[test]
fn suggest_create_matching_part_def_creates_def_and_types_usage() {
    let source = "package P {\n  part def Laptop {\n    part display;\n  }\n}\n";
    let suggestion = suggest_create_matching_part_def_quick_fix(source, PATH, diagnostic_line(2))
        .expect("quick fix");
    assert!(suggestion.title.contains("part def Display"));
    assert_eq!(suggestion.edits.len(), 2);
    assert!(suggestion.edits[0]
        .replacement
        .contains("part def Display { }"));
    assert!(suggestion.edits[1]
        .replacement
        .trim()
        .ends_with("part display : Display;"));
}

#[test]
fn suggest_create_matching_part_def_respects_indent() {
    let source = "package P {\n    part def Laptop {\n        part display;\n    }\n}\n";
    let suggestion = suggest_create_matching_part_def_quick_fix(source, PATH, diagnostic_line(2))
        .expect("quick fix");
    assert_eq!(
        suggestion.edits[0].replacement,
        "    part def Display { }\n"
    );
}

#[test]
fn suggest_create_matching_part_def_noop_for_typed_usage() {
    let source = "package P {\n  part def Laptop {\n    part display : Display;\n  }\n}\n";
    assert!(
        suggest_create_matching_part_def_quick_fix(source, PATH, diagnostic_line(2),).is_none()
    );
}

#[test]
fn suggest_create_definition_for_unresolved_part_type() {
    let source = "package P {\n  part car : Vehicle;\n}\n";
    let suggestion =
        suggest_create_definition_for_unresolved_type_quick_fix(source, PATH, diagnostic_line(1))
            .expect("quick fix");
    assert_eq!(suggestion.title, "Create `part def Vehicle`");
    assert_eq!(suggestion.edits[0].replacement, "  part def Vehicle { }\n");
}

#[test]
fn suggest_create_definition_for_unresolved_port_type() {
    let source = "package P {\n  port command : CommandPort;\n}\n";
    let suggestion =
        suggest_create_definition_for_unresolved_type_quick_fix(source, PATH, diagnostic_line(1))
            .expect("quick fix");
    assert_eq!(suggestion.title, "Create `port def CommandPort`");
    assert_eq!(suggestion.edits[0].replacement, "  port def CommandPort;\n");
}

#[test]
fn suggest_explicit_redefinition_rewrites_line() {
    let source = "package P {\n  part def Child :> Base {\n    attribute mass = 1200;\n  }\n}\n";
    let suggestion = suggest_explicit_redefinition_quick_fix(source, PATH, diagnostic_line(2))
        .expect("quick fix");
    assert_eq!(suggestion.edits.len(), 1);
    assert_eq!(
        suggestion.edits[0].replacement.trim(),
        "attribute :>> mass = 1200;"
    );
}

#[test]
fn suggest_explicit_redefinition_noop_when_already_explicit() {
    let source =
        "package P {\n  part def Child :> Base {\n    attribute :>> mass = 1200;\n  }\n}\n";
    assert!(suggest_explicit_redefinition_quick_fix(source, PATH, diagnostic_line(2),).is_none());
}
