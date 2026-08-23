use std::sync::Arc;
use std::time::Instant;

use tower_lsp::lsp_types::{Diagnostic, Url};
use tower_lsp::Client;
use tracing::info;

use crate::analysis::diagnostics_core;
use crate::common::util;
use sysml_query::publication::PublicationToken;
use sysml_query::resolved_slice::PublishedModel;

use crate::workspace::state::supports_semantic_queries;
use crate::workspace::{RuntimeConfig, WorkspaceHandle};

fn perf_logging_enabled(runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>) -> bool {
    runtime_config
        .get()
        .expect("initialize precedes all other LSP requests")
        .perf_logging_enabled
}

/// `spec42.development.diagnoseLibraryPaths` — development-only opt-in to include library
/// paths in the workspace-wide diagnostics sweep. See that setting's description and the
/// comment on the exclusion below.
fn diagnose_library_paths_enabled(
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
) -> bool {
    runtime_config
        .get()
        .expect("initialize precedes all other LSP requests")
        .diagnose_library_paths
}

pub(crate) async fn publish_document_diagnostics(
    client: &Client,
    handle: &WorkspaceHandle,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
    uri: Url,
) {
    let started_at = Instant::now();
    let snap = handle.snapshot();
    // Unlike `publish_workspace_diagnostics`'s debounced O(project files) republish pass
    // (which excludes library paths to bound its cost — see
    // docs/engineering/PERFORMANCE-GUARDRAILS.md), this only diagnoses the single document
    // that was just opened/changed. That cost is the same regardless of library
    // classification, so there's no performance reason to suppress it here — and doing so
    // meant editing a library file directly (e.g. via `spec42.kparLibraryPaths` local-dev
    // overrides) never showed diagnostics at all, even though hover still flagged
    // unresolved references for the same file.
    if !supports_semantic_queries(snap.session.lifecycle()) {
        if perf_logging_enabled(runtime_config) {
            info!(
                event = "diagnostics:document:deferred",
                uri = %uri,
                elapsed_ms = started_at.elapsed().as_millis() as u64
            );
        }
        return;
    }
    // Captured before the work starts and rechecked immediately before publishing: holding the
    // publication by `Arc` keeps this computation internally coherent, but says nothing about
    // whether it is still *current*. A slower computation for an older edit would otherwise land
    // after a newer one and leave the editor showing diagnostics for text the author has replaced.
    let publication = snap.session.publication();
    let diagnostics =
        collect_diagnostics_for_document(Some(Arc::clone(snap.session.current())), &uri).await;
    if !publish_if_current(client, handle, publication, uri.clone(), diagnostics).await {
        if perf_logging_enabled(runtime_config) {
            info!(
                event = "diagnostics:document:superseded",
                uri = %uri,
                elapsed_ms = started_at.elapsed().as_millis() as u64
            );
        }
        return;
    }
    if perf_logging_enabled(runtime_config) {
        info!(
            event = "diagnostics:document",
            uri = %uri,
            elapsed_ms = started_at.elapsed().as_millis() as u64
        );
    }
}

/// Whether diagnostics computed against `publication` may still be published.
///
/// Reads the *live* session rather than the snapshot the computation captured -- that is the whole
/// point. Holding the publication by `Arc` keeps a computation internally coherent; it says nothing
/// about whether the model state it describes is still the one the author is looking at.
///
/// Separate from the publish itself so the decision is testable without a `Client`: the publish is
/// a side effect, this is the rule.
fn may_publish(handle: &WorkspaceHandle, publication: PublicationToken) -> bool {
    handle
        .snapshot()
        .session
        .is_publication_current(&publication)
}

/// Publishes `diagnostics` only while `publication` still names the live session's state.
///
/// The check happens immediately before the publish, so the window between them carries no
/// `await`. Returns whether the diagnostics were published.
async fn publish_if_current(
    client: &Client,
    handle: &WorkspaceHandle,
    publication: PublicationToken,
    uri: Url,
    diagnostics: Vec<Diagnostic>,
) -> bool {
    if !may_publish(handle, publication) {
        return false;
    }
    client.publish_diagnostics(uri, diagnostics, None).await;
    true
}

