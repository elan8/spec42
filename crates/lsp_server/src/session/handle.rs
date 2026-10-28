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
        let report = handle.rebuild_publication_measured().await.unwrap();
        assert_eq!(report.outcome, PublicationOutcome::Published);
        let phases = report
            .phases
            .expect("a successful build reports complete owning-phase measurements");
        assert!(
            phases.resolution
                >= phases.resolve + phases.evaluation + phases.indexing + phases.diagnostics
        );
        assert!(report.construction >= phases.parse + phases.lowering + phases.resolution);
        assert!(phases.diagnostics >= phases.namespace_checks);
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

    #[tokio::test]
    async fn a_retained_failure_is_reported_once_until_a_publication_succeeds() {
        let mut state = ServerState::default();
        state.session.complete_startup();
        let uri = Url::parse("memory://workspace/model.sysml").unwrap();
        state
            .index
            .insert(uri.clone(), entry(&uri, "package W { part p; }"));
        state.semantic_revision += 1;
        state.publication_failure = Some(Arc::new(PublicationBuildFailure::construction_worker(
            "worker panicked",
        )));
        let handle = WorkspaceHandle::spawn(state);

        assert!(handle.unreported_publication_failure().is_some());
        assert!(
            handle.unreported_publication_failure().is_none(),
            "the same retained failure must not be announced again"
        );
        assert!(handle.rebuild_publication().await.unwrap());
        assert!(handle.unreported_publication_failure().is_none());
        assert!(
            handle
                .reported_publication_failure
                .lock()
                .unwrap()
                .is_none(),
            "a successful publication clears the record so a recurrence is reported"
        );
    }

    #[tokio::test]
    async fn watched_rename_rebuilds_one_publication_with_new_source_identity() {
        let old_uri = Url::parse("memory://workspace/old.sysml").unwrap();
        let new_uri = Url::parse("memory://workspace/new.sysml").unwrap();
        let mut state = ServerState::default();
        state.session.complete_startup();
        state.index.insert(
            old_uri.clone(),
            entry(&old_uri, "package Old { part def Part; }"),
        );
        let handle = WorkspaceHandle::spawn(state);
        handle.rebuild_publication().await.unwrap();
        let previous = handle.snapshot().session.publication();

        handle
            .apply_watched_file_changes(vec![
                (old_uri.clone(), None),
                (
                    new_uri.clone(),
                    Some("package New { part def Part; }".to_string()),
                ),
            ])
            .await
            .unwrap();

        let snapshot = handle.snapshot();
        assert!(!snapshot.index.contains_key(&old_uri));
        assert!(snapshot.index.contains_key(&new_uri));
        assert!(!snapshot.session.is_publication_current(&previous));
        assert!(matches!(
            snapshot
                .session
                .current()
                .inspection()
                .document_symbols(new_uri.as_str()),
            sysml_query::resolved_slice::QueryOutcome {
                answer: sysml_query::resolved_slice::QueryAnswer::Resolved(ref symbols),
                ..
            } if !symbols.is_empty()
        ));
    }

    #[tokio::test]
    async fn watched_save_echo_does_not_rebuild_publication() {
        let uri = Url::parse("memory://workspace/model.sysml").unwrap();
        let text = "package P { part def Product; }";
        let mut state = ServerState::default();
        state.session.complete_startup();
        state.index.insert(uri.clone(), entry(&uri, text));
        let handle = WorkspaceHandle::spawn(state);
        handle.rebuild_publication().await.unwrap();
        let previous = handle.snapshot().session.publication();

        handle
            .apply_watched_file_changes(vec![(uri, Some(text.to_string()))])
            .await
            .unwrap();

        assert!(handle.snapshot().session.is_publication_current(&previous));
    }

    #[tokio::test]
    async fn watched_delete_create_same_uri_preserves_file() {
        let uri = Url::parse("memory://workspace/model.sysml").unwrap();
        let text = "package P { part def Product; }";
        let mut state = ServerState::default();
        state.session.complete_startup();
        state.index.insert(uri.clone(), entry(&uri, text));
        let handle = WorkspaceHandle::spawn(state);
        handle.rebuild_publication().await.unwrap();

        handle
            .apply_watched_file_changes(vec![
                (uri.clone(), None),
                (uri.clone(), Some(text.to_string())),
            ])
            .await
            .unwrap();

        let snapshot = handle.snapshot();
        assert_eq!(
            snapshot.index.get(&uri).map(|entry| entry.content()),
            Some(text)
        );
        assert!(matches!(
            snapshot
                .session
                .current()
                .inspection()
                .document_symbols(uri.as_str()),
            sysml_query::resolved_slice::QueryOutcome {
                answer: sysml_query::resolved_slice::QueryAnswer::Resolved(ref symbols),
                ..
            } if !symbols.is_empty()
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
                    .apply_watched_file_changes(vec![(
                        refresh_uri,
                        Some("package Late { part p; }".to_string()),
                    )])
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

#[derive(Debug)]
pub(crate) struct PreparedDocumentEdit {
    pub(crate) uri: Url,
    pub(crate) version: i32,
    pub(crate) base_content: String,
    pub(crate) content: String,
    pub(crate) source_kind: sysml_query::source::SourceKind,
    editor_revision: super::editor_documents::EditorRevision,
}

#[cfg(test)]
mod editor_update_tests {
    use super::*;
    use crate::session::editor_documents::EditorEditRejection;

    fn opened(source: &str) -> (WorkspaceHandle, Url) {
        let uri = Url::parse("memory://editor/model.sysml").unwrap();
        let mut state = ServerState::default();
        state.session.complete_startup();
        state.index.insert(
            uri.clone(),
            crate::session::state::IndexEntry::for_test(&uri, source),
        );
        state.index.get_mut(&uri).unwrap().open_editor(Some(1));
        (WorkspaceHandle::spawn(state), uri)
    }

    async fn edit(
        handle: &WorkspaceHandle,
        uri: &Url,
        version: i32,
        text: &str,
    ) -> PreparedDocumentEdit {
        let (prepared, warnings) = handle
            .prepare_document_content_edit(
                uri.clone(),
                version,
                vec![TextDocumentContentChangeEvent {
                    range: None,
                    range_length: None,
                    text: text.to_owned(),
                }],
            )
            .await
            .unwrap();
        assert!(warnings.is_empty());
        let DocumentChangePreparation::Changed(edit) = prepared else {
            panic!("expected a changed source")
        };
        edit
    }

    async fn apply(handle: &WorkspaceHandle, edit: PreparedDocumentEdit) -> ParsedDocumentUpdate {
        let (document, parsed) = PreparedDocumentEdit::admit_text(
            edit.uri.clone(),
            &edit.content,
            edit.source_kind,
            &handle.snapshot().services,
        );
        handle
            .apply_parsed_document_update(edit, document, parsed, 0)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn no_op_advances_version_and_rejects_stale_edits_without_republishing() {
        let source = "package P { part def A; }";
        let (handle, uri) = opened(source);
        handle.rebuild_publication().await.unwrap();
        let publication = handle.snapshot().session.publication();
        let (prepared, _) = handle
            .prepare_document_content_edit(
                uri.clone(),
                4,
                vec![TextDocumentContentChangeEvent {
                    range: None,
                    range_length: None,
                    text: source.to_owned(),
                }],
            )
            .await
            .unwrap();
        assert!(matches!(prepared, DocumentChangePreparation::Unchanged));
        assert_eq!(handle.snapshot().index[&uri].editor_version(), Some(4));
        let (stale, _) = handle
            .prepare_document_content_edit(
                uri.clone(),
                3,
                vec![TextDocumentContentChangeEvent {
                    range: None,
                    range_length: None,
                    text: "package Stale;".to_owned(),
                }],
            )
            .await
            .unwrap();
        assert!(matches!(
            stale,
            DocumentChangePreparation::Rejected(EditorEditRejection::StaleVersion {
                current: 4,
                received: 3
            })
        ));
        let snapshot = handle.snapshot();
        assert_eq!(snapshot.index[&uri].content(), source);
        assert!(snapshot.session.is_publication_current(&publication));
    }

    #[tokio::test]
    async fn parsed_updates_reject_out_of_order_completion_and_old_open_intervals() {
        let (handle, uri) = opened("package Initial;");
        let older = edit(&handle, &uri, 2, "package Older;").await;
        let newer = edit(&handle, &uri, 3, "package Newer;").await;
        assert!(matches!(
            apply(&handle, newer).await,
            ParsedDocumentUpdate::Applied(_)
        ));
        assert!(matches!(
            apply(&handle, older).await,
            ParsedDocumentUpdate::Superseded
        ));
        let old_open = edit(&handle, &uri, 4, "package OldOpen;").await;
        handle.set_document_open(uri.clone(), false).await.unwrap();
        handle
            .set_editor_document_open(uri.clone(), 1)
            .await
            .unwrap();
        assert!(matches!(
            apply(&handle, old_open).await,
            ParsedDocumentUpdate::Superseded
        ));
        let snapshot = handle.snapshot();
        assert_eq!(snapshot.index[&uri].content(), "package Newer;");
        assert_eq!(snapshot.index[&uri].editor_version(), Some(1));
    }

    #[tokio::test]
    async fn parsing_an_edit_preserves_its_admitted_source_kind() {
        let (handle, uri) = opened("package Initial;");
        handle
            .actor
            .mutate({
                let uri = uri.clone();
                move |state| {
                    let document = state.services.source.admit_url(
                        uri.clone(),
                        "package Initial;",
                        sysml_query::source::SourceKind::Library,
                    );
                    let parsed = state.services.syntax.parse(&document);
                    let entry = state.index.get_mut(&uri).unwrap();
                    entry.document = document;
                    entry.parsed = parsed;
                }
            })
            .await
            .unwrap();
        let prepared = edit(&handle, &uri, 2, "package Changed;").await;
        assert_eq!(
            prepared.source_kind,
            sysml_query::source::SourceKind::Library
        );
        assert!(matches!(
            apply(&handle, prepared).await,
            ParsedDocumentUpdate::Applied(_)
        ));
        assert_eq!(
            handle.snapshot().index[&uri].document.kind(),
            sysml_query::source::SourceKind::Library
        );
    }

    #[tokio::test]
    async fn newer_no_op_supersedes_a_parse_even_when_base_bytes_match() {
        let source = "package Initial;";
        let (handle, uri) = opened(source);
        let pending = edit(&handle, &uri, 2, "package Stale;").await;
        let (preparation, _) = handle
            .prepare_document_content_edit(
                uri.clone(),
                3,
                vec![TextDocumentContentChangeEvent {
                    range: None,
                    range_length: None,
                    text: source.to_owned(),
                }],
            )
            .await
            .unwrap();
        assert!(matches!(preparation, DocumentChangePreparation::Unchanged));
        assert!(matches!(
            apply(&handle, pending).await,
            ParsedDocumentUpdate::Superseded
        ));
        let snapshot = handle.snapshot();
        assert_eq!(snapshot.index[&uri].content(), source);
        assert_eq!(snapshot.index[&uri].editor_version(), Some(3));
    }

    #[tokio::test]
    async fn opening_new_source_records_membership_and_version_atomically() {
        let handle = WorkspaceHandle::spawn(ServerState::default());
        let uri = Url::parse("memory://editor/new.sysml").unwrap();
        handle
            .store_document_text_fast(uri.clone(), "package Unsaved;".into(), Some(9))
            .await
            .unwrap();
        let snapshot = handle.snapshot();
        assert!(snapshot.is_editor_open(&uri));
        assert_eq!(snapshot.index[&uri].content(), "package Unsaved;");
        assert_eq!(snapshot.index[&uri].editor_version(), Some(9));
    }
}

#[derive(Debug)]
pub(crate) enum DocumentChangePreparation {
    Changed(PreparedDocumentEdit),
    Unchanged,
    Rejected(super::editor_documents::EditorEditRejection),
}

pub(crate) enum ParsedDocumentUpdate {
    Applied(Vec<(MessageType, String)>),
    Superseded,
}

/// Host boundary costs plus the canonical phase owner's complete construction measurements.
/// Timings are observations only; they do not participate in publication identity or admission.
pub(crate) struct PublicationRebuildReport {
    pub(crate) outcome: PublicationOutcome,
    pub(crate) preparation: std::time::Duration,
    pub(crate) preparation_queue: std::time::Duration,
    pub(crate) construction: std::time::Duration,
    pub(crate) admission: std::time::Duration,
    pub(crate) phases: Option<sysml_query::resolved_slice::BuildMeasurements>,
}

impl PreparedDocumentEdit {
    /// Admit and parse prospective text through the host's services, off the actor.
    pub(crate) fn admit_text(
        uri: Url,
        content: &str,
        kind: sysml_query::source::SourceKind,
        services: &sysml_query::Services,
    ) -> (SourceDocument, ParsedSource) {
        let document = services.source.admit_url(uri, content, kind);
        let parsed = services.syntax.parse(&document);
        (document, parsed)
    }
}

#[derive(Clone)]
pub(crate) struct WorkspaceHandle {
    identity: Arc<()>,
    actor: SessionActor<ServerState>,
    snapshot: SnapshotHandle<ServerState>,
    diagnostics_debounce_generation: Arc<AtomicU64>,
    /// The retained failure last shown to the user, so one failure is announced once.
    reported_publication_failure: Arc<std::sync::Mutex<Option<String>>>,
}

impl WorkspaceHandle {
    pub(crate) fn same_session(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.identity, &other.identity)
    }

    pub(crate) fn spawn(initial: ServerState) -> Self {
        let (actor, snapshot) = SessionActor::spawn(initial);
        Self {
            identity: Arc::new(()),
            actor,
            snapshot,
            diagnostics_debounce_generation: Arc::new(AtomicU64::new(0)),
            reported_publication_failure: Arc::default(),
        }
    }

    /// The retained publication failure, unless this same failure was already returned.
    ///
    /// Later edits that fail identically, or are superseded, keep the earlier report instead of
    /// repeating it. A successful publication clears the record so a recurrence is reported again.
    pub(crate) fn unreported_publication_failure(&self) -> Option<Arc<PublicationBuildFailure>> {
        let failure = self.snapshot().publication_failure.clone();
        let mut reported = self
            .reported_publication_failure
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let key = failure.as_ref().map(|failure| failure.to_string());
        if *reported == key {
            return None;
        }
        *reported = key;
        failure
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
        self.rebuild_publication_measured()
            .await
            .map(|report| report.outcome == PublicationOutcome::Published)
    }

    pub(crate) async fn rebuild_publication_measured(
        &self,
    ) -> Result<PublicationRebuildReport, MutatePanicked> {
        let preparation_started = std::time::Instant::now();
        let (staged, preparation) = self
            .actor
            .mutate(move |state| {
                let started = std::time::Instant::now();
                let (documents, reported, standard_library_availability) =
                    crate::session::state::publication_inputs(state);
                let result = match state
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
                };
                (result, started.elapsed())
            })
            .await?;
        let preparation_queue = preparation_started.elapsed().saturating_sub(preparation);
        let mut report = PublicationRebuildReport {
            outcome: PublicationOutcome::Failed,
            preparation,
            preparation_queue,
            construction: std::time::Duration::ZERO,
            admission: std::time::Duration::ZERO,
            phases: None,
        };
        let build = match staged {
            Ok(build) => build,
            Err(failure) => {
                tracing::error!(stage = ?failure.stage(), message = %failure.message(), "semantic publication preparation failed; retaining last good publication");
                return Ok(report);
            }
        };
        let token = build.token().clone();
        let publication = self.snapshot().services.publication.clone();
        let construction_started = std::time::Instant::now();
        let completion = match tokio::task::spawn_blocking(move || publication.construct(build))
            .await
        {
            Ok(completion) => completion,
            Err(join_error) => {
                let failure = Arc::new(PublicationBuildFailure::construction_worker(join_error));
                let admitted_failure = Arc::clone(&failure);
                report.construction = construction_started.elapsed();
                let admission_started = std::time::Instant::now();
                report.outcome = self.actor
                    .mutate_if_changed(move |state| {
                        let outcome = state.session.admit(
                            &token,
                            Err::<Arc<sysml_query::resolved_slice::PublishedModel>, _>(
                                (*admitted_failure).clone(),
                            ),
                        );
                        tracing::error!(stage = ?admitted_failure.stage(), message = %admitted_failure.message(), ?outcome, "semantic construction worker failed; retaining last good publication");
                        match outcome {
                            PublicationOutcome::Superseded => Mutation::Unchanged(outcome),
                            PublicationOutcome::IdentityMismatch | PublicationOutcome::Failed => {
                                state.publication_failure = Some(admitted_failure);
                                Mutation::Changed(outcome)
                            }
                            PublicationOutcome::Published => unreachable!("a failed build cannot publish"),
                        }
                    })
                    .await?.value;
                report.admission = admission_started.elapsed();
                return Ok(report);
            }
        };
        report.construction = construction_started.elapsed();
        report.phases = completion.measurements().copied();
        let failure = completion.failure().cloned().map(Arc::new);
        let admission_started = std::time::Instant::now();
        let mirrored = self
            .actor
            .mutate_if_changed(move |state| {
                let outcome = completion.admit(&mut state.session);
                if let Some(failure) = failure.as_ref() {
                    tracing::error!(stage = ?failure.stage(), message = %failure.message(), ?outcome, "semantic publication failed; retaining last good publication");
                }
                match outcome {
                    PublicationOutcome::Published => {
                        state.publication_failure = None;
                        crate::session::state::refresh_symbol_table_from_publication(state);
                        state
                            .services
                            .syntax
                            .retain(state.index.values().map(|entry| entry.parsed.digest()));
                        Mutation::Changed(outcome)
                    }
                    PublicationOutcome::Superseded => Mutation::Unchanged(outcome),
                    PublicationOutcome::IdentityMismatch => {
                        state.publication_failure = Some(Arc::new(PublicationBuildFailure::admission_identity_mismatch()));
                        Mutation::Changed(outcome)
                    }
                    PublicationOutcome::Failed => {
                        state.publication_failure = failure;
                        Mutation::Changed(outcome)
                    }
                }
            })
            .await?
            .value;
        report.outcome = mirrored;
        report.admission = admission_started.elapsed();
        Ok(report)
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
                s.diagnostic_readiness.reset(true);
            })
            .await
    }

    pub(crate) async fn begin_startup(&self) -> Result<(), MutatePanicked> {
        self.actor.mutate(|s| s.session.begin_startup()).await
    }

    pub(crate) async fn complete_startup(&self) -> Result<u64, MutatePanicked> {
        self.actor.mutate(|s| s.session.complete_startup()).await
    }

    pub(crate) async fn announce_readiness(
        &self,
        token: super::readiness::ReadinessToken,
    ) -> Result<bool, MutatePanicked> {
        self.actor
            .mutate_if_changed(move |s| {
                if s.diagnostic_readiness.announce(&token) {
                    Mutation::Changed(true)
                } else {
                    Mutation::Unchanged(false)
                }
            })
            .await
            .map(|outcome| outcome.value)
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
                    !s.is_editor_open(&uri)
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
                for (uri, mut entry) in documents {
                    entry.open_editor(None);
                    entry.rekey_editor_owner();
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
        self.set_document_presence(uri, open, None).await
    }

    pub(crate) async fn set_editor_document_open(
        &self,
        uri: Url,
        version: i32,
    ) -> Result<bool, MutatePanicked> {
        self.set_document_presence(uri, true, Some(version)).await
    }

    async fn set_document_presence(
        &self,
        uri: Url,
        open: bool,
        version: Option<i32>,
    ) -> Result<bool, MutatePanicked> {
        let library = self
            .actor
            .mutate_if_changed(move |s| {
                let Some(entry) = s.index.get_mut(&uri) else {
                    return Mutation::Unchanged(false);
                };
                let changed = if open {
                    entry.open_editor(version)
                } else {
                    entry.close_editor()
                };
                if !changed {
                    if open && version.is_some() {
                        return Mutation::Changed(false);
                    }
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

    /// Store admitted source, optionally establishing an editor open interval atomically.
    pub(crate) async fn store_document_text_fast(
        &self,
        uri: Url,
        text: String,
        editor_version: Option<i32>,
    ) -> Result<Option<String>, MutatePanicked> {
        self.actor
            .mutate_if_changed(move |state| {
                if editor_version.is_none()
                    && state
                        .index
                        .get(&uri)
                        .is_some_and(|entry| entry.content() == text)
                {
                    return Mutation::Unchanged(None);
                }
                let warning = crate::session::services::store_document_text_fast(state, &uri, text);
                if let Some(version) = editor_version {
                    state
                        .index
                        .get_mut(&uri)
                        .expect("stored source")
                        .open_editor(Some(version));
                    state
                        .recovery_search_symbols
                        .retain(|symbol| symbol.uri != uri);
                }
                state.semantic_revision = state.semantic_revision.wrapping_add(1);
                state.session.invalidate_inputs();
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
    ) -> Result<(DocumentChangePreparation, Vec<(MessageType, String)>), MutatePanicked> {
        use super::editor_documents::EditorEditRejection;
        self.actor
            .mutate_if_changed(move |state| {
                let Some(entry) = state.index.get(&uri) else {
                    return Mutation::Unchanged((
                        DocumentChangePreparation::Rejected(EditorEditRejection::NotIndexed),
                        Vec::new(),
                    ));
                };
                let editor_revision = match entry
                    .editor_revision
                    .as_ref()
                    .ok_or(EditorEditRejection::NotOpen)
                    .and_then(|revision| revision.prepare(version))
                {
                    Ok(revision) => revision,
                    Err(reason) => {
                        return Mutation::Unchanged((
                            DocumentChangePreparation::Rejected(reason),
                            Vec::new(),
                        ))
                    }
                };
                let base_content = entry.content().to_owned();
                let source_kind = entry.document.kind();
                let (content, changed, warnings) = crate::session::services::apply_content_changes(
                    &base_content,
                    &uri,
                    version,
                    changes,
                );
                if changed {
                    Mutation::Unchanged((
                        DocumentChangePreparation::Changed(PreparedDocumentEdit {
                            uri,
                            version,
                            base_content,
                            content,
                            source_kind,
                            editor_revision,
                        }),
                        warnings,
                    ))
                } else {
                    match state
                        .index
                        .get_mut(&uri)
                        .and_then(|entry| entry.editor_revision.as_mut())
                        .ok_or(EditorEditRejection::NotOpen)
                        .and_then(|revision| revision.advance(&editor_revision, version))
                    {
                        Ok(()) => {
                            Mutation::Changed((DocumentChangePreparation::Unchanged, warnings))
                        }
                        Err(reason) => Mutation::Unchanged((
                            DocumentChangePreparation::Rejected(reason),
                            warnings,
                        )),
                    }
                }
            })
            .await
            .map(|outcome| outcome.value)
    }

    pub(crate) async fn apply_parsed_document_update(
        &self,
        edit: PreparedDocumentEdit,
        document: SourceDocument,
        parsed: ParsedSource,
        parse_time_ms: u32,
    ) -> Result<ParsedDocumentUpdate, MutatePanicked> {
        self.actor
            .mutate_if_changed(move |s| {
                if s.index.get(&edit.uri).map(|entry| entry.content())
                    != Some(edit.base_content.as_str())
                    || s.index.get(&edit.uri).map(|entry| entry.document.kind())
                        != Some(edit.source_kind)
                {
                    return Mutation::Unchanged(ParsedDocumentUpdate::Superseded);
                }
                if s.index
                    .get_mut(&edit.uri)
                    .and_then(|entry| entry.editor_revision.as_mut())
                    .is_none_or(|revision| {
                        revision
                            .advance(&edit.editor_revision, edit.version)
                            .is_err()
                    })
                {
                    return Mutation::Unchanged(ParsedDocumentUpdate::Superseded);
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
                Mutation::Changed(ParsedDocumentUpdate::Applied(warnings))
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

    /// Applies one watched-file notification as a single source revision and rebuilds once.
    /// Rename notifications commonly contain a delete and a create in the same batch.
    pub(crate) async fn apply_watched_file_changes(
        &self,
        changes: Vec<(Url, Option<String>)>,
    ) -> Result<(bool, Vec<String>), MutatePanicked> {
        let outcome = self
            .actor
            .mutate_if_changed(move |state| {
                let mut warnings = Vec::new();
                let mut changed = false;
                for (uri, content) in changes {
                    match content {
                        Some(content) => {
                            if state
                                .index
                                .get(&uri)
                                .is_some_and(|entry| entry.content() == content)
                            {
                                continue;
                            }
                            if let Some(warning) =
                                crate::session::services::store_document_text_fast(
                                    state, &uri, content,
                                )
                            {
                                warnings.push(warning);
                            }
                        }
                        None => {
                            if !state.index.contains_key(&uri) {
                                continue;
                            }
                            crate::session::services::remove_document(state, &uri);
                        }
                    }
                    changed = true;
                }
                if changed {
                    state.semantic_revision = state.semantic_revision.wrapping_add(1);
                    state.session.invalidate_inputs();
                    Mutation::Changed(warnings)
                } else {
                    Mutation::Unchanged(warnings)
                }
            })
            .await?;
        if outcome.published {
            self.rebuild_publication().await?;
        }
        Ok((outcome.published, outcome.value))
    }

    // --- did_change_configuration (library reindex) ---------------------------------------

    pub(crate) async fn begin_library_reindex_if_changed(
        &self,
        new_library_paths: Vec<Url>,
    ) -> Result<Option<super::readiness::ReadinessToken>, MutatePanicked> {
        self.actor
            .mutate_if_changed(move |s| {
                let old = std::mem::take(&mut s.library_paths);
                if new_library_paths == old {
                    s.library_paths = old;
                    Mutation::Unchanged(None)
                } else {
                    let _ = crate::session::services::clear_documents_under_roots(s, &old);
                    s.diagnostic_readiness.reset(false);
                    s.library_paths = new_library_paths.clone();
                    s.session.invalidate_inputs();
                    Mutation::Changed(Some(s.diagnostic_readiness.token()))
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
