use super::*;

/// Admit off-actor prepared syntax for library search without admitting the full tree.
pub(crate) fn index_library_search_entries(
    state: &mut impl DocumentStore,
    parsed_entries: Vec<ParsedScanEntry>,
) -> usize {
    let mut indexed = 0usize;
    for entry in parsed_entries {
        let uri_norm = crate::common::util::normalize_file_uri(&entry.uri);
        if state.is_editor_open(&uri_norm) {
            continue;
        }
        let admitted = state
            .index()
            .get(&uri_norm)
            .is_some_and(|entry| entry.admitted_to_publication);
        let workspace_source = state
            .index()
            .get(&uri_norm)
            .is_some_and(|entry| entry.workspace_source);
        let symbols =
            library_search::search_symbols_from_recovered_short_names(&entry.parsed, &uri_norm)
                .into_iter()
                .map(library_search::RecoverySearchSymbol::into_search_only_symbol)
                .collect::<Vec<_>>();
        state.index_mut().insert(
            uri_norm.clone(),
            IndexEntry {
                document: entry.document.clone(),
                parsed: Some(entry.parsed),
                admitted_to_publication: admitted,
                workspace_source,
                editor_revision: None,
            },
        );
        state
            .recovery_search_symbols_mut()
            .retain(|symbol| symbol.uri != uri_norm);
        if !admitted {
            state.recovery_search_symbols_mut().extend(symbols);
        }
        indexed += 1;
    }
    indexed
}

pub(crate) fn clear_documents_under_roots(
    state: &mut impl DocumentStore,
    roots: &[Url],
) -> Vec<Url> {
    let (library, standard) = state.library_roots();
    let library = library.to_vec();
    let standard = standard.to_vec();
    // Open library buffers remain authoring inputs until didClose. Preserve their provenance
    // even when the removed root originally supplied workspace-classified scan entries.
    for (uri, entry) in state.index_mut().iter_mut() {
        if entry.workspace_source
            && util::uri_under_any_library(uri, roots)
            && !util::uri_under_any_library(uri, &library)
            && !util::uri_under_any_library(uri, &standard)
        {
            entry.document = entry.document.with_kind(SourceKind::Workspace);
        }
        if util::uri_under_any_library(uri, roots)
            && entry.is_editor_open()
            && !entry.workspace_source
            && !entry.document.kind().is_library()
        {
            entry.document = entry.document.with_kind(SourceKind::Library);
        }
    }
    let uris_to_remove: Vec<Url> = state
        .index()
        .keys()
        .filter(|uri| {
            util::uri_under_any_library(uri, roots)
                && !state.is_editor_open(uri)
                && !state.index()[*uri].workspace_source
        })
        .cloned()
        .collect();
    for uri in &uris_to_remove {
        remove_document(state, uri);
    }
    uris_to_remove
}
