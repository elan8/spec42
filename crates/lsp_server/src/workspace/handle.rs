//! `WorkspaceHandle` — a readability facade over `SessionActor<ServerState>` +
//! `SnapshotHandle<ServerState>`, giving `lsp_server`'s call sites one named method per
//! mutation-pipeline step instead of scattering raw `.mutate(|s| ...)` closures across
//! `documents.rs`/`mod.rs`. This is deliberately *not* a trait/interface for swappability —
//! `workspace_session` is already the right-sized abstraction, and with only one real consumer
//! (this crate) a second layer on top of it would be premature. See
//! `docs/engineering` design discussion for the reasoning.
//!
//! This is the publication boundary for the live workspace: document and configuration updates
//! enter through its named methods, and readers consume its immutable snapshots.

use std::collections::HashMap;
use std::sync::Arc;

use tower_lsp::lsp_types::{MessageType, TextDocumentContentChangeEvent, Url};
use workspace_session::{MutatePanicked, Mutation, PublicationToken, SessionActor, SnapshotHandle};

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

pub(crate) struct PreparedDocumentEdit {
    pub(crate) uri: Url,
    pub(crate) version: i32,
    pub(crate) base_content: String,
    pub(crate) content: String,
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
        standard_library_paths: Vec<Url>,
    ) -> Result<(), MutatePanicked> {
        self.actor
            .mutate(move |s| {
                s.workspace_roots = roots;
                s.library_paths = library_paths;
                s.standard_library_paths = standard_library_paths;
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
                let results =
                    crate::workspace::services::ingest_parsed_scan_entries_batch(s, entries);
                s.session.bump_version();
                results
            })
            .await
    }

    /// Plain `Arc` read, no actor round-trip needed — snapshots `(publication, index, library_paths)`
    /// for the caller to build a staged relink off of without holding anything.
    pub(crate) fn relink_snapshot(
        &self,
    ) -> (
        PublicationToken,
        HashMap<Url, IndexEntry>,
        Vec<Url>,
        Vec<Url>,
    ) {
        let snap = self.snapshot();
        (
            snap.session.publication(),
            snap.index.clone(),
            snap.library_paths.clone(),
            snap.standard_library_paths.clone(),
        )
    }

    pub(crate) async fn commit_startup_relink_or_stale(
        &self,
        expected_publication: PublicationToken,
        new_graph: SemanticGraph,
        new_symbols: Vec<SymbolEntry>,
    ) -> Result<StartupRelinkOutcome, MutatePanicked> {
        self.actor
            .mutate_if_changed(move |s| {
                if !s.session.is_publication_current(&expected_publication) {
                    return Mutation::Unchanged(StartupRelinkOutcome::Stale);
                }
                s.semantic_graph = new_graph;
                s.symbol_table = new_symbols;
                s.session.bump_version();
                Mutation::Changed(StartupRelinkOutcome::Committed)
            })
            .await
            .map(|outcome| outcome.value)
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
            .mutate_if_changed(move |s| {
                let indexed =
                    crate::workspace::services::index_library_paths_for_search(s, &library_paths);
                if indexed == 0 {
                    Mutation::Unchanged(0)
                } else {
                    s.session.bump_version();
                    Mutation::Changed(indexed)
                }
            })
            .await
            .map(|outcome| outcome.value)
    }

    // --- did_open / did_change ---------------------------------------------------------

    pub(crate) async fn store_document_text_fast(
        &self,
        uri: Url,
        text: String,
    ) -> Result<(Option<String>, Option<workspace_session::RelinkToken>), MutatePanicked> {
        self.actor
            .mutate_if_changed(move |s| {
                if s.index.get(&uri).is_some_and(|entry| entry.content == text) {
                    return Mutation::Unchanged((None, None));
                }
                let warning = crate::workspace::services::store_document_text_fast(s, &uri, text);
                let can_relink = matches!(
                    s.session.lifecycle(),
                    workspace::SessionLifecycle::Ready | workspace::SessionLifecycle::Reindexing
                );
                let token = if can_relink {
                    Some(s.session.schedule_relink())
                } else {
                    s.session.bump_version();
                    None
                };
                Mutation::Changed((warning, token))
            })
            .await
            .map(|outcome| outcome.value)
    }

    pub(crate) async fn prepare_document_content_edit(
        &self,
        uri: Url,
        version: i32,
        changes: Vec<TextDocumentContentChangeEvent>,
    ) -> Result<(Option<PreparedDocumentEdit>, Vec<(MessageType, String)>), MutatePanicked> {
        let snapshot = self.snapshot();
        let Some(entry) = snapshot.index.get(&uri) else {
            return Ok((
                None,
                vec![(MessageType::WARNING, format!("didChange: document {uri} was not in the server index (version {version}). Change was ignored until a full open/watch refresh occurs."))],
            ));
        };
        let base_content = entry.content.clone();
        let (content, changed, warnings) = crate::workspace::services::apply_content_changes(
            &base_content,
            &uri,
            version,
            changes,
        );
        Ok((
            changed.then_some(PreparedDocumentEdit {
                uri,
                version,
                base_content,
                content,
            }),
            warnings,
        ))
    }

    pub(crate) async fn apply_parsed_document_update(
        &self,
        edit: PreparedDocumentEdit,
        parsed: sysml_v2_parser::ParseResult,
        parse_time_ms: u32,
    ) -> Result<
        (
            Option<workspace_session::RelinkToken>,
            Vec<(MessageType, String)>,
        ),
        MutatePanicked,
    > {
        self.actor
            .mutate_if_changed(move |s| {
                if s.index.get(&edit.uri).map(|entry| &entry.content) != Some(&edit.base_content) {
                    return Mutation::Unchanged((None, Vec::new()));
                }
                s.index
                    .get_mut(&edit.uri)
                    .expect("base-content check proved document exists")
                    .content = edit.content;
                let warnings = crate::workspace::services::apply_parsed_document_update(
                    s,
                    &edit.uri,
                    edit.version,
                    parsed,
                    parse_time_ms,
                    false,
                );
                s.session.bump_version();
                let token = matches!(
                    s.session.lifecycle(),
                    workspace::SessionLifecycle::Ready | workspace::SessionLifecycle::Reindexing
                )
                .then(|| s.session.schedule_relink());
                Mutation::Changed((token, warnings))
            })
            .await
            .map(|outcome| outcome.value)
    }

    #[cfg(test)]
    pub(crate) async fn schedule_relink_if_ready(
        &self,
    ) -> Result<Option<workspace_session::RelinkToken>, MutatePanicked> {
        self.actor
            .mutate_if_changed(|s| {
                if matches!(
                    s.session.lifecycle(),
                    workspace::SessionLifecycle::Ready | workspace::SessionLifecycle::Reindexing
                ) {
                    Mutation::Changed(Some(s.session.schedule_relink()))
                } else {
                    Mutation::Unchanged(None)
                }
            })
            .await
            .map(|outcome| outcome.value)
    }

    /// Commits an async relink's result via `mutate`, so the caller can `.await` it and be
    /// guaranteed the new graph/lifecycle are visible before doing anything else (e.g.
    /// publishing diagnostics) — `SessionActor::mutate` applies the closure and publishes to
    /// the `watch` channel *before* replying, unlike the old fire-and-forget
    /// `report_job_result` path this replaced. Calls `session.commit_relink` to perform the
    /// actual `Reindexing -> Ready` lifecycle transition — without this, the session would stay
    /// stuck in `Reindexing` forever after any edit. Returns whether the token was still
    /// current (`false` means a newer relink superseded this one and nothing was applied — the
    /// caller should skip any follow-up work tied to this result, since the superseding relink
    /// will produce its own).
    pub(crate) async fn report_relink_result(
        &self,
        token: workspace_session::RelinkToken,
        new_graph: SemanticGraph,
        new_symbols: Vec<SymbolEntry>,
    ) -> Result<bool, MutatePanicked> {
        self.actor
            .mutate_if_changed(move |s| {
                if s.session.commit_relink(&token) {
                    s.semantic_graph = new_graph;
                    s.symbol_table = new_symbols;
                    Mutation::Changed(true)
                } else {
                    Mutation::Unchanged(false)
                }
            })
            .await
            .map(|outcome| outcome.value)
    }

    /// Commits an evaluated graph only if `expected_publication` still matches the live session —
    /// i.e. no relink has landed since evaluation was kicked off. Unlike `report_relink_result`,
    /// this does NOT go through `RelinkToken`/`commit_relink`: evaluation never changes the
    /// session lifecycle (`Ready`/`Reindexing`), it's an orthogonal side-channel update to just
    /// `semantic_graph`, so an owner-scoped publication check (same pattern as
    /// `commit_startup_relink_or_stale`/`update_render_cache`) is the right primitive here, not
    /// the relink lifecycle machinery. Returns whether the commit applied, so the caller can
    /// skip republishing diagnostics for a discarded (superseded) evaluation.
    pub(crate) async fn report_evaluation_result(
        &self,
        expected_publication: PublicationToken,
        evaluated_graph: SemanticGraph,
    ) -> Result<bool, MutatePanicked> {
        self.actor
            .mutate_if_changed(move |s| {
                if !s.session.is_publication_current(&expected_publication) {
                    return Mutation::Unchanged(false);
                }
                s.semantic_graph = evaluated_graph;
                s.session.bump_version();
                Mutation::Changed(true)
            })
            .await
            .map(|outcome| outcome.value)
    }

    // --- did_change_watched_files --------------------------------------------------------

    pub(crate) async fn refresh_document(
        &self,
        uri: Url,
        content: String,
    ) -> Result<Option<String>, MutatePanicked> {
        self.actor
            .mutate_if_changed(move |s| {
                if s.index
                    .get(&uri)
                    .is_some_and(|entry| entry.content == content)
                {
                    return Mutation::Unchanged(None);
                }
                let warning = crate::workspace::services::refresh_document(s, &uri, content);
                s.session.bump_version();
                Mutation::Changed(warning)
            })
            .await
            .map(|outcome| outcome.value)
    }

    pub(crate) async fn remove_document(&self, uri: Url) -> Result<(), MutatePanicked> {
        self.actor
            .mutate_if_changed(move |s| {
                if !s.index.contains_key(&uri) {
                    return Mutation::Unchanged(());
                }
                crate::workspace::services::remove_document(s, &uri);
                s.session.bump_version();
                Mutation::Changed(())
            })
            .await
            .map(|outcome| outcome.value)
    }

    // --- did_change_configuration (library reindex) ---------------------------------------

    pub(crate) async fn begin_library_reindex_if_changed(
        &self,
        new_library_paths: Vec<Url>,
    ) -> Result<bool, MutatePanicked> {
        self.actor
            .mutate_if_changed(move |s| {
                let old = std::mem::take(&mut s.library_paths);
                if new_library_paths == old {
                    s.library_paths = old;
                    Mutation::Unchanged(false)
                } else {
                    let _ = crate::workspace::services::clear_documents_under_roots(s, &old);
                    s.library_paths = new_library_paths.clone();
                    s.session.begin_library_reindex();
                    Mutation::Changed(true)
                }
            })
            .await
            .map(|outcome| outcome.value)
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
            .mutate_if_changed(move |s| {
                if s.index
                    .get(&uri)
                    .is_none_or(|entry| entry.parse_metadata.parse_cached)
                {
                    return Mutation::Unchanged(());
                }
                crate::lsp_runtime::custom::mark_sysml_model_parse_cached(s, &uri);
                s.session.bump_version();
                Mutation::Changed(())
            })
            .await
            .map(|outcome| outcome.value)
    }

    /// Clears index, symbol table, semantic graph, and the actor-owned render cache.
    pub(crate) async fn clear_cache_state(&self) -> Result<(usize, usize), MutatePanicked> {
        self.actor
            .mutate_if_changed(|s| {
                let counts = crate::lsp_runtime::custom::clear_document_store_state_full(s);
                s.session.bump_version();
                Mutation::Changed(counts)
            })
            .await
            .map(|outcome| outcome.value)
    }

    /// Commits a render-cache mutation only when `expected_publication` still matches the live
    /// session. Returns `None` when a concurrent edit superseded the build.
    pub(crate) async fn update_render_cache<R: Send + 'static>(
        &self,
        expected_publication: PublicationToken,
        apply: impl FnOnce(&mut workspace::ViewRenderCache) -> R + Send + 'static,
    ) -> Result<Option<R>, MutatePanicked> {
        self.actor
            .mutate_if_changed(move |s| {
                if !s.session.is_publication_current(&expected_publication) {
                    return Mutation::Unchanged(None);
                }
                // Deliberately does not bump the session version: this cache entry is written
                // *for* the current publication version, so bumping here would make the entry
                // stale the instant it's written -- the next `cached_response` lookup would read
                // the bumped version and never match what was just cached. `Mutation::Changed`
                // alone already signals the actor that a real mutation happened.
                let value = apply(&mut s.render_cache);
                Mutation::Changed(Some(value))
            })
            .await
            .map(|outcome| outcome.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::state::{IndexEntry, ParseMetadata, ServerState};

    fn ready_state() -> ServerState {
        let mut state = ServerState::default();
        state.session.complete_startup();
        state
    }

    fn insert_document(state: &mut ServerState, uri: &Url, content: &str) {
        state.index.insert(
            uri.clone(),
            IndexEntry {
                content: content.to_string(),
                parsed: None,
                parse_metadata: ParseMetadata::default(),
                include_in_semantic_graph: true,
            },
        );
    }

    #[tokio::test]
    async fn same_document_content_is_a_true_publication_no_op() {
        let uri = Url::parse("file:///same.sysml").unwrap();
        let mut state = ready_state();
        insert_document(&mut state, &uri, "package Same;");
        let handle = WorkspaceHandle::spawn(state);
        let before = handle.snapshot();
        let publication = before.session.publication();

        let result = handle
            .store_document_text_fast(uri, "package Same;".to_string())
            .await
            .unwrap();

        assert_eq!(result, (None, None));
        let after = handle.snapshot();
        assert!(Arc::ptr_eq(&before, &after));
        assert_eq!(after.session.publication(), publication);
    }

    #[tokio::test]
    async fn changed_document_content_invalidates_older_publication() {
        let uri = Url::parse("file:///changed.sysml").unwrap();
        let mut state = ready_state();
        insert_document(&mut state, &uri, "package Before;");
        let handle = WorkspaceHandle::spawn(state);
        let before = handle.snapshot().session.publication();

        handle
            .refresh_document(uri.clone(), "package After;".to_string())
            .await
            .unwrap();

        let after = handle.snapshot();
        assert!(!after.session.is_publication_current(&before));
        assert_eq!(after.index[&uri].content, "package After;");
    }

    #[tokio::test]
    async fn document_edit_publishes_text_parse_and_relink_state_together() {
        let uri = Url::parse("file:///atomic.sysml").unwrap();
        let mut state = ready_state();
        insert_document(&mut state, &uri, "package Before;");
        crate::workspace::state::refresh_published_model(&mut state);
        let handle = WorkspaceHandle::spawn(state);

        let (edit, warnings) = handle
            .prepare_document_content_edit(
                uri.clone(),
                2,
                vec![TextDocumentContentChangeEvent {
                    range: None,
                    range_length: None,
                    text: "package After;".to_string(),
                }],
            )
            .await
            .unwrap();
        assert!(warnings.is_empty());
        let edit = edit.unwrap();

        // Preparing and parsing prospective text never leaks it through the reader snapshot.
        let before = handle.snapshot();
        assert_eq!(before.index[&uri].content, "package Before;");
        assert_eq!(
            before.session.lifecycle(),
            workspace::SessionLifecycle::Ready
        );

        let parsed = crate::common::util::parse_for_editor(&edit.content);
        let observer = {
            let handle = handle.clone();
            let uri = uri.clone();
            tokio::spawn(async move {
                for _ in 0..128 {
                    let snapshot = handle.snapshot();
                    match snapshot.index[&uri].content.as_str() {
                        "package Before;" => {
                            assert!(snapshot.index[&uri].parsed.is_none());
                            assert_eq!(
                                snapshot.session.lifecycle(),
                                workspace::SessionLifecycle::Ready
                            );
                            let outcome = snapshot
                                .published_model
                                .as_ref()
                                .unwrap()
                                .navigation()
                                .target_at(
                                    uri.as_str(),
                                    sysml_query::resolved_slice::TextPosition {
                                        line: 0,
                                        character: 9,
                                    },
                                );
                            assert!(matches!(
                                outcome,
                                sysml_query::resolved_slice::QueryOutcome::Resolved(target)
                                    if target.name.as_ref() == "Before"
                            ));
                        }
                        "package After;" => {
                            assert!(snapshot.index[&uri].parsed.is_some());
                            assert_eq!(
                                snapshot.session.lifecycle(),
                                workspace::SessionLifecycle::Reindexing
                            );
                            assert!(snapshot.semantic_graph.all_uris().contains(&uri));
                            let outcome = snapshot
                                .published_model
                                .as_ref()
                                .unwrap()
                                .navigation()
                                .target_at(
                                    uri.as_str(),
                                    sysml_query::resolved_slice::TextPosition {
                                        line: 0,
                                        character: 9,
                                    },
                                );
                            assert!(matches!(
                                outcome,
                                sysml_query::resolved_slice::QueryOutcome::Resolved(target)
                                    if target.name.as_ref() == "After"
                            ));
                        }
                        other => panic!("unexpected published document text: {other}"),
                    }
                    tokio::task::yield_now().await;
                }
            })
        };
        let (token, _) = handle
            .apply_parsed_document_update(edit, parsed, 1)
            .await
            .unwrap();
        assert!(token.is_some());

        let after = handle.snapshot();
        assert_eq!(after.index[&uri].content, "package After;");
        assert!(after.index[&uri].parsed.is_some());
        assert_eq!(
            after.session.lifecycle(),
            workspace::SessionLifecycle::Reindexing
        );
        observer.await.unwrap();
    }

    #[tokio::test]
    async fn superseded_parse_cannot_replace_the_current_semantic_publication() {
        let uri = Url::parse("file:///superseded-navigation.sysml").unwrap();
        let mut state = ready_state();
        insert_document(&mut state, &uri, "package Before;");
        crate::workspace::state::refresh_published_model(&mut state);
        let handle = WorkspaceHandle::spawn(state);

        let stale = PreparedDocumentEdit {
            uri: uri.clone(),
            version: 2,
            base_content: "package Before;".to_string(),
            content: "package Stale;".to_string(),
        };
        let current = PreparedDocumentEdit {
            uri: uri.clone(),
            version: 3,
            base_content: "package Before;".to_string(),
            content: "package Current;".to_string(),
        };
        handle
            .apply_parsed_document_update(
                current,
                crate::common::util::parse_for_editor("package Current;"),
                1,
            )
            .await
            .unwrap();
        let (token, warnings) = handle
            .apply_parsed_document_update(
                stale,
                crate::common::util::parse_for_editor("package Stale;"),
                1,
            )
            .await
            .unwrap();
        assert!(token.is_none());
        assert!(warnings.is_empty());

        let snapshot = handle.snapshot();
        assert_eq!(snapshot.index[&uri].content, "package Current;");
        let outcome = snapshot
            .published_model
            .as_ref()
            .unwrap()
            .navigation()
            .target_at(
                uri.as_str(),
                sysml_query::resolved_slice::TextPosition {
                    line: 0,
                    character: 9,
                },
            );
        assert!(matches!(
            outcome,
            sysml_query::resolved_slice::QueryOutcome::Resolved(target)
                if target.name.as_ref() == "Current"
        ));
    }

    #[tokio::test]
    async fn removing_a_document_invalidates_older_publication() {
        let uri = Url::parse("file:///removed.sysml").unwrap();
        let mut state = ready_state();
        insert_document(&mut state, &uri, "package Removed;");
        let handle = WorkspaceHandle::spawn(state);
        let before = handle.snapshot().session.publication();

        handle.remove_document(uri.clone()).await.unwrap();

        let after = handle.snapshot();
        assert!(!after.session.is_publication_current(&before));
        assert!(!after.index.contains_key(&uri));
    }

    #[tokio::test]
    async fn unchanged_library_paths_retain_the_published_snapshot() {
        let handle = WorkspaceHandle::spawn(ready_state());
        let before = handle.snapshot();
        let publication = before.session.publication();

        assert!(!handle
            .begin_library_reindex_if_changed(Vec::new())
            .await
            .unwrap());

        let after = handle.snapshot();
        assert!(Arc::ptr_eq(&before, &after));
        assert_eq!(after.session.publication(), publication);
    }

    #[tokio::test]
    async fn publication_from_another_handle_is_rejected() {
        let seed = ready_state();
        let first = WorkspaceHandle::spawn(seed.clone());
        let second = WorkspaceHandle::spawn(seed);
        let foreign = first.snapshot().session.publication();

        assert!(!second
            .report_evaluation_result(foreign, SemanticGraph::new())
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn out_of_order_relink_completion_discards_the_superseded_result() {
        let handle = WorkspaceHandle::spawn(ready_state());
        let stale = handle.schedule_relink_if_ready().await.unwrap().unwrap();
        let current = handle.schedule_relink_if_ready().await.unwrap().unwrap();

        assert!(!handle
            .report_relink_result(stale, SemanticGraph::new(), Vec::new())
            .await
            .unwrap());
        assert!(handle
            .report_relink_result(current, SemanticGraph::new(), Vec::new())
            .await
            .unwrap());
    }

    /// Track C: `report_evaluation_result` must commit when the session hasn't moved on since
    /// evaluation was kicked off, and must silently discard (not commit, not panic) when a
    /// newer relink/edit has bumped the version in the meantime — the same "supersede, don't
    /// block" discipline `report_relink_result`/`update_render_cache` already follow.
    #[tokio::test]
    async fn report_evaluation_result_commits_only_when_version_still_current() {
        let handle = WorkspaceHandle::spawn(ServerState::default());
        let expected_publication = handle.snapshot().session.publication();

        let evaluated_graph = SemanticGraph::new();
        let committed = handle
            .report_evaluation_result(expected_publication, evaluated_graph)
            .await
            .expect("actor mutate should not panic");
        assert!(committed, "matching version should commit");
    }

    #[tokio::test]
    async fn report_evaluation_result_discards_stale_version() {
        let handle = WorkspaceHandle::spawn(ServerState::default());
        let stale_publication = handle.snapshot().session.publication();

        // Bump the version, simulating a relink/edit that landed while evaluation was running.
        handle.complete_startup().await.expect("complete startup");

        let evaluated_graph = SemanticGraph::new();
        let committed = handle
            .report_evaluation_result(stale_publication, evaluated_graph)
            .await
            .expect("actor mutate should not panic");
        assert!(!committed, "stale version must not commit");
    }

    /// `report_relink_result` must commit (and the caller sees `Ok(true)`) when the token is
    /// still current — this is the path that must be synchronous-on-await so a subsequent
    /// `handle.snapshot()` (e.g. for diagnostics collection) never races ahead of the commit.
    #[tokio::test]
    async fn report_relink_result_commits_when_token_current() {
        let handle = WorkspaceHandle::spawn(ServerState::default());
        handle
            .complete_startup()
            .await
            .expect("actor mutate should not panic");
        let token = handle
            .schedule_relink_if_ready()
            .await
            .expect("actor mutate should not panic")
            .expect("fresh session should be ready to schedule a relink");

        let new_graph = SemanticGraph::new();
        let committed = handle
            .report_relink_result(token, new_graph, Vec::new())
            .await
            .expect("actor mutate should not panic");
        assert!(committed, "current token should commit");

        assert!(matches!(
            handle.snapshot().session.lifecycle(),
            workspace::SessionLifecycle::Ready
        ));
    }

    /// A superseded token (a newer relink scheduled after this one) must not commit — the
    /// caller uses this to skip publishing diagnostics for a discarded, stale relink result.
    #[tokio::test]
    async fn report_relink_result_discards_superseded_token() {
        let handle = WorkspaceHandle::spawn(ServerState::default());
        handle
            .complete_startup()
            .await
            .expect("actor mutate should not panic");
        let stale_token = handle
            .schedule_relink_if_ready()
            .await
            .expect("actor mutate should not panic")
            .expect("fresh session should be ready to schedule a relink");

        // Commit the stale token first so the session returns to `Ready`, then schedule a
        // second relink — this mints a newer token that supersedes any future commit attempt
        // using `stale_token`.
        let first_new_graph = SemanticGraph::new();
        handle
            .report_relink_result(stale_token, first_new_graph, Vec::new())
            .await
            .expect("actor mutate should not panic");
        handle
            .schedule_relink_if_ready()
            .await
            .expect("actor mutate should not panic")
            .expect("session should be ready to schedule another relink");

        let superseded_new_graph = SemanticGraph::new();
        let committed = handle
            .report_relink_result(stale_token, superseded_new_graph, Vec::new())
            .await
            .expect("actor mutate should not panic");
        assert!(!committed, "superseded token must not commit");
    }
}
