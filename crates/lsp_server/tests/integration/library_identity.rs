//! Workspace/library overlap is an identity and host-configuration concern.
use super::harness::TestSession;
use serde_json::json;

fn parity_session(root: &std::path::Path, libraries: &[std::path::PathBuf]) -> TestSession {
    let mut session = TestSession::new_with_env(&[
        ("SPEC42_LSP_TEST_NO_STDLIB", std::path::Path::new("1")),
        ("SPEC42_LIBRARY_FULL_SCAN", std::path::Path::new("0")),
    ]);
    session.initialize_with_root_and_options(
        "library-admission-parity",
        Some(&url::Url::from_directory_path(root).unwrap()),
        Some(json!({"libraryPaths":libraries,"performanceLogging":{"enabled":true}})),
    );
    session
}

fn publication(
    session: &mut TestSession,
    uri: &str,
    version: i32,
) -> (serde_json::Value, serde_json::Value) {
    let (record, diagnostics) = session.wait_for_publication_version(uri, version);
    assert_eq!(record["outcome"], "Published");
    assert!(record["modelDigest"]
        .as_str()
        .is_some_and(|digest| !digest.is_empty()));
    (record["modelDigest"].clone(), diagnostics)
}

#[test]
fn library_source_events_also_reach_the_independent_workspace_authoring_owner() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().canonicalize().unwrap();
    let library = root.join("library");
    std::fs::create_dir(&library).unwrap();
    std::fs::write(
        library.join(".project.json"),
        r#"{"name":"library-project","version":"1.0.0"}"#,
    )
    .unwrap();
    let source = library.join("Model.sysml");
    std::fs::write(&source, "package Saved;").unwrap();
    let consumer = root.join("App.sysml");
    std::fs::write(&consumer, "package App;").unwrap();
    let uri = sysml_query::source::path_to_file_url(&source).unwrap();
    let consumer_uri = sysml_query::source::path_to_file_url(&consumer).unwrap();
    let mut session = parity_session(&root, &[library]);
    session.did_open(consumer_uri.as_str(), "package App; // opened", 1);
    publication(&mut session, consumer_uri.as_str(), 1);

    // Reads select the nested project's authoring owner. Both that owner and the enclosing
    // project's library consumer must ingest the editor revisions before answering them.
    for (version, name) in [(1, "Unsaved"), (2, "Latest")] {
        let text = format!("package {name};");
        if version == 1 {
            session.did_open(uri.as_str(), &text, version);
        } else {
            session.did_change_full(uri.as_str(), &text, version);
        }
        let response = session.request(
            "textDocument/documentSymbol",
            json!({"textDocument":{"uri":uri}}),
        );
        assert!(response.get("error").is_none(), "{response}");
        let names: Vec<_> = response["result"]
            .as_array()
            .unwrap()
            .iter()
            .map(|symbol| symbol["name"].as_str().unwrap())
            .collect();
        assert_eq!(names, [name]);
    }
    std::fs::write(&source, "package Restored;").unwrap();
    session.notify("textDocument/didClose", json!({"textDocument":{"uri":uri}}));
    let response = session.request(
        "textDocument/documentSymbol",
        json!({"textDocument":{"uri":uri}}),
    );
    assert!(response.get("error").is_none(), "{response}");
    let symbols = response["result"].as_array().unwrap();
    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0]["name"], "Restored");
}

