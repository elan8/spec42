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
use crate::workspace::services::{ParsedScanEntry, RebuildAllDocumentLinksMetrics};
use crate::workspace::state::{IndexEntry, ServerState};

/// Outcome of `commit_startup_relink_or_stale`: whether the staged relink was committed, or
/// whether it was superseded by a newer edit while it was being built (caller should retry).
#[derive(Clone, Copy)]
pub(crate) enum StartupRelinkOutcome {
    Committed,
    Stale,
}

#[cfg(test)]
mod publication_tests {
    use super::*;
    use crate::workspace::state::{IndexEntry, ParseMetadata};

    fn entry(content: &str) -> IndexEntry {
        IndexEntry {
            content: content.to_owned(),
            parsed: None,
            parse_metadata: ParseMetadata::default(),
            admitted_to_publication: true,
        }
    }

    #[tokio::test]
    async fn live_rebuild_mirrors_only_the_authority_snapshot() {
        let mut state = ServerState::default();
        state.session.complete_startup();
        let uri = Url::parse("memory://workspace/model.sysml").unwrap();
        state
            .index
            .insert(uri.clone(), entry("package W { part p; }"));
        state.semantic_revision += 1;
        let handle = WorkspaceHandle::spawn(state);

        assert!(handle.rebuild_publication().await.unwrap());
        let snapshot = handle.snapshot();
        assert!(snapshot.publication_failure.is_none());
        let symbols = snapshot
            .published_model
            .model()
            .inspection()
            .document_symbols(uri.as_str());
        assert!(matches!(
            symbols,
            sysml_query::resolved_slice::QueryOutcome::Resolved(ref symbols)
                | sysml_query::resolved_slice::QueryOutcome::Recovered(ref symbols)
                | sysml_query::resolved_slice::QueryOutcome::UnsupportedWith(ref symbols)
                if !symbols.is_empty()
        ));
    }

    #[tokio::test]
    async fn authority_supersession_keeps_the_live_last_good_model() {
        let state = ServerState::default();
        let last_good = state.published_model.clone().into_model();
        let older = state
            .publication_authority
            .begin_build(
                &[sysml_query::source::SourceService::new()
                    .admit(
                        "memory://workspace/old.sysml",
                        "package Old;",
                        sysml_query::source::SourceKind::Workspace,
                    )
                    .unwrap()],
                [],
            )
            .await
            .unwrap();
        let newer = state
            .publication_authority
            .begin_build(
                &[sysml_query::source::SourceService::new()
                    .admit(
                        "memory://workspace/new.sysml",
                        "package New;",
                        sysml_query::source::SourceKind::Workspace,
                    )
                    .unwrap()],
                [],
            )
            .await
            .unwrap();

        assert_eq!(
            state
                .publication_authority
                .finish_build(older.construct())
                .await
                .unwrap()
                .outcome,
            workspace_session::SemanticPublicationOutcome::Stale
        );
        assert!(Arc::ptr_eq(
            &state.published_model.clone().into_model(),
            &last_good
        ));
        assert_eq!(
            state
                .publication_authority
                .finish_build(newer.construct())
                .await
                .unwrap()
                .outcome,
            workspace_session::SemanticPublicationOutcome::Published
        );
    }
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

    /// Runs canonical semantic construction outside the ServerState actor, then publishes only
    /// through SemanticPublicationAuthority and mirrors its immutable reader snapshot atomically.
    pub(crate) async fn rebuild_publication(&self) -> Result<bool, MutatePanicked> {
        let state = self.snapshot();
        let expected_revision = state.semantic_revision;
        let (documents, reported) = crate::workspace::state::publication_inputs(state.as_ref());
        let authority = Arc::clone(&state.publication_authority);
        let build = match authority.begin_build(&documents, reported).await {
            Ok(build) => build,
            Err(workspace_session::SemanticAuthorityBeginError::Construction(failure)) => {
                let failure = Arc::new(failure);
                tracing::error!(stage = ?failure.stage(), message = %failure.message(), "semantic publication preparation failed; retaining last good publication");
                self.actor
                    .mutate(move |state| state.publication_failure = Some(failure))
                    .await?;
                return Ok(false);
            }
            Err(workspace_session::SemanticAuthorityBeginError::Owner(error)) => return Err(error),
        };
        let completion = tokio::task::spawn_blocking(move || build.construct())
            .await
            .expect("semantic construction task panicked");
        let result = authority.finish_build(completion).await?;
        let published = result.outcome == workspace_session::SemanticPublicationOutcome::Published;
        let snapshot = published.then(|| authority.snapshot());
        let failure = result.failure.map(Arc::new);
        if let Some(failure) = failure.as_ref() {
            tracing::error!(stage = ?failure.stage(), message = %failure.message(), outcome = ?result.outcome, "semantic publication failed; retaining last good publication");
        }
        let mirrored = self
            .actor
            .mutate_if_changed(move |state| {
                if state.semantic_revision != expected_revision {
                    return Mutation::Unchanged(false);
                }
                state.publication_failure = failure;
                if let Some(snapshot) = snapshot {
                    state.published_model = snapshot;
                    crate::workspace::state::refresh_symbol_table_from_publication(state);
                    Mutation::Changed(true)
                } else {
                    Mutation::Changed(false)
                }
            })
            .await?
            .value;
        Ok(published && mirrored)
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
        new_symbols: Vec<SymbolEntry>,
    ) -> Result<StartupRelinkOutcome, MutatePanicked> {
        let outcome = self
            .actor
            .mutate_if_changed(move |s| {
                if !s.session.is_publication_current(&expected_publication) {
                    return Mutation::Unchanged(StartupRelinkOutcome::Stale);
                }
                s.symbol_table = new_symbols;
                s.semantic_revision = s.semantic_revision.wrapping_add(1);
                s.session.bump_version();
                Mutation::Changed(StartupRelinkOutcome::Committed)
            })
            .await
            .map(|outcome| outcome.value)?;
        if matches!(outcome, StartupRelinkOutcome::Committed) {
            self.rebuild_publication().await?;
        }
        Ok(outcome)
    }

