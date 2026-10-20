//! The batch CLI host and the LSP must publish the same multi-document project.
use super::harness::{next_id, read_message, read_response, send_message, spawn_server_with_env};
use serde_json::{json, Value};
use spec42::cli::{CheckArgs, Cli, OutputFormat};
use std::collections::BTreeMap;
use sysml_diagnostics::DiagnosticSeverity;

fn comparable_diagnostic(diagnostic: &Value) -> Value {
    json!({
        "code": diagnostic["code"], "severity": diagnostic["severity"],
        "message": diagnostic["message"], "source": diagnostic["source"],
        "range": diagnostic["range"],
        "relatedInformation": diagnostic.get("relatedInformation").cloned().unwrap_or(json!([])),
    })
}

#[test]
fn cli_and_lsp_publish_the_same_multifile_diagnostics_in_both_orders() {
    for reverse in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path().canonicalize().unwrap();
        let short = "package Defs { part def Wheel; }\n".to_string();
        // A reference after many lines cannot accidentally be located against the short document.
        let long = format!("{}package Uses {{\n  private import Defs::*;\n  part def Vehicle {{\n    part wheel : Wheel;\n    ref selected = wheel;\n    attribute invalid : Absent;\n  }}\n}}\n", "// padding\n".repeat(80));
        let documents = if reverse {
            [long, short]
        } else {
            [short, long]
        };
        let mut uris = Vec::new();
        for (name, source) in ["a.sysml", "z.sysml"].into_iter().zip(&documents) {
            let path = root.join(name);
            std::fs::write(&path, source).unwrap();
            uris.push(
                sysml_query::source::normalize_uri(&url::Url::from_file_path(path).unwrap())
                    .to_string(),
            );
        }
        let cli = Cli {
            config_path: None,
            library_paths: vec![],
            stdlib_path: None,
            kpar_library_paths: vec![],
            project_libraries: vec![],
            disabled_kpar_libraries: vec![],
            no_stdlib: true,
            stdio: false,
            command: None,
        };
        // This is the CLI check command's host entry point, including filesystem admission.
        let (report, model) = spec42::perform_check_with_publication(
            &cli,
            &CheckArgs {
                path: root.clone(),
                workspace_root: Some(root.clone()),
                format: OutputFormat::Json,
                warnings_as_errors: false,
                baseline: None,
                strict_diagnostics: false,
            },
        )
        .expect("CLI must publish the multi-document workspace");
        assert_eq!(report.summary.document_count, 2);
        assert!(
            report.summary.warning_count > 0,
            "the parity assertion must exercise diagnostics"
        );
        assert!(model.publication().completeness().is_complete());
        let expected: BTreeMap<_, _> = report.documents.iter().map(|document| {
            let diagnostics: Vec<_> = document.diagnostics.iter().map(|diagnostic| {
                let range = |range: sysml_query::resolved_slice::TextRange| json!({
                    "start": {"line": range.start.line, "character": range.start.character},
                    "end": {"line": range.end.line, "character": range.end.character},
                });
                json!({
                    "code": diagnostic.code,
                    "severity": match diagnostic.severity {
                        DiagnosticSeverity::Error => 1, DiagnosticSeverity::Warning => 2,
                        DiagnosticSeverity::Information => 3,
                    },
                    "message": diagnostic.message, "source": diagnostic.source,
                    "range": range(diagnostic.range),
                    "relatedInformation": diagnostic.related_information.iter().map(|related| json!({
                        "location": {"uri": related.uri, "range": range(related.range)},
                        "message": related.message,
                    })).collect::<Vec<_>>(),
                })
            }).collect();
            (document.uri.clone(), diagnostics)
        }).collect();
        let mut server =
            spawn_server_with_env(&[("SPEC42_LSP_TEST_NO_STDLIB", std::path::Path::new("1"))]);
        let mut stdin = server.stdin.take().unwrap();
        let mut stdout = server.stdout.take().unwrap();
        let id = next_id();
        send_message(&mut stdin, &json!({"jsonrpc":"2.0", "id":id, "method":"initialize",
            "params":{"processId":null, "rootUri":url::Url::from_directory_path(&root).unwrap(), "capabilities":{}}}).to_string());
        read_response(&mut stdout, id).expect("initialize response");
        send_message(
            &mut stdin,
            &json!({"jsonrpc":"2.0", "method":"initialized", "params":{}}).to_string(),
        );
        let mut actual = BTreeMap::new();
        loop {
            let message: Value =
                serde_json::from_str(&read_message(&mut stdout).expect("project publication"))
                    .unwrap();
            if message["method"] == "textDocument/publishDiagnostics" {
                let uri = message["params"]["uri"].as_str().unwrap().to_string();
                if expected.contains_key(&uri) {
                    actual.insert(
                        uri,
                        message["params"]["diagnostics"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(comparable_diagnostic)
                            .collect::<Vec<_>>(),
                    );
                }
            }
            if message["method"] == "spec42/publicationChanged" && actual.len() == expected.len() {
                assert!(message["params"]["modelDigest"]
                    .as_str()
                    .is_some_and(|digest| !digest.is_empty()));
                break;
            }
        }
        assert_eq!(
            actual, expected,
            "startup diagnostic parity, reverse={reverse}"
        );
        let id = next_id();
        send_message(&mut stdin, &json!({"jsonrpc":"2.0", "id":id, "method":"workspace/symbol", "params":{"query":"Vehicle"}}).to_string());
        let symbols: Value =
            serde_json::from_str(&read_response(&mut stdout, id).expect("workspace symbols"))
                .unwrap();
        assert!(
            symbols["result"]
                .as_array()
                .unwrap()
                .iter()
                .any(|symbol| symbol["name"] == "Vehicle"),
            "the LSP must expose a semantic model: {symbols}"
        );
        // Reopen the same revision in the opposite order; syntax memo warmth and tab order are irrelevant.
        for index in [1, 0] {
            send_message(&mut stdin, &json!({"jsonrpc":"2.0", "method":"textDocument/didOpen", "params":{
                "textDocument":{"uri":uris[index], "languageId":"sysml", "version":1, "text":documents[index]}
            }}).to_string());
        }
        let mut opened = BTreeMap::new();
        while opened.len() != expected.len() {
            let message: Value = serde_json::from_str(
                &read_message(&mut stdout).expect("opened project diagnostics"),
            )
            .unwrap();
            if message["method"] == "textDocument/publishDiagnostics" {
                let uri = message["params"]["uri"].as_str().unwrap().to_string();
                if expected.contains_key(&uri) {
                    opened.insert(
                        uri,
                        message["params"]["diagnostics"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(comparable_diagnostic)
                            .collect::<Vec<_>>(),
                    );
                }
            }
        }
        assert_eq!(
            opened, expected,
            "tab-order diagnostic parity, reverse={reverse}"
        );
    }
}
