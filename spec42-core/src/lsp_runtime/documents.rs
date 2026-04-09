use std::sync::Arc;
use std::time::Instant;

use tokio::sync::RwLock;
use tower_lsp::lsp_types::*;
use tower_lsp::Client;
use tracing::{debug, info, warn};

use crate::common::util;
use crate::host::config::Spec42Config;
use crate::workspace::{
    clear_documents_under_roots, ingest_parsed_scan_entries, parse_scanned_entries,
    rebuild_all_document_links, refresh_document, remove_document, scan_sysml_files,
    store_document_text, ServerState,
};

use super::capabilities::server_capabilities;
use super::diagnostics::{publish_document_diagnostics, publish_workspace_diagnostics};
use super::lifecycle::{scan_roots, workspace_roots_from_initialize};

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

pub(crate) async fn initialize(
    state: &Arc<RwLock<ServerState>>,
    config: &Arc<Spec42Config>,
    server_name: &str,
    params: InitializeParams,
) -> tower_lsp::jsonrpc::Result<InitializeResult> {
    let initialize_start = Instant::now();
    let roots = workspace_roots_from_initialize(&params);
    let library_paths =
        util::parse_library_paths_from_value(params.initialization_options.as_ref());
    let startup_trace_id =
        util::parse_startup_trace_id_from_value(params.initialization_options.as_ref());
    let code_lens_enabled =
        util::parse_code_lens_enabled_from_value(params.initialization_options.as_ref(), true);
    let perf_logging_enabled =
        util::parse_perf_logging_enabled_from_value(params.initialization_options.as_ref(), false);
    if perf_logging_enabled {
        info!("startup:initialize:start");
        info!(
            trace_id = %startup_trace_id.as_deref().unwrap_or("-"),
            elapsed_ms = initialize_start.elapsed().as_millis() as u64,
            workspace_roots = roots.len(),
            library_paths = library_paths.len(),
            paths = ?library_paths.iter().map(|uri| uri.as_str()).collect::<Vec<_>>(),
            "startup:initialize:end"
        );
    }
    {
        let mut state = state.write().await;
        state.workspace_roots = roots;
        state.library_paths = library_paths;
        state.startup_trace_id = startup_trace_id;
        state.code_lens_enabled = code_lens_enabled;
        state.perf_logging_enabled = perf_logging_enabled;
    }
    Ok(InitializeResult {
        server_info: Some(ServerInfo {
            name: server_name.to_string(),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        }),
        capabilities: server_capabilities(config, code_lens_enabled),
    })
}

