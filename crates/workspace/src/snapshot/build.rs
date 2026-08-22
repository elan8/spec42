//! Immutable workspace snapshot assembly.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use source_identity::ContentDigest;
use sysml_source::{SysmlDocument, SysmlDocumentProvider};
use url::Url;

use crate::catalog::LibraryCatalog;
use crate::engine::HostEngineMetadata;
use crate::error::{map_provider_error, WorkspaceError, WorkspaceResult};
use crate::snapshot::context::{HostContext, HostPipelinePhase};
use crate::snapshot::discovery::{discover_target_files, path_to_file_url, resolve_workspace_root};
use sysml_query::resolved_slice::PublishedModel;

use crate::snapshot::metadata::HostArtifactMetadata;
use crate::snapshot::output::Spec42ProjectionOutput;
use crate::snapshot::request::{ValidationTiming, WorkspaceLoadRequest};
use crate::snapshot::validation::{collect_host_validation_report, HostValidationReport};
use crate::Spec42Engine;

/// Immutable workspace snapshot built once and queried by hosts and server adapters.
#[derive(Debug)]
pub struct HostWorkspaceSnapshot {
    metadata: HostArtifactMetadata,
    documents: Vec<SysmlDocument>,
    published_model: Arc<PublishedModel>,
    validation_report: OnceLock<HostValidationReport>,
    validation_target_files: Vec<PathBuf>,
    strict_diagnostics: bool,
    validation_timing: ValidationTiming,
    library_urls: Vec<Url>,
    library_paths: Vec<PathBuf>,
    workspace_root: PathBuf,
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

    /// The immutable publication this snapshot validates from.
    pub fn published_model(&self) -> &PublishedModel {
        &self.published_model
    }

    /// A shared handle to the same publication, for a host that keeps it beyond this snapshot.
    pub fn published_model_arc(&self) -> Arc<PublishedModel> {
        Arc::clone(&self.published_model)
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
            &self.published_model,
            &self.validation_target_files,
            Some(self.workspace_root.as_path()),
            &self.library_paths,
            self.strict_diagnostics,
        )?;
        let _ = self.validation_report.set(report);
        Ok(self
            .validation_report
            .get()
            .expect("validation initialized"))
    }

    pub fn validation_timing(&self) -> ValidationTiming {
        self.validation_timing
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
            validation_report,
        })
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
    context.check_continue(HostPipelinePhase::LoadingDocuments)?;
    let mut documents = match provider.load_documents() {
        Err(_message) if context.cancellation.is_cancelled() => {
            return Err(WorkspaceError::cancelled());
        }
        Err(message) => return Err(map_provider_error(message)),
        Ok(documents) => documents,
    };
    enrich_document_hashes(&mut documents);
    let total_bytes = documents.iter().map(|doc| doc.content.len() as u64).sum();
    context.enforce_document_limits(documents.len(), total_bytes)?;
    context.check_continue(HostPipelinePhase::LoadingDocuments)?;

    let workspace_root =
        resolve_workspace_root(&request.targets, request.workspace_root.as_deref())?;
    let target_files = discover_target_files(&request.targets)?;

    let library_paths = engine.package_roots().to_vec();
    let library_urls = library_paths
        .iter()
        .map(|path| path_to_file_url(path.as_path()))
        .collect::<WorkspaceResult<Vec<_>>>()?;

    // Publish once per coherent snapshot. Every immutable semantic consumer below shares this
    // exact identity rather than independently rebuilding equivalent-looking model state.
    let published_model = engine
        .publication_coordinator()
        .publish(&documents, [])
        .map_err(|error| WorkspaceError::internal_invariant_failure(error.to_string()))?;

    context.check_continue(HostPipelinePhase::CollectingValidation)?;
    let validation_report = if request.validation_timing == ValidationTiming::Eager {
        init_validation_report(
            ValidationTiming::Eager,
            collect_host_validation_report(
                &published_model,
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

    let document_digests = documents
        .iter()
        .filter_map(|doc| {
            doc.content_digest
                .map(|digest| (doc.uri.to_string(), digest))
        })
        .collect::<BTreeMap<_, _>>();

    let snapshot_metadata = HostArtifactMetadata::new(
        metadata.engine_version.clone(),
        catalog.root_digest.to_string(),
        document_digests,
    );

    Ok(HostWorkspaceSnapshot {
        metadata: snapshot_metadata,
        documents,
        published_model,
        validation_report,
        validation_target_files: target_files,
        strict_diagnostics: request.strict_diagnostics,
        validation_timing: request.validation_timing,
        library_urls,
        library_paths,
        workspace_root,
    })
}

/// Normalizes each document's URI (Windows drive-letter case) and populates `content_digest`/
/// `byte_size`. Public so embedders computing [`HostArtifactMetadata`] directly off an
/// other publication paths reuse the same normalization instead of hashing un-normalized URIs —
/// see `path_to_file_url`'s doc comment for what silently diverging normalization once broke.
pub fn enrich_document_hashes(documents: &mut [SysmlDocument]) {
    for document in documents {
        // Normalize here so the graph and the canonicalized `target_urls` computed via
        // `path_to_file_url` (which also lowercases the Windows drive letter) key on the
        // same URI string; providers aren't required to normalize themselves.
        document.uri = language_service::uri::normalize_uri(&document.uri);
        let bytes = document.content.as_bytes();
        document.byte_size = Some(bytes.len() as i64);
        document.content_digest = Some(ContentDigest::of_bytes(bytes));
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
    let snapshot =
        build_workspace_snapshot(engine, &catalog, &metadata, provider, request, &context)?;
    Ok(Arc::new(snapshot))
}