#[test]
fn configuration_and_incremental_import_changes_match_fresh_publications() {
    let directory = tempfile::tempdir().unwrap();
    let base = directory.path().canonicalize().unwrap();
    let root = base.join("workspace");
    let first = base.join("first-library");
    let second = base.join("second-library");
    for path in [&root, &first, &second] {
        std::fs::create_dir(path).unwrap();
    }
    std::fs::write(
        first.join("First.sysml"),
        "package First { part def Thing; }",
    )
    .unwrap();
    std::fs::write(
        second.join("Second.sysml"),
        "package Second { part def Thing; }",
    )
    .unwrap();
    std::fs::write(
        second.join("Unused.sysml"),
        "package Unused { part def Unused; }",
    )
    .unwrap();
    let source = root.join("App.sysml");
    std::fs::write(&source, "package App;").unwrap();
    let uri = sysml_query::source::path_to_file_url(&source)
        .unwrap()
        .to_string();
    let initial = "package App { private import First::*; part thing : Thing; }";
    let latest = initial.replace("First", "Second");
    let mut edited = parity_session(&root, std::slice::from_ref(&first));
    edited.did_open(&uri, initial, 1);
    publication(&mut edited, &uri, 1);
    edited.notify(
        "workspace/didChangeConfiguration",
        json!({"settings":{"spec42":{"libraryPaths":[second]}}}),
    );
    let start = initial.find("First").unwrap();
    edited.notify("textDocument/didChange", json!({
        "textDocument":{"uri":uri,"version":2},
        "contentChanges":[{"range":{"start":{"line":0,"character":start},"end":{"line":0,"character":start+5}},"text":"Second"}]
    }));
    let changed = publication(&mut edited, &uri, 2);
    let mut fresh = parity_session(&root, std::slice::from_ref(&second));
    fresh.did_open(&uri, &latest, 1);
    assert_eq!(changed, publication(&mut fresh, &uri, 1));
    drop(fresh);

    // Removing the import must prune the now unreachable library sources as on restart.
    edited.did_change_full(&uri, "package App; // no imports", 3);
    let pruned = publication(&mut edited, &uri, 3);
    let mut fresh = parity_session(&root, std::slice::from_ref(&second));
    fresh.did_open(&uri, "package App; // no imports", 1);
    assert_eq!(pruned, publication(&mut fresh, &uri, 1));
}

#[test]
fn cross_file_imports_and_observed_library_revisions_match_fresh_publications() {
    let directory = tempfile::tempdir().unwrap();
    let base = directory.path().canonicalize().unwrap();
    let root = base.join("workspace");
    let library = base.join("library");
    std::fs::create_dir(&root).unwrap();
    std::fs::create_dir(&library).unwrap();
    let definition = library.join("Second.sysml");
    std::fs::write(
        library.join("First.sysml"),
        "package First { part def Thing; }",
    )
    .unwrap();
    std::fs::write(&definition, "package Second { part def Thing; }").unwrap();
    let source = root.join("App.sysml");
    let other = root.join("Other.sysml");
    std::fs::write(&source, "package App;").unwrap();
    std::fs::write(
        &other,
        "package Other { private import First::*; part thing : Thing; }",
    )
    .unwrap();
    let uri = sysml_query::source::path_to_file_url(&source)
        .unwrap()
        .to_string();
    let mut edited = parity_session(&root, std::slice::from_ref(&library));
    edited.did_open(&uri, "package App; // initial", 1);
    publication(&mut edited, &uri, 1);
    std::fs::write(
        &other,
        "package Other { private import Second::*; part thing : Thing; }",
    )
    .unwrap();
    edited.notify("workspace/didChangeWatchedFiles", json!({"changes":[{"uri":sysml_query::source::path_to_file_url(&other).unwrap(),"type":2}]}));
    edited.did_change_full(&uri, "package App; // second", 2);
    let changed = publication(&mut edited, &uri, 2);
    let mut fresh = parity_session(&root, std::slice::from_ref(&library));
    fresh.did_open(&uri, "package App; // second", 1);
    assert_eq!(changed, publication(&mut fresh, &uri, 1));
    drop(fresh);

    std::fs::write(&definition, "package Second { part def Replacement; }").unwrap();
    edited.notify("workspace/didChangeWatchedFiles", json!({"changes":[{"uri":sysml_query::source::path_to_file_url(&definition).unwrap(),"type":2}]}));
    edited.did_change_full(&uri, "package App; // third", 3);
    let changed = publication(&mut edited, &uri, 3);
    let mut fresh = parity_session(&root, std::slice::from_ref(&library));
    fresh.did_open(&uri, "package App; // third", 1);
    assert_eq!(changed, publication(&mut fresh, &uri, 1));
}

