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
use sysml_query::syntax::ParsedSource;
use tower_lsp::lsp_types::{MessageType, TextDocumentContentChangeEvent, Url};

use crate::session::services::ParsedScanEntry;
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
        state.invalidate_inputs();
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
                    state.invalidate_inputs();
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
    input_sources: Vec<sysml_query::source::SourceDocument>,
    /// The syntax authority's opaque prepared batch holds leases across unrelated sweeps.
    syntax: sysml_query::syntax::PreparedSyntax,
    parsing: std::time::Duration,
    library_selection: std::time::Duration,
}

fn prepare_publication(
    snapshot: Arc<ServerState>,
) -> Result<PreparedInputs, PublicationBuildFailure> {
    let (input_sources, reported, availability) =
        crate::session::state::publication_inputs(&*snapshot);
    let inputs = snapshot.session.inputs();
    let cancellation = inputs.cancellation();
    let parse_started = std::time::Instant::now();
    let authored = snapshot.services.syntax.prepare(
        input_sources
            .iter()
            .take_while(|_| !cancellation.is_cancelled()),
    );
    if cancellation.is_cancelled() {
        return Err(PublicationBuildFailure::cancelled());
    }
    let selection_started = std::time::Instant::now();
    let documents = match &snapshot.library_catalog {
        Some(catalog) => catalog.publication_documents(
            &authored,
            &sysml_query::library::LibraryClosureOptions::default(),
            snapshot.library_admission,
        ),
        None => input_sources.clone(),
    };
    let library_selection = selection_started.elapsed();
    if cancellation.is_cancelled() {
        return Err(PublicationBuildFailure::cancelled());
    }
    let syntax = snapshot.services.syntax.prepare(&documents);
    let parsing = parse_started.elapsed().saturating_sub(library_selection);
    let publication = snapshot.services.publication.prepare_cancellable(
        &documents,
        availability,
        reported,
        cancellation,
    )?;
    Ok(PreparedInputs {
        publication,
        input_sources,
        syntax,
        parsing,
        library_selection,
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
            // Source identity is checked as a complete set before installing leases.
            assert!(
                prepared.input_sources.iter().all(|document| {
                    state.index.get(document.uri()).is_some_and(|entry| {
                        entry.document.digest() == document.digest()
                            && entry.document.language() == document.language()
                    })
                }),
                "prepared syntax must belong to the guarded desired source"
            );
            if state.library_catalog.is_some() {
                for entry in state.index.values_mut() {
                    if !entry.is_editor_open()
                        && !entry.workspace_source
                        && (entry.document.kind().is_library()
                            || crate::common::util::uri_under_any_library(
                                entry.document.uri(),
                                &state.library_paths,
                            )
                            || crate::common::util::uri_under_any_library(
                                entry.document.uri(),
                                &state.standard_library_paths,
                            ))
                    {
                        entry.admitted_to_publication = false;
                    }
                }
            }
            // Transfer the existing leases: admission never acquires or parses syntax. New
            // library sources were selected by the captured immutable canonical catalogue.
            for (document, parsed) in prepared.syntax.into_sources() {
                let uri = document.uri().clone();
                let editor_revision = state
                    .index
                    .get(&uri)
                    .and_then(|entry| entry.editor_revision.clone());
                let workspace_source = state
                    .index
                    .get(&uri)
                    .is_some_and(|entry| entry.workspace_source);
                state.index.insert(
                    uri,
                    crate::session::state::IndexEntry {
                        document,
                        parsed: Some(parsed),
                        admitted_to_publication: true,
                        workspace_source,
                        editor_revision,
                    },
                );
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
    use crate::session::state::IndexEntry;

    #[tokio::test]
    async fn migrated_open_sources_take_the_new_workspace_role_without_old_library_slots() {
        let uri = Url::parse("memory://migration/model.sysml").unwrap();
        let mut entry = IndexEntry::for_test(&uri, "package Unsaved;");
        entry.workspace_source = false;
        entry.document = entry
            .document
            .with_kind(sysml_query::source::SourceKind::Library)
            .with_library_location(2, "model.sysml");
        entry.open_editor(Some(17));
        let digest = entry.document.digest();
        let handle = WorkspaceHandle::spawn(ServerState::default());
        handle
            .adopt_open_documents(vec![(uri.clone(), entry)])
            .await
            .unwrap();
        let state = handle.snapshot();
        let migrated = &state.index[&uri];
        assert!(migrated.workspace_source);
        assert_eq!(
            migrated.document.kind(),
            sysml_query::source::SourceKind::Workspace
        );
        assert_eq!(migrated.document.library_location(), None);
        assert_eq!(migrated.document.digest(), digest);
        assert_eq!(migrated.editor_version(), Some(17));
    }

    #[test]
    fn prepared_syntax_crosses_sweeps_but_never_a_replaced_input_revision() {
        for superseded in [false, true] {
            let mut state = ServerState::default();
            state.session.complete_startup();
            let uri = Url::parse("memory://preparation/model.sysml").unwrap();
            let mut entry = IndexEntry::for_test(&uri, "package Prepared;");
            entry.parsed = None;
            state.index.insert(uri.clone(), entry);
            state.invalidate_inputs();
            let inputs = state.session.inputs();
            let prepared = prepare_publication(Arc::new(state.clone())).unwrap();
            for _ in 0..4 {
                state.services.syntax.retain([]);
            }
            if superseded {
                let mut newer = IndexEntry::for_test(&uri, "package Newer;");
                newer.parsed = None;
                state.index.insert(uri.clone(), newer);
                state.invalidate_inputs();
            }
            let admission = admit_prepared_publication(&mut state, &inputs, Ok(prepared));
            if superseded {
                assert!(matches!(
                    admission,
                    Mutation::Unchanged(PreparedPublicationAdmission::Superseded)
                ));
                assert!(state.index[&uri].parsed.is_none());
            } else {
                assert!(matches!(
                    admission,
                    Mutation::Changed(PreparedPublicationAdmission::Build(_))
                ));
                let entry = &state.index[&uri];
                assert_eq!(
                    entry.parsed.as_ref().unwrap().digest(),
                    entry.document.digest()
                );
                assert_eq!(
                    entry.parsed.as_ref().unwrap().language(),
                    entry.document.language()
                );
            }
        }
    }

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
                state.invalidate_inputs();
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
        state.invalidate_inputs();
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
        state.invalidate_inputs();
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
                        state.invalidate_inputs();
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
                state.invalidate_inputs();
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
    pub(crate) library_selection: std::time::Duration,
    pub(crate) construction: std::time::Duration,
    pub(crate) admission: std::time::Duration,
    pub(crate) phases: Option<sysml_query::resolved_slice::BuildMeasurements>,
}

impl PublicationRebuildReport {
    fn source_discovery_failed() -> Self {
        Self {
            outcome: PublicationOutcome::Failed,
            ..Self::superseded()
        }
    }

    fn superseded() -> Self {
        Self {
            outcome: PublicationOutcome::Superseded,
            preparation: std::time::Duration::ZERO,
            preparation_queue: std::time::Duration::ZERO,
            source_parsing: std::time::Duration::ZERO,
            library_selection: std::time::Duration::ZERO,
            construction: std::time::Duration::ZERO,
            admission: std::time::Duration::ZERO,
            phases: None,
        }
    }
}

#[cfg(test)]
mod library_reindex_tests {
    use super::*;
    use crate::session::state::{publication_inputs, IndexEntry};
    use sysml_query::source::SourceKind;

    #[tokio::test]
    async fn project_owned_bindings_ignore_defaults_but_retry_failed_discovery_with_the_same_roots()
    {
        let mut state = ServerState::default();
        state.session.complete_startup();
        state.library_paths_follow_host_defaults = false;
        let pinned = uri("file:///project-pinned/");
        state.library_paths = vec![pinned.clone()];
        let handle = WorkspaceHandle::spawn(state);
        let inputs = handle.snapshot().session.inputs();
        let defaults = vec![uri("file:///editor-defaults/")];
        assert!(handle
            .begin_library_reindex_if_changed(defaults.clone())
            .await
            .unwrap()
            .is_none());
        assert_eq!(handle.snapshot().library_paths, vec![pinned.clone()]);
        assert!(handle.snapshot().session.is_input_current(&inputs));
        handle
            .actor
            .mutate(|state| {
                state.diagnostic_readiness.require_library_sources();
                state.invalidate_inputs();
            })
            .await
            .unwrap();
        let failed = handle.snapshot().diagnostic_readiness.token();
        assert!(handle
            .fail_library_reindex(
                failed,
                PublicationBuildFailure::preparation_worker("inventory failed")
            )
            .await
            .unwrap());
        let retry = handle
            .begin_library_reindex_if_changed(defaults)
            .await
            .unwrap()
            .unwrap();
        let (work, request) = handle.queue_library_scan(retry).unwrap().into_parts();
        assert_eq!(request.library_paths, vec![pinned]);
        assert!(handle.snapshot().publication_failure.is_none());
        assert!(handle.finish_library_scan(&work).is_none());
    }

    fn uri(value: &str) -> Url {
        crate::common::util::normalize_file_uri(&Url::parse(value).unwrap())
    }

    fn scanned(handle: &WorkspaceHandle, url: &Url, text: &str) -> Vec<ParsedScanEntry> {
        let services = handle.snapshot().services.clone();
        let document = services
            .source
            .admit_url(url.clone(), text, SourceKind::Workspace);
        crate::session::services::parse_scanned_documents(vec![document], false, &services)
    }

    fn scan_catalog(
        handle: &WorkspaceHandle,
        entries: &[ParsedScanEntry],
    ) -> sysml_query::library::LibraryCatalog {
        let snapshot = handle.snapshot();
        let documents = entries
            .iter()
            .filter_map(|entry| {
                if crate::common::util::uri_under_any_library(
                    &entry.uri,
                    &snapshot.standard_library_paths,
                ) {
                    Some(entry.document.with_kind(SourceKind::StandardLibrary))
                } else if crate::common::util::uri_under_any_library(
                    &entry.uri,
                    &snapshot.library_paths,
                ) {
                    Some(entry.document.with_kind(SourceKind::Library))
                } else {
                    None
                }
            })
            .collect();
        snapshot.services.library.catalog_from_documents(documents)
    }

    async fn complete_library_scan(
        handle: &WorkspaceHandle,
        token: super::super::readiness::ReadinessToken,
        entries: Vec<ParsedScanEntry>,
    ) -> Result<LibraryReindexCompletion, MutatePanicked> {
        handle
            .complete_library_reindex(token, scan_catalog(handle, &entries))
            .await
    }

    async fn complete_startup_scan(
        handle: &WorkspaceHandle,
        token: super::super::readiness::ReadinessToken,
        entries: Vec<ParsedScanEntry>,
    ) -> Result<LibraryReindexCompletion, MutatePanicked> {
        let catalog = scan_catalog(handle, &entries);
        handle.ingest_startup_scan(token, entries, catalog).await
    }

    fn ready() -> WorkspaceHandle {
        let mut state = ServerState::default();
        state.session.complete_startup();
        WorkspaceHandle::spawn(state)
    }

    #[tokio::test]
    async fn configuration_replacement_keeps_startup_workspace_discovery_in_the_latest_request() {
        let handle = ready();
        let workspace_root = uri("file:///startup/workspace/");
        let old_root = uri("file:///startup/old-library/");
        let new_root = uri("file:///startup/new-library/");
        let standard_root = uri("file:///startup/standard/");
        handle
            .set_startup_config(
                vec![workspace_root.clone()],
                vec![old_root],
                vec![standard_root.clone()],
                sysml_query::StandardLibraryAvailability::Unavailable,
                true,
            )
            .await
            .unwrap();
        let old = handle.begin_startup().await.unwrap().unwrap();
        let (active, _) = handle.queue_library_scan(old.clone()).unwrap().into_parts();
        let workspace = uri("file:///startup/workspace/model.sysml");
        let old_library = uri("file:///startup/old-library/model.sysml");
        let mut stale = scanned(&handle, &workspace, "package Workspace;");
        stale.extend(scanned(&handle, &old_library, "package OldLibrary;"));
        let latest = handle
            .begin_library_reindex_if_changed(vec![new_root.clone()])
            .await
            .unwrap()
            .unwrap();
        assert!(handle.queue_library_scan(latest.clone()).is_none());
        assert!(matches!(
            complete_startup_scan(&handle, old, stale).await.unwrap(),
            LibraryReindexCompletion::Superseded
        ));
        assert!(!handle.snapshot().index.contains_key(&old_library));
        assert!(!handle.snapshot().index.contains_key(&workspace));
        // A library-only completion cannot release the still-required workspace barrier.
        assert!(matches!(
            complete_library_scan(&handle, latest.clone(), Vec::new())
                .await
                .unwrap(),
            LibraryReindexCompletion::Superseded
        ));
        assert_eq!(handle.analysis_occupancy(), (0, 0));
        let (next, request) = handle.finish_library_scan(&active).unwrap().into_parts();
        let startup = request.startup.unwrap();
        assert_eq!(startup.workspace_roots, vec![workspace_root]);
        assert_eq!(startup.standard_library_paths, vec![standard_root]);
        assert_eq!(request.library_paths, vec![new_root]);
        assert!(handle.library_scan_is_current(&request.readiness));
        let library = uri("file:///startup/new-library/model.sysml");
        let mut entries = scanned(&handle, &workspace, "package Workspace;");
        entries.extend(scanned(&handle, &library, "package LatestLibrary;"));
        assert!(matches!(
            complete_startup_scan(&handle, latest, entries)
                .await
                .unwrap(),
            LibraryReindexCompletion::Admitted(..)
        ));
        handle.rebuild_publication().await.unwrap();
        let snapshot = handle.snapshot();
        assert!(snapshot.published_index.contains_key(&workspace));
        assert!(snapshot.published_index.contains_key(&library));
        assert!(!snapshot.published_index.contains_key(&old_library));
        assert!(!snapshot.diagnostic_readiness.startup_sources_pending());
        assert_eq!(
            snapshot.session.analysis_status(),
            sysml_query::publication::AnalysisStatus::Current
        );
        assert!(handle.finish_library_scan(&next).is_none());
    }

    #[tokio::test]
    async fn observed_disk_updates_and_deletes_reject_old_startup_and_library_reads() {
        for startup in [false, true] {
            let handle = ready();
            let root = uri("file:///disk-observation/");
            let changed = uri("file:///disk-observation/changed.sysml");
            let deleted = uri("file:///disk-observation/deleted.sysml");
            let old = if startup {
                handle
                    .set_startup_config(
                        vec![root.clone()],
                        Vec::new(),
                        Vec::new(),
                        sysml_query::StandardLibraryAvailability::Unavailable,
                        true,
                    )
                    .await
                    .unwrap();
                handle.begin_startup().await.unwrap().unwrap()
            } else {
                handle
                    .begin_library_reindex_if_changed(vec![root.clone()])
                    .await
                    .unwrap()
                    .unwrap()
            };
            let (active, _) = handle.queue_library_scan(old.clone()).unwrap().into_parts();
            let mut stale = scanned(&handle, &changed, "package Old;");
            stale.extend(scanned(&handle, &deleted, "package Deleted;"));
            handle.begin_filesystem_observation().await.unwrap();
            assert!(!handle.library_scan_is_current(&old));
            // A configuration arriving during the read captures the pending disk barrier too.
            let current = handle
                .begin_library_reindex_if_changed(vec![
                    root.clone(),
                    uri("file:///disk-observation-extra/"),
                ])
                .await
                .unwrap()
                .unwrap();
            let result = if startup {
                complete_startup_scan(&handle, current.clone(), stale).await
            } else {
                complete_library_scan(&handle, current.clone(), stale).await
            }
            .unwrap();
            assert!(matches!(result, LibraryReindexCompletion::Superseded));
            handle
                .apply_watched_file_changes(vec![
                    (changed.clone(), Some("package New;".into())),
                    (deleted.clone(), None),
                ])
                .await
                .unwrap();
            let latest = handle
                .finish_filesystem_observation()
                .await
                .unwrap()
                .unwrap();
            assert!(!handle.source_epoch_is_current(&current));
            assert!(handle.queue_library_scan(latest.clone()).is_none());
            let (next, request) = handle.finish_library_scan(&active).unwrap().into_parts();
            assert!(handle.library_scan_is_current(&request.readiness));
            assert_eq!(request.startup.is_some(), startup);
            let entries = scanned(&handle, &changed, "package New;");
            let result = if startup {
                complete_startup_scan(&handle, latest, entries).await
            } else {
                complete_library_scan(&handle, latest, entries).await
            }
            .unwrap();
            assert!(matches!(result, LibraryReindexCompletion::Admitted(..)));
            handle.rebuild_publication().await.unwrap();
            assert_eq!(
                handle.snapshot().published_index[&changed].content(),
                "package New;"
            );
            assert!(!handle.snapshot().published_index.contains_key(&deleted));
            assert!(handle.finish_library_scan(&next).is_none());
        }
    }

    #[tokio::test]
    async fn search_preparation_cannot_reintroduce_a_removed_library_or_mutate_a_closed_owner() {
        let handle = ready();
        let old = handle
            .begin_library_reindex_if_changed(vec![uri("file:///search/old/")])
            .await
            .unwrap()
            .unwrap();
        let library = uri("file:///search/old/model.sysml");
        let entries = scanned(&handle, &library, "package Old;");
        handle
            .begin_library_reindex_if_changed(vec![uri("file:///search/new/")])
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            handle
                .admit_library_search(
                    old,
                    handle.snapshot().diagnostic_readiness.disk_revision(),
                    entries
                )
                .await
                .unwrap(),
            0
        );
        assert!(!handle.snapshot().index.contains_key(&library));
        let current = handle.snapshot().diagnostic_readiness.token();
        let new_library = uri("file:///search/new/model.sysml");
        let entries = scanned(&handle, &new_library, "package SearchOnly;");
        let inputs = handle.snapshot().session.inputs();
        assert_eq!(
            handle
                .admit_library_search(
                    current.clone(),
                    handle.snapshot().diagnostic_readiness.disk_revision(),
                    entries
                )
                .await
                .unwrap(),
            1
        );
        assert!(!handle.snapshot().index[&new_library].admitted_to_publication);
        assert!(handle.snapshot().session.is_input_current(&inputs));
        let entries = scanned(&handle, &library, "package Old;");
        handle.close_session().await.unwrap();
        assert_eq!(
            handle
                .admit_library_search(
                    current,
                    handle.snapshot().diagnostic_readiness.disk_revision(),
                    entries
                )
                .await
                .unwrap(),
            0
        );
        assert!(!handle.snapshot().index.contains_key(&library));
    }

    #[tokio::test]
    async fn startup_failure_retains_its_workspace_barrier_and_retries_the_complete_scope() {
        let handle = ready();
        let root = uri("file:///startup-retry/workspace/");
        handle
            .set_startup_config(
                vec![root.clone()],
                Vec::new(),
                Vec::new(),
                sysml_query::StandardLibraryAvailability::Unavailable,
                true,
            )
            .await
            .unwrap();
        let token = handle.begin_startup().await.unwrap().unwrap();
        let foreign = WorkspaceHandle::spawn(ServerState::default());
        let foreign_token = foreign.begin_startup().await.unwrap().unwrap();
        let inputs = handle.snapshot().session.inputs();
        assert!(matches!(
            complete_startup_scan(&handle, foreign_token, Vec::new())
                .await
                .unwrap(),
            LibraryReindexCompletion::Superseded
        ));
        assert!(handle.snapshot().session.is_input_current(&inputs));
        let retained = Arc::clone(handle.snapshot().session.current());
        assert!(handle
            .fail_library_reindex(
                token.clone(),
                PublicationBuildFailure::preparation_worker("discovery failed")
            )
            .await
            .unwrap());
        assert!(!handle
            .begin_readiness_notification(token.clone())
            .await
            .unwrap());
        assert_eq!(
            handle.snapshot().session.analysis_status(),
            sysml_query::publication::AnalysisStatus::FailedRetained
        );
        assert!(Arc::ptr_eq(handle.snapshot().session.current(), &retained));
        let retry = handle
            .begin_library_reindex_if_changed(Vec::new())
            .await
            .unwrap()
            .unwrap();
        let (work, request) = handle
            .queue_library_scan(retry.clone())
            .unwrap()
            .into_parts();
        assert_eq!(request.startup.unwrap().workspace_roots, vec![root]);
        assert!(matches!(
            complete_startup_scan(&handle, token, Vec::new())
                .await
                .unwrap(),
            LibraryReindexCompletion::Superseded
        ));
        let model = uri("file:///startup-retry/workspace/model.sysml");
        let entries = scanned(&handle, &model, "package Recovered;");
        complete_startup_scan(&handle, retry.clone(), entries)
            .await
            .unwrap();
        handle.rebuild_publication().await.unwrap();
        assert!(handle.snapshot().published_index.contains_key(&model));
        assert_eq!(
            handle.snapshot().session.analysis_status(),
            sysml_query::publication::AnalysisStatus::Current
        );
        assert!(handle.finish_library_scan(&work).is_none());
        handle.close_session().await.unwrap();
        assert!(matches!(
            complete_startup_scan(&handle, retry, Vec::new())
                .await
                .unwrap(),
            LibraryReindexCompletion::Superseded
        ));
    }

    #[tokio::test]
    async fn disk_observation_invalidates_search_reads_after_semantic_source_admission() {
        let handle = ready();
        let token = handle
            .begin_library_reindex_if_changed(vec![uri("file:///search-disk/")])
            .await
            .unwrap()
            .unwrap();
        complete_library_scan(&handle, token.clone(), Vec::new())
            .await
            .unwrap();
        handle.rebuild_publication().await.unwrap();
        let disk = handle.snapshot().diagnostic_readiness.disk_revision();
        let library = uri("file:///search-disk/deleted.sysml");
        let stale = scanned(&handle, &library, "package Deleted;");
        handle.begin_filesystem_observation().await.unwrap();
        handle
            .apply_watched_file_changes(vec![(library.clone(), None)])
            .await
            .unwrap();
        assert!(handle
            .finish_filesystem_observation()
            .await
            .unwrap()
            .is_none());
        // Readiness is unchanged; the independent disk frontier rejects the old search batch.
        assert!(handle.source_epoch_is_current(&token));
        assert!(!handle.library_search_is_current(&token, &disk));
        assert_eq!(
            handle
                .admit_library_search(token.clone(), disk, stale)
                .await
                .unwrap(),
            0
        );
        assert!(!handle.snapshot().index.contains_key(&library));
        let fresh = scanned(&handle, &library, "package Recreated;");
        let disk = handle.snapshot().diagnostic_readiness.disk_revision();
        assert_eq!(
            handle
                .admit_library_search(token, disk, fresh)
                .await
                .unwrap(),
            1
        );
        assert_eq!(
            handle.snapshot().index[&library].content(),
            "package Recreated;"
        );
    }

    #[tokio::test]
    async fn library_scan_burst_retains_only_one_active_and_the_latest_configuration() {
        let handle = ready();
        let initial = handle
            .begin_library_reindex_if_changed(vec![uri("file:///scan/initial/")])
            .await
            .unwrap()
            .unwrap();
        let (active, _) = handle.queue_library_scan(initial).unwrap().into_parts();
        let mut latest = None;
        for revision in 0..1_000 {
            let token = handle
                .begin_library_reindex_if_changed(vec![uri(&format!("file:///scan/{revision}/"))])
                .await
                .unwrap()
                .unwrap();
            assert!(handle.queue_library_scan(token.clone()).is_none());
            assert_eq!(handle.library_scan_occupancy(), (true, true));
            latest = Some(token);
        }
        let (current, request) = handle.finish_library_scan(&active).unwrap().into_parts();
        assert_eq!(request.library_paths, vec![uri("file:///scan/999/")]);
        assert!(handle.library_scan_is_current(&request.readiness));
        assert!(handle
            .snapshot()
            .diagnostic_readiness
            .is_current(&latest.unwrap()));
        assert_eq!(handle.library_scan_occupancy(), (true, false));
        assert!(handle.finish_library_scan(&active).is_none());
        assert_eq!(handle.library_scan_occupancy(), (true, false));
        assert!(handle.finish_library_scan(&current).is_none());
        assert_eq!(handle.library_scan_occupancy(), (false, false));
    }

    #[tokio::test]
    async fn foreign_scan_completions_and_close_cannot_start_pending_discovery() {
        let handle = ready();
        let other = ready();
        let root = vec![uri("file:///scan/initial/")];
        let initial = handle
            .begin_library_reindex_if_changed(root.clone())
            .await
            .unwrap()
            .unwrap();
        let foreign = other
            .begin_library_reindex_if_changed(root)
            .await
            .unwrap()
            .unwrap();
        assert!(handle.queue_library_scan(foreign.clone()).is_none());
        let (foreign_work, _) = other.queue_library_scan(foreign).unwrap().into_parts();
        let (active, _) = handle.queue_library_scan(initial).unwrap().into_parts();
        let pending = handle
            .begin_library_reindex_if_changed(vec![uri("file:///scan/latest/")])
            .await
            .unwrap()
            .unwrap();
        assert!(handle.queue_library_scan(pending.clone()).is_none());
        assert!(handle.finish_library_scan(&foreign_work).is_none());
        assert_eq!(handle.library_scan_occupancy(), (true, true));
        handle.close_session().await.unwrap();
        assert_eq!(handle.library_scan_occupancy(), (true, false));
        assert!(handle.queue_library_scan(pending).is_none());
        assert!(handle.finish_library_scan(&active).is_none());
        assert_eq!(handle.library_scan_occupancy(), (false, false));
        assert!(other.finish_library_scan(&foreign_work).is_none());
    }

    #[tokio::test]
    async fn discovery_releases_its_interval_without_waiting_for_semantic_publication() {
        let handle = ready();
        // Reserve semantic work without executing it, so awaiting publication here would
        // prevent the discovery driver from advancing to the next configuration.
        let _reserved = {
            let snapshot = handle.snapshot();
            let mut scheduler = handle.analysis_queue.lock().unwrap();
            scheduler.newest = Some(snapshot.session.inputs());
            let super::super::latest_work::Submission::Start(work) =
                scheduler.work.submit(snapshot)
            else {
                panic!("idle queue")
            };
            work
        };
        let initial = handle
            .begin_library_reindex_if_changed(vec![uri("file:///scan/initial/")])
            .await
            .unwrap()
            .unwrap();
        let (active, _) = handle
            .queue_library_scan(initial.clone())
            .unwrap()
            .into_parts();
        let admission = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            complete_library_scan(&handle, initial, Vec::new()),
        )
        .await
        .unwrap()
        .unwrap();
        assert!(matches!(admission, LibraryReindexCompletion::Admitted(..)));
        assert_eq!(
            handle.snapshot().session.analysis_status(),
            sysml_query::publication::AnalysisStatus::Updating
        );
        let latest = handle
            .begin_library_reindex_if_changed(vec![uri("file:///scan/latest/")])
            .await
            .unwrap()
            .unwrap();
        assert!(handle.queue_library_scan(latest).is_none());
        let (next, request) = handle.finish_library_scan(&active).unwrap().into_parts();
        assert!(handle.library_scan_is_current(&request.readiness));
        assert_eq!(request.library_paths, vec![uri("file:///scan/latest/")]);
        handle.close_session().await.unwrap();
        assert!(handle.finish_library_scan(&next).is_none());
    }

    #[tokio::test]
    async fn older_library_scan_cannot_mutate_a_newer_configuration_or_publication() {
        let handle = ready();
        let old = handle
            .begin_library_reindex_if_changed(vec![uri("file:///reindex/one/")])
            .await
            .unwrap()
            .unwrap();
        let old_document = uri("file:///reindex/one/model.sysml");
        let entries = scanned(&handle, &old_document, "package Old;");
        let current = handle
            .begin_library_reindex_if_changed(vec![uri("file:///reindex/two/")])
            .await
            .unwrap()
            .unwrap();
        let before = handle.snapshot();
        let inputs = before.session.inputs();
        let model = Arc::clone(before.session.current());
        let revision = before.semantic_revision;
        assert!(matches!(
            complete_library_scan(&handle, old, entries).await.unwrap(),
            LibraryReindexCompletion::Superseded
        ));
        let after = handle.snapshot();
        assert!(after.session.is_input_current(&inputs));
        assert!(Arc::ptr_eq(after.session.current(), &model));
        assert_eq!(after.semantic_revision, revision);
        assert!(!after.index.contains_key(&old_document));
        assert!(!after.diagnostic_readiness.announced());
        assert!(after.diagnostic_readiness.is_current(&current));
        let latest = uri("file:///reindex/two/model.sysml");
        let entries = scanned(&handle, &latest, "package Current;");
        assert!(matches!(
            complete_library_scan(&handle, current, entries)
                .await
                .unwrap(),
            LibraryReindexCompletion::Admitted(..)
        ));
        assert_eq!(
            handle
                .snapshot()
                .library_catalog
                .as_ref()
                .unwrap()
                .source_document(latest.as_str())
                .unwrap()
                .content(),
            "package Current;"
        );
    }

    #[tokio::test]
    async fn scan_admission_rejects_foreign_owners_and_closed_sessions() {
        let handle = ready();
        let root = uri("file:///reindex/library/");
        let current = handle
            .begin_library_reindex_if_changed(vec![root.clone()])
            .await
            .unwrap()
            .unwrap();
        let other = ready();
        let foreign = other
            .begin_library_reindex_if_changed(vec![root])
            .await
            .unwrap()
            .unwrap();
        let inputs = handle.snapshot().session.inputs();
        assert!(matches!(
            complete_library_scan(&handle, foreign, Vec::new())
                .await
                .unwrap(),
            LibraryReindexCompletion::Superseded
        ));
        assert!(handle.snapshot().session.is_input_current(&inputs));
        handle.close_session().await.unwrap();
        assert!(matches!(
            complete_library_scan(&handle, current, Vec::new())
                .await
                .unwrap(),
            LibraryReindexCompletion::Superseded
        ));
        assert_eq!(
            handle.snapshot().session.lifecycle(),
            sysml_query::publication::SessionLifecycle::Closed
        );
    }

    #[tokio::test]
    async fn a_saved_scan_merges_around_newer_open_editor_text_and_version() {
        let handle = ready();
        let root = uri("file:///reindex/library/");
        let current = handle
            .begin_library_reindex_if_changed(vec![root])
            .await
            .unwrap()
            .unwrap();
        let document = uri("file:///reindex/library/model.sysml");
        let entries = scanned(&handle, &document, "package Saved;");
        handle
            .store_document_text_fast(document.clone(), "package Unsaved;".into(), Some(17))
            .await
            .unwrap();
        assert!(matches!(
            complete_library_scan(&handle, current, entries)
                .await
                .unwrap(),
            LibraryReindexCompletion::Admitted(..)
        ));
        handle.rebuild_publication().await.unwrap();
        let snapshot = handle.snapshot();
        assert_eq!(snapshot.index[&document].content(), "package Unsaved;");
        assert_eq!(snapshot.index[&document].editor_version(), Some(17));
        assert_eq!(
            snapshot.published_index[&document].content(),
            "package Unsaved;"
        );
        assert_eq!(
            snapshot.session.analysis_status(),
            sysml_query::publication::AnalysisStatus::Current
        );
    }

    #[tokio::test]
    async fn edits_wait_for_the_current_library_sources_before_publishing() {
        let handle = ready();
        let workspace = uri("file:///reindex/workspace/model.sysml");
        handle
            .store_document_text_fast(workspace.clone(), "package Initial;".into(), Some(1))
            .await
            .unwrap();
        handle.rebuild_publication().await.unwrap();
        let previous = handle.snapshot();
        let model = Arc::clone(previous.session.current());
        let current = handle
            .begin_library_reindex_if_changed(vec![uri("file:///reindex/library/")])
            .await
            .unwrap()
            .unwrap();
        handle
            .store_document_text_fast(
                workspace.clone(),
                "package Edited { private import Library::*; }".into(),
                Some(2),
            )
            .await
            .unwrap();
        handle.request_rebuild();
        assert_eq!(handle.analysis_occupancy(), (0, 0));
        assert!(!handle.announce_readiness(current.clone()).await.unwrap());
        let pending = handle.snapshot();
        assert!(pending.diagnostic_readiness.source_inputs_pending());
        assert_eq!(
            pending.session.analysis_status(),
            sysml_query::publication::AnalysisStatus::Updating
        );
        assert!(Arc::ptr_eq(pending.session.current(), &model));
        assert_eq!(
            pending.published_index[&workspace].content(),
            "package Initial;"
        );
        let library = uri("file:///reindex/library/model.sysml");
        let entries = scanned(&handle, &library, "package Library;");
        complete_library_scan(&handle, current.clone(), entries)
            .await
            .unwrap();
        handle.rebuild_publication().await.unwrap();
        let complete = handle.snapshot();
        assert!(!complete.diagnostic_readiness.source_inputs_pending());
        assert_eq!(
            complete.published_index[&workspace].content(),
            "package Edited { private import Library::*; }"
        );
        assert!(complete.published_index.contains_key(&library));
        assert!(handle.announce_readiness(current.clone()).await.unwrap());
        let inputs = complete.session.inputs();
        assert!(matches!(
            complete_library_scan(&handle, current, Vec::new())
                .await
                .unwrap(),
            LibraryReindexCompletion::Superseded
        ));
        assert!(handle.snapshot().session.is_input_current(&inputs));
    }

    #[tokio::test]
    async fn failed_scan_stays_failed_across_edits_and_same_configuration_can_retry() {
        let handle = ready();
        let old = handle
            .begin_library_reindex_if_changed(vec![uri("file:///reindex/old/")])
            .await
            .unwrap()
            .unwrap();
        let root = uri("file:///reindex/current/");
        let current = handle
            .begin_library_reindex_if_changed(vec![root.clone()])
            .await
            .unwrap()
            .unwrap();
        let failure = || {
            PublicationBuildFailure::preparation_worker("injected library discovery worker failure")
        };
        assert!(!handle.fail_library_reindex(old, failure()).await.unwrap());
        assert!(handle.snapshot().publication_failure.is_none());
        let model = Arc::clone(handle.snapshot().session.current());
        assert!(handle
            .fail_library_reindex(current.clone(), failure())
            .await
            .unwrap());
        let workspace = uri("file:///reindex/workspace/model.sysml");
        handle
            .store_document_text_fast(workspace, "package Edited;".into(), Some(1))
            .await
            .unwrap();
        handle.request_rebuild();
        assert_eq!(handle.analysis_occupancy(), (0, 0));
        assert_eq!(
            handle.snapshot().session.analysis_status(),
            sysml_query::publication::AnalysisStatus::FailedRetained
        );
        assert!(Arc::ptr_eq(handle.snapshot().session.current(), &model));
        let report = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            handle.rebuild_publication_measured(),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(report.outcome, PublicationOutcome::Failed);
        assert!(report.phases.is_none());
        let retry = handle
            .begin_library_reindex_if_changed(vec![root])
            .await
            .unwrap()
            .unwrap();
        assert!(handle.snapshot().publication_failure.is_none());
        assert!(!handle
            .fail_library_reindex(current, failure())
            .await
            .unwrap());
        let library = uri("file:///reindex/current/model.sysml");
        let entries = scanned(&handle, &library, "package Recovered;");
        complete_library_scan(&handle, retry.clone(), entries)
            .await
            .unwrap();
        handle.rebuild_publication().await.unwrap();
        assert!(handle.announce_readiness(retry).await.unwrap());
        assert_eq!(
            handle.snapshot().session.analysis_status(),
            sysml_query::publication::AnalysisStatus::Current
        );
    }

    #[tokio::test]
    async fn removed_library_roots_keep_open_buffers_until_close_without_ghost_sources() {
        let old_root = uri("file:///reindex/old/");
        let open = uri("file:///reindex/old/open.sysml");
        let closed = uri("file:///reindex/old/closed.sysml");
        let mut state = ServerState::default();
        state.session.complete_startup();
        state.library_paths = vec![old_root];
        let mut entry = IndexEntry::for_test(&open, "package Unsaved;");
        entry.open_editor(Some(9));
        entry.workspace_source = false;
        state.index.insert(open.clone(), entry);
        let mut closed_entry = IndexEntry::for_test(&closed, "package Closed;");
        closed_entry.workspace_source = false;
        state.index.insert(closed.clone(), closed_entry);
        let handle = WorkspaceHandle::spawn(state);
        let current = handle
            .begin_library_reindex_if_changed(vec![uri("file:///reindex/new/")])
            .await
            .unwrap()
            .unwrap();
        let snapshot = handle.snapshot();
        assert!(!snapshot.index.contains_key(&closed));
        assert_eq!(snapshot.index[&open].content(), "package Unsaved;");
        assert_eq!(snapshot.index[&open].editor_version(), Some(9));
        assert_eq!(snapshot.index[&open].document.kind(), SourceKind::Library);
        let (documents, reported, _) = publication_inputs(&*snapshot);
        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].kind(), SourceKind::Library);
        assert_eq!(reported.as_slice(), &[Box::<str>::from(open.as_str())]);
        drop(snapshot);
        complete_library_scan(&handle, current, Vec::new())
            .await
            .unwrap();
        assert!(handle.set_document_open(open.clone(), false).await.unwrap());
        assert!(!handle.snapshot().index.contains_key(&open));
        handle.rebuild_publication().await.unwrap();
        assert!(!handle.snapshot().published_index.contains_key(&open));
    }
}

