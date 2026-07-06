use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use tower_lsp::lsp_types::{notification::Notification, *};
use tower_lsp::Client;
use tracing::{info, warn};

use crate::common::util;
use crate::host::config::Spec42Config;
use crate::views::dto::SemanticIndexReadyNotificationDto;
use crate::workspace::state::ServerState;
use crate::workspace::{parse_scanned_entries, scan_sysml_files, RuntimeConfig, WorkspaceHandle};
use workspace_session::{RelinkToken, TracksRelink};

use super::capabilities::server_capabilities;
use super::diagnostics::{publish_document_diagnostics, publish_workspace_diagnostics};
use super::lifecycle::{scan_roots, workspace_roots_from_initialize};

static WORKSPACE_DIAGNOSTICS_DEBOUNCE_GEN: AtomicU64 = AtomicU64::new(0);
const WORKSPACE_DIAGNOSTICS_DEBOUNCE_MS: u64 = 450;
const SEMANTIC_RELINK_DEBOUNCE_MS: u64 = 90;

fn schedule_workspace_diagnostics_republish(
    client: &Client,
    handle: &WorkspaceHandle,
    config: &Arc<Spec42Config>,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
) {
    let generation = WORKSPACE_DIAGNOSTICS_DEBOUNCE_GEN.fetch_add(1, Ordering::SeqCst) + 1;
    let client = client.clone();
    let handle = handle.clone();
    let config = Arc::clone(config);
    let runtime_config = Arc::clone(runtime_config);
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(WORKSPACE_DIAGNOSTICS_DEBOUNCE_MS)).await;
        if WORKSPACE_DIAGNOSTICS_DEBOUNCE_GEN.load(Ordering::SeqCst) != generation {
            return;
        }
        let lifecycle = handle.snapshot().session.lifecycle();
        if !crate::workspace::state::supports_semantic_queries(lifecycle) {
            return;
        }
        publish_workspace_diagnostics(&client, &handle, &config, &runtime_config, None).await;
    });
}

/// Schedules an async semantic relink with a 90 ms debounce.
///
/// `token` is issued by [`workspace::WorkspaceSession::schedule_relink`] and
/// encapsulates the current relink generation and snapshot version.
/// Only the relink task whose token is still current when the debounce
/// fires will run; all earlier tasks self-cancel via
/// [`workspace_session::TracksRelink::is_token_current`].
fn schedule_semantic_relink_after_change(
    client: &Client,
    handle: &WorkspaceHandle,
    config: &Arc<Spec42Config>,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
    changed_uri: Url,
    token: RelinkToken,
) {
    let client = client.clone();
    let handle = handle.clone();
    let config = Arc::clone(config);
    let runtime_config = Arc::clone(runtime_config);
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(SEMANTIC_RELINK_DEBOUNCE_MS)).await;

        let snapshot = {
            let snap = handle.snapshot();
            if !snap.is_token_current(&token) {
                return;
            }
            (
                snap.index.clone(),
                snap.library_paths.clone(),
                // Library files are not stored in the index when loaded from the
                // graph cache (cache hit path). Pass the library graph snapshot so
                // library types survive the workspace rebuild regardless of whether
                // library_paths is empty or not.
                snap.library_graph_snapshot.clone(),
            )
        };
        let (index, library_paths, base_graph) = snapshot;
        let perf_logging_enabled = runtime_config
            .get()
            .expect("initialize precedes all other LSP requests")
            .perf_logging_enabled;
        let library_snapshot_uris = base_graph.as_ref().map(|g| g.all_uris().len()).unwrap_or(0);
        let relink_start = Instant::now();
        let staged = tokio::task::spawn_blocking(move || {
            crate::workspace::rebuild_semantic_graph_staged(&index, &library_paths, base_graph)
        })
        .await;
        let Ok((new_graph, new_symbols, relink_metrics)) = staged else {
            client
                .log_message(
                    MessageType::WARNING,
                    "Async semantic relink failed before completion.",
                )
                .await;
            return;
        };

        // Compute diagnostics URIs from the locally-known (pre-commit) index/library_paths —
        // `report_relink_result` is fire-and-forget under the actor model, so there's no
        // synchronous confirmation of when (or whether) it applies. This function only ever
        // reads raw source/parsed data, never the semantic graph, so the pre-relink snapshot's
        // index is exactly as good as a post-commit read would be.
        let snap_for_diag = handle.snapshot();
        let mut diag_uris = crate::workspace::import_graph::workspace_uris_importing_declarations_from(
            &snap_for_diag.index,
            &snap_for_diag.library_paths,
            &changed_uri,
        );
        drop(snap_for_diag);
        // Always include the changed file — it was skipped during the fast
        // graph update and needs diagnostics from the fully-resolved graph.
        if !diag_uris.contains(&changed_uri) {
            diag_uris.push(changed_uri.clone());
        }

        handle.report_relink_result(token, new_graph, new_symbols);

        publish_workspace_diagnostics(&client, &handle, &config, &runtime_config, Some(&diag_uris))
            .await;

        log_perf(
            &client,
            perf_logging_enabled,
            "backend:asyncSemanticRelink",
            vec![
                ("uri", format!("{:?}", changed_uri.as_str())),
                ("generation", token.generation().to_string()),
                ("librarySnapshotUris", library_snapshot_uris.to_string()),
                ("relinkTotalMs", relink_metrics.total_ms.to_string()),
                (
                    "relinkRebuildGraphsMs",
                    relink_metrics.rebuild_graphs_ms.to_string(),
                ),
                (
                    "relinkCrossDocumentEdgesMs",
                    relink_metrics.cross_document_edges_ms.to_string(),
                ),
                (
                    "relinkCrossEdgeResolutionMs",
                    relink_metrics.cross_edge_resolution_ms.to_string(),
                ),
                (
                    "relinkWorkspaceRelationshipLinkingMs",
                    relink_metrics.workspace_relationship_linking_ms.to_string(),
                ),
                (
                    "relinkPendingRelationshipResolutionMs",
                    relink_metrics
                        .pending_relationship_resolution_ms
                        .to_string(),
                ),
                (
                    "relinkExpressionEvaluationMs",
                    relink_metrics.expression_evaluation_ms.to_string(),
                ),
                (
                    "relinkRefreshSymbolsMs",
                    relink_metrics.refresh_symbols_ms.to_string(),
                ),
                ("diagUrisCount", diag_uris.len().to_string()),
                ("elapsedMs", relink_start.elapsed().as_millis().to_string()),
            ],
        )
        .await;
    });
}

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

