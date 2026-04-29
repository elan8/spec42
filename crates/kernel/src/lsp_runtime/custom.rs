use crate::host::config::Spec42Config;
use crate::views::dto;
use crate::workspace::ServerState;
use std::time::Instant;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{MessageType, Url};
use tower_lsp::Client;

async fn log_perf(client: &Client, enabled: bool, event: &str, fields: Vec<(&str, String)>) {
    if !enabled {
        return;
    }
    let details = fields
        .into_iter()
        .map(|(key, value)| format!("\"{}\":{}", key, value))
        .collect::<Vec<_>>()
        .join(",");
    client
        .log_message(
            MessageType::INFO,
            format!("[SysML][perf] {{\"event\":\"{}\",{}}}", event, details),
        )
        .await;
}

pub(crate) async fn sysml_model_result(
    client: &Client,
    state: &ServerState,
    _config: &Spec42Config,
    params: serde_json::Value,
) -> Result<(dto::SysmlModelResultDto, Option<Url>)> {
    let request_start = Instant::now();
    let params_start = Instant::now();
    let (uri, scope) = crate::views::parse_sysml_model_params(&params)?;
    let workspace_visualization_requested =
        scope.iter().any(|entry| entry == "workspaceVisualization");
    let params_ms = params_start.elapsed().as_millis().max(1);
    let build_start = Instant::now();
    let index_lookup_start = Instant::now();
    let (effective_uri, entry) = match state.index.get(&uri) {
        Some(e) => (uri.clone(), e),
        None if workspace_visualization_requested => {
            let fallback_uri = state
                .semantic_graph
                .workspace_uris_excluding_libraries(&state.library_paths)
                .into_iter()
                .find(|candidate_uri| state.index.contains_key(candidate_uri));
            match fallback_uri.and_then(|fallback_uri| {
                state
                    .index
                    .get(&fallback_uri)
                    .map(|entry| (fallback_uri, entry))
            }) {
                Some((fallback_uri, entry)) => {
                    client
                        .log_message(
                            MessageType::INFO,
                            format!(
                                "sysml/model: request_uri={} not indexed; using workspaceVisualization fallback uri={}",
                                uri.as_str(),
                                fallback_uri.as_str()
                            ),
                        )
                        .await;
                    (fallback_uri, entry)
                }
                None => {
                    let index_lookup_ms = index_lookup_start.elapsed().as_millis().max(1);
                    log_perf(
                        client,
                        state.perf_logging_enabled,
                        "backend:sysmlModelLookupMiss",
                        vec![
                            ("uri", format!("{:?}", uri.as_str())),
                            ("scope", format!("{:?}", scope)),
                            ("paramsMs", params_ms.to_string()),
                            ("indexLookupMs", index_lookup_ms.to_string()),
                            ("indexSize", state.index.len().to_string()),
                            (
                                "totalMs",
                                request_start.elapsed().as_millis().max(1).to_string(),
                            ),
                        ],
                    )
                    .await;
                    return Ok((crate::views::empty_model_response(build_start), None));
                }
            }
        }
        None => {
            let index_lookup_ms = index_lookup_start.elapsed().as_millis().max(1);
            log_perf(
                client,
                state.perf_logging_enabled,
                "backend:sysmlModelLookupMiss",
                vec![
                    ("uri", format!("{:?}", uri.as_str())),
                    ("scope", format!("{:?}", scope)),
                    ("paramsMs", params_ms.to_string()),
                    ("indexLookupMs", index_lookup_ms.to_string()),
                    ("indexSize", state.index.len().to_string()),
                    (
                        "totalMs",
                        request_start.elapsed().as_millis().max(1).to_string(),
                    ),
                ],
            )
            .await;
            let uri_display = uri.as_str();
            let index_len = state.index.len();
            let indexed_uris: Vec<String> =
                state.index.keys().map(|u| u.as_str().to_string()).collect();
            client
                .log_message(
                    MessageType::WARNING,
                    format!(
                        "sysml/model: document not in index. request_uri={} (len={}) index_size={} indexed_uris_count={}. First 5 indexed: {:?}. Check URI normalization (e.g. drive letter casing on Windows).",
                        uri_display,
                        uri_display.len(),
                        index_len,
                        indexed_uris.len(),
                        indexed_uris.iter().take(5).collect::<Vec<_>>(),
                    ),
                )
                .await;
            return Ok((crate::views::empty_model_response(build_start), None));
        }
    };
    let index_lookup_ms = index_lookup_start.elapsed().as_millis().max(1);
    let parse_metadata = entry.parse_metadata;
    let response_build_start = Instant::now();
    let response = crate::build_sysml_model_response(
        &entry.content,
        entry.parsed.as_ref(),
        parse_metadata.parse_time_ms,
        parse_metadata.parse_cached,
        &state.semantic_graph,
        &effective_uri,
        &state.library_paths,
        &scope,
        build_start,
        state.perf_logging_enabled,
        client,
    )
    .await;
    let response_build_ms = response_build_start.elapsed().as_millis().max(1);
    let cache_mark_ms = 0;
    let graph_nodes = response
        .graph
        .as_ref()
        .map(|graph| graph.nodes.len())
        .unwrap_or(0);
    let graph_edges = response
        .graph
        .as_ref()
        .map(|graph| graph.edges.len())
        .unwrap_or(0);
    let activity_diagram_count = response
        .activity_diagrams
        .as_ref()
        .map(|diagrams| diagrams.len())
        .unwrap_or(0);
    let total_ms = request_start.elapsed().as_millis().max(1);
    log_perf(
        client,
        state.perf_logging_enabled,
        "backend:sysmlModelResult",
        vec![
            ("uri", format!("{:?}", uri.as_str())),
            ("scope", format!("{:?}", scope)),
            ("paramsMs", params_ms.to_string()),
            ("indexLookupMs", index_lookup_ms.to_string()),
            ("responseBuildMs", response_build_ms.to_string()),
            ("cacheMarkMs", cache_mark_ms.to_string()),
            ("parseCached", parse_metadata.parse_cached.to_string()),
            ("graphNodes", graph_nodes.to_string()),
            ("graphEdges", graph_edges.to_string()),
            ("activityDiagrams", activity_diagram_count.to_string()),
            ("totalMs", total_ms.to_string()),
        ],
    )
    .await;
    let should_mark_parse_cached = !parse_metadata.parse_cached;
    Ok((
        response,
        if should_mark_parse_cached {
            Some(effective_uri)
        } else {
            None
        },
    ))
}

