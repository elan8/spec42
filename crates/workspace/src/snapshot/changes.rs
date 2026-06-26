//! Logical document overlays for snapshot updates.

use std::collections::HashSet;

use sysml_model::{SysmlDocument, SysmlDocumentSourceKind};
use url::Url;

use crate::error::{WorkspaceResult, WorkspaceError};

use super::build::enrich_document_hashes;

/// Added, changed, and removed logical documents applied on top of a prior snapshot.
#[derive(Debug, Clone, Default)]
pub struct DocumentChanges {
    pub added: Vec<SysmlDocument>,
    pub changed: Vec<SysmlDocument>,
    pub removed: Vec<Url>,
}

impl DocumentChanges {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_added(mut self, documents: Vec<SysmlDocument>) -> Self {
        self.added = documents;
        self
    }

    pub fn with_changed(mut self, documents: Vec<SysmlDocument>) -> Self {
        self.changed = documents;
        self
    }

    pub fn with_removed(mut self, uris: Vec<Url>) -> Self {
        self.removed = uris;
        self
    }

    /// Replace one workspace document by URI (convenience for single-save editor flows).
    pub fn replace(mut self, document: SysmlDocument) -> Self {
        self.changed = vec![document];
        self
    }

    pub(crate) fn validate(&self) -> WorkspaceResult<()> {
        let mut seen = HashSet::new();
        for uri in self
            .added
            .iter()
            .chain(self.changed.iter())
            .map(|doc| doc.uri.to_string())
            .chain(self.removed.iter().map(|uri| uri.to_string()))
        {
            if !seen.insert(uri.clone()) {
                return Err(WorkspaceError::invalid_document_uri(format!(
                    "document URI appears in multiple change buckets: {uri}"
                )));
            }
        }
        Ok(())
    }
}

/// Merge logical document changes onto a prior document set.
pub fn apply_document_changes(
    previous: &[SysmlDocument],
    changes: &DocumentChanges,
) -> WorkspaceResult<Vec<SysmlDocument>> {
    changes.validate()?;

    let removed: HashSet<String> = changes
        .removed
        .iter()
        .map(|uri| uri.to_string())
        .collect();
    let changed: HashSet<String> = changes
        .changed
        .iter()
        .map(|doc| doc.uri.to_string())
        .collect();

    let mut documents: Vec<SysmlDocument> = previous
        .iter()
        .filter(|doc| !removed.contains(&doc.uri.to_string()))
        .filter(|doc| !changed.contains(&doc.uri.to_string()))
        .cloned()
        .collect();

    documents.extend(changes.changed.clone());
    documents.extend(changes.added.clone());

    let mut enriched = documents;
    enrich_document_hashes(&mut enriched);
    Ok(enriched)
}

/// Whether the changed document is eligible for single-document graph patching.
pub(crate) fn is_workspace_document(document: &SysmlDocument) -> bool {
    matches!(
        document.source_kind,
        SysmlDocumentSourceKind::Workspace | SysmlDocumentSourceKind::External
    )
}
