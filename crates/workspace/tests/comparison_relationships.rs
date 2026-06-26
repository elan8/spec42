#[path = "support/comparison_fixtures.rs"]
mod comparison_fixtures;

use comparison_fixtures::{load_snapshot, test_engine};
use workspace::compare_snapshots;
use tempfile::tempdir;

#[test]
fn added_and_removed_relationships_are_reported() {
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
        !report.relationships.removed.is_empty(),
        "removed satisfy relationship expected: {:?}",
        report.relationships.removed
    );
    assert!(
        report
            .relationships
            .removed
            .iter()
            .any(|edge| edge.kind.eq_ignore_ascii_case("satisfy")),
        "satisfy edge should be removed: {:?}",
        report.relationships.removed
    );
    assert!(report.relationships.added.is_empty());
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
        !report.relationships.added.is_empty(),
        "added relationships expected: {:?}",
        report.relationships.added
    );
}
