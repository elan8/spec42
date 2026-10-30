use super::*;

pub(crate) async fn initialize(
    projects: &ProjectRegistry,
    config: &Arc<Spec42Config>,
    server_name: &str,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
    params: InitializeParams,
) -> tower_lsp::jsonrpc::Result<InitializeResult> {
    let initialize_start = Instant::now();
    let roots = workspace_roots_from_initialize(&params);
    let library_path_base = roots.first().and_then(|root| root.to_file_path().ok());
    let client_library_paths = util::parse_library_paths_with_base(
        params.initialization_options.as_ref(),
        library_path_base.as_deref(),
    );
    let library_paths = util::merge_host_and_client_library_paths(
        &config.default_library_paths,
        client_library_paths,
    );
    let standard_library_paths = config
        .standard_library_paths
        .iter()
        .filter_map(|path| Url::from_file_path(path).ok())
        .map(|uri| util::normalize_file_uri(&uri))
        .collect::<Vec<_>>();
    let startup_trace_id =
        util::parse_startup_trace_id_from_value(params.initialization_options.as_ref());
    let code_lens_enabled =
        util::parse_code_lens_enabled_from_value(params.initialization_options.as_ref(), true);
    let perf_logging_enabled =
        util::parse_perf_logging_enabled_from_value(params.initialization_options.as_ref(), false);
    let semantic_tokens_debug =
        util::parse_semantic_tokens_debug_from_value(params.initialization_options.as_ref(), false);
    let diagnose_library_paths = util::parse_diagnose_library_paths_from_value(
        params.initialization_options.as_ref(),
        false,
    );
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
            versioned_document_changes: params
                .capabilities
                .workspace
                .as_ref()
                .and_then(|workspace| workspace.workspace_edit.as_ref())
                .and_then(|edit| edit.document_changes)
                .unwrap_or(false),
            library_path_base,
            startup_trace_id: startup_trace_id.clone(),
            code_lens_enabled,
            perf_logging_enabled,
            semantic_tokens_debug,
            semantic_tokens_refresh_supported: params
                .capabilities
                .workspace
                .as_ref()
                .and_then(|workspace| workspace.semantic_tokens.as_ref())
                .and_then(|tokens| tokens.refresh_support)
                .unwrap_or(false),
            diagnose_library_paths,
        })
        .expect("initialize called twice");
    projects
        .configure(roots, library_paths, standard_library_paths)
        .await;
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
    projects: &ProjectRegistry,
    server_name: &str,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
) {
    for (root, error) in projects.admission_errors() {
        client
            .log_message(
                MessageType::ERROR,
                format!("SysML project {} was not admitted: {error}", root.display()),
            )
            .await;
    }
    for handle in projects.handles() {
        initialized_handle(client, &handle, server_name, runtime_config).await;
    }
}

async fn initialized_handle(
    client: &Client,
    handle: &WorkspaceHandle,
    _server_name: &str,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
) {
    ensure_analysis_notifications(client, handle, runtime_config);
    let snapshot = handle.snapshot();
    let empty = snapshot.workspace_roots.is_empty() && snapshot.library_paths.is_empty();
    let token = snapshot.diagnostic_readiness.token();
    drop(snapshot);
    if empty {
        handle.complete_startup().await.ok();
        send_semantic_ready_notification(client, handle, token).await;
        publish_workspace_diagnostics(client, handle, runtime_config, None).await;
        return;
    }
    if let Ok(Some(token)) = handle.begin_startup().await {
        super::sync::start_source_discovery(client, handle, runtime_config, token);
    }
}

