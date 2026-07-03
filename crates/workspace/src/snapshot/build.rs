//! Immutable workspace snapshot assembly.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use std::time::Instant;

use language_service::InMemoryWorkspace;
use sysml_model::{
    build_render_snapshot, build_sysml_visualization_from_render_snapshot, empty_merged_ibd,
    full_ibd_for_render_snapshot, visualization_build_options, IbdBuildScope, IbdDataDto,
    SemanticGraph, SysmlDocument, SysmlDocumentProvider, SysmlVisualizationResultDto,
    WorkspaceParsedDocument, WorkspaceRenderSnapshot,
};
use sha2::{Digest, Sha256};
use url::Url;

use crate::catalog::LibraryCatalog;
use crate::engine::HostEngineMetadata;
use crate::error::{
    map_language_service_error, map_provider_error, map_render_snapshot_error, map_view_error,
    WorkspaceResult, WorkspaceError,
};
use crate::snapshot::context::{HostContext, HostPipelinePhase};
use crate::snapshot::discovery::{discover_target_files, path_to_file_url, resolve_workspace_root};
use crate::snapshot::facts::{collect_host_validation_report, project_host_semantic_model};
use crate::snapshot::metadata::HostArtifactMetadata;
use crate::snapshot::output::Spec42ProjectionOutput;
use crate::snapshot::projection::HostSemanticProjection;
use crate::snapshot::request::{ValidationTiming, WorkspaceLoadRequest};
use crate::snapshot::validation::HostValidationReport;
use crate::{IncrementalWorkspace, Spec42Engine};

/// Immutable workspace snapshot built once and queried by hosts and server adapters.
#[derive(Debug)]
pub struct HostWorkspaceSnapshot {
    metadata: HostArtifactMetadata,
    documents: Vec<SysmlDocument>,
    semantic_graph: SemanticGraph,
    parsed_documents: Vec<WorkspaceParsedDocument>,
    language_workspace: InMemoryWorkspace,
    render_snapshot: WorkspaceRenderSnapshot,
    validation_report: OnceLock<HostValidationReport>,
    validation_target_files: Vec<PathBuf>,
    strict_diagnostics: bool,
    validation_timing: ValidationTiming,
    semantic_projection: HostSemanticProjection,
    library_urls: Vec<Url>,
    library_paths: Vec<PathBuf>,
    workspace_root: PathBuf,
    workspace_root_uri: Url,
    build_instant: Instant,
    full_ibd_cache: OnceLock<IbdDataDto>,
}

impl HostWorkspaceSnapshot {
    pub fn metadata(&self) -> &HostArtifactMetadata {
        &self.metadata
    }

    pub fn artifact_metadata(&self) -> &HostArtifactMetadata {
        &self.metadata
    }

    pub fn documents(&self) -> &[SysmlDocument] {
        &self.documents
    }

    pub fn semantic_graph(&self) -> &SemanticGraph {
        &self.semantic_graph
    }

    pub fn semantic_graph_arc(&self) -> SemanticGraph {
        self.semantic_graph.clone()
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
        self.validation_report
            .get()
            .unwrap_or(empty_validation_report())
    }

    pub fn validation_ready(&self) -> bool {
        self.validation_report.get().is_some()
    }

    pub fn ensure_validation(&self) -> WorkspaceResult<&HostValidationReport> {
        if let Some(report) = self.validation_report.get() {
            return Ok(report);
        }
        let report = collect_host_validation_report(
            &self.semantic_graph,
            &self.documents,
            &self.library_urls,
            &self.validation_target_files,
            Some(self.workspace_root.as_path()),
            &self.library_paths,
            self.strict_diagnostics,
        )?;
        let _ = self.validation_report.set(report);
        Ok(self.validation_report.get().expect("validation initialized"))
    }

