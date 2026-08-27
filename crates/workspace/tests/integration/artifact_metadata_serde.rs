use std::collections::BTreeMap;

use workspace::{ContentDigest, HostArtifactMetadata, HostSchemaVersions};

#[test]
fn host_artifact_metadata_round_trips_through_json() {
    let mut document_digests = BTreeMap::new();
    document_digests.insert(
        "file:///demo/A.sysml".to_string(),
        ContentDigest::of_bytes(b"abc123"),
    );
    document_digests.insert(
        "file:///demo/B.sysml".to_string(),
        ContentDigest::of_bytes(b"def456"),
    );

    let metadata = HostArtifactMetadata {
        schema_versions: HostSchemaVersions {
            artifact_metadata_version: 2,
            projection_schema_version: 1,
            renderer_compatibility_version: 1,
            comparison_schema_version: 2,
        },
        engine_version: "0.33.0".to_string(),
        library_catalog_hash: "catalog-hash".to_string(),
        built_at: "2026-06-22T12:34:56Z".to_string(),
        document_digests,
    };

    let json = serde_json::to_string_pretty(&metadata).expect("serialize");
    let restored: HostArtifactMetadata = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(restored, metadata);
    assert_eq!(
        restored.document_digests.get("file:///demo/A.sysml"),
        Some(&ContentDigest::of_bytes(b"abc123"))
    );
}

#[test]
fn host_schema_versions_current_matches_constants() {
    let versions = HostSchemaVersions::current();
    assert_eq!(versions.artifact_metadata_version, 2);
    assert_eq!(versions.projection_schema_version, 17);
    assert_eq!(versions.comparison_schema_version, 2);
}

/// Plan §5.3: old (schema v1) metadata used a plain string `document_hashes` map with
/// unprefixed hex digests and no typed `content_digest`/`root_digest`. That shape must be
/// rejected on deserialize, not silently upgraded or defaulted.
#[test]
fn old_v1_metadata_shape_is_rejected_not_upgraded() {
    let old_v1_json = r#"{
        "schema_versions": {
            "artifact_metadata_version": 1,
            "projection_schema_version": 17,
            "renderer_compatibility_version": 1,
            "comparison_schema_version": 2
        },
        "engine_version": "0.33.0",
        "library_catalog_hash": "catalog-hash",
        "built_at": "2026-06-22T12:34:56Z",
        "document_hashes": {
            "file:///demo/A.sysml": "abc123"
        }
    }"#;

    let result: Result<HostArtifactMetadata, _> = serde_json::from_str(old_v1_json);
    assert!(
        result.is_err(),
        "old v1 metadata (document_hashes with unprefixed hex) must fail to deserialize \
         into the current HostArtifactMetadata shape, not silently succeed"
    );
}

/// A `document_digests` map using the old unprefixed-hex digest form (rather than the current
/// `blake3:<64 hex>` text form) must also be rejected: the digest parser never accepts
/// unprefixed/legacy hex.
#[test]
fn document_digests_with_unprefixed_legacy_hex_is_rejected() {
    let json = r#"{
        "schema_versions": {
            "artifact_metadata_version": 2,
            "projection_schema_version": 17,
            "renderer_compatibility_version": 1,
            "comparison_schema_version": 2
        },
        "engine_version": "0.33.0",
        "library_catalog_hash": "catalog-hash",
        "built_at": "2026-06-22T12:34:56Z",
        "document_digests": {
            "file:///demo/A.sysml": "0000000000000000000000000000000000000000000000000000000000000000"
        }
    }"#;

    let result: Result<HostArtifactMetadata, _> = serde_json::from_str(json);
    assert!(result.is_err());
}
