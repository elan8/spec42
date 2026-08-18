use std::collections::HashMap;
use std::sync::Arc;

use sysml_query::resolved_slice::TextPosition;
use sysml_source::{SysmlDocument, SysmlDocumentProvider};
use sysml_v2_parser::RootNamespace;
use url::Url;

use crate::symbol::{symbol_entries_for_uri, SymbolEntry};
use crate::uri::normalize_uri;

#[derive(Debug, Clone)]
struct DocumentEntry {
    path: String,
    content: String,
    #[allow(dead_code)]
    parsed: RootNamespace,
}

/// In-memory indexed workspace for headless language-service queries.
#[derive(Debug, Clone)]
pub struct InMemoryWorkspace {
    documents: HashMap<Url, DocumentEntry>,
    path_to_uri: HashMap<String, Url>,
    published_model: Arc<sysml_query::resolved_slice::PublishedModel>,
    symbol_table: Vec<SymbolEntry>,
}

/// Read-only workspace view used by navigation services.
pub trait WorkspaceSnapshot {
    fn resolve_uri_for_path(&self, path: &str) -> Option<Url>;
    fn path_for_uri(&self, uri: &Url) -> String;
    fn document_text(&self, uri: &Url) -> Option<&str>;
    fn published_model(&self) -> Option<&sysml_query::resolved_slice::PublishedModel>;
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
    /// Build a workspace from pre-loaded SysML documents (workspace + optional library docs).
    pub fn from_documents(documents: Vec<SysmlDocument>) -> Result<Self, String> {
        let sources = documents
            .iter()
            .map(|document| {
                sysml_query::resolved_slice::SourceDocument::from_uri(
                    document.uri.as_str(),
                    document.content.clone(),
                    sysml_query::resolved_slice::SourceKind::Workspace,
                )
                .map_err(|error| error.to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;
        let request = sysml_query::resolved_slice::BuildRequest::resolved(
            sources,
            sysml_query::resolved_slice::ConstructionStrategy::Sequential,
        )
        .map_err(|error| error.to_string())?;
        let published_model = Arc::new(
            sysml_query::resolved_slice::build(request).map_err(|error| error.to_string())?,
        );
        Self::from_documents_and_publication(documents, published_model)
    }

    /// Index documents against the exact immutable publication owned by the host.
    pub fn from_documents_and_publication(
        documents: Vec<SysmlDocument>,
        published_model: Arc<sysml_query::resolved_slice::PublishedModel>,
    ) -> Result<Self, String> {
        let mut documents_map = HashMap::new();
        let mut path_to_uri = HashMap::new();

        for document in &documents {
            let parsed = sysml_v2_parser::parse_for_editor(&document.content).root;
            let path = document
                .uri
                .path()
                .trim_start_matches('/')
                .replace('\\', "/");
            let path = document
                .uri
                .path()
                .split('/')
                .next_back()
                .map(str::to_string)
                .filter(|segment| !segment.is_empty())
                .unwrap_or_else(|| path.clone());

            // Prefer path_hint from original documents when available.
            let path = documents
                .iter()
                .find(|doc| doc.uri == document.uri)
                .and_then(|doc| doc.path_hint.clone())
                .unwrap_or(path);

            let uri = normalize_uri(&document.uri);
            path_to_uri.insert(path.clone(), uri.clone());
            documents_map.insert(
                uri,
                DocumentEntry {
                    path,
                    content: document.content.clone(),
                    parsed,
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
            published_model,
            symbol_table,
        })
    }

    pub fn from_provider(provider: &impl SysmlDocumentProvider) -> Result<Self, String> {
        let documents = provider.load_documents()?;
        Self::from_documents(documents)
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
