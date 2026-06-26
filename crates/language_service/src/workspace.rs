use std::collections::HashMap;
use std::sync::Arc;

use semantic_core::{
    build_semantic_graph_from_documents, SemanticGraph, SysmlDocument, TextPosition,
    WorkspaceParsedDocument,
};
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
    semantic_graph: Arc<SemanticGraph>,
    symbol_table: Vec<SymbolEntry>,
}

/// Read-only workspace view used by navigation services.
pub trait WorkspaceSnapshot {
    fn resolve_uri_for_path(&self, path: &str) -> Option<Url>;
    fn path_for_uri(&self, uri: &Url) -> String;
    fn document_text(&self, uri: &Url) -> Option<&str>;
    fn semantic_graph(&self) -> &SemanticGraph;
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
        let (semantic_graph, parsed_docs) = build_semantic_graph_from_documents(&documents)?;
        Self::from_graph_and_documents(Arc::new(semantic_graph), parsed_docs, &documents)
    }

    /// Build a workspace from an already-built semantic graph and parsed documents.
    pub fn from_graph_and_documents(
        semantic_graph: Arc<SemanticGraph>,
        parsed_docs: Vec<WorkspaceParsedDocument>,
        documents: &[SysmlDocument],
    ) -> Result<Self, String> {
        let mut documents_map = HashMap::new();
        let mut path_to_uri = HashMap::new();

        for parsed in parsed_docs {
            let path = parsed
                .uri
                .path()
                .trim_start_matches('/')
                .replace('\\', "/");
            let path = parsed
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
                .find(|doc| doc.uri == parsed.uri)
                .and_then(|doc| doc.path_hint.clone())
                .unwrap_or(path);

            let uri = normalize_uri(&parsed.uri);
            path_to_uri.insert(path.clone(), uri.clone());
            documents_map.insert(
                uri,
                DocumentEntry {
                    path,
                    content: parsed.content,
                    parsed: parsed.parsed,
                },
            );
        }

        let mut symbol_table = Vec::new();
        for uri in documents_map.keys() {
            symbol_table.extend(symbol_entries_for_uri(&semantic_graph, uri));
        }

        Ok(Self {
            documents: documents_map,
            path_to_uri,
            semantic_graph,
            symbol_table,
        })
    }

    pub fn from_provider(
        provider: &impl semantic_core::SysmlDocumentProvider,
    ) -> Result<Self, String> {
        let documents = provider.load_documents()?;
        Self::from_documents(documents)
    }
}

impl WorkspaceSnapshot for InMemoryWorkspace {
    fn resolve_uri_for_path(&self, path: &str) -> Option<Url> {
        let normalized = path.trim_start_matches('/').replace('\\', "/");
        self.path_to_uri.get(&normalized).cloned().or_else(|| {
            self.path_to_uri
                .iter()
                .find(|(key, _)| key.as_str() == normalized || key.ends_with(&format!("/{normalized}")))
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

    fn semantic_graph(&self) -> &SemanticGraph {
        &*self.semantic_graph
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

    fn semantic_graph(&self) -> &SemanticGraph {
        (*self).semantic_graph()
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
