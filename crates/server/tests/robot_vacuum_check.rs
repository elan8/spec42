//! R9 zero-warning gate: `perform_check` on the pinned robot-vacuum showcase
//! (same engine path as `spec42 check`).

#[path = "../../../tests/fixtures/robot_vacuum_fixture.rs"]
mod robot_vacuum_fixture;
#[path = "common/mod.rs"]
mod common;

use common::with_isolated_data_dir;
use robot_vacuum_fixture::require_robot_vacuum_fixture;
use spec42::cli::{CheckArgs, Cli, OutputFormat};
use spec42::perform_check;

#[test]
#[ignore = "CI fetches the pin; locally: FORCE_ROBOT_VACUUM_FETCH=1 bash scripts/fetch-robot-vacuum-cleaner.sh && cargo test -p server --test robot_vacuum_check -- --ignored"]
fn robot_vacuum_check_has_zero_errors_and_warnings() {
    with_isolated_data_dir(|| {
        let (root, model_dir) = require_robot_vacuum_fixture();

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
            path: model_dir,
            workspace_root: Some(root),
            format: OutputFormat::Json,
            warnings_as_errors: false,
            baseline: None,
            strict_diagnostics: false,
        };

        let report = perform_check(&cli, &args).expect("validation should run");
        assert!(
            report.summary.document_count > 0,
            "expected validated documents"
        );
        assert_eq!(
            report.summary.error_count, 0,
            "robot-vacuum must have zero errors; summary={:?}, sample={:?}",
            report.summary,
            report
                .documents
                .iter()
                .flat_map(|doc| doc.diagnostics.iter().map(|d| (&d.code, &d.message)))
                .take(20)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            report.summary.warning_count, 0,
            "robot-vacuum must have zero warnings (R9); summary={:?}, sample={:?}",
            report.summary,
            report
                .documents
                .iter()
                .flat_map(|doc| doc.diagnostics.iter().map(|d| (&d.code, &d.message)))
                .take(20)
                .collect::<Vec<_>>()
        );
    });
}
