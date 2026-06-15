//! Smoke test: embedded Elan8 KPAR domain libraries materialize and resolve MonetaryUnits.

mod common;

use common::with_isolated_data_dir;
use spec42::cli::{CheckArgs, Cli, OutputFormat};
use spec42::domain_libraries::EMBEDDED_DOMAIN_LIBRARIES_ARCHIVE;
use spec42::{perform_check, perform_doctor};
use tempfile::TempDir;
use tower_lsp::lsp_types::NumberOrString;

const SMOKE_MODEL: &str = r#"package KparDomainLibrariesSmoke {
  private import MonetaryUnits::*;

  part def Robot {
    attribute bomCost : MonetaryAmount = 120 [EUR];
  }
}
"#;

fn diagnostic_codes(report: &kernel::ValidationReport) -> Vec<String> {
    report
        .documents
        .iter()
        .flat_map(|document| document.diagnostics.iter())
        .filter_map(|diagnostic| match diagnostic.code.as_ref()? {
            NumberOrString::String(code) => Some(code.clone()),
            NumberOrString::Number(code) => Some(code.to_string()),
        })
        .collect()
}

#[cfg(feature = "embed-domain-libraries")]
#[test]
fn embedded_kpar_domain_libraries_resolve_monetary_units() {
    if EMBEDDED_DOMAIN_LIBRARIES_ARCHIVE.is_empty() {
        eprintln!(
            "Skipping embedded_kpar_domain_libraries_resolve_monetary_units: \
             rebuild after `scripts/fetch-domain-libraries-bundle.sh` with \
             embed-domain-libraries enabled"
        );
        return;
    }

    with_isolated_data_dir(|| {
        let temp = TempDir::new().expect("temp workspace");
        let model_path = temp.path().join("kpar-domain-smoke.sysml");
        std::fs::write(&model_path, SMOKE_MODEL).expect("write smoke model");

        let cli = Cli {
            config_path: None,
            library_paths: vec![],
            stdlib_path: None,
            domain_libraries_path: None,
            no_stdlib: false,
            stdio: false,
            command: None,
        };

        let doctor = perform_doctor(&cli).expect("doctor");
        assert_eq!(
            doctor.domain_libraries_source_kind, "bundled",
            "expected bundled domain libraries, got {:?}",
            doctor.domain_libraries_source_kind
        );
        assert!(
            doctor.domain_libraries_status.is_installed,
            "expected installed domain libraries status: {:?}",
            doctor.domain_libraries_status
        );
        assert!(
            doctor.resolved_domain_libraries_path.is_some(),
            "expected materialized domain libraries path"
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
        for code in [
            "unresolved_import_target",
            "unresolved_type_reference",
            "unknown_unit_symbol",
        ] {
            assert!(
                !codes.iter().any(|actual| actual == code),
                "MonetaryUnits and EUR should resolve via embedded KPAR domain libraries: {codes:?}"
            );
        }
        assert_eq!(
            report.summary.error_count, 0,
            "expected no errors in KPAR domain libraries smoke model: {:?}",
            report.documents
        );
    });
}
