//! Optional regression against the sysml-robot-vacuum-cleaner showcase model.
//! Set `SYSML_ROBOT_VACUUM_DIR` to the repository root (directory containing `model/`).

use spec42::cli::{CheckArgs, Cli, OutputFormat};
use spec42::perform_check;
use std::collections::HashMap;
use std::path::PathBuf;
use tower_lsp::lsp_types::NumberOrString;

fn diagnostic_code_counts(report: &kernel::ValidationReport) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for document in &report.documents {
        for diagnostic in &document.diagnostics {
            if let Some(NumberOrString::String(code)) = &diagnostic.code {
                *counts.entry(code.clone()).or_default() += 1;
            }
        }
    }
    counts
}

#[test]
#[ignore = "requires SYSML_ROBOT_VACUUM_DIR pointing at the robot vacuum showcase checkout"]
fn robot_vacuum_showcase_diagnostic_baseline() {
    let Some(root) = std::env::var_os("SYSML_ROBOT_VACUUM_DIR") else {
        return;
    };
    let root = PathBuf::from(root);
    let model_dir = root.join("model");
    if !model_dir.is_dir() {
        panic!(
            "SYSML_ROBOT_VACUUM_DIR must contain a model/ directory: {}",
            model_dir.display()
        );
    }

    let cli = Cli {
        config_path: None,
        library_paths: Vec::new(),
        stdlib_path: None,
        domain_libraries_path: None,
        no_stdlib: false,
        stdio: false,
        command: None,
    };
    let report = perform_check(
        &cli,
        &CheckArgs {
            path: model_dir,
            workspace_root: Some(root),
            format: OutputFormat::Json,
            warnings_as_errors: false,
            baseline: None,
            strict_diagnostics: false,
        },
    )
    .expect("robot vacuum validation report");

    let code_counts = diagnostic_code_counts(&report);

    assert_eq!(report.summary.error_count, 0, "expected zero errors");
    assert_eq!(
        code_counts
            .get("verification_case_invalid_shape")
            .copied()
            .unwrap_or(0),
        0,
        "verification cases with then-action and no explicit return are valid SysML v2 (S42-LIM-003)"
    );
    assert_eq!(
        code_counts
            .get("unresolved_pending_relationship")
            .copied()
            .unwrap_or(0),
        0,
        "unqualified verify requirement names must resolve via private import SystemRequirements::*"
    );
    assert_eq!(
        code_counts
            .get("unresolved_redefines_target")
            .copied()
            .unwrap_or(0),
        0,
        "specialized part local attributes must not emit unresolved_redefines_target"
    );
    assert_eq!(
        code_counts.get("unknown_unit_symbol").copied().unwrap_or(0),
        0,
        "MonetaryUnits::EUR should be recognized from bundled domain libraries"
    );
    assert_eq!(
        code_counts
            .get("analysis_evaluation_unresolved")
            .copied()
            .unwrap_or(0),
        0,
        "VerdictKind::pass verification returns should evaluate"
    );
    assert_eq!(
        code_counts
            .get("multiple_initial_states")
            .copied()
            .unwrap_or(0),
        0,
        "named transitions with first source must not count as initial transitions"
    );
}
