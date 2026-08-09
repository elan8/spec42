#[path = "support/comparison_fixtures.rs"]
mod comparison_fixtures;

use comparison_fixtures::{load_snapshot, test_engine};
use tempfile::tempdir;
use workspace::compare_snapshots;

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
        report
            .elements
            .removed
            .iter()
            .any(|node| node.name == "item"),
        "removed part usage should be reported: {:?}",
        report.elements.removed
    );
    assert!(
        report
            .elements
            .added
            .iter()
            .any(|node| node.name == "other"),
        "added part usage should be reported: {:?}",
        report.elements.added
    );
}

#[test]
fn renamed_element_is_removed_and_added_when_its_semantic_id_changes() {
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
    assert!(report
        .elements
        .removed
        .iter()
        .any(|node| node.name == "item"));
    assert!(report
        .elements
        .added
        .iter()
        .any(|node| node.name == "widget"));
    assert!(
        report.elements.changed.iter().any(|change| {
            change.identity.qualified_name == "Demo"
                && change.fields.iter().any(|field| field.field == "range")
        }),
        "the parent package range records the source-fidelity change: {:?}",
        report.elements.changed
    );
}

#[test]
fn identical_snapshot_has_no_element_churn() {
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
    assert!(report.elements.added.is_empty());
    assert!(report.elements.removed.is_empty());
    assert!(report.elements.changed.is_empty());
}
