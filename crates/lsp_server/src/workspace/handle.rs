//! `WorkspaceHandle` — a readability facade over `SessionActor<ServerState>` +
//! `SnapshotHandle<ServerState>`, giving `lsp_server`'s call sites one named method per
//! mutation-pipeline step instead of scattering raw `.mutate(|s| ...)` closures across
//! `documents.rs`/`mod.rs`. This is deliberately *not* a trait/interface for swappability —
//! `workspace_session` is already the right-sized abstraction, and with only one real consumer
//! (this crate) a second layer on top of it would be premature. See
//! `docs/engineering` design discussion for the reasoning.
//!
//! Standalone in this phase: not yet constructed/used by `Backend`, `documents.rs`, `mod.rs`,
//! or `diagnostics.rs` — that wiring is later phases of the same migration.

use std::collections::HashMap;
use std::sync::Arc;

use tower_lsp::lsp_types::{MessageType, TextDocumentContentChangeEvent, Url};
use workspace_session::{MutatePanicked, SessionActor, SnapshotHandle};

use crate::language::SymbolEntry;
use crate::semantic::SemanticGraph;
use crate::workspace::services::{ParsedScanEntry, RebuildAllDocumentLinksMetrics};
use crate::workspace::state::{IndexEntry, ServerState};

/// Outcome of `commit_startup_relink_or_stale`: whether the staged relink was committed, or
/// whether it was superseded by a newer edit while it was being built (caller should retry).
pub(crate) enum StartupRelinkOutcome {
    Committed,
    Stale,
}

#[derive(Clone)]
pub(crate) struct WorkspaceHandle {
    actor: SessionActor<ServerState>,
    snapshot: SnapshotHandle<ServerState>,
}

impl WorkspaceHandle {
    pub(crate) fn spawn(initial: ServerState) -> Self {
        let (actor, snapshot) = SessionActor::spawn(initial);
        Self { actor, snapshot }
    }

    /// Latest published snapshot. Non-blocking, never awaits — the whole point.
    pub(crate) fn snapshot(&self) -> Arc<ServerState> {
        self.snapshot.current()
    }

    /// A cloned handle for callers that want their own `wait_for` subscription (e.g. the
    /// `sysml/model` handler waiting for `Reindexing → Ready`).
    pub(crate) fn snapshot_handle(&self) -> SnapshotHandle<ServerState> {
        self.snapshot.clone()
    }

    // --- Startup ---------------------------------------------------------------------

    pub(crate) async fn set_startup_config(
        &self,
        roots: Vec<Url>,
        library_paths: Vec<Url>,
    ) -> Result<(), MutatePanicked> {
        self.actor
            .mutate(move |s| {
                s.workspace_roots = roots;
                s.library_paths = library_paths;
                s.session.reset();
            })
            .await
    }

    pub(crate) async fn begin_startup(&self) -> Result<(), MutatePanicked> {
        self.actor.mutate(|s| s.session.begin_startup()).await
    }

    pub(crate) async fn complete_startup(&self) -> Result<u64, MutatePanicked> {
        self.actor.mutate(|s| s.session.complete_startup()).await
    }

    pub(crate) async fn inject_cached_library_graph(
        &self,
        graph: SemanticGraph,
    ) -> Result<(), MutatePanicked> {
        self.actor
            .mutate(move |s| {
                s.semantic_graph = graph.clone();
                s.library_graph_snapshot = Some(graph);
                s.session.bump_version();
            })
            .await
    }

    pub(crate) async fn ingest_startup_scan(
        &self,
        entries: Vec<ParsedScanEntry>,
    ) -> Result<Vec<(Url, Option<String>)>, MutatePanicked> {
        self.actor
            .mutate(move |s| {
                let results = crate::workspace::services::ingest_parsed_scan_entries_batch(
                    s, entries,
                );
                s.session.bump_version();
                results
            })
            .await
    }

