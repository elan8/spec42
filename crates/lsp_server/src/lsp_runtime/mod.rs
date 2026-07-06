mod capabilities;
mod custom;
mod diagnostics;
mod documents;
mod features;
mod hierarchy;
mod lifecycle;
mod navigation;
mod references_resolver;
mod symbols;

use std::sync::Arc;
use std::time::Instant;
use std::{future::Future, pin::Pin};

use tokio::sync::{Mutex, RwLock};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::host::config::Spec42Config;
use crate::views::dto;
use crate::workspace::viz_cache::WorkspaceRenderCache;
use crate::workspace::state::SemanticLifecycle;
use crate::workspace::{RuntimeConfig, ServerState};
use tokio::sync::watch;
use custom::{
    mark_sysml_model_parse_cached, sysml_clear_cache_result, sysml_feature_inspector_result,
    sysml_library_search_result, sysml_model_result, sysml_server_stats_result,
    sysml_visualization_result,
};
use sysml_model::SysmlVisualizationResultDto;

struct Backend {
    client: Client,
    state: Arc<RwLock<ServerState>>,
    render_cache: Arc<Mutex<WorkspaceRenderCache>>,
    config: Arc<Spec42Config>,
    start_time: Instant,
    server_name: String,
    /// Lifecycle watch receiver kept outside the `RwLock` so query handlers
    /// can wait for `Reindexing → Ready` without acquiring any lock.
    lifecycle_rx: watch::Receiver<SemanticLifecycle>,
    /// Write-once startup configuration, set during `initialize` and read
    /// everywhere else without acquiring the `ServerState` lock. LSP
    /// guarantees `initialize` precedes every other request.
    runtime_config: Arc<std::sync::OnceLock<RuntimeConfig>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        documents::initialize(
            &self.state,
            &self.config,
            &self.server_name,
            &self.runtime_config,
            params,
        )
        .await
    }

    async fn initialized(&self, _: InitializedParams) {
        documents::initialized(
            &self.client,
            &self.state,
            &self.config,
            &self.server_name,
            &self.runtime_config,
        )
        .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        documents::did_open(
            &self.client,
            &self.state,
            &self.config,
            &self.runtime_config,
            params,
        )
        .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        documents::did_change(
            &self.client,
            &self.state,
            &self.config,
            &self.runtime_config,
            params,
        )
        .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        documents::did_close(&self.client, params).await;
    }

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        documents::did_change_watched_files(
            &self.client,
            &self.state,
            &self.config,
            &self.runtime_config,
            params,
        )
        .await;
    }

    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        documents::did_change_configuration(
            &self.client,
            &self.state,
            &self.config,
            &self.runtime_config,
            params,
        )
        .await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let hover_uri = params
            .text_document_position_params
            .text_document
            .uri
            .clone();
        let state = self.state.read().await;
        let perf_logging_enabled = self
            .runtime_config
            .get()
            .expect("initialize precedes all other LSP requests")
            .perf_logging_enabled;
        features::hover(
            &state,
            hover_uri,
            params.text_document_position_params.position,
            perf_logging_enabled,
        )
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let state = self.state.read().await;
        let perf_logging_enabled = self
            .runtime_config
            .get()
            .expect("initialize precedes all other LSP requests")
            .perf_logging_enabled;
        features::completion(
            &state,
            params.text_document_position.text_document.uri,
            params.text_document_position.position,
            perf_logging_enabled,
        )
    }

    async fn completion_resolve(&self, params: CompletionItem) -> Result<CompletionItem> {
        let state = self.state.read().await;
        features::completion_resolve(&state, params)
    }

    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        let state = self.state.read().await;
        features::signature_help(
            &state,
            params.text_document_position_params.text_document.uri,
            params.text_document_position_params.position,
        )
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let state = self.state.read().await;
        let perf_logging_enabled = self
            .runtime_config
            .get()
            .expect("initialize precedes all other LSP requests")
            .perf_logging_enabled;
        features::goto_definition(
            &state,
            params.text_document_position_params.text_document.uri,
            params.text_document_position_params.position,
            perf_logging_enabled,
        )
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let state = self.state.read().await;
        let perf_logging_enabled = self
            .runtime_config
            .get()
            .expect("initialize precedes all other LSP requests")
            .perf_logging_enabled;
        features::references(
            &state,
            params.text_document_position.text_document.uri,
            params.text_document_position.position,
            params.context.include_declaration,
            perf_logging_enabled,
        )
    }

    async fn document_link(&self, params: DocumentLinkParams) -> Result<Option<Vec<DocumentLink>>> {
        let state = self.state.read().await;
        features::document_link(&state, params.text_document.uri)
    }

    async fn document_highlight(
        &self,
        params: DocumentHighlightParams,
    ) -> Result<Option<Vec<DocumentHighlight>>> {
        let state = self.state.read().await;
        let perf_logging_enabled = self
            .runtime_config
            .get()
            .expect("initialize precedes all other LSP requests")
            .perf_logging_enabled;
        features::document_highlight(
            &state,
            params.text_document_position_params.text_document.uri,
            params.text_document_position_params.position,
            perf_logging_enabled,
        )
    }

    async fn selection_range(
        &self,
        params: SelectionRangeParams,
    ) -> Result<Option<Vec<SelectionRange>>> {
        let state = self.state.read().await;
        features::selection_range(&state, params.text_document.uri, params.positions)
    }

    async fn prepare_rename(
        &self,
        params: TextDocumentPositionParams,
    ) -> Result<Option<PrepareRenameResponse>> {
        let state = self.state.read().await;
        let perf_logging_enabled = self
            .runtime_config
            .get()
            .expect("initialize precedes all other LSP requests")
            .perf_logging_enabled;
        features::prepare_rename(
            &state,
            params.text_document.uri,
            params.position,
            perf_logging_enabled,
        )
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let state = self.state.read().await;
        let perf_logging_enabled = self
            .runtime_config
            .get()
            .expect("initialize precedes all other LSP requests")
            .perf_logging_enabled;
        features::rename(
            &state,
            params.text_document_position.text_document.uri,
            params.text_document_position.position,
            params.new_name,
            perf_logging_enabled,
        )
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let state = self.state.read().await;
        features::document_symbol(&state, params.text_document.uri)
    }

    async fn folding_range(&self, params: FoldingRangeParams) -> Result<Option<Vec<FoldingRange>>> {
        let state = self.state.read().await;
        features::folding_range(&state, params.text_document.uri)
    }

    #[allow(deprecated)]
    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> Result<Option<Vec<SymbolInformation>>> {
        let state = self.state.read().await;
        let perf_logging_enabled = self
            .runtime_config
            .get()
            .expect("initialize precedes all other LSP requests")
            .perf_logging_enabled;
        features::workspace_symbol(&state, params.query, perf_logging_enabled)
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let state = self.state.read().await;
        features::code_action(
            &state,
            params.text_document.uri,
            &params.context.diagnostics,
        )
    }

    async fn code_lens(&self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        let state = self.state.read().await;
        let runtime_config = self
            .runtime_config
            .get()
            .expect("initialize precedes all other LSP requests");
        features::code_lens(
            &state,
            params.text_document.uri,
            runtime_config.code_lens_enabled,
            runtime_config.perf_logging_enabled,
        )
    }

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let state = self.state.read().await;
        features::inlay_hint(&state, params.text_document.uri, params.range)
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let state = self.state.read().await;
        features::formatting(&state, params.text_document.uri, params.options)
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let state = self.state.read().await;
        let perf_logging_enabled = self
            .runtime_config
            .get()
            .expect("initialize precedes all other LSP requests")
            .perf_logging_enabled;
        let Some((tokens, log_lines)) = features::semantic_tokens_full_request(
            &state,
            params.text_document.uri,
            perf_logging_enabled,
        )?
        else {
            return Ok(None);
        };
        drop(state);
        if perf_logging_enabled {
            for line in &log_lines {
                self.client.log_message(MessageType::LOG, line).await;
            }
        }
        Ok(Some(SemanticTokensResult::Tokens(tokens)))
    }

    async fn semantic_tokens_range(
        &self,
        params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        let state = self.state.read().await;
        let perf_logging_enabled = self
            .runtime_config
            .get()
            .expect("initialize precedes all other LSP requests")
            .perf_logging_enabled;
        let Some((tokens, log_lines)) = features::semantic_tokens_range_request(
            &state,
            params.text_document.uri,
            params.range,
            perf_logging_enabled,
        )?
        else {
            return Ok(None);
        };
        drop(state);
        if perf_logging_enabled {
            for line in &log_lines {
                self.client.log_message(MessageType::LOG, line).await;
            }
        }
        Ok(Some(SemanticTokensRangeResult::Tokens(tokens)))
    }

    async fn linked_editing_range(
        &self,
        params: LinkedEditingRangeParams,
    ) -> Result<Option<LinkedEditingRanges>> {
        let state = self.state.read().await;
        features::linked_editing_range(
            &state,
            params.text_document_position_params.text_document.uri,
            params.text_document_position_params.position,
        )
    }

    async fn moniker(&self, params: MonikerParams) -> Result<Option<Vec<Moniker>>> {
        let state = self.state.read().await;
        features::moniker(
            &state,
            params.text_document_position_params.text_document.uri,
            params.text_document_position_params.position,
        )
    }

    async fn prepare_type_hierarchy(
        &self,
        params: TypeHierarchyPrepareParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        let state = self.state.read().await;
        features::prepare_type_hierarchy(
            &state,
            params.text_document_position_params.text_document.uri,
            params.text_document_position_params.position,
        )
    }

    async fn supertypes(
        &self,
        params: TypeHierarchySupertypesParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        let state = self.state.read().await;
        features::supertypes(&state, params.item.uri.clone(), params.item.selection_range)
    }

    async fn subtypes(
        &self,
        params: TypeHierarchySubtypesParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        let state = self.state.read().await;
        features::subtypes(&state, params.item.uri.clone(), params.item.selection_range)
    }

    async fn prepare_call_hierarchy(
        &self,
        params: CallHierarchyPrepareParams,
    ) -> Result<Option<Vec<CallHierarchyItem>>> {
        let state = self.state.read().await;
        features::prepare_call_hierarchy(
            &state,
            params.text_document_position_params.text_document.uri,
            params.text_document_position_params.position,
        )
    }

    async fn incoming_calls(
        &self,
        params: CallHierarchyIncomingCallsParams,
    ) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
        let state = self.state.read().await;
        features::incoming_calls(&state, params.item.uri.clone(), params.item.selection_range)
    }

    async fn outgoing_calls(
        &self,
        params: CallHierarchyOutgoingCallsParams,
    ) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
        let state = self.state.read().await;
        features::outgoing_calls(&state, params.item.uri.clone(), params.item.selection_range)
    }
}