#[test]
fn removing_a_library_role_preserves_workspace_authored_sources_with_restart_parity() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().canonicalize().unwrap();
    let library = root.join("authored-library");
    std::fs::create_dir(&library).unwrap();
    std::fs::write(
        library.join("Authored.sysml"),
        "package Authored { part def Thing; }",
    )
    .unwrap();
    let source = root.join("App.sysml");
    std::fs::write(&source, "package App;").unwrap();
    let uri = sysml_query::source::path_to_file_url(&source)
        .unwrap()
        .to_string();
    let mut edited = parity_session(&root, &[library]);
    edited.did_open(&uri, "package App; // initial", 1);
    publication(&mut edited, &uri, 1);
    edited.notify(
        "workspace/didChangeConfiguration",
        json!({"settings":{"spec42":{"libraryPaths":[]}}}),
    );
    edited.did_change_full(&uri, "package App; // updated", 2);
    let current = publication(&mut edited, &uri, 2);
    let mut fresh = parity_session(&root, &[]);
    fresh.did_open(&uri, "package App; // updated", 1);
    assert_eq!(current, publication(&mut fresh, &uri, 1));
}

#[test]
fn loose_documents_select_libraries_without_scanning_unopened_siblings() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().canonicalize().unwrap();
    let workspace = root.join("loose");
    let library = root.join("library");
    std::fs::create_dir(&workspace).unwrap();
    std::fs::create_dir(&library).unwrap();
    std::fs::write(
        library.join("First.sysml"),
        "package First { part def Thing; }",
    )
    .unwrap();
    std::fs::write(
        library.join("Second.sysml"),
        "package Second { part def Thing; }",
    )
    .unwrap();
    let source = workspace.join("App.sysml");
    let sibling = workspace.join("Unopened.sysml");
    std::fs::write(&source, "package App;").unwrap();
    std::fs::write(&sibling, "package Unopened;").unwrap();
    let uri = sysml_query::source::path_to_file_url(&source)
        .unwrap()
        .to_string();
    let session = || {
        let mut session = TestSession::new_with_env(&[
            ("SPEC42_LSP_TEST_NO_STDLIB", std::path::Path::new("1")),
            ("SPEC42_LIBRARY_FULL_SCAN", std::path::Path::new("0")),
        ]);
        session.initialize_with_options(
            "loose-library-parity",
            Some(json!({"libraryPaths":[library],"performanceLogging":{"enabled":true}})),
        );
        session
    };
    let mut edited = session();
    edited.did_open(
        &uri,
        "package App { private import First::*; part thing : Thing; }",
        1,
    );
    publication(&mut edited, &uri, 1);
    let latest = "package App { private import Second::*; part thing : Thing; }";
    edited.did_change_full(&uri, latest, 2);
    let current = publication(&mut edited, &uri, 2);
    std::fs::write(&sibling, "package DifferentUnopened;").unwrap();
    let mut fresh = session();
    fresh.did_open(&uri, latest, 1);
    assert_eq!(current, publication(&mut fresh, &uri, 1));
}

#[test]
fn unsaved_library_overlays_and_close_restore_fresh_publication_identity() {
    library_overlay_restart_parity(false);
}

#[test]
fn project_rediscovery_preserves_library_overlays_and_close_restart_parity() {
    library_overlay_restart_parity(true);
}

fn library_overlay_restart_parity(rediscover: bool) {
    let directory = tempfile::tempdir().unwrap();
    let base = directory.path().canonicalize().unwrap();
    let root = base.join("workspace");
    let library = base.join("library");
    std::fs::create_dir(&root).unwrap();
    std::fs::create_dir(&library).unwrap();
    let definition = library.join("First.sysml");
    let saved = "package First { part def Thing; }";
    let unsaved = "package First { part def Replacement; }";
    std::fs::write(&definition, saved).unwrap();
    let source = root.join("App.sysml");
    std::fs::write(&source, "package App;").unwrap();
    let uri = sysml_query::source::path_to_file_url(&source)
        .unwrap()
        .to_string();
    let library_uri = sysml_query::source::path_to_file_url(&definition)
        .unwrap()
        .to_string();
    let initial = "package App { private import First::*; part thing : Thing; }";
    let latest = format!("{initial} // updated");
    let mut edited = parity_session(&root, std::slice::from_ref(&library));
    edited.did_open(&uri, initial, 1);
    publication(&mut edited, &uri, 1);
    edited.did_open(&library_uri, saved, 1);
    publication(&mut edited, &library_uri, 1);
    edited.did_change_full(&library_uri, unsaved, 2);
    publication(&mut edited, &library_uri, 2);
    if rediscover {
        let config = root.join("syside.toml");
        std::fs::write(&config, "include = ['**/*.sysml']").unwrap();
        edited.notify(
            "workspace/didChangeWatchedFiles",
            json!({"changes":[{"uri":sysml_query::source::path_to_file_url(&config).unwrap(),"type":1}]}),
        );
    }
    edited.did_change_full(&uri, &latest, 2);
    let current = publication(&mut edited, &uri, 2);
    let mut fresh = parity_session(&root, std::slice::from_ref(&library));
    fresh.did_open(&uri, &latest, 1);
    publication(&mut fresh, &uri, 1);
    fresh.did_open(&library_uri, unsaved, 1);
    assert_eq!(current, publication(&mut fresh, &library_uri, 1));
    drop(fresh);

    edited.notify(
        "textDocument/didClose",
        json!({"textDocument":{"uri":library_uri}}),
    );
    let restored = format!("{initial} // restored");
    edited.did_change_full(&uri, &restored, 3);
    let (digest, mut diagnostics) = publication(&mut edited, &uri, 3);
    // Closing an authoring surface clears its former diagnostics. A fresh session never
    // reported that closed library; both states therefore leave the same empty result there.
    let cleared = diagnostics
        .as_object_mut()
        .unwrap()
        .remove(&library_uri)
        .unwrap();
    assert_eq!(cleared, json!([]));
    let mut fresh = parity_session(&root, std::slice::from_ref(&library));
    fresh.did_open(&uri, &restored, 1);
    assert_eq!((digest, diagnostics), publication(&mut fresh, &uri, 1));
}

