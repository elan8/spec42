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
pub(crate) fn apply_document_content_edit(
    state: &mut impl DocumentStore,
    uri_norm: &Url,
    version: i32,
    content_changes: Vec<TextDocumentContentChangeEvent>,
) -> (bool, Vec<(MessageType, String)>) {
    let mut runtime_warnings = Vec::new();
    let content_changed = if let Some(entry) = state.index_mut().get_mut(uri_norm) {
        let mut content_changed = false;
        for change in content_changes {
            if let Some(range) = change.range {
                if let Some(new_text) =
                    util::apply_incremental_change(&entry.content, &range, &change.text)
                {
                    if new_text != entry.content {
                        entry.content = new_text;
                        content_changed = true;
                    }
                } else {
                    runtime_warnings.push((
                        MessageType::WARNING,
                        format!(
                            "didChange: ignored invalid incremental edit for {} at {}:{}..{}:{} (version {}).",
                            uri_norm,
                            range.start.line,
                            range.start.character,
                            range.end.line,
                            range.end.character,
                            version,
                        ),
                    ));
                }
            } else if entry.content != change.text {
                entry.content = change.text;
                content_changed = true;
            }
        }
        content_changed
    } else {
        runtime_warnings.push((
            MessageType::WARNING,
            format!(
                "didChange: document {} was not in the server index (version {}). Change was ignored until a full open/watch refresh occurs.",
                uri_norm, version
            ),
        ));
        false
    };
    (content_changed, runtime_warnings)
}

/// Applies an already-computed parse result (produced off the write lock, e.g.
/// via `spawn_blocking`) to the document and incrementally patches the
/// semantic graph/symbol table for that URI. This is the potentially-slow
/// half of a document update — callers should compute `parsed_result` without
/// holding the server's write lock so a slow parse of malformed/incomplete
/// syntax can't stall every other request.
pub(crate) fn apply_parsed_document_update(
    state: &mut impl DocumentStore,
    uri_norm: &Url,
    version: i32,
    parsed_result: sysml_v2_parser::ParseResult,
    parse_time_ms: u32,
    evaluate: bool,
) -> Vec<(MessageType, String)> {
    let mut runtime_warnings = Vec::new();
    let Some(entry) = state.index_mut().get_mut(uri_norm) else {
        return runtime_warnings;
    };
    entry.parsed = Some(parsed_result.root);
    entry.parse_metadata = ParseMetadata {
        parse_time_ms,
        parse_cached: false,
    };
    if !parsed_result.errors.is_empty() {
        runtime_warnings.push((
            MessageType::LOG,
            format!(
                "sysml parse_for_editor produced {} diagnostic(s) after didChange for {} (version {}).",
                parsed_result.errors.len(),
                uri_norm,
                version
            ),
        ));
    }

    let parsed = state
        .index()
        .get(uri_norm)
        .and_then(|entry| entry.parsed.as_ref())
        .cloned();
    update_semantic_graph_for_uri(state, uri_norm, parsed.as_ref(), evaluate);
    refresh_symbols_for_uri(state, uri_norm);

    runtime_warnings
}

#[cfg(test)]
fn apply_document_changes_impl(
    state: &mut ServerState,
    uri_norm: &Url,
    version: i32,
    content_changes: Vec<TextDocumentContentChangeEvent>,
    evaluate: bool,
) -> Vec<(MessageType, String)> {
    let (content_changed, mut runtime_warnings) =
        apply_document_content_edit(state, uri_norm, version, content_changes);

    if content_changed {
        let content = state
            .index
            .get(uri_norm)
            .map(|entry| entry.content.clone())
            .unwrap_or_default();
        let parse_start = Instant::now();
        let parsed_result = util::parse_for_editor(&content);
        let parse_time_ms = elapsed_ms(parse_start);
        runtime_warnings.extend(apply_parsed_document_update(
            state,
            uri_norm,
            version,
            parsed_result,
            parse_time_ms,
            evaluate,
        ));
    }

    runtime_warnings
}

#[cfg(test)]
pub(crate) fn apply_document_changes(
    state: &mut ServerState,
    uri_norm: &Url,
    version: i32,
    content_changes: Vec<TextDocumentContentChangeEvent>,
) -> Vec<(MessageType, String)> {
    apply_document_changes_impl(state, uri_norm, version, content_changes, true)
}

