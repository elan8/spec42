//! Artifact-level ratchet for the shipped standard library (issue #135).
//!
//! `standard_library_admission.md` (the "Standard-library publication ratchet" snapshot step)
//! proves a small workspace resolves against the *curated* corpus under
//! `tests/snapshots/sysml.library/`. It does not touch the pinned KPAR archives the binary
//! actually ships. This test does: it materializes the embedded bundle exactly as a user's
//! install would, admits every document with `StandardLibrary` provenance into one canonical
//! publication, explicitly requests diagnostics for every library document (a normal
//! publication omits unopened library diagnostics), and asserts the bundle unpacks completely
//! and matches the reviewed diagnostic inventory.
//!
//! A bundle, parser, lowering, resolution or diagnostic change that alters the shipped standard
//! library's known inventory fails here rather than shipping silently.

#![cfg(feature = "embed-stdlib")]

use crate::common::with_isolated_data_dir;
use spec42::cli::Cli;
use spec42::perform_doctor;
use spec42::stdlib::EMBEDDED_STDLIB_ARCHIVE;
use workspace::{
    EngineBuilder, HostContext, HostFilesystemProvider, SourceKind, ValidationTiming,
    WorkspaceLoadRequest,
};

/// KPAR archives in `config/standard-library.json`. Update on a standard-library bump.
const EXPECTED_KPAR_COUNT: usize = 10;

/// Source documents the pinned `2026-04` bundle unpacks to. Update on a standard-library bump;
/// a change here is a deliberate inventory change that belongs in the same commit.
const EXPECTED_DOCUMENT_COUNT: usize = 94;

const EXPECTED_DIAGNOSTICS: &[(&str, usize)] =
    &[("ambiguous_reference", 2), ("unresolved_reference", 9)];

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
fn bundled_standard_library_diagnostic_inventory_is_ratcheted() {
    if EMBEDDED_STDLIB_ARCHIVE.is_empty() {
        eprintln!(
            "Skipping bundled_standard_library_diagnostic_inventory_is_ratcheted: \
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
        let stdlib_path = std::path::Path::new(stdlib_path);

        // Load the package roots as libraries in full. A normal workspace scan deliberately stops
        // at each nested `.project.json` boundary, so scanning the common parent would discover 94
        // validation targets without admitting any of them to the publication.
        let engine = EngineBuilder::default()
            .cache_dir(stdlib_path.join(".ratchet-cache"))
            .standard_library_path(stdlib_path)
            .build()
            .expect("standard-library engine");
        let provider = HostFilesystemProvider::from_paths_with_standard_library(
            stdlib_path,
            None,
            engine.package_roots(),
            &engine.library_catalog().stdlib.roots,
            engine.services().clone(),
        )
        .with_full_library_scan(true);
        let snapshot = engine
            .load_workspace(
                provider,
                WorkspaceLoadRequest::single_target(stdlib_path.into())
                    .with_validation_timing(ValidationTiming::Deferred),
                HostContext::default(),
            )
            .expect("load bundled standard library");

        assert_eq!(
            snapshot.documents().len(),
            EXPECTED_DOCUMENT_COUNT,
            "publication admitted {} bundled documents, expected {EXPECTED_DOCUMENT_COUNT}",
            snapshot.documents().len()
        );
        assert!(
            snapshot
                .documents()
                .iter()
                .all(|document| document.kind() == SourceKind::StandardLibrary),
            "every bundled document must retain StandardLibrary provenance"
        );

        let report = snapshot
            .ensure_validation()
            .expect("diagnose bundled standard library");

        assert_eq!(
            report.summary.document_count, EXPECTED_DOCUMENT_COUNT,
            "bundled standard library unpacked to {} documents, expected {EXPECTED_DOCUMENT_COUNT} \
             -- a deliberate inventory change updates the constant in this test",
            report.summary.document_count
        );

        let offenders: Vec<String> = report
            .documents
            .iter()
            .flat_map(|document| {
                let name = document.uri.rsplit('/').next().unwrap_or(&document.uri);
                document.diagnostics.iter().map(move |diagnostic| {
                    format!(
                        "{name}:{} {}: {}",
                        diagnostic.range.start.line + 1,
                        diagnostic.code,
                        diagnostic.message
                    )
                })
            })
            .collect();
        let mut actual = std::collections::BTreeMap::<String, usize>::new();
        for diagnostic in report
            .documents
            .iter()
            .flat_map(|document| &document.diagnostics)
        {
            *actual.entry(diagnostic.code.clone()).or_default() += 1;
        }
        let expected = EXPECTED_DIAGNOSTICS
            .iter()
            .map(|(code, count)| ((*code).to_owned(), *count))
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(report.summary.error_count, 2);
        assert_eq!(report.summary.warning_count, 9);
        assert_eq!(report.summary.information_count, 0);
        assert_eq!(
            actual,
            expected,
            "bundled standard-library diagnostic inventory changed:\n  {}",
            offenders.join("\n  ")
        );
    });
}
