
use crate::comparison_fixtures::{load_snapshot, test_engine};
use tempfile::tempdir;
use workspace::compare_snapshots;

#[test]
fn introduced_parse_diagnostic_is_reported() {
    let cache = tempdir().expect("tempdir");
    let engine = test_engine(&cache);
    let previous = load_snapshot(
        &engine,
        &cache,
        "Demo.sysml",
        "package Demo { part def Thing; }",
    );
    let next = load_snapshot(&engine, &cache, "Demo.sysml", "package Demo { part def; }");

    let report = compare_snapshots(&previous, &next).expect("compare");
    let introduced_count: usize = report
        .diagnostics
        .by_document
        .values()
        .map(|entry| entry.introduced.len())
        .sum();
    assert!(
        introduced_count > 0,
        "introduced diagnostics: {:?}",
        report.diagnostics
    );
}

#[test]
fn resolved_parse_diagnostic_is_reported() {
    let cache = tempdir().expect("tempdir");
    let engine = test_engine(&cache);
    let previous = load_snapshot(&engine, &cache, "Demo.sysml", "package Demo { part def; }");
    let next = load_snapshot(
        &engine,
        &cache,
        "Demo.sysml",
        "package Demo { part def Thing; }",
    );

    let report = compare_snapshots(&previous, &next).expect("compare");
    let resolved_count: usize = report
        .diagnostics
        .by_document
        .values()
        .map(|entry| entry.resolved.len())
        .sum();
    assert!(
        resolved_count > 0,
        "resolved diagnostics: {:?}",
        report.diagnostics
    );
}

#[test]
fn identical_valid_snapshots_have_no_diagnostic_churn() {
    let cache = tempdir().expect("tempdir");
    let engine = test_engine(&cache);
    let content = "package Demo { part def Thing; }";
    let previous = load_snapshot(&engine, &cache, "Demo.sysml", content);
    let next = load_snapshot(&engine, &cache, "Demo.sysml", content);
    let report = compare_snapshots(&previous, &next).expect("compare");
    assert!(report.diagnostics.by_document.is_empty());
}