#[cfg(test)]
pub(crate) fn apply_document_changes_fast(
    state: &mut ServerState,
    uri_norm: &Url,
    version: i32,
    content_changes: Vec<TextDocumentContentChangeEvent>,
) -> Vec<(MessageType, String)> {
    apply_document_changes_impl(state, uri_norm, version, content_changes, false)
}

pub(crate) fn remove_document(state: &mut impl DocumentStore, uri_norm: &Url) {
    state.index_mut().remove(uri_norm);
    state.symbol_table_mut().retain(|entry| entry.uri != *uri_norm);
    state.semantic_graph_mut().remove_nodes_for_uri(uri_norm);
}

fn semantic_graph_uris(index: &std::collections::HashMap<Url, IndexEntry>) -> Vec<Url> {
    let (workspace, library) = semantic_graph_uris_split(index, &[]);
    let mut uris = workspace;
    uris.extend(library);
    uris
}

fn semantic_graph_uris_split(
    index: &std::collections::HashMap<Url, IndexEntry>,
    library_paths: &[Url],
) -> (Vec<Url>, Vec<Url>) {
    let mut workspace = Vec::new();
    let mut library = Vec::new();
    for (uri, entry) in index {
        if !entry.include_in_semantic_graph {
            continue;
        }
        if util::uri_under_any_library(uri, library_paths) {
            library.push(uri.clone());
        } else {
            workspace.push(uri.clone());
        }
    }
    workspace.sort();
    library.sort();
    (workspace, library)
}

/// Scan configured library roots for `sysml/librarySearch` without merging the full tree into the semantic graph.
pub(crate) fn index_library_paths_for_search(
    state: &mut impl DocumentStore,
    library_paths: &[Url],
) -> usize {
    if library_paths.is_empty() {
        return 0;
    }
    let (entries, _) = crate::workspace::scan::scan_sysml_files(library_paths.to_vec());
    if entries.is_empty() {
        return 0;
    }
    let parsed_entries = parse_scanned_entries(entries, false, None);
    let mut indexed = 0usize;
    for entry in parsed_entries {
        let uri_norm = crate::common::util::normalize_file_uri(&entry.uri);
        if state.index().contains_key(&uri_norm) {
            continue;
        }
        state.index_mut().insert(
            uri_norm.clone(),
            IndexEntry {
                content: entry.content.clone(),
                parsed: entry.parsed,
                parse_metadata: entry.parse_metadata,
                include_in_semantic_graph: false,
            },
        );
        let mut symbols = Vec::new();
        library_search::add_short_name_symbol_entries(&mut symbols, &entry.content, &uri_norm);
        state.symbol_table_mut().extend(symbols);
        indexed += 1;
    }
    indexed
}

/// Build `(source_kind, WorkspaceParsedDocument)` entries for `uris` from `index`, for
/// feeding into `IncrementalWorkspace::load_parsed`/`load_parsed_from`. URIs with no parsed
/// content (or missing from `index`) are silently skipped, matching this module's prior
/// behavior of only merging documents that parsed successfully.
fn parsed_entries_for_uris(
    index: &std::collections::HashMap<Url, IndexEntry>,
    uris: &[Url],
    kind: SysmlDocumentSourceKind,
) -> Vec<(SysmlDocumentSourceKind, WorkspaceParsedDocument)> {
    uris.iter()
        .filter_map(|uri| {
            let entry = index.get(uri)?;
            let parsed = entry.parsed.clone()?;
            Some((
                kind,
                WorkspaceParsedDocument {
                    uri: uri.clone(),
                    content: entry.content.clone(),
                    parsed,
                    parse_time_ms: entry.parse_metadata.parse_time_ms,
                    parse_cached: entry.parse_metadata.parse_cached,
                },
            ))
        })
        .collect()
}

