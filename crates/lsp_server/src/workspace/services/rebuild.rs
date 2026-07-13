use super::*;

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
    evaluate: bool,
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
    engine.load_parsed_from(base_graph.unwrap_or_default(), entries, evaluate);
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

