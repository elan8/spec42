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
pub use context::HostContext;
pub use metadata::HostSnapshotMetadata;
pub use projection::{
    HostSemanticModelNode, HostSemanticModelRelationship, HostSemanticProjection,
};
pub use request::WorkspaceLoadRequest;
pub use validation::{
    HostValidatedDocument, HostValidationReport, HostValidationSummary,
};
