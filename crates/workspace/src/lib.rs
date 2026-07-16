//! Protocol-neutral workspace build, snapshot, comparison and library management for Spec42.

pub mod catalog;
pub mod comparison;
pub mod engine;
pub mod error;
pub mod incremental;
pub mod library;
pub mod library_graph_cache;
pub mod parse_cache;
pub mod provider;
pub mod robot_vacuum_perf;
pub mod semantic;
pub mod session;
pub mod snapshot;
pub mod version;
pub mod view_cache;

pub use catalog::{HostConfigFile, HostLibraryRequest, LibraryCatalog};
pub use comparison::{
    HostDiagnosticComparison, HostDiagnosticIdentity, HostDocumentDiagnosticComparison,
    HostElementChange, HostElementComparison, HostElementFieldChange, HostElementIdentity,
    HostRelationshipComparison, HostRelationshipIdentity, HostViewCatalogChange,
    HostViewCatalogEntry, HostViewCatalogFieldChange, HostViewComparison, HostViewPayloadChange,
    IdentityPreservationStatus, SemanticComparisonReport, compare_snapshots,
};
pub use engine::{EngineBuilder, HostEngineMetadata, Spec42Engine};
pub use error::{WorkspaceError, WorkspaceResult};
pub use incremental::{
    IncrementalWorkspace, WorkspaceUpdateMetrics, build_view_catalog, project_semantic_model,
    render_view, validate_workspace,
};
pub use library::{
    LibraryArchive, LibraryBundle, LibraryInstallRoot, LibraryPackageRoots, LibrarySource,
    ResolvedExplicitLibrary,
    bundle::LibraryBundleConfig,
    domain::{
        DomainLibrariesConfig, DomainLibrariesPaths, DomainLibrariesStatus,
        domain_libraries_paths_from_data_dir,
    },
    resolve_explicit_library_path,
    stdlib::{
        StandardLibraryConfig, StandardLibraryPaths, StandardLibraryStatus, project_dirs,
        standard_library_paths_from_data_dir,
    },
};
pub use provider::{
    ChangesetDocumentProvider, FileSystemDocumentProvider, HostFilesystemProvider,
    InMemoryDocumentProvider, SysmlDocument, SysmlDocumentProvider, SysmlDocumentSourceKind,
};
pub use semantic::{
    NodeId, RelationshipKind, SemanticGraph, SemanticNode, add_cross_document_edges_for_uri,
    build_graph_from_doc, evaluate_expressions, hover_markdown_for_node,
};
pub use session::{RelinkToken, SessionLifecycle, WorkspaceSession};
pub use snapshot::discovery::{discover_target_files, path_to_file_url, resolve_workspace_root};
pub use snapshot::{
    CancellationToken, DocumentChanges, HostContext, HostExpression, HostExpressionArgument,
    HostElementFacts, HostFeatureValue, HostMembershipKind, HostMultiplicity, HostPipelinePhase,
    HostRelationshipMetaclass, HostResourceLimits, HostSemanticModelNode,
    HostSemanticModelRelationship, HostSemanticProjection,
    HostValidatedDocument, HostValidationReport, HostValidationSummary, HostWorkspaceSnapshot,
    Spec42ProjectionOutput, ValidationTiming, WorkspaceLoadRequest, apply_document_changes,
    enrich_document_hashes,
};
pub use version::{HostArtifactMetadata, HostSchemaVersions};
pub use view_cache::{
    ViewCacheKey, ViewRenderCache, VisualizationBuildOutcome, visualization_response_is_cacheable,
    workspace_root_for_uri,
};
