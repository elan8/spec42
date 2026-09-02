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

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use session_actor::{MutatePanicked, Mutation, SessionActor, SnapshotHandle};
use sysml_query::publication::{PublicationBuildFailure, PublicationOutcome, SemanticBuild};
use sysml_query::source::SourceDocument;
use sysml_query::syntax::ParsedSource;
use tower_lsp::lsp_types::{MessageType, TextDocumentContentChangeEvent, Url};

use crate::session::services::{ParsedScanEntry, RebuildAllDocumentLinksMetrics};
use crate::session::state::ServerState;

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

        let previous = handle.snapshot().session.publication();
        assert!(handle.rebuild_publication().await.unwrap());
        let snapshot = handle.snapshot();
        assert!(!snapshot.session.is_publication_current(&previous));
        assert!(snapshot.publication_failure.is_none());
        let symbols = snapshot
            .session
            .current()
            .inspection()
            .document_symbols(uri.as_str());
        assert!(matches!(
            symbols,
            sysml_query::resolved_slice::QueryOutcome {
                answer: sysml_query::resolved_slice::QueryAnswer::Resolved(ref symbols),
                ..
            }
                if !symbols.is_empty()
        ));
        assert_eq!(
            snapshot.semantic_symbols.len(),
            crate::language::symbol_entries_for_uri(snapshot.session.current(), &uri).len(),
            "the actor must publish the model and its semantic projection together"
        );
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
                    sysml_query::resolved_slice::QueryOutcome {
                        answer: sysml_query::resolved_slice::QueryAnswer::Resolved(ref symbols),
                        ..
                    }
                        if !symbols.is_empty()
                ),
                "an accepted document must be in the publication that answers requests for it, \
                 got {symbols:?}"
            );
            assert_eq!(
                snapshot
                    .semantic_symbols
                    .iter()
                    .filter(|symbol| symbol.uri == uri)
                    .count(),
                crate::language::symbol_entries_for_uri(snapshot.session.current(), &uri).len(),
                "an out-of-order completion must not split the model from its symbol projection"
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
    diagnostics_debounce_generation: Arc<AtomicU64>,
}

impl WorkspaceHandle {
    pub(crate) fn spawn(initial: ServerState) -> Self {
        let (actor, snapshot) = SessionActor::spawn(initial);
        Self {
            actor,
            snapshot,
            diagnostics_debounce_generation: Arc::new(AtomicU64::new(0)),
        }
    }

    pub(crate) fn next_diagnostics_debounce_generation(&self) -> u64 {
        self.diagnostics_debounce_generation
            .fetch_add(1, Ordering::SeqCst)
            + 1
    }