#[test]
fn editing_and_closing_a_library_after_root_removal_keeps_its_original_consumer() {
    let directory = tempfile::tempdir().unwrap();
    let base = directory.path().canonicalize().unwrap();
    let root = base.join("workspace");
    let library = base.join("library");
    std::fs::create_dir(&root).unwrap();
    std::fs::create_dir(&library).unwrap();
    let definition = library.join("First.sysml");
    let saved = "package First { part def Thing; }";
    let unsaved = "package First { part def Replacement; }";
    std::fs::write(&definition, saved).unwrap();
    let source = root.join("App.sysml");
    std::fs::write(&source, "package App;").unwrap();
    let uri = sysml_query::source::path_to_file_url(&source)
        .unwrap()
        .to_string();
    let library_uri = sysml_query::source::path_to_file_url(&definition)
        .unwrap()
        .to_string();
    let initial = "package App { private import First::*; part thing : Thing; }";
    let latest = format!("{initial} // removed root");
    let remove = |session: &mut TestSession| {
        session.notify(
            "workspace/didChangeConfiguration",
            json!({"settings":{"spec42":{"libraryPaths":[]}}}),
        );
    };
    let mut edited = parity_session(&root, std::slice::from_ref(&library));
    edited.did_open(&uri, initial, 1);
    publication(&mut edited, &uri, 1);
    edited.did_open(&library_uri, saved, 1);
    publication(&mut edited, &library_uri, 1);
    remove(&mut edited);
    edited.did_change_full(&library_uri, unsaved, 2);
    publication(&mut edited, &library_uri, 2);
    edited.did_change_full(&uri, &latest, 2);
    let after_removal = publication(&mut edited, &uri, 2);

    // The same admitted inputs must result whether the buffer changes before or after removing
    // the root. The still-open interval retains its library provenance in both sessions.
    let mut control = parity_session(&root, std::slice::from_ref(&library));
    control.did_open(&uri, initial, 1);
    publication(&mut control, &uri, 1);
    control.did_open(&library_uri, unsaved, 1);
    publication(&mut control, &library_uri, 1);
    remove(&mut control);
    control.did_change_full(&uri, &latest, 2);
    assert_eq!(after_removal, publication(&mut control, &uri, 2));
    drop(control);

    edited.notify(
        "textDocument/didClose",
        json!({"textDocument":{"uri":library_uri}}),
    );
    let closed = format!("{initial} // closed");
    edited.did_change_full(&uri, &closed, 3);
    let (digest, mut diagnostics) = publication(&mut edited, &uri, 3);
    assert_eq!(
        diagnostics
            .as_object_mut()
            .unwrap()
            .remove(&library_uri)
            .unwrap(),
        json!([])
    );
    let mut fresh = parity_session(&root, &[]);
    fresh.did_open(&uri, &closed, 1);
    assert_eq!((digest, diagnostics), publication(&mut fresh, &uri, 1));
}

