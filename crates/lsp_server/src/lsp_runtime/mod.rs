mod capabilities;
pub(crate) mod custom;
mod diagnostics;
mod documents;
mod features;
mod generation;
mod hierarchy;
mod lifecycle;
mod navigation;
mod references_resolver;
mod symbols;

use std::sync::Arc;
use std::time::Instant;
use std::{future::Future, pin::Pin};

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::host::config::Spec42Config;
use crate::session::state::ServerState;
use crate::session::{RuntimeConfig, WorkspaceHandle};
use crate::views::dto;
use custom::{
    sysml_feature_inspector_result, sysml_library_search_result, sysml_server_stats_result,
};
use generation::{
    DiagramViewsParams, DiagramViewsResult, GenerateParams, GenerateResult, GeneratorService,
    StateTransitionViewsParams, StateTransitionViewsResult,
};

struct Backend {
    client: Client,
    handle: WorkspaceHandle,
    config: Arc<Spec42Config>,
    start_time: Instant,
    server_name: String,
    /// Write-once startup configuration, set during `initialize` and read
    /// everywhere else without touching the actor. LSP guarantees
    /// `initialize` precedes every other request.
    runtime_config: Arc<std::sync::OnceLock<RuntimeConfig>>,
    generator_service: Arc<std::result::Result<GeneratorService, String>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        documents::initialize(
            &self.handle,
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
            &self.handle,
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
            &self.handle,
            &self.config,
            &self.runtime_config,
            params,
        )
        .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        documents::did_change(
            &self.client,
            &self.handle,
            &self.config,
            &self.runtime_config,
            params,
        )
        .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        documents::did_close(&self.client, &self.handle, params).await;
    }

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        documents::did_change_watched_files(
            &self.client,
            &self.handle,
            &self.config,
            &self.runtime_config,
            params,
        )
        .await;
    }

    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        documents::did_change_configuration(
            &self.client,
            &self.handle,
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
        let state = self.handle.snapshot();
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
        let state = self.handle.snapshot();
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
        let state = self.handle.snapshot();
        features::completion_resolve(&state, params)
    }

    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        let state = self.handle.snapshot();
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
        let state = self.handle.snapshot();
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
        let state = self.handle.snapshot();
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
        let state = self.handle.snapshot();
        features::document_link(&state, params.text_document.uri)
    }

    async fn document_highlight(
        &self,
        params: DocumentHighlightParams,
    ) -> Result<Option<Vec<DocumentHighlight>>> {
        let state = self.handle.snapshot();
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
        let state = self.handle.snapshot();
        features::selection_range(&state, params.text_document.uri, params.positions)
    }

    async fn prepare_rename(
        &self,
        params: TextDocumentPositionParams,
    ) -> Result<Option<PrepareRenameResponse>> {
        let state = self.handle.snapshot();
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
        let state = self.handle.snapshot();
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
        let state = self.handle.snapshot();
        features::document_symbol(&state, params.text_document.uri)
    }

    async fn folding_range(&self, params: FoldingRangeParams) -> Result<Option<Vec<FoldingRange>>> {
        let state = self.handle.snapshot();
        features::folding_range(&state, params.text_document.uri)
    }

    #[allow(deprecated)]
    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> Result<Option<Vec<SymbolInformation>>> {
        let state = self.handle.snapshot();
        let perf_logging_enabled = self
            .runtime_config
            .get()
            .expect("initialize precedes all other LSP requests")
            .perf_logging_enabled;
        features::workspace_symbol(&state, params.query, perf_logging_enabled)
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let state = self.handle.snapshot();
        features::code_action(
            &state,
            params.text_document.uri,
            params.range,
            &params.context.diagnostics,
        )
    }

    async fn code_lens(&self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        let state = self.handle.snapshot();
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
        let state = self.handle.snapshot();
        features::inlay_hint(&state, params.text_document.uri, params.range)
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let state = self.handle.snapshot();
        features::formatting(&state, params.text_document.uri, params.options)
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let state = self.handle.snapshot();
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
        let state = self.handle.snapshot();
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
        let state = self.handle.snapshot();
        features::linked_editing_range(
            &state,
            params.text_document_position_params.text_document.uri,
            params.text_document_position_params.position,
        )
    }

    async fn prepare_type_hierarchy(
        &self,
        params: TypeHierarchyPrepareParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        let state = self.handle.snapshot();
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
        let state = self.handle.snapshot();
        features::supertypes(&state, params.item.uri.clone(), params.item.selection_range)
    }

    async fn subtypes(
        &self,
        params: TypeHierarchySubtypesParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        let state = self.handle.snapshot();
        features::subtypes(&state, params.item.uri.clone(), params.item.selection_range)
    }
}

