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
        eprintln!(
            "[sysml-ls] initialize: workspace_roots={} library_paths={} -> {:?}",
            roots.len(),
            library_paths.len(),
            library_paths
                .iter()
                .map(|u| u.as_str())
                .collect::<Vec<_>>()
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
            capabilities: server_capabilities(),
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
                    eprintln!(
                        "[sysml-ls] workspace scan: parse failed for {} ({} diagnostics): {:?}",
                        uri_norm.as_str(),
                        errs.len(),
                        errs,
                    );
                    if errs.is_empty() {
                        eprintln!(
                            "[sysml-ls] parse() returned None but parse_with_diagnostics had 0 errors (parser may fail without filling diagnostics)",
                        );
                    }
                }
                update_semantic_graph_for_uri(&mut st, &uri_norm, parsed.as_ref());
                uris_loaded.push(uri_norm.clone());
                st.index
                    .insert(uri_norm.clone(), IndexEntry { content, parsed });
                let new_entries =
                    semantic_model::symbol_entries_for_uri(&st.semantic_graph, &uri_norm);
                update_symbol_table_for_uri(&mut st, &uri_norm, Some(&new_entries));
                if util::uri_under_any_library(&uri_norm, &st.library_paths) {
                    let graph_nodes_for_uri = st.semantic_graph.nodes_for_uri(&uri_norm).len();
                    let parsed_root_elements = st
                        .index
                        .get(&uri_norm)
                        .and_then(|entry| entry.parsed.as_ref())
                        .map(|root| root.elements.len())
                        .unwrap_or(0);
                    eprintln!(
                        "[sysml-ls] library file indexed: uri={} parsed_ok={} root_elements={} graph_nodes={} symbol_entries={}",
                        uri_norm,
                        st.index
                            .get(&uri_norm)
                            .and_then(|entry| entry.parsed.as_ref())
                            .is_some(),
                        parsed_root_elements,
                        graph_nodes_for_uri,
                        new_entries.len()
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
            eprintln!(
                "[sysml-ls] workspace scan complete: loaded={} candidate_files={} roots={} skipped_non_file_roots={} read_failures={} uri_failures={}. Sample: {:?}",
                uris_loaded.len(),
                summary.candidate_files,
                summary.roots_scanned,
                summary.roots_skipped_non_file,
                summary.read_failures,
                summary.uri_failures,
                uris_loaded.iter().take(5).map(|u| u.as_str()).collect::<Vec<_>>(),
            );
            if !low_coverage_library_files.is_empty() {
                eprintln!(
                    "[sysml-ls] workspace scan low-coverage library files: {} (showing up to 10)",
                    low_coverage_library_files.len()
                );
                for (uri, graph_nodes, symbol_entries) in
                    low_coverage_library_files.iter().take(10)
                {
                    eprintln!(
                        "  - {} graph_nodes={} symbol_entries={}",
                        uri, graph_nodes, symbol_entries
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
            let new_entries =
                semantic_model::symbol_entries_for_uri(&state.semantic_graph, &uri_norm);
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
                let new_entries =
                    semantic_model::symbol_entries_for_uri(&state.semantic_graph, &uri_norm);
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
                            let new_entries = semantic_model::symbol_entries_for_uri(
                                &state.semantic_graph,
                                &uri_norm,
                            );
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
        eprintln!(
            "[sysml-ls] didChangeConfiguration: new library_paths={} -> {:?}",
            new_library_paths.len(),
            new_library_paths
                .iter()
                .map(|u| u.as_str())
                .collect::<Vec<_>>()
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
                let new_entries =
                    semantic_model::symbol_entries_for_uri(&st.semantic_graph, &uri_norm);
                update_symbol_table_for_uri(&mut st, &uri_norm, Some(&new_entries));
                if util::uri_under_any_library(&uri_norm, &st.library_paths) {
                    let graph_nodes_for_uri = st.semantic_graph.nodes_for_uri(&uri_norm).len();
                    let parsed_root_elements = st
                        .index
                        .get(&uri_norm)
                        .and_then(|entry| entry.parsed.as_ref())
                        .map(|root| root.elements.len())
                        .unwrap_or(0);
                    eprintln!(
                        "[sysml-ls] library file reindexed: uri={} parsed_ok={} root_elements={} graph_nodes={} symbol_entries={}",
                        uri_norm,
                        st.index
                            .get(&uri_norm)
                            .and_then(|entry| entry.parsed.as_ref())
                            .is_some(),
                        parsed_root_elements,
                        graph_nodes_for_uri,
                        new_entries.len()
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
            eprintln!(
                "[sysml-ls] didChangeConfiguration: library reindex loaded_files={} library_symbols={}",
                uris_loaded.len(),
                library_symbol_count
            );
            if !low_coverage_library_files.is_empty() {
                eprintln!(
                    "[sysml-ls] didChangeConfiguration: low-coverage library files {} (showing up to 10)",
                    low_coverage_library_files.len()
                );
                for (uri, graph_nodes, symbol_entries) in
                    low_coverage_library_files.iter().take(10)
                {
                    eprintln!(
                        "  - {} graph_nodes={} symbol_entries={}",
                        uri, graph_nodes, symbol_entries
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

        let range = Range::new(
            Position::new(line, char_start),
            Position::new(line, char_end),
        );

        // Prefer keyword hover (case-insensitive) so "attribute" shows keyword help, not a symbol named "attribute"
        if let Some(md) = keyword_hover_markdown(&word.to_lowercase()) {
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
                    target.name == word
                        || target.id.qualified_name.ends_with(&format!("::{}", word))
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

        // Look up in symbol table: collect all matches (same file first) to handle name collisions.
        let same_file: Vec<_> = state
            .symbol_table
            .iter()
            .filter(|e| e.name == word && e.uri == uri_norm)
            .collect();
        let other_files: Vec<_> = state
            .symbol_table
            .iter()
            .filter(|e| e.name == word && e.uri != uri_norm)
            .collect();
        let all_matches = if same_file.is_empty() {
            &other_files
        } else {
            &same_file
        };
        if let Some(entry) = all_matches.first() {
            let value = if all_matches.len() > 1 {
                let mut md = format!(
                    "**{}** — {} definitions (use Go to Definition to choose):\n\n",
                    word,
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

        if is_reserved_keyword(&word) {
            return Ok(None);
        }

        // 2.2: Try graph-based resolution via typing/specializes edges (works cross-file).
        if let Some(node) = state.semantic_graph.find_node_at_position(&uri_norm, pos) {
            for target in state
                .semantic_graph
                .outgoing_typing_or_specializes_targets(node)
            {
                if target.name == word || target.id.qualified_name.ends_with(&format!("::{}", word))
                {
                    return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                        uri: target.id.uri.clone(),
                        range: target.range,
                    })));
                }
            }
        }

        // Fall back to symbol table: collect all matches to handle name collisions (e.g. package and part def same name).
        let same_file: Vec<_> = state
            .symbol_table
            .iter()
            .filter(|e| e.name == word && e.uri == uri_norm)
            .map(|e| Location {
                uri: e.uri.clone(),
                range: e.range,
            })
            .collect();
        let other_files: Vec<_> = state
            .symbol_table
            .iter()
            .filter(|e| e.name == word && e.uri != uri_norm)
            .map(|e| Location {
                uri: e.uri.clone(),
                range: e.range,
            })
            .collect();
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
        let symbols = collect_document_symbols(doc);
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
        eprintln!(
            "[sysml-ls] librarySearch: query='{}' limit={} library_paths={} library_symbols={}",
            query,
            limit,
            state.library_paths.len(),
            library_symbol_count
        );

        let mut ranked: Vec<(i64, &crate::language::SymbolEntry)> = state
            .symbol_table
            .iter()
            .filter(|entry| util::uri_under_any_library(&entry.uri, &state.library_paths))
            .filter_map(|entry| {
                let score = if query.is_empty() {
                    1_000
                } else {
                    library_search_score(&entry.name, &query)?
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
        let items = ranked
            .into_iter()
            .take(limit)
            .map(|(score, entry)| dto::SysmlLibrarySearchItemDto {
                name: entry.name.clone(),
                kind: symbol_kind_label(entry.kind).to_string(),
                container: entry.container_name.clone(),
                uri: entry.uri.to_string(),
                range: dto::range_to_dto(entry.range),
                score,
                source: library_source_label(&entry.uri).to_string(),
                path: entry.uri.path().to_string(),
            })
            .collect();
        Ok(dto::SysmlLibrarySearchResultDto { items, total })
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

/// Run the Spec42 LSP server using the provided configuration.
pub async fn run(config: Arc<Spec42Config>, server_name: &str) {
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

fn symbol_kind_label(kind: SymbolKind) -> &'static str {
    match kind {
        SymbolKind::FILE => "file",
        SymbolKind::MODULE => "module",
        SymbolKind::NAMESPACE => "namespace",
        SymbolKind::PACKAGE => "package",
        SymbolKind::CLASS => "class",
        SymbolKind::METHOD => "method",
        SymbolKind::PROPERTY => "property",
        SymbolKind::FIELD => "field",
        SymbolKind::CONSTRUCTOR => "constructor",
        SymbolKind::ENUM => "enum",
        SymbolKind::INTERFACE => "interface",
        SymbolKind::FUNCTION => "function",
        SymbolKind::VARIABLE => "variable",
        SymbolKind::CONSTANT => "constant",
        SymbolKind::STRING => "string",
        SymbolKind::NUMBER => "number",
        SymbolKind::BOOLEAN => "boolean",
        SymbolKind::ARRAY => "array",
        SymbolKind::OBJECT => "object",
        SymbolKind::KEY => "key",
        SymbolKind::NULL => "null",
        SymbolKind::ENUM_MEMBER => "enumMember",
        SymbolKind::STRUCT => "struct",
        SymbolKind::EVENT => "event",
        SymbolKind::OPERATOR => "operator",
        SymbolKind::TYPE_PARAMETER => "typeParameter",
        _ => "symbol",
    }
}

fn library_source_label(uri: &Url) -> &'static str {
    let path = uri.path().to_ascii_lowercase();
    if path.contains("/standard-library/") {
        "standard"
    } else {
        "custom"
    }
}

fn library_search_score(name: &str, query_lc: &str) -> Option<i64> {
    let name_lc = name.to_ascii_lowercase();
    if name_lc == query_lc {
        return Some(10_000);
    }
    if name_lc.starts_with(query_lc) {
        return Some(8_000 - (name_lc.len() as i64));
    }
    if let Some(pos) = name_lc.find(query_lc) {
        return Some(6_000 - (pos as i64) * 10 - (name_lc.len() as i64));
    }
    fuzzy_subsequence_score(&name_lc, query_lc).map(|s| 4_000 + s)
}

fn fuzzy_subsequence_score(text: &str, query: &str) -> Option<i64> {
    if query.is_empty() {
        return Some(0);
    }
    let mut score: i64 = 0;
    let mut text_index = 0usize;
    let text_chars: Vec<char> = text.chars().collect();
    for ch in query.chars() {
        let mut found = None;
        for (idx, c) in text_chars.iter().enumerate().skip(text_index) {
            if *c == ch {
                found = Some(idx);
                break;
            }
        }
        let idx = found?;
        score += 100 - ((idx - text_index) as i64 * 3);
        text_index = idx + 1;
    }
    Some(score.max(0))
}
