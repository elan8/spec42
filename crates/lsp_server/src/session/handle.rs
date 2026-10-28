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

use std::sync::atomic::Ordering;
use std::sync::Arc;

use session_actor::{MutatePanicked, Mutation, SessionActor, SnapshotHandle};
use sysml_query::publication::{
    InputRevisionToken, PreparedPublication, PublicationBuildFailure, PublicationOutcome,
    SemanticBuild,
};
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
        handle.rebuild_publication().await.unwrap();

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
        handle.rebuild_publication().await.unwrap();

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
            handle.rebuild_publication().await.unwrap();

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
pub(crate) enum DocumentChange {
    Changed,
    Unchanged,
    Rejected(super::editor_documents::EditorEditRejection),
}

#[cfg(test)]
mod editor_update_tests {
    use super::*;
    use crate::session::editor_documents::EditorEditRejection;

    fn opened(source: &str) -> (WorkspaceHandle, Url) {
        let uri = Url::parse("memory://editor/model.sysml").unwrap();
        let mut state = ServerState::default();
        state.session.complete_startup();
        let mut entry = crate::session::state::IndexEntry::for_test(&uri, source);
        entry.open_editor(Some(1));
        state.index.insert(uri.clone(), entry);
        (WorkspaceHandle::spawn(state), uri)
    }

    async fn edit(handle: &WorkspaceHandle, uri: &Url, version: i32, text: &str) -> DocumentChange {
        let (change, warnings) = handle
            .apply_document_content_edit(
                uri.clone(),
                version,
                vec![TextDocumentContentChangeEvent {
                    range: None,
                    range_length: None,
                    text: text.into(),
                }],
            )
            .await
            .unwrap();
        assert!(warnings.is_empty());
        change
    }

    #[tokio::test]
    async fn no_op_advances_version_and_rejects_stale_edits_without_republishing() {
        let source = "package P { part def A; }";
        let (handle, uri) = opened(source);
        handle.rebuild_publication().await.unwrap();
        let publication = handle.snapshot().session.publication();
        assert!(matches!(
            edit(&handle, &uri, 4, source).await,
            DocumentChange::Unchanged
        ));
        assert!(matches!(
            edit(&handle, &uri, 3, "package Stale;").await,
            DocumentChange::Rejected(EditorEditRejection::StaleVersion {
                current: 4,
                received: 3
            })
        ));
        let snapshot = handle.snapshot();
        assert_eq!(snapshot.index[&uri].editor_version(), Some(4));
        assert_eq!(snapshot.index[&uri].content(), source);
        assert!(snapshot.session.is_publication_current(&publication));
        assert!(snapshot.index[&uri].parsed.is_some());
    }

    #[tokio::test]
    async fn incremental_edits_are_saved_before_parsing_and_survive_preparation_failure() {
        use tower_lsp::lsp_types::{Position, Range};
        let (handle, uri) = opened("package Initial;");
        handle.rebuild_publication().await.unwrap();
        assert!(matches!(
            edit(&handle, &uri, 2, "package Changed;").await,
            DocumentChange::Changed
        ));
        assert!(handle.snapshot().index[&uri].parsed.is_none());
        let failed = handle
            .rebuild_with_preparer(|_| Err(PublicationBuildFailure::preparation_worker("injected")))
            .await
            .unwrap();
        assert_eq!(failed.outcome, PublicationOutcome::Failed);
        assert_eq!(
            handle.snapshot().published_index[&uri].content(),
            "package Initial;"
        );
        let (change, warnings) = handle
            .apply_document_content_edit(
                uri.clone(),
                5,
                vec![TextDocumentContentChangeEvent {
                    range: Some(Range::new(Position::new(0, 8), Position::new(0, 15))),
                    range_length: None,
                    text: "Latest".into(),
                }],
            )
            .await
            .unwrap();
        assert!(warnings.is_empty());
        assert!(matches!(change, DocumentChange::Changed));
        assert_eq!(handle.snapshot().index[&uri].content(), "package Latest;");
        assert!(handle.rebuild_publication().await.unwrap());
        let snapshot = handle.snapshot();
        assert_eq!(snapshot.published_index[&uri].content(), "package Latest;");
        assert_eq!(snapshot.index[&uri].editor_version(), Some(5));
        assert!(snapshot.index[&uri].parsed.is_some());
    }