fn workspace_file_count(state: &ServerState) -> usize {
    state
        .semantic_graph
        .workspace_uris_excluding_libraries(&state.library_paths)
        .len()
}

pub(crate) struct SemanticIndexReady;

impl Notification for SemanticIndexReady {
    type Params = SemanticIndexReadyNotificationDto;
    const METHOD: &'static str = "spec42/semanticIndexReady";
}

pub(crate) fn semantic_index_ready_notification(
    state: &ServerState,
) -> SemanticIndexReadyNotificationDto {
    SemanticIndexReadyNotificationDto {
        lifecycle: "ready".to_string(),
        semantic_state_version: state.session.version(),
        workspace_file_count: workspace_file_count(state),
    }
}

/// Sends the `spec42/semanticIndexReady` LSP notification to the client.
/// The session must already be in `Ready` state before calling this.
async fn send_semantic_ready_notification(client: &Client, handle: &WorkspaceHandle) {
    let params = semantic_index_ready_notification(&handle.snapshot());
    client.send_notification::<SemanticIndexReady>(params).await;
}

pub(crate) async fn initialize(
    handle: &WorkspaceHandle,
    config: &Arc<Spec42Config>,
    server_name: &str,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
    params: InitializeParams,
) -> tower_lsp::jsonrpc::Result<InitializeResult> {
    let initialize_start = Instant::now();
    let roots = workspace_roots_from_initialize(&params);
    let client_library_paths =
        util::parse_library_paths_from_value(params.initialization_options.as_ref());
    let library_paths = util::merge_host_and_client_library_paths(
        &config.default_library_paths,
        client_library_paths,
    );
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
    runtime_config
        .set(RuntimeConfig {
            startup_trace_id: startup_trace_id.clone(),
            code_lens_enabled,
            perf_logging_enabled,
        })
        .expect("initialize called twice");
    handle
        .set_startup_config(roots, library_paths)
        .await
        .ok();
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
    handle: &WorkspaceHandle,
    config: &Arc<Spec42Config>,
    server_name: &str,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
) {
    let (workspace_roots, library_paths) = {
        let snap = handle.snapshot();
        (snap.workspace_roots.clone(), snap.library_paths.clone())
    };
    let cfg = runtime_config
        .get()
        .expect("initialize precedes all other LSP requests");
    let startup_trace_id = cfg.startup_trace_id.clone();
    let perf_logging_enabled = cfg.perf_logging_enabled;
    if perf_logging_enabled {
        client
            .log_message(MessageType::INFO, format!("{} initialized", server_name))
            .await;
    }
    let scan_roots = if crate::workspace::library_closure::library_full_scan_enabled() {
        scan_roots(&workspace_roots, &library_paths)
    } else {
        workspace_roots.clone()
    };
    if scan_roots.is_empty() && library_paths.is_empty() {
        handle.complete_startup().await.ok();
        send_semantic_ready_notification(client, handle).await;
        return;
    }
    handle.begin_startup().await.ok();
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

    let handle = handle.clone();
    let config = Arc::clone(config);
    let runtime_config = Arc::clone(runtime_config);
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
        let library_paths_for_closure = library_paths.clone();
        // Resolve the parse cache directory once for the whole startup scan.
        let cache_dir = crate::workspace::parse_cache::default_cache_dir();
        if let Some(dir) = &cache_dir {
            let dir = dir.clone();
            tokio::task::spawn_blocking(move || {
                crate::workspace::parse_cache::evict_stale_entries(&dir);
            })
            .await
            .ok();
        }
        tokio::task::spawn_blocking(crate::workspace::library_graph_cache::evict_stale_entries)
            .await
            .ok();
        let parse_worker_start = Instant::now();
        let parsed_entries = tokio::task::spawn_blocking(move || {
            // Workspace files are not cached — they change on every edit.
            parse_scanned_entries(entries, should_parallel_parse, None)
        })
        .await
        .unwrap_or_default();
        let parse_worker_ms = parse_worker_start.elapsed().as_millis() as u64;

        // --- Library graph cache check (Level 1 + Level 2) ---
        // If the library graph was built previously and library files haven't
        // changed (verified via file metadata fingerprint), skip all library
        // disk I/O, parsing, and graph construction.
        // Keep a clone for the post-rebuild cache store call (cache miss path).
        let library_paths_for_store = library_paths_for_closure.clone();
        let library_graph_cache_hit =
            if !crate::workspace::library_closure::library_full_scan_enabled()
                && !library_paths_for_closure.is_empty()
            {
                let lp = library_paths_for_closure.clone();
                tokio::task::spawn_blocking(move || {
                    crate::workspace::library_graph_cache::load(&lp)
                })
                .await
                .ok()
                .flatten()
            } else {
                None
            };

        let (_library_parsed_count, _library_total_count, parsed_entries) =
            if let Some(cached_graph) = library_graph_cache_hit.as_ref() {
                // Cache hit: inject the pre-built library graph into state now so the
                // relink loop can merge workspace documents on top of it.
                handle
                    .inject_cached_library_graph(cached_graph.clone())
                    .await
                    .ok();
                if perf_logging_enabled {
                    info!(
                        trace_id = %startup_trace_id.as_deref().unwrap_or("-"),
                        "startup:library-graph-cache:hit"
                    );
                }
                // Only workspace entries go through relink; library graph is pre-loaded.
                (0usize, 0usize, parsed_entries)
            } else {
                // Cache miss: load library files from disk the normal way.
                let workspace_closure_inputs: Vec<(String, String)> = parsed_entries
                    .iter()
                    .map(|entry| (entry.uri.to_string(), entry.content.clone()))
                    .collect();
                let library_entries = match tokio::task::spawn_blocking(move || {
                    let workspace_sources: Vec<sysml_model::WorkspaceSource<'_>> =
                        workspace_closure_inputs
                            .iter()
                            .map(|(path, content)| sysml_model::WorkspaceSource {
                                path: path.as_str(),
                                content: content.as_str(),
                            })
                            .collect();
                    crate::workspace::library_closure::load_library_closure_scan_entries(
                        &workspace_sources,
                        &library_paths_for_closure,
                    )
                })
                .await
                {
                    Ok(Ok(entries)) => entries,
                    Ok(Err(err)) => {
                        warn!("library import closure load failed: {err}");
                        Vec::new()
                    }
                    Err(err) => {
                        warn!("library import closure task failed: {err}");
                        Vec::new()
                    }
                };
                let library_parsed = if library_entries.is_empty() {
                    Vec::new()
                } else {
                    let parallel =
                        library_entries.len() >= parallel_parse_min_files && should_parallel_parse;
                    // Library files are stable between upgrades — use the parse cache.
                    tokio::task::spawn_blocking(move || {
                        parse_scanned_entries(library_entries, parallel, cache_dir)
                    })
                    .await
                    .unwrap_or_default()
                };
                let lpc = library_parsed
                    .iter()
                    .filter(|e| e.parse_metadata.parse_cached)
                    .count();
                let ltc = library_parsed.len();
                info!(
                    library_cache_hits = lpc,
                    library_total = ltc,
                    "startup: library parse cache stats"
                );
                let combined = parsed_entries.into_iter().chain(library_parsed).collect();
                (lpc, ltc, combined)
            };
        let library_graph_cache_was_hit = library_graph_cache_hit.is_some();
        let merge_index_start = Instant::now();
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
        let ingest_results = handle
            .ingest_startup_scan(parsed_entries)
            .await
            .unwrap_or_default();
        let ingest_ms = merge_index_start.elapsed().as_millis() as u64;

        let relink_start = Instant::now();
        let relink_metrics;
        let mut stale_retries = 0u32;
        let mut relink_used_fallback = false;
        let mut uris_loaded = Vec::new();
        let mut low_coverage_library_files = Vec::new();
        loop {
            // Snapshot index/library_paths (a plain `Arc` read, no lock) before running the
            // expensive rebuild off the actor so semantic-token and hover requests can proceed
            // concurrently instead of queueing behind anything.
            let (snapshot_version, index_snapshot, library_paths_snapshot) =
                handle.relink_snapshot();
            let base_graph_for_rebuild = library_graph_cache_was_hit
                .then(|| handle.snapshot().semantic_graph.clone());
            let (new_graph, new_symbols, staged_relink_metrics) =
                tokio::task::spawn_blocking(move || {
                    crate::workspace::rebuild_semantic_graph_staged(
                        &index_snapshot,
                        &library_paths_snapshot,
                        base_graph_for_rebuild,
                    )
                })
                .await
                .unwrap_or_else(|e| panic!("startup relink task panicked: {e:?}"));

            let outcome = handle
                .commit_startup_relink_or_stale(snapshot_version, new_graph, new_symbols)
                .await;
            match outcome {
                Ok(crate::workspace::handle::StartupRelinkOutcome::Committed) => {
                    let mut metrics = staged_relink_metrics;
                    metrics.total_ms = relink_start.elapsed().as_millis() as u32;
                    relink_metrics = metrics;
                }
                Ok(crate::workspace::handle::StartupRelinkOutcome::Stale) | Err(_) => {
                    stale_retries += 1;
                    if stale_retries < 3 {
                        continue;
                    }
                    let fallback_metrics = handle.fallback_full_rebuild().await.unwrap_or_default();
                    relink_metrics = fallback_metrics;
                    relink_used_fallback = true;
                }
            }

            // On cache miss, persist the newly-built library graph so future startups
            // can skip the ~10s disk I/O + ~2.4s graph construction.
            if !library_graph_cache_was_hit
                && !relink_used_fallback
                && !library_paths_for_store.is_empty()
                && !crate::workspace::library_closure::library_full_scan_enabled()
            {
                let snap = handle.snapshot();
                let graph_to_cache = snap
                    .semantic_graph
                    .extract_library_subgraph(&snap.library_paths);
                let lp = library_paths_for_store.clone();
                tokio::task::spawn_blocking(move || {
                    crate::workspace::library_graph_cache::store(&lp, &graph_to_cache);
                });
            }

            let snap = handle.snapshot();
            if !crate::workspace::library_closure::library_full_scan_enabled()
                && !snap.library_paths.is_empty()
            {
                let library_paths_for_search = snap.library_paths.clone();
                drop(snap);
                let search_indexed = handle
                    .index_library_paths_for_search(library_paths_for_search)
                    .await
                    .unwrap_or(0);
                if search_indexed > 0 && perf_logging_enabled {
                    info!(
                        trace_id = %startup_trace_id.as_deref().unwrap_or("-"),
                        search_indexed,
                        "startup:library-search-index:end"
                    );
                }
            }

            let snap = handle.snapshot();
            for (uri_norm, warning) in &ingest_results {
                if let Some(message) = warning {
                    warn!("workspace scan ingest warning: {}", message);
                }
                uris_loaded.push(uri_norm.clone());
                if util::uri_under_any_library(uri_norm, &snap.library_paths) {
                    let graph_nodes_for_uri = snap.semantic_graph.nodes_for_uri(uri_norm).len();
                    let symbol_entries_count = snap
                        .symbol_table
                        .iter()
                        .filter(|entry| entry.uri == *uri_norm)
                        .count();

                    if snap
                        .index
                        .get(uri_norm)
                        .and_then(|entry| entry.parsed.as_ref())
                        .is_some()
                        && symbol_entries_count <= 2
                    {
                        low_coverage_library_files.push((
                            uri_norm.to_string(),
                            graph_nodes_for_uri,
                            symbol_entries_count,
                        ));
                    }
                }
            }
            break;
        }
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
                stale_retries = stale_retries,
                relink_fallback = relink_used_fallback,
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
        let diagnostics_start = Instant::now();
        handle.complete_startup().await.ok();
        send_semantic_ready_notification(&client, &handle).await;
        publish_workspace_diagnostics(&client, &handle, &config, &runtime_config, None).await;
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
                ("relinkStaleRetries", stale_retries.to_string()),
                ("relinkUsedFallback", relink_used_fallback.to_string()),
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
                    "relinkCrossEdgeResolutionMs",
                    relink_metrics.cross_edge_resolution_ms.to_string(),
                ),
                (
                    "relinkWorkspaceRelationshipLinkingMs",
                    relink_metrics.workspace_relationship_linking_ms.to_string(),
                ),
                (
                    "relinkPendingRelationshipResolutionMs",
                    relink_metrics
                        .pending_relationship_resolution_ms
                        .to_string(),
                ),
                (
                    "relinkExpressionEvaluationMs",
                    relink_metrics.expression_evaluation_ms.to_string(),
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
                ("libraryCacheHit", library_graph_cache_was_hit.to_string()),
            ],
        )
        .await;
    });
}

