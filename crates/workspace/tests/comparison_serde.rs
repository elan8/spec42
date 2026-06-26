use std::collections::BTreeMap;

use workspace::{
    compare_snapshots, HostElementComparison, HostSchemaVersions, IdentityPreservationStatus,
    SemanticComparisonReport,
};

#[path = "support/comparison_fixtures.rs"]
mod comparison_fixtures;

use comparison_fixtures::{load_snapshot, test_engine};
use tempfile::tempdir;

#[test]
fn semantic_comparison_report_round_trips_through_json() {
    let report = SemanticComparisonReport {
        schema_versions: HostSchemaVersions::current(),
        compared_at: "2026-06-22T12:00:00Z".to_string(),
        previous_artifact: workspace::HostArtifactMetadata {
            schema_versions: HostSchemaVersions::current(),
            engine_version: "0.32.0".to_string(),
            library_catalog_hash: "prev".to_string(),
            built_at: "2026-06-22T11:00:00Z".to_string(),
            document_hashes: BTreeMap::new(),
        },
        next_artifact: workspace::HostArtifactMetadata {
            schema_versions: HostSchemaVersions::current(),
            engine_version: "0.32.0".to_string(),
            library_catalog_hash: "next".to_string(),
            built_at: "2026-06-22T12:00:00Z".to_string(),
            document_hashes: BTreeMap::new(),
        },
        identity_preservation: IdentityPreservationStatus::IncompatibleEnvironment,
        elements: HostElementComparison::default(),
        relationships: Default::default(),
        diagnostics: Default::default(),
        views: Default::default(),
    };

    let json = serde_json::to_string_pretty(&report).expect("serialize");
    let restored: SemanticComparisonReport = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(restored, report);
    assert_eq!(restored.schema_versions.comparison_schema_version, 1);
}

#[test]
fn identical_snapshots_produce_empty_element_diff() {
    let cache = tempdir().expect("tempdir");
    let engine = test_engine(&cache);
    let content = r#"
package Demo {
    part def Thing;
    part item : Thing;
}
"#;
    let previous = load_snapshot(&engine, &cache, "Demo.sysml", content);
    let next = load_snapshot(&engine, &cache, "Demo.sysml", content);
    let report = compare_snapshots(&previous, &next).expect("compare");

    assert!(report.elements.added.is_empty());
    assert!(report.elements.removed.is_empty());
    assert!(report.elements.changed.is_empty());
    assert_eq!(
        report.identity_preservation,
        IdentityPreservationStatus::Preserved
    );
}
