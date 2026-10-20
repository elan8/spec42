//! Workspace/library overlap is an identity and host-configuration concern.
use super::harness::TestSession;
use serde_json::json;

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