    /// Plain `Arc` read, no actor round-trip needed — snapshots `(version, index, library_paths)`
    /// for the caller to build a staged relink off of without holding anything.
    pub(crate) fn relink_snapshot(&self) -> (u64, HashMap<Url, IndexEntry>, Vec<Url>) {
        let snap = self.snapshot();
        (
            snap.session.version(),
            snap.index.clone(),
            snap.library_paths.clone(),
        )
    }

    pub(crate) async fn commit_startup_relink_or_stale(
        &self,
        expected_version: u64,
        new_graph: SemanticGraph,
        new_symbols: Vec<SymbolEntry>,
    ) -> Result<StartupRelinkOutcome, MutatePanicked> {
        self.actor
            .mutate(move |s| {
                if s.session.version() != expected_version {
                    return StartupRelinkOutcome::Stale;
                }
                s.semantic_graph = new_graph;
                s.symbol_table = new_symbols;
                s.session.bump_version();
                StartupRelinkOutcome::Committed
            })
            .await
    }

    pub(crate) async fn fallback_full_rebuild(
        &self,
    ) -> Result<RebuildAllDocumentLinksMetrics, MutatePanicked> {
        self.actor
            .mutate(|s| {
                let metrics = crate::workspace::services::rebuild_all_document_links(s);
                s.session.bump_version();
                metrics
            })
            .await
    }

    pub(crate) async fn index_library_paths_for_search(
        &self,
        library_paths: Vec<Url>,
    ) -> Result<usize, MutatePanicked> {
        self.actor
            .mutate(move |s| {
                crate::workspace::services::index_library_paths_for_search(s, &library_paths)
            })
            .await
    }

    pub(crate) async fn bump_version(&self) -> Result<u64, MutatePanicked> {
        self.actor.mutate(|s| s.session.bump_version()).await
    }

    // --- did_open / did_change ---------------------------------------------------------

    pub(crate) async fn store_document_text_fast(
        &self,
        uri: Url,
        text: String,
    ) -> Result<(Option<String>, Option<workspace_session::RelinkToken>), MutatePanicked> {
        self.actor
            .mutate(move |s| {
                let warning =
                    crate::workspace::services::store_document_text_fast(s, &uri, text);
                let can_relink = matches!(
                    s.session.lifecycle(),
                    workspace::SessionLifecycle::Ready | workspace::SessionLifecycle::Reindexing
                );
                let token = can_relink.then(|| s.session.schedule_relink());
                (warning, token)
            })
            .await
    }

    pub(crate) async fn apply_document_content_edit(
        &self,
        uri: Url,
        version: i32,
        changes: Vec<TextDocumentContentChangeEvent>,
    ) -> Result<(bool, Vec<(MessageType, String)>), MutatePanicked> {
        self.actor
            .mutate(move |s| {
                crate::workspace::services::apply_document_content_edit(s, &uri, version, changes)
            })
            .await
    }

    pub(crate) async fn apply_parsed_document_update(
        &self,
        uri: Url,
        version: i32,
        parsed: sysml_v2_parser::ParseResult,
        parse_time_ms: u32,
    ) -> Result<Vec<(MessageType, String)>, MutatePanicked> {
        self.actor
            .mutate(move |s| {
                crate::workspace::services::apply_parsed_document_update(
                    s,
                    &uri,
                    version,
                    parsed,
                    parse_time_ms,
                    false,
                )
            })
            .await
    }

    pub(crate) async fn schedule_relink_if_ready(
        &self,
    ) -> Result<Option<workspace_session::RelinkToken>, MutatePanicked> {
        self.actor
            .mutate(|s| {
                matches!(
                    s.session.lifecycle(),
                    workspace::SessionLifecycle::Ready | workspace::SessionLifecycle::Reindexing
                )
                .then(|| s.session.schedule_relink())
            })
            .await
    }

