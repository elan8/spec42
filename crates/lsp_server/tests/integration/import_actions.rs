//! Import completion and whole-file actions at the actual protocol boundary.

use super::harness::TestSession;
use serde_json::{json, Value};

const URI: &str = "file:///c:/spec42-lsp-tests/import-actions.sysml";
const LIB_URI: &str = "file:///c:/spec42-lsp-tests/import-catalog.sysml";
const LIB: &str = "package Catalog { part def Motor; port def Plug; }";

fn capabilities() -> Value {
    json!({"workspace":{"workspaceEdit":{"documentChanges":true}},
        "textDocument":{"completion":{"completionItem":{"resolveSupport":{"properties":["additionalTextEdits","detail","documentation"]}}}}})
}

fn position(source: &str, name: &str) -> Value {
    let offset = source.rfind(name).expect("reference");
    let before = &source[..offset];
    json!({"line":before.bytes().filter(|b| *b == b'\n').count(),
        "character":before.rsplit('\n').next().unwrap().encode_utf16().count()})
}

fn items(session: &mut TestSession, source: &str, prefix: &str) -> Vec<Value> {
    let mut at = position(source, prefix);
    at["character"] =
        json!(at["character"].as_u64().unwrap() + prefix.encode_utf16().count() as u64);
    let result = session.request(
        "textDocument/completion",
        json!({"textDocument":{"uri":URI},"position":at}),
    );
    result["result"]["items"]
        .as_array()
        .expect("completion list")
        .clone()
}

fn apply(source: &str, edits: &[Value]) -> String {
    let mut edits = edits
        .iter()
        .map(|edit| serde_json::from_value::<tower_lsp::lsp_types::TextEdit>(edit.clone()).unwrap())
        .collect::<Vec<_>>();
    edits.sort_by_key(|edit| (edit.range.start.line, edit.range.start.character));
    let mut text = source.to_owned();
    for edit in edits.into_iter().rev() {
        let start = language_service::position_to_byte_offset(
            &text,
            edit.range.start.line,
            edit.range.start.character,
        )
        .unwrap();
        let end = language_service::position_to_byte_offset(
            &text,
            edit.range.end.line,
            edit.range.end.character,
        )
        .unwrap();
        text.replace_range(start..end, &edit.new_text);
    }
    text
}

fn session(source: &str, caps: Value) -> TestSession {
    let mut session = TestSession::new();
    session.initialize_with_capabilities("import-actions", caps);
    session.did_open(LIB_URI, LIB, 1);
    session.did_open(URI, source, 1);
    session.barrier();
    session
}

#[test]
fn import_completion_delivers_an_atomic_transaction_and_revalidates_on_resolve() {
    let source = "package Machine {\r\n    /*🧭*/ part drive : Mot;\r\n}\r\n";
    let mut session = session(source, capabilities());
    let item = items(&mut session, source, "Mot")
        .into_iter()
        .find(|item| item["label"] == "Motor")
        .expect("Motor auto import");
    assert_eq!(item["additionalTextEdits"].as_array().unwrap().len(), 1);
    let resolved = session.request("completionItem/resolve", item.clone());
    assert!(resolved.get("error").is_none(), "{resolved}");
    assert_eq!(resolved["result"]["textEdit"], item["textEdit"]);
    assert_eq!(
        resolved["result"]["additionalTextEdits"],
        item["additionalTextEdits"]
    );
    let mut edits = item["additionalTextEdits"]
        .as_array()
        .expect("import edits")
        .clone();
    edits.push(item["textEdit"].clone());
    let fixed = apply(source, &edits);
    assert_eq!(fixed,"package Machine {\r\n    private import Catalog::Motor;\r\n    /*🧭*/ part drive : Motor;\r\n}\r\n");
    session.did_change_full(URI, &fixed, 2);
    session.barrier();
    let definition = session.request(
        "textDocument/definition",
        json!({"textDocument":{"uri":URI},"position":position(&fixed,"Motor")}),
    );
    assert!(
        definition["result"]["uri"]
            .as_str()
            .is_some_and(|uri| uri.eq_ignore_ascii_case(LIB_URI)),
        "{definition}"
    );
}

