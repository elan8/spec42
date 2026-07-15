//! Incremental and full-rebuild snapshot updates.

use std::sync::Arc;
use std::time::Instant;

use language_service::InMemoryWorkspace;
use sysml_model::{SemanticGraph, SysmlDocument, WorkspaceParsedDocument, build_render_snapshot};

use crate::error::{WorkspaceResult, map_language_service_error, map_render_snapshot_error};
use crate::provider::InMemoryDocumentProvider;
use crate::snapshot::build::{
    HostWorkspaceSnapshot, assemble_host_workspace_snapshot, build_workspace_snapshot,
};
use crate::snapshot::changes::{DocumentChanges, apply_document_changes, is_workspace_document};
use crate::snapshot::context::{HostContext, HostPipelinePhase};
use crate::snapshot::discovery::discover_target_files;
use crate::snapshot::facts::{collect_host_validation_report, project_host_semantic_model};
use crate::snapshot::request::{ValidationTiming, WorkspaceLoadRequest};
use crate::{IncrementalWorkspace, Spec42Engine};

pub fn update_workspace_snapshot(
    engine: &Spec42Engine,
    previous: &HostWorkspaceSnapshot,
    changes: DocumentChanges,
    request: WorkspaceLoadRequest,
    context: HostContext,
) -> WorkspaceResult<Arc<HostWorkspaceSnapshot>> {
    context.check_continue(HostPipelinePhase::LoadingDocuments)?;

    let merged_documents = apply_document_changes(previous.documents(), &changes)?;
    let total_bytes = merged_documents
        .iter()
        .map(|doc| doc.content.len() as u64)
        .sum();
    context.enforce_document_limits(merged_documents.len(), total_bytes)?;

    if can_use_incremental_update(engine, previous, &changes, &merged_documents) {
        match try_incremental_update(
            engine,
            previous,
            &changes,
            &merged_documents,
            &request,
            &context,
        ) {
            Ok(snapshot) => return Ok(Arc::new(snapshot)),
            Err(err) if err.code() == "cancelled" => return Err(err),
            Err(_) => {}
        }
    }

    let provider = InMemoryDocumentProvider::new(merged_documents);
    build_workspace_snapshot(
        engine,
        engine.library_catalog(),
        engine.metadata(),
        provider,
        request,
        &context,
    )
    .map(Arc::new)
}

fn can_use_incremental_update(
    engine: &Spec42Engine,
    previous: &HostWorkspaceSnapshot,
    changes: &DocumentChanges,
    merged_documents: &[SysmlDocument],
) -> bool {
    if !engine.experimental_incremental_updates() {
        return false;
    }
    if !changes.added.is_empty() || !changes.removed.is_empty() {
        return false;
    }
    if changes.changed.len() != 1 {
        return false;
    }
    let changed = &changes.changed[0];
    if !is_workspace_document(changed) {
        return false;
    }
    if previous.metadata().library_catalog_hash != engine.library_catalog().content_hash {
        return false;
    }
    merged_documents.iter().any(|doc| doc.uri == changed.uri)
}

fn try_incremental_update(
    engine: &Spec42Engine,
    previous: &HostWorkspaceSnapshot,
    changes: &DocumentChanges,
    merged_documents: &[SysmlDocument],
    request: &WorkspaceLoadRequest,
    context: &HostContext,
) -> WorkspaceResult<HostWorkspaceSnapshot> {
    let changed = &changes.changed[0];

    context.check_continue(HostPipelinePhase::BuildingGraph)?;
    // `IncrementalWorkspace` reconstructed from the previous snapshot's state — its `graph`
    // is `Arc`-backed, so this doesn't deep-copy until `apply_document` actually mutates it.
    let mut incremental_workspace = IncrementalWorkspace::from_parts(
        previous.semantic_graph_arc(),
        previous.parsed_documents().to_vec(),
    );
    // `cache_dir: None` — this path has never gone through the parse cache; keep it that way
    // for now rather than changing behavior as part of this migration (see Tier 2
    // unified-incremental-engine design, open question 4).
    incremental_workspace.apply_document(changed, None);

    let semantic_graph = incremental_workspace.graph();
    let parsed_documents = incremental_workspace.documents();

    context.enforce_graph_limits(
        semantic_graph.node_ids_by_qualified_name.len(),
        semantic_graph.graph.edge_count(),
    )?;
    context.check_continue(HostPipelinePhase::BuildingGraph)?;

    assemble_snapshot_from_state(
        engine,
        previous,
        merged_documents,
        semantic_graph,
        parsed_documents,
        request,
        context,
    )
}

fn assemble_snapshot_from_state(
    engine: &Spec42Engine,
    previous: &HostWorkspaceSnapshot,
    documents: &[SysmlDocument],
    semantic_graph: SemanticGraph,
    parsed_documents: Vec<WorkspaceParsedDocument>,
    request: &WorkspaceLoadRequest,
    context: &HostContext,
) -> WorkspaceResult<HostWorkspaceSnapshot> {
    let build_instant = Instant::now();
    let target_files = discover_target_files(&request.targets)?;
    let library_paths = previous.library_paths().to_vec();
    let library_urls = previous.library_urls().to_vec();
    let workspace_root = previous.workspace_root().to_path_buf();
    let workspace_root_uri = previous.workspace_root_uri().clone();

    context.check_continue(HostPipelinePhase::BuildingLanguageWorkspace)?;
    let language_workspace = InMemoryWorkspace::from_graph_and_documents(
        semantic_graph.clone(),
        parsed_documents.clone(),
        documents,
    )
    .map_err(map_language_service_error)?;
    context.check_continue(HostPipelinePhase::BuildingLanguageWorkspace)?;

    let render_version = previous.view_catalog().version.wrapping_add(1);
    context.check_continue(HostPipelinePhase::BuildingViewCatalog)?;
    let render_snapshot = build_render_snapshot(
        &semantic_graph,
        &parsed_documents,
        &library_urls,
        &workspace_root_uri,
        render_version,
    )
    .map_err(map_render_snapshot_error)?;
    context.check_continue(HostPipelinePhase::BuildingViewCatalog)?;

    context.check_continue(HostPipelinePhase::CollectingValidation)?;
    let validation_report = if request.validation_timing == ValidationTiming::Eager {
        crate::snapshot::build::init_validation_report(
            ValidationTiming::Eager,
            collect_host_validation_report(
                &semantic_graph,
                documents,
                &library_urls,
                &target_files,
                Some(workspace_root.as_path()),
                &library_paths,
                request.strict_diagnostics,
            )?,
        )?
    } else {
        std::sync::OnceLock::new()
    };
    context.check_continue(HostPipelinePhase::CollectingValidation)?;

    context.check_continue(HostPipelinePhase::ProjectingModel)?;
    let semantic_projection = project_host_semantic_model(&semantic_graph, &target_files)?;
    context.check_continue(HostPipelinePhase::ProjectingModel)?;

    Ok(assemble_host_workspace_snapshot(
        engine.metadata(),
        engine.library_catalog(),
        documents.to_vec(),
        semantic_graph,
        parsed_documents,
        language_workspace,
        render_snapshot,
        validation_report,
        target_files,
        request.strict_diagnostics,
        request.validation_timing,
        semantic_projection,
        library_urls,
        library_paths,
        workspace_root,
        workspace_root_uri,
        build_instant,
    ))
}
