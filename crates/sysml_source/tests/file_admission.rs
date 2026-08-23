//! Guard 5 from `planning/C_host_boundaries.md`: SysML file admission has one owner.

use std::fs;
use std::path::{Path, PathBuf};

use sysml_source::is_sysml_like;

/// The extensions the system admits, and the case rule it admits them under.
///
/// Admission is ASCII case-insensitive, which is what the authority's own directory walk and the
/// library catalogue indexer already did. A consumer walk that skipped `Model.SysML` while the
/// authority admitted it would disagree with the authority about what the model contains.
#[test]
fn admission_is_case_insensitive_over_the_two_source_extensions() {
    assert!(is_sysml_like(Path::new("/w/Model.sysml")));
    assert!(is_sysml_like(Path::new("/w/Model.kerml")));
    assert!(is_sysml_like(Path::new("/w/Model.SysML")));
    assert!(is_sysml_like(Path::new("/w/Model.KerML")));
    assert!(!is_sysml_like(Path::new("/w/Model.txt")));
    assert!(!is_sysml_like(Path::new("/w/Model")));
    assert!(!is_sysml_like(Path::new("/w/sysml")));
}

/// Crates whose SysML extension literals are not an admission decision.
///
/// `kpar` owns an archive format whose manifest names the source extensions it packs, and this
/// crate declares them. Everything else asks [`is_sysml_like`].
const ADMISSION_LITERAL_ALLOWED_CRATES: [&str; 2] = ["kpar", "sysml_source"];

/// No consumer re-derives "is this a SysML file" from an extension literal.
///
/// Five copies with two different case-sensitivity behaviours is what this replaced. A literal in
/// a non-test source file is how a sixth begins.
#[test]
fn only_the_source_authority_decides_sysml_file_admission() {
    let crates_root = repository_root().join("crates");
    let mut offenders = Vec::new();
    for source in rust_sources(&crates_root) {
        let relative = source.strip_prefix(&crates_root).expect("under crates/");
        let crate_name = relative
            .components()
            .next()
            .map(|component| component.as_os_str().to_string_lossy().to_string())
            .unwrap_or_default();
        if ADMISSION_LITERAL_ALLOWED_CRATES.contains(&crate_name.as_str()) {
            continue;
        }
        // Tests name fixture files by extension; that is a fixture, not an admission rule.
        if relative
            .components()
            .any(|component| component.as_os_str() == "tests" || component.as_os_str() == "benches")
        {
            continue;
        }
        let text = fs::read_to_string(&source).expect("read source");
        let lines: Vec<&str> = text.lines().collect();
        for (index, _) in lines.iter().enumerate() {
            let window = lines[index..lines.len().min(index + WINDOW_LINES)].join(" ");
            if !names_both_source_extensions(&window) || !window_tests_an_extension(&window) {
                continue;
            }
            let tail = lines[index + 1..lines.len().min(index + WINDOW_LINES)].join(" ");
            if names_both_source_extensions(&tail) && window_tests_an_extension(&tail) {
                // Reported once, at the first line of the predicate.
                continue;
            }
            offenders.push(format!("{}:{}", source.display(), index + 1));
        }
    }
    assert!(
        offenders.is_empty(),
        "SysML file admission belongs to sysml_source::is_sysml_like; found extension tests at {offenders:?}"
    );
}

/// How many consecutive lines are considered one predicate.
///
/// An admission predicate names both extensions, and a rustfmt-wrapped one spreads them over a
/// few lines. Anything that names only one is not deciding admission: a diagnostic `source`
/// value, the `sysml` root namespace, and stem extraction all legitimately say "sysml" alone.
const WINDOW_LINES: usize = 4;

/// Whether the window compares a path's extension rather than merely naming the words.
///
/// Stem extraction (`strip_suffix(".sysml")`) and substring heuristics over configured paths both
/// name the extensions without deciding what the model admits.
fn window_tests_an_extension(window: &str) -> bool {
    ["extension", "ends_with", "eq_ignore_ascii_case", "matches!"]
        .iter()
        .any(|operator| window.contains(operator))
}

fn names_both_source_extensions(window: &str) -> bool {
    let lower = window.to_ascii_lowercase();
    let names_sysml = lower.contains("\"sysml\"") || lower.contains("\".sysml\"");
    let names_kerml = lower.contains("\"kerml\"") || lower.contains("\".kerml\"");
    names_sysml && names_kerml
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
        if path.is_dir() {
            if path.file_name().is_some_and(|name| name == "target") {
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
