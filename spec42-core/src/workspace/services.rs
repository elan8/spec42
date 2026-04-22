use crate::common::util;
use tracing::warn;
use crate::semantic_model;
use crate::workspace::library_search;
use crate::workspace::state::{IndexEntry, ParseMetadata, ScanSummary, ServerState};
use std::time::Instant;
use sysml_v2_parser::RootNamespace;
use tower_lsp::lsp_types::{MessageType, TextDocumentContentChangeEvent, Url};
use walkdir::WalkDir;

fn elapsed_ms(start: Instant) -> u32 {
    start.elapsed().as_millis().max(1) as u32
}

pub(crate) fn indexed_text(state: &ServerState, uri_norm: &Url) -> Option<String> {
    state.index.get(uri_norm).map(|entry| entry.content.clone())
}

pub(crate) fn indexed_text_or_empty(state: &ServerState, uri_norm: &Url) -> String {
    indexed_text(state, uri_norm).unwrap_or_default()
}

pub(crate) fn scan_sysml_files(roots: Vec<Url>) -> (Vec<(Url, String)>, ScanSummary) {
    let mut out = Vec::new();
    let mut summary = ScanSummary::default();
    for root in roots {
        summary.roots_scanned += 1;
        let path = match root.to_file_path() {
            Ok(path) => path,
            Err(_) => {
                summary.roots_skipped_non_file += 1;
                continue;
            }
        };
        for entry in WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_map(|entry| entry.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }
            let ext = entry.path().extension().and_then(|ext| ext.to_str());
            if ext != Some("sysml") && ext != Some("kerml") {
                continue;
            }
            summary.candidate_files += 1;
            match std::fs::read_to_string(entry.path()) {
                Ok(content) => match Url::from_file_path(entry.path()) {
                    Ok(uri) => {
                        summary.files_loaded += 1;
                        out.push((uri, content));
                    }
                    Err(_) => summary.uri_failures += 1,
                },
                Err(_) => summary.read_failures += 1,
            }
        }
    }
    (out, summary)
}

#[derive(Debug)]
pub(crate) struct ParsedScanEntry {
    pub(crate) ordinal: usize,
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

fn parse_scanned_entry(ordinal: usize, uri: Url, content: String) -> ParsedScanEntry {
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
        ordinal,
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
) -> Vec<ParsedScanEntry> {
    if entries.is_empty() {
        return Vec::new();
    }

    if !parallel_enabled || entries.len() < 2 {
        return entries
            .into_iter()
            .enumerate()
            .map(|(ordinal, (uri, content))| parse_scanned_entry(ordinal, uri, content))
            .collect();
    }

    let worker_count = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1)
        .min(entries.len())
        .max(1);

    let mut buckets: Vec<Vec<(usize, Url, String)>> =
        (0..worker_count).map(|_| Vec::new()).collect();
    for (ordinal, (uri, content)) in entries.into_iter().enumerate() {
        buckets[ordinal % worker_count].push((ordinal, uri, content));
    }

    let mut handles = Vec::with_capacity(worker_count);
    for bucket in buckets {
        handles.push(std::thread::spawn(move || {
            bucket
                .into_iter()
                .map(|(ordinal, uri, content)| parse_scanned_entry(ordinal, uri, content))
                .collect::<Vec<ParsedScanEntry>>()
        }));
    }

    let mut parsed_entries = Vec::new();
    for handle in handles {
        let mut batch = handle.join().unwrap_or_default();
        parsed_entries.append(&mut batch);
    }
    parsed_entries.sort_by_key(|entry| entry.ordinal);
    parsed_entries
}

fn update_symbol_table_for_uri(
    state: &mut ServerState,
    uri: &Url,
    new_entries: Option<&[crate::language::SymbolEntry]>,
) {
    state.symbol_table.retain(|entry| entry.uri != *uri);
    if let Some(entries) = new_entries {
        state.symbol_table.extend(entries.iter().cloned());
    }
}

fn update_semantic_graph_for_uri(
    state: &mut ServerState,
    uri: &Url,
    doc: Option<&RootNamespace>,
    evaluate: bool,
) {
    state.semantic_graph.remove_nodes_for_uri(uri);
    if let Some(doc) = doc {
        let new_graph = semantic_model::build_graph_from_doc(doc, uri);
        state.semantic_graph.merge(new_graph);
        semantic_model::add_cross_document_edges_for_uri(&mut state.semantic_graph, uri);
        if evaluate {
            semantic_model::evaluate_expressions(&mut state.semantic_graph);
        }
    }
}