pub(crate) async fn initialized(
    client: &Client,
    state: &Arc<RwLock<ServerState>>,
    config: &Arc<Spec42Config>,
    server_name: &str,
) {
    client
        .log_message(MessageType::INFO, format!("{} initialized", server_name))
        .await;
    let (workspace_roots, library_paths, startup_trace_id, perf_logging_enabled) = {
        let st = state.read().await;
        (
            st.workspace_roots.clone(),
            st.library_paths.clone(),
            st.startup_trace_id.clone(),
            st.perf_logging_enabled,
        )
    };
    let scan_roots = scan_roots(&workspace_roots, &library_paths);
    if scan_roots.is_empty() {
        return;
    }
    if perf_logging_enabled {
        info!(
            trace_id = %startup_trace_id.as_deref().unwrap_or("-"),
            scan_roots = scan_roots.len(),
            "startup:initialized:scan:start"
        );
        client
            .log_message(
                MessageType::INFO,
                format!(
                    "[startup][backend] trace_id={} phase=initialized:scan:start roots={}",
                    startup_trace_id.as_deref().unwrap_or("-"),
                    scan_roots.len()
                ),
            )
            .await;
    }

    let state = Arc::clone(state);
    let config = Arc::clone(config);
    let client = client.clone();
    tokio::spawn(async move {
        let scan_total_start = Instant::now();
        let discover_read_start = Instant::now();
        let (entries, summary) = tokio::task::spawn_blocking(move || scan_sysml_files(scan_roots))
            .await
            .unwrap_or_default();
        let discover_read_ms = discover_read_start.elapsed().as_millis() as u64;
        let parallel_parse_enabled = util::env_flag_enabled("SPEC42_PARALLEL_STARTUP_PARSE", true);
        let parallel_parse_min_files =
            util::env_usize("SPEC42_PARALLEL_STARTUP_PARSE_MIN_FILES", 10);
        let should_parallel_parse =
            parallel_parse_enabled && entries.len() >= parallel_parse_min_files;
        let parse_worker_start = Instant::now();
        let parsed_entries = tokio::task::spawn_blocking(move || {
            parse_scanned_entries(entries, should_parallel_parse)
        })
        .await
        .unwrap_or_default();
        let parse_worker_ms = parse_worker_start.elapsed().as_millis() as u64;
        let merge_index_start = Instant::now();
        let mut st = state.write().await;
        let mut uris_loaded = Vec::new();
        let mut low_coverage_library_files = Vec::new();
        for parsed_entry in &parsed_entries {
            let uri_norm = util::normalize_file_uri(&parsed_entry.uri);
            if parsed_entry.parsed.is_none() {
                warn!(
                    uri = %uri_norm,
                    diagnostics = parsed_entry.parse_errors.len(),
                    errors = ?parsed_entry.parse_errors,
                    "workspace scan parse failed"
                );
            }
        }
        for (uri_norm, warning) in ingest_parsed_scan_entries(&mut st, parsed_entries) {
            if let Some(message) = warning {
                warn!("workspace scan ingest warning: {}", message);
            }
            uris_loaded.push(uri_norm.clone());
            if util::uri_under_any_library(&uri_norm, &st.library_paths) {
                let graph_nodes_for_uri = st.semantic_graph.nodes_for_uri(&uri_norm).len();
                let symbol_entries =
                    crate::semantic_model::symbol_entries_for_uri(&st.semantic_graph, &uri_norm);
                debug!(
                    uri = %uri_norm,
                    graph_nodes = graph_nodes_for_uri,
                    symbol_entries = symbol_entries.len(),
                    "library file indexed"
                );
                if st
                    .index
                    .get(&uri_norm)
                    .and_then(|entry| entry.parsed.as_ref())
                    .is_some()
                    && symbol_entries.len() <= 2
                {
                    low_coverage_library_files.push((
                        uri_norm.to_string(),
                        graph_nodes_for_uri,
                        symbol_entries.len(),
                    ));
                }
            }
        }
        let ingest_ms = merge_index_start.elapsed().as_millis() as u64;
        let relink_metrics = rebuild_all_document_links(&mut st);
        let merge_index_ms = merge_index_start.elapsed().as_millis() as u64;
        if perf_logging_enabled {
            info!(
                trace_id = %startup_trace_id.as_deref().unwrap_or("-"),
                phase_discover_read_ms = discover_read_ms,
                phase_parse_workers_ms = parse_worker_ms,
                phase_merge_index_ms = merge_index_ms,
                phase_index_parse_ms = parse_worker_ms + merge_index_ms,
                parallel_parse_enabled = parallel_parse_enabled,
                parallel_parse_min_files = parallel_parse_min_files,
                parallel_parse_active = should_parallel_parse,
                elapsed_ms = scan_total_start.elapsed().as_millis() as u64,
                loaded = uris_loaded.len(),
                candidate_files = summary.candidate_files,
                roots = summary.roots_scanned,
                skipped_non_file_roots = summary.roots_skipped_non_file,
                read_failures = summary.read_failures,
                uri_failures = summary.uri_failures,
                sample = ?uris_loaded.iter().take(5).map(|uri| uri.as_str()).collect::<Vec<_>>(),
                "startup:initialized:scan:end"
            );
        }
        if !low_coverage_library_files.is_empty() {
            warn!(
                files = low_coverage_library_files.len(),
                "workspace scan low-coverage library files (showing up to 10)"
            );
        }
        drop(st);
        let diagnostics_start = Instant::now();
        publish_workspace_diagnostics(&client, &state, &config, None).await;
        let diagnostics_ms = diagnostics_start.elapsed().as_millis() as u64;
        log_perf(
            &client,
            perf_logging_enabled,
            "backend:startupScanPhases",
            vec![
                (
                    "traceId",
                    format!("\"{}\"", startup_trace_id.as_deref().unwrap_or("-")),
                ),
                ("discoverReadMs", discover_read_ms.to_string()),
                ("parseWorkersMs", parse_worker_ms.to_string()),
                ("ingestMs", ingest_ms.to_string()),
                ("relinkTotalMs", relink_metrics.total_ms.to_string()),
                (
                    "relinkRemoveNodesMs",
                    relink_metrics.remove_nodes_ms.to_string(),
                ),
                (
                    "relinkRebuildGraphsMs",
                    relink_metrics.rebuild_graphs_ms.to_string(),
                ),
                (
                    "relinkCrossDocumentEdgesMs",
                    relink_metrics.cross_document_edges_ms.to_string(),
                ),
                (
                    "relinkRefreshSymbolsMs",
                    relink_metrics.refresh_symbols_ms.to_string(),
                ),
                ("diagnosticsMs", diagnostics_ms.to_string()),
                ("uriCount", relink_metrics.uri_count.to_string()),
                (
                    "parsedDocCount",
                    relink_metrics.parsed_doc_count.to_string(),
                ),
                ("loaded", uris_loaded.len().to_string()),
                ("candidateFiles", summary.candidate_files.to_string()),
            ],
        )
        .await;
    });
}

