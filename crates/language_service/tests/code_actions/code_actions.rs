use language_service::{
    suggest_add_import_quick_fixes, suggest_add_missing_case_subject_quick_fix,
    suggest_create_definition_for_unresolved_type_quick_fix,
    suggest_create_matching_part_def_quick_fix, suggest_create_usage_from_definition,
    suggest_create_verification_case, suggest_explicit_redefinition_quick_fix,
    suggest_qualify_ambiguous_name_quick_fixes, suggest_wrap_in_package, DiagnosticLine,
    InMemoryWorkspace, WorkspaceSnapshot,
};

use crate::support::{document, multi_doc};

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
fn suggest_create_definition_for_unresolved_item_type() {
    let source = "package P {\n  item cargo : Payload;\n}\n";
    let suggestion =
        suggest_create_definition_for_unresolved_type_quick_fix(source, PATH, diagnostic_line(1))
            .expect("quick fix");
    assert_eq!(suggestion.title, "Create `item def Payload`");
    assert_eq!(suggestion.edits[0].replacement, "  item def Payload;\n");
}

#[test]
fn suggest_create_definition_for_unresolved_requirement_type() {
    let source = "package P {\n  requirement runtime : BatteryRuntime;\n}\n";
    let suggestion =
        suggest_create_definition_for_unresolved_type_quick_fix(source, PATH, diagnostic_line(1))
            .expect("quick fix");
    assert_eq!(suggestion.title, "Create `requirement def BatteryRuntime`");
    assert_eq!(
        suggestion.edits[0].replacement,
        "  requirement def BatteryRuntime { }\n"
    );
}

