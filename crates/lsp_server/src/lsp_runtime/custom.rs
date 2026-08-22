use crate::views::dto;
use crate::views::dto::range_to_dto;
use crate::workspace::state::DocumentStore;
use crate::workspace::ServerState;
use std::time::Instant;
use sysml_query::resolved_slice::{TextPosition, TextRange};
use tower_lsp::jsonrpc::Result;

/// Builds the `sysml/featureInspector` answer for one position.
///
/// Every semantic field comes from `PublishedModel::element_details_at`. What stays here is
/// lexical: which token the cursor is on, whether it is a reserved keyword, and whether it is the
/// unit suffix of a value. Those are source-fidelity questions the publication does not answer and
/// deliberately does not own.
pub(crate) fn sysml_feature_inspector_result(
    state: &ServerState,
    params: serde_json::Value,
) -> Result<dto::SysmlFeatureInspectorResultDto> {
    let (uri, position) = crate::views::parse_sysml_feature_inspector_params(&params)?;
    let Some(entry) = state.index.get(&uri) else {
        return Ok(crate::views::empty_feature_inspector_response(
            &uri, position,
        ));
    };
    let model = state.published_model.model();
    let text = entry.content.clone();
    // Queried once: the response and the selection classification below are two readings of the
    // same settled answer, and asking twice would suggest they could differ.
    let at = crate::views::feature_inspector::details_at(model, &uri, position);
    let mut response = match &at {
        Some(at) => crate::views::feature_inspector::feature_inspector_response(
            &uri,
            position,
            at,
            Some(text.as_str()),
        ),
        None => crate::views::empty_feature_inspector_response(&uri, position),
    };

    if let Some((unit, range)) = language_service::unit_value_suffix_selection_at_position(
        &text,
        position.line,
        position.character,
    ) {
        response.selection = dto::SysmlFeatureInspectorSelectionDto {
            kind: "unit".to_string(),
            text: Some(unit),
            range: Some(range_to_dto(range)),
        };
        return Ok(response);
    }

    let Some((line, start, end, word)) =
        language_service::word_at_position(&text, position.line, position.character)
    else {
        return Ok(response);
    };
    let selection_range = TextRange {
        start: TextPosition {
            line,
            character: start,
        },
        end: TextPosition {
            line,
            character: end,
        },
    };
    response.selection.text = Some(word.clone());
    response.selection.range = Some(range_to_dto(selection_range));

    if language_service::is_reserved_keyword(&word) {
        response.selection.kind = "keyword".to_string();
        response.language_help = language_service::keyword_help(&word).map(|help| {
            dto::SysmlFeatureInspectorLanguageHelpDto {
                keyword: word,
                description: help.description.to_string(),
                syntax: help.syntax.map(str::to_string),
            }
        });
        return Ok(response);
    }

    // The publication decides both of these. A reference is a reference because the publication
    // placed one at this position, not because a name lookup happened to succeed, and an
    // unresolved or unsupported one is deliberately *not* a reference selection: the inspector has
    // no target to show for it.
    let on_reference = at.as_ref().is_some_and(|at| {
        matches!(
            at.referenced,
            sysml_query::resolved_slice::ReferencedDetails::Resolved(_)
                | sysml_query::resolved_slice::ReferencedDetails::Ambiguous(_)
        )
    });
    let on_own_name = at.as_ref().is_some_and(|at| {
        at.containing
            .as_ref()
            .is_some_and(|details| crate::views::feature_inspector::covers_name(details, position))
    });

    if on_reference {
        response.selection.kind = "reference".to_string();
    } else if on_own_name {
        response.selection.kind = "element".to_string();
    } else if word.parse::<f64>().is_ok() {
        response.selection.kind = "value".to_string();
    }
    Ok(response)
}

