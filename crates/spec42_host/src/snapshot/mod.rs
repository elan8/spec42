//! Snapshot types and workspace loading.

mod build;
mod context;
mod discovery;
mod facts;
mod metadata;
mod projection;
mod request;
mod validation;

pub use build::{load_workspace_snapshot, HostWorkspaceSnapshot};
pub use context::{
    CancellationToken, HostContext, HostPipelinePhase, HostResourceLimits,
};
pub use metadata::HostArtifactMetadata;
pub use projection::{
    HostSemanticModelNode, HostSemanticModelRelationship, HostSemanticProjection,
};
pub use request::WorkspaceLoadRequest;
pub use validation::{
    HostValidatedDocument, HostValidationReport, HostValidationSummary,
};
