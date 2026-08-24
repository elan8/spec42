//! Snapshot types and workspace loading.

mod build;
mod changes;
mod context;
pub mod discovery;
mod metadata;
mod output;
mod request;
mod update;
mod validation;

pub use build::{load_workspace_snapshot, HostWorkspaceSnapshot, ValidationState};
pub use changes::{apply_document_changes, DocumentChanges};
pub use context::{CancellationToken, HostContext, HostPipelinePhase, HostResourceLimits};
pub use metadata::HostArtifactMetadata;
pub use output::Spec42ProjectionOutput;
pub use request::{ValidationTiming, WorkspaceLoadRequest};
pub use update::update_workspace_snapshot;
pub(crate) use validation::summarize as summarize_validated_documents;
pub use validation::{HostValidatedDocument, HostValidationReport, HostValidationSummary};
