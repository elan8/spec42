//! `WorkspaceHandle` — a readability facade over `SessionActor<ServerState>` +
//! `SnapshotHandle<ServerState>`, giving `lsp_server`'s call sites one named method per
//! mutation-pipeline step instead of scattering raw `.mutate(|s| ...)` closures across
//! `documents.rs`/`mod.rs`. This is deliberately *not* a trait/interface for swappability —
//! `session_actor` is already the right-sized abstraction, and with only one real consumer
//! (this crate) a second layer on top of it would be premature. See
//! `docs/engineering` design discussion for the reasoning.
//!
//! This is the publication boundary for the live workspace: document and configuration updates
//! enter through its named methods, and readers consume its immutable snapshots.

use std::collections::HashMap;
use std::sync::Arc;

use session_actor::{MutatePanicked, Mutation, SessionActor, SnapshotHandle};
use sysml_query::publication::{
    PublicationOutcome, PublicationToken, RelinkToken, SemanticBuild, SessionLifecycle,
};
use sysml_query::source::SourceDocument;
use sysml_query::syntax::ParsedSource;
use tower_lsp::lsp_types::{MessageType, TextDocumentContentChangeEvent, Url};

use crate::language::SymbolEntry;
use crate::session::services::{ParsedScanEntry, RebuildAllDocumentLinksMetrics};
use crate::session::state::{IndexEntry, ServerState};

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
    use crate::session::state::IndexEntry;

    fn entry(uri: &Url, content: &str) -> IndexEntry {
        IndexEntry::for_test(uri, content)
    }

    #[tokio::test]
    async fn live_rebuild_mirrors_only_the_authority_snapshot() {
        let mut state = ServerState::default();
        state.session.complete_startup();
        let uri = Url::parse("memory://workspace/model.sysml").unwrap();
        state
            .index
            .insert(uri.clone(), entry(&uri, "package W { part p; }"));
        state.semantic_revision += 1;
        let handle = WorkspaceHandle::spawn(state);

        assert!(handle.rebuild_publication().await.unwrap());
        let snapshot = handle.snapshot();
        assert!(snapshot.publication_failure.is_none());
        let symbols = snapshot
            .session
            .current()
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

    /// A document the host has admitted must never be missing from the publication that answers
    /// requests for it. The regression: `rebuild_publication` prepared its inputs off the actor
    /// and only *then* took a build token, so a rebuild that had already read a stale index could
    /// be handed a newer build generation than a concurrent rebuild carrying the new document —
    /// superseding the fresher build and leaving the document in `index` but out of
    /// `session.current()` forever. `textDocument/references` then answered empty for a document
    /// the server had accepted and published diagnostics for.
    ///
    /// This races a bare rebuild against a document refresh and asserts the invariant the fix
    /// establishes: however the two land, the document the host accepted is in the publication
    /// that answers requests for it. It is a convergence guard rather than a deterministic
    /// reproduction — reaching the inverted order requires the losing build to be prepared before
    /// and tokened after the winner, which preparing under the `begin_build` mutation makes
    /// unrepresentable but which no scheduling hint can force on demand.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn a_concurrent_rebuild_never_supersedes_the_build_that_carries_a_new_document() {
        let uri = Url::parse("memory://workspace/late.sysml").unwrap();
        for _ in 0..25 {
            let mut state = ServerState::default();
            state.session.complete_startup();
            let handle = WorkspaceHandle::spawn(state);

            let racer = handle.clone();
            let bare = tokio::spawn(async move { racer.rebuild_publication().await });
            let refresher = handle.clone();
            let refresh_uri = uri.clone();
            let refresh = tokio::spawn(async move {
                refresher
                    .refresh_document(refresh_uri, "package Late { part p; }".to_string())
                    .await
            });
            bare.await.unwrap().expect("bare rebuild");
            refresh.await.unwrap().expect("document refresh");

            let snapshot = handle.snapshot();
            assert!(
                snapshot.index.contains_key(&uri),
                "the host accepted the document"
            );
            let symbols = snapshot
                .session
                .current()
                .inspection()
                .document_symbols(uri.as_str());
            assert!(
                matches!(
                    symbols,
                    sysml_query::resolved_slice::QueryOutcome::Resolved(ref symbols)
                        | sysml_query::resolved_slice::QueryOutcome::Recovered(ref symbols)
                        | sysml_query::resolved_slice::QueryOutcome::UnsupportedWith(ref symbols)
                        if !symbols.is_empty()
                ),
                "an accepted document must be in the publication that answers requests for it, \
                 got {symbols:?}"
            );
        }
    }

    #[tokio::test]
    async fn a_superseded_build_keeps_the_live_last_good_model() {
        let mut state = ServerState::default();
        let last_good = Arc::clone(state.session.current());
        let source = &state.services.source;
        let older = state
            .services
            .publication
            .prepare(
                &[source
                    .admit(
                        "memory://workspace/old.sysml",
                        "package Old;",
                        sysml_query::source::SourceKind::Workspace,
                    )
                    .unwrap()],
                [],
            )
            .unwrap();
        let newer = state
            .services
            .publication
            .prepare(
                &[source
                    .admit(
                        "memory://workspace/new.sysml",
                        "package New;",
                        sysml_query::source::SourceKind::Workspace,
                    )
                    .unwrap()],
                [],
            )
            .unwrap();
        let older_token = state.session.begin_build(older.identity().clone());
        let newer_token = state.session.begin_build(newer.identity().clone());
        let publication = state.services.publication.clone();

        let older_done = publication.construct(SemanticBuild::new(older_token, older));
        assert_eq!(
            older_done.admit(&mut state.session),
            PublicationOutcome::Superseded
        );
        assert!(Arc::ptr_eq(state.session.current(), &last_good));
        let newer_done = publication.construct(SemanticBuild::new(newer_token, newer));
        assert_eq!(
            newer_done.admit(&mut state.session),
            PublicationOutcome::Published
        );
    }
}

