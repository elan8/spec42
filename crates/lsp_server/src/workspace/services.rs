use crate::common::util;
use crate::semantic;
use crate::workspace::library_search;
use crate::workspace::parse_cache;
use crate::workspace::state::{DocumentStore, IndexEntry, ParseMetadata};
#[cfg(test)]
use crate::workspace::state::ServerState;
use rayon::prelude::*;
use std::path::Path;
use std::time::Instant;
use sysml_v2_parser::RootNamespace;
use tower_lsp::lsp_types::{MessageType, TextDocumentContentChangeEvent, Url};
use workspace::semantic::WorkspaceParsedDocument;
use workspace::{IncrementalWorkspace, SysmlDocumentSourceKind};

fn elapsed_ms(start: Instant) -> u32 {
    start.elapsed().as_millis().max(1) as u32
}

pub(crate) fn indexed_text(state: &impl DocumentStore, uri_norm: &Url) -> Option<String> {
    state.index().get(uri_norm).map(|entry| entry.content.clone())
}

pub(crate) fn indexed_text_or_empty(state: &impl DocumentStore, uri_norm: &Url) -> String {
    indexed_text(state, uri_norm).unwrap_or_default()
}

#[derive(Debug)]
pub(crate) struct ParsedScanEntry {
    pub(crate) uri: Url,
    pub(crate) content: String,
    pub(crate) parsed: Option<RootNamespace>,
    pub(crate) parse_errors: Vec<String>,
    pub(crate) parse_metadata: ParseMetadata,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct RebuildAllDocumentLinksMetrics {
    pub(crate) uri_count: usize,
    pub(crate) parsed_doc_count: usize,
    pub(crate) remove_nodes_ms: u32,
    pub(crate) rebuild_graphs_ms: u32,
    pub(crate) cross_edge_resolution_ms: u32,
    pub(crate) workspace_relationship_linking_ms: u32,
    pub(crate) pending_relationship_resolution_ms: u32,
    pub(crate) expression_evaluation_ms: u32,
    pub(crate) cross_document_edges_ms: u32,
    pub(crate) refresh_symbols_ms: u32,
    pub(crate) total_ms: u32,
}

fn warning_from_parse_errors(
    uri_norm: &Url,
    parse_errors: &[String],
    diagnostic_count: usize,
    context: &str,
) -> Option<String> {
    if parse_errors.is_empty() {
        None
    } else {
        Some(format!(
            "sysml parse for editor produced {} diagnostic(s) for {} during {}: {}",
            diagnostic_count,
            uri_norm.as_str(),
            context,
            parse_errors.join("; ")
        ))
    }
}

fn parse_scanned_entry(uri: Url, content: String, cache_dir: Option<&Path>) -> ParsedScanEntry {
    // Try cache before parsing.
    if let Some(dir) = cache_dir {
        let hash = parse_cache::content_hash(content.as_bytes());
        if let Some(root) = parse_cache::load(dir, &hash) {
            tracing::debug!(uri = %uri, "parse cache hit");
            return ParsedScanEntry {
                uri,
                content,
                parsed: Some(root),
                parse_errors: vec![],
                parse_metadata: ParseMetadata {
                    parse_time_ms: 0,
                    parse_cached: true,
                },
            };
        }
        tracing::debug!(uri = %uri, "parse cache miss — parsing and storing");
        // Cache miss: parse normally then store.
        let entry = parse_scanned_entry_cold(uri, content);
        if let Some(root) = &entry.parsed {
            parse_cache::store(dir, &hash, root);
        }
        return entry;
    }
    parse_scanned_entry_cold(uri, content)
}

fn parse_scanned_entry_cold(uri: Url, content: String) -> ParsedScanEntry {
    let parse_start = Instant::now();
    let parsed_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        util::parse_for_editor(&content)
    }));
    let parse_time_ms = elapsed_ms(parse_start);
    let parser_panicked = parsed_result.is_err();
    let (parsed, mut parse_errors) = match parsed_result {
        Ok(result) => {
            let errs = result
                .errors
                .iter()
                .take(5)
                .map(|e| {
                    let loc = e
                        .to_lsp_range()
                        .map(|(sl, sc, _, _)| format!("{}:{}", sl, sc))
                        .unwrap_or_else(|| format!("{:?}:{:?}", e.line, e.column));
                    format!("{loc} {}", e.message)
                })
                .collect::<Vec<_>>();
            (Some(result.root), errs)
        }
        Err(_) => (None, util::parse_failure_diagnostics(&content, 5)),
    };
    if parser_panicked {
        parse_errors.push("parser panicked while parsing scanned workspace file".to_string());
    }
    ParsedScanEntry {
        uri,
        content,
        parsed,
        parse_errors,
        parse_metadata: ParseMetadata {
            parse_time_ms,
            parse_cached: false,
        },
    }
}