impl Backend {
    async fn sysml_feature_inspector(
        &self,
        params: serde_json::Value,
    ) -> Result<dto::SysmlFeatureInspectorResultDto> {
        let state = self.handle.snapshot();
        sysml_feature_inspector_result(&state, params)
    }

    async fn sysml_server_stats(&self) -> Result<dto::SysmlServerStatsDto> {
        let state = self.handle.snapshot();
        Ok(sysml_server_stats_result(&state, self.start_time))
    }

    async fn sysml_clear_cache(&self) -> Result<dto::SysmlClearCacheResultDto> {
        let (documents, symbol_tables) = self.handle.clear_cache_state().await.unwrap_or((0, 0));
        Ok(dto::SysmlClearCacheResultDto {
            documents,
            symbol_tables,
            semantic_tokens: 0,
        })
    }

    async fn spec42_generate(&self, params: GenerateParams) -> Result<GenerateResult> {
        let model_uri = Url::parse(&params.model_uri).map_err(|error| {
            tower_lsp::jsonrpc::Error::invalid_params(format!("invalid model URI: {error}"))
        })?;
        let state = self.handle.snapshot();
        if !state
            .index
            .contains_key(&crate::common::util::normalize_file_uri(&model_uri))
        {
            return Err(tower_lsp::jsonrpc::Error::invalid_params(
                "model URI is not part of the current workspace publication",
            ));
        }
        let publication = Arc::clone(state.session.current());
        let module_bytes = params
            .module_bytes()
            .map_err(tower_lsp::jsonrpc::Error::invalid_params)?;
        let service = Arc::clone(&self.generator_service);
        tokio::task::spawn_blocking(move || {
            let service = service
                .as_ref()
                .as_ref()
                .map_err(|message| message.clone())?;
            service.generate(
                &module_bytes,
                publication,
                &params.args,
                params.expected_model_digest.as_deref(),
            )
        })
        .await
        .map_err(|error| {
            tower_lsp::jsonrpc::Error::invalid_params(format!(
                "generator worker did not complete: {error}"
            ))
        })?
        .map_err(tower_lsp::jsonrpc::Error::invalid_params)
    }

    async fn spec42_state_transition_views(
        &self,
        params: StateTransitionViewsParams,
    ) -> Result<StateTransitionViewsResult> {
        let model_uri = Url::parse(&params.model_uri).map_err(|error| {
            tower_lsp::jsonrpc::Error::invalid_params(format!("invalid model URI: {error}"))
        })?;
        let state = self.handle.snapshot();
        if !state
            .index
            .contains_key(&crate::common::util::normalize_file_uri(&model_uri))
        {
            return Err(tower_lsp::jsonrpc::Error::invalid_params(
                "model URI is not part of the current workspace publication",
            ));
        }
        let publication = Arc::clone(state.session.current());
        let service = Arc::clone(&self.generator_service);
        tokio::task::spawn_blocking(move || {
            let service = service
                .as_ref()
                .as_ref()
                .map_err(|message| message.clone())?;
            service.state_transition_views(publication)
        })
        .await
        .map_err(|error| {
            tower_lsp::jsonrpc::Error::invalid_params(format!(
                "state-transition catalog worker did not complete: {error}"
            ))
        })?
        .map_err(tower_lsp::jsonrpc::Error::invalid_params)
    }