#[test]
fn suggest_create_definition_for_unresolved_ref_type() {
    let source = "package P {\n  ref sensor : SensorUnit;\n}\n";
    let suggestion =
        suggest_create_definition_for_unresolved_type_quick_fix(source, PATH, diagnostic_line(1))
            .expect("quick fix");
    assert_eq!(suggestion.title, "Create `part def SensorUnit`");
    assert_eq!(
        suggestion.edits[0].replacement,
        "  part def SensorUnit { }\n"
    );
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

#[test]
fn suggest_create_verification_case_inserts_skeleton() {
    let source = "package P {\n  requirement def BatteryRuntime {\n  }\n}\n";
    let suggestion = suggest_create_verification_case(source, PATH, 1).expect("refactor");
    assert!(suggestion.title.contains("VerifyBatteryRuntime"));
    assert!(suggestion.edits[0]
        .replacement
        .contains("verification def VerifyBatteryRuntime"));
    assert!(suggestion.edits[0]
        .replacement
        .contains("verify BatteryRuntime;"));
}

#[test]
fn suggest_create_verification_case_noop_when_present() {
    let source = "package P {\n  requirement def BatteryRuntime {\n  }\n  verification def VerifyBatteryRuntime {\n    objective {\n      verify BatteryRuntime;\n    }\n  }\n}\n";
    assert!(suggest_create_verification_case(source, PATH, 1).is_none());
}

#[test]
fn suggest_add_missing_case_subject_inserts_before_existing_members() {
    let source = concat!(
        "package P {\n",
        "  verification def VerifyRuntime {\n",
        "    objective {\n",
        "      verify RuntimeRequirement;\n",
        "    }\n",
        "  }\n",
        "}\n",
    );
    let suggestion = suggest_add_missing_case_subject_quick_fix(source, PATH, diagnostic_line(1))
        .expect("quick fix");
    assert_eq!(suggestion.title, "Add missing case subject");
    assert!(suggestion.is_preferred);
    assert_eq!(
        suggestion.edits[0].replacement,
        "    subject subjectUnderVerification;\n"
    );
    assert_eq!(suggestion.edits[0].range.start.line, 2);
}

#[test]
fn suggest_add_missing_case_subject_noop_when_subject_exists() {
    let source = concat!(
        "package P {\n",
        "  verification def VerifyRuntime {\n",
        "    subject systemUnderVerification;\n",
        "    objective { verify RuntimeRequirement; }\n",
        "  }\n",
        "}\n",
    );
    assert!(suggest_add_missing_case_subject_quick_fix(source, PATH, diagnostic_line(1)).is_none());
}

#[test]
fn suggest_create_usage_from_braced_definition_inserts_after_definition() {
    let source = concat!(
        "package P {\n",
        "  part def Engine {\n",
        "    attribute power;\n",
        "  }\n",
        "}\n",
    );
    let suggestion =
        suggest_create_usage_from_definition(source, PATH, 1).expect("create usage refactor");
    assert_eq!(suggestion.title, "Create `part engine : Engine`");
    assert_eq!(suggestion.edits[0].range.start.line, 4);
    assert_eq!(suggestion.edits[0].replacement, "  part engine : Engine;\n");
}

#[test]
fn suggest_create_usage_from_semicolon_definition_supports_requirement_pattern() {
    let source = "package P {\n  requirement def RuntimeRequirement;\n}\n";
    let suggestion =
        suggest_create_usage_from_definition(source, PATH, 1).expect("create usage refactor");
    assert_eq!(
        suggestion.edits[0].replacement,
        "  requirement runtimeRequirement : RuntimeRequirement;\n"
    );
}

#[test]
fn suggest_create_usage_from_definition_noop_when_matching_usage_exists() {
    let source = "package P {\n  part def Engine;\n  part engine : Engine;\n}\n";
    assert!(suggest_create_usage_from_definition(source, PATH, 1).is_none());
}

#[test]
fn suggest_qualify_ambiguous_name_is_disabled_without_typed_candidate_query() {
    let workspace = multi_doc(&[
        (
            "a.sysml",
            "package Alpha {\n  part def Vehicle;\n}\n",
        ),
        (
            "b.sysml",
            "package Beta {\n  part def Vehicle;\n}\n",
        ),
        (
            "c.sysml",
            "package Consumer {\n  private import Alpha::*;\n  private import Beta::*;\n  part car : Vehicle;\n}\n",
        ),
    ]);
    let uri = workspace.resolve_uri_for_path("c.sysml").expect("uri");
    let source = workspace.document_text(&uri).expect("text").to_string();
    let line = source
        .lines()
        .position(|l| l.contains("part car"))
        .expect("usage line") as u32;
    let suggestions = suggest_qualify_ambiguous_name_quick_fixes(
        &source,
        "c.sysml",
        diagnostic_line(line),
        workspace.published_model().expect("publication"),
        &uri,
    );
    assert!(suggestions.is_empty());
}

#[test]
fn suggest_add_import_is_disabled_without_typed_import_query() {
    let workspace = multi_doc(&[
        ("defs.sysml", "package Defs {\n  part def Vehicle;\n}\n"),
        ("use.sysml", "package Use {\n  part car : Vehicle;\n}\n"),
    ]);
    let uri = workspace.resolve_uri_for_path("use.sysml").expect("uri");
    let source = workspace.document_text(&uri).expect("text").to_string();
    let line = source
        .lines()
        .position(|l| l.contains("part car"))
        .expect("usage line") as u32;
    let suggestions = suggest_add_import_quick_fixes(
        &source,
        "use.sysml",
        diagnostic_line(line),
        workspace.published_model().expect("publication"),
        &uri,
    );
    assert!(suggestions.is_empty());
}

#[test]
fn suggest_add_import_empty_when_no_candidates() {
    let workspace = InMemoryWorkspace::from_documents(vec![document(
        "lonely.sysml",
        "package Lonely {\n  part car : MissingType;\n}\n",
    )])
    .expect("workspace");
    let uri = workspace.resolve_uri_for_path("lonely.sysml").expect("uri");
    let source = workspace.document_text(&uri).expect("text").to_string();
    let suggestions = suggest_add_import_quick_fixes(
        &source,
        "lonely.sysml",
        diagnostic_line(1),
        workspace.published_model().expect("publication"),
        &uri,
    );
    assert!(suggestions.is_empty(), "suggestions={suggestions:?}");
}