/// Immutable source-discovery inputs. The queue holds one active and one latest request.
pub(crate) struct LibraryScanRequest {
    pub(crate) library_paths: Vec<Url>,
    pub(crate) standard_library_paths: Vec<Url>,
    pub(crate) readiness: super::readiness::ReadinessToken,
    pub(crate) startup: Option<StartupScanInputs>,
    pub(crate) disk_revision: super::readiness::DiskRevisionToken,
}

pub(crate) struct ObservedFileChange {
    pub(crate) uri: Url,
    pub(crate) content: Option<String>,
    pub(crate) workspace_source: bool,
}

pub(crate) struct StartupScanInputs {
    pub(crate) workspace_roots: Vec<Url>,
    pub(crate) standard_library_paths: Vec<Url>,
}
pub(crate) type LibraryScanWork = super::latest_work::Work<LibraryScanRequest>;
pub(crate) use super::latest_work::WorkToken as LibraryScanWorkToken;

/// Only the scan for the current project/configuration epoch may admit source inputs.
pub(crate) enum LibraryReindexCompletion {
    Admitted(Vec<(Url, Option<String>)>),
    Superseded,
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
    library_scans: Arc<std::sync::Mutex<super::latest_work::LatestWork<LibraryScanRequest>>>,
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
            library_scans: Arc::default(),
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
            let current = self.snapshot();
            if !current.session.is_input_current(&expected) {
                return Ok(PublicationRebuildReport::superseded());
            }
            if current.diagnostic_readiness.source_inputs_pending()
                && current.publication_failure.is_some()
            {
                return Ok(PublicationRebuildReport::source_discovery_failed());
            }
            drop(current);
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
            if snapshot.diagnostic_readiness.source_inputs_pending() {
                // Edits remain desired inputs, but cannot publish without the configured
                // libraries. The successful scan schedules the latest complete source state.
                self.analysis_events.send_replace(AnalysisEvent::Requested);
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
        self.library_scans
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .close();
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
        let library_selection = prepared
            .as_ref()
            .map_or(std::time::Duration::ZERO, |inputs| inputs.library_selection);
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
            library_selection,
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
        follow_host_defaults: bool,
    ) -> Result<(), MutatePanicked> {
        self.actor
            .mutate(move |s| {
                s.workspace_roots = roots;
                s.library_paths = library_paths;
                s.standard_library_paths = standard_library_paths;
                s.standard_library_availability = standard_library_availability;
                s.library_paths_follow_host_defaults = follow_host_defaults;
                s.library_catalog = None;
                s.session.reset();
                s.diagnostic_readiness.reset(true);
            })
            .await
    }