pub(crate) async fn publish_workspace_diagnostics(
    client: &Client,
    handle: &WorkspaceHandle,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
    target_uris: Option<&[Url]>,
) {
    let started_at = Instant::now();
    // Single snapshot read for this entire operation — every document diagnosed below
    // (including each parallel `JoinSet` task) shares this exact graph/lifecycle, so a
    // concurrent relink landing mid-flight can't make different documents in the same publish
    // call disagree about what state they were diagnosed against.
    let snap = handle.snapshot();
    if !supports_semantic_queries(snap.session.lifecycle()) {
        if perf_logging_enabled(runtime_config) {
            info!(
                event = "diagnostics:workspace:deferred",
                target_uris = target_uris.map(|uris| uris.len()).unwrap_or(0),
                elapsed_ms = started_at.elapsed().as_millis() as u64
            );
        }
        return;
    }
    let docs: Vec<Url> = if let Some(targets) = target_uris {
        targets
            .iter()
            .filter(|uri| snap.index.contains_key(*uri))
            .cloned()
            .collect()
    } else if diagnose_library_paths_enabled(runtime_config) {
        // `spec42.development.diagnoseLibraryPaths` opt-in: include library paths anyway,
        // trading the performance guardrail below for full coverage while developing or
        // debugging a library through a local override.
        snap.index.keys().cloned().collect()
    } else {
        // Excludes library paths deliberately — this pass is O(project files) and runs on a
        // debounce after every edit; including the bundled standard library and any configured
        // KPAR libraries here would make every keystroke revalidate the whole library corpus.
        // See docs/engineering/PERFORMANCE-GUARDRAILS.md. `publish_document_diagnostics` still
        // diagnoses individual library files when they're actually opened/edited (no exclusion
        // there — see its comment), so this only affects the *background* cross-file sweep.
        snap.index
            .keys()
            .filter(|uri| !util::uri_under_any_library(uri, &snap.library_paths))
            .cloned()
            .collect()
    };

    let doc_count = docs.len();
    let mut published_count = 0usize;
    let mut diagnostic_count = 0usize;

    // One captured publication for the whole sweep, rechecked inside each task: a relink landing
    // mid-flight supersedes every task still running, and none of them may publish for the model
    // state the author has already moved past.
    let publication = snap.session.publication();
    let mut join_set = tokio::task::JoinSet::new();
    for uri in docs {
        let model = Some(Arc::clone(snap.session.current()));
        let client = client.clone();
        let handle = handle.clone();
        join_set.spawn(async move {
            let diagnostics = collect_diagnostics_for_document(model, &uri).await;
            let count = diagnostics.len();
            let published =
                publish_if_current(&client, &handle, publication, uri, diagnostics).await;
            (published, count)
        });
    }

    while let Some(res) = join_set.join_next().await {
        if let Ok((true, count)) = res {
            diagnostic_count += count;
            published_count += 1;
        }
    }
    if perf_logging_enabled(runtime_config) {
        info!(
            event = "diagnostics:workspace",
            target_uris = target_uris.map(|uris| uris.len()).unwrap_or(0),
            published_docs = published_count,
            discovered_docs = doc_count,
            diagnostics = diagnostic_count,
            elapsed_ms = started_at.elapsed().as_millis() as u64
        );
    }
}