pub(crate) async fn did_open(
    client: &Client,
    state: &Arc<RwLock<ServerState>>,
    config: &Arc<Spec42Config>,
    params: DidOpenTextDocumentParams,
) {
    let uri = params.text_document.uri.clone();
    let uri_norm = util::normalize_file_uri(&uri);
    let text = params.text_document.text;
    let warning = {
        let mut state = state.write().await;
        store_document_text(&mut state, &uri_norm, text.clone())
    };
    if let Some(message) = warning {
        client.log_message(MessageType::WARNING, message).await;
    }
    publish_document_diagnostics(client, state, config, uri, &text).await;
}

pub(crate) async fn did_change(
    client: &Client,
    state: &Arc<RwLock<ServerState>>,
    config: &Arc<Spec42Config>,
    params: DidChangeTextDocumentParams,
) {
    let uri = params.text_document.uri.clone();
    let uri_norm = util::normalize_file_uri(&uri);
    let version = params.text_document.version;
    let apply_start = Instant::now();
    let warnings = {
        let mut state = state.write().await;
        crate::workspace::apply_document_changes(
            &mut state,
            &uri_norm,
            version,
            params.content_changes,
        )
    };
    let apply_ms = apply_start.elapsed().as_millis() as u64;
    let text = {
        let state = state.read().await;
        crate::workspace::indexed_text_or_empty(&state, &uri_norm)
    };
    for (ty, message) in warnings {
        client.log_message(ty, message).await;
    }
    let diagnostics_start = Instant::now();
    publish_document_diagnostics(client, state, config, uri, &text).await;
    let diagnostics_ms = diagnostics_start.elapsed().as_millis() as u64;
    let perf_logging_enabled = {
        let state = state.read().await;
        state.perf_logging_enabled
    };
    log_perf(
        client,
        perf_logging_enabled,
        "backend:didChange",
        vec![
            ("uri", format!("{:?}", uri_norm.as_str())),
            ("version", version.to_string()),
            ("applyChangesMs", apply_ms.to_string()),
            ("diagnosticsMs", diagnostics_ms.to_string()),
        ],
    )
    .await;
}

pub(crate) async fn did_close(client: &Client, params: DidCloseTextDocumentParams) {
    client
        .publish_diagnostics(params.text_document.uri, vec![], None)
        .await;
}

pub(crate) async fn did_change_watched_files(
    client: &Client,
    state: &Arc<RwLock<ServerState>>,
    config: &Arc<Spec42Config>,
    params: DidChangeWatchedFilesParams,
) {
    use tower_lsp::lsp_types::FileChangeType;

    let total_start = Instant::now();
    let perf_logging_enabled = {
        let state = state.read().await;
        state.perf_logging_enabled
    };
    let mut runtime_warnings = Vec::new();
    let mut changed_or_created_uris = Vec::new();
    let mut deleted_uris = Vec::new();
    let mut refresh_document_ms = 0u64;
    for event in params.changes {
        let uri_norm = util::normalize_file_uri(&event.uri);
        if event.typ == FileChangeType::CREATED || event.typ == FileChangeType::CHANGED {
            match event.uri.to_file_path() {
                Ok(path) => match tokio::fs::read_to_string(&path).await {
                    Ok(content) => {
                        let refresh_start = Instant::now();
                        let warning = {
                            let mut state = state.write().await;
                            refresh_document(&mut state, &uri_norm, content)
                        };
                        refresh_document_ms += refresh_start.elapsed().as_millis() as u64;
                        if let Some(message) = warning {
                            runtime_warnings.push(format!("didChangeWatchedFiles: {}", message));
                        }
                        changed_or_created_uris.push(uri_norm.clone());
                    }
                    Err(error) => runtime_warnings.push(format!(
                        "didChangeWatchedFiles: failed to read changed file {}: {}",
                        uri_norm, error
                    )),
                },
                Err(_) => runtime_warnings.push(format!(
                    "didChangeWatchedFiles: ignored non-file URI {}",
                    uri_norm
                )),
            }
        } else if event.typ == FileChangeType::DELETED {
            let mut state = state.write().await;
            remove_document(&mut state, &uri_norm);
            deleted_uris.push(uri_norm);
        }
    }
    for msg in runtime_warnings {
        client.log_message(MessageType::WARNING, msg).await;
    }
    let diagnostics_start = Instant::now();
    if !changed_or_created_uris.is_empty() {
        publish_workspace_diagnostics(client, state, config, Some(&changed_or_created_uris)).await;
    }
    let diagnostics_ms = diagnostics_start.elapsed().as_millis() as u64;
    let deleted_uri_count = deleted_uris.len();
    for uri in deleted_uris {
        client.publish_diagnostics(uri, vec![], None).await;
    }
    log_perf(
        client,
        perf_logging_enabled,
        "backend:didChangeWatchedFiles",
        vec![
            (
                "changedOrCreatedUris",
                changed_or_created_uris.len().to_string(),
            ),
            ("deletedUris", deleted_uri_count.to_string()),
            ("refreshDocumentMs", refresh_document_ms.to_string()),
            ("diagnosticsMs", diagnostics_ms.to_string()),
            ("totalMs", total_start.elapsed().as_millis().to_string()),
        ],
    )
    .await;
}