#[test]
fn library_settings_update_defaults_before_the_first_loose_project_exists() {
    let directory = tempfile::tempdir().unwrap();
    let base = directory.path().canonicalize().unwrap();
    let root = base.join("loose");
    let old = base.join("old");
    let new = base.join("new");
    for path in [&root, &old, &new] {
        std::fs::create_dir(path).unwrap();
    }
    std::fs::write(old.join("Old.sysml"), "package Old;").unwrap();
    std::fs::write(new.join("Next.sysml"), "package Next { part def Thing; }").unwrap();
    let source = root.join("App.sysml");
    std::fs::write(&source, "package App;").unwrap();
    let uri = sysml_query::source::path_to_file_url(&source)
        .unwrap()
        .to_string();
    let open = |library: &std::path::Path| {
        let mut session =
            TestSession::new_with_env(&[("SPEC42_LSP_TEST_NO_STDLIB", std::path::Path::new("1"))]);
        session.initialize_with_options(
            "future-project-defaults",
            Some(json!({"libraryPaths":[library],"performanceLogging":{"enabled":true}})),
        );
        session
    };
    let text = "package App { private import Next::*; part thing : Thing; }";
    let mut changed = open(&old);
    changed.notify(
        "workspace/didChangeConfiguration",
        json!({"settings":{"spec42":{"libraryPaths":[new]}}}),
    );
    changed.did_open(&uri, text, 1);
    let current = publication(&mut changed, &uri, 1);
    let mut fresh = open(&new);
    fresh.did_open(&uri, text, 1);
    assert_eq!(current, publication(&mut fresh, &uri, 1));
}

#[test]
fn editor_library_defaults_do_not_replace_manifest_owned_project_bindings() {
    let directory = tempfile::tempdir().unwrap();
    let base = directory.path().canonicalize().unwrap();
    let root = base.join("project");
    let old = base.join("old");
    let new = base.join("new");
    for path in [&root, &old, &new] {
        std::fs::create_dir(path).unwrap();
    }
    std::fs::write(
        root.join(".project.json"),
        r#"{"name":"pinned-project","version":"1.0.0"}"#,
    )
    .unwrap();
    std::fs::write(old.join("Old.sysml"), "package Old;").unwrap();
    std::fs::write(new.join("Next.sysml"), "package Next { part def Thing; }").unwrap();
    let source = root.join("App.sysml");
    std::fs::write(&source, "package App;").unwrap();
    let uri = sysml_query::source::path_to_file_url(&source)
        .unwrap()
        .to_string();
    let text = "package App { private import Next::*; part thing : Thing; }";
    let mut changed = parity_session(&root, &[old]);
    changed.did_open(&uri, text, 1);
    publication(&mut changed, &uri, 1);
    changed.notify(
        "workspace/didChangeConfiguration",
        json!({"settings":{"spec42":{"libraryPaths":[new]}}}),
    );
    let latest = format!("{text} // unchanged project bindings");
    changed.did_change_full(&uri, &latest, 2);
    let current = publication(&mut changed, &uri, 2);
    let mut fresh = parity_session(&root, &[new]);
    fresh.did_open(&uri, &latest, 1);
    assert_eq!(current, publication(&mut fresh, &uri, 1));
}

#[test]
fn a_library_inside_the_workspace_keeps_one_identity_for_absolute_and_relative_paths() {
    for relative in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path().canonicalize().unwrap();
        let library = root.join("libs");
        std::fs::create_dir(&library).unwrap();
        std::fs::write(
            library.join("Custom.sysml"),
            "package Custom { part def Thing; }",
        )
        .unwrap();
        let model = root.join("App.sysml");
        std::fs::write(
            &model,
            "package App { private import Custom::*; part thing : Thing; }",
        )
        .unwrap();
        let uri = sysml_query::source::path_to_file_url(&model).unwrap();
        let library_path = if relative {
            "libs".to_owned()
        } else {
            library.to_string_lossy().into_owned()
        };
        let mut session =
            TestSession::new_with_env(&[("SPEC42_LSP_TEST_NO_STDLIB", std::path::Path::new("1"))]);
        session.initialize_with_root_and_options(
            "overlap-regression",
            Some(&url::Url::from_directory_path(&root).unwrap()),
            Some(json!({"libraryPaths":[library_path]})),
        );
        assert!(
            session.wait_for_diagnostics(uri.as_str()).is_empty(),
            "relative={relative}"
        );
        let stats = session.request("sysml/serverStats", serde_json::Value::Null);
        assert_eq!(
            stats["result"]["caches"]["documents"], 2,
            "one workspace source and one library source: {stats}"
        );
        let definition = session.request("workspace/symbol", json!({"query":"Thing"}));
        assert_eq!(
            definition["result"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|symbol| symbol["name"] == "Thing")
                .count(),
            1
        );
    }
}