pub(crate) struct PreparedDocumentEdit {
    pub(crate) uri: Url,
    pub(crate) version: i32,
    pub(crate) base_content: String,
    pub(crate) content: String,
}

impl PreparedDocumentEdit {
    /// Admit and parse prospective text through the host's services, off the actor.
    pub(crate) fn admit_text(
        uri: Url,
        content: &str,
        services: &sysml_query::Services,
    ) -> (SourceDocument, ParsedSource) {
        let document =
            services
                .source
                .admit_url(uri, content, sysml_query::source::SourceKind::Workspace);
        let parsed = services.syntax.parse(&document);
        (document, parsed)
    }
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

    /// Runs canonical semantic construction outside the ServerState actor and admits the result
    /// at the session's publication barrier, mirroring it into the reader snapshot atomically.
    ///
    /// Reading the inputs, preparing them, and taking the build token all happen inside **one**
    /// actor turn, and that is a correctness requirement, not a tidiness one. The session admits
    /// only the newest build generation (`Session::admit` supersedes every older token), so build
    /// generation order must equal input order: if a build could be prepared from one revision of
    /// the index and then be handed a *newer* generation than a concurrently running build that
    /// saw more documents, the fresher build would be superseded by the staler one and its
    /// documents would silently never enter the publication — a document present in `index` but
    /// absent from `session.current()`, which is exactly what makes `textDocument/references`
    /// answer empty for a document the host has already accepted. Preparing under the same
    /// mutation that calls `begin_build` makes that inversion unrepresentable.
    ///
    /// `prepare` never parses, so it is cheap enough to run on the actor; `construct` parses
    /// whatever the syntax memo does not hold and resolves the model on a blocking thread. A
    /// `Superseded` outcome is therefore always benign: the superseding build was prepared from
    /// inputs at least as new as this one's. After admission the memo is swept to the revisions
    /// the index still holds.
    pub(crate) async fn rebuild_publication(&self) -> Result<bool, MutatePanicked> {
        let staged = self
            .actor
            .mutate(move |state| {
                let (documents, reported) = crate::session::state::publication_inputs(state);
                match state.services.publication.prepare(&documents, reported) {
                    Ok(prepared) => {
                        let token = state.session.begin_build(prepared.identity().clone());
                        Ok(SemanticBuild::new(token, prepared))
                    }
                    Err(failure) => {
                        let failure = Arc::new(failure);
                        state.publication_failure = Some(Arc::clone(&failure));
                        Err(failure)
                    }
                }
            })
            .await?;
        let build = match staged {
            Ok(build) => build,
            Err(failure) => {
                tracing::error!(stage = ?failure.stage(), message = %failure.message(), "semantic publication preparation failed; retaining last good publication");
                return Ok(false);
            }
        };
        let publication = self.snapshot().services.publication.clone();
        let completion = tokio::task::spawn_blocking(move || publication.construct(build))
            .await
            .expect("semantic construction task panicked");
        let failure = completion.failure().cloned().map(Arc::new);
        let mirrored = self
            .actor
            .mutate_if_changed(move |state| {
                let outcome = completion.admit(&mut state.session);
                if let Some(failure) = failure.as_ref() {
                    tracing::error!(stage = ?failure.stage(), message = %failure.message(), ?outcome, "semantic publication failed; retaining last good publication");
                }
                state.publication_failure = failure;
                match outcome {
                    PublicationOutcome::Published => {
                        crate::session::state::refresh_symbol_table_from_publication(state);
                        state
                            .services
                            .syntax
                            .retain(state.index.values().map(|entry| entry.parsed.digest()));
                        Mutation::Changed(true)
                    }
                    PublicationOutcome::Superseded => Mutation::Unchanged(false),
                    PublicationOutcome::IdentityMismatch | PublicationOutcome::Failed => {
                        Mutation::Changed(false)
                    }
                }
            })
            .await?
            .value;
        Ok(mirrored)
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
                    crate::session::services::ingest_parsed_scan_entries_batch(s, entries);
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
                let metrics = crate::session::services::rebuild_all_document_links(s);
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
                    crate::session::services::index_library_paths_for_search(s, &library_paths);
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
    ) -> Result<(Option<String>, Option<RelinkToken>), MutatePanicked> {
        self.actor
            .mutate_if_changed(move |s| {
                if s.index
                    .get(&uri)
                    .is_some_and(|entry| entry.content() == text)
                {
                    return Mutation::Unchanged((None, None));
                }
                let warning = crate::session::services::store_document_text_fast(s, &uri, text);
                let can_relink = matches!(
                    s.session.lifecycle(),
                    SessionLifecycle::Ready | SessionLifecycle::Reindexing
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
        let base_content = entry.content().to_owned();
        let (content, changed, warnings) =
            crate::session::services::apply_content_changes(&base_content, &uri, version, changes);
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
        document: SourceDocument,
        parsed: ParsedSource,
        parse_time_ms: u32,
    ) -> Result<(Option<RelinkToken>, Vec<(MessageType, String)>), MutatePanicked> {
        self.actor
            .mutate_if_changed(move |s| {
                if s.index.get(&edit.uri).map(|entry| entry.content())
                    != Some(edit.base_content.as_str())
                {
                    return Mutation::Unchanged((None, Vec::new()));
                }
                let warnings = crate::session::services::apply_parsed_document_update(
                    s,
                    &edit.uri,
                    edit.version,
                    document,
                    parsed,
                    parse_time_ms,
                    false,
                );
                s.session.bump_version();
                let token = matches!(
                    s.session.lifecycle(),
                    SessionLifecycle::Ready | SessionLifecycle::Reindexing
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
    ) -> Result<Option<RelinkToken>, MutatePanicked> {
        self.actor
            .mutate_if_changed(|s| {
                if matches!(
                    s.session.lifecycle(),
                    SessionLifecycle::Ready | SessionLifecycle::Reindexing
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
        token: RelinkToken,
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
                    .is_some_and(|entry| entry.content() == content)
                {
                    return Mutation::Unchanged(None);
                }
                let warning = crate::session::services::refresh_document(s, &uri, content);
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
                crate::session::services::remove_document(s, &uri);
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
                    let _ = crate::session::services::clear_documents_under_roots(s, &old);
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
                    crate::session::services::ingest_parsed_scan_entries(s, entries);
                let relink_metrics = crate::session::services::rebuild_all_document_links(s);
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
