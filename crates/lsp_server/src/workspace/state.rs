use crate::language::SymbolEntry;
use crate::semantic;
use std::sync::Arc;
use sysml_v2_parser::RootNamespace;
use tower_lsp::lsp_types::Url;
use workspace_session::{RelinkToken, TracksRelink};

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ParseMetadata {
    pub(crate) parse_time_ms: u32,
    pub(crate) parse_cached: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct IndexEntry {
    pub(crate) content: String,
    pub(crate) parsed: Option<RootNamespace>,
    pub(crate) parse_metadata: ParseMetadata,
    /// When `false`, the file is indexed for `sysml/librarySearch` only (not merged into the semantic graph).
    pub(crate) include_in_semantic_graph: bool,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct RuntimeConfig {
    pub(crate) startup_trace_id: Option<String>,
    pub(crate) code_lens_enabled: bool,
    pub(crate) perf_logging_enabled: bool,
    /// Development-only: include library paths in the debounced workspace-wide diagnostics
    /// sweep. See `spec42.development.diagnoseLibraryPaths` and
    /// `publish_workspace_diagnostics`'s comment.
    pub(crate) diagnose_library_paths: bool,
}

/// The server's live workspace state — managed exclusively by a single
/// `workspace_session::SessionActor<ServerState>` (see `crate::workspace::WorkspaceHandle`).
/// `Clone` is required by the actor's `Arc::make_mut` clone-on-write mutation strategy; readers
/// only ever see an `Arc<ServerState>` snapshot, never a lock guard.
#[derive(Clone, Default)]
pub(crate) struct ServerState {
    pub(crate) workspace_roots: Vec<Url>,
    pub(crate) library_paths: Vec<Url>,
    pub(crate) standard_library_paths: Vec<Url>,
    pub(crate) session: workspace::WorkspaceSession,
    pub(crate) index: std::collections::HashMap<Url, IndexEntry>,
    pub(crate) symbol_table: Vec<SymbolEntry>,
    pub(crate) semantic_graph: semantic::SemanticGraph,
    pub(crate) published_model: Option<Arc<sysml_query::resolved_slice::PublishedModel>>,
    /// Snapshot of the library-only portion of the semantic graph.
    ///
    /// Set during startup when library files are loaded from cache (no library paths
    /// configured) or extracted from the full graph on cache miss. Passed as `base_graph`
    /// to `rebuild_semantic_graph_staged` during async relinking so that library types
    /// remain available even though they are not stored in the `index`.
    pub(crate) library_graph_snapshot: Option<semantic::SemanticGraph>,
    pub(crate) render_cache: workspace::ViewRenderCache,
}

impl TracksRelink for ServerState {
    fn is_token_current(&self, token: &RelinkToken) -> bool {
        self.session.is_token_current(token)
    }

    fn rekey_for_actor(&mut self) {
        self.session.rekey_for_owner();
    }
}

/// Shared accessors letting `workspace/services.rs`'s free functions stay agnostic of the
/// concrete state type (kept as a trait rather than inlined directly against `ServerState`
/// since several of those functions are also exercised in isolation by their own unit tests).
pub(crate) trait DocumentStore {
    fn index(&self) -> &std::collections::HashMap<Url, IndexEntry>;
    fn index_mut(&mut self) -> &mut std::collections::HashMap<Url, IndexEntry>;
    fn symbol_table_mut(&mut self) -> &mut Vec<SymbolEntry>;
    fn semantic_graph(&self) -> &semantic::SemanticGraph;
    fn semantic_graph_mut(&mut self) -> &mut semantic::SemanticGraph;
    fn published_model_mut(
        &mut self,
    ) -> &mut Option<Arc<sysml_query::resolved_slice::PublishedModel>>;
}

impl DocumentStore for ServerState {
    fn index(&self) -> &std::collections::HashMap<Url, IndexEntry> {
        &self.index
    }
    fn index_mut(&mut self) -> &mut std::collections::HashMap<Url, IndexEntry> {
        &mut self.index
    }
    fn symbol_table_mut(&mut self) -> &mut Vec<SymbolEntry> {
        &mut self.symbol_table
    }
    fn semantic_graph(&self) -> &semantic::SemanticGraph {
        &self.semantic_graph
    }
    fn semantic_graph_mut(&mut self) -> &mut semantic::SemanticGraph {
        &mut self.semantic_graph
    }
    fn published_model_mut(
        &mut self,
    ) -> &mut Option<Arc<sysml_query::resolved_slice::PublishedModel>> {
        &mut self.published_model
    }
}

pub(crate) fn refresh_published_model(state: &mut impl DocumentStore) {
    let sources = state
        .index()
        .iter()
        .filter(|(_, entry)| entry.include_in_semantic_graph)
        .filter_map(|(uri, entry)| {
            sysml_query::resolved_slice::SourceDocument::from_uri(
                uri.as_str(),
                entry.content.clone(),
                sysml_query::resolved_slice::SourceKind::Workspace,
            )
            .ok()
        })
        .collect::<Vec<_>>();
    *state.published_model_mut() = sysml_query::resolved_slice::BuildRequest::resolved(
        sources,
        sysml_query::resolved_slice::ConstructionStrategy::Parallel,
    )
    .ok()
    .and_then(|request| sysml_query::resolved_slice::build(request).ok())
    .map(Arc::new);
}

pub(crate) fn supports_semantic_queries(lifecycle: workspace::SessionLifecycle) -> bool {
    matches!(lifecycle, workspace::SessionLifecycle::Ready)
}

#[derive(Debug, Default)]
pub(crate) struct ScanSummary {
    pub(crate) roots_scanned: usize,
    pub(crate) roots_skipped_non_file: usize,
    pub(crate) candidate_files: usize,
    pub(crate) files_loaded: usize,
    pub(crate) read_failures: usize,
    pub(crate) uri_failures: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_state_is_clone() {
        fn assert_clone<T: Clone>() {}
        assert_clone::<ServerState>();
    }

    #[test]
    fn tracks_relink_delegates_to_session() {
        let mut state = ServerState::default();
        state.session.begin_startup();
        state.session.complete_startup();

        let first_token = state.session.schedule_relink();
        assert!(
            state.is_token_current(&first_token),
            "freshly scheduled token must be current"
        );

        let second_token = state.session.schedule_relink();
        assert!(
            !state.is_token_current(&first_token),
            "superseded token must no longer be current"
        );
        assert!(
            state.is_token_current(&second_token),
            "the newly scheduled token must be current"
        );
    }
}