impl Backend {
    async fn sysml_model(&self, params: serde_json::Value) -> Result<dto::SysmlModelResultDto> {
        let request_start = Instant::now();
        // Log handler dispatch time BEFORE acquiring the lock so we can compare
        // against the frontend's getModelRequestStart timestamp and see how long
        // the request sat in the transport/queue before reaching this handler.
        {
            let is_perf = self
                .runtime_config
                .get()
                .map(|c| c.perf_logging_enabled)
                .unwrap_or(false);
            if is_perf {
                self.client
                    .log_message(
                        MessageType::INFO,
                        "[SysML][perf] {\"event\":\"backend:sysmlModelHandlerStart\"}",
                    )
                    .await;
            }
        }
        // Wait for any in-flight async relink to complete so the response
        // reflects a fully-resolved semantic graph (satisfy/perform/subject edges etc).
        // The watch receiver wakes instantly when commit_relink fires — no polling needed.
        let mut lifecycle_rx = self.lifecycle_rx.clone();
        let _ = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            lifecycle_rx.wait_for(|&l| l != SemanticLifecycle::Reindexing),
        )
        .await;
        let read_lock_wait_start = Instant::now();
        let state = self.state.read().await;
        let mut render_cache = self.render_cache.lock().await;
        let read_lock_wait_ms = read_lock_wait_start.elapsed().as_millis().max(1);
        let perf_logging_enabled = self
            .runtime_config
            .get()
            .expect("initialize precedes all other LSP requests")
            .perf_logging_enabled;
        let (response, parse_cached_uri) = sysml_model_result(
            &self.client,
            &state,
            &mut render_cache,
            &self.config,
            params,
            perf_logging_enabled,
        )
        .await?;
        drop(render_cache);
        drop(state);