    #[tokio::test]
    async fn preparation_cannot_overwrite_a_newer_edit_or_reopened_editor_metadata() {
        let (handle, uri) = opened("package Initial;");
        edit(&handle, &uri, 2, "package Older;").await;
        let old_inputs = handle.snapshot().session.inputs();
        let old_prepared = prepare_publication(handle.snapshot()).unwrap();
        edit(&handle, &uri, 3, "package Newer;").await;
        let key = uri.clone();
        assert!(handle
            .actor
            .mutate(move |state| matches!(
                admit_prepared_publication(state, &old_inputs, Ok(old_prepared)),
                Mutation::Unchanged(PreparedPublicationAdmission::Superseded)
            ))
            .await
            .unwrap());
        assert_eq!(handle.snapshot().index[&key].content(), "package Newer;");
        handle.set_document_open(uri.clone(), false).await.unwrap();
        handle
            .set_editor_document_open(uri.clone(), -8)
            .await
            .unwrap();
        // Opening can invalidate reporting policy. Prepare for the actual input owner/revision,
        // then change only the no-op editor version while parsing is in flight.
        let inputs = handle.snapshot().session.inputs();
        let prepared = prepare_publication(handle.snapshot()).unwrap();
        assert!(matches!(
            edit(&handle, &uri, -2, "package Newer;").await,
            DocumentChange::Unchanged
        ));
        handle
            .actor
            .mutate(move |state| {
                assert!(matches!(
                    admit_prepared_publication(state, &inputs, Ok(prepared)),
                    Mutation::Changed(PreparedPublicationAdmission::Build(_))
                ));
            })
            .await
            .unwrap();
        assert_eq!(handle.snapshot().index[&uri].editor_version(), Some(-2));
    }

    #[tokio::test]
    async fn watched_updates_cannot_replace_open_editor_text_or_its_version() {
        let (handle, uri) = opened("package Unsaved;");
        let inputs = handle.snapshot().session.inputs();
        let (changed, warnings) = handle
            .apply_watched_file_changes(vec![
                (uri.clone(), None),
                (uri.clone(), Some("package Disk;".into())),
            ])
            .await
            .unwrap();
        assert!(!changed);
        assert!(warnings.is_empty());
        let snapshot = handle.snapshot();
        assert_eq!(snapshot.index[&uri].content(), "package Unsaved;");
        assert_eq!(snapshot.index[&uri].editor_version(), Some(1));
        assert!(snapshot.session.is_input_current(&inputs));
    }

    #[tokio::test]
    async fn raw_editor_changes_preserve_admitted_library_provenance() {
        let (handle, uri) = opened("package Library;");
        let key = uri.clone();
        handle
            .actor
            .mutate(move |state| {
                let entry = state.index.get_mut(&key).unwrap();
                entry.document = entry
                    .document
                    .with_kind(sysml_query::source::SourceKind::Library);
            })
            .await
            .unwrap();
        assert!(matches!(
            edit(&handle, &uri, 2, "package Changed;").await,
            DocumentChange::Changed
        ));
        assert_eq!(
            handle.snapshot().index[&uri].document.kind(),
            sysml_query::source::SourceKind::Library
        );
        assert!(handle.snapshot().index[&uri].parsed.is_none());
    }

    #[tokio::test]
    async fn opening_new_source_records_membership_version_and_pending_syntax_atomically() {
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
        assert!(snapshot.index[&uri].parsed.is_none());
    }
}

#[cfg(test)]
mod coherence_tests {
    use super::*;
    use crate::session::snapshot::ServerStateSnapshot;
    use crate::session::state::IndexEntry;
    use language_service::WorkspaceSnapshot;
    use sysml_query::publication::AnalysisStatus;

