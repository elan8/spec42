use super::*;

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

/// Whether `content` (freshly read from disk for `uri`) already matches what the server has
/// tracked in memory for that URI — i.e. this watched-file event is just an echo of an edit
/// the server already knows about via `textDocument/didChange`, not a genuinely new change.
/// Pure, `Client`-free predicate so it can be unit tested directly without spinning up a real
/// LSP client/subprocess (see the test module below for why that matters here).
pub(crate) fn watched_file_content_already_current(handle: &WorkspaceHandle, uri: &Url, content: &str) -> bool {
    handle
        .snapshot()
        .index
        .get(uri)
        .map(|entry| entry.content == content)
        .unwrap_or(false)
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
                        // The editor already sent `textDocument/didChange` for its own edits
                        // (handled cheaply/incrementally); saving that same content to disk
                        // then fires this notification too, with disk content that's already
                        // byte-identical to what the server has tracked. Doing the full,
                        // synchronous `refresh_document` (whole-graph relink + eager evaluate)
                        // again in that case is pure waste — skip it. A genuinely external
                        // edit (another editor, git checkout, a formatter) still has different
                        // content and gets the full treatment below, unchanged.
                        if watched_file_content_already_current(handle, &uri_norm, &content) {
                            continue;
                        }

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