    async fn spec42_diagram_views(&self, params: DiagramViewsParams) -> Result<DiagramViewsResult> {
        let model_uri = Url::parse(&params.model_uri).map_err(|error| {
            tower_lsp::jsonrpc::Error::invalid_params(format!("invalid model URI: {error}"))
        })?;
        let state = self.handle.snapshot();
        if !state
            .index
            .contains_key(&crate::common::util::normalize_file_uri(&model_uri))
        {
            return Err(tower_lsp::jsonrpc::Error::invalid_params(
                "model URI is not part of the current workspace publication",
            ));
        }
        let publication = Arc::clone(state.session.current());
        let service = Arc::clone(&self.generator_service);
        tokio::task::spawn_blocking(move || {
            let service = service
                .as_ref()
                .as_ref()
                .map_err(|message| message.clone())?;
            service.diagram_views(publication)
        })
        .await
        .map_err(|error| {
            tower_lsp::jsonrpc::Error::invalid_params(format!(
                "diagram catalog worker did not complete: {error}"
            ))
        })?
        .map_err(tower_lsp::jsonrpc::Error::invalid_params)
    }

    async fn sysml_library_search(
        &self,
        params: serde_json::Value,
    ) -> Result<dto::SysmlLibrarySearchResultDto> {
        let state = self.handle.snapshot();
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
    let handle = WorkspaceHandle::spawn(ServerState::new(config.services.clone()));
    let start_time = Instant::now();
    let server_name = server_name.to_string();
    let custom_rpc_methods = config.custom_rpc_method_names();
    let service_config = Arc::clone(&config);
    let runtime_config = Arc::new(std::sync::OnceLock::<RuntimeConfig>::new());
    let generator_service = Arc::new(GeneratorService::new());

    let mut builder = LspService::build(move |client| Backend {
        client,
        handle: handle.clone(),
        config: Arc::clone(&service_config),
        start_time,
        server_name: server_name.clone(),
        runtime_config: Arc::clone(&runtime_config),
        generator_service: Arc::clone(&generator_service),
    })
    // TODO(follow-up): Model projections and diagrams return as generator plugins consuming
    // typed immutable-model queries. Do not restore the legacy graph DTO custom methods.
    .custom_method("sysml/featureInspector", Backend::sysml_feature_inspector)
    .custom_method("sysml/serverStats", Backend::sysml_server_stats)
    .custom_method("sysml/clearCache", Backend::sysml_clear_cache)
    .custom_method("sysml/librarySearch", Backend::sysml_library_search)
    .custom_method("spec42/generate", Backend::spec42_generate)
    .custom_method("spec42/diagramViews", Backend::spec42_diagram_views)
    .custom_method(
        "spec42/stateTransitionViews",
        Backend::spec42_state_transition_views,
    );

    for method in custom_rpc_methods {
        let method_name: &'static str = Box::leak(method.into_boxed_str());
        builder = builder.custom_method(method_name, make_custom_rpc_handler(method_name));
    }

    let (service, socket) = builder.finish();

    // Serialize top-level LSP message handling (one message's handler body runs to
    // completion before the next starts). This restores the ordering guarantee the
    // integration test suite's `lsp_barrier` helper relies on (a request right after a
    // notification observes that notification's effects) — the old `RwLock`-based code
    // provided this "by accident" since its mutations never yielded across a task
    // boundary, so tower-lsp's default 4-way concurrent dispatch never got a chance to
    // interleave them. The actor-based `WorkspaceHandle` legitimately yields at its
    // mailbox round trip, exposing that tower-lsp does not otherwise guarantee ordering
    // between concurrently-dispatched messages (`buffer_unordered`, see
    // `tower_lsp::Server::concurrency_level`'s default of 4). This does NOT reintroduce
    // the original blocking-on-slow-work bug: the actual heavy lifting (async relink,
    // render rebuild) already runs in a detached `tokio::spawn` task outside of any
    // handler's own body, so it is not counted against this concurrency limit — only the
    // fast, actor-mediated handler bodies themselves are serialized.
    Server::new(stdin, stdout, socket)
        .concurrency_level(1)
        .serve(service)
        .await;
}
