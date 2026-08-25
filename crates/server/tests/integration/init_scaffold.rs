//! `spec42 init` starter-workspace integration coverage.

use std::fs;
use std::process::Command;

use crate::common::with_isolated_data_dir;
use spec42::cli::{CheckArgs, Cli, OutputFormat};
use spec42::{perform_check, starter_workspace};
use tempfile::TempDir;

fn no_stdlib_cli() -> Cli {
    Cli {
        config_path: None,
        library_paths: vec![],
        stdlib_path: None,
        kpar_library_paths: Vec::new(),
        project_libraries: Vec::new(),
        disabled_kpar_libraries: Vec::new(),
        no_stdlib: true,
        stdio: false,
        command: None,
    }
}

#[test]
fn scaffold_creates_a_workspace_that_current_check_validates() {
    with_isolated_data_dir(|| {
        let temp = TempDir::new().expect("temp directory");
        let root = temp.path().join("starter");
        let result =
            starter_workspace::scaffold(&root, Vec::new()).expect("scaffold starter workspace");

        assert_eq!(result.files_written, 6);
        assert!(root.join("README.md").is_file());
        assert!(root.join("model/definitions/system.sysml").is_file());
        let project: kpar::Project = serde_json::from_slice(
            &fs::read(root.join(".project.json")).expect("read project manifest"),
        )
        .expect("parse project manifest");
        assert_eq!(project.name, "starter");
        assert_eq!(project.version, "0.1.0");
        assert!(project.usage.is_empty());

        let report = perform_check(
            &no_stdlib_cli(),
            &CheckArgs {
                path: root.clone(),
                workspace_root: Some(root),
                format: OutputFormat::Json,
                warnings_as_errors: false,
                baseline: None,
                strict_diagnostics: false,
            },
        )
        .expect("starter workspace check should run");

        assert_eq!(
            report.summary.error_count, 0,
            "starter workspace must stay valid with the current check pipeline: {:?}",
            report.documents
        );
    });
}

#[test]
fn scaffold_promotes_nonempty_target_without_changing_existing_files() {
    let temp = TempDir::new().expect("temp directory");
    let root = temp.path().join("existing-project");
    fs::create_dir_all(&root).expect("create existing target");
    let existing = root.join("keep.sysml");
    fs::write(&existing, "package Keep;\n").expect("write existing file");

    let result = starter_workspace::scaffold(&root, Vec::new()).expect("promote existing model");

    assert_eq!(result.files_written, 1);
    assert_eq!(
        fs::read_to_string(&existing).expect("read existing file"),
        "package Keep;\n"
    );
    assert!(!root.join("README.md").exists());
    assert!(root.join(".project.json").is_file());
}

#[test]
fn scaffold_never_overwrites_an_existing_manifest() {
    let temp = TempDir::new().expect("temp directory");
    let root = temp.path().join("existing-project");
    fs::create_dir_all(&root).expect("create existing target");
    let manifest = root.join(".project.json");
    let authored = br#"{"name":"authored","version":"9.8.7"}"#;
    fs::write(&manifest, authored).expect("write authored manifest");

    let result =
        starter_workspace::scaffold(&root, Vec::new()).expect("existing project is initialized");

    assert_eq!(result.files_written, 0);
    assert_eq!(fs::read(manifest).expect("read manifest"), authored);
    assert!(!root.join("README.md").exists());
}

#[test]
fn init_cli_promotes_an_existing_model_and_preserves_its_manifest_on_repeat() {
    let temp = TempDir::new().expect("temp directory");
    let root = temp.path().join("existing-project");
    fs::create_dir_all(&root).expect("create existing target");
    let model = root.join("Keep.sysml");
    fs::write(&model, "package Keep;\n").expect("write existing model");

    let first = Command::new(env!("CARGO_BIN_EXE_spec42"))
        .args(["init", root.to_str().expect("utf-8 root")])
        .output()
        .expect("run init");
    assert!(
        first.status.success(),
        "init failed: {}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert_eq!(
        fs::read_to_string(&model).expect("read model"),
        "package Keep;\n"
    );
    let manifest = root.join(".project.json");
    assert!(manifest.is_file());
    let initialized: kpar::Project =
        serde_json::from_slice(&fs::read(&manifest).expect("read initialized manifest"))
            .expect("parse initialized manifest");
    assert!(
        !initialized.usage.is_empty(),
        "default init must pin the resolved bundled libraries"
    );
    assert!(initialized
        .usage
        .iter()
        .all(|usage| usage.version_constraint.is_some()));
    assert!(initialized.usage.iter().any(|usage| {
        usage.resource == "https://www.omg.org/spec/SysML/20250201/Systems-Library.kpar"
            && usage.version_constraint.as_deref() == Some("2.0.0")
    }));

    let authored = br#"{"name":"authored","version":"9.8.7"}"#;
    fs::write(&manifest, authored).expect("replace with authored manifest");
    let second = Command::new(env!("CARGO_BIN_EXE_spec42"))
        .args(["init", root.to_str().expect("utf-8 root")])
        .output()
        .expect("run repeated init");
    assert!(
        second.status.success(),
        "repeated init failed: {}",
        String::from_utf8_lossy(&second.stderr)
    );
    assert_eq!(fs::read(manifest).expect("read manifest"), authored);
}