    pub(crate) async fn begin_startup(
        &self,
    ) -> Result<Option<super::readiness::ReadinessToken>, MutatePanicked> {
        let token = self
            .actor
            .mutate_if_changed(|s| {
                if s.session.lifecycle() != sysml_query::publication::SessionLifecycle::Cold {
                    return Mutation::Unchanged(None);
                }
                s.session.begin_startup();
                s.diagnostic_readiness.begin_startup_sources();
                s.publication_failure = None;
                s.invalidate_inputs();
                Mutation::Changed(Some(s.diagnostic_readiness.token()))
            })
            .await?
            .value;
        if token.is_some() {
            self.request_rebuild();
        }
        Ok(token)
    }

    /// A loose project admits explicitly opened sources, not its whole parent directory. Its
    /// library inventory still has to cross the source barrier before the first publication.
    pub(crate) async fn begin_open_document_sources(
        &self,
    ) -> Result<Option<super::readiness::ReadinessToken>, MutatePanicked> {
        let token = self
            .actor
            .mutate_if_changed(|s| {
                if s.session.lifecycle() != sysml_query::publication::SessionLifecycle::Cold
                    || s.diagnostic_readiness.source_inputs_pending()
                {
                    return Mutation::Unchanged(None);
                }
                s.diagnostic_readiness.require_library_sources();
                s.publication_failure = None;
                s.invalidate_inputs();
                Mutation::Changed(Some(s.diagnostic_readiness.token()))
            })
            .await?
            .value;
        if token.is_some() {
            self.request_rebuild();
        }
        Ok(token)
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

    pub(crate) async fn begin_readiness_notification(
        &self,
        token: super::readiness::ReadinessToken,
    ) -> Result<bool, MutatePanicked> {
        self.actor
            .mutate_if_changed(move |s| {
                if !crate::session::state::supports_semantic_queries(s.session.lifecycle())
                    || (!s.diagnostic_readiness.needs_initial_notification()
                        && s.session.analysis_status()
                            != sysml_query::publication::AnalysisStatus::Current)
                {
                    return Mutation::Unchanged(false);
                }
                if s.diagnostic_readiness.begin_notification(&token) {
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
        expected: super::readiness::ReadinessToken,
        mut entries: Vec<ParsedScanEntry>,
        catalog: sysml_query::library::LibraryCatalog,
    ) -> Result<LibraryReindexCompletion, MutatePanicked> {
        let result = self
            .actor
            .mutate_if_changed(move |s| {
                if !s.diagnostic_readiness.is_current(&expected)
                    || !s.diagnostic_readiness.startup_sources_pending()
                    || s.diagnostic_readiness.disk_observations_pending()
                    || s.session.lifecycle() == sysml_query::publication::SessionLifecycle::Closed
                {
                    return Mutation::Unchanged(LibraryReindexCompletion::Superseded);
                }
                // Editor text is authoritative for open documents. A startup or manifest-change
                // rescan may have read an older saved revision from disk while didOpen/didChange
                // was already published; never replace that revision at the scan barrier.
                entries.retain(|entry| {
                    let uri = crate::common::util::normalize_file_uri(&entry.uri);
                    !s.is_editor_open(&uri)
                });
                let results =
                    crate::session::services::ingest_parsed_scan_entries_batch(s, entries);
                s.library_catalog = Some(catalog);
                assert!(s.diagnostic_readiness.admit_source_inputs(&expected));
                s.session.complete_startup();
                s.publication_failure = None;
                s.semantic_revision = s.semantic_revision.wrapping_add(1);
                s.invalidate_inputs();
                Mutation::Changed(LibraryReindexCompletion::Admitted(results))
            })
            .await?
            .value;
        if matches!(result, LibraryReindexCompletion::Admitted(..)) {
            self.request_rebuild();
        }
        Ok(result)
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
                    // Registry migration selected this URI for the new workspace owner. Its
                    // former project's library role and slot do not define the new scope.
                    entry.workspace_source = true;
                    let kind = if crate::common::util::uri_under_any_library(
                        &uri,
                        &state.standard_library_paths,
                    ) {
                        sysml_query::source::SourceKind::StandardLibrary
                    } else if crate::common::util::uri_under_any_library(&uri, &state.library_paths)
                    {
                        sysml_query::source::SourceKind::Library
                    } else {
                        sysml_query::source::SourceKind::Workspace
                    };
                    entry.document = entry.document.with_kind(kind);
                    state.index.insert(uri, entry);
                }
                state.invalidate_inputs();
            })
            .await
    }

    pub(crate) async fn admit_library_search(
        &self,
        expected: super::readiness::ReadinessToken,
        disk: super::readiness::DiskRevisionToken,
        entries: Vec<ParsedScanEntry>,
    ) -> Result<usize, MutatePanicked> {
        self.actor
            .mutate_if_changed(move |s| {
                if !s.diagnostic_readiness.is_search_current(&expected, &disk)
                    || s.session.lifecycle() == sysml_query::publication::SessionLifecycle::Closed
                {
                    return Mutation::Unchanged(0);
                }
                let indexed = crate::session::services::index_library_search_entries(s, entries);
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
                let configured_library =
                    crate::common::util::uri_under_any_library(&uri, &s.library_paths)
                        || crate::common::util::uri_under_any_library(
                            &uri,
                            &s.standard_library_paths,
                        );
                let library = configured_library
                    || s.index
                        .get(&uri)
                        .is_some_and(|entry| entry.document.kind().is_library());
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
                    if !open && !configured_library {
                        // The removed library stayed admitted solely for this editor interval.
                        crate::session::services::remove_document(s, &uri);
                    }
                    s.semantic_revision = s.semantic_revision.wrapping_add(1);
                    s.invalidate_inputs();
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
                state.invalidate_inputs();
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
                    state.invalidate_inputs();
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
                    s.invalidate_inputs();
                    Mutation::Changed(true)
                } else {
                    Mutation::Unchanged(false)
                }
            })
            .await
            .map(|outcome| outcome.value)
    }