    #[tokio::test]
    async fn pending_and_failed_work_retain_the_paired_source_until_successful_admission() {
        let uri = Url::parse("memory://coherence/model.sysml").unwrap();
        let mut state = ServerState::default();
        state.session.complete_startup();
        state
            .index
            .insert(uri.clone(), IndexEntry::for_test(&uri, "package Initial;"));
        state.session.invalidate_inputs();
        let handle = WorkspaceHandle::spawn(state);
        assert!(handle.rebuild_publication().await.unwrap());
        let initial = handle.snapshot();
        let model = Arc::clone(initial.session.current());
        let sources = Arc::clone(&initial.published_index);
        let initial_parse = initial.published_index[&uri]
            .parsed
            .as_ref()
            .unwrap()
            .digest();
        let library_root = Url::parse("file:///configured/new-library/").unwrap();
        handle
            .actor
            .mutate({
                let uri = uri.clone();
                let library_root = library_root.clone();
                move |state| {
                    state
                        .index
                        .insert(uri.clone(), IndexEntry::for_test(&uri, "package Changed;"));
                    state.library_paths = vec![library_root];
                    state.session.invalidate_inputs();
                }
            })
            .await
            .unwrap();
        let pending = handle.snapshot();
        assert_eq!(pending.session.analysis_status(), AnalysisStatus::Updating);
        assert_eq!(pending.index[&uri].content(), "package Changed;");
        assert!(Arc::ptr_eq(&pending.published_index, &sources));
        let reader = ServerStateSnapshot::new(&pending, false);
        assert_eq!(reader.document_text(&uri), Some("package Initial;"));
        assert_eq!(reader.parsed(&uri).unwrap().digest(), initial_parse);
        assert!(reader.library_paths().is_empty());
        let failed = handle
            .rebuild_with_preparer(|_| {
                Err(PublicationBuildFailure::preparation_worker(
                    "cannot prepare inputs",
                ))
            })
            .await
            .unwrap();
        assert_eq!(failed.outcome, PublicationOutcome::Failed);
        let retained = handle.snapshot();
        assert_eq!(
            retained.session.analysis_status(),
            AnalysisStatus::FailedRetained
        );
        assert!(Arc::ptr_eq(retained.session.current(), &model));
        assert!(Arc::ptr_eq(&retained.published_index, &sources));
        assert_eq!(
            ServerStateSnapshot::new(&retained, false).document_text(&uri),
            Some("package Initial;")
        );
        assert!(handle.rebuild_publication().await.unwrap());
        let current = handle.snapshot();
        assert_eq!(current.session.analysis_status(), AnalysisStatus::Current);
        assert_eq!(
            ServerStateSnapshot::new(&current, false).document_text(&uri),
            Some("package Changed;")
        );
        assert_eq!(
            ServerStateSnapshot::new(&current, false).library_paths(),
            &[library_root]
        );
        assert!(!Arc::ptr_eq(&current.published_index, &sources));
    }
}

/// A preparation is not a build until its captured input revision passes the actor barrier.
enum PreparedPublicationAdmission {
    Build(Box<SemanticBuild>),
    Superseded,
    Failed(Arc<PublicationBuildFailure>),
    Cancelled,
}

struct PreparedInputs {
    publication: PreparedPublication,
    syntax: Vec<(SourceDocument, ParsedSource)>,
    parsing: std::time::Duration,
}

fn prepare_publication(
    snapshot: Arc<ServerState>,
) -> Result<PreparedInputs, PublicationBuildFailure> {
    let (documents, reported, availability) = crate::session::state::publication_inputs(&*snapshot);
    let inputs = snapshot.session.inputs();
    let cancellation = inputs.cancellation();
    let parse_started = std::time::Instant::now();
    let mut syntax = Vec::with_capacity(documents.len());
    for document in &documents {
        if cancellation.is_cancelled() {
            return Err(PublicationBuildFailure::cancelled());
        }
        let parsed = snapshot.services.syntax.parse(document);
        syntax.push((document.clone(), parsed));
    }
    let parsing = parse_started.elapsed();
    let publication = snapshot.services.publication.prepare_cancellable(
        &documents,
        availability,
        reported,
        cancellation,
    )?;
    Ok(PreparedInputs {
        publication,
        syntax,
        parsing,
    })
}

fn admit_prepared_publication(
    state: &mut ServerState,
    expected: &InputRevisionToken,
    prepared: Result<PreparedInputs, PublicationBuildFailure>,
) -> Mutation<PreparedPublicationAdmission> {
    if !state.session.is_input_current(expected) {
        return Mutation::Unchanged(PreparedPublicationAdmission::Superseded);
    }
    match prepared {
        Ok(prepared) => {
            // Source and parse identity are checked as a complete set before installing leases.
            assert!(
                prepared.syntax.iter().all(|(document, parsed)| {
                    state.index.get(document.uri()).is_some_and(|entry| {
                        entry.document.digest() == document.digest()
                            && entry.document.language() == document.language()
                            && parsed.digest() == document.digest()
                    })
                }),
                "prepared syntax must belong to the guarded desired source"
            );
            for (document, parsed) in prepared.syntax {
                state.index.get_mut(document.uri()).unwrap().parsed = Some(parsed);
            }
            let prepared = prepared.publication;
            let token = state.session.begin_build(prepared.identity().clone());
            Mutation::Changed(PreparedPublicationAdmission::Build(Box::new(
                SemanticBuild::new(token, prepared),
            )))
        }
        Err(failure) if failure.is_cancelled() => {
            state.session.finish_preparation_cancelled();
            state.publication_failure = None;
            Mutation::Changed(PreparedPublicationAdmission::Cancelled)
        }
        Err(failure) => {
            state.session.finish_preparation_failure();
            let failure = Arc::new(failure);
            state.publication_failure = Some(Arc::clone(&failure));
            Mutation::Changed(PreparedPublicationAdmission::Failed(failure))
        }
    }
}

