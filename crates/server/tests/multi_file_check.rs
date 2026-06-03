//! Smoke test: `perform_check` on a multi-file workspace (import + ref resolution).

use std::fs;

use spec42::cli::{CheckArgs, Cli, OutputFormat};
use spec42::perform_check;
use tempfile::TempDir;

#[test]
fn multi_file_workspace_validates_with_explicit_workspace_root() {
    let temp = TempDir::new().expect("temp workspace");
    let root = temp.path().to_path_buf();
    fs::write(
        root.join("defs.sysml"),
        r#"package Domain {
  part def CelestialBody;
}"#,
    )
    .expect("write defs");
    fs::write(
        root.join("usage.sysml"),
        r#"package Consumer {
  import Domain::**;
  part def Orbit {
    ref primary : CelestialBody;
  }
}"#,
    )
    .expect("write usage");

    let cli = Cli {
        config_path: None,
        library_paths: vec![],
        stdlib_path: None,
        no_stdlib: true,
        stdio: false,
        command: None,
    };
    let args = CheckArgs {
        path: root.clone(),
        workspace_root: Some(root),
        format: OutputFormat::Json,
    };

    let report = perform_check(&cli, &args).expect("validation should run");
    assert_eq!(
        report.summary.error_count, 0,
        "expected no errors in multi-file import workspace: {:?}",
        report.documents
    );
}

#[test]
fn multi_file_workspace_validates_when_checking_single_file_with_root() {
    let temp = TempDir::new().expect("temp workspace");
    let root = temp.path().to_path_buf();
    fs::write(
        root.join("defs.sysml"),
        r#"package Domain {
  part def Tree;
}"#,
    )
    .expect("write defs");
    let usage_path = root.join("usage.sysml");
    fs::write(
        &usage_path,
        r#"package Usage {
  import Domain::*;
  part def Imported {
    ref branch : Tree;
  }
}"#,
    )
    .expect("write usage");

    let cli = Cli {
        config_path: None,
        library_paths: vec![],
        stdlib_path: None,
        no_stdlib: true,
        stdio: false,
        command: None,
    };
    let args = CheckArgs {
        path: usage_path,
        workspace_root: Some(root),
        format: OutputFormat::Json,
    };

    let report = perform_check(&cli, &args).expect("validation should run");
    assert_eq!(
        report.summary.error_count, 0,
        "expected usage file check with workspace root to resolve imported Tree"
    );
}
