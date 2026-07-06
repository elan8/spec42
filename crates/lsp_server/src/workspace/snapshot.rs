use language_service::{SymbolEntry as LsSymbolEntry, WorkspaceSnapshot};
use tower_lsp::lsp_types::Url;

use crate::common::text_span::to_core_range;
use crate::common::util;
use crate::language::SymbolEntry;
use crate::workspace::ServerState;

/// Adapter that exposes LSP [`ServerState`] through the neutral [`WorkspaceSnapshot`] trait.
pub(crate) struct ServerStateSnapshot<'a> {
    state: &'a ServerState,
    symbol_table: Vec<LsSymbolEntry>,
    perf_logging_enabled: bool,
}

impl<'a> ServerStateSnapshot<'a> {
    pub(crate) fn new(state: &'a ServerState, perf_logging_enabled: bool) -> Self {
        let symbol_table = state
            .symbol_table
            .iter()
            .map(convert_symbol_entry)
            .collect();
        Self {
            state,
            symbol_table,
            perf_logging_enabled,
        }
    }
}

fn convert_symbol_entry(entry: &SymbolEntry) -> LsSymbolEntry {
    LsSymbolEntry {
        name: entry.name.clone(),
        uri: entry.uri.clone(),
        range: to_core_range(entry.range),
        container_name: entry.container_name.clone(),
        detail: entry.detail.clone(),
        description: entry.description.clone(),
        signature: entry.signature.clone(),
    }
}

impl WorkspaceSnapshot for ServerStateSnapshot<'_> {
    fn resolve_uri_for_path(&self, path: &str) -> Option<Url> {
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
            .map(|entry| entry.content.as_str())
    }

    fn semantic_graph(&self) -> &crate::semantic::SemanticGraph {
        &self.state.semantic_graph
    }

    fn symbol_table(&self) -> &[LsSymbolEntry] {
        &self.symbol_table
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
        crate::workspace::state::supports_semantic_queries(self.state.session.lifecycle())
    }
}
