use std::collections::HashMap;
use std::sync::Arc;

use sysml_query::resolved_slice::TextPosition;
use sysml_query::source::SourceDocument;
use url::Url;

use crate::symbol::{symbol_entries_for_uri, SymbolEntry};
use crate::uri::normalize_uri;

#[derive(Debug, Clone)]
struct DocumentEntry {
    path: String,
    content: String,
    /// The admitted document, kept so the syntax service can be asked for its tree. A document
    /// handle, not a tree: the tree stays in the service's memo.
    document: SourceDocument,
}

/// In-memory indexed workspace for headless language-service queries.
#[derive(Debug, Clone)]
pub struct InMemoryWorkspace {
    documents: HashMap<Url, DocumentEntry>,
    path_to_uri: HashMap<String, Url>,
    syntax: sysml_query::syntax::SyntaxService,
    published_model: Arc<sysml_query::resolved_slice::PublishedModel>,
    symbol_table: Vec<SymbolEntry>,
}

/// Read-only workspace view used by navigation services.
pub trait WorkspaceSnapshot {
    fn resolve_uri_for_path(&self, path: &str) -> Option<Url>;
    fn path_for_uri(&self, uri: &Url) -> String;
    fn document_text(&self, uri: &Url) -> Option<&str>;
    /// The tree the syntax service parsed for this document.
    ///
    /// A handle into the service's memo, so asking is a lookup rather than a parse. Services that
    /// need a syntax answer ask the tree for it instead of re-deriving one from the text.
    fn parsed(&self, uri: &Url) -> Option<sysml_query::syntax::ParsedSource>;
    fn published_model(&self) -> Option<&sysml_query::resolved_slice::PublishedModel>;
    fn semantic_status(&self) -> crate::dto::SemanticResultStatus {
        self.published_model()
            .map(|model| {
                crate::dto::SemanticResultStatus::from_publication(
                    model.publication().completeness(),
                )
            })
            .unwrap_or_default()
    }
    fn symbol_table(&self) -> &[SymbolEntry];
    fn index_uris(&self) -> Vec<Url>;
    fn normalize_uri(&self, uri: &Url) -> Url {
        normalize_uri(uri)
    }
    fn perf_logging_enabled(&self) -> bool {
        false
    }
    fn supports_semantic_queries(&self) -> bool {
        true
    }
    fn library_paths(&self) -> &[Url] {
        &[]
    }
}

impl InMemoryWorkspace {
    /// Index documents against the exact immutable publication owned by the host.
    pub fn from_documents_and_publication(
        documents: Vec<SourceDocument>,
        published_model: Arc<sysml_query::resolved_slice::PublishedModel>,
    ) -> Result<Self, String> {
        let mut documents_map = HashMap::new();
        let mut path_to_uri = HashMap::new();

        for document in &documents {
            let full_path = document
                .uri()
                .path()
                .trim_start_matches('/')
                .replace('\\', "/");
            let last_segment = document
                .uri()
                .path()
                .split('/')
                .next_back()
                .map(str::to_string)
                .filter(|segment| !segment.is_empty())
                .unwrap_or(full_path);
            // The logical path a host addresses this document by, when it gave one.
            let path = document
                .path_hint()
                .map(str::to_owned)
                .unwrap_or(last_segment);

            let uri = document.uri().clone();
            path_to_uri.insert(path.clone(), uri.clone());
            documents_map.insert(
                uri,
                DocumentEntry {
                    path,
                    content: document.content().to_owned(),
                    document: document.clone(),
                },
            );
        }

        let mut symbol_table = Vec::new();
        for uri in documents_map.keys() {
            symbol_table.extend(symbol_entries_for_uri(&published_model, uri));
        }

        Ok(Self {
            documents: documents_map,
            path_to_uri,
            syntax: sysml_query::syntax::SyntaxService::new(),
            published_model,
            symbol_table,
        })
    }
}

impl WorkspaceSnapshot for InMemoryWorkspace {
    fn resolve_uri_for_path(&self, path: &str) -> Option<Url> {
        if let Ok(uri) = Url::parse(path) {
            let normalized = normalize_uri(&uri);
            if self.documents.contains_key(&normalized) {
                return Some(normalized);
            }
        }
        let normalized = path.trim_start_matches('/').replace('\\', "/");
        self.path_to_uri.get(&normalized).cloned().or_else(|| {
            self.path_to_uri
                .iter()
                .find(|(key, _)| {
                    key.as_str() == normalized || key.ends_with(&format!("/{normalized}"))
                })
                .map(|(_, uri)| uri.clone())
        })
    }

    fn path_for_uri(&self, uri: &Url) -> String {
        let normalized = normalize_uri(uri);
        self.documents
            .get(&normalized)
            .map(|entry| entry.path.clone())
            .unwrap_or_else(|| uri.path().trim_start_matches('/').to_string())
    }

    fn document_text(&self, uri: &Url) -> Option<&str> {
        self.documents
            .get(&normalize_uri(uri))
            .map(|entry| entry.content.as_str())
    }

    fn parsed(&self, uri: &Url) -> Option<sysml_query::syntax::ParsedSource> {
        self.documents
            .get(&normalize_uri(uri))
            .map(|entry| self.syntax.parse(&entry.document))
    }

    fn published_model(&self) -> Option<&sysml_query::resolved_slice::PublishedModel> {
        Some(&self.published_model)
    }

    fn symbol_table(&self) -> &[SymbolEntry] {
        &self.symbol_table
    }

    fn index_uris(&self) -> Vec<Url> {
        self.documents.keys().cloned().collect()
    }
}

impl WorkspaceSnapshot for &InMemoryWorkspace {
    fn resolve_uri_for_path(&self, path: &str) -> Option<Url> {
        (*self).resolve_uri_for_path(path)
    }

    fn path_for_uri(&self, uri: &Url) -> String {
        (*self).path_for_uri(uri)
    }

    fn document_text(&self, uri: &Url) -> Option<&str> {
        (*self).document_text(uri)
    }

    fn parsed(&self, uri: &Url) -> Option<sysml_query::syntax::ParsedSource> {
        (*self).parsed(uri)
    }

    fn published_model(&self) -> Option<&sysml_query::resolved_slice::PublishedModel> {
        (*self).published_model()
    }

    fn symbol_table(&self) -> &[SymbolEntry] {
        (*self).symbol_table()
    }

    fn index_uris(&self) -> Vec<Url> {
        (*self).index_uris()
    }
}

/// Resolve a logical path and position to a document URI.
pub fn uri_for_path(workspace: &impl WorkspaceSnapshot, path: &str) -> Option<Url> {
    workspace.resolve_uri_for_path(path)
}

/// Convert path + position to URI + TextPosition for internal queries.
pub fn resolve_document_position(
    workspace: &impl WorkspaceSnapshot,
    path: &str,
    position: TextPosition,
) -> Option<(Url, TextPosition)> {
    let uri = workspace.resolve_uri_for_path(path)?;
    Some((uri, position))
}
