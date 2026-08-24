use std::fs;
use std::path::{Path, PathBuf};

use sysml_diagnostics::{severity_label, DiagnosticSeverity};

/// Every host names a severity the same way.
///
/// A severity label is reporting policy, so this crate owns it. The CLI text and JUnit reports
/// spelled `Information` as `"info"` while the comparison harness spelled it `"information"`, over
/// the same publication - a difference no reader could explain. The labels are pinned here.
#[test]
fn severity_labels_are_the_reported_spellings() {
    assert_eq!(severity_label(DiagnosticSeverity::Error), "error");
    assert_eq!(severity_label(DiagnosticSeverity::Warning), "warning");
    assert_eq!(severity_label(DiagnosticSeverity::Information), "info");
}

/// No consumer declares a second severity label table.
///
/// The divergence this replaced was two functions with the same name and different output. A
/// consumer that wants a label calls [`severity_label`]; declaring one is how the two spellings
/// come back.
#[test]
fn only_the_reporting_crate_declares_a_severity_label() {
    let crates_root = repository_root().join("crates");
    let owner = crates_root.join("sysml_diagnostics");
    let mut offenders = Vec::new();
    for source in rust_sources(&crates_root) {
        if source.starts_with(&owner) {
            continue;
        }
        let text = fs::read_to_string(&source).expect("read source");
        if text.contains("fn severity_label(") {
            offenders.push(source.display().to_string());
        }
    }
    assert!(
        offenders.is_empty(),
        "severity labelling belongs to sysml_diagnostics::severity_label; found declarations in {offenders:?}"
    );
}

fn rust_sources(directory: &Path) -> Vec<PathBuf> {
    let mut output = Vec::new();
    collect_rust_sources(directory, &mut output);
    output.sort();
    output
}

fn collect_rust_sources(directory: &Path, output: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if entry.file_type().map_or(true, |kind| kind.is_symlink()) {
            // A symlinked directory is scratch or tooling state, never a repository source,
            // and following one can loop.
            continue;
        }
        if path.is_dir() {
            if path.file_name().is_some_and(|name| name == "target" || name == ".cache") {
                continue;
            }
            collect_rust_sources(&path, output);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            output.push(path);
        }
    }
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("repository root")
        .to_path_buf()
}
