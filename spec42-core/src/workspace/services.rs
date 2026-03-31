use crate::lsp::library_search;
use crate::semantic_model;
use crate::util;
use crate::workspace::state::{IndexEntry, ScanSummary, ServerState};
use sysml_parser::RootNamespace;
use tower_lsp::lsp_types::{MessageType, TextDocumentContentChangeEvent, Url};
use walkdir::WalkDir;

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
}

fn parse_scanned_entry(ordinal: usize, uri: Url, content: String) -> ParsedScanEntry {
    let parsed_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        sysml_parser::parse(&content)
    }));
    let parser_panicked = parsed_result.is_err();
    let parsed = match parsed_result {
        Ok(result) => result.ok(),
        Err(_) => None,
    };
    let mut parse_errors = if parsed.is_none() {
        util::parse_failure_diagnostics(&content, 5)
    } else {
        Vec::new()
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

pub(crate) fn store_document_text(
    state: &mut ServerState,
    uri_norm: &Url,
    text: String,
) -> Option<String> {
    let parsed = sysml_parser::parse(&text).ok();
    let warning = if parsed.is_none() {
        let errs = util::parse_failure_diagnostics(&text, 5);
        Some(if errs.is_empty() {
            format!(
                "sysml parse failed for {} (0 diagnostics; parser returned no AST and no error list)",
                uri_norm.as_str(),
            )
        } else {
            format!(
                "sysml parse failed for {} ({} error(s)): {}",
                uri_norm.as_str(),
                errs.len(),
                errs.join("; "),
            )
        })
    } else {
        None
    };

    update_semantic_graph_for_uri(state, uri_norm, parsed.as_ref());
    state.index.insert(
        uri_norm.clone(),
        IndexEntry {
            content: text,
            parsed,
        },
    );
    refresh_symbols_for_uri(state, uri_norm);
    warning
}

pub(crate) fn refresh_document(
    state: &mut ServerState,
    uri_norm: &Url,
    content: String,
) -> Option<String> {
    store_document_text(state, uri_norm, content)
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
            entry.parsed = sysml_parser::parse(&entry.content).ok();
            if entry.parsed.is_none() {
                let errs = util::parse_failure_diagnostics(&entry.content, 5);
                runtime_warnings.push((
                    MessageType::LOG,
                    if errs.is_empty() {
                        format!(
                            "sysml parse failed after didChange for {} (version {}): parser returned no AST and no diagnostics; keeping diagnostics-only degraded mode.",
                            uri_norm, version
                        )
                    } else {
                        format!(
                            "sysml parse failed after didChange for {} (version {}, {} error(s)): {}; keeping diagnostics-only degraded mode.",
                            uri_norm,
                            version,
                            errs.len(),
                            errs.join("; "),
                        )
                    },
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
        assert!(state.index.get(&uri).expect("updated doc").content.contains("Motor"));

        remove_document(&mut state, &uri);
        assert!(!state.index.contains_key(&uri));
        assert!(state.semantic_graph.nodes_for_uri(&uri).is_empty());
        assert!(state.symbol_table.iter().all(|entry| entry.uri != uri));
    }
}