/// Computes diagnostics for a single document from the publication the caller already captured --
/// no `handle.snapshot()` call here. This is deliberate: every document diagnosed within one
/// `publish_workspace_diagnostics` call (including its parallel per-document tasks) must read the
/// same publication, otherwise a concurrent rebuild landing mid-flight could make different
/// documents in the same publish operation disagree about what model state they describe. Holding
/// the publication by `Arc` is what makes that guarantee hold across the await: the captured
/// publication is immutable and cannot be superseded underneath a task.
async fn collect_diagnostics_for_document(
    model: Option<Arc<PublishedModel>>,
    uri: &Url,
) -> Vec<Diagnostic> {
    let uri_norm = util::normalize_file_uri(uri);
    tokio::task::spawn_blocking(move || {
        diagnostics_core::collect_document_diagnostics(
            model.as_deref(),
            &uri_norm,
            diagnostics_core::lsp_reporting(),
            diagnostics_core::lsp_postprocess_options(),
        )
    })
    .await
    .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `publish_workspace_diagnostics`/`publish_document_diagnostics` now capture
    /// `handle.snapshot()` exactly once and derive the graph/lifecycle from that single
    /// capture — this is what guarantees every document diagnosed within one publish call
    /// (including its parallel per-document `JoinSet` tasks) sees identical state, even if a
    /// concurrent relink (from an unrelated edit) lands mid-flight. This test proves the
    /// property the fix depends on: a captured snapshot is a frozen point-in-time read that a
    /// later mutation cannot retroactively change.
    #[tokio::test]
    async fn captured_snapshot_is_immune_to_a_concurrent_relink_landing_afterward() {
        let handle = WorkspaceHandle::spawn(crate::workspace::state::ServerState::default());
        handle
            .complete_startup()
            .await
            .expect("actor mutate should not panic");

        let snap = handle.snapshot();
        assert_eq!(
            snap.session.lifecycle(),
            sysml_query::publication::SessionLifecycle::Ready
        );

        // A concurrent edit to some other document schedules a relink, flipping the *live*
        // session to Reindexing. Diagnostics for a document diagnosed against `snap` must not
        // observe this — that's the whole point of consolidating to a single snapshot capture.
        handle
            .schedule_relink_if_ready()
            .await
            .expect("actor mutate should not panic");

        assert_eq!(
            handle.snapshot().session.lifecycle(),
            sysml_query::publication::SessionLifecycle::Reindexing,
            "sanity check: the live session did move on"
        );
        assert_eq!(
            snap.session.lifecycle(),
            sysml_query::publication::SessionLifecycle::Ready,
            "a snapshot captured before a concurrent relink must stay Ready — proving it's \
             immune to a later, independent read observing Reindexing"
        );
    }

    /// Diagnostics computed for a superseded publication must not reach the editor.
    ///
    /// The captured snapshot staying coherent is necessary but not sufficient: a slower
    /// computation for an older edit is still *publishable* unless something checks whether it is
    /// current. This exercises that check, so removing it fails here.
    #[tokio::test]
    async fn diagnostics_for_a_superseded_publication_are_not_published() {
        let handle = WorkspaceHandle::spawn(crate::workspace::state::ServerState::default());
        handle
            .complete_startup()
            .await
            .expect("actor mutate should not panic");

        let captured = handle.snapshot().session.publication();
        assert!(
            may_publish(&handle, captured),
            "work captured against the live publication may publish"
        );

        // Any invalidating operation supersedes work already in flight.
        handle
            .schedule_relink_if_ready()
            .await
            .expect("actor mutate should not panic");

        assert!(
            !may_publish(&handle, captured),
            "an older computation must not publish over the newer edit that superseded it"
        );
    }

    /// A token from another session may never publish, whichever version it happens to carry.
    ///
    /// Version numbers are per-session counters, so two sessions produce colliding ones. Without
    /// the owner in the token, a stale result from one workspace could publish into another.
    #[tokio::test]
    async fn diagnostics_from_another_session_are_never_published() {
        let first = WorkspaceHandle::spawn(crate::workspace::state::ServerState::default());
        let second = WorkspaceHandle::spawn(crate::workspace::state::ServerState::default());
        let foreign = first.snapshot().session.publication();

        assert!(!may_publish(&second, foreign));
    }
}