pub(crate) fn parse_scanned_entries(
    entries: Vec<(Url, String)>,
    parallel_enabled: bool,
    cache_dir: Option<std::path::PathBuf>,
) -> Vec<ParsedScanEntry> {
    if entries.is_empty() {
        return Vec::new();
    }

    if !parallel_enabled || entries.len() < 2 {
        return entries
            .into_iter()
            .map(|(uri, content)| parse_scanned_entry(uri, content, cache_dir.as_deref()))
            .collect();
    }

    // `into_par_iter` on a `Vec` is an indexed parallel iterator, so `collect()` preserves
    // the original order regardless of which worker finishes first.
    entries
        .into_par_iter()
        .map(|(uri, content)| parse_scanned_entry(uri, content, cache_dir.as_deref()))
        .collect()
}

fn update_symbol_table_for_uri(
    state: &mut impl DocumentStore,
    uri: &Url,
    new_entries: Option<&[crate::language::SymbolEntry]>,
) {
    state.symbol_table_mut().retain(|entry| entry.uri != *uri);
    if let Some(entries) = new_entries {
        state.symbol_table_mut().extend(entries.iter().cloned());
    }
}

fn update_semantic_graph_for_uri(
    state: &mut impl DocumentStore,
    uri: &Url,
    doc: Option<&RootNamespace>,
    evaluate: bool,
) {
    semantic::patch_graph_for_document(state.semantic_graph_mut(), uri, doc, evaluate);
}

