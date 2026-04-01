//! Public wrappers intended for Criterion benchmarks.
//!
//! The LSP server’s workspace/index pipeline is intentionally `pub(crate)`. Benches live in
//! `benches/` and compile as an external crate, so they need a narrow public surface to call into
//! the real implementation without exposing the entire workspace module as public API.

use tower_lsp::lsp_types::Url;

use crate::workspace::state::ServerState;
use crate::{semantic_model, semantic_model::SemanticGraph};

/// Scan roots for `.sysml` / `.kerml` files and read their contents.
///
/// This mirrors the discovery+read portion of Spec42’s workspace scan.
pub fn scan_sysml_files(roots: Vec<Url>) -> Vec<(Url, String)> {
    crate::workspace::scan_sysml_files(roots).0
}

/// Parse a batch of scanned `(Url, String)` entries using Spec42’s editor-oriented parse path.
///
/// Returns the number of parsed entries (one per input file).
pub fn parse_scanned_entries(entries: Vec<(Url, String)>, parallel_enabled: bool) -> usize {
    crate::workspace::parse_scanned_entries(entries, parallel_enabled).len()
}

/// Parse a batch of scanned `(Url, String)` entries and ingest them into a fresh server state.
///
/// Returns the number of indexed documents after the ingest.
pub fn startup_index_scanned_entries(entries: Vec<(Url, String)>, parallel_enabled: bool) -> usize {
    let parsed_entries = crate::workspace::parse_scanned_entries(entries, parallel_enabled);
    let mut state = ServerState::default();
    crate::workspace::ingest_parsed_scan_entries(&mut state, parsed_entries);
    state.index.len()
}

/// Parse and build per-document semantic graphs without merging them.
///
/// Returns the total number of graph nodes across all parsed documents.
pub fn build_document_graphs(entries: Vec<(Url, String)>, parallel_enabled: bool) -> usize {
    let parsed_entries = crate::workspace::parse_scanned_entries(entries, parallel_enabled);
    let mut total_nodes = 0usize;
    for entry in parsed_entries {
        if let Some(doc) = entry.parsed.as_ref() {
            let graph = semantic_model::build_graph_from_doc(doc, &entry.uri);
            total_nodes += graph.nodes_for_uri(&entry.uri).len();
        }
    }
    total_nodes
}

/// Parse, build, and merge per-document semantic graphs without cross-document linking.
///
/// Returns the number of indexed URIs in the merged graph.
pub fn merge_document_graphs(entries: Vec<(Url, String)>, parallel_enabled: bool) -> usize {
    let parsed_entries = crate::workspace::parse_scanned_entries(entries, parallel_enabled);
    let mut merged = SemanticGraph::default();
    let mut uris = Vec::new();
    for entry in parsed_entries {
        if let Some(doc) = entry.parsed.as_ref() {
            let graph = semantic_model::build_graph_from_doc(doc, &entry.uri);
            merged.merge(graph);
            uris.push(entry.uri);
        }
    }
    uris.iter()
        .map(|uri| usize::from(!merged.nodes_for_uri(uri).is_empty()))
        .sum()
}

/// Parse, build, merge, and add cross-document edges for all scanned documents.
///
/// Returns the number of indexed URIs in the merged graph.
pub fn link_cross_document_relationships(
    entries: Vec<(Url, String)>,
    parallel_enabled: bool,
) -> usize {
    let parsed_entries = crate::workspace::parse_scanned_entries(entries, parallel_enabled);
    let mut merged = SemanticGraph::default();
    let mut uris = Vec::new();
    for entry in parsed_entries {
        if let Some(doc) = entry.parsed.as_ref() {
            let graph = semantic_model::build_graph_from_doc(doc, &entry.uri);
            merged.merge(graph);
            uris.push(entry.uri);
        }
    }
    for uri in &uris {
        semantic_model::add_cross_document_edges_for_uri(&mut merged, uri);
    }
    uris.iter()
        .map(|uri| usize::from(!merged.nodes_for_uri(uri).is_empty()))
        .sum()
}

/// Parse, build, merge, link cross-document relationships, and extract symbols.
///
/// Returns the total number of symbol entries across all indexed documents.
pub fn extract_symbols_from_workspace(
    entries: Vec<(Url, String)>,
    parallel_enabled: bool,
) -> usize {
    let parsed_entries = crate::workspace::parse_scanned_entries(entries, parallel_enabled);
    let mut merged = SemanticGraph::default();
    let mut uris = Vec::new();
    for entry in parsed_entries {
        if let Some(doc) = entry.parsed.as_ref() {
            let graph = semantic_model::build_graph_from_doc(doc, &entry.uri);
            merged.merge(graph);
            uris.push(entry.uri);
        }
    }
    for uri in &uris {
        semantic_model::add_cross_document_edges_for_uri(&mut merged, uri);
    }
    uris.iter()
        .map(|uri| crate::semantic_model::symbol_entries_for_uri(&merged, uri).len())
        .sum()
}
