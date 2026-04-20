mod capabilities;
mod custom;
mod diagnostics;
mod documents;
mod features;
mod hierarchy;
mod lifecycle;
mod lookup_helpers;
mod navigation;
mod references_resolver;
mod symbols;

use std::sync::Arc;
use std::time::Instant;

use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::host::config::Spec42Config;
use crate::views::dto;
use crate::workspace::ServerState;
use custom::{
    mark_sysml_model_parse_cached, sysml_clear_cache_result, sysml_diagram_result,
    sysml_feature_inspector_result, sysml_model_result, sysml_server_stats_result,
    sysml_visualization_result,
};

struct Backend {
    client: Client,
    state: Arc<RwLock<ServerState>>,
    config: Arc<Spec42Config>,
    start_time: Instant,
    server_name: String,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        documents::initialize(&self.state, &self.config, &self.server_name, params).await
    }

    async fn initialized(&self, _: InitializedParams) {
        documents::initialized(&self.client, &self.state, &self.config, &self.server_name).await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        documents::did_open(&self.client, &self.state, &self.config, params).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        documents::did_change(&self.client, &self.state, &self.config, params).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        documents::did_close(&self.client, params).await;
    }

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        documents::did_change_watched_files(&self.client, &self.state, &self.config, params).await;
    }

    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        documents::did_change_configuration(&self.client, &self.state, &self.config, params).await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let state = self.state.read().await;
        features::hover(
            &state,
            params.text_document_position_params.text_document.uri,
            params.text_document_position_params.position,
        )
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let state = self.state.read().await;
        features::completion(
            &state,
            params.text_document_position.text_document.uri,
            params.text_document_position.position,
        )
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
        features::goto_definition(
            &state,
            params.text_document_position_params.text_document.uri,
            params.text_document_position_params.position,
        )
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let state = self.state.read().await;
        features::references(
            &state,
            params.text_document_position.text_document.uri,
            params.text_document_position.position,
            params.context.include_declaration,
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
        features::document_highlight(
            &state,
            params.text_document_position_params.text_document.uri,
            params.text_document_position_params.position,
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
        features::prepare_rename(&state, params.text_document.uri, params.position)
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let state = self.state.read().await;
        features::rename(
            &state,
            params.text_document_position.text_document.uri,
            params.text_document_position.position,
            params.new_name,
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
        features::workspace_symbol(&state, params.query)
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
        features::code_lens(&state, params.text_document.uri)
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
        let Some((tokens, log_lines)) =
            features::semantic_tokens_full_request(&state, params.text_document.uri)?
        else {
            return Ok(None);
        };
        drop(state);
        for line in &log_lines {
            self.client.log_message(MessageType::LOG, line).await;
        }
        Ok(Some(SemanticTokensResult::Tokens(tokens)))
    }

    async fn semantic_tokens_range(
        &self,
        params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        let state = self.state.read().await;
        let Some((tokens, log_lines)) = features::semantic_tokens_range_request(
            &state,
            params.text_document.uri,
            params.range,
        )?
        else {
            return Ok(None);
        };
        drop(state);
        for line in &log_lines {
            self.client.log_message(MessageType::LOG, line).await;
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
        let read_lock_wait_start = Instant::now();
        let state = self.state.read().await;
        let read_lock_wait_ms = read_lock_wait_start.elapsed().as_millis().max(1);
        let (response, parse_cached_uri) =
            sysml_model_result(&self.client, &state, &self.config, params).await?;
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
        let perf_logging_enabled = {
            let state = self.state.read().await;
            state.perf_logging_enabled
        };
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

    async fn sysml_diagram(&self, params: serde_json::Value) -> Result<dto::SysmlDiagramResultDto> {
        let state = self.state.read().await;
        sysml_diagram_result(&self.client, &state, &self.config, params).await
    }

    async fn sysml_visualization(
        &self,
        params: serde_json::Value,
    ) -> Result<dto::SysmlVisualizationResultDto> {
        let state = self.state.read().await;
        sysml_visualization_result(&state, params)
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
        Ok(sysml_clear_cache_result(&mut state))
    }

    async fn sysml_library_search(
        &self,
        params: serde_json::Value,
    ) -> Result<dto::SysmlLibrarySearchResultDto> {
        let params: dto::SysmlLibrarySearchParamsDto = serde_json::from_value(params)
            .map_err(|error| tower_lsp::jsonrpc::Error::invalid_params(error.to_string()))?;
        let query = params.query.trim().to_lowercase();
        let limit = params.limit.unwrap_or(100).clamp(1, 500);
        let state = self.state.read().await;

        let mut ranked: Vec<(i64, &crate::language::SymbolEntry)> = state
            .symbol_table
            .iter()
            .filter(|entry| {
                crate::common::util::uri_under_any_library(&entry.uri, &state.library_paths)
            })
            .filter_map(|entry| {
                let normalized_name =
                    crate::workspace::library_search::normalized_library_symbol_name(
                        entry,
                        state.index.get(&entry.uri),
                    );
                let score = if query.is_empty() {
                    1_000
                } else {
                    crate::workspace::library_search::library_search_score(
                        &normalized_name,
                        &query,
                    )?
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
        let items: Vec<dto::SysmlLibrarySearchItemDto> = ranked
            .into_iter()
            .take(effective_limit)
            .map(|(score, entry)| dto::SysmlLibrarySearchItemDto {
                name: crate::workspace::library_search::normalized_library_symbol_name(
                    entry,
                    state.index.get(&entry.uri),
                ),
                kind: crate::workspace::library_search::symbol_kind_label(entry.kind).to_string(),
                container: entry.container_name.clone(),
                uri: entry.uri.to_string(),
                range: dto::range_to_dto(entry.range),
                score,
                source: crate::workspace::library_search::library_source_label(&entry.uri)
                    .to_string(),
                path: entry.uri.path().to_string(),
            })
            .collect();

        let sources = crate::workspace::library_search::build_library_tree(items);
        let symbol_total = sources
            .iter()
            .map(|src| {
                src.packages
                    .iter()
                    .map(|pkg| pkg.symbols.len())
                    .sum::<usize>()
            })
            .sum();
        Ok(dto::SysmlLibrarySearchResultDto {
            sources,
            symbol_total,
            total,
        })
    }
}

pub async fn run(config: Arc<Spec42Config>, server_name: &str) {
    crate::host::logging::init_tracing();
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let state = Arc::new(RwLock::new(ServerState::default()));
    let start_time = Instant::now();
    let server_name = server_name.to_string();

    let (service, socket) = LspService::build(move |client| Backend {
        client,
        state: Arc::clone(&state),
        config: Arc::clone(&config),
        start_time,
        server_name: server_name.clone(),
    })
    .custom_method("sysml/model", Backend::sysml_model)
    .custom_method("sysml/diagram", Backend::sysml_diagram)
    .custom_method("sysml/visualization", Backend::sysml_visualization)
    .custom_method("sysml/featureInspector", Backend::sysml_feature_inspector)
    .custom_method("sysml/serverStats", Backend::sysml_server_stats)
    .custom_method("sysml/clearCache", Backend::sysml_clear_cache)
    .custom_method("sysml/librarySearch", Backend::sysml_library_search)
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