    pub(crate) async fn fallback_full_rebuild(
        &self,
    ) -> Result<RebuildAllDocumentLinksMetrics, MutatePanicked> {
        let metrics = self
            .actor
            .mutate(|s| {
                let metrics = crate::workspace::services::rebuild_all_document_links(s);
                s.semantic_revision = s.semantic_revision.wrapping_add(1);
                s.session.bump_version();
                metrics
            })
            .await?;
        self.rebuild_publication().await?;
        Ok(metrics)
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

    /// Records that the editor opened or closed `uri`.
    ///
    /// Which documents are open decides which library files the publication reports diagnostics
    /// for, so this republishes: an opened library file has none until the model is rebuilt with
    /// it named. Returns whether the set changed.
    pub(crate) async fn set_document_open(
        &self,
        uri: Url,
        open: bool,
    ) -> Result<bool, MutatePanicked> {
        let library = self
            .actor
            .mutate_if_changed(move |s| {
                let changed = if open {
                    s.open_in_editor.insert(uri.clone())
                } else {
                    s.open_in_editor.remove(&uri)
                };
                if !changed {
                    return Mutation::Unchanged(false);
                }
                // Only a library file's reporting depends on this; a workspace document is
                // reported either way, so rebuilding for one would be pure cost.
                let library = crate::common::util::uri_under_any_library(&uri, &s.library_paths)
                    || crate::common::util::uri_under_any_library(&uri, &s.standard_library_paths);
                if library {
                    // Opening a configured library document promotes its indexed source from the
                    // search-only corpus into the admitted model. Provenance remains Library;
                    // only admission changes, and the canonical publication owns its semantics.
                    if open {
                        if let Some(entry) = s.index.get_mut(&uri) {
                            entry.admitted_to_publication = true;
                        }
                    }
                    s.semantic_revision = s.semantic_revision.wrapping_add(1);
                    s.session.bump_version();
                }
                Mutation::Changed(library)
            })
            .await
            .map(|outcome| outcome.value)?;
        if library {
            self.rebuild_publication().await?;
        }
        Ok(library)
    }

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
        parsed: sysml_resolution::syntax::SyntaxParse,
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
        new_symbols: Vec<SymbolEntry>,
    ) -> Result<bool, MutatePanicked> {
        let committed = self
            .actor
            .mutate_if_changed(move |s| {
                if s.session.commit_relink(&token) {
                    s.symbol_table = new_symbols;
                    s.semantic_revision = s.semantic_revision.wrapping_add(1);
                    Mutation::Changed(true)
                } else {
                    Mutation::Unchanged(false)
                }
            })
            .await
            .map(|outcome| outcome.value)?;
        if committed {
            self.rebuild_publication().await?;
        }
        Ok(committed)
    }

    // --- did_change_watched_files --------------------------------------------------------

    pub(crate) async fn refresh_document(
        &self,
        uri: Url,
        content: String,
    ) -> Result<Option<String>, MutatePanicked> {
        let outcome = self
            .actor
            .mutate_if_changed(move |s| {
                if s.index
                    .get(&uri)
                    .is_some_and(|entry| entry.content == content)
                {
                    return Mutation::Unchanged(None);
                }
                let warning = crate::workspace::services::refresh_document(s, &uri, content);
                s.semantic_revision = s.semantic_revision.wrapping_add(1);
                s.session.bump_version();
                Mutation::Changed(warning)
            })
            .await?;
        if outcome.published {
            self.rebuild_publication().await?;
        }
        Ok(outcome.value)
    }

    pub(crate) async fn remove_document(&self, uri: Url) -> Result<(), MutatePanicked> {
        let outcome = self
            .actor
            .mutate_if_changed(move |s| {
                if !s.index.contains_key(&uri) {
                    return Mutation::Unchanged(());
                }
                crate::workspace::services::remove_document(s, &uri);
                s.semantic_revision = s.semantic_revision.wrapping_add(1);
                s.session.bump_version();
                Mutation::Changed(())
            })
            .await?;
        if outcome.published {
            self.rebuild_publication().await?;
        }
        Ok(())
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
        let result = self
            .actor
            .mutate(move |s| {
                let ingest_results =
                    crate::workspace::services::ingest_parsed_scan_entries(s, entries);
                let relink_metrics = crate::workspace::services::rebuild_all_document_links(s);
                s.semantic_revision = s.semantic_revision.wrapping_add(1);
                s.session.complete_reindex();
                (ingest_results, relink_metrics)
            })
            .await?;
        self.rebuild_publication().await?;
        Ok(result)
    }

    // --- cache management ---------------------------------------------------------------

    /// Clears indexed documents, symbols, and the current immutable publication.
    pub(crate) async fn clear_cache_state(&self) -> Result<(usize, usize), MutatePanicked> {
        let counts = self
            .actor
            .mutate_if_changed(|s| {
                let counts = crate::lsp_runtime::custom::clear_document_store_state_full(s);
                s.semantic_revision = s.semantic_revision.wrapping_add(1);
                s.session.bump_version();
                Mutation::Changed(counts)
            })
            .await
            .map(|outcome| outcome.value)?;
        self.rebuild_publication().await?;
        Ok(counts)
    }
}
