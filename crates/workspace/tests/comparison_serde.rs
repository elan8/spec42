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
            engine_version: "0.33.0".to_string(),
            library_catalog_hash: "prev".to_string(),
            built_at: "2026-06-22T11:00:00Z".to_string(),
            document_digests: BTreeMap::new(),
        },
        next_artifact: workspace::HostArtifactMetadata {
            schema_versions: HostSchemaVersions::current(),
            engine_version: "0.33.0".to_string(),
            library_catalog_hash: "next".to_string(),
            built_at: "2026-06-22T12:00:00Z".to_string(),
            document_digests: BTreeMap::new(),
        },
        identity_preservation: IdentityPreservationStatus::IncompatibleEnvironment,
        elements: HostElementComparison::default(),
        relationships: Default::default(),
        diagnostics: Default::default(),
        facts: Default::default(),
        views: Default::default(),
    };

    let json = serde_json::to_string_pretty(&report).expect("serialize");
    let restored: SemanticComparisonReport = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(restored, report);
    assert_eq!(restored.schema_versions.comparison_schema_version, 2);
}

#[test]
fn persisted_schema_1_report_defaults_sections_added_in_schema_2() {
    let persisted_schema_1 = serde_json::json!({
        "schema_versions": {
            "artifact_metadata_version": 2,
            "projection_schema_version": 14,
            "renderer_compatibility_version": 1,
            "comparison_schema_version": 1
        },
        "compared_at": "2026-06-22T12:00:00Z",
        "previous_artifact": {
            "schema_versions": {
                "artifact_metadata_version": 2,
                "projection_schema_version": 14,
                "renderer_compatibility_version": 1,
                "comparison_schema_version": 1
            },
            "engine_version": "0.33.0",
            "library_catalog_hash": "catalog",
            "built_at": "2026-06-22T11:00:00Z",
            "document_digests": {}
        },
        "next_artifact": {
            "schema_versions": {
                "artifact_metadata_version": 2,
                "projection_schema_version": 14,
                "renderer_compatibility_version": 1,
                "comparison_schema_version": 1
            },
            "engine_version": "0.33.0",
            "library_catalog_hash": "catalog",
            "built_at": "2026-06-22T12:00:00Z",
            "document_digests": {}
        },
        "identity_preservation": "preserved",
        "elements": {
            "added": [], "removed": [],
            "changed": [{
                "identity": { "uri": "file:///demo.sysml", "qualified_name": "Demo::item" },
                "fields": [{ "field": "name", "previous": "before", "next": "after" }]
            }]
        },
        "relationships": {
            "added": [{ "source": "Demo::item", "target": "Demo::Thing", "kind": "typing" }],
            "removed": []
        },
        "diagnostics": {
            "by_document": {
                "file:///demo.sysml": {
                    "introduced": [{
                        "uri": "file:///demo.sysml", "code": "semantic.example",
                        "severity": "warning", "message": "example"
                    }],
                    "resolved": []
                }
            }
        },
        "views": {
            "catalog_added": [], "catalog_removed": [], "catalog_changed": [],
            "changed_view_payloads": []
        }
    });

    let restored: SemanticComparisonReport =
        serde_json::from_value(persisted_schema_1).expect("decode schema 1");
    assert_eq!(restored.schema_versions.comparison_schema_version, 1);
    assert_eq!(restored.facts, Default::default());
    assert!(restored.relationships.changed.is_empty());
    assert_eq!(restored.relationships.added[0].semantic_id, "");
    assert_eq!(restored.elements.changed[0].identity.semantic_id, "");
    let diagnostic = &restored.diagnostics.by_document["file:///demo.sysml"].introduced[0];
    assert_eq!(diagnostic.range, None);
    assert!(diagnostic.source.is_empty());
    assert!(diagnostic.related_information.is_empty());
    assert_eq!(
        restored.elements.changed[0].fields[0].previous,
        serde_json::json!("before")
    );
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
    assert!(report.relationships.added.is_empty());
    assert!(report.relationships.removed.is_empty());
    assert!(report.relationships.changed.is_empty());
    assert_eq!(report.facts, Default::default());
    assert_eq!(
        report.identity_preservation,
        IdentityPreservationStatus::Preserved
    );
}
