//! Immutable workspace snapshot assembly.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use sysml_query::source::{SourceDocument, SourceProvider, Url};

use crate::engine::HostEngineMetadata;
use crate::error::{map_provider_error, WorkspaceError, WorkspaceResult};
use crate::snapshot::context::{HostContext, HostPipelinePhase};
use crate::snapshot::discovery::{discover_target_files, path_to_file_url, resolve_workspace_root};
use library_catalog::{
    resolve_project_manifest_dependencies, LibraryCatalog, ProjectDependencyResolution,
};
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
    documents: Vec<SourceDocument>,
    published_model: Arc<PublishedModel>,
    validation_report: OnceLock<HostValidationReport>,
    validation_target_files: Vec<PathBuf>,
    strict_diagnostics: bool,
    validation_timing: ValidationTiming,
    library_urls: Vec<Url>,
    library_paths: Vec<PathBuf>,
    workspace_root: PathBuf,
    project_dependencies: Vec<ProjectDependencyResolution>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationState<'a> {
    Deferred,
    Ready(&'a HostValidationReport),
}

impl HostWorkspaceSnapshot {
    pub fn metadata(&self) -> &HostArtifactMetadata {
        &self.metadata
    }

    pub fn artifact_metadata(&self) -> &HostArtifactMetadata {
        &self.metadata
    }

    pub fn documents(&self) -> &[SourceDocument] {
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

    /// Resolution of every dependency authored by this snapshot's `.project.json`.
    pub fn project_dependencies(&self) -> &[ProjectDependencyResolution] {
        &self.project_dependencies
    }

    pub fn validation(&self) -> ValidationState<'_> {
        self.validation_report
            .get()
            .map_or(ValidationState::Deferred, ValidationState::Ready)
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
    provider: impl SourceProvider,
    request: WorkspaceLoadRequest,
    context: &HostContext,
) -> WorkspaceResult<HostWorkspaceSnapshot> {
    context.check_continue(HostPipelinePhase::LoadingDocuments)?;
    let workspace_root =
        resolve_workspace_root(&request.targets, request.workspace_root.as_deref())?;
    let manifest_present = workspace_root.join(library_catalog::PROJECT_FILE).is_file();
    let project_dependencies = resolve_manifest_dependencies(&workspace_root, catalog)?;
    let selected_dependency_roots = project_dependencies
        .iter()
        .filter_map(|resolution| match resolution {
            ProjectDependencyResolution::Satisfied { package_roots, .. } => Some(package_roots),
            _ => None,
        })
        .flatten()
        .map(|root| root.canonicalize().unwrap_or_else(|_| root.clone()))
        .collect::<Vec<_>>();
    // A batch snapshot is all-or-nothing: a file the provider could not admit is an error here,
    // not a warning.
    let loaded = engine
        .source()
        .load(&provider)
        .and_then(|report| report.require_complete());
    let mut documents = match loaded {
        Err(_error) if context.cancellation.is_cancelled() => {
            return Err(WorkspaceError::cancelled());
        }
        Err(error) => return Err(map_provider_error(error)),
        Ok(documents) => documents,
    };
    if manifest_present {
        let candidate_roots = catalog
            .dependency_candidates
            .iter()
            .flat_map(|candidate| candidate.package_roots.iter())
            .map(|root| root.canonicalize().unwrap_or_else(|_| root.clone()))
            .collect::<Vec<_>>();
        documents.retain(|document| {
            let Ok(path) = document.uri().to_file_path() else {
                return true;
            };
            !candidate_roots.iter().any(|root| path.starts_with(root))
                || selected_dependency_roots
                    .iter()
                    .any(|root| path.starts_with(root))
        });
    }
    let total_bytes = documents.iter().map(|doc| doc.byte_len() as u64).sum();
    context.enforce_document_limits(documents.len(), total_bytes)?;
    context.check_continue(HostPipelinePhase::LoadingDocuments)?;

    let target_files = discover_target_files(&request.targets)?;

    let mut library_paths = engine.package_roots().to_vec();
    if manifest_present {
        let candidate_roots = catalog
            .dependency_candidates
            .iter()
            .flat_map(|candidate| candidate.package_roots.iter())
            .map(|root| root.canonicalize().unwrap_or_else(|_| root.clone()))
            .collect::<Vec<_>>();
        library_paths.retain(|root| {
            let canonical = root.canonicalize().unwrap_or_else(|_| root.clone());
            !candidate_roots.contains(&canonical)
        });
        for root in selected_dependency_roots {
            if !library_paths.contains(&root) {
                library_paths.push(root);
            }
        }
    }
    let library_urls = library_paths
        .iter()
        .map(|path| path_to_file_url(path.as_path()))
        .collect::<WorkspaceResult<Vec<_>>>()?;

    // Publish once per coherent snapshot. Every immutable semantic consumer below shares this
    // exact identity rather than independently rebuilding equivalent-looking model state.
    let published_model = engine
        .services()
        .publication
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
        .map(|doc| (doc.uri().to_string(), doc.digest()))
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
        project_dependencies,
    })
}

fn resolve_manifest_dependencies(
    workspace_root: &Path,
    catalog: &LibraryCatalog,
) -> WorkspaceResult<Vec<ProjectDependencyResolution>> {
    let manifest_path = workspace_root.join(library_catalog::PROJECT_FILE);
    if !manifest_path.is_file() {
        return Ok(Vec::new());
    }
    let resolutions =
        resolve_project_manifest_dependencies(&manifest_path, &catalog.dependency_candidates)
            .map_err(WorkspaceError::unresolved_library_environment)?;
    let failures: Vec<_> = resolutions
        .iter()
        .filter(|resolution| !matches!(resolution, ProjectDependencyResolution::Satisfied { .. }))
        .collect();
    if !failures.is_empty() {
        let states = serde_json::to_string(&failures).map_err(|error| {
            WorkspaceError::internal_invariant_failure(format!(
                "Could not serialize project dependency states: {error}"
            ))
        })?;
        return Err(WorkspaceError::unresolved_library_environment(format!(
            "Project dependencies from {} were not satisfied: {states}. Spec42 does not fetch dependency resources implicitly.",
            manifest_path.display()
        )));
    }
    Ok(resolutions)
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
    provider: impl SourceProvider,
    request: WorkspaceLoadRequest,
    context: HostContext,
) -> WorkspaceResult<Arc<HostWorkspaceSnapshot>> {
    let catalog = engine.library_catalog().clone();
    let metadata = engine.metadata().clone();
    let snapshot =
        build_workspace_snapshot(engine, &catalog, &metadata, provider, request, &context)?;
    Ok(Arc::new(snapshot))
}
