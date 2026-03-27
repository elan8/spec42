//! SysML v2 language server (LSP over stdio).
//! This module hosts the LSP server implementation and is meant to be reused by
//! multiple binaries (e.g. `spec42` and `spec42-pro`) via an injected [`Spec42Config`].

use crate::config::Spec42Config;
use crate::dto;
use crate::lsp::capabilities::server_capabilities;
use crate::lsp::custom::{
    sysml_clear_cache_result, sysml_model_result, sysml_server_stats_result,
};
use crate::lsp::editing::indexed_text_for_uri;
use crate::lsp::hierarchy::{
    call_hierarchy_item_for_node, moniker_for_node, type_hierarchy_item_for_node,
};
use crate::lsp::indexing::{
    remove_symbol_table_entries_for_uri, scan_sysml_files, update_semantic_graph_for_uri,
    update_symbol_table_for_uri,
};
use crate::lsp::library_search;
use crate::lsp::lifecycle::{scan_roots, workspace_roots_from_initialize};
use crate::lsp::navigation::{collect_document_links, selection_ranges_for_positions};
use crate::lsp::request_helpers::indexed_text;
use crate::lsp::symbols::{build_code_lens, build_inlay_hints};
use crate::lsp::types::{IndexEntry, ServerState};
use crate::semantic_model;
use crate::util;

use crate::semantic_tokens::{
    ast_semantic_ranges, semantic_tokens_full, semantic_tokens_range,
};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::language::{
    collect_definition_ranges, collect_document_symbols, collect_folding_ranges, completion_prefix,
    find_reference_ranges, format_document, is_reserved_keyword, keyword_doc,
    keyword_hover_markdown, line_prefix_at_position, suggest_wrap_in_package, sysml_keywords,
    word_at_position,
};

// -------------------------
// Custom requests (extension)
// -------------------------