pub(crate) async fn did_open(
    client: &Client,
    handle: &WorkspaceHandle,
    config: &Arc<Spec42Config>,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
    params: DidOpenTextDocumentParams,
) {
    let uri = params.text_document.uri.clone();
    let uri_norm = util::normalize_file_uri(&uri);
    let text = params.text_document.text;
    let did_open_start = Instant::now();
    let perf_logging_enabled = runtime_config
        .get()
        .expect("initialize precedes all other LSP requests")
        .perf_logging_enabled;

    // Check whether the file is already indexed with identical content before
    // mutating. If so, the startup scan already built the semantic graph for
    // this URI and no expensive re-evaluation is needed.
    let open_status = {
        let snap = handle.snapshot();
        match snap.index.get(&uri_norm) {
            None => "newFile",
            Some(entry) if entry.content != text => "contentChanged",
            _ => "alreadyIndexed",
        }
    };
    let already_indexed = open_status == "alreadyIndexed";

    let (warning, lock_wait_ms, scheduled_relink) = if already_indexed {
        // Fast path: content unchanged — skip re-parse and re-evaluation entirely.
        let lock_start = Instant::now();
        handle.bump_version().await.ok();
        let lock_wait_ms = lock_start.elapsed().as_millis();
        (None, lock_wait_ms, false)
    } else {
        // New or changed file: parse without the expensive cross-document
        // evaluation pass, then schedule an async relink so that cross-document
        // edges and expression evaluation happen outside the lock.
        let lock_start = Instant::now();
        let (warning, token) = handle
            .store_document_text_fast(uri_norm.clone(), text.clone())
            .await
            .unwrap_or_default();
        let lock_wait_ms = lock_start.elapsed().as_millis();
        let scheduled_relink = token.is_some();
        if let Some(token) = token {
            schedule_semantic_relink_after_change(
                client,
                handle,
                config,
                runtime_config,
                uri_norm.clone(),
                token,
            );
        }
        (warning, lock_wait_ms, scheduled_relink)
    };

    if perf_logging_enabled {
        client
            .log_message(
                MessageType::INFO,
                format!(
                    "[SysML][perf] {{\"event\":\"backend:didOpen\",\"uri\":{:?},\"lockWaitMs\":{},\"openStatus\":{:?},\"scheduledRelink\":{}}}",
                    uri_norm.as_str(), lock_wait_ms, open_status, scheduled_relink,
                ),
            )
            .await;
    }
    if let Some(message) = warning {
        client.log_message(MessageType::WARNING, message).await;
    }
    // Only publish diagnostics immediately when the graph is already fully
    // resolved (already_indexed path). For new/changed files the relink task
    // owns diagnostic publication after the fully-resolved graph is committed.
    if already_indexed {
        let client = client.clone();
        let handle = handle.clone();
        let config = Arc::clone(config);
        let runtime_config = Arc::clone(runtime_config);
        let uri_norm_log = uri_norm.clone();
        tokio::spawn(async move {
            let diag_start = Instant::now();
            publish_document_diagnostics(&client, &handle, &config, &runtime_config, uri, &text)
                .await;
            if perf_logging_enabled {
                client
                    .log_message(
                        MessageType::INFO,
                        format!(
                            "[SysML][perf] {{\"event\":\"backend:didOpenComplete\",\"uri\":{:?},\"diagnosticsMs\":{},\"totalMs\":{}}}",
                            uri_norm_log.as_str(),
                            diag_start.elapsed().as_millis(),
                            did_open_start.elapsed().as_millis()
                        ),
                    )
                    .await;
            }
        });
    }
    // Workspace diagnostics are NOT republished here. did_open fires whenever
    // VS Code switches to a file tab, which is far too frequent. The workspace
    // diagnostics were already published at the end of the startup scan and
    // will be updated by did_change when files actually change.
}

