use std::collections::BTreeMap;

use workspace::{HostArtifactMetadata, HostSchemaVersions};

#[test]
fn host_artifact_metadata_round_trips_through_json() {
    let mut document_hashes = BTreeMap::new();
    document_hashes.insert("file:///demo/A.sysml".to_string(), "abc123".to_string());
    document_hashes.insert("file:///demo/B.sysml".to_string(), "def456".to_string());

    let metadata = HostArtifactMetadata {
        schema_versions: HostSchemaVersions {
            artifact_metadata_version: 1,
            projection_schema_version: 1,
            renderer_compatibility_version: 1,
            comparison_schema_version: 1,
        },
        engine_version: "0.33.0".to_string(),
        library_catalog_hash: "catalog-hash".to_string(),
        built_at: "2026-06-22T12:34:56Z".to_string(),
        document_hashes,
    };

    let json = serde_json::to_string_pretty(&metadata).expect("serialize");
    let restored: HostArtifactMetadata = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(restored, metadata);
    assert_eq!(
        restored
            .document_hashes
            .get("file:///demo/A.sysml")
            .map(String::as_str),
        Some("abc123")
    );
}

#[test]
fn host_schema_versions_current_matches_constants() {
    let versions = HostSchemaVersions::current();
    assert_eq!(versions.artifact_metadata_version, 1);
    assert_eq!(versions.comparison_schema_version, 1);
}
