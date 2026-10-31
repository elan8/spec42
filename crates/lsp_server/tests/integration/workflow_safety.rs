//! End-to-end editor safety contracts: apply edits as an editor would and retain versions.
use super::harness::TestSession;
use serde_json::json;
use tower_lsp::lsp_types::{DocumentSymbol, Position, Range, SelectionRange, TextEdit};

fn column(text: &str, needle: &str) -> u32 {
    text[..text.find(needle).unwrap()].encode_utf16().count() as u32
}
fn apply(text: &str, edits: &[TextEdit]) -> String {
    let mut edits: Vec<_> = edits
        .iter()
        .map(|edit| {
            let offset = |position: Position| {
                language_service::text::position_to_byte_offset(
                    text,
                    position.line,
                    position.character,
                )
                .expect("edit addresses a character boundary")
            };
            (
                offset(edit.range.start)..offset(edit.range.end),
                &edit.new_text,
            )
        })
        .collect();
    edits.sort_by_key(|(range, _)| std::cmp::Reverse(range.start));
    let mut result = text.to_owned();
    for (range, replacement) in edits {
        result.replace_range(range, replacement);
    }
    result
}
fn request_at(
    session: &mut TestSession,
    method: &str,
    uri: &str,
    character: u32,
) -> serde_json::Value {
    session.request(
        method,
        json!({"textDocument":{"uri":uri},"position":{"line":0,"character":character}}),
    )
}

#[test]
fn formatting_is_idempotent_when_the_editor_applies_the_edits() {
    let mut session = TestSession::new();
    session.initialize_default("formatting_safety");
    let uri = "file:///c:/spec42-lsp-tests/formatting-safety.sysml";
    for (index, source) in [
        "package P {part def Motor;}",
        "package P {\n    part def Motor;\n}\n",
        "package P { /* é😀 */ part def Motor; }",
    ]
    .iter()
    .enumerate()
    {
        session.did_open(uri, source, index as i32 * 2 + 1);
        session.barrier();
        let format = |session: &mut TestSession| {
            session.request(
                "textDocument/formatting",
                json!({"textDocument":{"uri":uri},"options":{"tabSize":4,"insertSpaces":true}}),
            )
        };
        let response = format(&mut session);
        let edits: Vec<TextEdit> = serde_json::from_value(response["result"].clone()).unwrap();
        let formatted = apply(source, &edits);
        session.did_change_full(uri, &formatted, index as i32 * 2 + 2);
        session.barrier();
        assert_eq!(format(&mut session)["result"], json!([]));
    }
}

#[test]
fn unicode_navigation_rename_and_hover_use_editor_coordinates() {
    let mut session = TestSession::new();
    session.initialize_default("unicode_safety");
    let uri = "file:///c:/spec42-lsp-tests/unicode-safety.sysml";
    let source = "package P { /* é😀 Motor */ part def Motor; part vehicle : Motor; }";
    session.did_open(uri, source, 1);
    session.barrier();
    let target = column(source, "Motor;");
    let reference = source[..source.rfind("Motor").unwrap()]
        .encode_utf16()
        .count() as u32;
    let definition = request_at(&mut session, "textDocument/definition", uri, reference);
    assert_eq!(
        definition["result"]["range"]["start"],
        json!({"line":0,"character":target})
    );
    let hover = request_at(&mut session, "textDocument/hover", uri, reference);
    assert_eq!(hover["result"]["range"]["start"]["character"], reference);
    assert!(request_at(
        &mut session,
        "textDocument/hover",
        uri,
        column(source, "Motor */")
    )["result"]
        .is_null());
    let rename = session.request("textDocument/rename", json!({"textDocument":{"uri":uri},"position":{"line":0,"character":reference},"newName":"Engine"}));
    let edits: Vec<TextEdit> =
        serde_json::from_value(rename["result"]["changes"][uri].clone()).unwrap();
    assert_eq!(
        apply(source, &edits),
        "package P { /* é😀 Motor */ part def Engine; part vehicle : Engine; }"
    );
    let changed = source.replacen("é😀", "😀éé", 1);
    session.did_change_full(uri, &changed, 2);
    session.barrier();
    let reference = changed[..changed.rfind("Motor").unwrap()]
        .encode_utf16()
        .count() as u32;
    let definition = request_at(&mut session, "textDocument/definition", uri, reference);
    assert_eq!(
        definition["result"]["range"]["start"],
        json!({"line":0,"character":column(&changed, "Motor;")})
    );
}

#[test]
fn rename_carries_each_open_documents_version() {
    let mut session = TestSession::new();
    session.request("initialize", json!({"processId":null,"rootUri":null,"capabilities":{"workspace":{"workspaceEdit":{"documentChanges":true}}}}));
    session.notify("initialized", json!({}));
    let definitions = "file:///c:/spec42-lsp-tests/rename-types.sysml";
    let uses = "file:///c:/spec42-lsp-tests/rename-uses.sysml";
    session.did_open(definitions, "package Types { part def Motor; }", 3);
    session.did_open(uses, "package Uses { part vehicle : Types::Motor; }", 7);
    session.barrier();
    let response = session.request("textDocument/rename", json!({"textDocument":{"uri":definitions},"position":{"line":0,"character":25},"newName":"Engine"}));
    assert!(response["result"]["changes"].is_null(), "{response}");
    let documents = response["result"]["documentChanges"]
        .as_array()
        .expect("versioned edits");
    assert_eq!(documents.len(), 2);
    for (uri, version) in [(definitions, 3), (uses, 7)] {
        let edit = documents
            .iter()
            .find(|edit| edit["textDocument"]["uri"] == uri)
            .unwrap();
        assert_eq!(edit["textDocument"]["version"], version);
    }
}