#[cfg(test)]
mod preparation_tests {
    use super::*;

    #[tokio::test]
    async fn slow_preparation_does_not_block_the_actor_and_rejects_superseded_inputs() {
        let mut state = ServerState::default();
        state.session.complete_startup();
        let handle = WorkspaceHandle::spawn(state);
        let (started_tx, started_rx) = tokio::sync::oneshot::channel();
        let (resume_tx, resume_rx) = std::sync::mpsc::channel();
        let preparing = handle.clone();
        let rebuild = tokio::spawn(async move {
            preparing
                .rebuild_with_preparer(move |snapshot| {
                    let _ = started_tx.send(());
                    // Dropping the sender on a failed assertion also releases this worker.
                    let _ = resume_rx.recv();
                    prepare_publication(snapshot)
                })
                .await
        });
        started_rx.await.unwrap();
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            handle.actor.mutate(|state| {
                state.session.invalidate_inputs();
            }),
        )
        .await
        .expect("preparation must leave the source actor responsive")
        .unwrap();
        resume_tx.send(()).unwrap();
        let report = rebuild.await.unwrap().unwrap();
        assert_eq!(report.outcome, PublicationOutcome::Superseded);
        assert!(handle.snapshot().publication_failure.is_none());
        assert!(handle.rebuild_publication().await.unwrap());
    }

    #[tokio::test]
    async fn preparation_worker_panic_retains_the_last_good_publication_with_its_stage() {
        let handle = WorkspaceHandle::spawn(ServerState::default());
        let last_good = Arc::clone(handle.snapshot().session.current());
        let report = handle
            .rebuild_with_preparer(|_| panic!("preparation worker failed"))
            .await
            .unwrap();
        assert_eq!(report.outcome, PublicationOutcome::Failed);
        let snapshot = handle.snapshot();
        assert!(Arc::ptr_eq(snapshot.session.current(), &last_good));
        assert_eq!(
            snapshot.publication_failure.as_ref().unwrap().stage(),
            sysml_query::publication::PublicationFailureStage::PreparationWorker
        );
    }

    #[test]
    fn older_preparation_cannot_supersede_a_build_for_newer_inputs() {
        let mut state = ServerState::default();
        state.session.complete_startup();
        let older_inputs = state.session.inputs();
        let older = prepare_publication(Arc::new(state.clone())).unwrap();
        state.session.invalidate_inputs();
        let current_inputs = state.session.inputs();
        let current = prepare_publication(Arc::new(state.clone())).unwrap();
        let Mutation::Changed(PreparedPublicationAdmission::Build(current)) =
            admit_prepared_publication(&mut state, &current_inputs, Ok(current))
        else {
            panic!("current preparation must start a build")
        };
        assert!(matches!(
            admit_prepared_publication(&mut state, &older_inputs, Ok(older)),
            Mutation::Unchanged(PreparedPublicationAdmission::Superseded)
        ));
        let completion = state.services.publication.construct(*current);
        assert_eq!(
            completion.admit(&mut state.session),
            PublicationOutcome::Published
        );
    }

    #[test]
    fn preparation_failures_are_retained_only_for_current_owner_and_inputs() {
        let mut state = ServerState::default();
        state.session.complete_startup();
        let older = state.session.inputs();
        state.session.invalidate_inputs();
        let failure = || PublicationBuildFailure::preparation_worker("preparation panicked");
        assert!(matches!(
            admit_prepared_publication(&mut state, &older, Err(failure())),
            Mutation::Unchanged(PreparedPublicationAdmission::Superseded)
        ));
        let foreign = ServerState::default().session.inputs();
        assert!(matches!(
            admit_prepared_publication(&mut state, &foreign, Err(failure())),
            Mutation::Unchanged(PreparedPublicationAdmission::Superseded)
        ));
        assert!(state.publication_failure.is_none());
        let current = state.session.inputs();
        assert!(matches!(
            admit_prepared_publication(&mut state, &current, Err(failure())),
            Mutation::Changed(PreparedPublicationAdmission::Failed(_))
        ));
        assert_eq!(
            state.publication_failure.as_ref().unwrap().stage(),
            sysml_query::publication::PublicationFailureStage::PreparationWorker
        );
    }
}

