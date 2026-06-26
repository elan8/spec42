use workspace::{compare_snapshots, HostWorkspaceSnapshot};

pub fn assert_snapshot_parity(
    label: &str,
    expected: &HostWorkspaceSnapshot,
    actual: &HostWorkspaceSnapshot,
) {
    let expected_summary = &expected.validation().summary;
    let actual_summary = &actual.validation().summary;
    assert_eq!(
        expected_summary.document_count, actual_summary.document_count,
        "{label}: document_count mismatch"
    );
    assert_eq!(
        expected_summary.error_count, actual_summary.error_count,
        "{label}: error_count mismatch"
    );
    assert_eq!(
        expected_summary.warning_count, actual_summary.warning_count,
        "{label}: warning_count mismatch"
    );
    assert_eq!(
        expected_summary.information_count, actual_summary.information_count,
        "{label}: information_count mismatch"
    );

    let report = compare_snapshots(expected, actual).unwrap_or_else(|err| {
        panic!("{label}: compare_snapshots failed: {err}");
    });

    assert!(
        report.elements.added.is_empty()
            && report.elements.removed.is_empty()
            && report.elements.changed.is_empty(),
        "{label}: element diff {:?}",
        report.elements
    );
    assert!(
        report.relationships.added.is_empty() && report.relationships.removed.is_empty(),
        "{label}: relationship diff {:?}",
        report.relationships
    );
    assert!(
        report
            .diagnostics
            .by_document
            .values()
            .all(|doc| doc.introduced.is_empty() && doc.resolved.is_empty()),
        "{label}: diagnostic diff {:?}",
        report.diagnostics
    );
    assert!(
        report.views.catalog_added.is_empty()
            && report.views.catalog_removed.is_empty()
            && report.views.catalog_changed.is_empty()
            && report.views.changed_view_payloads.is_empty(),
        "{label}: view diff {:?}",
        report.views
    );
}