#[path = "support/comparison_fixtures.rs"]
mod comparison_fixtures;

use comparison_fixtures::{load_snapshot, test_engine};
use workspace::compare_snapshots;
use tempfile::tempdir;

#[test]
fn introduced_parse_diagnostic_is_reported() {
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
    part def Thing
}
"#,
    );

    let report = compare_snapshots(&previous, &next).expect("compare");

    let introduced_count: usize = report
        .diagnostics
        .by_document
        .values()
        .map(|entry| entry.introduced.len())
        .sum();
    assert!(
        introduced_count > 0,
        "parse error should be introduced: {:?}",
        report.diagnostics
    );
}

#[test]
fn resolved_parse_diagnostic_is_reported() {
    let cache = tempdir().expect("tempdir");
    let engine = test_engine(&cache);

    let previous = load_snapshot(
        &engine,
        &cache,
        "Demo.sysml",
        r#"
package Demo {
    part def Thing
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
}
"#,
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
        "parse error should be resolved: {:?}",
        report.diagnostics
    );
}