pub(crate) fn sysml_library_search_result(
    state: &ServerState,
    params: serde_json::Value,
) -> Result<dto::SysmlLibrarySearchResultDto> {
    let params: dto::SysmlLibrarySearchParamsDto = serde_json::from_value(params)
        .map_err(|error| tower_lsp::jsonrpc::Error::invalid_params(error.to_string()))?;
    let query = params.query.trim().to_lowercase();
    let limit = params.limit.unwrap_or(100).clamp(1, 500);

    // Startup can admit a library document to the publication before the search-only index pass.
    // Build any missing entries from that same publication, retaining the parser-backed search
    // projection only for documents deliberately not admitted to semantic construction.
    let mut search_symbols = state.symbol_table.clone();
    for (uri, index_entry) in &state.index {
        if !crate::common::util::uri_under_any_library(uri, &state.library_paths)
            || search_symbols.iter().any(|entry| entry.uri == *uri)
        {
            continue;
        }
        // Only non-admitted corpus documents may use syntax recovery for search. Admitted
        // documents are represented exclusively by the committed publication query.
        if !index_entry.admitted_to_publication {
            search_symbols.extend(
                crate::workspace::library_search::recover_short_name_search_symbols(
                    &index_entry.content,
                    uri,
                )
                .into_iter()
                .map(
                    crate::workspace::library_search::RecoverySearchSymbol::into_search_only_symbol,
                ),
            );
        }
    }

    let mut ranked: Vec<(i64, &crate::language::SymbolEntry)> = search_symbols
        .iter()
        .filter(|entry| {
            crate::common::util::uri_under_any_library(&entry.uri, &state.library_paths)
        })
        .filter_map(|entry| {
            let score = if query.is_empty() {
                1_000
            } else {
                crate::workspace::library_search::library_search_score(&entry.name, &query)?
            };
            Some((score, entry))
        })
        .collect();

    if query.is_empty() {
        ranked.sort_by(|(_, entry_a), (_, entry_b)| {
            entry_a
                .uri
                .path()
                .cmp(entry_b.uri.path())
                .then(entry_a.name.cmp(&entry_b.name))
        });
    } else {
        ranked.sort_by(|(score_a, entry_a), (score_b, entry_b)| {
            score_b
                .cmp(score_a)
                .then(entry_a.name.len().cmp(&entry_b.name.len()))
                .then(entry_a.name.cmp(&entry_b.name))
        });
    }

    let total = ranked.len();
    let effective_limit = if query.is_empty() { total } else { limit };
    let items: Vec<crate::workspace::library_search::LibrarySearchItem> = ranked
        .into_iter()
        .take(effective_limit)
        .map(
            |(score, entry)| crate::workspace::library_search::LibrarySearchItem {
                name: entry.name.clone(),
                kind: crate::workspace::library_search::symbol_kind_label(entry.kind).to_string(),
                container: entry.container_name.clone(),
                uri: entry.uri.to_string(),
                range: entry.range,
                score,
                source: crate::workspace::library_search::library_source_label(&entry.uri)
                    .to_string(),
                path: entry.uri.path().to_string(),
            },
        )
        .collect();

    let domain_sources = crate::workspace::library_search::build_library_tree(items);
    let sources = crate::views::library_search_adapter::to_dto_sources(domain_sources);
    let symbol_total = sources
        .iter()
        .map(|src| {
            src.packages
                .iter()
                .map(|pkg| pkg.symbols.len())
                .sum::<usize>()
        })
        .sum();
    Ok(dto::SysmlLibrarySearchResultDto {
        sources,
        symbol_total,
        total,
    })
}

pub(crate) fn sysml_server_stats_result(
    state: &ServerState,
    start_time: Instant,
) -> dto::SysmlServerStatsDto {
    dto::SysmlServerStatsDto {
        uptime: start_time.elapsed().as_secs(),
        memory: dto::SysmlServerMemoryDto { rss: 0 },
        caches: dto::SysmlServerCachesDto {
            documents: state.index.len(),
            symbol_tables: state.symbol_table.len(),
            semantic_tokens: 0,
        },
    }
}

/// Clears the document-store side of the cache (index/symbol table/publication/render cache),
/// returning the pre-clear document and symbol counts. Called via
/// `WorkspaceHandle::clear_cache_state` inside an actor `mutate` closure.
pub(crate) fn clear_document_store_state(state: &mut impl DocumentStore) -> (usize, usize) {
    let docs = state.index().len();
    let syms = state.symbol_table_mut().len();
    state.index_mut().clear();
    state.symbol_table_mut().clear();
    (docs, syms)
}

pub(crate) fn clear_document_store_state_full(state: &mut ServerState) -> (usize, usize) {
    clear_document_store_state(state)
}
