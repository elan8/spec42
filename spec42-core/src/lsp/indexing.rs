use crate::lsp::types::{ScanSummary, ServerState};
use crate::semantic_model;
use crate::util;
use sysml_parser::RootNamespace;
use tower_lsp::lsp_types::Url;
use walkdir::WalkDir;

pub(crate) fn scan_sysml_files(roots: Vec<Url>) -> (Vec<(Url, String)>, ScanSummary) {
    let mut out = Vec::new();
    let mut summary = ScanSummary::default();
    for root in roots {
        summary.roots_scanned += 1;
        let path = match root.to_file_path() {
            Ok(p) => p,
            Err(_) => {
                summary.roots_skipped_non_file += 1;
                continue;
            }
        };
        for entry in WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }
            let ext = entry.path().extension().and_then(|e| e.to_str());
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
        .map(|n| n.get())
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

    let mut parsed_entries: Vec<ParsedScanEntry> = Vec::new();
    for handle in handles {
        let mut batch = handle.join().unwrap_or_default();
        parsed_entries.append(&mut batch);
    }
    parsed_entries.sort_by_key(|entry| entry.ordinal);
    parsed_entries
}

/// Removes all symbol table entries for `uri`, then appends `new_entries` if provided.
pub(crate) fn update_symbol_table_for_uri(
    state: &mut ServerState,
    uri: &Url,
    new_entries: Option<&[crate::language::SymbolEntry]>,
) {
    state.symbol_table.retain(|e| e.uri != *uri);
    if let Some(entries) = new_entries {
        state.symbol_table.extend(entries.iter().cloned());
    }
}

/// Removes all symbol table entries for `uri`.
pub(crate) fn remove_symbol_table_entries_for_uri(state: &mut ServerState, uri: &Url) {
    state.symbol_table.retain(|e| e.uri != *uri);
}

/// Updates the semantic graph for a URI: removes existing nodes, then merges new graph from parsed doc.
pub(crate) fn update_semantic_graph_for_uri(
    state: &mut ServerState,
    uri: &Url,
    doc: Option<&RootNamespace>,
) {
    state.semantic_graph.remove_nodes_for_uri(uri);
    if let Some(d) = doc {
        let new_graph = semantic_model::build_graph_from_doc(d, uri);
        state.semantic_graph.merge(new_graph);
        semantic_model::add_cross_document_edges_for_uri(&mut state.semantic_graph, uri);
    }
}
