//! Immutable snapshot replacement.
//!
//! Updates deliberately take the canonical full-build path. The removed graph-patching path
//! mutated a legacy semantic representation and then rebuilt the immutable publication anyway,
//! leaving two independently constructed semantic states in one snapshot. If incremental
//! publication is reintroduced, it must operate at the immutable publication owner and prove
//! full/incremental equivalence.

use std::sync::Arc;

use crate::error::WorkspaceResult;
use crate::provider::InMemoryDocumentProvider;
use crate::snapshot::build::{build_workspace_snapshot, HostWorkspaceSnapshot};
use crate::snapshot::changes::{apply_document_changes, DocumentChanges};
use crate::snapshot::context::{HostContext, HostPipelinePhase};
use crate::snapshot::request::WorkspaceLoadRequest;
use crate::Spec42Engine;

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