        let cache_mark_lock_wait_start = Instant::now();
        if let Some(uri) = parse_cached_uri {
            let mut state = self.state.write().await;
            mark_sysml_model_parse_cached(&mut state, &uri);
        }
        let cache_mark_lock_wait_ms = cache_mark_lock_wait_start.elapsed().as_millis().max(1);
        let total_ms = request_start.elapsed().as_millis().max(1);
        let parse_time_ms = response
            .stats
            .as_ref()
            .map(|stats| stats.parse_time_ms)
            .unwrap_or(0);
        let model_build_time_ms = response
            .stats
            .as_ref()
            .map(|stats| stats.model_build_time_ms)
            .unwrap_or(0);
        let node_count = response
            .graph
            .as_ref()
            .map(|graph| graph.nodes.len())
            .unwrap_or(0);
        let edge_count = response
            .graph
            .as_ref()
            .map(|graph| graph.edges.len())
            .unwrap_or(0);
        let client = self.client.clone();
        tokio::spawn(async move {
            if !perf_logging_enabled {
                return;
            }
            client
                .log_message(
                    MessageType::INFO,
                    format!(
                        "[SysML][perf] {{\"event\":\"backend:sysmlModelRequest\",\"lockWaitMs\":{},\"readLockWaitMs\":{},\"cacheMarkLockWaitMs\":{},\"totalMs\":{},\"parseTimeMs\":{},\"modelBuildTimeMs\":{},\"graphNodes\":{},\"graphEdges\":{}}}",
                        read_lock_wait_ms + cache_mark_lock_wait_ms,
                        read_lock_wait_ms,
                        cache_mark_lock_wait_ms,
                        total_ms,
                        parse_time_ms,
                        model_build_time_ms,
                        node_count,
                        edge_count,
                    ),
                )
                .await;
        });
        Ok(response)
    }

    async fn sysml_visualization(
        &self,
        params: serde_json::Value,
    ) -> Result<SysmlVisualizationResultDto> {
        let request_start = Instant::now();
        let state = self.state.read().await;
        let mut render_cache = self.render_cache.lock().await;
        let perf_logging_enabled = self
            .runtime_config
            .get()
            .expect("initialize precedes all other LSP requests")
            .perf_logging_enabled;
        let (response, build_meta) = sysml_visualization_result(&state, &mut render_cache, params)?;
        drop(render_cache);
        drop(state);
        if perf_logging_enabled {
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
            let general_view_nodes = response
                .general_view_graph
                .as_ref()
                .map(|graph| graph.nodes.len())
                .unwrap_or(0);
            let general_view_edges = response
                .general_view_graph
                .as_ref()
                .map(|graph| graph.edges.len())
                .unwrap_or(0);
            let model_build_time_ms = response
                .stats
                .as_ref()
                .map(|stats| stats.model_build_time_ms)
                .unwrap_or(0);
            self.client
                .log_message(
                    MessageType::INFO,
                    format!(
                        "[SysML][perf] {{\"event\":\"backend:sysmlVisualizationRequest\",\"view\":\"{}\",\"modelReady\":{},\"totalMs\":{},\"cacheHit\":{},\"ibdMs\":{},\"viewEvalMs\":{},\"sceneMs\":{},\"modelBuildTimeMs\":{},\"graphNodes\":{},\"graphEdges\":{},\"generalViewNodes\":{},\"generalViewEdges\":{},\"viewCandidates\":{}}}",
                        response.view,
                        response.model_ready,
                        request_start.elapsed().as_millis().max(1),
                        build_meta.cache_hit,
                        build_meta.ibd_ms,
                        build_meta.view_eval_ms,
                        build_meta.scene_ms,
                        model_build_time_ms,
                        graph_nodes,
                        graph_edges,
                        general_view_nodes,
                        general_view_edges,
                        response.view_candidates.len(),
                    ),
                )
                .await;
        }
        Ok(response)
    }

    async fn sysml_feature_inspector(
        &self,
        params: serde_json::Value,
    ) -> Result<dto::SysmlFeatureInspectorResultDto> {
        let state = self.state.read().await;
        sysml_feature_inspector_result(&state, params)
    }

    async fn sysml_server_stats(&self) -> Result<dto::SysmlServerStatsDto> {
        let state = self.state.read().await;
        Ok(sysml_server_stats_result(&state, self.start_time))
    }

    async fn sysml_clear_cache(&self) -> Result<dto::SysmlClearCacheResultDto> {
        let mut state = self.state.write().await;
        let mut render_cache = self.render_cache.lock().await;
        Ok(sysml_clear_cache_result(&mut state, &mut render_cache))
    }

    async fn sysml_library_search(
        &self,
        params: serde_json::Value,
    ) -> Result<dto::SysmlLibrarySearchResultDto> {
        let state = self.state.read().await;
        sysml_library_search_result(&state, params)
    }

    async fn custom_rpc_method(
        &self,
        method: &'static str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let context = crate::CustomRpcContext {
            config: self.config.as_ref(),
            server_name: &self.server_name,
            server_start_time: self.start_time,
        };
        for provider in &self.config.custom_rpc_providers {
            if let Some(result) = provider.try_handle(method, params.clone(), context)? {
                return Ok(result);
            }
        }
        Err(tower_lsp::jsonrpc::Error::method_not_found())
    }
}

