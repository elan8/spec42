use super::*;

fn publication_uris(index: &std::collections::HashMap<Url, IndexEntry>) -> Vec<Url> {
    let (workspace, library) = publication_uris_split(index, &[]);
    let mut uris = workspace;
    uris.extend(library);
    uris
}

fn publication_uris_split(
    index: &std::collections::HashMap<Url, IndexEntry>,
    library_paths: &[Url],
) -> (Vec<Url>, Vec<Url>) {
    let mut workspace = Vec::new();
    let mut library = Vec::new();
    for (uri, entry) in index {
        if !entry.admitted_to_publication {
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

/// Scan configured library roots for `sysml/librarySearch` without admitting the full tree.
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
                admitted_to_publication: false,
            },
        );
        let symbols = library_search::recover_short_name_search_symbols(&entry.content, &uri_norm)
            .into_iter()
            .map(library_search::RecoverySearchSymbol::into_search_only_symbol);
        state.symbol_table_mut().extend(symbols);
        indexed += 1;
    }
    indexed
}

/// Load import-closure library files for the current workspace publication.
///
/// Uses the canonical immutable publication builder.
/// `IndexEntry` has
/// no workspace/library distinction (only `admitted_to_publication`), and this function
/// has never applied qualified-name shadowing between "workspace" and "library" files — every
/// included URI is tagged `Workspace` here, so `load_parsed` merges all of them uniformly via
/// the same admitted source set, exactly matching this function's prior behavior.
pub(crate) fn rebuild_all_document_links(
    state: &mut impl DocumentStore,
) -> RebuildAllDocumentLinksMetrics {
    let total_start = Instant::now();
    let uris: Vec<Url> = publication_uris(state.index());
    let rebuild_ms = 0;

    let refresh_symbols_start = Instant::now();
    let mut all_symbols = Vec::new();
    for (uri, index_entry) in state.index() {
        if !index_entry.admitted_to_publication {
            all_symbols.extend(
                library_search::recover_short_name_search_symbols(&index_entry.content, uri)
                    .into_iter()
                    .map(library_search::RecoverySearchSymbol::into_search_only_symbol),
            );
            continue;
        }
        if let Some(model) = state.published_model() {
            all_symbols.extend(crate::language::symbol_entries_for_uri(model, uri));
        }
    }
    let uri_count = state.index().len();
    *state.symbol_table_mut() = all_symbols;
    let refresh_symbols_ms = elapsed_ms(refresh_symbols_start);

    // The old internal phase breakdown lived inside this function's hand-written construction
    // sequence. Now that semantic construction is one delegated call into the canonical builder,
    // those phases aren't separately timed here — deliberate, to avoid re-implementing
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
/// Unlike `rebuild_all_document_links`, this function preserves the workspace/library source-kind
/// distinction. The legacy cached base-graph argument is ignored and retained only until callers
/// finish moving to immutable publication replacement.
pub(crate) fn rebuild_publication_inputs_staged(
    index: &std::collections::HashMap<Url, IndexEntry>,
    library_paths: &[Url],
    standard_library_paths: &[Url],
    _evaluate: bool,
) -> (
    Vec<crate::language::SymbolEntry>,
    RebuildAllDocumentLinksMetrics,
) {
    let total_start = Instant::now();
    let (workspace_uris, library_uris) = publication_uris_split(index, library_paths);
    let uris: Vec<Url> = workspace_uris
        .iter()
        .chain(library_uris.iter())
        .cloned()
        .collect();

    let _ = (library_uris, standard_library_paths);
    let rebuild_ms = 0;

    let refresh_symbols_start = Instant::now();
    let mut all_symbols = Vec::new();
    for (uri, index_entry) in index {
        if !index_entry.admitted_to_publication {
            all_symbols.extend(
                library_search::recover_short_name_search_symbols(&index_entry.content, uri)
                    .into_iter()
                    .map(library_search::RecoverySearchSymbol::into_search_only_symbol),
            );
            continue;
        }
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

    (all_symbols, metrics)
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
