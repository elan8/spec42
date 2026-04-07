use crate::common::util;
use crate::semantic_model;
use crate::workspace::library_search;
use crate::workspace::state::{IndexEntry, ParseMetadata, ScanSummary, ServerState};
use std::time::Instant;
use sysml_parser::RootNamespace;
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

fn update_semantic_graph_for_uri(state: &mut ServerState, uri: &Url, doc: Option<&RootNamespace>) {
    state.semantic_graph.remove_nodes_for_uri(uri);
    if let Some(doc) = doc {
        let new_graph = semantic_model::build_graph_from_doc(doc, uri);
        state.semantic_graph.merge(new_graph);
        semantic_model::add_cross_document_edges_for_uri(&mut state.semantic_graph, uri);
    }
}

fn refresh_symbols_for_uri(state: &mut ServerState, uri: &Url) {
    let mut new_entries = semantic_model::symbol_entries_for_uri(&state.semantic_graph, uri);
    if let Some(index_entry) = state.index.get(uri) {
        library_search::add_short_name_symbol_entries(&mut new_entries, &index_entry.content, uri);
    }
    update_symbol_table_for_uri(state, uri, Some(&new_entries));
}

pub(crate) fn store_parsed_document_text(
    state: &mut ServerState,
    uri_norm: &Url,
    text: String,
    parsed: Option<RootNamespace>,
    parse_metadata: ParseMetadata,
    parse_errors: &[String],
    diagnostic_count: usize,
    context: &str,
) -> Option<String> {
    update_semantic_graph_for_uri(state, uri_norm, parsed.as_ref());
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
        update_semantic_graph_for_uri(state, uri_norm, parsed.as_ref());
        refresh_symbols_for_uri(state, uri_norm);
    }

    runtime_warnings
}

pub(crate) fn remove_document(state: &mut ServerState, uri_norm: &Url) {
    state.index.remove(uri_norm);
    state.symbol_table.retain(|entry| entry.uri != *uri_norm);
    state.semantic_graph.remove_nodes_for_uri(uri_norm);
}

pub(crate) fn rebuild_document_links(state: &mut ServerState, uri_norm: &Url) {
    let parsed = state
        .index
        .get(uri_norm)
        .and_then(|entry| entry.parsed.as_ref())
        .cloned();
    state.semantic_graph.remove_nodes_for_uri(uri_norm);
    if let Some(doc) = parsed.as_ref() {
        let new_graph = semantic_model::build_graph_from_doc(doc, uri_norm);
        state.semantic_graph.merge(new_graph);
        semantic_model::add_cross_document_edges_for_uri(&mut state.semantic_graph, uri_norm);
    }
    refresh_symbols_for_uri(state, uri_norm);
}

pub(crate) fn rebuild_non_library_document_links(state: &mut ServerState) {
    let uris: Vec<Url> = state
        .index
        .keys()
        .filter(|uri| !util::uri_under_any_library(uri, &state.library_paths))
        .cloned()
        .collect();
    for uri in uris {
        rebuild_document_links(state, &uri);
    }
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
mod tests {
    use super::{apply_document_changes, remove_document, store_document_text};
    use crate::workspace::state::ServerState;
    use tower_lsp::lsp_types::{Position, Range, TextDocumentContentChangeEvent, Url};

    fn fixture_uri() -> Url {
        Url::parse("file:///C:/workspace/test.sysml").expect("fixture uri")
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
}
