//! Identity preservation status derived from snapshot artifact metadata.

use crate::version::HostArtifactMetadata;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityPreservationStatus {
    /// Same library catalog hash, engine version, and document URI set.
    Preserved,
    /// `library_catalog_hash` and/or `engine_version` differ between snapshots.
    IncompatibleEnvironment,
    /// Workspace document URI set changed (add, remove, or rename).
    DocumentSetChanged,
}

pub(crate) fn assess_identity_preservation(
    previous: &HostArtifactMetadata,
    next: &HostArtifactMetadata,
) -> IdentityPreservationStatus {
    if previous.library_catalog_hash != next.library_catalog_hash
        || previous.engine_version != next.engine_version
    {
        return IdentityPreservationStatus::IncompatibleEnvironment;
    }

    let previous_uris: std::collections::BTreeSet<_> = previous.document_hashes.keys().collect();
    let next_uris: std::collections::BTreeSet<_> = next.document_hashes.keys().collect();
    if previous_uris != next_uris {
        return IdentityPreservationStatus::DocumentSetChanged;
    }

    IdentityPreservationStatus::Preserved
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::version::{HostArtifactMetadata, HostSchemaVersions};

    #[test]
    fn incompatible_environment_when_catalog_hash_differs() {
        let previous = sample_metadata("catalog-a", "file:///a.sysml");
        let next = sample_metadata("catalog-b", "file:///a.sysml");
        assert_eq!(
            assess_identity_preservation(&previous, &next),
            IdentityPreservationStatus::IncompatibleEnvironment
        );
    }

    #[test]
    fn preserved_when_only_content_hash_differs() {
        let mut previous = sample_metadata("catalog", "file:///a.sysml");
        let mut next = sample_metadata("catalog", "file:///a.sysml");
        previous
            .document_hashes
            .insert("file:///a.sysml".into(), "hash-a".into());
        next.document_hashes
            .insert("file:///a.sysml".into(), "hash-b".into());
        assert_eq!(
            assess_identity_preservation(&previous, &next),
            IdentityPreservationStatus::Preserved
        );
    }

    fn sample_metadata(catalog_hash: &str, uri: &str) -> HostArtifactMetadata {
        HostArtifactMetadata {
            schema_versions: HostSchemaVersions::current(),
            engine_version: "0.32.0".to_string(),
            library_catalog_hash: catalog_hash.to_string(),
            built_at: "2026-06-22T10:00:00Z".to_string(),
            document_hashes: BTreeMap::from([(uri.to_string(), "hash".to_string())]),
        }
    }
}