/// Startup is one interval of the same bounded source queue as configuration replacement.
/// A replacement captures workspace roots again until startup sources have been admitted.
pub(super) async fn run_startup_scan(
    client: &Client,
    handle: &WorkspaceHandle,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
    request: crate::session::handle::LibraryScanRequest,
) {
    let token = request.readiness;
    let inputs = request.startup.expect("startup request");
    if !handle.library_scan_is_current(&token) {
        return;
    }
    let total = Instant::now();
    let roots = inputs.workspace_roots;
    let services = handle.snapshot().services.clone();
    let source = services.source.clone();
    let discover = Instant::now();
    let (entries, summary) =
        match tokio::task::spawn_blocking(move || scan_sysml_files(roots, &source)).await {
            Ok(Ok(result)) => result,
            result => {
                super::sync::fail_source_scan(
                    client,
                    handle,
                    token,
                    format!("Startup discovery failed: {result:?}"),
                )
                .await;
                return;
            }
        };
    let discover_ms = discover.elapsed().as_millis();
    if !handle.library_scan_is_current(&token) {
        return;
    }
    let parallel = util::env_flag_enabled("SPEC42_PARALLEL_STARTUP_PARSE", true)
        && entries.len() >= util::env_usize("SPEC42_PARALLEL_STARTUP_PARSE_MIN_FILES", 10);
    let parse = Instant::now();
    let syntax_services = services.clone();
    let workspace_entries = match tokio::task::spawn_blocking(move || {
        parse_scanned_documents(entries, parallel, &syntax_services)
    })
    .await
    {
        Ok(entries) => entries,
        Err(error) => {
            super::sync::fail_source_scan(
                client,
                handle,
                token,
                format!("Startup syntax worker failed: {error}"),
            )
            .await;
            return;
        }
    };
    let parse_ms = parse.elapsed().as_millis();
    if !handle.library_scan_is_current(&token) {
        return;
    }
    let library = Instant::now();
    let library_service = services.library.clone();
    let library_paths = request.library_paths.clone();
    let standard_paths = inputs.standard_library_paths;
    let catalog = match tokio::task::spawn_blocking(move || {
        library_service.prepare_catalog_for_roots(&library_paths, &standard_paths)
    })
    .await
    {
        Ok(Ok(catalog)) => catalog,
        result => {
            super::sync::fail_source_scan(
                client,
                handle,
                token,
                format!("Startup library inventory failed: {result:?}"),
            )
            .await;
            return;
        }
    };
    let library_ms = library.elapsed().as_millis();
    let entries = workspace_entries;
    super::sync::index_catalog_for_search(
        client,
        handle,
        &token,
        &request.disk_revision,
        catalog.clone(),
    )
    .await;
    let admission = Instant::now();
    let loaded = match handle
        .ingest_startup_scan(token.clone(), entries, catalog.clone())
        .await
    {
        Ok(crate::session::handle::LibraryReindexCompletion::Admitted(results)) => {
            let count = results.len();
            for (_, warning) in results {
                if let Some(warning) = warning {
                    client.log_message(MessageType::WARNING, warning).await;
                }
            }
            count
        }
        Ok(crate::session::handle::LibraryReindexCompletion::Superseded) => return,
        Err(error) => {
            client
                .log_message(
                    MessageType::ERROR,
                    format!("Startup source admission failed: {error:?}"),
                )
                .await;
            return;
        }
    };
    let admission_ms = admission.elapsed().as_millis();
    if !handle.source_epoch_is_current(&token) {
        return;
    }
    if summary.roots_skipped_non_file > 0 || summary.read_failures > 0 || summary.uri_failures > 0 {
        client.log_message(MessageType::WARNING, format!("Startup source discovery skipped inputs: non_file_roots={}, read_failures={}, uri_failures={}", summary.roots_skipped_non_file, summary.read_failures, summary.uri_failures)).await;
    }
    log_perf(
        client,
        runtime_config
            .get()
            .is_some_and(|config| config.perf_logging_enabled),
        "backend:startupScanPhases",
        vec![
            (
                "traceId",
                serde_json::to_string(
                    &runtime_config
                        .get()
                        .and_then(|config| config.startup_trace_id.as_deref()),
                )
                .expect("startup trace serializes"),
            ),
            ("discoverReadMs", discover_ms.to_string()),
            ("parseWorkersMs", parse_ms.to_string()),
            ("libraryInventoryMs", library_ms.to_string()),
            ("sourceAdmissionMs", admission_ms.to_string()),
            ("loadedFiles", loaded.to_string()),
            ("analysisScheduled", "true".to_owned()),
            ("totalMs", total.elapsed().as_millis().to_string()),
        ],
    )
    .await;
}

#[cfg(test)]
mod readiness_tests {
    use super::*;
    use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
    use tower_lsp::{LanguageServer, LspService, Server};

    struct EmptyScanServer {
        client: Client,
        handle: WorkspaceHandle,
        config: Arc<std::sync::OnceLock<RuntimeConfig>>,
        reindex: bool,
    }

    #[tower_lsp::async_trait]
    impl LanguageServer for EmptyScanServer {
        async fn initialize(
            &self,
            _: InitializeParams,
        ) -> tower_lsp::jsonrpc::Result<InitializeResult> {
            Ok(InitializeResult::default())
        }

