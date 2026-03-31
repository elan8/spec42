use crate::language::SymbolEntry;
use crate::semantic_model;
use sysml_parser::RootNamespace;
use tower_lsp::lsp_types::Url;

#[derive(Debug)]
pub(crate) struct IndexEntry {
    pub(crate) content: String,
    pub(crate) parsed: Option<RootNamespace>,
}

#[derive(Debug, Default)]
pub(crate) struct ServerState {
    pub(crate) workspace_roots: Vec<Url>,
    pub(crate) library_paths: Vec<Url>,
    pub(crate) startup_trace_id: Option<String>,
    pub(crate) index: std::collections::HashMap<Url, IndexEntry>,
    pub(crate) symbol_table: Vec<SymbolEntry>,
    pub(crate) semantic_graph: semantic_model::SemanticGraph,
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
