use crate::language::SymbolEntry;
use crate::semantic;
use crate::workspace::coordinator::SemanticCoordinator;
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
}

#[derive(Default)]
pub(crate) struct ServerState {
    pub(crate) workspace_roots: Vec<Url>,
    pub(crate) library_paths: Vec<Url>,
    pub(crate) coordinator: SemanticCoordinator,
    pub(crate) index: std::collections::HashMap<Url, IndexEntry>,
    pub(crate) symbol_table: Vec<SymbolEntry>,
    pub(crate) semantic_graph: semantic::SemanticGraph,
    /// Snapshot of the library-only portion of the semantic graph.
    ///
    /// Set during startup when library files are loaded from cache (no library paths
    /// configured) or extracted from the full graph on cache miss. Passed as `base_graph`
    /// to `rebuild_semantic_graph_staged` during async relinking so that library types
    /// remain available even though they are not stored in the `index`.
    pub(crate) library_graph_snapshot: Option<semantic::SemanticGraph>,
}

#[derive(Clone, Default)]
pub(crate) struct WorkspaceState {
    pub(crate) workspace_roots: Vec<Url>,
    pub(crate) library_paths: Vec<Url>,
    pub(crate) session: workspace::WorkspaceSession,
    pub(crate) index: std::collections::HashMap<Url, IndexEntry>,
    pub(crate) symbol_table: Vec<SymbolEntry>,
    pub(crate) semantic_graph: semantic::SemanticGraph,
    pub(crate) library_graph_snapshot: Option<semantic::SemanticGraph>,
}

impl TracksRelink for WorkspaceState {
    fn is_token_current(&self, token: &RelinkToken) -> bool {
        self.session.is_token_current(token)
    }
}

/// Shared accessors letting `workspace/services.rs`'s free functions operate on either
/// `ServerState` (the live `RwLock`-guarded state, until it's fully retired) or
/// `WorkspaceState` (the new `SessionActor`-managed state) without duplicating their logic.
pub(crate) trait DocumentStore {
    fn index(&self) -> &std::collections::HashMap<Url, IndexEntry>;
    fn index_mut(&mut self) -> &mut std::collections::HashMap<Url, IndexEntry>;
    fn symbol_table_mut(&mut self) -> &mut Vec<SymbolEntry>;
    fn semantic_graph(&self) -> &semantic::SemanticGraph;
    fn semantic_graph_mut(&mut self) -> &mut semantic::SemanticGraph;
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
}

impl DocumentStore for WorkspaceState {
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
}

pub(crate) fn supports_semantic_queries(lifecycle: workspace::SessionLifecycle) -> bool {
    matches!(lifecycle, workspace::SessionLifecycle::Ready)
}

pub(crate) fn suppresses_transient_semantic_diagnostics(
    lifecycle: workspace::SessionLifecycle,
) -> bool {
    !matches!(lifecycle, workspace::SessionLifecycle::Ready)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum SemanticLifecycle {
    #[default]
    Cold,
    Indexing,
    Ready,
    Reindexing,
}

impl SemanticLifecycle {
    pub(crate) fn supports_semantic_queries(self) -> bool {
        matches!(self, Self::Ready)
    }

    pub(crate) fn suppresses_transient_semantic_diagnostics(self) -> bool {
        !matches!(self, Self::Ready)
    }
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
    fn workspace_state_is_clone() {
        fn assert_clone<T: Clone>() {}
        assert_clone::<WorkspaceState>();
    }

    #[test]
    fn tracks_relink_delegates_to_session() {
        let mut state = WorkspaceState::default();
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
