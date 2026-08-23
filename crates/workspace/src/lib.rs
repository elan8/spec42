#![recursion_limit = "256"]

//! The batch host: engine, directory snapshots, validation, comparison and schema versions over
//! the `sysml_query` services. Library provisioning is `library_catalog`'s; closure, parsing and
//! publication are the authorities'.

pub mod comparison;
pub mod engine;
pub mod error;
pub mod provider;
pub mod snapshot;
pub mod validation;
pub mod version;

pub use comparison::{
    compare_snapshots, HostDiagnosticComparison, HostDiagnosticIdentity,
    HostDiagnosticRelatedInformation, HostDocumentDiagnosticComparison, IdentityPreservationStatus,
    SemanticComparisonReport,
};
pub use engine::{EngineBuilder, HostEngineMetadata, Spec42Engine};
pub use error::{WorkspaceError, WorkspaceResult};
pub use library_catalog::{
    HostConfigFile, HostLibraryRequest, LibraryCatalog, StandardLibraryConfig,
};
pub use provider::{ChangesetDocumentProvider, FileSystemDocumentProvider, HostFilesystemProvider};
pub use snapshot::discovery::{discover_target_files, path_to_file_url, resolve_workspace_root};
pub use snapshot::{
    apply_document_changes, CancellationToken, DocumentChanges, HostContext, HostPipelinePhase,
    HostResourceLimits, HostValidatedDocument, HostValidationReport, HostValidationSummary,
    HostWorkspaceSnapshot, Spec42ProjectionOutput, ValidationTiming, WorkspaceLoadRequest,
};
pub use sysml_query::library::{LibraryClosureOptions, LibraryRoot};
pub use sysml_query::publication::{PublicationBuildFailure, PublicationFailureStage};
pub use sysml_query::source::{
    ContentDigest, InMemoryProvider, RootDigest, SourceDocument, SourceKind, SourceProvider,
};
pub use validation::{
    built_workspace_input_from_snapshot, report_from_built_workspace, validate_paths,
    BuiltWorkspaceInput, PipelineHook, ValidationPipelineHook, ValidationRequest,
};
pub use version::{HostArtifactMetadata, HostSchemaVersions};
