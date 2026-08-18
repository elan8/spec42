use language_service::{hover, WorkspaceSnapshot};

use super::support::{multi_doc, position_for, position_for_within, single_doc};

#[test]
fn hover_inspects_a_declared_element_from_the_publication() {
    let content = "package P { part def X; }";
    let workspace = single_doc("test.sysml", content);
    let position = position_for_within(content, "part def X", "X");
    let result = hover(&workspace, "test.sysml", position).expect("hover");
    assert!(result.contents.contains("part definition"));
    assert!(result.contents.contains("P::X"));
}

#[test]
fn hover_inspects_the_typed_reference_target() {
    let content = "package P { part def Vehicle; part car : Vehicle; }";
    let workspace = single_doc("test.sysml", content);
    let position = position_for_within(content, ": Vehicle", "Vehicle");
    let result = hover(&workspace, "test.sysml", position).expect("hover");
    assert!(result.contents.contains("part definition"));
    assert!(result.contents.contains("P::Vehicle"));
}

#[test]
fn hover_reports_cross_document_target_location() {
    let workspace = multi_doc(&[
        ("defs.sysml", "package Defs { part def Vehicle; }"),
        (
            "use.sysml",
            "package Use { private import Defs::*; part car : Vehicle; }",
        ),
    ]);
    let content = workspace
        .document_text(&workspace.resolve_uri_for_path("use.sysml").expect("uri"))
        .expect("text");
    let position = position_for_within(content, ": Vehicle", "Vehicle");
    let result = hover(&workspace, "use.sysml", position).expect("hover");
    assert!(result.contents.contains("Defined in:"));
    assert!(result.contents.contains("defs.sysml"));
}

#[test]
fn hover_on_keyword_prefers_the_containing_typed_element() {
    let content = "package P;";
    let workspace = single_doc("test.sysml", content);
    let position = position_for(content, "package");
    let result = hover(&workspace, "test.sysml", position).expect("hover");
    assert!(result.contents.contains("package"));
}