fn make_custom_rpc_handler(
    method_name: &'static str,
) -> impl for<'a> Fn(
    &'a Backend,
    serde_json::Value,
) -> Pin<Box<dyn Future<Output = Result<serde_json::Value>> + Send + 'a>>
       + Clone
       + Send
       + Sync
       + 'static {
    move |backend: &Backend, params| Box::pin(backend.custom_rpc_method(method_name, params))
}

pub async fn run(config: Arc<Spec42Config>, server_name: &str) {
    crate::host::logging::init_tracing();
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let state = Arc::new(RwLock::new(ServerState::default()));
    let render_cache = Arc::new(Mutex::new(WorkspaceRenderCache::default()));
    let start_time = Instant::now();
    let server_name = server_name.to_string();
    let custom_rpc_methods = config.custom_rpc_method_names();
    let service_config = Arc::clone(&config);
    let runtime_config = Arc::new(std::sync::OnceLock::<RuntimeConfig>::new());
    // Subscribe to lifecycle changes before handing state to LspService.
    // The receiver lives outside the RwLock so sysml/model can wait for
    // Reindexing→Ready without acquiring any lock.
    let lifecycle_rx = state.try_read().unwrap().coordinator.subscribe();

    let mut builder = LspService::build(move |client| Backend {
        client,
        state: Arc::clone(&state),
        render_cache: Arc::clone(&render_cache),
        config: Arc::clone(&service_config),
        start_time,
        server_name: server_name.clone(),
        lifecycle_rx: lifecycle_rx.clone(),
        runtime_config: Arc::clone(&runtime_config),
    })
    .custom_method("sysml/model", Backend::sysml_model)
    .custom_method("sysml/visualization", Backend::sysml_visualization)
    .custom_method("sysml/featureInspector", Backend::sysml_feature_inspector)
    .custom_method("sysml/serverStats", Backend::sysml_server_stats)
    .custom_method("sysml/clearCache", Backend::sysml_clear_cache)
    .custom_method("sysml/librarySearch", Backend::sysml_library_search);

    for method in custom_rpc_methods {
        let method_name: &'static str = Box::leak(method.into_boxed_str());
        builder = builder.custom_method(method_name, make_custom_rpc_handler(method_name));
    }

    let (service, socket) = builder.finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