#[cfg(test)]
mod scheduler_tests {
    use super::*;
    use crate::session::latest_work::Submission;
    use crate::session::state::IndexEntry;
    use sysml_query::publication::AnalysisStatus;

    #[tokio::test]
    async fn a_reserved_active_interval_coalesces_a_burst_into_the_latest_complete_sources() {
        let mut state = ServerState::default();
        state.session.complete_startup();
        let handle = WorkspaceHandle::spawn(state);
        // Reserve an interval without running it: edits and requests race a paused worker,
        // deterministically, without relying on wall-clock sleeps or parser workload size.
        let reserved = {
            let snapshot = handle.snapshot();
            let mut scheduler = handle.analysis_queue.lock().unwrap();
            scheduler.newest = Some(snapshot.session.inputs());
            let Submission::Start(work) = scheduler.work.submit(snapshot) else {
                panic!("idle queue")
            };
            work
        };
        let uri = Url::parse("memory://scheduler/model.sysml").unwrap();
        for revision in 0..100 {
            handle
                .actor
                .mutate({
                    let uri = uri.clone();
                    move |state| {
                        state.index.insert(
                            uri.clone(),
                            IndexEntry::for_test(&uri, &format!("package P{revision};")),
                        );
                        state.session.invalidate_inputs();
                    }
                })
                .await
                .unwrap();
            handle.queue_analysis(handle.snapshot());
            assert_eq!(
                handle.analysis_queue.lock().unwrap().work.occupancy(),
                (true, true)
            );
        }
        let active = handle.clone();
        let driver = tokio::spawn(async move {
            active.run_analysis(reserved).await;
        });
        let report = handle.rebuild_publication_measured().await.unwrap();
        assert_eq!(report.outcome, PublicationOutcome::Published);
        driver.await.unwrap();
        let current = handle.snapshot();
        assert_eq!(current.session.analysis_status(), AnalysisStatus::Current);
        assert_eq!(current.published_index[&uri].content(), "package P99;");
        assert_eq!(
            handle.analysis_queue.lock().unwrap().work.occupancy(),
            (false, false)
        );
        let (documents, reported, availability) =
            crate::session::state::publication_inputs(&*current);
        let fresh = sysml_query::Services::default()
            .publication
            .publish_with_standard_library_availability(&documents, availability, reported)
            .unwrap();
        assert_eq!(
            current.session.current().publication().identity(),
            fresh.publication().identity()
        );
    }

    #[tokio::test]
    async fn closing_an_owner_discards_pending_work_and_keeps_its_last_coherent_pair() {
        let mut state = ServerState::default();
        state.session.complete_startup();
        let handle = WorkspaceHandle::spawn(state);
        let reserved = {
            let snapshot = handle.snapshot();
            let mut scheduler = handle.analysis_queue.lock().unwrap();
            let Submission::Start(work) = scheduler.work.submit(snapshot) else {
                panic!("idle queue")
            };
            work
        };
        handle
            .actor
            .mutate(|state| {
                state.session.invalidate_inputs();
            })
            .await
            .unwrap();
        handle.queue_analysis(handle.snapshot());
        let model = Arc::clone(handle.snapshot().session.current());
        let sources = Arc::clone(&handle.snapshot().published_index);
        handle.close_session().await.unwrap();
        handle.run_analysis(reserved).await;
        assert_eq!(
            handle.rebuild_publication_measured().await.unwrap().outcome,
            PublicationOutcome::Superseded
        );
        let closed = handle.snapshot();
        assert!(Arc::ptr_eq(closed.session.current(), &model));
        assert!(Arc::ptr_eq(&closed.published_index, &sources));
        assert_eq!(
            handle.analysis_queue.lock().unwrap().work.occupancy(),
            (false, false)
        );
    }
}

