#[path = "support/comparison_fixtures.rs"]
mod comparison_fixtures;

use comparison_fixtures::{load_snapshot, test_engine};
use spec42_host::compare_snapshots;
use tempfile::tempdir;

#[test]
fn view_catalog_change_is_reported_when_expose_changes() {
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
    view def structure {
        expose item;
    }
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
    part other : Thing;
    view def structure {
        expose item;
        expose other;
    }
}
"#,
    );

    let report = compare_snapshots(&previous, &next).expect("compare");

    let has_catalog_or_payload_change = !report.views.catalog_changed.is_empty()
        || !report.views.changed_view_payloads.is_empty()
        || !report.views.catalog_added.is_empty();
    assert!(
        has_catalog_or_payload_change || !report.elements.added.is_empty(),
        "view or element changes expected: {:?}",
        report.views
    );
}

#[test]
fn supported_view_payload_hash_changes_when_model_changes() {
    let cache = tempdir().expect("tempdir");
    let engine = test_engine(&cache);

    let base = r#"
package Demo {
    part def Thing;
    part item : Thing;
    view def overview {
        expose item;
    }
}
"#;

    let previous = load_snapshot(&engine, &cache, "Demo.sysml", base);

    let next = load_snapshot(
        &engine,
        &cache,
        "Demo.sysml",
        r#"
package Demo {
    part def Thing;
    part item : Thing;
    part extra : Thing;
    view def overview {
        expose item;
        expose extra;
    }
}
"#,
    );

    let report = compare_snapshots(&previous, &next).expect("compare");
    assert!(
        !report.views.changed_view_payloads.is_empty()
            || !report.elements.added.is_empty(),
        "payload or element change expected: views={:?} elements={:?}",
        report.views.changed_view_payloads,
        report.elements.added
    );
}