/// Load import-closure library files for the current workspace index (semantic graph merge).
///
/// Delegates the merge/link computation to `workspace::IncrementalWorkspace` (Tier 2
/// unified-incremental-engine Phase 4) instead of hand-rolling it — the same primitives
/// `sysml_model::build_and_link_graph_parallel` and `workspace::Spec42Engine`'s full-load
/// path already use, closing the last hand-copied sequence of this shape. `IndexEntry` has
/// no workspace/library distinction (only `include_in_semantic_graph`), and this function
/// has never applied qualified-name shadowing between "workspace" and "library" files — every
/// included URI is tagged `Workspace` here, so `load_parsed` merges all of them uniformly via
/// plain `SemanticGraph::merge`, exactly matching this function's prior behavior.
pub(crate) fn rebuild_all_document_links(
    state: &mut impl DocumentStore,
) -> RebuildAllDocumentLinksMetrics {
    let total_start = Instant::now();
    let uris: Vec<Url> = semantic_graph_uris(state.index());
    let entries = parsed_entries_for_uris(state.index(), &uris, SysmlDocumentSourceKind::Workspace);

    let rebuild_start = Instant::now();
    let mut engine = IncrementalWorkspace::new();
    engine.load_parsed(entries);
    let graph = engine.graph();
    let rebuild_ms = elapsed_ms(rebuild_start);

    let refresh_symbols_start = Instant::now();
    let mut all_symbols = Vec::new();
    for (uri, index_entry) in state.index() {
        if !index_entry.include_in_semantic_graph {
            let mut search_symbols = Vec::new();
            library_search::add_short_name_symbol_entries(
                &mut search_symbols,
                &index_entry.content,
                uri,
            );
            all_symbols.extend(search_symbols);
            continue;
        }
        let mut new_entries = semantic::symbol_entries_for_uri(&graph, uri);
        library_search::add_short_name_symbol_entries(&mut new_entries, &index_entry.content, uri);
        all_symbols.extend(new_entries);
    }
    let uri_count = state.index().len();
    *state.symbol_table_mut() = all_symbols;
    *state.semantic_graph_mut() = graph;
    let refresh_symbols_ms = elapsed_ms(refresh_symbols_start);

    // The 7-phase breakdown this metrics struct used to carry (remove-nodes, rebuild-graphs,
    // cross-edge-resolution, workspace-relationship-linking, pending-relationship-resolution,
    // expression-evaluation) lived inside this function's own hand-written sequence. Now
    // that the graph computation is one delegated call into `IncrementalWorkspace`, those
    // phases aren't separately timed here anymore — deliberate, to avoid re-implementing
    // `link_parsed_documents_parallel`'s internals a second time just to get timing points
    // (see the Tier 2 unified-incremental-engine design doc's Phase 4 write-up). The combined
    // time is reported as `cross_document_edges_ms`, matching its pre-existing role as this
    // function's "whole graph computation" umbrella field, so downstream log consumers keep
    // a meaningful (if coarser) number instead of a silent `0`.
    RebuildAllDocumentLinksMetrics {
        uri_count,
        parsed_doc_count: uris.len(),
        remove_nodes_ms: 0,
        rebuild_graphs_ms: 0,
        cross_edge_resolution_ms: 0,
        workspace_relationship_linking_ms: 0,
        pending_relationship_resolution_ms: 0,
        expression_evaluation_ms: 0,
        cross_document_edges_ms: rebuild_ms,
        refresh_symbols_ms,
        total_ms: elapsed_ms(total_start),
    }
}