#[test]
fn completion_bounds_its_import_proofs_and_requests_a_narrower_prefix() {
    let definitions = (0..40)
        .map(|index| format!("part def Motor{index:02};"))
        .collect::<String>();
    let source = "package Machine {\n    part drive : Mot;\n}\n";
    let narrowed = "package Machine {\n    part drive : Motor39;\n}\n";
    let mut session = TestSession::new();
    session.initialize_with_capabilities("import-actions", capabilities());
    session.did_open(LIB_URI, &format!("package Catalog {{ {definitions} }}"), 1);
    session.did_open(URI, source, 1);
    session.barrier();
    let mut at = position(source, "Mot");
    at["character"] = json!(at["character"].as_u64().unwrap() + 3);
    let response = session.request(
        "textDocument/completion",
        json!({"textDocument":{"uri":URI},"position":at}),
    );
    let imports = response["result"]["items"]
        .as_array()
        .expect("completion list")
        .iter()
        .filter(|item| item.get("additionalTextEdits").is_some())
        .count();
    assert_eq!(imports, 10, "{response}");
    assert_eq!(response["result"]["isIncomplete"], true);

    session.did_change_full(URI, narrowed, 2);
    session.barrier();
    let items = items(&mut session, narrowed, "Motor39");
    assert!(
        items
            .iter()
            .any(|item| item["label"] == "Motor39" && item.get("additionalTextEdits").is_some()),
        "a narrower prefix reaches a candidate past the bound: {items:?}"
    );
}

#[test]
fn completion_import_rejects_old_versions_and_reopened_documents() {
    let source = "package Machine { part drive : Mot; }";
    let mut session = session(source, capabilities());
    let item = items(&mut session, source, "Mot")
        .into_iter()
        .find(|item| item["label"] == "Motor")
        .unwrap();
    session.did_change_full(URI, source, 2); // A no-op still advances the editor revision.
    session.barrier();
    let stale = session.request_once("completionItem/resolve", item.clone());
    assert_eq!(stale["error"]["code"], -32801, "{stale}");
    let typed = source.replace("Mot;", "Moto;");
    session.did_change_full(URI, &typed, 3);
    session.barrier();
    let superseded = session.request_once("completionItem/resolve", item.clone());
    assert_eq!(superseded["error"]["code"], -32801, "{superseded}");
    let fresh = items(&mut session, &typed, "Moto")
        .into_iter()
        .find(|item| item["label"] == "Motor")
        .unwrap();
    let resolved = session.request("completionItem/resolve", fresh);
    assert!(resolved.get("error").is_none(), "{resolved}");
    session.did_close(URI);
    session.barrier();
    session.did_open(URI, source, 1);
    session.barrier();
    let reopened = session.request_once("completionItem/resolve", item);
    assert_eq!(reopened["error"]["code"], -32801, "{reopened}");
}

#[test]
fn clients_without_guarded_import_capabilities_keep_ordinary_completion() {
    let source = "package Machine { part def MotorLocal; part drive : Mot; }";
    let mut session = session(source, json!({}));
    let result = items(&mut session, source, "Mot");
    assert!(result.iter().any(|item| item["label"] == "MotorLocal"));
    assert!(!result.iter().any(|item| item["label"] == "Motor"));
    let actions = session.request(
        "textDocument/codeAction",
        json!({"textDocument":{"uri":URI},
        "range":{"start":{"line":0,"character":0},"end":{"line":0,"character":0}},
        "context":{"diagnostics":[],"only":["source.fixAll.spec42.imports"]}}),
    );
    assert_eq!(actions["result"], json!([]));
}

#[test]
fn completion_never_imports_from_a_superseded_other_document() {
    let source = "package Machine { part drive : Mot; }";
    let mut session = session(source, capabilities());
    session.did_change_full(LIB_URI, "package Catalog { part def Other; }", 2);
    let mut at = position(source, "Mot");
    at["character"] = json!(at["character"].as_u64().unwrap() + 3);
    let result = session.request_once(
        "textDocument/completion",
        json!({
            "textDocument":{"uri":URI}, "position":at
        }),
    );
    if result.get("error").is_some() {
        assert_eq!(result["error"]["code"], -32801, "{result}");
    } else {
        assert!(
            result["result"]["items"]
                .as_array()
                .unwrap()
                .iter()
                .all(|item| item["label"] != "Motor"),
            "{result}"
        );
    }
}

#[test]
fn fix_all_imports_is_one_versioned_document_transaction_and_respects_only() {
    let source = "package One { part a : Motor; part b : Motor; } package Two { port p : Plug; }";
    let mut session = session(source, capabilities());
    let params = json!({"textDocument":{"uri":URI},
        "range":{"start":{"line":0,"character":0},"end":{"line":0,"character":0}},
        "context":{"diagnostics":[],"only":["source.fixAll"]}});
    let actions = session.request("textDocument/codeAction", params.clone());
    let actions = actions["result"].as_array().expect("actions");
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0]["kind"], "source.fixAll.spec42.imports");
    let document = &actions[0]["edit"]["documentChanges"][0];
    assert_eq!(document["textDocument"]["version"], 1);
    let edits = document["edits"].as_array().unwrap();
    assert_eq!(edits.len(), 2);
    let fixed = apply(source, edits);
    assert_eq!(fixed,"package One { private import Catalog::Motor; part a : Motor; part b : Motor; } package Two { private import Catalog::Plug; port p : Plug; }");
    session.did_change_full(URI, &fixed, 2);
    session.barrier();
    let again = session.request("textDocument/codeAction", params);
    assert_eq!(again["result"], json!([]));
}