pub(crate) async fn did_change(
    client: &Client,
    handle: &WorkspaceHandle,
    config: &Arc<Spec42Config>,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
    params: DidChangeTextDocumentParams,
) {
    let uri = params.text_document.uri.clone();
    let uri_norm = util::normalize_file_uri(&uri);
    let version = params.text_document.version;
    let apply_start = Instant::now();

    // Phase 1: apply the incoming text edit. This is cheap in-memory string
    // surgery, applied inline on the actor — never CPU-bound parsing.
    let (content_changed, mut warnings) = handle
        .apply_document_content_edit(uri_norm.clone(), version, params.content_changes)
        .await
        .unwrap_or_default();

    // Phase 2: parse the new content WITHOUT holding up the actor. Malformed or
    // incomplete syntax (e.g. an unterminated view/viewpoint) can make error
    // recovery in the parser noticeably slower than the happy path; running
    // it as a `mutate` closure would stall every other in-flight request
    // (hover, completion, further edits) behind it. `spawn_blocking` also
    // keeps this off the async executor thread.
    if content_changed {
        let content = handle
            .snapshot()
            .index
            .get(&uri_norm)
            .map(|entry| entry.content.clone());
        if let Some(content) = content {
            let parse_start = Instant::now();
            let parse_outcome =
                tokio::task::spawn_blocking(move || util::parse_for_editor(&content)).await;
            let parse_time_ms = (parse_start.elapsed().as_millis().max(1)) as u32;
            match parse_outcome {
                Ok(parsed_result) => {
                    warnings.extend(
                        handle
                            .apply_parsed_document_update(
                                uri_norm.clone(),
                                version,
                                parsed_result,
                                parse_time_ms,
                            )
                            .await
                            .unwrap_or_default(),
                    );
                }
                Err(_) => {
                    warnings.push((
                        MessageType::WARNING,
                        format!(
                            "didChange: parse task for {} (version {}) failed unexpectedly; document kept at its previous parsed state.",
                            uri_norm, version
                        ),
                    ));
                }
            }
        }
    }

    // Phase 3: decide whether to schedule a relink now that the document's
    // own parse/graph patch is committed.
    let perf_logging_enabled = runtime_config
        .get()
        .expect("initialize precedes all other LSP requests")
        .perf_logging_enabled;
    let token = handle.schedule_relink_if_ready().await.unwrap_or_default();
    let apply_ms = apply_start.elapsed().as_millis() as u64;
    for (ty, message) in warnings {
        if ty == MessageType::LOG && !perf_logging_enabled {
            continue;
        }
        client.log_message(ty, message).await;
    }
    // Diagnostics are NOT published here. Cross-document edges and expression
    // evaluation haven't run yet; the relink task publishes diagnostics after
    // committing the fully-resolved graph.
    if let Some(token) = token {
        schedule_semantic_relink_after_change(
            client,
            handle,
            config,
            runtime_config,
            uri_norm.clone(),
            token,
        );
    }
    log_perf(
        client,
        perf_logging_enabled,
        "backend:didChange",
        vec![
            ("uri", format!("{:?}", uri_norm.as_str())),
            ("version", version.to_string()),
            ("applyChangesMs", apply_ms.to_string()),
        ],
    )
    .await;
    schedule_workspace_diagnostics_republish(client, handle, config, runtime_config);
}

