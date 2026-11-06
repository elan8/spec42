//! The batch CLI host and the LSP must publish the same multi-document project.
use super::harness::{next_id, read_message, read_response, send_message, spawn_server_with_env};
use serde_json::{json, Value};
use spec42::cli::{CheckArgs, Cli, OutputFormat};
use std::collections::BTreeMap;
use sysml_diagnostics::DiagnosticSeverity;

#[test]
fn clean_saved_project_remains_clean_through_editor_and_watcher_lifecycles() {
    use super::harness::TestSession;
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().canonicalize().unwrap();
    let definition = "package Definitions { part def Component; }\n";
    let usage =
        "package Application { private import Definitions::*; part component : Component; }\n";
    let definitions = root.join("Definitions.sysml");
    let application = root.join("Application.sysml");
    std::fs::write(&definitions, definition).unwrap();
    std::fs::write(&application, usage).unwrap();
    let uri = |path: &std::path::Path| {
        sysml_query::source::path_to_file_url(path)
            .unwrap()
            .to_string()
    };
    let definition_uri = uri(&definitions);
    let application_uri = uri(&application);
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
    let (report, _) = spec42::perform_check_with_publication(
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
    .unwrap();
    assert_eq!(report.summary.error_count, 0);
    assert_eq!(report.summary.warning_count, 0);
    let mut session =
        TestSession::new_with_env(&[("SPEC42_LSP_TEST_NO_STDLIB", std::path::Path::new("1"))]);
    session.initialize_with_root(
        "saved-source-parity",
        &url::Url::from_directory_path(&root).unwrap(),
    );
    session.did_open(&application_uri, usage, 1);
    let diagnostics = session.wait_for_diagnostics_version(&application_uri, 1);
    assert_eq!(diagnostics["diagnostics"], json!([]));
    for interval in 0..3 {
        session.did_open(&definition_uri, definition, 1);
        let diagnostics = session.wait_for_diagnostics_version(&definition_uri, 1);
        assert_eq!(
            diagnostics["diagnostics"],
            json!([]),
            "open interval {interval}"
        );
        for version in 2..=5 {
            let text = format!("{definition}// revision {version}\n");
            std::fs::write(&definitions, &text).unwrap();
            session.did_change_full(&definition_uri, &text, version);
            session.notify(
                "textDocument/didSave",
                json!({"textDocument":{"uri":definition_uri}, "text":text}),
            );
            session.notify(
                "workspace/didChangeWatchedFiles",
                json!({"changes":[{"uri":definition_uri,"type":2}]}),
            );
            let diagnostics = session.wait_for_diagnostics_version(&definition_uri, version);
            assert_eq!(
                diagnostics["diagnostics"],
                json!([]),
                "interval {interval}, version {version}"
            );
            let consumer_text = format!("{usage}// interval {interval}, revision {version}\n");
            std::fs::write(&application, &consumer_text).unwrap();
            session.did_change_full(&application_uri, &consumer_text, interval * 4 + version);
            let diagnostics =
                session.wait_for_diagnostics_version(&application_uri, interval * 4 + version);
            assert_eq!(
                diagnostics["diagnostics"],
                json!([]),
                "consumer interval {interval}, version {version}"
            );
            let symbols = session.request("workspace/symbol", json!({"query":"Component"}));
            let definitions: Vec<_> = symbols["result"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|symbol| symbol["name"] == "Component")
                .collect();
            assert_eq!(definitions.len(), 1, "{symbols}");
        }
        session.did_close(&definition_uri);
        session.barrier();
    }
}

fn comparable_diagnostic(diagnostic: &Value) -> Value {
    json!({
        "code": diagnostic["code"], "severity": diagnostic["severity"],
        "message": diagnostic["message"], "source": diagnostic["source"],
        "range": diagnostic["range"],
        "relatedInformation": diagnostic.get("relatedInformation").cloned().unwrap_or(json!([])),
    })
}

#[test]
fn shared_library_edits_in_multiple_workspace_roots_preserve_clean_diagnostics() {
    use super::harness::TestSession;
    for library_is_workspace in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path().canonicalize().unwrap();
        let library = root.join("shared");
        let a = root.join("a");
        let b = root.join("b");
        for folder in [&library, &a, &b] {
            std::fs::create_dir(folder).unwrap();
        }
        let definition = "package Shared { part def Component; }\n";
        let library_file = library.join("Shared.sysml");
        std::fs::write(&library_file, definition).unwrap();
        let library_uri = sysml_query::source::path_to_file_url(&library_file).unwrap();
        let mut consumers = Vec::new();
        for (folder, package) in [(&a, "A"), (&b, "B")] {
            let path = folder.join("Model.sysml");
            let text = format!(
                "package {package} {{ private import Shared::*; part component : Component; }}\n"
            );
            std::fs::write(&path, &text).unwrap();
            consumers.push((path, text));
        }
        let mut session =
            TestSession::new_with_env(&[("SPEC42_LSP_TEST_NO_STDLIB", std::path::Path::new("1"))]);
        let mut folders = vec![
            json!({"uri":url::Url::from_directory_path(&a).unwrap(),"name":"a"}),
            json!({"uri":url::Url::from_directory_path(&b).unwrap(),"name":"b"}),
        ];
        if library_is_workspace {
            folders.push(
                json!({"uri":url::Url::from_directory_path(&library).unwrap(),"name":"shared"}),
            );
        }
        let initialized = session.request(
            "initialize",
            json!({
                "processId":null, "rootUri":url::Url::from_directory_path(&a).unwrap(),
                "workspaceFolders":folders, "capabilities":{},
                "initializationOptions":{"libraryPaths":[library]}
            }),
        );
        assert!(initialized.get("error").is_none(), "{initialized}");
        session.notify("initialized", json!({}));
        for (path, text) in &consumers {
            let uri = sysml_query::source::path_to_file_url(path).unwrap();
            session.did_open(uri.as_str(), text, 1);
            assert_eq!(
                session.wait_for_diagnostics_version(uri.as_str(), 1)["diagnostics"],
                json!([]),
                "initial diagnostics for {}, library workspace={library_is_workspace}",
                path.display()
            );
        }
        session.did_open(library_uri.as_str(), definition, 1);
        assert_eq!(
            session.wait_for_diagnostics_version(library_uri.as_str(), 1)["diagnostics"],
            json!([])
        );
        for version in 2..=6 {
            let text = format!("{definition}// version {version}\n");
            std::fs::write(&library_file, &text).unwrap();
            session.did_change_full(library_uri.as_str(), &text, version);
            session.notify(
                "textDocument/didSave",
                json!({"textDocument":{"uri":library_uri}}),
            );
            session.notify(
                "workspace/didChangeWatchedFiles",
                json!({"changes":[{"uri":library_uri,"type":2}]}),
            );
            assert_eq!(
                session.wait_for_diagnostics_version(library_uri.as_str(), version)["diagnostics"],
                json!([])
            );
            for (path, saved) in &consumers {
                let uri = sysml_query::source::path_to_file_url(path).unwrap();
                let text = format!("{saved}// version {version}\n");
                std::fs::write(path, &text).unwrap();
                session.did_change_full(uri.as_str(), &text, version);
                assert_eq!(
                    session.wait_for_diagnostics_version(uri.as_str(), version)["diagnostics"],
                    json!([]),
                    "library workspace={library_is_workspace}, version={version}"
                );
            }
        }
        session.did_close(library_uri.as_str());
        session.did_open(
            library_uri.as_str(),
            &std::fs::read_to_string(&library_file).unwrap(),
            1,
        );
        assert_eq!(
            session.wait_for_diagnostics_version(library_uri.as_str(), 1)["diagnostics"],
            json!([])
        );
        // Chat previews and editing snapshots must not become competing declarations.
        for scheme in [
            "chat-editing-text-model",
            "vscode-chat-code-block",
            "chat-editing-snapshot-text-model",
            "git",
        ] {
            let virtual_uri =
                url::Url::parse(&format!("{scheme}://snapshot/Shared.sysml")).unwrap();
            session.did_open(virtual_uri.as_str(), definition, 1);
        }
        session.barrier();
        for version in 2..=30 {
            let name = if version % 2 == 0 {
                "Component"
            } else {
                "Intermediate"
            };
            session.did_change_full(
                library_uri.as_str(),
                &format!("package Shared {{ part def {name}; }} // {version}"),
                version,
            );
        }
        assert_eq!(
            session.wait_for_diagnostics_version(library_uri.as_str(), 30)["diagnostics"],
            json!([])
        );
        for (path, saved) in &consumers {
            let uri = sysml_query::source::path_to_file_url(path).unwrap();
            let text = format!("{saved}// after chat previews and edit burst\n");
            std::fs::write(path, &text).unwrap();
            session.did_change_full(uri.as_str(), &text, 7);
            assert_eq!(
                session.wait_for_diagnostics_version(uri.as_str(), 7)["diagnostics"],
                json!([])
            );
        }
        // Settle each structural revision rather than allowing the worker to coalesce a burst.
        // Repeatedly replace a declaration and update both consumers to that exact saved state.
        for version in 31..=50 {
            let name = format!("Component{version}");
            let text = format!("package Shared {{ part def {name}; }}\n");
            std::fs::write(&library_file, &text).unwrap();
            session.did_change_full(library_uri.as_str(), &text, version);
            assert_eq!(
                session.wait_for_diagnostics_version(library_uri.as_str(), version)["diagnostics"],
                json!([])
            );
            for (path, saved) in &consumers {
                let uri = sysml_query::source::path_to_file_url(path).unwrap();
                let text = saved.replace("Component", &name);
                std::fs::write(path, &text).unwrap();
                session.did_change_full(uri.as_str(), &text, version);
                session.notify("textDocument/didSave", json!({"textDocument":{"uri":uri}}));
                session.notify(
                    "workspace/didChangeWatchedFiles",
                    json!({"changes":[{"uri":uri,"type":2},{"uri":library_uri,"type":2}]}),
                );
                assert_eq!(
                    session.wait_for_diagnostics_version(uri.as_str(), version)["diagnostics"],
                    json!([]),
                    "settled batch {version}, library workspace={library_is_workspace}"
                );
            }
        }
    }
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
