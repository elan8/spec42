use std::sync::Arc;
use std::time::Instant;

use tower_lsp::lsp_types::{Diagnostic, Url};
use tower_lsp::Client;
use tracing::info;

use crate::analysis::diagnostics_core;
use crate::common::util;
use crate::semantic::SemanticGraph;
use crate::workspace::state::supports_semantic_queries;
use crate::workspace::{RuntimeConfig, WorkspaceHandle};

fn perf_logging_enabled(runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>) -> bool {
    runtime_config
        .get()
        .expect("initialize precedes all other LSP requests")
        .perf_logging_enabled
}

pub(crate) async fn publish_document_diagnostics(
    client: &Client,
    handle: &WorkspaceHandle,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
    uri: Url,
    text: &str,
) {
    let started_at = Instant::now();
    let snap = handle.snapshot();
    if util::uri_under_any_library(&uri, &snap.library_paths) {
        return;
    }
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
    let diagnostics =
        collect_diagnostics_for_document(&snap.semantic_graph, &snap.library_paths, &uri, text)
            .await;
    if perf_logging_enabled(runtime_config) {
        info!(
            event = "diagnostics:document",
            uri = %uri,
            count = diagnostics.len(),
            elapsed_ms = started_at.elapsed().as_millis() as u64
        );
    }
    client.publish_diagnostics(uri, diagnostics, None).await;
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
    let docs: Vec<(Url, String)> = if let Some(targets) = target_uris {
        targets
            .iter()
            .filter_map(|uri| {
                snap.index
                    .get(uri)
                    .map(|entry| (uri.clone(), entry.content.clone()))
            })
            .collect()
    } else {
        snap.index
            .iter()
            .filter(|(uri, _)| !util::uri_under_any_library(uri, &snap.library_paths))
            .map(|(uri, entry)| (uri.clone(), entry.content.clone()))
            .collect()
    };

    let doc_count = docs.len();
    let mut published_count = 0usize;
    let mut diagnostic_count = 0usize;

    let mut join_set = tokio::task::JoinSet::new();
    for (uri, text) in docs {
        let graph = snap.semantic_graph.clone();
        let library_paths = snap.library_paths.clone();
        let client = client.clone();
        join_set.spawn(async move {
            let diagnostics =
                collect_diagnostics_for_document(&graph, &library_paths, &uri, &text).await;
            let count = diagnostics.len();
            client.publish_diagnostics(uri, diagnostics, None).await;
            count
        });
    }

    while let Some(res) = join_set.join_next().await {
        if let Ok(count) = res {
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

/// Computes diagnostics for a single document from state the caller already captured — no
/// `handle.snapshot()` call here. This is deliberate: every document diagnosed within one
/// `publish_workspace_diagnostics` call (including its parallel per-document tasks) must see
/// the exact same graph, otherwise a concurrent relink landing mid-flight could make different
/// documents in the same publish operation disagree about what state they were diagnosed
/// against.
async fn collect_diagnostics_for_document(
    graph: &SemanticGraph,
    library_paths: &[Url],
    uri: &Url,
    text: &str,
) -> Vec<Diagnostic> {
    let uri_norm = util::normalize_file_uri(uri);
    let graph = graph.clone();
    let library_paths = library_paths.to_vec();
    let text = text.to_owned();
    tokio::task::spawn_blocking(move || {
        diagnostics_core::collect_document_diagnostics(
            &graph,
            &library_paths,
            &uri_norm,
            &text,
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
        assert_eq!(snap.session.lifecycle(), workspace::SessionLifecycle::Ready);

        // A concurrent edit to some other document schedules a relink, flipping the *live*
        // session to Reindexing. Diagnostics for a document diagnosed against `snap` must not
        // observe this — that's the whole point of consolidating to a single snapshot capture.
        handle
            .schedule_relink_if_ready()
            .await
            .expect("actor mutate should not panic");

        assert_eq!(
            handle.snapshot().session.lifecycle(),
            workspace::SessionLifecycle::Reindexing,
            "sanity check: the live session did move on"
        );
        assert_eq!(
            snap.session.lifecycle(),
            workspace::SessionLifecycle::Ready,
            "a snapshot captured before a concurrent relink must stay Ready — proving it's \
             immune to a later, independent read observing Reindexing"
        );
    }
}