#[test]
fn configured_and_unconfigured_library_copies_do_not_split_path_identity() {
    for configured in [false, true] {
        let stdlib = tempfile::tempdir().unwrap();
        let systems = stdlib.path().join("SysML_Systems_Library-2.0.0");
        std::fs::create_dir(&systems).unwrap();
        let source = "standard library package Bundled { part def Thing; }";
        std::fs::write(systems.join("Bundled.sysml"), source).unwrap();
        let workspace = tempfile::tempdir().unwrap();
        let root = workspace.path().canonicalize().unwrap();
        let copy = root.join("bundled-copy");
        std::fs::create_dir(&copy).unwrap();
        std::fs::write(copy.join("Bundled.sysml"), source).unwrap();
        let model = root.join("App.sysml");
        std::fs::write(
            &model,
            "package App { private import Bundled::*; part thing : Thing; }",
        )
        .unwrap();
        let mut session = TestSession::new_with_env(&[
            ("SPEC42_LSP_TEST_STDLIB", stdlib.path()),
            ("SPEC42_LIBRARY_FULL_SCAN", std::path::Path::new("0")),
        ]);
        let options = if configured {
            json!({"libraryPaths":[copy]})
        } else {
            json!({})
        };
        session.initialize_with_root_and_options(
            "duplicate-library-regression",
            Some(&url::Url::from_directory_path(&root).unwrap()),
            Some(options),
        );
        let uri = sysml_query::source::path_to_file_url(&model).unwrap();
        let diagnostics = session.wait_for_diagnostics(uri.as_str());
        let ambiguous = diagnostics
            .iter()
            .any(|diagnostic| diagnostic["code"] == "ambiguous_import_target");
        assert!(!ambiguous, "current package-closure policy must remain unambiguous: configured={configured}: {diagnostics:?}");
        if configured {
            assert!(
                diagnostics
                    .iter()
                    .all(|diagnostic| diagnostic["code"] != "unresolved_type_reference"),
                "{diagnostics:?}"
            );
        }
    }
}

#[test]
fn relative_library_configuration_changes_keep_the_original_editor_root() {
    let workspace = tempfile::tempdir().unwrap();
    let root = workspace.path().canonicalize().unwrap();
    let old = root.join("old-library");
    std::fs::create_dir(&old).unwrap();
    std::fs::write(old.join("Old.sysml"), "package Old;").unwrap();
    let new = tempfile::tempdir().unwrap();
    let new_root = new.path().canonicalize().unwrap();
    std::fs::write(
        new_root.join("New.sysml"),
        "package New { part def Thing; }",
    )
    .unwrap();
    let model = root.join("App.sysml");
    std::fs::write(
        &model,
        "package App { private import New::*; part thing : Thing; }",
    )
    .unwrap();
    let mut session =
        TestSession::new_with_env(&[("SPEC42_LSP_TEST_NO_STDLIB", std::path::Path::new("1"))]);
    session.initialize_with_root_and_options(
        "relative-library-change",
        Some(&url::Url::from_directory_path(&root).unwrap()),
        Some(json!({"libraryPaths":[old]})),
    );
    let uri = sysml_query::source::path_to_file_url(&model).unwrap();
    let initial = session.wait_for_diagnostics(uri.as_str());
    assert!(
        initial
            .iter()
            .any(|diagnostic| diagnostic["code"] == "unresolved_import"
                || diagnostic["code"] == "unresolved_import_target"
                || diagnostic["code"] == "unresolved_type_reference"),
        "{initial:?}"
    );
    let relative = format!("../{}", new_root.file_name().unwrap().to_str().unwrap());
    session.notify(
        "workspace/didChangeConfiguration",
        json!({"settings":{"spec42":{"libraryPaths":[relative]}}}),
    );
    assert!(session.wait_for_diagnostics(uri.as_str()).is_empty());
}
