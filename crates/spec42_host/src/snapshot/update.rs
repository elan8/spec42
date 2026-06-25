//! Incremental and full-rebuild snapshot updates.

use std::sync::Arc;
use std::time::Instant;

use language_service::InMemoryWorkspace;
use semantic_core::{
    add_cross_document_edges_for_uri, build_graph_from_doc, build_render_snapshot,
    finalize_workspace_graph, SysmlDocument, WorkspaceParsedDocument,
};

use crate::error::{map_language_service_error, map_render_snapshot_error, HostResult};
use crate::provider::InMemoryDocumentProvider;
use crate::snapshot::build::{
    assemble_host_workspace_snapshot, build_workspace_snapshot, HostWorkspaceSnapshot,
};
use crate::snapshot::changes::{apply_document_changes, is_workspace_document, DocumentChanges};
use crate::snapshot::context::{HostContext, HostPipelinePhase};
use crate::snapshot::discovery::discover_target_files;
use crate::snapshot::facts::{collect_host_validation_report, project_host_semantic_model};
use crate::snapshot::request::{ValidationTiming, WorkspaceLoadRequest};
use crate::Spec42Engine;

pub fn update_workspace_snapshot(
    engine: &Spec42Engine,
    previous: &HostWorkspaceSnapshot,
    changes: DocumentChanges,
    request: WorkspaceLoadRequest,
    context: HostContext,
) -> HostResult<Arc<HostWorkspaceSnapshot>> {
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
    merged_documents
        .iter()
        .any(|doc| doc.uri == changed.uri)
}

fn try_incremental_update(
    engine: &Spec42Engine,
    previous: &HostWorkspaceSnapshot,
    changes: &DocumentChanges,
    merged_documents: &[SysmlDocument],
    request: &WorkspaceLoadRequest,
    context: &HostContext,
) -> HostResult<HostWorkspaceSnapshot> {
    let changed = &changes.changed[0];
    let uri = changed.uri.clone();

    context.check_continue(HostPipelinePhase::BuildingGraph)?;
    let mut semantic_graph = previous.semantic_graph().clone();
    semantic_graph.remove_nodes_for_uri(&uri);

    let parsed_documents = patch_parsed_documents(previous.parsed_documents(), changed)?;
    if let Ok(parsed) = sysml_v2_parser::parse(&changed.content) {
        let doc_graph = build_graph_from_doc(&parsed, &uri);
        semantic_graph.merge(doc_graph);
        add_cross_document_edges_for_uri(&mut semantic_graph, &uri);
    }

    finalize_workspace_graph(&mut semantic_graph);
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

fn patch_parsed_documents(
    previous: &[WorkspaceParsedDocument],
    changed: &SysmlDocument,
) -> HostResult<Vec<WorkspaceParsedDocument>> {
    let parse_start = Instant::now();
    let mut parsed_documents: Vec<WorkspaceParsedDocument> = previous
        .iter()
        .filter(|doc| doc.uri != changed.uri)
        .cloned()
        .collect();

    if let Ok(parsed) = sysml_v2_parser::parse(&changed.content) {
        let parse_time_ms = parse_start.elapsed().as_millis().max(1) as u32;
        parsed_documents.push(WorkspaceParsedDocument {
            uri: changed.uri.clone(),
            content: changed.content.clone(),
            parsed,
            parse_time_ms,
            parse_cached: false,
        });
    }

    Ok(parsed_documents)
}

fn assemble_snapshot_from_state(
    engine: &Spec42Engine,
    previous: &HostWorkspaceSnapshot,
    documents: &[SysmlDocument],
    semantic_graph: semantic_core::SemanticGraph,
    parsed_documents: Vec<WorkspaceParsedDocument>,
    request: &WorkspaceLoadRequest,
    context: &HostContext,
) -> HostResult<HostWorkspaceSnapshot> {
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
