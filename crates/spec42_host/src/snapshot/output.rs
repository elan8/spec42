//! Typed output returned by the embedding API.

use crate::snapshot::metadata::HostArtifactMetadata;
use crate::snapshot::projection::HostSemanticProjection;
use crate::snapshot::validation::HostValidationReport;

/// Typed result of projecting a workspace.
///
/// Replaces the previous opaque `Vec<Spec42Artifact>` JSON blobs so embedders
/// can work directly with Rust structs and choose their own persistence format.
#[derive(Debug, Clone)]
pub struct Spec42ProjectionOutput {
    pub metadata: HostArtifactMetadata,
    pub semantic_projection: HostSemanticProjection,
    pub validation_report: HostValidationReport,
}
