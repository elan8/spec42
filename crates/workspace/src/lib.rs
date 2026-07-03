//! Protocol-neutral workspace build, snapshot, comparison and library management for Spec42.

pub mod catalog;
pub mod semantic;
pub mod comparison;
pub mod engine;
pub mod error;
pub mod incremental;
pub mod library;
pub mod library_graph_cache;
pub mod parse_cache;
pub mod provider;
pub mod robot_vacuum_perf;
pub mod session;
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
pub use error::{WorkspaceResult, WorkspaceError};
pub use incremental::{
    build_view_catalog, project_semantic_model, render_view, validate_workspace,
    IncrementalWorkspace, WorkspaceUpdateMetrics,
};
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
pub use session::{RelinkToken, SessionLifecycle, WorkspaceSession};
pub use snapshot::{
    apply_document_changes, enrich_document_hashes, CancellationToken, DocumentChanges,
    HostContext, HostPipelinePhase, HostResourceLimits, HostSemanticModelNode,
    HostSemanticModelRelationship, HostSemanticProjection, HostValidatedDocument,
    HostValidationReport, HostValidationSummary, HostWorkspaceSnapshot, Spec42ProjectionOutput,
    ValidationTiming, WorkspaceLoadRequest,
};
pub use snapshot::discovery::{discover_target_files, path_to_file_url, resolve_workspace_root};
pub use version::{HostArtifactMetadata, HostSchemaVersions};
pub use semantic::{
    add_cross_document_edges_for_uri, build_graph_from_doc, evaluate_expressions,
    hover_markdown_for_node, NodeId, RelationshipKind, SemanticGraph, SemanticNode,
};
