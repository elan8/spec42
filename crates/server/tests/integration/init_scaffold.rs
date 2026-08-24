//! `spec42 init` starter-workspace integration coverage.

use std::fs;

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
        let result = starter_workspace::scaffold(&root).expect("scaffold starter workspace");

        assert_eq!(result.files_written, 5);
        assert!(root.join("README.md").is_file());
        assert!(root.join("model/definitions/system.sysml").is_file());

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
fn scaffold_refuses_nonempty_targets_without_changing_them() {
    let temp = TempDir::new().expect("temp directory");
    let root = temp.path().join("existing-project");
    fs::create_dir_all(&root).expect("create existing target");
    let existing = root.join("keep.sysml");
    fs::write(&existing, "package Keep;\n").expect("write existing file");

    let error = starter_workspace::scaffold(&root).expect_err("non-empty target must be refused");

    assert!(error.contains("existing files are never overwritten"));
    assert_eq!(
        fs::read_to_string(&existing).expect("read existing file"),
        "package Keep;\n"
    );
    assert!(!root.join("README.md").exists());
}
