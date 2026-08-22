//! Semantic comparison between two immutable workspace snapshots.

mod diagnostics;
mod identity;

use std::time::SystemTime;

pub use diagnostics::{
    HostDiagnosticComparison, HostDiagnosticIdentity, HostDiagnosticRelatedInformation,
    HostDocumentDiagnosticComparison,
};
pub use identity::IdentityPreservationStatus;

use crate::error::WorkspaceResult;
use crate::snapshot::HostWorkspaceSnapshot;
use crate::version::{rfc3339_timestamp, HostArtifactMetadata, HostSchemaVersions};

/// Facts-only comparison report between two workspace snapshots.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SemanticComparisonReport {
    pub schema_versions: HostSchemaVersions,
    pub compared_at: String,
    pub previous_artifact: HostArtifactMetadata,
    pub next_artifact: HostArtifactMetadata,
    pub identity_preservation: IdentityPreservationStatus,
    pub diagnostics: HostDiagnosticComparison,
}

/// Compare two immutable workspace snapshots and return a versioned facts-only report.
pub fn compare_snapshots(
    previous: &HostWorkspaceSnapshot,
    next: &HostWorkspaceSnapshot,
) -> WorkspaceResult<SemanticComparisonReport> {
    let previous_artifact = previous.metadata().clone();
    let next_artifact = next.metadata().clone();
    let identity_preservation =
        identity::assess_identity_preservation(&previous_artifact, &next_artifact);

    let diagnostics = diagnostics::compare_diagnostics(previous.validation(), next.validation())?;

    Ok(SemanticComparisonReport {
        schema_versions: HostSchemaVersions::current(),
        compared_at: rfc3339_timestamp(SystemTime::now()),
        previous_artifact,
        next_artifact,
        identity_preservation,
        diagnostics,
    })
}
