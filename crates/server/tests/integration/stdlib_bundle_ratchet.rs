//! Artifact-level ratchet for the shipped standard library (issue #135).
//!
//! `standard_library_admission.md` (the "Standard-library publication ratchet" snapshot step)
//! proves a small workspace resolves against the *curated* corpus under
//! `tests/snapshots/sysml.library/`. It does not touch the pinned KPAR archives the binary
//! actually ships. This test does: it materializes the embedded bundle exactly as a user's
//! install would, admits every document with `StandardLibrary` provenance into one canonical
//! publication, explicitly requests diagnostics for every library document (a normal
//! publication omits unopened library diagnostics), and asserts the bundle unpacks completely
//! and publishes with zero diagnostics of any severity.
//!
//! A bundle, parser, lowering, resolution or diagnostic change that regresses the shipped
//! standard library fails here rather than shipping silently.

#![cfg(feature = "embed-stdlib")]

use crate::common::with_isolated_data_dir;
use spec42::cli::{CheckArgs, Cli, OutputFormat};
use spec42::stdlib::EMBEDDED_STDLIB_ARCHIVE;
use spec42::{perform_check, perform_doctor};

/// KPAR archives in `config/standard-library.json`. Update on a standard-library bump.
const EXPECTED_KPAR_COUNT: usize = 10;

/// Source documents the pinned `2026-04` bundle unpacks to. Update on a standard-library bump;
/// a change here is a deliberate inventory change that belongs in the same commit.
const EXPECTED_DOCUMENT_COUNT: usize = 94;

fn base_cli() -> Cli {
    Cli {
        config_path: None,
        library_paths: vec![],
        stdlib_path: None,
        kpar_library_paths: Vec::new(),
        project_libraries: Vec::new(),
        disabled_kpar_libraries: Vec::new(),
        no_stdlib: false,
        stdio: false,
        command: None,
    }
}

#[test]
fn bundled_standard_library_publishes_with_zero_diagnostics() {
    if EMBEDDED_STDLIB_ARCHIVE.is_empty() {
        eprintln!(
            "Skipping bundled_standard_library_publishes_with_zero_diagnostics: \
             rebuild after `scripts/fetch-stdlib-bundle.sh` with embed-stdlib enabled"
        );
        return;
    }

    with_isolated_data_dir(|| {
        let cli = base_cli();

        // `perform_doctor` materializes the embedded bundle to the isolated data dir, the same
        // on-disk layout a managed install produces.
        let doctor = perform_doctor(&cli).expect("doctor");
        assert_eq!(
            doctor.stdlib_source_kind, "bundled",
            "expected the embedded bundle, got {:?}",
            doctor.stdlib_source_kind
        );
        assert!(
            doctor.standard_library_status.is_installed,
            "embedded bundle did not install: {:?}",
            doctor.standard_library_status
        );
        assert_eq!(
            doctor.stdlib_roots.len(),
            EXPECTED_KPAR_COUNT,
            "expected {EXPECTED_KPAR_COUNT} materialized KPAR roots, got {:?}",
            doctor.stdlib_roots
        );
        let stdlib_path = doctor
            .resolved_stdlib_path
            .as_deref()
            .expect("materialized standard-library path");

        // Checking the materialized root makes every bundled document a target: its overlap
        // with the configured library root is coalesced to `StandardLibrary` provenance, and
        // the batch pipeline then reports diagnostics for each one.
        let args = CheckArgs {
            path: stdlib_path.into(),
            workspace_root: None,
            format: OutputFormat::Json,
            warnings_as_errors: false,
            baseline: None,
            strict_diagnostics: false,
        };
        let report = perform_check(&cli, &args).expect("check bundled standard library");

        assert_eq!(
            report.summary.document_count, EXPECTED_DOCUMENT_COUNT,
            "bundled standard library unpacked to {} documents, expected {EXPECTED_DOCUMENT_COUNT} \
             -- a deliberate inventory change updates the constant in this test",
            report.summary.document_count
        );

        let offenders: Vec<String> = report
            .documents
            .iter()
            .filter(|document| !document.diagnostics.is_empty())
            .map(|document| {
                let name = document.uri.rsplit('/').next().unwrap_or(&document.uri);
                let codes: Vec<&str> = document
                    .diagnostics
                    .iter()
                    .map(|diagnostic| diagnostic.code.as_str())
                    .collect();
                format!("{name}: {codes:?}")
            })
            .collect();
        assert!(
            offenders.is_empty(),
            "bundled standard library is not diagnostic-clean ({} errors, {} warnings, {} info):\n  {}",
            report.summary.error_count,
            report.summary.warning_count,
            report.summary.information_count,
            offenders.join("\n  ")
        );
    });
}
