//! Protocol-neutral host embedding API for Spec42.

pub mod catalog;
pub mod comparison;
pub mod engine;
pub mod error;
pub mod library;
pub mod provider;
pub mod robot_vacuum_perf;
pub mod snapshot;
pub mod version;

pub use catalog::{HostConfigFile, HostLibraryRequest, LibraryCatalog};
pub use comparison::{
    compare_snapshots, HostDiagnosticComparison, HostDiagnosticIdentity,
    HostDocumentDiagnosticComparison, HostElementChange, HostElementComparison,
    HostElementFieldChange, HostElementIdentity, HostRelationshipComparison,
    HostRelationshipIdentity, HostViewCatalogChange, HostViewCatalogEntry,
    HostViewCatalogFieldChange, HostViewComparison, HostViewPayloadChange,
    IdentityPreservationStatus, SemanticComparisonReport,
};
pub use engine::{EngineBuilder, HostEngineMetadata, Spec42Engine};
pub use error::{HostResult, Spec42HostError};
pub use library::{
    bundle::LibraryBundleConfig,
    domain::{
        domain_libraries_paths_from_data_dir, DomainLibrariesConfig, DomainLibrariesPaths,
        DomainLibrariesStatus,
    },
    resolve_explicit_library_path, LibraryArchive, LibraryBundle, LibraryInstallRoot,
    LibraryPackageRoots, LibrarySource, ResolvedExplicitLibrary,
    stdlib::{
        project_dirs, standard_library_paths_from_data_dir, StandardLibraryConfig,
        StandardLibraryPaths, StandardLibraryStatus,
    },
};
pub use provider::{
    ChangesetDocumentProvider, FileSystemDocumentProvider, HostFilesystemProvider,
    InMemoryDocumentProvider, SysmlDocument, SysmlDocumentProvider, SysmlDocumentSourceKind,
};
pub use snapshot::{
    apply_document_changes, CancellationToken, DocumentChanges, HostContext, HostPipelinePhase,
    HostResourceLimits, HostSemanticModelNode, HostSemanticModelRelationship,
    HostSemanticProjection, HostValidatedDocument, HostValidationReport, HostValidationSummary,
    HostWorkspaceSnapshot, WorkspaceLoadRequest,
};
pub use version::{HostArtifactMetadata, HostSchemaVersions};
