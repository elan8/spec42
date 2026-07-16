use super::*;

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
    handle.set_startup_config(roots, library_paths).await.ok();
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
            let base_graph_for_rebuild =
                library_graph_cache_was_hit.then(|| handle.snapshot().semantic_graph.clone());
            let (new_graph, new_symbols, staged_relink_metrics) =
                tokio::task::spawn_blocking(move || {
                    crate::workspace::rebuild_semantic_graph_staged(
                        &index_snapshot,
                        &library_paths_snapshot,
                        base_graph_for_rebuild,
                        true, // startup: settle fully before first use, not the live-edit fast path
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
        publish_workspace_diagnostics(&client, &handle, &runtime_config, None).await;
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