/// A staged version of rebuild_all_document_links that operates on a consistent snapshot
/// and returns the results to be committed. This allows the heavy lifting (parsing,
/// graph building, relinking) to happen WITHOUT holding a write lock on ServerState.
///
/// See `rebuild_all_document_links`'s doc comment for why this delegates to
/// `IncrementalWorkspace` now. Unlike that function, this one does have a real
/// workspace/library distinction (`workspace_uris`/`library_uris`) and a `base_graph` reuse
/// path (library-graph-cache hit) — both preserved via `SysmlDocumentSourceKind` tagging and
/// `IncrementalWorkspace::load_parsed_from`.
pub(crate) fn rebuild_semantic_graph_staged(
    index: &std::collections::HashMap<Url, IndexEntry>,
    library_paths: &[Url],
    base_graph: Option<semantic::SemanticGraph>,
) -> (
    semantic::SemanticGraph,
    Vec<crate::language::SymbolEntry>,
    RebuildAllDocumentLinksMetrics,
) {
    let total_start = Instant::now();
    let (workspace_uris, library_uris) = semantic_graph_uris_split(index, library_paths);
    // On cache hit the index only has workspace entries, so library_uris will be empty.
    // We still collect them to compute the total URI set for cross-doc resolution.
    let cached_library_uris: Vec<Url> = if let Some(ref bg) = base_graph {
        bg.all_uris()
    } else {
        Vec::new()
    };
    let uris: Vec<Url> = workspace_uris
        .iter()
        .chain(library_uris.iter())
        .chain(cached_library_uris.iter())
        .cloned()
        .collect();

    let rebuild_start = Instant::now();
    let mut entries =
        parsed_entries_for_uris(index, &workspace_uris, SysmlDocumentSourceKind::Workspace);
    entries.extend(parsed_entries_for_uris(
        index,
        &library_uris,
        SysmlDocumentSourceKind::Library,
    ));
    // If a base graph was provided (library graph cache hit), start from it so library nodes
    // are already present for cross-document resolution — `load_parsed_from` resolves
    // cross-document edges against the base graph's existing URIs too, not just `entries`'.
    let mut engine = IncrementalWorkspace::new();
    engine.load_parsed_from(base_graph.unwrap_or_default(), entries);
    let semantic_graph = engine.graph();
    let rebuild_ms = elapsed_ms(rebuild_start);

    let refresh_symbols_start = Instant::now();
    let mut all_symbols = Vec::new();
    for (uri, index_entry) in index {
        if !index_entry.include_in_semantic_graph {
            let mut search_symbols = Vec::new();
            library_search::add_short_name_symbol_entries(
                &mut search_symbols,
                &index_entry.content,
                uri,
            );
            all_symbols.extend(search_symbols);
            continue;
        }
        let mut new_entries = semantic::symbol_entries_for_uri(&semantic_graph, uri);
        library_search::add_short_name_symbol_entries(&mut new_entries, &index_entry.content, uri);
        all_symbols.extend(new_entries);
    }
    let refresh_symbols_ms = elapsed_ms(refresh_symbols_start);

    // See `rebuild_all_document_links` for why the 7-phase breakdown collapses to
    // `cross_document_edges_ms` now.
    let metrics = RebuildAllDocumentLinksMetrics {
        uri_count: index.len(),
        parsed_doc_count: uris.len(),
        remove_nodes_ms: 0,
        rebuild_graphs_ms: 0,
        cross_edge_resolution_ms: 0,
        workspace_relationship_linking_ms: 0,
        pending_relationship_resolution_ms: 0,
        expression_evaluation_ms: 0,
        cross_document_edges_ms: rebuild_ms,
        refresh_symbols_ms,
        total_ms: elapsed_ms(total_start),
    };

    (semantic_graph, all_symbols, metrics)
}