#[derive(Debug)]
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
        let roots: Vec<Url> = workspace_roots_from_initialize(&params);
        let library_paths: Vec<Url> =
            util::parse_library_paths_from_value(params.initialization_options.as_ref());
        info!(
            workspace_roots = roots.len(),
            library_paths = library_paths.len(),
            paths = ?library_paths.iter().map(|u| u.as_str()).collect::<Vec<_>>(),
            "initialize"
        );
        {
            let mut state = self.state.write().await;
            state.workspace_roots = roots;
            state.library_paths = library_paths;
        }
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: self.server_name.clone(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            capabilities: server_capabilities(&self.config),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, format!("{} initialized", self.server_name))
            .await;
        let state = Arc::clone(&self.state);
        let (workspace_roots, library_paths) = {
            let st = state.read().await;
            (st.workspace_roots.clone(), st.library_paths.clone())
        };
        let scan_roots: Vec<Url> = scan_roots(&workspace_roots, &library_paths);
        if scan_roots.is_empty() {
            return;
        }
        let client = self.client.clone();
        tokio::spawn(async move {
            let (entries, summary) =
                tokio::task::spawn_blocking(move || scan_sysml_files(scan_roots))
                    .await
                    .unwrap_or_default();
            let mut st = state.write().await;
            let mut uris_loaded = Vec::new();
            let mut low_coverage_library_files: Vec<(String, usize, usize)> = Vec::new();
            for (uri, content) in entries {
                let uri_norm = util::normalize_file_uri(&uri);
                let parsed = sysml_parser::parse(&content).ok();
                if parsed.is_none() {
                    let errs = util::parse_failure_diagnostics(&content, 5);
                    warn!(
                        uri = %uri_norm,
                        diagnostics = errs.len(),
                        errors = ?errs,
                        "workspace scan parse failed"
                    );
                    if errs.is_empty() {
                        warn!("parse() returned None but parse_with_diagnostics had 0 errors");
                    }
                }
                update_semantic_graph_for_uri(&mut st, &uri_norm, parsed.as_ref());
                uris_loaded.push(uri_norm.clone());
                st.index
                    .insert(uri_norm.clone(), IndexEntry { content, parsed });
                let mut new_entries =
                    semantic_model::symbol_entries_for_uri(&st.semantic_graph, &uri_norm);
                if let Some(index_entry) = st.index.get(&uri_norm) {
                    library_search::add_short_name_symbol_entries(
                        &mut new_entries,
                        &index_entry.content,
                        &uri_norm,
                    );
                }
                update_symbol_table_for_uri(&mut st, &uri_norm, Some(&new_entries));
                if util::uri_under_any_library(&uri_norm, &st.library_paths) {
                    let graph_nodes_for_uri = st.semantic_graph.nodes_for_uri(&uri_norm).len();
                    let parsed_root_elements = st
                        .index
                        .get(&uri_norm)
                        .and_then(|entry| entry.parsed.as_ref())
                        .map(|root| root.elements.len())
                        .unwrap_or(0);
                    debug!(
                        uri = %uri_norm,
                        parsed_ok = st.index
                            .get(&uri_norm)
                            .and_then(|entry| entry.parsed.as_ref())
                            .is_some(),
                        root_elements = parsed_root_elements,
                        graph_nodes = graph_nodes_for_uri,
                        symbol_entries = new_entries.len(),
                        "library file indexed"
                    );
                    if st
                        .index
                        .get(&uri_norm)
                        .and_then(|entry| entry.parsed.as_ref())
                        .is_some()
                        && new_entries.len() <= 2
                    {
                        low_coverage_library_files.push((
                            uri_norm.to_string(),
                            graph_nodes_for_uri,
                            new_entries.len(),
                        ));
                    }
                }
            }
            for u in &uris_loaded {
                semantic_model::add_cross_document_edges_for_uri(&mut st.semantic_graph, u);
            }
            info!(
                loaded = uris_loaded.len(),
                candidate_files = summary.candidate_files,
                roots = summary.roots_scanned,
                skipped_non_file_roots = summary.roots_skipped_non_file,
                read_failures = summary.read_failures,
                uri_failures = summary.uri_failures,
                sample = ?uris_loaded.iter().take(5).map(|u| u.as_str()).collect::<Vec<_>>(),
                "workspace scan complete"
            );
            if !low_coverage_library_files.is_empty() {
                warn!(
                    files = low_coverage_library_files.len(),
                    "workspace scan low-coverage library files (showing up to 10)"
                );
                for (uri, graph_nodes, symbol_entries) in
                    low_coverage_library_files.iter().take(10)
                {
                    debug!(
                        uri = %uri,
                        graph_nodes = *graph_nodes,
                        symbol_entries = *symbol_entries,
                        "low-coverage library file"
                    );
                }
            }
            if summary.roots_skipped_non_file > 0
                || summary.read_failures > 0
                || summary.uri_failures > 0
            {
                client
                    .log_message(
                        MessageType::WARNING,
                        format!(
                            "workspace scan completed with skips: loaded {} of {} candidate SysML/KerML files across {} root(s); skipped_non_file_roots={}, read_failures={}, uri_failures={}.",
                            summary.files_loaded,
                            summary.candidate_files,
                            summary.roots_scanned,
                            summary.roots_skipped_non_file,
                            summary.read_failures,
                            summary.uri_failures,
                        ),
                    )
                    .await;
            }
        });
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let uri_norm = util::normalize_file_uri(&uri);
        let text = params.text_document.text;
        let parsed = sysml_parser::parse(&text).ok();
        if parsed.is_none() {
            let errs = util::parse_failure_diagnostics(&text, 5);
            let msg = if errs.is_empty() {
                format!(
                    "sysml parse failed for {} (0 diagnostics; parser returned no AST and no error list)",
                    uri_norm.as_str(),
                )
            } else {
                format!(
                    "sysml parse failed for {} ({} error(s)): {}",
                    uri_norm.as_str(),
                    errs.len(),
                    errs.join("; "),
                )
            };
            self.client.log_message(MessageType::WARNING, msg).await;
        }
        {
            let mut state = self.state.write().await;
            update_semantic_graph_for_uri(&mut state, &uri_norm, parsed.as_ref());
            state.index.insert(
                uri_norm.clone(),
                IndexEntry {
                    content: text.clone(),
                    parsed,
                },
            );
            let mut new_entries =
                semantic_model::symbol_entries_for_uri(&state.semantic_graph, &uri_norm);
            if let Some(index_entry) = state.index.get(&uri_norm) {
                library_search::add_short_name_symbol_entries(
                    &mut new_entries,
                    &index_entry.content,
                    &uri_norm,
                );
            }
            update_symbol_table_for_uri(&mut state, &uri_norm, Some(&new_entries));
        }
        self.publish_diagnostics_for_document(uri, &text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let uri_norm = util::normalize_file_uri(&uri);
        let version = params.text_document.version;
        let mut runtime_warnings: Vec<(MessageType, String)> = Vec::new();
        {
            let mut state = self.state.write().await;
            let should_update = if let Some(entry) = state.index.get_mut(&uri_norm) {
                let mut content_changed = false;
                for change in params.content_changes {
                    if let Some(range) = change.range {
                        if let Some(new_text) =
                            util::apply_incremental_change(&entry.content, &range, &change.text)
                        {
                            if new_text != entry.content {
                                entry.content = new_text;
                                content_changed = true;
                            }
                        } else {
                            runtime_warnings.push((
                                MessageType::WARNING,
                                format!(
                                    "didChange: ignored invalid incremental edit for {} at {}:{}..{}:{} (version {}).",
                                    uri_norm,
                                    range.start.line,
                                    range.start.character,
                                    range.end.line,
                                    range.end.character,
                                    version,
                                ),
                            ));
                        }
                    } else if entry.content != change.text {
                        entry.content = change.text;
                        content_changed = true;
                    }
                }
                if content_changed {
                    entry.parsed = sysml_parser::parse(&entry.content).ok();
                    if entry.parsed.is_none() {
                        let errs = util::parse_failure_diagnostics(&entry.content, 5);
                        let msg = if errs.is_empty() {
                            format!(
                                "sysml parse failed after didChange for {} (version {}): parser returned no AST and no diagnostics; keeping diagnostics-only degraded mode.",
                                uri_norm, version
                            )
                        } else {
                            format!(
                                "sysml parse failed after didChange for {} (version {}, {} error(s)): {}; keeping diagnostics-only degraded mode.",
                                uri_norm,
                                version,
                                errs.len(),
                                errs.join("; "),
                            )
                        };
                        runtime_warnings.push((MessageType::LOG, msg));
                    }
                }
                content_changed
            } else {
                runtime_warnings.push((
                    MessageType::WARNING,
                    format!(
                        "didChange: document {} was not in the server index (version {}). Change was ignored until a full open/watch refresh occurs.",
                        uri_norm, version
                    ),
                ));
                false
            };
            if should_update {
                let doc_for_graph = state
                    .index
                    .get(&uri_norm)
                    .and_then(|e| e.parsed.as_ref())
                    .map(|root| semantic_model::build_graph_from_doc(root, &uri_norm));
                if let Some(new_graph) = doc_for_graph {
                    state.semantic_graph.remove_nodes_for_uri(&uri_norm);
                    state.semantic_graph.merge(new_graph);
                    semantic_model::add_cross_document_edges_for_uri(
                        &mut state.semantic_graph,
                        &uri_norm,
                    );
                } else {
                    state.semantic_graph.remove_nodes_for_uri(&uri_norm);
                }
                let mut new_entries =
                    semantic_model::symbol_entries_for_uri(&state.semantic_graph, &uri_norm);
                if let Some(index_entry) = state.index.get(&uri_norm) {
                    library_search::add_short_name_symbol_entries(
                        &mut new_entries,
                        &index_entry.content,
                        &uri_norm,
                    );
                }
                update_symbol_table_for_uri(&mut state, &uri_norm, Some(&new_entries));
            }
        }
        let state = self.state.read().await;
        let text = indexed_text_for_uri(&state, &uri_norm);
        drop(state);
        for (ty, msg) in runtime_warnings {
            self.client.log_message(ty, msg).await;
        }
        self.publish_diagnostics_for_document(uri, &text).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        // Keep index entry (last known content) so workspace features still see it until watch/scan.
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }

    async fn did_change_watched_files(
        &self,
        params: tower_lsp::lsp_types::DidChangeWatchedFilesParams,
    ) {
        use tower_lsp::lsp_types::FileChangeType;
        let mut state = self.state.write().await;
        let mut runtime_warnings: Vec<String> = Vec::new();
        for event in params.changes {
            let uri_norm = util::normalize_file_uri(&event.uri);
            if event.typ == FileChangeType::CREATED || event.typ == FileChangeType::CHANGED {
                match event.uri.to_file_path() {
                    Ok(path) => match tokio::fs::read_to_string(&path).await {
                        Ok(content) => {
                            let parsed = sysml_parser::parse(&content).ok();
                            if parsed.is_none() {
                                let errs = util::parse_failure_diagnostics(&content, 5);
                                runtime_warnings.push(if errs.is_empty() {
                                    format!(
                                        "didChangeWatchedFiles: parse failed for {} after file change, but parser returned no diagnostics.",
                                        uri_norm
                                    )
                                } else {
                                    format!(
                                        "didChangeWatchedFiles: parse failed for {} after file change ({} error(s)): {}",
                                        uri_norm,
                                        errs.len(),
                                        errs.join("; "),
                                    )
                                });
                            }
                            update_semantic_graph_for_uri(&mut state, &uri_norm, parsed.as_ref());
                            state
                                .index
                                .insert(uri_norm.clone(), IndexEntry { content, parsed });
                            let mut new_entries = semantic_model::symbol_entries_for_uri(
                                &state.semantic_graph,
                                &uri_norm,
                            );
                            if let Some(index_entry) = state.index.get(&uri_norm) {
                                library_search::add_short_name_symbol_entries(
                                    &mut new_entries,
                                    &index_entry.content,
                                    &uri_norm,
                                );
                            }
                            update_symbol_table_for_uri(&mut state, &uri_norm, Some(&new_entries));
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
                state.index.remove(&uri_norm);
                remove_symbol_table_entries_for_uri(&mut state, &uri_norm);
                state.semantic_graph.remove_nodes_for_uri(&uri_norm);
            }
        }
        drop(state);
        for msg in runtime_warnings {
            self.client.log_message(MessageType::WARNING, msg).await;
        }
    }

    async fn did_change_configuration(
        &self,
        params: tower_lsp::lsp_types::DidChangeConfigurationParams,
    ) {
        let new_library_paths = params
            .settings
            .get("spec42")
            .map(|v| util::parse_library_paths_from_value(Some(v)))
            .unwrap_or_else(|| util::parse_library_paths_from_value(Some(&params.settings)));
        info!(
            library_paths = new_library_paths.len(),
            paths = ?new_library_paths.iter().map(|u| u.as_str()).collect::<Vec<_>>(),
            "didChangeConfiguration: new library paths"
        );
        let mut state = self.state.write().await;
        let old_library_paths = std::mem::take(&mut state.library_paths);
        if new_library_paths == old_library_paths {
            state.library_paths = old_library_paths;
            return;
        }
        let uris_to_remove: Vec<Url> = state
            .index
            .keys()
            .filter(|uri| util::uri_under_any_library(uri, &old_library_paths))
            .cloned()
            .collect();
        for uri in &uris_to_remove {
            state.index.remove(uri);
            remove_symbol_table_entries_for_uri(&mut state, uri);
            state.semantic_graph.remove_nodes_for_uri(uri);
        }
        state.library_paths = new_library_paths.clone();
        drop(state);
        let state = Arc::clone(&self.state);
        let client = self.client.clone();
        tokio::spawn(async move {
            let (entries, summary) =
                tokio::task::spawn_blocking(move || scan_sysml_files(new_library_paths))
                    .await
                    .unwrap_or_default();
            let mut st = state.write().await;
            let mut uris_loaded = Vec::new();
            let mut runtime_warnings: Vec<String> = Vec::new();
            let mut low_coverage_library_files: Vec<(String, usize, usize)> = Vec::new();
            for (uri, content) in entries {
                let uri_norm = util::normalize_file_uri(&uri);
                let parsed = sysml_parser::parse(&content).ok();
                if parsed.is_none() {
                    let errs = util::parse_failure_diagnostics(&content, 5);
                    runtime_warnings.push(if errs.is_empty() {
                        format!(
                            "didChangeConfiguration: parse failed while indexing library file {} with no diagnostics.",
                            uri_norm
                        )
                    } else {
                        format!(
                            "didChangeConfiguration: parse failed while indexing library file {} ({} error(s)): {}",
                            uri_norm,
                            errs.len(),
                            errs.join("; "),
                        )
                    });
                }
                update_semantic_graph_for_uri(&mut st, &uri_norm, parsed.as_ref());
                uris_loaded.push(uri_norm.clone());
                st.index
                    .insert(uri_norm.clone(), IndexEntry { content, parsed });
                let mut new_entries =
                    semantic_model::symbol_entries_for_uri(&st.semantic_graph, &uri_norm);
                if let Some(index_entry) = st.index.get(&uri_norm) {
                    library_search::add_short_name_symbol_entries(
                        &mut new_entries,
                        &index_entry.content,
                        &uri_norm,
                    );
                }
                update_symbol_table_for_uri(&mut st, &uri_norm, Some(&new_entries));
                if util::uri_under_any_library(&uri_norm, &st.library_paths) {
                    let graph_nodes_for_uri = st.semantic_graph.nodes_for_uri(&uri_norm).len();
                    let parsed_root_elements = st
                        .index
                        .get(&uri_norm)
                        .and_then(|entry| entry.parsed.as_ref())
                        .map(|root| root.elements.len())
                        .unwrap_or(0);
                    debug!(
                        uri = %uri_norm,
                        parsed_ok = st.index
                            .get(&uri_norm)
                            .and_then(|entry| entry.parsed.as_ref())
                            .is_some(),
                        root_elements = parsed_root_elements,
                        graph_nodes = graph_nodes_for_uri,
                        symbol_entries = new_entries.len(),
                        "library file reindexed"
                    );
                    if st
                        .index
                        .get(&uri_norm)
                        .and_then(|entry| entry.parsed.as_ref())
                        .is_some()
                        && new_entries.len() <= 2
                    {
                        low_coverage_library_files.push((
                            uri_norm.to_string(),
                            graph_nodes_for_uri,
                            new_entries.len(),
                        ));
                    }
                }
            }
            for u in &uris_loaded {
                semantic_model::add_cross_document_edges_for_uri(&mut st.semantic_graph, u);
            }
            let library_symbol_count = st
                .symbol_table
                .iter()
                .filter(|entry| util::uri_under_any_library(&entry.uri, &st.library_paths))
                .count();
            info!(
                loaded_files = uris_loaded.len(),
                library_symbols = library_symbol_count,
                "didChangeConfiguration: library reindex complete"
            );
            if !low_coverage_library_files.is_empty() {
                warn!(
                    files = low_coverage_library_files.len(),
                    "didChangeConfiguration: low-coverage library files (showing up to 10)"
                );
                for (uri, graph_nodes, symbol_entries) in
                    low_coverage_library_files.iter().take(10)
                {
                    debug!(
                        uri = %uri,
                        graph_nodes = *graph_nodes,
                        symbol_entries = *symbol_entries,
                        "low-coverage library file after reindex"
                    );
                }
            }
            drop(st);
            if summary.roots_skipped_non_file > 0
                || summary.read_failures > 0
                || summary.uri_failures > 0
            {
                runtime_warnings.push(format!(
                    "didChangeConfiguration: library reindex completed with skips: loaded {} of {} candidate SysML/KerML files across {} root(s); skipped_non_file_roots={}, read_failures={}, uri_failures={}.",
                    summary.files_loaded,
                    summary.candidate_files,
                    summary.roots_scanned,
                    summary.roots_skipped_non_file,
                    summary.read_failures,
                    summary.uri_failures,
                ));
            }
            for msg in runtime_warnings {
                client.log_message(MessageType::WARNING, msg).await;
            }
        });
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .clone();
        let uri_norm = util::normalize_file_uri(&uri);
        let pos = params.text_document_position_params.position;
        let state = self.state.read().await;
        let text = match indexed_text(&state, &uri_norm) {
            Some(t) => t,
            None => return Ok(None),
        };
        let (line, char_start, char_end, word) =
            match word_at_position(&text, pos.line, pos.character) {
                Some(t) => t,
                None => return Ok(None),
            };
        let lookup_name = word
            .rsplit("::")
            .next()
            .map(str::to_string)
            .unwrap_or_else(|| word.clone());
        let qualifier = word.rsplit_once("::").map(|(q, _)| q.to_string());

        let range = Range::new(
            Position::new(line, char_start),
            Position::new(line, char_end),
        );

        // Prefer keyword hover (case-insensitive) so "attribute" shows keyword help, not a symbol named "attribute"
        if let Some(md) = keyword_hover_markdown(&lookup_name.to_lowercase()) {
            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: md,
                }),
                range: Some(range),
            }));
        }

        if let Some(node) = state.semantic_graph.find_node_at_position(&uri_norm, pos) {
            let target_match = state
                .semantic_graph
                .outgoing_typing_or_specializes_targets(node)
                .into_iter()
                .find(|target| {
                    target.name == lookup_name
                        || target
                            .id
                            .qualified_name
                            .ends_with(&format!("::{}", lookup_name))
                });

            let markdown = if let Some(target) = target_match {
                semantic_model::hover_markdown_for_node(
                    &state.semantic_graph,
                    target,
                    target.id.uri != uri_norm,
                )
            } else {
                semantic_model::hover_markdown_for_node(
                    &state.semantic_graph,
                    node,
                    node.id.uri != uri_norm,
                )
            };

            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: markdown,
                }),
                range: Some(range),
            }));
        }

        // Look up in symbol table: collect all matches (same file first) to handle collisions.
        let (same_file, other_files) = collect_symbol_matches_for_lookup(
            &state,
            &uri_norm,
            &lookup_name,
            qualifier.as_deref(),
        );
        let all_matches = if same_file.is_empty() {
            &other_files
        } else {
            &same_file
        };
        if let Some(entry) = all_matches.first() {
            let value = if all_matches.len() > 1 {
                let mut md = format!(
                    "**{}** — {} definitions (use Go to Definition to choose):\n\n",
                    lookup_name,
                    all_matches.len()
                );
                for e in all_matches.iter() {
                    let kind = e.detail.as_deref().unwrap_or("element");
                    let container = e.container_name.as_deref().unwrap_or("(top level)");
                    md.push_str(&format!("• `{}` in `{}`\n", kind, container));
                }
                md.push('\n');
                md.push_str(&util::symbol_hover_markdown(entry, entry.uri != uri_norm));
                md
            } else {
                util::symbol_hover_markdown(entry, entry.uri != uri_norm)
            };
            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value,
                }),
                range: Some(range),
            }));
        }

        Ok(None)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let uri_norm = util::normalize_file_uri(&uri);
        let pos = params.text_document_position.position;
        let state = self.state.read().await;
        let text = match indexed_text(&state, &uri_norm) {
            Some(t) => t,
            None => return Ok(None),
        };
        let line_prefix = line_prefix_at_position(&text, pos.line, pos.character);
        let prefix = completion_prefix(&line_prefix);

        let mut items = Vec::new();

        for kw in sysml_keywords() {
            if prefix.is_empty() || kw.starts_with(prefix) {
                items.push(CompletionItem {
                    label: (*kw).to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: keyword_doc(kw).map(String::from),
                    ..Default::default()
                });
            }
        }

        let mut seen = std::collections::HashSet::<String>::new();
        for entry in &state.symbol_table {
            if (prefix.is_empty() || entry.name.starts_with(prefix))
                && seen.insert(entry.name.clone())
            {
                items.push(CompletionItem {
                    label: entry.name.clone(),
                    kind: Some(CompletionItemKind::REFERENCE),
                    detail: entry.description.clone().or_else(|| entry.detail.clone()),
                    ..Default::default()
                });
            }
        }

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .clone();
        let uri_norm = util::normalize_file_uri(&uri);
        let pos = params.text_document_position_params.position;
        let state = self.state.read().await;
        let text = match state.index.get(&uri_norm).map(|e| e.content.as_str()) {
            Some(t) => t,
            None => return Ok(None),
        };
        let line = text.lines().nth(pos.line as usize).unwrap_or("");
        let cursor_prefix = line
            .chars()
            .take(pos.character as usize)
            .collect::<String>();
        let active_param = cursor_prefix.matches(',').count() as u32;
        let label = if line.contains("part def") {
            "part def <Name> : <Type>"
        } else if line.contains("port def") || line.contains("port ") {
            "port <name> : <PortType>"
        } else if line.contains("attribute") {
            "attribute <name> : <AttributeType>"
        } else {
            "name : Type"
        };
        Ok(Some(SignatureHelp {
            signatures: vec![SignatureInformation {
                label: label.to_string(),
                documentation: Some(Documentation::String(
                    "Basic SysML declaration shape".to_string(),
                )),
                parameters: None,
                active_parameter: Some(active_param),
            }],
            active_signature: Some(0),
            active_parameter: Some(active_param),
        }))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .clone();
        let uri_norm = util::normalize_file_uri(&uri);
        let pos = params.text_document_position_params.position;
        let state = self.state.read().await;
        let text = match indexed_text(&state, &uri_norm) {
            Some(t) => t,
            None => return Ok(None),
        };
        let (_, _, _, word) = match word_at_position(&text, pos.line, pos.character) {
            Some(t) => t,
            None => return Ok(None),
        };
        let lookup_name = word
            .rsplit("::")
            .next()
            .map(str::to_string)
            .unwrap_or_else(|| word.clone());
        let qualifier = word.rsplit_once("::").map(|(q, _)| q.to_string());
        debug!(
            uri = %uri_norm,
            line = pos.line,
            character = pos.character,
            word = %word,
            lookup_name = %lookup_name,
            qualifier = ?qualifier,
            "goto_definition tokenized input"
        );

        if is_reserved_keyword(&word) || is_reserved_keyword(&lookup_name) {
            return Ok(None);
        }

        // 2.2: Try graph-based resolution via typing/specializes edges (works cross-file).
        if let Some(node) = state.semantic_graph.find_node_at_position(&uri_norm, pos) {
            for target in state
                .semantic_graph
                .outgoing_typing_or_specializes_targets(node)
            {
                if target.name == lookup_name
                    || target
                        .id
                        .qualified_name
                        .ends_with(&format!("::{}", lookup_name))
                {
                    return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                        uri: target.id.uri.clone(),
                        range: target.range,
                    })));
                }
            }
        }

        // Fall back to symbol table: collect all matches to handle name collisions.
        let (same_file_matches, other_file_matches) = collect_symbol_matches_for_lookup(
            &state,
            &uri_norm,
            &lookup_name,
            qualifier.as_deref(),
        );
        let same_file: Vec<Location> = same_file_matches
            .into_iter()
            .map(|e| Location {
                uri: e.uri.clone(),
                range: e.range,
            })
            .collect();
        let other_files: Vec<Location> = other_file_matches
            .into_iter()
            .map(|e| Location {
                uri: e.uri.clone(),
                range: e.range,
            })
            .collect();
        if same_file.is_empty() && other_files.is_empty() {
            let similar: Vec<String> = state
                .symbol_table
                .iter()
                .filter(|e| e.name.eq_ignore_ascii_case(&lookup_name))
                .take(5)
                .map(|e| {
                    format!(
                        "{} @ {} (container={})",
                        e.name,
                        e.uri.path(),
                        e.container_name.as_deref().unwrap_or("(none)")
                    )
                })
                .collect();
            debug!(
                lookup_name = %lookup_name,
                qualifier = ?qualifier,
                symbol_table_size = state.symbol_table.len(),
                similar = ?similar,
                "goto_definition no symbol-table matches"
            );
        } else {
            debug!(
                lookup_name = %lookup_name,
                qualifier = ?qualifier,
                same_file_matches = same_file.len(),
                other_file_matches = other_files.len(),
                "goto_definition symbol-table matches"
            );
        }
        let locations = if same_file.is_empty() {
            other_files
        } else {
            same_file
        };
        if let [location] = locations.as_slice() {
            return Ok(Some(GotoDefinitionResponse::Scalar(location.clone())));
        }
        if !locations.is_empty() {
            return Ok(Some(GotoDefinitionResponse::Array(locations)));
        }
        if let Some(q) = qualifier.as_deref() {
            debug_qualified_lookup_context(&state, &lookup_name, q, &uri_norm);
        }
        Ok(None)
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri.clone();
        let uri_norm = util::normalize_file_uri(&uri);
        let pos = params.text_document_position.position;
        let include_declaration = params.context.include_declaration;
        let state = self.state.read().await;
        let text = match state.index.get(&uri_norm).map(|e| e.content.as_str()) {
            Some(t) => t.to_string(),
            None => return Ok(None),
        };
        let (_, _, _, word) = match word_at_position(&text, pos.line, pos.character) {
            Some(t) => t,
            None => return Ok(None),
        };

        let mut def_locations: Vec<(Url, Range)> = Vec::new();
        for (u, entry) in state.index.iter() {
            if let Some(ref doc) = entry.parsed {
                for (name, range) in collect_definition_ranges(doc) {
                    if name == word {
                        def_locations.push((u.clone(), range));
                    }
                }
            }
        }

        let mut locations: Vec<Location> = Vec::new();
        for (u, entry) in state.index.iter() {
            for range in find_reference_ranges(&entry.content, &word) {
                locations.push(Location {
                    uri: u.clone(),
                    range,
                });
            }
        }

        if !include_declaration {
            for (def_uri, def_range) in &def_locations {
                locations.retain(|loc| !(loc.uri == *def_uri && loc.range == *def_range));
            }
        }

        Ok(Some(locations))
    }

    async fn document_link(
        &self,
        params: DocumentLinkParams,
    ) -> Result<Option<Vec<DocumentLink>>> {
        let uri = params.text_document.uri;
        let uri_norm = util::normalize_file_uri(&uri);
        let state = self.state.read().await;
        let text = match state.index.get(&uri_norm).map(|e| e.content.as_str()) {
            Some(t) => t,
            None => return Ok(None),
        };
        let links = collect_document_links(text, |import_name| {
            state
                .symbol_table
                .iter()
                .find(|e| e.name == import_name)
                .map(|s| s.uri.clone())
        });
        Ok(Some(links))
    }

    async fn document_highlight(
        &self,
        params: DocumentHighlightParams,
    ) -> Result<Option<Vec<DocumentHighlight>>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .clone();
        let uri_norm = util::normalize_file_uri(&uri);
        let pos = params.text_document_position_params.position;
        let state = self.state.read().await;
        let text = match state.index.get(&uri_norm).map(|e| e.content.as_str()) {
            Some(t) => t.to_string(),
            None => return Ok(None),
        };
        let (_, _, _, word) = match word_at_position(&text, pos.line, pos.character) {
            Some(t) => t,
            None => return Ok(None),
        };
        if is_reserved_keyword(&word) {
            return Ok(None);
        }
        let highlights: Vec<DocumentHighlight> = find_reference_ranges(&text, &word)
            .into_iter()
            .map(|range| DocumentHighlight {
                range,
                kind: Some(DocumentHighlightKind::TEXT),
            })
            .collect();
        Ok(Some(highlights))
    }

    async fn selection_range(
        &self,
        params: SelectionRangeParams,
    ) -> Result<Option<Vec<SelectionRange>>> {
        let uri = params.text_document.uri;
        let uri_norm = util::normalize_file_uri(&uri);
        let state = self.state.read().await;
        let text = match state.index.get(&uri_norm).map(|e| e.content.as_str()) {
            Some(t) => t,
            None => return Ok(None),
        };
        let out = selection_ranges_for_positions(text, &params.positions, word_at_position);
        Ok(Some(out))
    }

    async fn prepare_rename(
        &self,
        params: TextDocumentPositionParams,
    ) -> Result<Option<PrepareRenameResponse>> {
        let uri = params.text_document.uri;
        let uri_norm = util::normalize_file_uri(&uri);
        let pos = params.position;
        let state = self.state.read().await;
        let text = match state.index.get(&uri_norm).map(|e| e.content.as_str()) {
            Some(t) => t.to_string(),
            None => return Ok(None),
        };
        let (line, char_start, char_end, word) =
            match word_at_position(&text, pos.line, pos.character) {
                Some(t) => t,
                None => return Ok(None),
            };
        if is_reserved_keyword(&word) {
            return Ok(None);
        }
        let range = Range::new(
            Position::new(line, char_start),
            Position::new(line, char_end),
        );
        Ok(Some(PrepareRenameResponse::Range(range)))
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let uri = params.text_document_position.text_document.uri.clone();
        let uri_norm = util::normalize_file_uri(&uri);
        let pos = params.text_document_position.position;
        let new_name = params.new_name;
        let state = self.state.read().await;
        let text = match state.index.get(&uri_norm).map(|e| e.content.as_str()) {
            Some(t) => t.to_string(),
            None => return Ok(None),
        };
        let (_, _, _, word) = match word_at_position(&text, pos.line, pos.character) {
            Some(t) => t,
            None => return Ok(None),
        };
        if is_reserved_keyword(&word) {
            return Ok(None);
        }

        let mut locations: Vec<Location> = Vec::new();
        for (u, entry) in state.index.iter() {
            for range in find_reference_ranges(&entry.content, &word) {
                locations.push(Location {
                    uri: u.clone(),
                    range,
                });
            }
        }

        if locations.is_empty() {
            return Ok(Some(WorkspaceEdit::default()));
        }

        let mut changes: std::collections::HashMap<Url, Vec<TextEdit>> =
            std::collections::HashMap::new();
        for loc in locations {
            changes.entry(loc.uri.clone()).or_default().push(TextEdit {
                range: loc.range,
                new_text: new_name.clone(),
            });
        }
        Ok(Some(WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        }))
    }

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let uri = params.text_document.uri;
        let uri_norm = util::normalize_file_uri(&uri);
        let state = self.state.read().await;
        match state.index.get(&uri_norm) {
            Some(_) => {}
            None => return Ok(None),
        };
        let hints = build_inlay_hints(&state, &uri_norm);
        Ok(Some(hints))
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;
        let uri_norm = util::normalize_file_uri(&uri);
        let state = self.state.read().await;
        let entry = match state.index.get(&uri_norm) {
            Some(e) => e,
            None => return Ok(None),
        };
        let doc = match &entry.parsed {
            Some(d) => d,
            None => return Ok(None),
        };
        let raw_symbols = collect_document_symbols(doc);
        let empty_name_examples: Vec<String> = raw_symbols
            .iter()
            .filter(|s| s.name.trim().is_empty())
            .take(5)
            .map(|s| s.detail.clone().unwrap_or_else(|| "(no detail)".to_string()))
            .collect();
        let raw_count = raw_symbols.len();
        let symbols = sanitize_document_symbols(raw_symbols);
        let sanitized_count = symbols.len();
        if sanitized_count < raw_count {
            self.client
                .log_message(
                    MessageType::WARNING,
                    format!(
                        "documentSymbol: filtered {} empty-name symbol(s) for {}. examples={:?}",
                        raw_count - sanitized_count,
                        uri_norm.as_str(),
                        empty_name_examples
                    ),
                )
                .await;
        }
        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    }

    async fn folding_range(&self, params: FoldingRangeParams) -> Result<Option<Vec<FoldingRange>>> {
        let uri = params.text_document.uri;
        let uri_norm = util::normalize_file_uri(&uri);
        let state = self.state.read().await;
        let entry = match state.index.get(&uri_norm) {
            Some(e) => e,
            None => return Ok(None),
        };
        let doc = match &entry.parsed {
            Some(d) => d,
            None => return Ok(None),
        };
        Ok(Some(collect_folding_ranges(doc)))
    }

    #[allow(deprecated)] // SymbolInformation.deprecated; use tags in future
    async fn symbol(
        &self,
        params: tower_lsp::lsp_types::WorkspaceSymbolParams,
    ) -> Result<Option<Vec<tower_lsp::lsp_types::SymbolInformation>>> {
        let query = params.query.to_lowercase();
        let state = self.state.read().await;
        let out: Vec<SymbolInformation> = state
            .symbol_table
            .iter()
            .filter(|e| query.is_empty() || e.name.to_lowercase().contains(&query))
            .map(|e| SymbolInformation {
                name: e.name.clone(),
                kind: e.kind,
                tags: None,
                deprecated: None,
                location: Location {
                    uri: e.uri.clone(),
                    range: e.range,
                },
                container_name: e.container_name.clone(),
            })
            .collect();
        Ok(Some(out))
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = params.text_document.uri.clone();
        let uri_norm = util::normalize_file_uri(&uri);
        let state = self.state.read().await;
        let text = match state.index.get(&uri_norm).map(|e| e.content.as_str()) {
            Some(t) => t.to_string(),
            None => return Ok(None),
        };
        drop(state);

        let mut actions: Vec<CodeActionOrCommand> = Vec::new();
        if let Some(action) = suggest_wrap_in_package(&text, &uri) {
            actions.push(CodeActionOrCommand::CodeAction(action));
        }
        Ok(Some(actions))
    }

    async fn code_lens(&self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        let uri = params.text_document.uri;
        let uri_norm = util::normalize_file_uri(&uri);
        let state = self.state.read().await;
        let out = build_code_lens(&state, &uri_norm);
        Ok(Some(out))
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;
        let uri_norm = util::normalize_file_uri(&uri);
        let state = self.state.read().await;
        let text = match state.index.get(&uri_norm).map(|e| e.content.as_str()) {
            Some(t) => t.to_string(),
            None => return Ok(None),
        };
        drop(state);
        Ok(Some(format_document(&text, &params.options)))
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;
        let uri_norm = util::normalize_file_uri(&uri);
        let state = self.state.read().await;
        let (text, ast_ranges) = match state.index.get(&uri_norm) {
            Some(e) => (
                e.content.clone(),
                e.parsed.as_ref().map(ast_semantic_ranges),
            ),
            None => return Ok(None),
        };
        drop(state);
        let (tokens, log_lines) = semantic_tokens_full(&text, ast_ranges.as_deref());
        for line in &log_lines {
            self.client.log_message(MessageType::LOG, line).await;
        }
        Ok(Some(SemanticTokensResult::Tokens(tokens)))
    }

    async fn semantic_tokens_range(
        &self,
        params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        let uri = params.text_document.uri;
        let uri_norm = util::normalize_file_uri(&uri);
        let range = params.range;
        let state = self.state.read().await;
        let (text, ast_ranges) = match state.index.get(&uri_norm) {
            Some(e) => (
                e.content.clone(),
                e.parsed.as_ref().map(ast_semantic_ranges),
            ),
            None => return Ok(None),
        };
        drop(state);
        let (tokens, log_lines) = semantic_tokens_range(
            &text,
            range.start.line,
            range.start.character,
            range.end.line,
            range.end.character,
            ast_ranges.as_deref(),
        );
        for line in &log_lines {
            self.client.log_message(MessageType::LOG, line).await;
        }
        Ok(Some(SemanticTokensRangeResult::Tokens(tokens)))
    }

    async fn linked_editing_range(
        &self,
        params: LinkedEditingRangeParams,
    ) -> Result<Option<LinkedEditingRanges>> {
        let uri = params.text_document_position_params.text_document.uri;
        let uri_norm = util::normalize_file_uri(&uri);
        let pos = params.text_document_position_params.position;
        let state = self.state.read().await;
        let text = match state.index.get(&uri_norm).map(|e| e.content.as_str()) {
            Some(t) => t,
            None => return Ok(None),
        };
        let (line, _, _, word) = match word_at_position(text, pos.line, pos.character) {
            Some(t) => t,
            None => return Ok(None),
        };
        if is_reserved_keyword(&word) {
            return Ok(None);
        }
        // Keep linked editing constrained to same-line declaration context to avoid broad global edits.
        let line_text = text.lines().nth(line as usize).unwrap_or("");
        let declaration_like = line_text.contains(" def ")
            || line_text.trim_start().starts_with("part ")
            || line_text.trim_start().starts_with("port ")
            || line_text.trim_start().starts_with("attribute ")
            || line_text.trim_start().starts_with("action ");
        if !declaration_like {
            return Ok(None);
        }
        let ranges: Vec<_> = find_reference_ranges(text, &word)
            .into_iter()
            .filter(|r| r.start.line == line)
            .collect();
        if ranges.is_empty() {
            return Ok(None);
        }
        Ok(Some(LinkedEditingRanges {
            ranges,
            word_pattern: None,
        }))
    }

    async fn moniker(&self, params: MonikerParams) -> Result<Option<Vec<Moniker>>> {
        let uri = params.text_document_position_params.text_document.uri;
        let uri_norm = util::normalize_file_uri(&uri);
        let pos = params.text_document_position_params.position;
        let state = self.state.read().await;
        let node = match state.semantic_graph.find_node_at_position(&uri_norm, pos) {
            Some(n) => n,
            None => return Ok(None),
        };
        Ok(Some(vec![moniker_for_node(node)]))
    }

    async fn prepare_type_hierarchy(
        &self,
        params: TypeHierarchyPrepareParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        let uri = params.text_document_position_params.text_document.uri;
        let uri_norm = util::normalize_file_uri(&uri);
        let pos = params.text_document_position_params.position;
        let state = self.state.read().await;
        let node = match state.semantic_graph.find_node_at_position(&uri_norm, pos) {
            Some(n) => n,
            None => return Ok(None),
        };
        Ok(Some(vec![type_hierarchy_item_for_node(node)]))
    }

    async fn supertypes(
        &self,
        params: TypeHierarchySupertypesParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        let uri = params.item.uri.clone();
        let range = params.item.selection_range;
        let state = self.state.read().await;
        let node = match state.semantic_graph.find_node_at_position(&uri, range.start) {
            Some(n) => n,
            None => return Ok(None),
        };
        let items = state
            .semantic_graph
            .outgoing_typing_or_specializes_targets(node)
            .into_iter()
            .map(type_hierarchy_item_for_node)
            .collect::<Vec<_>>();
        Ok(Some(items))
    }

    async fn subtypes(
        &self,
        params: TypeHierarchySubtypesParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        let uri = params.item.uri.clone();
        let range = params.item.selection_range;
        let state = self.state.read().await;
        let node = match state.semantic_graph.find_node_at_position(&uri, range.start) {
            Some(n) => n,
            None => return Ok(None),
        };
        let items = state
            .semantic_graph
            .incoming_typing_or_specializes_sources(node)
            .into_iter()
            .map(type_hierarchy_item_for_node)
            .collect::<Vec<_>>();
        Ok(Some(items))
    }

    async fn prepare_call_hierarchy(
        &self,
        params: CallHierarchyPrepareParams,
    ) -> Result<Option<Vec<CallHierarchyItem>>> {
        let uri = params.text_document_position_params.text_document.uri;
        let uri_norm = util::normalize_file_uri(&uri);
        let pos = params.text_document_position_params.position;
        let state = self.state.read().await;
        let node = match state.semantic_graph.find_node_at_position(&uri_norm, pos) {
            Some(n) => n,
            None => return Ok(None),
        };
        Ok(Some(vec![call_hierarchy_item_for_node(node)]))
    }

    async fn incoming_calls(
        &self,
        params: CallHierarchyIncomingCallsParams,
    ) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
        let uri = params.item.uri.clone();
        let range = params.item.selection_range;
        let state = self.state.read().await;
        let node = match state.semantic_graph.find_node_at_position(&uri, range.start) {
            Some(n) => n,
            None => return Ok(None),
        };
        let from_ranges = vec![range];
        let calls = state
            .semantic_graph
            .incoming_perform_sources(node)
            .into_iter()
            .map(|src| CallHierarchyIncomingCall {
                from: call_hierarchy_item_for_node(src),
                from_ranges: from_ranges.clone(),
            })
            .collect();
        Ok(Some(calls))
    }

    async fn outgoing_calls(
        &self,
        params: CallHierarchyOutgoingCallsParams,
    ) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
        let uri = params.item.uri.clone();
        let range = params.item.selection_range;
        let state = self.state.read().await;
        let node = match state.semantic_graph.find_node_at_position(&uri, range.start) {
            Some(n) => n,
            None => return Ok(None),
        };
        let from_ranges = vec![range];
        let calls = state
            .semantic_graph
            .outgoing_perform_targets(node)
            .into_iter()
            .map(|target| CallHierarchyOutgoingCall {
                to: call_hierarchy_item_for_node(target),
                from_ranges: from_ranges.clone(),
            })
            .collect();
        Ok(Some(calls))
    }
}

