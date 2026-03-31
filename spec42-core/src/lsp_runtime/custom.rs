use crate::config::Spec42Config;
use crate::dto;
use crate::workspace::ServerState;
use std::time::Instant;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::MessageType;
use tower_lsp::Client;

pub(crate) async fn sysml_model_result(
    client: &Client,
    state: &ServerState,
    config: &Spec42Config,
    params: serde_json::Value,
) -> Result<dto::SysmlModelResultDto> {
    let (uri, scope) = crate::views::parse_sysml_model_params(&params)?;
    let build_start = Instant::now();
    let entry = match state.index.get(&uri) {
        Some(e) => e,
        None => {
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
            return Ok(crate::views::empty_model_response(build_start));
        }
    };
    Ok(crate::build_sysml_model_response(
        &entry.content,
        entry.parsed.as_ref(),
        &state.semantic_graph,
        &uri,
        &state.library_paths,
        &scope,
        build_start,
        client,
        &config.diagram_providers,
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
