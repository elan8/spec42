#[path = "support/comparison_fixtures.rs"]
mod comparison_fixtures;

use comparison_fixtures::{load_snapshot, test_engine};
use tempfile::tempdir;
use workspace::compare_snapshots;

#[test]
fn removed_satisfy_relationship_is_reported() {
    let cache = tempdir().expect("tempdir");
    let engine = test_engine(&cache);
    let previous = load_snapshot(
        &engine,
        &cache,
        "Demo.sysml",
        r#"
package Demo {
    requirement def Req;
    requirement req : Req;
    part def Part;
    part part : Part;
    satisfy req by part;
}
"#,
    );
    let next = load_snapshot(
        &engine,
        &cache,
        "Demo.sysml",
        r#"
package Demo {
    requirement def Req;
    requirement req : Req;
    part def Part;
    part part : Part;
}
"#,
    );

    let report = compare_snapshots(&previous, &next).expect("compare");
    assert!(
        report
            .relationships
            .removed
            .iter()
            .any(|edge| edge.kind.eq_ignore_ascii_case("satisfy")),
        "removed satisfy relationship expected: {:?}",
        report.relationships.removed
    );
}

#[test]
fn added_typing_relationship_is_reported() {
    let cache = tempdir().expect("tempdir");
    let engine = test_engine(&cache);
    let previous = load_snapshot(
        &engine,
        &cache,
        "Demo.sysml",
        r#"
package Demo {
    part def Thing;
}
"#,
    );
    let next = load_snapshot(
        &engine,
        &cache,
        "Demo.sysml",
        r#"
package Demo {
    part def Thing;
    part item : Thing;
}
"#,
    );

    let report = compare_snapshots(&previous, &next).expect("compare");
    assert!(
        report
            .relationships
            .added
            .iter()
            .any(|edge| edge.kind.eq_ignore_ascii_case("typing")),
        "added typing relationship expected: {:?}",
        report.relationships.added
    );
}

#[test]
fn identical_snapshot_has_no_relationship_churn() {
    let cache = tempdir().expect("tempdir");
    let engine = test_engine(&cache);
    let content = r#"
package Demo {
    part def Thing;
}
"#;
    let previous = load_snapshot(&engine, &cache, "Demo.sysml", content);
    let next = load_snapshot(&engine, &cache, "Demo.sysml", content);
    let report = compare_snapshots(&previous, &next).expect("compare");
    assert!(report.relationships.added.is_empty());
    assert!(report.relationships.removed.is_empty());
    assert!(report.relationships.changed.is_empty());
}
