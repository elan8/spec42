use crate::language::SymbolEntry;
use crate::semantic_model;
use sysml_parser::RootNamespace;
use tower_lsp::lsp_types::Url;

/// Per-file index entry: content and optional parsed AST (invalidated when content changes).
#[derive(Debug)]
pub(crate) struct IndexEntry {
    pub(crate) content: String,
    pub(crate) parsed: Option<RootNamespace>,
}

#[derive(Debug, Default)]
pub(crate) struct ServerState {
    /// Workspace root URIs from initialize (workspace_folders or root_uri).
    pub(crate) workspace_roots: Vec<Url>,
    /// Library path roots from config (e.g. SysML-v2-Release). Indexed like workspace_roots.
    pub(crate) library_paths: Vec<Url>,
    /// One source of truth: URI -> (content, parsed). Open docs and workspace-scanned files.
    pub(crate) index: std::collections::HashMap<Url, IndexEntry>,
    /// Workspace-wide symbol table: flat list of definable symbols, updated when index changes.
    pub(crate) symbol_table: Vec<SymbolEntry>,
    /// Semantic graph (nodes = elements, edges = relationships). Source for sysml/model.
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
