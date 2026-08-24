use language_service::{SymbolEntry, WorkspaceSnapshot};
use std::sync::Arc;
use tower_lsp::lsp_types::Url;

use crate::common::util;
use crate::session::ServerState;

/// Adapter that exposes LSP [`ServerState`] through the neutral [`WorkspaceSnapshot`] trait.
pub(crate) struct ServerStateSnapshot<'a> {
    state: &'a ServerState,
    perf_logging_enabled: bool,
    published_model: Option<Arc<sysml_query::resolved_slice::PublishedModel>>,
}

impl<'a> ServerStateSnapshot<'a> {
    pub(crate) fn new(state: &'a ServerState, perf_logging_enabled: bool) -> Self {
        Self {
            state,
            perf_logging_enabled,
            published_model: Some(Arc::clone(state.session.current())),
        }
    }
}

impl WorkspaceSnapshot for ServerStateSnapshot<'_> {
    fn resolve_uri_for_path(&self, path: &str) -> Option<Url> {
        if let Ok(uri) = Url::parse(path) {
            let normalized = util::normalize_file_uri(&uri);
            if self.state.index.contains_key(&normalized) {
                return Some(normalized);
            }
        }
        let normalized = path.trim_start_matches('/').replace('\\', "/");
        self.state.index.keys().find_map(|uri| {
            let uri_norm = util::normalize_file_uri(uri);
            let uri_path = uri_norm.path().trim_start_matches('/').replace('\\', "/");
            if uri_path == normalized || uri_path.ends_with(&format!("/{normalized}")) {
                Some(uri_norm)
            } else {
                None
            }
        })
    }

    fn path_for_uri(&self, uri: &Url) -> String {
        let normalized = util::normalize_file_uri(uri);
        normalized.path().trim_start_matches('/').replace('\\', "/")
    }

    fn document_text(&self, uri: &Url) -> Option<&str> {
        self.state
            .index
            .get(&util::normalize_file_uri(uri))
            .map(|entry| entry.content())
    }

    fn parsed(&self, uri: &Url) -> Option<sysml_query::syntax::ParsedSource> {
        self.state
            .index
            .get(&util::normalize_file_uri(uri))
            .map(|entry| entry.parsed.clone())
    }

    fn published_model(&self) -> Option<&sysml_query::resolved_slice::PublishedModel> {
        self.published_model.as_deref()
    }

    fn symbol_table(&self) -> &[SymbolEntry] {
        &self.state.semantic_symbols
    }

    fn index_uris(&self) -> Vec<Url> {
        self.state.index.keys().cloned().collect()
    }

    fn normalize_uri(&self, uri: &Url) -> Url {
        util::normalize_file_uri(uri)
    }

    fn perf_logging_enabled(&self) -> bool {
        self.perf_logging_enabled
    }

    fn library_paths(&self) -> &[Url] {
        &self.state.library_paths
    }

    fn supports_semantic_queries(&self) -> bool {
        crate::session::state::supports_semantic_queries(self.state.session.lifecycle())
    }
}
