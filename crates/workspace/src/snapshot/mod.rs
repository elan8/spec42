//! Snapshot types and workspace loading.

mod build;
mod changes;
mod context;
pub mod discovery;
pub(crate) mod facts;
mod metadata;
mod output;
mod projection;
mod request;
mod update;
mod validation;

pub use build::{HostWorkspaceSnapshot, enrich_document_hashes, load_workspace_snapshot};
pub use changes::{DocumentChanges, apply_document_changes};
pub use context::{CancellationToken, HostContext, HostPipelinePhase, HostResourceLimits};
pub use metadata::HostArtifactMetadata;
pub use output::Spec42ProjectionOutput;
pub use projection::{
    HostElementFacts, HostExpression, HostExpressionArgument, HostFeatureProperties,
    HostFeatureValue, HostMembershipKind, HostMultiplicity, HostRelationshipMetaclass,
    HostSemanticModelNode, HostSemanticModelRelationship, HostSemanticProjection,
};
pub use request::{ValidationTiming, WorkspaceLoadRequest};
pub use update::update_workspace_snapshot;
pub use validation::{HostValidatedDocument, HostValidationReport, HostValidationSummary};
