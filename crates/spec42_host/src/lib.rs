//! Protocol-neutral host embedding API for Spec42.

pub mod catalog;
pub mod engine;
pub mod error;
pub mod library;
pub mod provider;
pub mod snapshot;

pub use catalog::{HostConfigFile, HostLibraryRequest, LibraryCatalog};
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
    HostContext, HostSemanticModelNode, HostSemanticModelRelationship, HostSemanticProjection,
    HostSnapshotMetadata, HostValidatedDocument, HostValidationReport, HostValidationSummary,
    HostWorkspaceSnapshot, WorkspaceLoadRequest,
};
