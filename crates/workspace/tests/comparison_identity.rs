use workspace::{compare_snapshots, IdentityPreservationStatus};

#[path = "support/comparison_fixtures.rs"]
mod comparison_fixtures;

use comparison_fixtures::{load_snapshot, test_engine};
use tempfile::tempdir;

#[test]
fn document_set_changed_when_uri_set_differs() {
    let cache = tempdir().expect("tempdir");
    let engine = test_engine(&cache);

    let previous = load_snapshot(
        &engine,
        &cache,
        "A.sysml",
        r#"package A { part def Thing; }"#,
    );
    let next = load_snapshot(
        &engine,
        &cache,
        "B.sysml",
        r#"package B { part def Thing; }"#,
    );

    let report = compare_snapshots(&previous, &next).expect("compare");
    assert_eq!(
        report.identity_preservation,
        IdentityPreservationStatus::DocumentSetChanged
    );
}

#[test]
fn preserved_when_same_document_uri_set_and_catalog() {
    let cache = tempdir().expect("tempdir");
    let engine = test_engine(&cache);
    let content = r#"package Demo { part def Thing; }"#;

    let previous = load_snapshot(&engine, &cache, "Demo.sysml", content);
    let next = load_snapshot(&engine, &cache, "Demo.sysml", content);

    let report = compare_snapshots(&previous, &next).expect("compare");
    assert_eq!(
        report.identity_preservation,
        IdentityPreservationStatus::Preserved
    );
}
