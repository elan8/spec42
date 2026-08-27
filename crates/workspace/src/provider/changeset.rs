//! In-memory document changes layered on a base provider.

use std::collections::HashSet;

use sysml_query::source::{
    SourceAuthority, SourceDocument, SourceError, SourceLoadReport, SourceProvider, Url,
};

/// Overlay added/changed documents and remove logical URIs from a base provider.
#[derive(Debug, Clone)]
pub struct ChangesetDocumentProvider<P> {
    base: P,
    added: Vec<SourceDocument>,
    changed: Vec<SourceDocument>,
    removed: HashSet<String>,
}

impl<P> ChangesetDocumentProvider<P> {
    pub fn new(base: P) -> Self {
        Self {
            base,
            added: Vec::new(),
            changed: Vec::new(),
            removed: HashSet::new(),
        }
    }

    pub fn with_added(mut self, documents: Vec<SourceDocument>) -> Self {
        self.added = documents;
        self
    }

    pub fn with_changed(mut self, documents: Vec<SourceDocument>) -> Self {
        self.changed = documents;
        self
    }

    pub fn with_removed(mut self, uris: Vec<Url>) -> Self {
        self.removed = uris.into_iter().map(|uri| uri.to_string()).collect();
        self
    }
}

impl<P: SourceProvider> SourceProvider for ChangesetDocumentProvider<P> {
    fn load(&self, authority: &SourceAuthority) -> Result<SourceLoadReport, SourceError> {
        let mut report = self.base.load(authority)?;
        report
            .documents
            .retain(|doc| !self.removed.contains(&doc.uri().to_string()));

        let changed_uris: HashSet<String> = self
            .changed
            .iter()
            .map(|doc| doc.uri().to_string())
            .collect();
        report
            .documents
            .retain(|doc| !changed_uris.contains(&doc.uri().to_string()));
        report.documents.extend(self.changed.clone());
        report.documents.extend(self.added.clone());
        Ok(report)
    }
}