fn refresh_symbols_for_uri(state: &mut impl DocumentStore, uri: &Url) {
    let mut new_entries = semantic::symbol_entries_for_uri(state.semantic_graph(), uri);
    if let Some(index_entry) = state.index().get(uri) {
        library_search::add_short_name_symbol_entries(&mut new_entries, &index_entry.content, uri);
    }
    update_symbol_table_for_uri(state, uri, Some(&new_entries));
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn store_parsed_document_text(
    state: &mut impl DocumentStore,
    uri_norm: &Url,
    text: String,
    parsed: Option<RootNamespace>,
    parse_metadata: ParseMetadata,
    parse_errors: &[String],
    diagnostic_count: usize,
    context: &str,
    evaluate: bool,
) -> Option<String> {
    update_semantic_graph_for_uri(state, uri_norm, parsed.as_ref(), evaluate);
    state.index_mut().insert(
        uri_norm.clone(),
        IndexEntry {
            content: text,
            parsed,
            parse_metadata,
            include_in_semantic_graph: true,
        },
    );
    refresh_symbols_for_uri(state, uri_norm);
    warning_from_parse_errors(uri_norm, parse_errors, diagnostic_count, context)
}

pub(crate) fn store_document_text(
    state: &mut impl DocumentStore,
    uri_norm: &Url,
    text: String,
) -> Option<String> {
    let parse_start = Instant::now();
    let parsed_result = util::parse_for_editor(&text);
    let parse_time_ms = elapsed_ms(parse_start);
    let parse_errors = parsed_result
        .errors
        .iter()
        .take(5)
        .map(|e| e.message.clone())
        .collect::<Vec<_>>();
    store_parsed_document_text(
        state,
        uri_norm,
        text,
        Some(parsed_result.root),
        ParseMetadata {
            parse_time_ms,
            parse_cached: false,
        },
        &parse_errors,
        parsed_result.errors.len(),
        "store_document_text",
        true,
    )
}

/// Like `store_document_text` but skips the expensive cross-document evaluation
/// pass (`evaluate: false`). The caller is responsible for scheduling an async
/// relink to rebuild cross-document edges and expression evaluation.
pub(crate) fn store_document_text_fast(
    state: &mut impl DocumentStore,
    uri_norm: &Url,
    text: String,
) -> Option<String> {
    let parse_start = Instant::now();
    let parsed_result = util::parse_for_editor(&text);
    let parse_time_ms = elapsed_ms(parse_start);
    let parse_errors = parsed_result
        .errors
        .iter()
        .take(5)
        .map(|e| e.message.clone())
        .collect::<Vec<_>>();
    store_parsed_document_text(
        state,
        uri_norm,
        text,
        Some(parsed_result.root),
        ParseMetadata {
            parse_time_ms,
            parse_cached: false,
        },
        &parse_errors,
        parsed_result.errors.len(),
        "store_document_text_fast",
        false,
    )
}

pub(crate) fn refresh_document(
    state: &mut impl DocumentStore,
    uri_norm: &Url,
    content: String,
) -> Option<String> {
    store_document_text(state, uri_norm, content)
}

pub(crate) fn ingest_parsed_scan_entries(
    state: &mut impl DocumentStore,
    entries: Vec<ParsedScanEntry>,
) -> Vec<(Url, Option<String>)> {
    let mut loaded = Vec::with_capacity(entries.len());
    for entry in entries {
        let uri_norm = util::normalize_file_uri(&entry.uri);
        let warning = store_parsed_document_text(
            state,
            &uri_norm,
            entry.content,
            entry.parsed,
            entry.parse_metadata,
            &entry.parse_errors,
            entry.parse_errors.len(),
            "workspace_scan",
            false,
        );
        loaded.push((uri_norm, warning));
    }
    semantic::evaluate_expressions(state.semantic_graph_mut());
    loaded
}

/// A faster version of ingest_parsed_scan_entries that avoids per-document relinking/evaluation.
/// Intended for use during startup when a full relink is performed immediately after.
pub(crate) fn ingest_parsed_scan_entries_batch(
    state: &mut impl DocumentStore,
    entries: Vec<ParsedScanEntry>,
) -> Vec<(Url, Option<String>)> {
    let mut loaded = Vec::with_capacity(entries.len());
    for entry in entries {
        let uri_norm = util::normalize_file_uri(&entry.uri);
        state.index_mut().insert(
            uri_norm.clone(),
            IndexEntry {
                content: entry.content,
                parsed: entry.parsed,
                parse_metadata: entry.parse_metadata,
                include_in_semantic_graph: true,
            },
        );
        let warning = warning_from_parse_errors(
            &uri_norm,
            &entry.parse_errors,
            entry.parse_errors.len(),
            "workspace_scan_batch",
        );
        loaded.push((uri_norm, warning));
    }
    loaded
}

/// Applies incoming text edits to the in-memory document only (no parsing, no
/// semantic graph work). Cheap and safe to run while holding the server's
/// write lock. Returns whether the content actually changed, so the caller
/// can decide whether a (potentially slow) parse is needed.

mod edits;
mod rebuild;
pub(crate) use edits::{apply_document_content_edit, apply_parsed_document_update, remove_document};
#[cfg(test)]
pub(crate) use edits::{apply_document_changes, apply_document_changes_fast};
pub(crate) use rebuild::{
    clear_documents_under_roots, index_library_paths_for_search, rebuild_all_document_links,
    rebuild_semantic_graph_staged,
};

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests;
