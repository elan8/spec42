//! Immutable workspace snapshot assembly.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Instant, SystemTime};

use language_service::InMemoryWorkspace;
use semantic_core::{
    build_render_snapshot, build_semantic_graph_from_documents, build_sysml_visualization_workspace,
    SemanticGraph, SysmlDocument, SysmlDocumentProvider, SysmlVisualizationResultDto,
    WorkspaceParsedDocument, WorkspaceRenderSnapshot,
};
use sha2::{Digest, Sha256};
use url::Url;

use crate::catalog::LibraryCatalog;
use crate::engine::HostEngineMetadata;
use crate::error::{HostResult, Spec42HostError};
use crate::snapshot::context::HostContext;
use crate::snapshot::discovery::{discover_target_files, path_to_file_url, resolve_workspace_root};
use crate::snapshot::facts::{collect_host_validation_report, project_host_semantic_model};
use crate::snapshot::metadata::HostSnapshotMetadata;
use crate::snapshot::projection::HostSemanticProjection;
use crate::snapshot::request::WorkspaceLoadRequest;
use crate::snapshot::validation::HostValidationReport;
use crate::Spec42Engine;

/// Immutable workspace snapshot built once and queried by hosts and server adapters.
#[derive(Debug)]
pub struct HostWorkspaceSnapshot {
    metadata: HostSnapshotMetadata,
    documents: Vec<SysmlDocument>,
    semantic_graph: SemanticGraph,
    parsed_documents: Vec<WorkspaceParsedDocument>,
    language_workspace: InMemoryWorkspace,
    render_snapshot: WorkspaceRenderSnapshot,
    validation_report: HostValidationReport,
    semantic_projection: HostSemanticProjection,
    library_urls: Vec<Url>,
    library_paths: Vec<PathBuf>,
    workspace_root: PathBuf,
    workspace_root_uri: Url,
    build_instant: Instant,
}

impl HostWorkspaceSnapshot {
    pub fn metadata(&self) -> &HostSnapshotMetadata {
        &self.metadata
    }

    pub fn documents(&self) -> &[SysmlDocument] {
        &self.documents
    }

    pub fn semantic_graph(&self) -> &SemanticGraph {
        &self.semantic_graph
    }

    pub fn parsed_documents(&self) -> &[WorkspaceParsedDocument] {
        &self.parsed_documents
    }

    pub fn library_urls(&self) -> &[Url] {
        &self.library_urls
    }

    pub fn library_paths(&self) -> &[PathBuf] {
        &self.library_paths
    }

    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    pub fn workspace_root_uri(&self) -> &Url {
        &self.workspace_root_uri
    }

    pub fn validation(&self) -> &HostValidationReport {
        &self.validation_report
    }

    pub fn semantic_projection(&self) -> &HostSemanticProjection {
        &self.semantic_projection
    }

    pub fn language_workspace(&self) -> &InMemoryWorkspace {
        &self.language_workspace
    }

    pub fn view_catalog(&self) -> &WorkspaceRenderSnapshot {
        &self.render_snapshot
    }

    pub fn prepare_view(
        &self,
        view: &str,
        selected_view: Option<&str>,
    ) -> Result<SysmlVisualizationResultDto, Spec42HostError> {
        build_sysml_visualization_workspace(
            &self.semantic_graph,
            &self.parsed_documents,
            &self.library_urls,
            &self.workspace_root_uri,
            view,
            selected_view,
            self.build_instant,
        )
        .map_err(|message| Spec42HostError::unresolved_library_environment(message))
    }
}

pub(crate) fn build_workspace_snapshot(
    engine: &Spec42Engine,
    catalog: &LibraryCatalog,
    metadata: &HostEngineMetadata,
    provider: impl SysmlDocumentProvider,
    request: WorkspaceLoadRequest,
    _context: HostContext,
) -> HostResult<HostWorkspaceSnapshot> {
    let build_instant = Instant::now();

    let mut documents = provider.load_documents().map_err(Spec42HostError::from)?;
    enrich_document_hashes(&mut documents);

    let workspace_root = resolve_workspace_root(
        &request.targets,
        request.workspace_root.as_deref(),
    )?;
    let target_files = discover_target_files(&request.targets)?;

    let library_paths = engine.package_roots().to_vec();
    let library_urls = library_paths
        .iter()
        .map(|path| path_to_file_url(path.as_path()))
        .collect::<HostResult<Vec<_>>>()?;

    let workspace_root_uri = path_to_file_url(&workspace_root)?;

    let (semantic_graph, parsed_documents) =
        build_semantic_graph_from_documents(&documents).map_err(Spec42HostError::from)?;

    let language_workspace =
        InMemoryWorkspace::from_documents(documents.clone()).map_err(Spec42HostError::from)?;

    let render_snapshot = build_render_snapshot(
        &semantic_graph,
        &parsed_documents,
        &library_urls,
        &workspace_root_uri,
        1,
    )
    .map_err(Spec42HostError::from)?;

    let validation_report = collect_host_validation_report(
        &semantic_graph,
        &documents,
        &library_urls,
        &target_files,
        Some(workspace_root.as_path()),
        &library_paths,
        request.strict_diagnostics,
    )?;

    let semantic_projection = project_host_semantic_model(&semantic_graph, &target_files)?;

    let document_hashes = documents
        .iter()
        .map(|doc| (doc.uri.to_string(), doc.sha256.clone().unwrap_or_default()))
        .collect::<BTreeMap<_, _>>();

    let snapshot_metadata = HostSnapshotMetadata {
        engine_version: metadata.engine_version.clone(),
        projection_schema_version: metadata.projection_schema_version,
        renderer_compatibility_version: metadata.renderer_compatibility_version,
        library_catalog_hash: catalog.content_hash.clone(),
        built_at: SystemTime::now(),
        document_hashes,
    };

    Ok(HostWorkspaceSnapshot {
        metadata: snapshot_metadata,
        documents,
        semantic_graph,
        parsed_documents,
        language_workspace,
        render_snapshot,
        validation_report,
        semantic_projection,
        library_urls,
        library_paths,
        workspace_root,
        workspace_root_uri,
        build_instant,
    })
}

fn enrich_document_hashes(documents: &mut [SysmlDocument]) {
    for document in documents {
        let bytes = document.content.as_bytes();
        document.byte_size = Some(bytes.len() as i64);
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        document.sha256 = Some(format!("{:x}", hasher.finalize()));
    }
}

pub fn load_workspace_snapshot(
    engine: &Spec42Engine,
    provider: impl SysmlDocumentProvider,
    request: WorkspaceLoadRequest,
    context: HostContext,
) -> HostResult<Arc<HostWorkspaceSnapshot>> {
    let catalog = engine.library_catalog().clone();
    let metadata = engine.metadata().clone();
    let snapshot = build_workspace_snapshot(
        engine,
        &catalog,
        &metadata,
        provider,
        request,
        context,
    )?;
    Ok(Arc::new(snapshot))
}