#[test]
fn selection_parents_contain_the_child_and_the_document_end() {
    let mut session = TestSession::new();
    session.initialize_default("selection_safety");
    let uri = "file:///c:/spec42-lsp-tests/selection-safety.sysml";
    let source = "package P { /* 😀 */ part vehicle; }";
    session.did_open(uri, source, 1);
    session.barrier();
    let response = session.request("textDocument/selectionRange", json!({"textDocument":{"uri":uri},"positions":[{"line":0,"character":column(source,"vehicle")}]}));
    let selections: Vec<SelectionRange> =
        serde_json::from_value(response["result"].clone()).unwrap();
    let mut current = &selections[0];
    while let Some(parent) = &current.parent {
        assert!(parent.range.start <= current.range.start && parent.range.end >= current.range.end);
        assert_ne!(parent.range, current.range);
        current = parent;
    }
    assert_eq!(
        current.range,
        Range::new(
            Position::new(0, 0),
            Position::new(0, source.encode_utf16().count() as u32)
        )
    );
}

#[test]
fn hierarchy_uses_the_referenced_type_and_rejects_stale_items() {
    let mut session = TestSession::new();
    session.initialize_default("hierarchy_safety");
    let uri = "file:///c:/spec42-lsp-tests/hierarchy-safety.sysml";
    let source = "package P { part def Motor; part vehicle : Motor; }";
    session.did_open(
        "file:///c:/spec42-lsp-tests/unrelated-recovery.sysml",
        "package Broken { @@@",
        1,
    );
    session.did_open(uri, source, 1);
    session.barrier();
    let reference = source.rfind("Motor").unwrap() as u32;
    let response = request_at(
        &mut session,
        "textDocument/prepareTypeHierarchy",
        uri,
        reference,
    );
    let item = response["result"][0].clone();
    assert_eq!(item["name"], "Motor");
    assert_eq!(
        item["selectionRange"]["start"]["character"],
        column(source, "Motor;")
    );
    session.did_change_full(uri, &format!("\n{source}"), 2);
    session.barrier();
    let stale = session.request_once("typeHierarchy/subtypes", json!({"item":item}));
    assert_eq!(stale["error"]["code"], -32801);
}

#[test]
fn qualified_import_links_and_outline_names_have_precise_ranges() {
    let mut session = TestSession::new();
    session.initialize_default("outline_links_safety");
    let uri = "file:///c:/spec42-lsp-tests/outline-links-safety.sysml";
    let source = "package A { package B { part def Motor; } }\npackage P { private import A::B::*; enum def Mode { idle; } state def Running; calc def Compute; constraint def Valid; state running; calc compute; constraint valid; }";
    session.did_open(uri, source, 1);
    session.barrier();
    let response = session.request(
        "textDocument/documentLink",
        json!({"textDocument":{"uri":uri}}),
    );
    assert_eq!(response["result"][0]["target"], uri, "{response}");
    assert_eq!(
        response["result"][0]["range"]["start"],
        json!({"line":1,"character":27})
    );
    let response = session.request(
        "textDocument/documentSymbol",
        json!({"textDocument":{"uri":uri}}),
    );
    let symbols: Vec<DocumentSymbol> = serde_json::from_value(response["result"].clone()).unwrap();
    let children = &symbols[1].children.as_ref().unwrap();
    assert_eq!(
        children
            .iter()
            .map(|item| item.name.as_str())
            .collect::<Vec<_>>(),
        vec!["Mode", "Running", "Compute", "Valid", "running", "compute", "valid"]
    );
    for item in children.iter() {
        let range = item.selection_range;
        let start = language_service::text::position_to_byte_offset(
            source,
            range.start.line,
            range.start.character,
        )
        .unwrap();
        let end = language_service::text::position_to_byte_offset(
            source,
            range.end.line,
            range.end.character,
        )
        .unwrap();
        assert_eq!(&source[start..end], item.name);
    }
}

#[test]
fn placeholder_features_do_not_offer_edits_or_fake_signatures() {
    let mut session = TestSession::new();
    session.initialize_default("unsupported_features");
    let uri = "file:///c:/spec42-lsp-tests/unsupported-features.sysml";
    let source = "package P { part def Left { port shared; } part def Right { port shared; } }";
    session.did_open(uri, source, 1);
    session.barrier();
    for method in [
        "textDocument/linkedEditingRange",
        "textDocument/signatureHelp",
    ] {
        let response = request_at(&mut session, method, uri, column(source, "shared"));
        assert!(response["error"].is_null(), "{response}");
        assert!(response["result"].is_null(), "{response}");
    }
}