#[derive(Clone)]
pub(crate) enum AnalysisEvent {
    Idle,
    Requested,
    Completed {
        inputs: InputRevisionToken,
        result: Result<Arc<PublicationRebuildReport>, Arc<MutatePanicked>>,
    },
    Closed,
}

#[derive(Default)]
struct AnalysisScheduler {
    work: super::latest_work::LatestWork<Arc<ServerState>>,
    newest: Option<InputRevisionToken>,
}

/// Host boundary costs plus the canonical phase owner's complete construction measurements.
/// Timings are observations only; they do not participate in publication identity or admission.
#[derive(Clone)]
pub(crate) struct PublicationRebuildReport {
    pub(crate) outcome: PublicationOutcome,
    pub(crate) preparation: std::time::Duration,
    pub(crate) preparation_queue: std::time::Duration,
    /// Syntax acquisition within preparation, including canonical memo hits.
    pub(crate) source_parsing: std::time::Duration,
    pub(crate) construction: std::time::Duration,
    pub(crate) admission: std::time::Duration,
    pub(crate) phases: Option<sysml_query::resolved_slice::BuildMeasurements>,
}

impl PublicationRebuildReport {
    fn superseded() -> Self {
        Self {
            outcome: PublicationOutcome::Superseded,
            preparation: std::time::Duration::ZERO,
            preparation_queue: std::time::Duration::ZERO,
            source_parsing: std::time::Duration::ZERO,
            construction: std::time::Duration::ZERO,
            admission: std::time::Duration::ZERO,
            phases: None,
        }
    }
}

#[derive(Clone)]
pub(crate) struct WorkspaceHandle {
    identity: Arc<()>,
    actor: SessionActor<ServerState>,
    snapshot: SnapshotHandle<ServerState>,
    /// The retained failure last shown to the user, so one failure is announced once.
    reported_publication_failure: Arc<std::sync::Mutex<Option<String>>>,
    analysis_queue: Arc<std::sync::Mutex<AnalysisScheduler>>,
    analysis_events: tokio::sync::watch::Sender<AnalysisEvent>,
    analysis_notifications_started: Arc<std::sync::atomic::AtomicBool>,
}

