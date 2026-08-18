//! Building the immutable publication a workspace snapshot validates from.
//!
//! One publication per snapshot, admitted from the same documents the snapshot admitted, so
//! validation answers about exactly the model state the snapshot describes. It is built alongside
//! the graph rather than derived from it: the two have separate owners, and deriving one from the
//! other would make the graph an input to validation again.

use sysml_source::{SysmlDocument, SysmlDocumentSourceKind};
use sysml_query::resolved_slice::{
    build, BuildRequest, ConstructionStrategy, PublishedModel, SourceDocument, SourceKind,
};

use crate::error::{WorkspaceError, WorkspaceResult};

/// Publishes `documents` as one immutable model.
///
/// Every admitted document keeps its authored source kind, so the publication can tell a workspace
/// source from a library one -- which is what decides whose diagnostics are reported.
pub fn publish_documents(documents: &[SysmlDocument]) -> WorkspaceResult<PublishedModel> {
    let sources = documents
        .iter()
        .map(|document| {
            SourceDocument::from_uri(
                document.uri.as_str(),
                document.content.clone(),
                source_kind(document.source_kind),
            )
            .map_err(|error| {
                WorkspaceError::internal_invariant_failure(format!(
                    "admitting {}: {error}",
                    document.uri
                ))
            })
        })
        .collect::<WorkspaceResult<Vec<_>>>()?;
    let request =
        BuildRequest::resolved(sources, ConstructionStrategy::Parallel).map_err(|error| {
            WorkspaceError::internal_invariant_failure(format!("publication request: {error}"))
        })?;
    build(request).map_err(|error| {
        WorkspaceError::internal_invariant_failure(format!("publication: {error}"))
    })
}

fn source_kind(kind: SysmlDocumentSourceKind) -> SourceKind {
    match kind {
        SysmlDocumentSourceKind::Workspace => SourceKind::Workspace,
        SysmlDocumentSourceKind::StandardLibrary => SourceKind::StandardLibrary,
        SysmlDocumentSourceKind::Library => SourceKind::Library,
        SysmlDocumentSourceKind::External => SourceKind::External,
    }
}
