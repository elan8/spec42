//! In-memory document changes layered on a base provider.

use std::collections::HashSet;

use sysml_model::{SysmlDocument, SysmlDocumentProvider};
use url::Url;

/// Overlay added/changed documents and remove logical URIs from a base provider.
#[derive(Debug, Clone)]
pub struct ChangesetDocumentProvider<P> {
    base: P,
    added: Vec<SysmlDocument>,
    changed: Vec<SysmlDocument>,
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

    pub fn with_added(mut self, documents: Vec<SysmlDocument>) -> Self {
        self.added = documents;
        self
    }

    pub fn with_changed(mut self, documents: Vec<SysmlDocument>) -> Self {
        self.changed = documents;
        self
    }

    pub fn with_removed(mut self, uris: Vec<Url>) -> Self {
        self.removed = uris.into_iter().map(|uri| uri.to_string()).collect();
        self
    }
}

impl<P: SysmlDocumentProvider> SysmlDocumentProvider for ChangesetDocumentProvider<P> {
    fn load_documents(&self) -> Result<Vec<SysmlDocument>, String> {
        let mut documents = self.base.load_documents()?;
        documents.retain(|doc| !self.removed.contains(&doc.uri.to_string()));

        let changed_uris: HashSet<String> = self
            .changed
            .iter()
            .map(|doc| doc.uri.to_string())
            .collect();
        documents.retain(|doc| !changed_uris.contains(&doc.uri.to_string()));
        documents.extend(self.changed.clone());
        documents.extend(self.added.clone());
        Ok(documents)
    }
}