    // --- did_change_watched_files --------------------------------------------------------

    pub(crate) async fn begin_filesystem_observation(&self) -> Result<(), MutatePanicked> {
        self.actor
            .mutate(|s| {
                s.diagnostic_readiness.begin_disk_observation();
                if s.diagnostic_readiness.source_inputs_pending() {
                    s.publication_failure = None;
                    s.invalidate_inputs();
                }
            })
            .await
    }

    pub(crate) async fn finish_filesystem_observation(
        &self,
    ) -> Result<Option<super::readiness::ReadinessToken>, MutatePanicked> {
        let token = self
            .actor
            .mutate(|s| {
                if s.diagnostic_readiness.finish_disk_observation()
                    && s.session.lifecycle() != sysml_query::publication::SessionLifecycle::Closed
                {
                    s.publication_failure = None;
                    s.invalidate_inputs();
                    Some(s.diagnostic_readiness.token())
                } else {
                    None
                }
            })
            .await?;
        if token.is_some() {
            self.request_rebuild();
        }
        Ok(token)
    }

    /// Ingest one watched-file batch as a single desired revision; the caller schedules analysis.
    /// Rename notifications commonly contain a delete and a create in the same batch.
    pub(crate) async fn apply_observed_file_changes(
        &self,
        changes: Vec<ObservedFileChange>,
    ) -> Result<(bool, Vec<String>), MutatePanicked> {
        let outcome = self
            .actor
            .mutate_if_changed(move |state| {
                let mut warnings = Vec::new();
                let mut changed = false;
                let mut catalog_changed = false;
                for ObservedFileChange {
                    uri,
                    content,
                    workspace_source,
                } in changes
                {
                    // The editor remains authoritative until didClose, including across saves,
                    // renames and a delayed disk-read completion.
                    if state.is_editor_open(&uri) {
                        continue;
                    }
                    if let Some(catalog) = &state.library_catalog {
                        let saved = catalog.source_document(uri.as_str());
                        let library = saved.is_some()
                            || crate::common::util::uri_under_any_library(
                                &uri,
                                &state.library_paths,
                            )
                            || crate::common::util::uri_under_any_library(
                                &uri,
                                &state.standard_library_paths,
                            );
                        if library {
                            catalog_changed |= match &content {
                                Some(content) => {
                                    saved.is_none_or(|saved| saved.content() != content)
                                }
                                None => saved.is_some(),
                            };
                        }
                    }
                    match content {
                        Some(content) => {
                            if state
                                .index
                                .get(&uri)
                                .is_some_and(|entry| entry.content() == content)
                            {
                                let entry = state.index.get_mut(&uri).unwrap();
                                if entry.workspace_source != workspace_source {
                                    entry.workspace_source = workspace_source;
                                    changed = true;
                                }
                                continue;
                            }
                            if let Some(warning) =
                                crate::session::services::store_document_text_fast(
                                    state, &uri, content,
                                )
                            {
                                warnings.push(warning);
                            }
                            state.index.get_mut(&uri).unwrap().workspace_source = workspace_source;
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
                if changed || catalog_changed {
                    if catalog_changed {
                        state.diagnostic_readiness.require_library_sources();
                    }
                    state.semantic_revision = state.semantic_revision.wrapping_add(1);
                    state.invalidate_inputs();
                    state.services.syntax.retain(
                        state
                            .index
                            .values()
                            .filter_map(|entry| entry.parsed.as_ref().map(ParsedSource::digest)),
                    );
                    Mutation::Changed(warnings)
                } else {
                    Mutation::Unchanged(warnings)
                }
            })
            .await?;
        Ok((outcome.published, outcome.value))
    }

    #[cfg(test)]
    async fn apply_watched_file_changes(
        &self,
        changes: Vec<(Url, Option<String>)>,
    ) -> Result<(bool, Vec<String>), MutatePanicked> {
        self.apply_observed_file_changes(
            changes
                .into_iter()
                .map(|(uri, content)| ObservedFileChange {
                    uri,
                    content,
                    workspace_source: true,
                })
                .collect(),
        )
        .await
    }

    // --- did_change_configuration (library reindex) ---------------------------------------

    pub(crate) fn source_epoch_is_current(&self, token: &super::readiness::ReadinessToken) -> bool {
        let snapshot = self.snapshot();
        snapshot.diagnostic_readiness.is_current(token)
            && snapshot.session.lifecycle() != sysml_query::publication::SessionLifecycle::Closed
    }

    pub(crate) fn library_search_is_current(
        &self,
        token: &super::readiness::ReadinessToken,
        disk: &super::readiness::DiskRevisionToken,
    ) -> bool {
        let snapshot = self.snapshot();
        snapshot.diagnostic_readiness.is_search_current(token, disk)
            && snapshot.session.lifecycle() != sysml_query::publication::SessionLifecycle::Closed
    }

    pub(crate) fn library_scan_is_current(&self, token: &super::readiness::ReadinessToken) -> bool {
        let snapshot = self.snapshot();
        snapshot.diagnostic_readiness.is_current(token)
            && snapshot.diagnostic_readiness.source_inputs_pending()
            && !snapshot.diagnostic_readiness.disk_observations_pending()
            && snapshot.session.lifecycle() != sysml_query::publication::SessionLifecycle::Closed
    }

    pub(crate) fn queue_library_scan(
        &self,
        token: super::readiness::ReadinessToken,
    ) -> Option<LibraryScanWork> {
        use super::latest_work::Submission;
        let mut queue = self
            .library_scans
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let snapshot = self.snapshot();
        if queue.is_closed()
            || !snapshot.diagnostic_readiness.is_current(&token)
            || !snapshot.diagnostic_readiness.source_inputs_pending()
            || snapshot.session.lifecycle() == sysml_query::publication::SessionLifecycle::Closed
        {
            return None;
        }
        match queue.submit(LibraryScanRequest {
            library_paths: snapshot.library_paths.clone(),
            standard_library_paths: snapshot.standard_library_paths.clone(),
            readiness: token,
            disk_revision: snapshot.diagnostic_readiness.disk_revision(),
            startup: snapshot
                .diagnostic_readiness
                .startup_sources_pending()
                .then(|| StartupScanInputs {
                    workspace_roots: snapshot.workspace_roots.clone(),
                    standard_library_paths: snapshot.standard_library_paths.clone(),
                }),
        }) {
            Submission::Start(work) => Some(work),
            Submission::Queued | Submission::Closed => None,
        }
    }

    pub(crate) fn finish_library_scan(
        &self,
        token: &LibraryScanWorkToken,
    ) -> Option<LibraryScanWork> {
        self.library_scans
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .complete(token)
            .ok()
            .flatten()
    }

    #[cfg(test)]
    fn library_scan_occupancy(&self) -> (bool, bool) {
        self.library_scans.lock().unwrap().occupancy()
    }

    pub(crate) async fn begin_library_reindex_if_changed(
        &self,
        new_library_paths: Vec<Url>,
    ) -> Result<Option<super::readiness::ReadinessToken>, MutatePanicked> {
        let token = self
            .actor
            .mutate_if_changed(move |s| {
                if s.session.lifecycle() == sysml_query::publication::SessionLifecycle::Closed {
                    return Mutation::Unchanged(None);
                }
                let new_library_paths = if s.library_paths_follow_host_defaults {
                    new_library_paths
                } else {
                    s.library_paths.clone()
                };
                let old = std::mem::take(&mut s.library_paths);
                let retry_failed = s.diagnostic_readiness.source_inputs_pending()
                    && s.publication_failure.is_some();
                if new_library_paths == old && !retry_failed {
                    s.library_paths = old;
                    Mutation::Unchanged(None)
                } else {
                    s.library_paths = new_library_paths;
                    let _ = crate::session::services::clear_documents_under_roots(s, &old);
                    s.diagnostic_readiness.reset(false);
                    s.publication_failure = None;
                    s.invalidate_inputs();
                    Mutation::Changed(Some(s.diagnostic_readiness.token()))
                }
            })
            .await
            .map(|outcome| outcome.value)?;
        if token.is_some() {
            self.request_rebuild();
        }
        Ok(token)
    }

    pub(crate) async fn complete_library_reindex(
        &self,
        expected: super::readiness::ReadinessToken,
        catalog: sysml_query::library::LibraryCatalog,
    ) -> Result<LibraryReindexCompletion, MutatePanicked> {
        let result = self
            .actor
            .mutate_if_changed(move |s| {
                if !s.diagnostic_readiness.is_current(&expected)
                    || !s.diagnostic_readiness.source_inputs_pending()
                    || s.diagnostic_readiness.startup_sources_pending()
                    || s.diagnostic_readiness.disk_observations_pending()
                    || s.session.lifecycle() == sysml_query::publication::SessionLifecycle::Closed
                {
                    return Mutation::Unchanged(LibraryReindexCompletion::Superseded);
                }
                // The catalogue is immutable input. Every publication selects its own closure
                // from the latest authored source; saved scans never overwrite editor buffers.
                s.library_catalog = Some(catalog);
                assert!(s.diagnostic_readiness.admit_source_inputs(&expected));
                if s.session.lifecycle() == sysml_query::publication::SessionLifecycle::Cold {
                    s.session.complete_startup();
                }
                s.publication_failure = None;
                s.semantic_revision = s.semantic_revision.wrapping_add(1);
                s.invalidate_inputs();
                Mutation::Changed(LibraryReindexCompletion::Admitted(Vec::new()))
            })
            .await?
            .value;
        if matches!(result, LibraryReindexCompletion::Admitted(..)) {
            // Release the discovery interval before awaiting semantic construction: the next
            // configuration's sources may be a prerequisite of that construction.
            self.request_rebuild();
        }
        Ok(result)
    }

    pub(crate) async fn fail_library_reindex(
        &self,
        expected: super::readiness::ReadinessToken,
        failure: PublicationBuildFailure,
    ) -> Result<bool, MutatePanicked> {
        let changed = self
            .actor
            .mutate_if_changed(move |state| {
                if !state.diagnostic_readiness.is_current(&expected)
                    || !state.diagnostic_readiness.source_inputs_pending()
                    || state.session.lifecycle()
                        == sysml_query::publication::SessionLifecycle::Closed
                {
                    return Mutation::Unchanged(false);
                }
                state.session.finish_preparation_failure();
                state.publication_failure = Some(Arc::new(failure));
                Mutation::Changed(true)
            })
            .await?
            .value;
        if changed {
            let scheduler = self
                .analysis_queue
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if !scheduler.work.is_closed() {
                self.analysis_events.send_replace(AnalysisEvent::Requested);
            }
        }
        Ok(changed)
    }

    // --- cache management ---------------------------------------------------------------

    /// Clears indexed documents, symbols, and the current immutable publication.
    pub(crate) async fn clear_cache_state(&self) -> Result<(usize, usize), MutatePanicked> {
        let counts = self
            .actor
            .mutate_if_changed(|s| {
                let counts = crate::lsp_runtime::custom::clear_document_store_state_full(s);
                s.semantic_revision = s.semantic_revision.wrapping_add(1);
                s.invalidate_inputs();
                Mutation::Changed(counts)
            })
            .await
            .map(|outcome| outcome.value)?;
        self.rebuild_publication().await?;
        Ok(counts)
    }
}