pub(crate) async fn did_change_configuration(
    client: &Client,
    state: &Arc<RwLock<ServerState>>,
    params: DidChangeConfigurationParams,
) {
    let new_library_paths = params
        .settings
        .get("spec42")
        .map(|value| util::parse_library_paths_from_value(Some(value)))
        .unwrap_or_else(|| util::parse_library_paths_from_value(Some(&params.settings)));
    let changed = {
        let mut state = state.write().await;
        let old_library_paths = std::mem::take(&mut state.library_paths);
        if new_library_paths == old_library_paths {
            state.library_paths = old_library_paths;
            false
        } else {
            let _ = clear_documents_under_roots(&mut state, &old_library_paths);
            state.library_paths = new_library_paths.clone();
            true
        }
    };
    if !changed {
        return;
    }

    let state = Arc::clone(state);
    let client = client.clone();
    tokio::spawn(async move {
        let perf_logging_enabled = {
            let st = state.read().await;
            st.perf_logging_enabled
        };
        let total_start = Instant::now();
        let discover_read_start = Instant::now();
        let (entries, summary) =
            tokio::task::spawn_blocking(move || scan_sysml_files(new_library_paths))
                .await
                .unwrap_or_default();
        let discover_read_ms = discover_read_start.elapsed().as_millis() as u64;
        let parallel_parse_enabled = util::env_flag_enabled("SPEC42_PARALLEL_STARTUP_PARSE", true);
        let parallel_parse_min_files =
            util::env_usize("SPEC42_PARALLEL_STARTUP_PARSE_MIN_FILES", 10);
        let should_parallel_parse =
            parallel_parse_enabled && entries.len() >= parallel_parse_min_files;
        let parse_worker_start = Instant::now();
        let parsed_entries = tokio::task::spawn_blocking(move || {
            parse_scanned_entries(entries, should_parallel_parse)
        })
        .await
        .unwrap_or_default();
        let parse_worker_ms = parse_worker_start.elapsed().as_millis() as u64;
        let ingest_start = Instant::now();
        let mut st = state.write().await;
        let mut warnings = Vec::new();
        for (_uri_norm, warning) in ingest_parsed_scan_entries(&mut st, parsed_entries) {
            if let Some(message) = warning {
                warnings.push(format!("didChangeConfiguration: {}", message));
            }
        }
        let ingest_ms = ingest_start.elapsed().as_millis() as u64;
        let relink_metrics = rebuild_all_document_links(&mut st);
        drop(st);
        if summary.roots_skipped_non_file > 0
            || summary.read_failures > 0
            || summary.uri_failures > 0
        {
            warnings.push(format!(
                "didChangeConfiguration: library reindex completed with skips: loaded {} of {} candidate SysML/KerML files across {} root(s); skipped_non_file_roots={}, read_failures={}, uri_failures={}.",
                summary.files_loaded,
                summary.candidate_files,
                summary.roots_scanned,
                summary.roots_skipped_non_file,
                summary.read_failures,
                summary.uri_failures,
            ));
        }
        for warning in warnings {
            client.log_message(MessageType::WARNING, warning).await;
        }
        log_perf(
            &client,
            perf_logging_enabled,
            "backend:didChangeConfigurationReindex",
            vec![
                ("discoverReadMs", discover_read_ms.to_string()),
                ("parseWorkersMs", parse_worker_ms.to_string()),
                ("ingestMs", ingest_ms.to_string()),
                ("relinkTotalMs", relink_metrics.total_ms.to_string()),
                (
                    "relinkRemoveNodesMs",
                    relink_metrics.remove_nodes_ms.to_string(),
                ),
                (
                    "relinkRebuildGraphsMs",
                    relink_metrics.rebuild_graphs_ms.to_string(),
                ),
                (
                    "relinkCrossDocumentEdgesMs",
                    relink_metrics.cross_document_edges_ms.to_string(),
                ),
                (
                    "relinkRefreshSymbolsMs",
                    relink_metrics.refresh_symbols_ms.to_string(),
                ),
                ("uriCount", relink_metrics.uri_count.to_string()),
                (
                    "parsedDocCount",
                    relink_metrics.parsed_doc_count.to_string(),
                ),
                ("loadedFiles", summary.files_loaded.to_string()),
                ("candidateFiles", summary.candidate_files.to_string()),
                ("totalMs", total_start.elapsed().as_millis().to_string()),
            ],
        )
        .await;
    });
}
