#![recursion_limit = "256"]

//! Protocol-neutral workspace build, snapshot, comparison and library management for Spec42.

pub mod cache;
pub mod catalog;
pub mod comparison;
pub mod engine;
pub mod error;
pub mod library;
pub mod parse_cache;
pub mod provider;
pub mod session;
pub mod snapshot;
pub mod version;

pub use catalog::{HostConfigFile, HostLibraryRequest, LibraryCatalog};
pub use comparison::{
    compare_snapshots, HostDiagnosticComparison, HostDiagnosticIdentity,
    HostDiagnosticRelatedInformation, HostDocumentDiagnosticComparison, IdentityPreservationStatus,
    SemanticComparisonReport,
};
pub use engine::{EngineBuilder, HostEngineMetadata, Spec42Engine};
pub use error::{WorkspaceError, WorkspaceResult};
pub use library::{
    bundle::LibraryBundleConfig,
    managed::{
        kpar_library_paths_from_data_dir, registry_configs, KparLibraryConfig, KparLibraryPaths,
        KparLibraryStatus,
    },
    resolve_explicit_library_path, resolve_library_closure,
    stdlib::{
        project_dirs, standard_library_paths_from_data_dir, StandardLibraryConfig,
        StandardLibraryPaths, StandardLibraryStatus,
    },
    LibraryArchive, LibraryBundle, LibraryClosureOptions, LibraryInstallRoot, LibraryPackageRoots,
    LibrarySource, LoadedLibraryFile, ResolvedExplicitLibrary, WorkspaceSource,
};
pub use provider::{
    ChangesetDocumentProvider, FileSystemDocumentProvider, HostFilesystemProvider,
    InMemoryDocumentProvider, SysmlDocument, SysmlDocumentProvider, SysmlDocumentSourceKind,
};
pub use semantic_publication::{
    PreparedPublication, PublicationBuildFailure, PublicationCoordinator, PublicationFailureStage,
};
pub use source_identity::{ContentDigest, RootDigest};

pub use session::{PublicationToken, RelinkToken, SessionLifecycle, WorkspaceSession};
pub use snapshot::discovery::{discover_target_files, path_to_file_url, resolve_workspace_root};
pub use snapshot::{
    apply_document_changes, enrich_document_hashes, CancellationToken, DocumentChanges,
    HostContext, HostPipelinePhase, HostResourceLimits, HostValidatedDocument,
    HostValidationReport, HostValidationSummary, HostWorkspaceSnapshot, Spec42ProjectionOutput,
    ValidationTiming, WorkspaceLoadRequest,
};
pub use version::{HostArtifactMetadata, HostSchemaVersions};
