//! Versioned schema identifiers and persistable artifact metadata.

use std::collections::BTreeMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sysml_model::ContentDigest;

/// Schema v2 breaks compatibility with v1: `SysmlDocument.sha256: Option<String>` became
/// `content_digest: Option<ContentDigest>`, `HostArtifactMetadata.document_hashes` became
/// `document_digests: BTreeMap<String, ContentDigest>`, and `LibraryCatalog.content_hash`
/// became `root_digest: RootDigest` computed from verified content. Old v1 metadata (containing
/// the old field shapes) is rejected on load, never upgraded or defaulted (plan §5.3).
pub const ARTIFACT_METADATA_VERSION: u32 = 2;
/// Schema v17 adds the host evaluation query/value projection.
pub const PROJECTION_SCHEMA_VERSION: u32 = 17;
pub const RENDERER_COMPATIBILITY_VERSION: u32 = 1;
/// Schema v2 adds complete typed projection-fact and relationship-change comparison.
/// Deserializers retain defaults for v1's absent additive sections.
pub const COMPARISON_SCHEMA_VERSION: u32 = 2;

/// Version identifiers for host-persisted artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostSchemaVersions {
    pub artifact_metadata_version: u32,
    pub projection_schema_version: u32,
    pub renderer_compatibility_version: u32,
    pub comparison_schema_version: u32,
}

impl HostSchemaVersions {
    pub fn current() -> Self {
        Self {
            artifact_metadata_version: ARTIFACT_METADATA_VERSION,
            projection_schema_version: PROJECTION_SCHEMA_VERSION,
            renderer_compatibility_version: RENDERER_COMPATIBILITY_VERSION,
            comparison_schema_version: COMPARISON_SCHEMA_VERSION,
        }
    }
}

/// Persistable metadata describing an immutable workspace snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostArtifactMetadata {
    pub schema_versions: HostSchemaVersions,
    pub engine_version: String,
    pub library_catalog_hash: String,
    pub built_at: String,
    pub document_digests: BTreeMap<String, ContentDigest>,
}

impl HostArtifactMetadata {
    pub fn new(
        engine_version: impl Into<String>,
        library_catalog_hash: impl Into<String>,
        document_digests: BTreeMap<String, ContentDigest>,
    ) -> Self {
        Self {
            schema_versions: HostSchemaVersions::current(),
            engine_version: engine_version.into(),
            library_catalog_hash: library_catalog_hash.into(),
            built_at: rfc3339_timestamp(SystemTime::now()),
            document_digests,
        }
    }
}

/// Format a UTC timestamp as RFC3339 (`YYYY-MM-DDTHH:MM:SSZ`).
pub fn rfc3339_timestamp(time: SystemTime) -> String {
    let secs = time
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs();
    format_rfc3339_from_unix_secs(secs)
}

fn format_rfc3339_from_unix_secs(secs: u64) -> String {
    let days = secs / 86_400;
    let rem = secs % 86_400;
    let hour = rem / 3_600;
    let minute = (rem % 3_600) / 60;
    let second = rem % 60;

    let (year, month, day) = civil_from_days(days as i64);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if month <= 2 { 1 } else { 0 };
    (year, month as u32, day as u32)
}