pub(crate) fn clear_documents_under_roots(
    state: &mut impl DocumentStore,
    roots: &[Url],
) -> Vec<Url> {
    let uris_to_remove: Vec<Url> = state
        .index()
        .keys()
        .filter(|uri| util::uri_under_any_library(uri, roots))
        .cloned()
        .collect();
    for uri in &uris_to_remove {
        remove_document(state, uri);
    }
    uris_to_remove
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use std::fs;

    use super::{
        apply_document_changes, apply_document_changes_fast, rebuild_all_document_links,
        remove_document, store_document_text,
    };
    use crate::analysis::compute_semantic_diagnostics;
    use crate::workspace::state::ServerState;
    use tower_lsp::lsp_types::{
        NumberOrString, Position, Range, TextDocumentContentChangeEvent, Url,
    };

    fn fixture_uri() -> Url {
        Url::parse("file:///C:/workspace/test.sysml").expect("fixture uri")
    }

    fn find_attribute_node<'a>(
        state: &'a ServerState,
        uri: &Url,
        name: &str,
    ) -> &'a crate::semantic::SemanticNode {
        state
            .semantic_graph
            .nodes_for_uri(uri)
            .into_iter()
            .find(|node| node.element_kind == "attribute" && node.name == name)
            .expect("attribute node")
    }

    fn register_units_library_document(state: &mut ServerState) -> Url {
        const UNITS_CATALOG: &str = r#"
            package Units {
                attribute <m> 'metre' : LengthUnit;
                attribute <cm> 'centimetre' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 1E-02; } }
                attribute <ft> 'foot' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 3.048E-01; } }
            }
        "#;
        let temp = tempfile::tempdir().expect("temp dir");
        let library_root = temp.path().canonicalize().expect("canonical library root");
        let units_path = library_root
            .join("sysml.library")
            .join("Domain Libraries")
            .join("Quantities and Units")
            .join("FixtureUnits.sysml");
        fs::create_dir_all(
            units_path
                .parent()
                .expect("fixture units parent directory exists"),
        )
        .expect("create units fixture directory");
        fs::write(&units_path, UNITS_CATALOG).expect("write units fixture");
        let units_uri = Url::from_file_path(&units_path).expect("units uri");
        // Keep the temporary directory alive for the lifetime of the test process.
        std::mem::forget(temp);
        let warning = store_document_text(state, &units_uri, UNITS_CATALOG.to_string());
        assert!(warning.is_none());
        units_uri
    }

    #[test]
    fn store_apply_and_remove_document_keeps_index_and_symbol_table_in_sync() {
        let uri = fixture_uri();
        let mut state = ServerState::default();

        let warning = store_document_text(
            &mut state,
            &uri,
            "package Demo { part def Engine; part motor : Engine; }".to_string(),
        );
        assert!(warning.is_none());
        assert!(state.index.contains_key(&uri));
        let first_entry = state.index.get(&uri).expect("stored doc");
        assert!(first_entry.parse_metadata.parse_time_ms > 0);
        assert!(!first_entry.parse_metadata.parse_cached);
        assert!(!state.symbol_table.is_empty());
        assert!(!state.semantic_graph.nodes_for_uri(&uri).is_empty());

        let warnings = apply_document_changes(
            &mut state,
            &uri,
            2,
            vec![TextDocumentContentChangeEvent {
                range: Some(Range::new(Position::new(0, 24), Position::new(0, 30))),
                range_length: None,
                text: "Motor".to_string(),
            }],
        );
        assert!(warnings.is_empty());
        assert!(state
            .index
            .get(&uri)
            .expect("updated doc")
            .content
            .contains("Motor"));
        assert!(
            state
                .index
                .get(&uri)
                .expect("updated doc")
                .parse_metadata
                .parse_time_ms
                > 0
        );
        assert!(
            !state
                .index
                .get(&uri)
                .expect("updated doc")
                .parse_metadata
                .parse_cached
        );

        remove_document(&mut state, &uri);
        assert!(!state.index.contains_key(&uri));
        assert!(state.semantic_graph.nodes_for_uri(&uri).is_empty());
        assert!(state.symbol_table.iter().all(|entry| entry.uri != uri));
    }

    #[test]
    fn fast_apply_updates_document_without_running_workspace_evaluation() {
        let uri = fixture_uri();
        let mut state = ServerState::default();

        store_document_text(
            &mut state,
            &uri,
            "package Demo { part def Rocket { attribute mass = 1 + 1; } }".to_string(),
        );
        let mass = find_attribute_node(&state, &uri, "mass");
        assert_eq!(
            mass.attributes.get("evaluatedValue"),
            Some(&serde_json::json!(2))
        );

        let warnings = apply_document_changes_fast(
            &mut state,
            &uri,
            2,
            vec![TextDocumentContentChangeEvent {
                range: Some(Range::new(Position::new(0, 54), Position::new(0, 55))),
                range_length: None,
                text: "2".to_string(),
            }],
        );
        assert!(warnings.is_empty());
        assert!(state
            .index
            .get(&uri)
            .expect("updated doc")
            .content
            .contains("1 + 2"));
        let mass = find_attribute_node(&state, &uri, "mass");
        assert!(
            !mass.attributes.contains_key("evaluatedValue"),
            "fast path should defer expression evaluation until async relink"
        );
    }

    #[test]
    fn rebuild_all_document_links_relinks_library_documents_after_dependency_ingest() {
        let temp = tempfile::tempdir().expect("temp dir");
        let library_root = temp.path().canonicalize().expect("canonical library root");
        let importer_uri =
            Url::from_file_path(library_root.join("AImporter.sysml")).expect("importer uri");
        let dependency_uri =
            Url::from_file_path(library_root.join("ZBase.sysml")).expect("dependency uri");
        let library_root_uri = Url::from_file_path(&library_root).expect("library root uri");
        let mut state = ServerState::default();
        state.library_paths = vec![library_root_uri];

        store_document_text(
            &mut state,
            &importer_uri,
            "package Demo { import Base::*; part def RuntimeCluster { attribute clusterName : Name; } }"
                .to_string(),
        );
        store_document_text(
            &mut state,
            &dependency_uri,
            "package Base { attribute def Name; }".to_string(),
        );

        rebuild_all_document_links(&mut state);

        let rebuilt_diagnostics = compute_semantic_diagnostics(
            &state.semantic_graph,
            &importer_uri,
            crate::DiagnosticsHostContext,
        );
        assert!(
            rebuilt_diagnostics.iter().all(|d| {
                d.code
                    .as_ref()
                    .is_none_or(|code| {
                        !matches!(
                            code,
                            NumberOrString::String(value) if value == "unresolved_type_reference"
                        )
                    })
            }),
            "expected no unresolved_type_reference after full relink, got: {rebuilt_diagnostics:#?}"
        );
    }

    #[test]
    fn rebuild_all_document_links_relinks_public_reexport_chains_after_dependency_ingest() {
        let temp = tempfile::tempdir().expect("temp dir");
        let library_root = temp.path().canonicalize().expect("canonical library root");
        let importer_uri =
            Url::from_file_path(library_root.join("CImporter.sysml")).expect("importer uri");
        let reexport_uri =
            Url::from_file_path(library_root.join("BReexport.sysml")).expect("reexport uri");
        let dependency_uri =
            Url::from_file_path(library_root.join("ABase.sysml")).expect("dependency uri");
        let library_root_uri = Url::from_file_path(&library_root).expect("library root uri");
        let mut state = ServerState::default();
        state.library_paths = vec![library_root_uri];

        store_document_text(
            &mut state,
            &importer_uri,
            "package Consumer { import Domain::*; part def RuntimeCluster { attribute clusterName : Name; } }"
                .to_string(),
        );
        store_document_text(
            &mut state,
            &reexport_uri,
            "package Domain { public import Base::*; }".to_string(),
        );
        store_document_text(
            &mut state,
            &dependency_uri,
            "package Base { attribute def Name; }".to_string(),
        );

        rebuild_all_document_links(&mut state);

        let rebuilt_diagnostics = compute_semantic_diagnostics(
            &state.semantic_graph,
            &importer_uri,
            crate::DiagnosticsHostContext,
        );
        assert!(
            rebuilt_diagnostics.iter().all(|d| {
                d.code.as_ref().is_none_or(|code| {
                    !matches!(
                        code,
                        NumberOrString::String(value) if value == "unresolved_type_reference"
                    )
                })
            }),
            "expected no unresolved_type_reference after public re-export relink, got: {rebuilt_diagnostics:#?}"
        );
    }

    #[test]
    fn store_document_text_persists_evaluated_attributes() {
        let uri = fixture_uri();
        let mut state = ServerState::default();
        let warning = store_document_text(
            &mut state,
            &uri,
            "package Demo { part def Rocket { attribute mass = (1 + 2); } }".to_string(),
        );
        assert!(warning.is_none());

        let mass = find_attribute_node(&state, &uri, "mass");
        assert_eq!(
            mass.attributes.get("evaluationStatus"),
            Some(&serde_json::json!("ok"))
        );
        assert_eq!(
            mass.attributes.get("evaluatedValue"),
            Some(&serde_json::json!(3))
        );
        assert!(
            !mass.attributes.contains_key("evaluatedUnit"),
            "phase-1 arithmetic without unit should not emit evaluatedUnit"
        );
    }

    #[test]
    fn rebuild_all_document_links_recomputes_evaluated_attributes() {
        let uri = fixture_uri();
        let mut state = ServerState::default();
        store_document_text(
            &mut state,
            &uri,
            "package Demo { part def Rocket { attribute mass = (8 + 4) / 3; } }".to_string(),
        );

        rebuild_all_document_links(&mut state);

        let mass = find_attribute_node(&state, &uri, "mass");
        assert_eq!(
            mass.attributes.get("evaluationStatus"),
            Some(&serde_json::json!("ok"))
        );
        assert_eq!(
            mass.attributes.get("evaluatedValue"),
            Some(&serde_json::json!(4))
        );
    }

    #[test]
    fn store_document_text_resolves_referenced_attributes() {
        let uri = fixture_uri();
        let mut state = ServerState::default();
        let warning = store_document_text(
            &mut state,
            &uri,
            "package Demo { part def Rocket { attribute base = 10; attribute mass = base + 5; } }"
                .to_string(),
        );
        assert!(warning.is_none());

        let mass = find_attribute_node(&state, &uri, "mass");
        assert_eq!(
            mass.attributes.get("evaluationStatus"),
            Some(&serde_json::json!("ok"))
        );
        assert_eq!(
            mass.attributes.get("evaluatedValue"),
            Some(&serde_json::json!(15))
        );
    }

    #[test]
    fn rebuild_all_document_links_recomputes_referenced_attributes() {
        let uri = fixture_uri();
        let mut state = ServerState::default();
        store_document_text(
            &mut state,
            &uri,
            "package Demo { part def Rocket { attribute base = 20; attribute offset = base + 2; attribute mass = offset + 3; } }"
                .to_string(),
        );

        rebuild_all_document_links(&mut state);

        let mass = find_attribute_node(&state, &uri, "mass");
        assert_eq!(
            mass.attributes.get("evaluationStatus"),
            Some(&serde_json::json!("ok"))
        );
        assert_eq!(
            mass.attributes.get("evaluatedValue"),
            Some(&serde_json::json!(25))
        );
    }

    #[test]
    fn store_document_text_evaluates_unit_conversions() {
        let mut state = ServerState::default();
        let _units_uri = register_units_library_document(&mut state);
        let uri = fixture_uri();
        let warning = store_document_text(
            &mut state,
            &uri,
            "package Demo { part def Rocket { attribute distance = 1 [m] + 50 [cm] + 1 [ft]; } }"
                .to_string(),
        );
        assert!(warning.is_none());

        let distance = find_attribute_node(&state, &uri, "distance");
        assert_eq!(
            distance.attributes.get("evaluationStatus"),
            Some(&serde_json::json!("ok")),
            "distance attributes: {:#?}",
            distance.attributes
        );
        assert_eq!(
            distance.attributes.get("evaluatedUnit"),
            Some(&serde_json::json!("m"))
        );
        let value = distance
            .attributes
            .get("evaluatedValue")
            .and_then(serde_json::Value::as_f64)
            .expect("evaluated numeric value");
        assert!(
            (value - 1.8048).abs() < 1e-9,
            "expected 1.8048 m after conversion, got {value}"
        );
    }

    #[test]
    fn rebuild_all_document_links_recomputes_unit_conversions() {
        let mut state = ServerState::default();
        let _units_uri = register_units_library_document(&mut state);
        let uri = fixture_uri();
        store_document_text(
            &mut state,
            &uri,
            "package Demo { part def Rocket { attribute distance = 1 [m] + 1 [ft]; } }".to_string(),
        );

        rebuild_all_document_links(&mut state);

        let distance = find_attribute_node(&state, &uri, "distance");
        assert_eq!(
            distance.attributes.get("evaluationStatus"),
            Some(&serde_json::json!("ok")),
            "distance attributes after rebuild: {:#?}",
            distance.attributes
        );
        assert_eq!(
            distance.attributes.get("evaluatedUnit"),
            Some(&serde_json::json!("m"))
        );
        let value = distance
            .attributes
            .get("evaluatedValue")
            .and_then(serde_json::Value::as_f64)
            .expect("evaluated numeric value");
        assert!(
            (value - 1.3048).abs() < 1e-9,
            "expected 1.3048 m after rebuild conversion, got {value}"
        );
    }
}
