//! Snapshot metadata for reproducible host artifacts.

use std::collections::BTreeMap;
use std::time::SystemTime;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct HostSnapshotMetadata {
    pub engine_version: String,
    pub projection_schema_version: u32,
    pub renderer_compatibility_version: u32,
    pub library_catalog_hash: String,
    pub built_at: SystemTime,
    pub document_hashes: BTreeMap<String, String>,
}
