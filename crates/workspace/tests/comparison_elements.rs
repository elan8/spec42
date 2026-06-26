#[path = "support/comparison_fixtures.rs"]
mod comparison_fixtures;

use comparison_fixtures::{load_snapshot, test_engine};
use workspace::compare_snapshots;
use tempfile::tempdir;

#[test]
fn added_and_removed_elements_are_reported() {
    let cache = tempdir().expect("tempdir");
    let engine = test_engine(&cache);

    let previous = load_snapshot(
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

    let next = load_snapshot(
        &engine,
        &cache,
        "Demo.sysml",
        r#"
package Demo {
    part def Thing;
    part def Other;
    part other : Other;
}
"#,
    );

    let report = compare_snapshots(&previous, &next).expect("compare");

    assert!(
        report.elements.removed.iter().any(|node| node.name == "item"),
        "removed part usage should be reported: {:?}",
        report.elements.removed
    );
    assert!(
        report.elements.added.iter().any(|node| node.name == "other"),
        "added part usage should be reported: {:?}",
        report.elements.added
    );
}

#[test]
fn renamed_element_reports_field_change() {
    let cache = tempdir().expect("tempdir");
    let engine = test_engine(&cache);

    let previous = load_snapshot(
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

    let next = load_snapshot(
        &engine,
        &cache,
        "Demo.sysml",
        r#"
package Demo {
    part def Thing;
    part widget : Thing;
}
"#,
    );

    let report = compare_snapshots(&previous, &next).expect("compare");

    assert!(
        report.elements.removed.iter().any(|node| node.name == "item"),
        "old usage removed: {:?}",
        report.elements.removed
    );
    assert!(
        report.elements.added.iter().any(|node| node.name == "widget"),
        "new usage added: {:?}",
        report.elements.added
    );
}
