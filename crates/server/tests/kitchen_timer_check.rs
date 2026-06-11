//! Smoke test: `perform_check` on a bundled example (same engine as `spec42 check` / MCP `spec42_check`).

mod common;

use std::path::PathBuf;

use common::with_isolated_data_dir;
use spec42::cli::{CheckArgs, Cli, OutputFormat};
use spec42::perform_check;

#[test]
fn kitchen_timer_example_validates() {
    with_isolated_data_dir(|| {
        kitchen_timer_example_validates_body();
    });
}

fn kitchen_timer_example_validates_body() {
    let example =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/timer/KitchenTimer.sysml");
    let example = example
        .canonicalize()
        .unwrap_or_else(|_| panic!("missing example file at {}", example.display()));

    let cli = Cli {
        config_path: None,
        library_paths: vec![],
        stdlib_path: None,
        domain_libraries_path: None,
        no_stdlib: false,
        stdio: false,
        command: None,
    };
    let args = CheckArgs {
        path: example,
        workspace_root: None,
        format: OutputFormat::Json,
        warnings_as_errors: false,
        baseline: None,
        strict_diagnostics: false,
    };

    let report = perform_check(&cli, &args).expect("validation should run");
    assert_eq!(
        report.summary.error_count, 0,
        "expected no errors in KitchenTimer example"
    );
}

#[test]
fn kitchen_timer_example_validates_with_explicit_workspace_root() {
    with_isolated_data_dir(|| {
        kitchen_timer_example_validates_with_explicit_workspace_root_body();
    });
}

fn kitchen_timer_example_validates_with_explicit_workspace_root_body() {
    let example =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/timer/KitchenTimer.sysml");
    let example = example
        .canonicalize()
        .unwrap_or_else(|_| panic!("missing example file at {}", example.display()));
    let workspace_root = example
        .parent()
        .map(|path| path.to_path_buf())
        .expect("example parent dir");

    let cli = Cli {
        config_path: None,
        library_paths: vec![],
        stdlib_path: None,
        domain_libraries_path: None,
        no_stdlib: false,
        stdio: false,
        command: None,
    };
    let args = CheckArgs {
        path: example,
        workspace_root: Some(workspace_root),
        format: OutputFormat::Json,
        warnings_as_errors: false,
        baseline: None,
        strict_diagnostics: false,
    };

    let report = perform_check(&cli, &args).expect("validation should run");
    assert_eq!(
        report.summary.error_count, 0,
        "expected no errors in KitchenTimer example with explicit workspace root"
    );
}
