//! Reliability and determinism gates for newly-added LSP handlers.

use super::harness::TestSession;

#[test]
fn lsp_new_handlers_survive_invalid_intermediate_text() {
    let mut session = TestSession::new();
    let uri = "file:///quality-invalid.sysml";
    session.initialize_default("quality_gates_test");
    session.did_open(uri, "package P { part def Engine; part v : Engine; }\n", 1);
    session.did_change_full(uri, "package P { part def Engine part v : ; }\n", 2);
    session.barrier();

    let probes = vec![
        (
            "textDocument/signatureHelp",
            serde_json::json!({"textDocument":{"uri":uri},"position":{"line":0,"character":25}}),
        ),
        (
            "textDocument/documentLink",
            serde_json::json!({"textDocument":{"uri":uri}}),
        ),
        (
            "textDocument/codeLens",
            serde_json::json!({"textDocument":{"uri":uri}}),
        ),
        (
            "textDocument/linkedEditingRange",
            serde_json::json!({"textDocument":{"uri":uri},"position":{"line":0,"character":20}}),
        ),
        (
            "textDocument/inlayHint",
            serde_json::json!({"textDocument":{"uri":uri},"range":{"start":{"line":0,"character":0},"end":{"line":10,"character":0}}}),
        ),
    ];

    for (method, params) in probes {
        let resp = session.request(method, params);
        assert!(
            resp.get("error").is_none(),
            "{} should not fail on invalid intermediate text: {}",
            method,
            resp
        );
    }
}

#[test]
fn lsp_code_lens_is_deterministic_for_same_document_state() {
    let mut session = TestSession::new();
    let uri = "file:///quality-deterministic.sysml";
    let content = "package P {\n  part def Engine;\n  part vehicle : Engine;\n}\n";
    session.initialize_default("quality_gates_test");
    session.did_open(uri, content, 1);
    session.barrier();

    let first = session.request(
        "textDocument/codeLens",
        serde_json::json!({"textDocument":{"uri":uri}}),
    );
    let second = session.request(
        "textDocument/codeLens",
        serde_json::json!({"textDocument":{"uri":uri}}),
    );
    assert_eq!(
        first["result"], second["result"],
        "code lens should be stable across repeated requests for unchanged document"
    );
}

#[test]
fn lsp_code_lens_does_not_emit_inherited_attribute_lines_for_part_defs() {
    let mut session = TestSession::new();
    let uri = "file:///quality-inherited-codelens.sysml";
    let content = r#"
        package P {
          part def Base {
            attribute mass = 1200 [kg];
          }
          part def Car :> Base;
        }
    "#;
    session.initialize_default("quality_gates_test");
    session.did_open(uri, content, 1);
    session.barrier();

    let response = session.request(
        "textDocument/codeLens",
        serde_json::json!({"textDocument":{"uri":uri}}),
    );
    let lenses = response["result"].as_array().expect("code lens result array");
    let labels = lenses
        .iter()
        .filter_map(|lens| lens["command"]["title"].as_str())
        .collect::<Vec<_>>();
    assert!(
        labels.iter().all(|label| !label.starts_with("inherited ")),
        "inherited attribute code lenses should be disabled, got {labels:#?}"
    );
}

#[test]
fn lsp_code_lens_does_not_anchor_inherited_attributes_in_part_body() {
    let mut session = TestSession::new();
    let uri = "file:///quality-inherited-codelens-body-end.sysml";
    let content = r#"
        package P {
          part def Base {
            attribute mass = 1200 [kg];
          }
          part def Car :> Base {
            attribute dryMass = 900 [kg];
            attribute propellantMass = 300 [kg];
          }
        }
    "#;
    session.initialize_default("quality_gates_test");
    session.did_open(uri, content, 1);
    session.barrier();

    let response = session.request(
        "textDocument/codeLens",
        serde_json::json!({"textDocument":{"uri":uri}}),
    );
    let lenses = response["result"].as_array().expect("code lens result array");
    let labels = lenses
        .iter()
        .filter_map(|lens| lens["command"]["title"].as_str())
        .collect::<Vec<_>>();
    assert!(
        labels.iter().all(|label| !label.starts_with("inherited ")),
        "inherited attribute code lenses should be disabled, got {labels:#?}"
    );
}