impl Backend {
    async fn sysml_model(&self, params: serde_json::Value) -> Result<dto::SysmlModelResultDto> {
        let state = self.state.read().await;
        sysml_model_result(&self.client, &state, &self.config, params).await
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
        let params: dto::SysmlLibrarySearchParamsDto = serde_json::from_value(params).map_err(
            |error| tower_lsp::jsonrpc::Error::invalid_params(error.to_string()),
        )?;
        let query = params.query.trim().to_lowercase();
        let limit = params.limit.unwrap_or(100).clamp(1, 500);
        let state = self.state.read().await;
        let library_symbol_count = state
            .symbol_table
            .iter()
            .filter(|entry| util::uri_under_any_library(&entry.uri, &state.library_paths))
            .count();
        debug!(
            query = %query,
            limit = limit,
            library_paths = state.library_paths.len(),
            library_symbols = library_symbol_count,
            "librarySearch"
        );

        let mut ranked: Vec<(i64, &crate::language::SymbolEntry)> = state
            .symbol_table
            .iter()
            .filter(|entry| util::uri_under_any_library(&entry.uri, &state.library_paths))
            .filter_map(|entry| {
                let normalized_name =
                    library_search::normalized_library_symbol_name(entry, state.index.get(&entry.uri));
                let score = if query.is_empty() {
                    1_000
                } else {
                    library_search::library_search_score(&normalized_name, &query)?
                };
                Some((score, entry))
            })
            .collect();

        ranked.sort_by(|(score_a, entry_a), (score_b, entry_b)| {
            score_b
                .cmp(score_a)
                .then(entry_a.name.len().cmp(&entry_b.name.len()))
                .then(entry_a.name.cmp(&entry_b.name))
        });

        let total = ranked.len();
        let items: Vec<dto::SysmlLibrarySearchItemDto> = ranked
            .into_iter()
            .take(limit)
            .map(|(score, entry)| dto::SysmlLibrarySearchItemDto {
                name: library_search::normalized_library_symbol_name(entry, state.index.get(&entry.uri)),
                kind: library_search::symbol_kind_label(entry.kind).to_string(),
                container: entry.container_name.clone(),
                uri: entry.uri.to_string(),
                range: dto::range_to_dto(entry.range),
                score,
                source: library_search::library_source_label(&entry.uri).to_string(),
                path: entry.uri.path().to_string(),
            })
            .collect();

        let sources = library_search::build_library_tree(items);
        let symbol_total = sources
            .iter()
            .map(|src| src.packages.iter().map(|pkg| pkg.symbols.len()).sum::<usize>())
            .sum();
        Ok(dto::SysmlLibrarySearchResultDto {
            sources,
            symbol_total,
            total,
        })
    }