fn refresh_symbols_for_uri(state: &mut ServerState, uri: &Url) {
    let mut new_entries = semantic_model::symbol_entries_for_uri(&state.semantic_graph, uri);
    if let Some(index_entry) = state.index.get(uri) {
        library_search::add_short_name_symbol_entries(&mut new_entries, &index_entry.content, uri);
    }
    update_symbol_table_for_uri(state, uri, Some(&new_entries));
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn store_parsed_document_text(
    state: &mut ServerState,
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
    state.index.insert(
        uri_norm.clone(),
        IndexEntry {
            content: text,
            parsed,
            parse_metadata,
        },
    );
    refresh_symbols_for_uri(state, uri_norm);
    warning_from_parse_errors(uri_norm, parse_errors, diagnostic_count, context)
}

pub(crate) fn store_document_text(
    state: &mut ServerState,
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

pub(crate) fn refresh_document(
    state: &mut ServerState,
    uri_norm: &Url,
    content: String,
) -> Option<String> {
    store_document_text(state, uri_norm, content)
}

pub(crate) fn ingest_parsed_scan_entries(
    state: &mut ServerState,
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
    semantic_model::evaluate_expressions(&mut state.semantic_graph);
    loaded
}

/// A faster version of ingest_parsed_scan_entries that avoids per-document relinking/evaluation.
/// Intended for use during startup when a full relink is performed immediately after.
pub(crate) fn ingest_parsed_scan_entries_batch(
    state: &mut ServerState,
    entries: Vec<ParsedScanEntry>,
) -> Vec<(Url, Option<String>)> {
    let mut loaded = Vec::with_capacity(entries.len());
    for entry in entries {
        let uri_norm = util::normalize_file_uri(&entry.uri);
        state.index.insert(
            uri_norm.clone(),
            IndexEntry {
                content: entry.content,
                parsed: entry.parsed,
                parse_metadata: entry.parse_metadata,
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

pub(crate) fn apply_document_changes(
    state: &mut ServerState,
    uri_norm: &Url,
    version: i32,
    content_changes: Vec<TextDocumentContentChangeEvent>,
) -> Vec<(MessageType, String)> {
    let mut runtime_warnings = Vec::new();
    let should_update = if let Some(entry) = state.index.get_mut(uri_norm) {
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
        if content_changed {
            let parse_start = Instant::now();
            let parsed_result = util::parse_for_editor(&entry.content);
            entry.parsed = Some(parsed_result.root);
            entry.parse_metadata = ParseMetadata {
                parse_time_ms: elapsed_ms(parse_start),
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

    if should_update {
        let parsed = state
            .index
            .get(uri_norm)
            .and_then(|entry| entry.parsed.as_ref())
            .cloned();
        update_semantic_graph_for_uri(state, uri_norm, parsed.as_ref(), true);
        refresh_symbols_for_uri(state, uri_norm);
    }

    runtime_warnings
}

pub(crate) fn remove_document(state: &mut ServerState, uri_norm: &Url) {
    state.index.remove(uri_norm);
    state.symbol_table.retain(|entry| entry.uri != *uri_norm);
    state.semantic_graph.remove_nodes_for_uri(uri_norm);
}

pub(crate) fn rebuild_all_document_links(
    state: &mut ServerState,
) -> RebuildAllDocumentLinksMetrics {
    let total_start = Instant::now();
    let uris: Vec<Url> = state.index.keys().cloned().collect();
    let parsed_docs: Vec<(Url, RootNamespace)> = uris
        .iter()
        .filter_map(|uri| {
            state
                .index
                .get(uri)
                .and_then(|entry| entry.parsed.as_ref())
                .cloned()
                .map(|parsed| (uri.clone(), parsed))
        })
        .collect();

    let remove_nodes_start = Instant::now();
    for uri in &uris {
        state.semantic_graph.remove_nodes_for_uri(uri);
    }
    let remove_nodes_ms = elapsed_ms(remove_nodes_start);

    let rebuild_graphs_start = Instant::now();
    let worker_count = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1)
        .min(parsed_docs.len())
        .max(1);

    let mut buckets: Vec<Vec<(Url, RootNamespace)>> = (0..worker_count).map(|_| Vec::new()).collect();
    for (i, item) in parsed_docs.into_iter().enumerate() {
        buckets[i % worker_count].push(item);
    }

    let mut handles = Vec::with_capacity(worker_count);
    for bucket in buckets {
        handles.push(std::thread::spawn(move || {
            bucket
                .into_iter()
                .map(|(uri, parsed)| (uri.clone(), semantic_model::build_graph_from_doc(&parsed, &uri)))
                .collect::<Vec<_>>()
        }));
    }

    for handle in handles {
        let batch = handle.join().unwrap_or_default();
        for (_uri, g) in batch {
            state.semantic_graph.merge(g);
        }
    }
    let rebuild_graphs_ms = elapsed_ms(rebuild_graphs_start);

    let cross_document_edges_start = Instant::now();
    let worker_count = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1)
        .min(uris.len())
        .max(1);

    let mut uri_buckets: Vec<Vec<Url>> = (0..worker_count).map(|_| Vec::new()).collect();
    for (i, uri) in uris.iter().enumerate() {
        uri_buckets[i % worker_count].push(uri.clone());
    }

    let mut cross_handles = Vec::with_capacity(worker_count);
    // Move the graph into an Arc for shared read access in workers
    let graph_arc = std::sync::Arc::new(std::mem::take(&mut state.semantic_graph));

    for bucket in uri_buckets {
        let graph_ref = graph_arc.clone();
        cross_handles.push(std::thread::spawn(move || {
            let mut edges = Vec::new();
            for uri in bucket {
                edges.extend(semantic_model::resolve_cross_document_edges_for_uri(
                    &graph_ref, &uri,
                ));
            }
            edges
        }));
    }

    // Move the graph back to state
    state.semantic_graph = std::sync::Arc::try_unwrap(graph_arc).unwrap_or_else(|arc| {
        // This shouldn't happen if all threads finished.
        // If it does, we have to clone or return default.
        warn!("Arc unwrap failed for semantic_graph, cloning instead.");
        (*arc).clone()
    });

    for handle in cross_handles {
        let edges = handle.join().unwrap_or_default();
        for (src_id, tgt_id, kind) in edges {
            if let (Some(&src_idx), Some(&tgt_idx)) = (
                state.semantic_graph.node_index_by_id.get(&src_id),
                state.semantic_graph.node_index_by_id.get(&tgt_id),
            ) {
                state.semantic_graph.graph.add_edge(src_idx, tgt_idx, kind);
            }
        }
    }
    semantic_model::evaluate_expressions(&mut state.semantic_graph);
    let cross_document_edges_ms = elapsed_ms(cross_document_edges_start);

    let refresh_symbols_start = Instant::now();
    let mut all_symbols = Vec::new();
    for uri in &uris {
        let mut new_entries = semantic_model::symbol_entries_for_uri(&state.semantic_graph, uri);
        if let Some(index_entry) = state.index.get(uri) {
            library_search::add_short_name_symbol_entries(
                &mut new_entries,
                &index_entry.content,
                uri,
            );
        }
        all_symbols.extend(new_entries);
    }
    state.symbol_table = all_symbols;
    let refresh_symbols_ms = elapsed_ms(refresh_symbols_start);

    RebuildAllDocumentLinksMetrics {
        uri_count: state.index.len(),
        parsed_doc_count: uris.len(), // Use uris.len() as we processed all requested uris
        remove_nodes_ms,
        rebuild_graphs_ms,
        cross_document_edges_ms,
        refresh_symbols_ms,
        total_ms: elapsed_ms(total_start),
    }
}

/// A staged version of rebuild_all_document_links that operates on a consistent snapshot
/// and returns the results to be committed. This allows the heavy lifting (parsing,
/// graph building, relinking) to happen WITHOUT holding a write lock on ServerState.
pub(crate) fn rebuild_semantic_graph_staged(
    index: &std::collections::HashMap<Url, IndexEntry>,
    _library_paths: &[Url],
) -> (
    semantic_model::SemanticGraph,
    Vec<crate::language::SymbolEntry>,
    RebuildAllDocumentLinksMetrics,
) {
    let total_start = Instant::now();
    let uris: Vec<Url> = index.keys().cloned().collect();
    let parsed_docs: Vec<(Url, RootNamespace)> = uris
        .iter()
        .filter_map(|uri| {
            index
                .get(uri)
                .and_then(|entry| entry.parsed.as_ref())
                .cloned()
                .map(|parsed| (uri.clone(), parsed))
        })
        .collect();

    let mut semantic_graph = semantic_model::SemanticGraph::new();

    let rebuild_graphs_start = Instant::now();
    let worker_count = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1)
        .min(parsed_docs.len())
        .max(1);

    let mut buckets: Vec<Vec<(Url, RootNamespace)>> = (0..worker_count).map(|_| Vec::new()).collect();
    for (i, item) in parsed_docs.into_iter().enumerate() {
        buckets[i % worker_count].push(item);
    }

    let mut handles = Vec::with_capacity(worker_count);
    for bucket in buckets {
        handles.push(std::thread::spawn(move || {
            bucket
                .into_iter()
                .map(|(uri, parsed)| {
                    (
                        uri.clone(),
                        semantic_model::build_graph_from_doc(&parsed, &uri),
                    )
                })
                .collect::<Vec<_>>()
        }));
    }

    for handle in handles {
        let batch = handle.join().unwrap_or_default();
        for (_uri, g) in batch {
            semantic_graph.merge(g);
        }
    }
    let rebuild_graphs_ms = elapsed_ms(rebuild_graphs_start);

    let cross_document_edges_start = Instant::now();
    let mut uri_buckets: Vec<Vec<Url>> = (0..worker_count).map(|_| Vec::new()).collect();
    for (i, uri) in uris.iter().enumerate() {
        uri_buckets[i % worker_count].push(uri.clone());
    }

    let mut cross_handles = Vec::with_capacity(worker_count);
    let graph_arc = std::sync::Arc::new(semantic_graph);

    for bucket in uri_buckets {
        let graph_ref = graph_arc.clone();
        cross_handles.push(std::thread::spawn(move || {
            let mut edges = Vec::new();
            for uri in bucket {
                edges.extend(semantic_model::resolve_cross_document_edges_for_uri(
                    &graph_ref, &uri,
                ));
            }
            edges
        }));
    }

    semantic_graph = std::sync::Arc::try_unwrap(graph_arc).unwrap_or_else(|arc| (*arc).clone());

    for handle in cross_handles {
        let edges = handle.join().unwrap_or_default();
        for (src_id, tgt_id, kind) in edges {
            if let (Some(&src_idx), Some(&tgt_idx)) = (
                semantic_graph.node_index_by_id.get(&src_id),
                semantic_graph.node_index_by_id.get(&tgt_id),
            ) {
                semantic_graph.graph.add_edge(src_idx, tgt_idx, kind);
            }
        }
    }
    semantic_model::evaluate_expressions(&mut semantic_graph);
    let cross_document_edges_ms = elapsed_ms(cross_document_edges_start);

    let refresh_symbols_start = Instant::now();
    let mut all_symbols = Vec::new();
    for uri in &uris {
        let mut new_entries = semantic_model::symbol_entries_for_uri(&semantic_graph, uri);
        if let Some(index_entry) = index.get(uri) {
            library_search::add_short_name_symbol_entries(
                &mut new_entries,
                &index_entry.content,
                uri,
            );
        }
        all_symbols.extend(new_entries);
    }
    let refresh_symbols_ms = elapsed_ms(refresh_symbols_start);

    let metrics = RebuildAllDocumentLinksMetrics {
        uri_count: index.len(),
        parsed_doc_count: uris.len(),
        remove_nodes_ms: 0, // No nodes to remove in a fresh graph
        rebuild_graphs_ms,
        cross_document_edges_ms,
        refresh_symbols_ms,
        total_ms: elapsed_ms(total_start),
    };

    (semantic_graph, all_symbols, metrics)
}

pub(crate) fn clear_documents_under_roots(state: &mut ServerState, roots: &[Url]) -> Vec<Url> {
    let uris_to_remove: Vec<Url> = state
        .index
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
        apply_document_changes, rebuild_all_document_links, remove_document, store_document_text,
    };
    use crate::analysis::checks::compute_semantic_diagnostics;
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
    ) -> &'a crate::semantic_model::SemanticNode {
        state
            .semantic_graph
            .nodes_for_uri(uri)
            .into_iter()
            .find(|node| node.element_kind == "attribute" && node.name == name)
            .expect("attribute node")
    }

    fn register_units_library_document(state: &mut ServerState) -> Url {
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
        fs::write(
            &units_path,
            r#"
            package Units {
                attribute <m> 'metre' : LengthUnit;
                attribute <cm> 'centimetre' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 1E-02; } }
                attribute <ft> 'foot' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 3.048E-01; } }
            }
            "#,
        )
        .expect("write units fixture");
        let units_uri = Url::from_file_path(&units_path).expect("units uri");
        // Keep the temporary directory alive for the lifetime of the test process.
        std::mem::forget(temp);
        let warning = store_document_text(
            state,
            &units_uri,
            "package UnitsFixture { attribute def Marker; }".to_string(),
        );
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

        let rebuilt_diagnostics =
            compute_semantic_diagnostics(&state.semantic_graph, &importer_uri);
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

        let rebuilt_diagnostics =
            compute_semantic_diagnostics(&state.semantic_graph, &importer_uri);
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
