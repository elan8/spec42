//! Smoke test: embedded OMG KPAR stdlib materializes and resolves ScalarValues imports.

use crate::common::with_isolated_data_dir;
use spec42::cli::{CheckArgs, Cli, OutputFormat};
use spec42::stdlib::EMBEDDED_STDLIB_ARCHIVE;
use spec42::{perform_check, perform_doctor};
use tempfile::TempDir;

const SMOKE_MODEL: &str = r#"package KparStdlibSmoke {
  private import ScalarValues::Real;
  part def Vehicle {
    attribute mass : Real;
  }
}
"#;

fn diagnostic_codes(report: &workspace::HostValidationReport) -> Vec<String> {
    report
        .documents
        .iter()
        .flat_map(|document| document.diagnostics.iter())
        .map(|diagnostic| diagnostic.code.clone())
        .collect()
}

#[cfg(feature = "embed-stdlib")]
#[test]
fn embedded_kpar_stdlib_resolves_scalar_values_import() {
    if EMBEDDED_STDLIB_ARCHIVE.is_empty() {
        eprintln!(
            "Skipping embedded_kpar_stdlib_resolves_scalar_values_import: \
             rebuild after `scripts/fetch-stdlib-bundle.sh` with embed-stdlib enabled"
        );
        return;
    }

    with_isolated_data_dir(|| {
        let temp = TempDir::new().expect("temp workspace");
        let model_path = temp.path().join("kpar-smoke.sysml");
        std::fs::write(&model_path, SMOKE_MODEL).expect("write smoke model");

        let cli = Cli {
            config_path: None,
            library_paths: vec![],
            stdlib_path: None,
            kpar_library_paths: Vec::new(),
            disabled_kpar_libraries: Vec::new(),
            no_stdlib: false,
            stdio: false,
            command: None,
        };

        let doctor = perform_doctor(&cli).expect("doctor");
        assert!(
            doctor.stdlib_source_kind == "bundled"
                || doctor.stdlib_source_kind == "canonical-managed",
            "expected bundled stdlib, got {:?}",
            doctor.stdlib_source_kind
        );
        assert!(
            doctor.standard_library_status.is_installed,
            "expected installed stdlib status: {:?}",
            doctor.standard_library_status
        );
        assert!(
            !doctor.stdlib_roots.is_empty(),
            "expected materialized KPAR stdlib roots, got {:?}",
            doctor.stdlib_roots
        );

        let args = CheckArgs {
            path: model_path,
            workspace_root: None,
            format: OutputFormat::Json,
            warnings_as_errors: false,
            baseline: None,
            strict_diagnostics: false,
        };

        let report = perform_check(&cli, &args).expect("check");
        let codes = diagnostic_codes(&report);
        assert!(
            !codes.iter().any(|code| code == "unresolved_type_reference"),
            "ScalarValues::Real should resolve via embedded KPAR stdlib: {codes:?}"
        );
        assert!(
            !codes.iter().any(|code| code == "unresolved_import_target"),
            "ScalarValues import should resolve via embedded KPAR stdlib: {codes:?}"
        );
        assert_eq!(
            report.summary.error_count, 0,
            "expected no errors in KPAR stdlib smoke model: {:?}",
            report.documents
        );
    });
}

#[test]
fn scalar_values_import_unresolved_with_no_stdlib() {
    with_isolated_data_dir(|| {
        let temp = TempDir::new().expect("temp workspace");
        let model_path = temp.path().join("kpar-smoke.sysml");
        std::fs::write(&model_path, SMOKE_MODEL).expect("write smoke model");

        let cli = Cli {
            config_path: None,
            library_paths: vec![],
            stdlib_path: None,
            kpar_library_paths: Vec::new(),
            disabled_kpar_libraries: Vec::new(),
            no_stdlib: true,
            stdio: false,
            command: None,
        };
        let args = CheckArgs {
            path: model_path,
            workspace_root: None,
            format: OutputFormat::Json,
            warnings_as_errors: false,
            baseline: None,
            strict_diagnostics: false,
        };

        let report = perform_check(&cli, &args).expect("check");
        let codes = diagnostic_codes(&report);
        assert!(
            codes.iter().any(|code| code == "unresolved_type_reference"),
            "expected unresolved Real without stdlib, got: {codes:?}"
        );
    });
}