    /// Fire-and-forget: hands back an async relink's result. Merged in only if `token` is
    /// still current; otherwise dropped silently by the actor. Calls `session.commit_relink`
    /// to perform the actual `Reindexing -> Ready` lifecycle transition — without this, the
    /// session would stay stuck in `Reindexing` forever after any edit (the actor's own
    /// pre-merge `is_token_current` check only decides *whether* to merge, it doesn't
    /// transition the lifecycle by itself).
    pub(crate) fn report_relink_result(
        &self,
        token: workspace_session::RelinkToken,
        new_graph: SemanticGraph,
        new_symbols: Vec<SymbolEntry>,
    ) {
        self.actor.report_job_result(token, move |s| {
            if s.session.commit_relink(&token) {
                s.semantic_graph = new_graph;
                s.symbol_table = new_symbols;
            }
        });
    }

    // --- did_change_watched_files --------------------------------------------------------

    pub(crate) async fn refresh_document(
        &self,
        uri: Url,
        content: String,
    ) -> Result<Option<String>, MutatePanicked> {
        self.actor
            .mutate(move |s| {
                let warning = crate::workspace::services::refresh_document(s, &uri, content);
                s.session.bump_version();
                warning
            })
            .await
    }

    pub(crate) async fn remove_document(&self, uri: Url) -> Result<(), MutatePanicked> {
        self.actor
            .mutate(move |s| {
                crate::workspace::services::remove_document(s, &uri);
                s.session.bump_version();
            })
            .await
    }

    // --- did_change_configuration (library reindex) ---------------------------------------

    pub(crate) async fn begin_library_reindex_if_changed(
        &self,
        new_library_paths: Vec<Url>,
    ) -> Result<bool, MutatePanicked> {
        self.actor
            .mutate(move |s| {
                let old = std::mem::take(&mut s.library_paths);
                if new_library_paths == old {
                    s.library_paths = old;
                    false
                } else {
                    let _ = crate::workspace::services::clear_documents_under_roots(s, &old);
                    s.library_paths = new_library_paths.clone();
                    s.session.begin_library_reindex();
                    true
                }
            })
            .await
    }

    pub(crate) async fn complete_library_reindex(
        &self,
        entries: Vec<ParsedScanEntry>,
    ) -> Result<(Vec<(Url, Option<String>)>, RebuildAllDocumentLinksMetrics), MutatePanicked> {
        self.actor
            .mutate(move |s| {
                let ingest_results =
                    crate::workspace::services::ingest_parsed_scan_entries(s, entries);
                let relink_metrics = crate::workspace::services::rebuild_all_document_links(s);
                s.session.complete_reindex();
                (ingest_results, relink_metrics)
            })
            .await
    }

    // --- custom RPC methods (sysml/model, sysml/clearCache) -------------------------------

    pub(crate) async fn mark_parse_cached(&self, uri: Url) -> Result<(), MutatePanicked> {
        self.actor
            .mutate(move |s| crate::lsp_runtime::custom::mark_sysml_model_parse_cached(s, &uri))
            .await
    }

    /// Clears index, symbol table, semantic graph, and the actor-owned render cache.
    pub(crate) async fn clear_cache_state(&self) -> Result<(usize, usize), MutatePanicked> {
        self.actor
            .mutate(crate::lsp_runtime::custom::clear_document_store_state_full)
            .await
    }

    /// Commits a render-cache mutation only when `expected_version` still matches the live
    /// session. Returns `None` when a concurrent edit superseded the build.
    pub(crate) async fn update_render_cache<R: Send + 'static>(
        &self,
        expected_version: u64,
        apply: impl FnOnce(&mut workspace::ViewRenderCache) -> R + Send + 'static,
    ) -> Result<Option<R>, MutatePanicked> {
        self.actor
            .mutate(move |s| {
                if s.session.version() != expected_version {
                    return None;
                }
                Some(apply(&mut s.render_cache))
            })
            .await
    }
}
