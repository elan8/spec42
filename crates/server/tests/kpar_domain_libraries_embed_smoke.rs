//! Smoke tests: config-driven embedded KPAR libraries (domain + method).

mod common;

use common::with_isolated_data_dir;
use spec42::cli::{CheckArgs, Cli, OutputFormat};
use spec42::kpar_libraries::{embedded_archive, embedded_entry};
use spec42::{perform_check, perform_doctor};
use tempfile::TempDir;
use tower_lsp::lsp_types::NumberOrString;

const DOMAIN_SMOKE_MODEL: &str = r#"package KparDomainLibrariesSmoke {
  private import MonetaryUnits::*;

  part def Robot {
    attribute bomCost : MonetaryAmount = 120 [EUR];
  }
}
"#;

const METHOD_SMOKE_MODEL: &str = r#"package KparMethodLibrariesSmoke {
  private import Elan8RequirementManagement::*;

  attribute id : Identifier;
}
"#;

fn diagnostic_codes(report: &lsp_server::ValidationReport) -> Vec<String> {
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

fn empty_cli() -> Cli {
    Cli {
        config_path: None,
        library_paths: vec![],
        stdlib_path: None,
        kpar_library_paths: Vec::new(),
        no_stdlib: false,
        stdio: false,
        command: None,
    }
}

fn doctor_library<'a>(
    report: &'a spec42::DoctorReport,
    id: &str,
) -> Option<&'a spec42::environment::DoctorKparLibrary> {
    report.kpar_libraries.iter().find(|library| library.id == id)
}

#[cfg(feature = "embed-kpar-libraries")]
#[test]
fn embedded_kpar_domain_libraries_resolve_monetary_units() {
    let archive = embedded_archive("domain").unwrap_or(&[]);
    if archive.is_empty() {
        eprintln!(
            "Skipping embedded_kpar_domain_libraries_resolve_monetary_units: \
             rebuild after packing domain KPAR with embed-kpar-libraries enabled"
        );
        return;
    }
    assert!(
        embedded_entry("domain").is_some(),
        "domain must be present in generated KPAR registry"
    );

    with_isolated_data_dir(|| {
        let temp = TempDir::new().expect("temp workspace");
        let model_path = temp.path().join("kpar-domain-smoke.sysml");
        std::fs::write(&model_path, DOMAIN_SMOKE_MODEL).expect("write smoke model");

        let cli = empty_cli();
        let doctor = perform_doctor(&cli).expect("doctor");
        let domain = doctor_library(&doctor, "domain").expect("domain in doctor.kpar_libraries");
        assert_eq!(
            domain.source_kind, "bundled",
            "expected bundled domain libraries, got {:?}",
            domain.source_kind
        );
        assert!(
            domain.status.is_installed,
            "expected installed domain libraries status: {:?}",
            domain.status
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
                "MonetaryUnits and EUR should resolve via embedded domain KPAR: {codes:?}"
            );
        }
        assert_eq!(
            report.summary.error_count, 0,
            "expected no errors in domain libraries smoke model: {:?}",
            report.documents
        );
    });
}

#[cfg(feature = "embed-kpar-libraries")]
#[test]
fn embedded_kpar_method_libraries_resolve_elan8_requirement_management() {
    let archive = embedded_archive("method").unwrap_or(&[]);
    if archive.is_empty() {
        eprintln!(
            "Skipping embedded_kpar_method_libraries_resolve_elan8_requirement_management: \
             rebuild after packing method KPAR with embed-kpar-libraries enabled"
        );
        return;
    }
    assert!(
        embedded_entry("method").is_some(),
        "method must be present in generated KPAR registry"
    );

    with_isolated_data_dir(|| {
        let temp = TempDir::new().expect("temp workspace");
        let model_path = temp.path().join("kpar-method-smoke.sysml");
        std::fs::write(&model_path, METHOD_SMOKE_MODEL).expect("write smoke model");

        let cli = empty_cli();
        let doctor = perform_doctor(&cli).expect("doctor");
        let method = doctor_library(&doctor, "method").expect("method in doctor.kpar_libraries");
        assert_eq!(
            method.source_kind, "bundled",
            "expected bundled method libraries, got {:?}",
            method.source_kind
        );
        assert!(
            method.status.is_installed,
            "expected installed method libraries status: {:?}",
            method.status
        );
        let install = method.path.as_ref().expect("method install path");
        assert!(
            std::path::Path::new(install)
                .join("method")
                .join("Elan8Method.sysml")
                .is_file(),
            "expected method/Elan8Method.sysml under {install}"
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
            !codes.iter().any(|actual| actual == "unresolved_import_target"),
            "Elan8RequirementManagement should resolve via embedded method KPAR: {codes:?}"
        );
        assert_eq!(
            report.summary.error_count, 0,
            "expected no errors in method libraries smoke model: {:?}",
            report.documents
        );
    });
}