    pub fn validation_timing(&self) -> ValidationTiming {
        self.validation_timing
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

    /// Consume the snapshot and return a typed projection output.
    ///
    /// Ensures validation has run, then moves the typed structs into a
    /// [`Spec42ProjectionOutput`] so the caller can persist or inspect them
    /// without going through JSON.
    pub fn into_projection_output(self) -> WorkspaceResult<Spec42ProjectionOutput> {
        let validation_report = self.ensure_validation()?.clone();
        Ok(Spec42ProjectionOutput {
            metadata: self.metadata,
            semantic_projection: self.semantic_projection,
            validation_report,
        })
    }

    pub fn prepare_view(
        &self,
        view: &str,
        selected_view: Option<&str>,
    ) -> Result<SysmlVisualizationResultDto, WorkspaceError> {
        let options = visualization_build_options(view);
        let full_ibd = if options.ibd_build_scope == IbdBuildScope::ViewExposedPackages
            && (view == "general-view"
                || (view == "interconnection-view" && options.slim_interconnection_payload))
        {
            empty_merged_ibd()
        } else {
            self.full_ibd_cache
                .get_or_init(|| {
                    full_ibd_for_render_snapshot(
                        &self.semantic_graph,
                        &self.render_snapshot,
                        None,
                    )
                })
                .clone()
        };
        build_sysml_visualization_from_render_snapshot(
            &self.semantic_graph,
            &self.parsed_documents,
            &self.render_snapshot,
            view,
            selected_view,
            self.build_instant,
            full_ibd,
            options,
        )
        .map_err(|message| map_view_error(view, message))
    }
}

pub(crate) fn build_workspace_snapshot(
    engine: &Spec42Engine,
    catalog: &LibraryCatalog,
    metadata: &HostEngineMetadata,
    provider: impl SysmlDocumentProvider,
    request: WorkspaceLoadRequest,
    context: &HostContext,
) -> WorkspaceResult<HostWorkspaceSnapshot> {
    let build_instant = Instant::now();

    context.check_continue(HostPipelinePhase::LoadingDocuments)?;
    let mut documents = match provider.load_documents() {
        Err(_message) if context.cancellation.is_cancelled() => {
            return Err(WorkspaceError::cancelled());
        }
        Err(message) => return Err(map_provider_error(message)),
        Ok(documents) => documents,
    };
    enrich_document_hashes(&mut documents);
    let total_bytes = documents
        .iter()
        .map(|doc| doc.content.len() as u64)
        .sum();
    context.enforce_document_limits(documents.len(), total_bytes)?;
    context.check_continue(HostPipelinePhase::LoadingDocuments)?;

    let workspace_root = resolve_workspace_root(
        &request.targets,
        request.workspace_root.as_deref(),
    )?;
    let target_files = discover_target_files(&request.targets)?;

    let library_paths = engine.package_roots().to_vec();
    let library_urls = library_paths
        .iter()
        .map(|path| path_to_file_url(path.as_path()))
        .collect::<WorkspaceResult<Vec<_>>>()?;

    let workspace_root_uri = path_to_file_url(&workspace_root)?;

    context.check_continue(HostPipelinePhase::BuildingGraph)?;
    let mut incremental_workspace = IncrementalWorkspace::new();
    incremental_workspace.load(&documents);
    let semantic_graph = incremental_workspace.graph();
    let parsed_documents = incremental_workspace.documents();
    context.enforce_graph_limits(
        semantic_graph.node_ids_by_qualified_name.len(),
        semantic_graph.graph.edge_count(),
    )?;
    context.check_continue(HostPipelinePhase::BuildingGraph)?;

    context.check_continue(HostPipelinePhase::BuildingLanguageWorkspace)?;
    let language_workspace = InMemoryWorkspace::from_graph_and_documents(
        semantic_graph.clone(),
        parsed_documents.clone(),
        &documents,
    )
    .map_err(map_language_service_error)?;
    context.check_continue(HostPipelinePhase::BuildingLanguageWorkspace)?;

    context.check_continue(HostPipelinePhase::BuildingViewCatalog)?;
    let render_snapshot = build_render_snapshot(
        &semantic_graph,
        &parsed_documents,
        &library_urls,
        &workspace_root_uri,
        1,
    )
    .map_err(map_render_snapshot_error)?;
    context.check_continue(HostPipelinePhase::BuildingViewCatalog)?;

    context.check_continue(HostPipelinePhase::CollectingValidation)?;
    let validation_report = if request.validation_timing == ValidationTiming::Eager {
        init_validation_report(
            ValidationTiming::Eager,
            collect_host_validation_report(
                &semantic_graph,
                &documents,
                &library_urls,
                &target_files,
                Some(workspace_root.as_path()),
                &library_paths,
                request.strict_diagnostics,
            )?,
        )?
    } else {
        OnceLock::new()
    };
    context.check_continue(HostPipelinePhase::CollectingValidation)?;

    context.check_continue(HostPipelinePhase::ProjectingModel)?;
    let semantic_projection = project_host_semantic_model(&semantic_graph, &target_files)?;
    context.check_continue(HostPipelinePhase::ProjectingModel)?;

    let document_hashes = documents
        .iter()
        .map(|doc| (doc.uri.to_string(), doc.sha256.clone().unwrap_or_default()))
        .collect::<BTreeMap<_, _>>();

    let snapshot_metadata = HostArtifactMetadata::new(
        metadata.engine_version.clone(),
        catalog.content_hash.clone(),
        document_hashes,
    );

    Ok(HostWorkspaceSnapshot {
        metadata: snapshot_metadata,
        documents,
        semantic_graph,
        parsed_documents,
        language_workspace,
        render_snapshot,
        validation_report,
        validation_target_files: target_files,
        strict_diagnostics: request.strict_diagnostics,
        validation_timing: request.validation_timing,
        semantic_projection,
        library_urls,
        library_paths,
        workspace_root,
        workspace_root_uri,
        build_instant,
        full_ibd_cache: OnceLock::new(),
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn assemble_host_workspace_snapshot(
    metadata: &HostEngineMetadata,
    catalog: &LibraryCatalog,
    documents: Vec<SysmlDocument>,
    semantic_graph: SemanticGraph,
    parsed_documents: Vec<WorkspaceParsedDocument>,
    language_workspace: InMemoryWorkspace,
    render_snapshot: WorkspaceRenderSnapshot,
    validation_report: OnceLock<HostValidationReport>,
    validation_target_files: Vec<PathBuf>,
    strict_diagnostics: bool,
    validation_timing: ValidationTiming,
    semantic_projection: HostSemanticProjection,
    library_urls: Vec<Url>,
    library_paths: Vec<PathBuf>,
    workspace_root: PathBuf,
    workspace_root_uri: Url,
    build_instant: Instant,
) -> HostWorkspaceSnapshot {
    let document_hashes = documents
        .iter()
        .map(|doc| (doc.uri.to_string(), doc.sha256.clone().unwrap_or_default()))
        .collect::<BTreeMap<_, _>>();

    let snapshot_metadata = HostArtifactMetadata::new(
        metadata.engine_version.clone(),
        catalog.content_hash.clone(),
        document_hashes,
    );

    HostWorkspaceSnapshot {
        metadata: snapshot_metadata,
        documents,
        semantic_graph,
        parsed_documents,
        language_workspace,
        render_snapshot,
        validation_report,
        validation_target_files,
        strict_diagnostics,
        validation_timing,
        semantic_projection,
        library_urls,
        library_paths,
        workspace_root,
        workspace_root_uri,
        build_instant,
        full_ibd_cache: OnceLock::new(),
    }
}

pub(crate) fn enrich_document_hashes(documents: &mut [SysmlDocument]) {
    for document in documents {
        // Normalize here so the graph and the canonicalized `target_urls` computed via
        // `path_to_file_url` (which also lowercases the Windows drive letter) key on the
        // same URI string; providers aren't required to normalize themselves.
        document.uri = language_service::uri::normalize_uri(&document.uri);
        let bytes = document.content.as_bytes();
        document.byte_size = Some(bytes.len() as i64);
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        document.sha256 = Some(format!("{:x}", hasher.finalize()));
    }
}

fn empty_validation_report() -> &'static HostValidationReport {
    static EMPTY: OnceLock<HostValidationReport> = OnceLock::new();
    EMPTY.get_or_init(HostValidationReport::default)
}

pub(crate) fn init_validation_report(
    timing: ValidationTiming,
    eager_report: HostValidationReport,
) -> WorkspaceResult<OnceLock<HostValidationReport>> {
    let slot = OnceLock::new();
    if timing == ValidationTiming::Eager {
        slot.set(eager_report).map_err(|_| {
            WorkspaceError::internal_invariant_failure("validation report slot already initialized")
        })?;
    }
    Ok(slot)
}

pub fn load_workspace_snapshot(
    engine: &Spec42Engine,
    provider: impl SysmlDocumentProvider,
    request: WorkspaceLoadRequest,
    context: HostContext,
) -> WorkspaceResult<Arc<HostWorkspaceSnapshot>> {
    let catalog = engine.library_catalog().clone();
    let metadata = engine.metadata().clone();
    let snapshot = build_workspace_snapshot(
        engine,
        &catalog,
        &metadata,
        provider,
        request,
        &context,
    )?;
    Ok(Arc::new(snapshot))
}