pub(crate) async fn did_close(client: &Client, params: DidCloseTextDocumentParams) {
    client
        .publish_diagnostics(params.text_document.uri, vec![], None)
        .await;
}

pub(crate) async fn did_change_watched_files(
    client: &Client,
    handle: &WorkspaceHandle,
    config: &Arc<Spec42Config>,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
    params: DidChangeWatchedFilesParams,
) {
    use tower_lsp::lsp_types::FileChangeType;

    let total_start = Instant::now();
    let perf_logging_enabled = runtime_config
        .get()
        .expect("initialize precedes all other LSP requests")
        .perf_logging_enabled;
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
                        let warning = handle
                            .refresh_document(uri_norm.clone(), content)
                            .await
                            .unwrap_or_default();
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
            handle.remove_document(uri_norm.clone()).await.ok();
            deleted_uris.push(uri_norm);
        }
    }
    for msg in runtime_warnings {
        client.log_message(MessageType::WARNING, msg).await;
    }
    let diagnostics_start = Instant::now();
    if !changed_or_created_uris.is_empty() {
        publish_workspace_diagnostics(
            client,
            handle,
            config,
            runtime_config,
            Some(&changed_or_created_uris),
        )
        .await;
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
    handle: &WorkspaceHandle,
    config: &Arc<Spec42Config>,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
    params: DidChangeConfigurationParams,
) {
    let client_library_paths = params
        .settings
        .get("spec42")
        .map(|value| util::parse_library_paths_from_value(Some(value)))
        .unwrap_or_else(|| util::parse_library_paths_from_value(Some(&params.settings)));
    let new_library_paths = util::merge_host_and_client_library_paths(
        &config.default_library_paths,
        client_library_paths,
    );
    let changed = handle
        .begin_library_reindex_if_changed(new_library_paths.clone())
        .await
        .unwrap_or(false);
    if !changed {
        return;
    }

    let handle = handle.clone();
    let config = Arc::clone(config);
    let runtime_config = Arc::clone(runtime_config);
    let client = client.clone();
    tokio::spawn(async move {
        let perf_logging_enabled = runtime_config
            .get()
            .expect("initialize precedes all other LSP requests")
            .perf_logging_enabled;
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
            parse_scanned_entries(entries, should_parallel_parse, None)
        })
        .await
        .unwrap_or_default();
        let parse_worker_ms = parse_worker_start.elapsed().as_millis() as u64;
        let ingest_start = Instant::now();
        let (ingest_results, relink_metrics) = handle
            .complete_library_reindex(parsed_entries)
            .await
            .unwrap_or_default();
        let mut warnings = Vec::new();
        for (_uri_norm, warning) in ingest_results {
            if let Some(message) = warning {
                warnings.push(format!("didChangeConfiguration: {}", message));
            }
        }
        let ingest_ms = ingest_start.elapsed().as_millis() as u64;
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
        send_semantic_ready_notification(&client, &handle).await;
        let diagnostics_start = Instant::now();
        publish_workspace_diagnostics(&client, &handle, &config, &runtime_config, None).await;
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
                (
                    "diagnosticsMs",
                    diagnostics_start.elapsed().as_millis().to_string(),
                ),
                ("totalMs", total_start.elapsed().as_millis().to_string()),
            ],
        )
        .await;
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_index_ready_notification_includes_version_and_file_count() {
        let mut state = ServerState::default();
        state.session.begin_startup();
        // Simulate 6 bumps so the version reaching Ready is 7.
        for _ in 0..6 {
            state.session.bump_version();
        }
        state.session.complete_startup();
        let params = semantic_index_ready_notification(&state);
        assert_eq!(params.lifecycle, "ready");
        assert_eq!(params.semantic_state_version, 8); // begin(1) + 6 bumps + complete(1) = 8
        assert_eq!(params.workspace_file_count, 0);
    }
}