    pub(crate) fn is_current_diagnostics_debounce_generation(&self, generation: u64) -> bool {
        self.diagnostics_debounce_generation.load(Ordering::SeqCst) == generation
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
                let (documents, reported, standard_library_availability) =
                    crate::session::state::publication_inputs(state);
                match state
                    .services
                    .publication
                    .prepare_with_standard_library_availability(
                        &documents,
                        standard_library_availability,
                        reported,
                    ) {
                    Ok(prepared) => {
                        let token = state.session.begin_build(prepared.identity().clone());
                        Ok(SemanticBuild::new(token, prepared))
                    }
                    Err(failure) => {
                        state.session.finish_preparation_failure();
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
        let token = build.token().clone();
        let publication = self.snapshot().services.publication.clone();
        let completion = match tokio::task::spawn_blocking(move || publication.construct(build))
            .await
        {
            Ok(completion) => completion,
            Err(join_error) => {
                let failure = Arc::new(PublicationBuildFailure::construction_worker(join_error));
                let admitted_failure = Arc::clone(&failure);
                self.actor
                    .mutate_if_changed(move |state| {
                        let outcome = state.session.admit(
                            &token,
                            Err::<Arc<sysml_query::resolved_slice::PublishedModel>, _>(
                                (*admitted_failure).clone(),
                            ),
                        );
                        tracing::error!(stage = ?admitted_failure.stage(), message = %admitted_failure.message(), ?outcome, "semantic construction worker failed; retaining last good publication");
                        match outcome {
                            PublicationOutcome::Superseded => Mutation::Unchanged(false),
                            PublicationOutcome::IdentityMismatch | PublicationOutcome::Failed => {
                                state.publication_failure = Some(admitted_failure);
                                Mutation::Changed(false)
                            }
                            PublicationOutcome::Published => unreachable!("a failed build cannot publish"),
                        }
                    })
                    .await?;
                return Ok(false);
            }
        };
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
        standard_library_availability: sysml_query::StandardLibraryAvailability,
    ) -> Result<(), MutatePanicked> {
        self.actor
            .mutate(move |s| {
                s.workspace_roots = roots;
                s.library_paths = library_paths;
                s.standard_library_paths = standard_library_paths;
                s.standard_library_availability = standard_library_availability;
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
        mut entries: Vec<ParsedScanEntry>,
    ) -> Result<Vec<(Url, Option<String>)>, MutatePanicked> {
        self.actor
            .mutate(move |s| {
                // Editor text is authoritative for open documents. A startup or manifest-change
                // rescan may have read an older saved revision from disk while didOpen/didChange
                // was already published; never replace that revision at the scan barrier.
                entries.retain(|entry| {
                    let uri = crate::common::util::normalize_file_uri(&entry.uri);
                    !s.open_in_editor.contains(&uri)
                });
                let results =
                    crate::session::services::ingest_parsed_scan_entries_batch(s, entries);
                s.session.invalidate_inputs();
                results
            })
            .await
    }

    pub(crate) async fn adopt_open_documents(
        &self,
        documents: Vec<(Url, crate::session::state::IndexEntry)>,
    ) -> Result<(), MutatePanicked> {
        self.actor
            .mutate(move |state| {
                for (uri, entry) in documents {
                    state.open_in_editor.insert(uri.clone());
                    state.index.insert(uri, entry);
                }
                state.session.invalidate_inputs();
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
                    crate::session::services::index_library_paths_for_search(s, &library_paths);
                if indexed == 0 {
                    Mutation::Unchanged(0)
                } else {
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
                        s.recovery_search_symbols.retain(|symbol| symbol.uri != uri);
                    }
                    s.semantic_revision = s.semantic_revision.wrapping_add(1);
                    s.session.invalidate_inputs();
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
    ) -> Result<Option<String>, MutatePanicked> {
        self.actor
            .mutate_if_changed(move |s| {
                if s.index
                    .get(&uri)
                    .is_some_and(|entry| entry.content() == text)
                {
                    return Mutation::Unchanged(None);
                }
                let warning = crate::session::services::store_document_text_fast(s, &uri, text);
                s.session.invalidate_inputs();
                Mutation::Changed(warning)
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
    ) -> Result<Vec<(MessageType, String)>, MutatePanicked> {
        self.actor
            .mutate_if_changed(move |s| {
                if s.index.get(&edit.uri).map(|entry| entry.content())
                    != Some(edit.base_content.as_str())
                {
                    return Mutation::Unchanged(Vec::new());
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
                s.session.invalidate_inputs();
                Mutation::Changed(warnings)
            })
            .await
            .map(|outcome| outcome.value)
    }

    #[cfg(test)]
    pub(crate) async fn invalidate_semantic_inputs_if_ready(&self) -> Result<bool, MutatePanicked> {
        self.actor
            .mutate_if_changed(|s| {
                if matches!(
                    s.session.lifecycle(),
                    sysml_query::publication::SessionLifecycle::Ready
                        | sysml_query::publication::SessionLifecycle::Reindexing
                ) {
                    s.session.invalidate_inputs();
                    Mutation::Changed(true)
                } else {
                    Mutation::Unchanged(false)
                }
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
                s.session.invalidate_inputs();
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
                s.session.invalidate_inputs();
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
                    s.session.invalidate_inputs();
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
                let relink_metrics = RebuildAllDocumentLinksMetrics::default();
                s.semantic_revision = s.semantic_revision.wrapping_add(1);
                s.session.invalidate_inputs();
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
                s.session.invalidate_inputs();
                Mutation::Changed(counts)
            })
            .await
            .map(|outcome| outcome.value)?;
        self.rebuild_publication().await?;
        Ok(counts)
    }
}