        async fn initialized(&self, _: InitializedParams) {
            if self.reindex {
                ensure_analysis_notifications(&self.client, &self.handle, &self.config);
                let token = self
                    .handle
                    .begin_library_reindex_if_changed(vec![Url::parse(
                        "file:///empty-reindex/library/",
                    )
                    .unwrap()])
                    .await
                    .unwrap()
                    .unwrap();
                self.handle
                    .complete_library_reindex(
                        token,
                        self.handle
                            .snapshot()
                            .services
                            .library
                            .catalog_from_documents(Vec::new()),
                    )
                    .await
                    .unwrap();
            } else {
                initialized_handle(&self.client, &self.handle, "empty-scan-test", &self.config)
                    .await;
            }
        }

        async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn empty_scan_announces_before_republishing_a_deferred_open_document() {
        assert_readiness_before_diagnostics(false, false).await;
    }

    #[tokio::test]
    async fn reindex_publication_announces_before_diagnostics_without_awaiting_a_scan_task() {
        assert_readiness_before_diagnostics(true, false).await;
    }

    #[tokio::test]
    async fn startup_scan_publication_announces_before_diagnostics_without_a_second_edit() {
        assert_readiness_before_diagnostics(false, true).await;
    }

    async fn assert_readiness_before_diagnostics(reindex: bool, workspace_scan: bool) {
        let directory = workspace_scan.then(|| tempfile::tempdir().unwrap());
        let uri = if let Some(directory) = &directory {
            let path = directory.path().join("bad.sysml");
            std::fs::write(&path, "package Saved;").unwrap();
            sysml_query::source::path_to_file_url(&path).unwrap()
        } else {
            Url::parse("file:///empty-scan/bad.sysml").unwrap()
        };
        let mut state = ServerState::default();
        if let Some(directory) = &directory {
            state.workspace_roots = vec![Url::from_directory_path(directory.path()).unwrap()];
        }
        if reindex {
            state.session.complete_startup();
        }
        state.index.insert(
            uri.clone(),
            crate::session::state::IndexEntry::for_test(&uri, "package P { } }"),
        );
        state.index.get_mut(&uri).unwrap().open_editor(None);
        let handle = WorkspaceHandle::spawn(state);
        handle.rebuild_publication().await.unwrap();
        let config = Arc::new(std::sync::OnceLock::new());
        config.set(RuntimeConfig::default()).unwrap();
        let (service, socket) = LspService::new(|client| EmptyScanServer {
            client,
            handle,
            config,
            reindex,
        });
        let (mut client_io, server_io) = tokio::io::duplex(16384);
        let (reader, writer) = tokio::io::split(server_io);
        let task = tokio::spawn(Server::new(reader, writer, socket).serve(service));
        let messages = [
            serde_json::json!({"jsonrpc":"2.0","id":1,"method":"initialize",
                "params":{"capabilities":{}}}),
            serde_json::json!({"jsonrpc":"2.0","method":"initialized","params":{}}),
        ];
        // Send initialize first, then await its response before initialized.
        let mut input = BufReader::new(&mut client_io);
        let mut saw_ready = false;
        let result = tokio::time::timeout(Duration::from_secs(10), async {
            for message in messages {
                let body = message.to_string();
                input
                    .get_mut()
                    .write_all(format!("Content-Length: {}\r\n\r\n{}", body.len(), body).as_bytes())
                    .await
                    .unwrap();
                loop {
                    let mut length = None;
                    loop {
                        let mut header = String::new();
                        assert_ne!(input.read_line(&mut header).await.unwrap(), 0);
                        if header == "\r\n" {
                            break;
                        }
                        if let Some(value) = header.strip_prefix("Content-Length:") {
                            length = Some(value.trim().parse::<usize>().unwrap());
                        }
                    }
                    let mut body = vec![0; length.unwrap()];
                    input.read_exact(&mut body).await.unwrap();
                    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();
                    match response["method"].as_str() {
                        Some("spec42/semanticIndexReady") => saw_ready = true,
                        Some("textDocument/publishDiagnostics") => {
                            assert!(saw_ready, "diagnostics preceded readiness: {response}");
                            assert_eq!(response["params"]["uri"], uri.as_str());
                            assert!(!response["params"]["diagnostics"]
                                .as_array()
                                .unwrap()
                                .is_empty());
                            return;
                        }
                        _ => {}
                    }
                    if response["id"] == 1 {
                        break;
                    }
                }
            }
        })
        .await;
        task.abort();
        result.expect("the deferred document must receive diagnostics without another edit");
    }
}