    async fn publish_diagnostics_for_document(&self, uri: tower_lsp::lsp_types::Url, text: &str) {
        let mut diagnostics = Vec::new();
        let result = sysml_parser::parse_with_diagnostics(text);
        for e in &result.errors {
            let range = e
                .to_lsp_range()
                .map(|(sl, sc, el, ec)| Range {
                    start: Position::new(sl, sc),
                    end: Position::new(el, ec),
                })
                .unwrap_or_else(|| Range {
                    start: Position::new(0, 0),
                    end: Position::new(0, 0),
                });
            let severity = e
                .severity
                .map(|s| match s {
                    sysml_parser::DiagnosticSeverity::Error => DiagnosticSeverity::ERROR,
                    sysml_parser::DiagnosticSeverity::Warning => DiagnosticSeverity::WARNING,
                })
                .unwrap_or(DiagnosticSeverity::ERROR);
            diagnostics.push(Diagnostic {
                range,
                severity: Some(severity),
                code: e.code.clone().map(tower_lsp::lsp_types::NumberOrString::String),
                code_description: None,
                source: Some("sysml".to_string()),
                message: e.message.clone(),
                related_information: None,
                tags: None,
                data: None,
            });
        }
        if result.errors.is_empty() {
            for range in util::missing_semicolon_ranges(text) {
                diagnostics.push(Diagnostic {
                    range,
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(tower_lsp::lsp_types::NumberOrString::String(
                        "missing_semicolon".to_string(),
                    )),
                    code_description: None,
                    source: Some("sysml".to_string()),
                    message: "Missing ';' at end of statement.".to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }
        }
        // When parse succeeded, add semantic diagnostics from all check providers.
        if result.errors.is_empty() {
            let uri_norm = util::normalize_file_uri(&uri);
            let state = self.state.read().await;
            for provider in &self.config.check_providers {
                diagnostics.extend(provider.compute_diagnostics(&state.semantic_graph, &uri_norm));
            }
            drop(state);
        }
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

fn sanitize_document_symbols(
    symbols: Vec<tower_lsp::lsp_types::DocumentSymbol>,
) -> Vec<tower_lsp::lsp_types::DocumentSymbol> {
    fn sanitize_one(
        mut symbol: tower_lsp::lsp_types::DocumentSymbol,
    ) -> Option<tower_lsp::lsp_types::DocumentSymbol> {
        if symbol.name.trim().is_empty() {
            return None;
        }
        if let Some(children) = symbol.children.take() {
            let cleaned: Vec<_> = children.into_iter().filter_map(sanitize_one).collect();
            symbol.children = if cleaned.is_empty() {
                None
            } else {
                Some(cleaned)
            };
        }
        Some(symbol)
    }

    symbols.into_iter().filter_map(sanitize_one).collect()
}

/// Run the Spec42 LSP server using the provided configuration.
pub async fn run(config: Arc<Spec42Config>, server_name: &str) {
    crate::logging::init_tracing();
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
    .custom_method("sysml/serverStats", Backend::sysml_server_stats)
    .custom_method("sysml/clearCache", Backend::sysml_clear_cache)
    .custom_method("sysml/librarySearch", Backend::sysml_library_search)
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}


fn collect_symbol_matches_for_lookup<'a>(
    state: &'a crate::lsp::types::ServerState,
    uri_norm: &Url,
    lookup_name: &str,
    qualifier: Option<&str>,
) -> (
    Vec<&'a crate::language::SymbolEntry>,
    Vec<&'a crate::language::SymbolEntry>,
) {
    let mut same_file = Vec::new();
    let mut other_files = Vec::new();
    for entry in state.symbol_table.iter() {
        if !symbol_matches_definition_lookup(
            &entry.name,
            entry.container_name.as_deref(),
            entry.uri.path(),
            lookup_name,
            qualifier,
        ) {
            continue;
        }
        if entry.uri == *uri_norm {
            same_file.push(entry);
        } else {
            other_files.push(entry);
        }
    }
    (same_file, other_files)
}

fn symbol_matches_definition_lookup(
    candidate_name: &str,
    container_name: Option<&str>,
    candidate_path: &str,
    lookup_name: &str,
    qualifier: Option<&str>,
) -> bool {
    if candidate_name != lookup_name {
        return false;
    }
    match qualifier {
        None => true,
        Some(q) => {
            let q_lc = q.to_ascii_lowercase();
            if container_name
                .map(|c| {
                    let c_lc = c.to_ascii_lowercase();
                    c_lc == q_lc || c_lc.ends_with(&format!("::{}", q_lc))
                })
                .unwrap_or(false)
            {
                return true;
            }
            let path_lc = candidate_path.to_ascii_lowercase();
            path_lc.ends_with(&format!("/{}.sysml", q_lc))
                || path_lc.ends_with(&format!("/{}.kerml", q_lc))
        }
    }
}

fn debug_qualified_lookup_context(
    state: &crate::lsp::types::ServerState,
    lookup_name: &str,
    qualifier: &str,
    request_uri: &Url,
) {
    if lookup_name.is_empty() || qualifier.is_empty() {
        return;
    }
    let qualifier_lc = qualifier.to_ascii_lowercase();
    let needle = format!("<{}>", lookup_name);
    let mut inspected: Vec<String> = Vec::new();
    let mut qualifier_symbol_hits = 0usize;
    for (candidate_uri, entry) in state.index.iter() {
        let candidate_uri = util::normalize_file_uri(candidate_uri);
        if candidate_uri == *request_uri {
            continue;
        }
        if !util::uri_under_any_library(&candidate_uri, &state.library_paths) {
            continue;
        }
        let path_lc = candidate_uri.path().to_ascii_lowercase();
        let path_matches = path_lc.ends_with(&format!("/{}.sysml", qualifier_lc))
            || path_lc.ends_with(&format!("/{}.kerml", qualifier_lc));
        if !path_matches {
            continue;
        }
        let symbols_for_uri: Vec<&crate::language::SymbolEntry> = state
            .symbol_table
            .iter()
            .filter(|s| util::normalize_file_uri(&s.uri) == candidate_uri)
            .collect();
        qualifier_symbol_hits += symbols_for_uri
            .iter()
            .filter(|s| s.name.eq_ignore_ascii_case(lookup_name))
            .count();
        let has_angle_short = entry.content.contains(&needle);
        inspected.push(format!(
            "{} symbols={} matching_name={} has_angle_short={}",
            candidate_uri.path(),
            symbols_for_uri.len(),
            symbols_for_uri
                .iter()
                .filter(|s| s.name.eq_ignore_ascii_case(lookup_name))
                .count(),
            has_angle_short
        ));
        if inspected.len() >= 5 {
            break;
        }
    }
    warn!(
        lookup_name = %lookup_name,
        qualifier = %qualifier,
        qualifier_symbol_hits = qualifier_symbol_hits,
        inspected = ?inspected,
        "goto_definition qualified lookup diagnostics"
    );
}