impl WorkspaceHandle {
    pub(crate) fn same_session(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.identity, &other.identity)
    }

    pub(crate) fn spawn(initial: ServerState) -> Self {
        let (actor, snapshot) = SessionActor::spawn(initial);
        let (analysis_events, _) = tokio::sync::watch::channel(AnalysisEvent::Idle);
        Self {
            identity: Arc::new(()),
            actor,
            snapshot,
            reported_publication_failure: Arc::default(),
            analysis_queue: Arc::default(),
            analysis_events,
            analysis_notifications_started: Arc::default(),
        }
    }

    pub(crate) fn analysis_events(&self) -> tokio::sync::watch::Receiver<AnalysisEvent> {
        self.analysis_events.subscribe()
    }

    pub(crate) fn claim_analysis_notifications(&self) -> bool {
        self.analysis_notifications_started
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }

    pub(crate) fn analysis_occupancy(&self) -> (usize, usize) {
        let (active, pending) = self
            .analysis_queue
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .work
            .occupancy();
        (usize::from(active), usize::from(pending))
    }

    pub(crate) fn request_rebuild(&self) {
        self.queue_analysis(self.snapshot());
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

    /// Latest published snapshot. Non-blocking, never awaits — the whole point.
    pub(crate) fn snapshot(&self) -> Arc<ServerState> {
        self.snapshot.current()
    }

    /// Prepare and construct from an immutable source snapshot outside the actor. Preparation
    /// can build a cold library stratum; it must not block ordered source ingestion. At the actor
    /// barrier, verify the captured owner/revision before assigning a build generation. This keeps
    /// a slow preparation for older inputs from superseding a newer build.
    pub(crate) async fn rebuild_publication(&self) -> Result<bool, MutatePanicked> {
        self.rebuild_publication_measured()
            .await
            .map(|report| report.outcome == PublicationOutcome::Published)
    }

    pub(crate) async fn rebuild_publication_measured(
        &self,
    ) -> Result<PublicationRebuildReport, MutatePanicked> {
        let snapshot = self.snapshot();
        let expected = snapshot.session.inputs();
        let mut events = self.analysis_events();
        self.queue_analysis(snapshot);
        loop {
            if !self.snapshot().session.is_input_current(&expected) {
                return Ok(PublicationRebuildReport::superseded());
            }
            match events.borrow_and_update().clone() {
                AnalysisEvent::Completed { inputs, result } if inputs == expected => {
                    return result
                        .map(|report| (*report).clone())
                        .map_err(|_| MutatePanicked);
                }
                AnalysisEvent::Closed => return Ok(PublicationRebuildReport::superseded()),
                _ => {}
            }
            if events.changed().await.is_err() {
                return Ok(PublicationRebuildReport::superseded());
            }
        }
    }

    /// Queue complete immutable inputs. The only spawned driver runs one active job and takes
    /// at most one replacement snapshot after it finishes; repeated requests cannot spawn jobs.
    fn queue_analysis(&self, snapshot: Arc<ServerState>) {
        use super::latest_work::Submission;
        let inputs = snapshot.session.inputs();
        let submission = {
            let mut scheduler = self
                .analysis_queue
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if scheduler.newest.as_ref() == Some(&inputs) || scheduler.work.is_closed() {
                return;
            }
            let submission = scheduler.work.submit(snapshot);
            if !matches!(submission, Submission::Closed) {
                scheduler.newest = Some(inputs.clone());
                self.analysis_events.send_replace(AnalysisEvent::Requested);
            }
            submission
        };
        if let Submission::Start(work) = submission {
            let handle = self.clone();
            tokio::spawn(async move {
                handle.run_analysis(work).await;
            });
        }
    }

    async fn run_analysis(&self, mut work: super::latest_work::Work<Arc<ServerState>>) {
        loop {
            let (token, snapshot) = work.into_parts();
            let inputs = snapshot.session.inputs();
            let result = self
                .rebuild_snapshot_with_preparer(snapshot, prepare_publication)
                .await
                .map(Arc::new)
                .map_err(Arc::new);
            let next = {
                let mut scheduler = self
                    .analysis_queue
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                let next = match scheduler.work.complete(&token) {
                    Ok(next) => next,
                    Err(_) => return,
                };
                if next.is_none() {
                    scheduler.newest = None;
                }
                if !scheduler.work.is_closed() {
                    self.analysis_events
                        .send_replace(AnalysisEvent::Completed { inputs, result });
                }
                next
            };
            match next {
                Some(next) => work = next,
                None => return,
            }
        }
    }

    pub(crate) async fn close_session(&self) -> Result<(), MutatePanicked> {
        self.actor.mutate(|state| state.session.close()).await?;
        let mut scheduler = self
            .analysis_queue
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        scheduler.work.close();
        scheduler.newest = None;
        self.analysis_events.send_replace(AnalysisEvent::Closed);
        Ok(())
    }

    #[cfg(test)]
    async fn rebuild_with_preparer(
        &self,
        prepare: impl FnOnce(Arc<ServerState>) -> Result<PreparedInputs, PublicationBuildFailure>
            + Send
            + 'static,
    ) -> Result<PublicationRebuildReport, MutatePanicked> {
        self.rebuild_snapshot_with_preparer(self.snapshot(), prepare)
            .await
    }

    async fn rebuild_snapshot_with_preparer(
        &self,
        snapshot: Arc<ServerState>,
        prepare: impl FnOnce(Arc<ServerState>) -> Result<PreparedInputs, PublicationBuildFailure>
            + Send
            + 'static,
    ) -> Result<PublicationRebuildReport, MutatePanicked> {
        let preparation_started = std::time::Instant::now();
        let expected = snapshot.session.inputs();
        let prepared = tokio::task::spawn_blocking(move || {
            let started = std::time::Instant::now();
            let result = prepare(snapshot);
            (result, started.elapsed())
        })
        .await;
        let (prepared, preparation) = match prepared {
            Ok(result) => result,
            Err(error) => (
                Err(PublicationBuildFailure::preparation_worker(error)),
                std::time::Duration::ZERO,
            ),
        };
        let source_parsing = prepared
            .as_ref()
            .map_or(std::time::Duration::ZERO, |inputs| inputs.parsing);
        let staged = self
            .actor
            .mutate_if_changed(move |state| admit_prepared_publication(state, &expected, prepared))
            .await?
            .value;
        let preparation_queue = preparation_started.elapsed().saturating_sub(preparation);
        let mut report = PublicationRebuildReport {
            outcome: PublicationOutcome::Failed,
            preparation,
            preparation_queue,
            source_parsing,
            construction: std::time::Duration::ZERO,
            admission: std::time::Duration::ZERO,
            phases: None,
        };
        let build = match staged {
            PreparedPublicationAdmission::Build(build) => *build,
            PreparedPublicationAdmission::Superseded => {
                report.outcome = PublicationOutcome::Superseded;
                return Ok(report);
            }
            PreparedPublicationAdmission::Cancelled => {
                report.outcome = PublicationOutcome::Cancelled;
                return Ok(report);
            }
            PreparedPublicationAdmission::Failed(failure) => {
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
                            PublicationOutcome::Published | PublicationOutcome::Cancelled => unreachable!("a failed construction worker cannot publish or cancel a completed model"),
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
                        state.publish_source_snapshot();
                        state.publication_failure = None;
                        crate::session::state::refresh_symbol_table_from_publication(state);
                        state
                            .services
                            .syntax
                            .retain(state.index.values().filter_map(|entry| entry.parsed.as_ref().map(ParsedSource::digest)));
                        Mutation::Changed(outcome)
                    }
                    PublicationOutcome::Superseded => Mutation::Unchanged(outcome),
                    PublicationOutcome::Cancelled => {
                        state.publication_failure = None;
                        Mutation::Changed(outcome)
                    }
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
            self.request_rebuild();
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
                state.services.syntax.retain(
                    state
                        .index
                        .values()
                        .filter_map(|entry| entry.parsed.as_ref().map(ParsedSource::digest)),
                );
                Mutation::Changed(warning)
            })
            .await
            .map(|outcome| outcome.value)
    }

    /// Save ordered editor deltas before any fallible or cancellable worker starts.
    pub(crate) async fn apply_document_content_edit(
        &self,
        uri: Url,
        version: i32,
        changes: Vec<TextDocumentContentChangeEvent>,
    ) -> Result<(DocumentChange, Vec<(MessageType, String)>), MutatePanicked> {
        use super::editor_documents::EditorEditRejection;
        self.actor
            .mutate_if_changed(move |state| {
                let Some(entry) = state.index.get(&uri) else {
                    return Mutation::Unchanged((
                        DocumentChange::Rejected(EditorEditRejection::NotIndexed),
                        Vec::new(),
                    ));
                };
                let captured = match entry
                    .editor_revision
                    .as_ref()
                    .ok_or(EditorEditRejection::NotOpen)
                    .and_then(|revision| revision.prepare(version))
                {
                    Ok(captured) => captured,
                    Err(reason) => {
                        return Mutation::Unchanged((DocumentChange::Rejected(reason), Vec::new()))
                    }
                };
                let (content, changed, warnings) = crate::session::services::apply_content_changes(
                    entry.content(),
                    &uri,
                    version,
                    changes,
                );
                state
                    .index
                    .get_mut(&uri)
                    .unwrap()
                    .editor_revision
                    .as_mut()
                    .unwrap()
                    .advance(&captured, version)
                    .expect("validated in the same actor mutation");
                if changed {
                    crate::session::services::store_document_text_fast(state, &uri, content);
                    state.session.invalidate_inputs();
                    state.services.syntax.retain(
                        state
                            .index
                            .values()
                            .filter_map(|entry| entry.parsed.as_ref().map(ParsedSource::digest)),
                    );
                    Mutation::Changed((DocumentChange::Changed, warnings))
                } else {
                    Mutation::Changed((DocumentChange::Unchanged, warnings))
                }
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

    /// Ingest one watched-file batch as a single desired revision; the caller schedules analysis.
    /// Rename notifications commonly contain a delete and a create in the same batch.
    pub(crate) async fn apply_watched_file_changes(
        &self,
        changes: Vec<(Url, Option<String>)>,
    ) -> Result<(bool, Vec<String>), MutatePanicked> {
        let outcome =
            self.actor
                .mutate_if_changed(move |state| {
                    let mut warnings = Vec::new();
                    let mut changed = false;
                    for (uri, content) in changes {
                        // The editor remains authoritative until didClose, including across saves,
                        // renames and a delayed disk-read completion.
                        if state.is_editor_open(&uri) {
                            continue;
                        }
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
                        state.services.syntax.retain(
                            state.index.values().filter_map(|entry| {
                                entry.parsed.as_ref().map(ParsedSource::digest)
                            }),
                        );
                        Mutation::Changed(warnings)
                    } else {
                        Mutation::Unchanged(warnings)
                    }
                })
                .await?;
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
