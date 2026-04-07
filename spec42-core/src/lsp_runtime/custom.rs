use crate::host::config::Spec42Config;
use crate::views::dto;
use crate::workspace::ServerState;
use std::time::Instant;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{MessageType, Url};
use tower_lsp::Client;

async fn log_perf(client: &Client, event: &str, fields: Vec<(&str, String)>) {
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
    let params_ms = params_start.elapsed().as_millis().max(1);
    let build_start = Instant::now();
    let index_lookup_start = Instant::now();
    let entry = match state.index.get(&uri) {
        Some(e) => e,
        None => {
            let index_lookup_ms = index_lookup_start.elapsed().as_millis().max(1);
            log_perf(
                client,
                "backend:sysmlModelLookupMiss",
                vec![
                    ("uri", format!("{:?}", uri.as_str())),
                    ("scope", format!("{:?}", scope)),
                    ("paramsMs", params_ms.to_string()),
                    ("indexLookupMs", index_lookup_ms.to_string()),
                    ("indexSize", state.index.len().to_string()),
                    ("totalMs", request_start.elapsed().as_millis().max(1).to_string()),
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
        &uri,
        &state.library_paths,
        &scope,
        build_start,
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
            Some(uri)
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

pub(crate) async fn sysml_diagram_result(
    client: &Client,
    state: &ServerState,
    _config: &Spec42Config,
    params: serde_json::Value,
) -> Result<dto::SysmlDiagramResultDto> {
    let (uri, kind, options) = crate::views::parse_sysml_diagram_params(&params)?;
    let build_start = Instant::now();
    let entry = match state.index.get(&uri) {
        Some(e) => e,
        None => {
            client
                .log_message(
                    MessageType::WARNING,
                    format!(
                        "sysml/diagram: document not in index. request_uri={}",
                        uri.as_str(),
                    ),
                )
                .await;
            return Ok(crate::views::empty_diagram_response(
                &kind,
                &uri,
                build_start,
            ));
        }
    };
    Ok(crate::build_sysml_diagram_response(
        &entry.content,
        entry.parsed.as_ref(),
        &state.semantic_graph,
        &uri,
        &state.library_paths,
        &kind,
        &options,
        build_start,
        client,
    )
    .await)
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
    state.semantic_graph = crate::semantic_model::SemanticGraph::default();
    dto::SysmlClearCacheResultDto {
        documents: docs,
        symbol_tables: syms,
        semantic_tokens: 0,
    }
}