pub(crate) fn mark_sysml_model_parse_cached(state: &mut ServerState, uri: &Url) {
    if let Some(entry) = state.index.get_mut(uri) {
        entry.parse_metadata.parse_cached = true;
    }
}

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
    if entry.parsed.is_none() {
        return Ok(crate::views::empty_feature_inspector_response(
            &uri, position,
        ));
    }
    Ok(crate::views::build_sysml_feature_inspector_response(
        &state.semantic_graph,
        &uri,
        position,
    ))
}

pub(crate) fn sysml_visualization_result(
    state: &ServerState,
    params: serde_json::Value,
) -> Result<dto::SysmlVisualizationResultDto> {
    let (workspace_root_uri, view, selected_view) =
        crate::views::parse_sysml_visualization_params(&params)?;
    Ok(crate::views::build_sysml_visualization_response(
        &state.semantic_graph,
        &state.index,
        &workspace_root_uri,
        &state.library_paths,
        &view,
        selected_view.as_deref(),
        Instant::now(),
    ))
}

pub(crate) fn software_visualization_result(
    params: serde_json::Value,
) -> Result<dto::SoftwareVisualizationResultDto> {
    let (workspace_root_uri, view) = crate::views::parse_software_visualization_params(&params)?;
    Ok(crate::views::build_software_visualization_response(
        &workspace_root_uri,
        &view,
        Instant::now(),
    ))
}

pub(crate) fn software_analyze_workspace_result(
    params: serde_json::Value,
) -> Result<dto::SoftwareAnalyzeWorkspaceResultDto> {
    let workspace_root_uri = crate::views::parse_software_analyze_workspace_params(&params)?;
    let workspace_path = workspace_root_uri
        .to_file_path()
        .map_err(|_| tower_lsp::jsonrpc::Error::invalid_params("software/analyzeWorkspace: invalid workspaceRootUri"))?;
    let model = crate::software_architecture::analyze_rust_workspace(&workspace_path);
    Ok(dto::SoftwareAnalyzeWorkspaceResultDto {
        version: 0,
        workspace_model: crate::views::build_software_workspace_model_dto(&model),
    })
}

pub(crate) fn software_project_view_result(
    params: serde_json::Value,
) -> Result<dto::SoftwareVisualizationResultDto> {
    let (workspace_root_uri, view, model) = crate::views::parse_software_project_view_params(&params)?;
    Ok(crate::views::build_software_project_view_response(
        &workspace_root_uri,
        &view,
        &model,
        Instant::now(),
    ))
}

pub(crate) fn sysml_library_search_result(
    state: &ServerState,
    params: serde_json::Value,
) -> Result<dto::SysmlLibrarySearchResultDto> {
    let params: dto::SysmlLibrarySearchParamsDto = serde_json::from_value(params)
        .map_err(|error| tower_lsp::jsonrpc::Error::invalid_params(error.to_string()))?;
    let query = params.query.trim().to_lowercase();
    let limit = params.limit.unwrap_or(100).clamp(1, 500);

    let mut ranked: Vec<(i64, &crate::language::SymbolEntry)> = state
        .symbol_table
        .iter()
        .filter(|entry| crate::common::util::uri_under_any_library(&entry.uri, &state.library_paths))
        .filter_map(|entry| {
            let normalized_name = crate::workspace::library_search::normalized_library_symbol_name(
                entry,
                state.index.get(&entry.uri),
            );
            let score = if query.is_empty() {
                1_000
            } else {
                crate::workspace::library_search::library_search_score(&normalized_name, &query)?
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
        .map(|(score, entry)| crate::workspace::library_search::LibrarySearchItem {
            name: crate::workspace::library_search::normalized_library_symbol_name(
                entry,
                state.index.get(&entry.uri),
            ),
            kind: crate::workspace::library_search::symbol_kind_label(entry.kind).to_string(),
            container: entry.container_name.clone(),
            uri: entry.uri.to_string(),
            range: entry.range,
            score,
            source: crate::workspace::library_search::library_source_label(&entry.uri).to_string(),
            path: entry.uri.path().to_string(),
        })
        .collect();

    let domain_sources = crate::workspace::library_search::build_library_tree(items);
    let sources = crate::views::library_search_adapter::to_dto_sources(domain_sources);
    let symbol_total = sources
        .iter()
        .map(|src| src.packages.iter().map(|pkg| pkg.symbols.len()).sum::<usize>())
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

pub(crate) fn sysml_clear_cache_result(state: &mut ServerState) -> dto::SysmlClearCacheResultDto {
    let docs = state.index.len();
    let syms = state.symbol_table.len();
    state.index.clear();
    state.symbol_table.clear();
    state.semantic_graph = crate::semantic::SemanticGraph::default();
    dto::SysmlClearCacheResultDto {
        documents: docs,
        symbol_tables: syms,
        semantic_tokens: 0,
    }
}
