use super::*;

/// Scan configured library roots for `sysml/librarySearch` without admitting the full tree.
pub(crate) fn index_library_paths_for_search(
    state: &mut impl DocumentStore,
    library_paths: &[Url],
) -> usize {
    if library_paths.is_empty() {
        return 0;
    }
    let (entries, _) = scan_sysml_files(library_paths.to_vec(), &state.services().source);
    if entries.is_empty() {
        return 0;
    }
    let parsed_entries = parse_scanned_documents(entries, false, state.services());
    let mut indexed = 0usize;
    for entry in parsed_entries {
        let uri_norm = crate::common::util::normalize_file_uri(&entry.uri);
        if state.index().contains_key(&uri_norm) {
            continue;
        }
        let symbols =
            library_search::search_symbols_from_recovered_short_names(&entry.parsed, &uri_norm)
                .into_iter()
                .map(library_search::RecoverySearchSymbol::into_search_only_symbol)
                .collect::<Vec<_>>();
        state.index_mut().insert(
            uri_norm.clone(),
            IndexEntry {
                document: entry.document.clone(),
                parsed: entry.parsed,
                admitted_to_publication: false,
            },
        );
        state.recovery_search_symbols_mut().extend(symbols);
        indexed += 1;
    }
    indexed
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
