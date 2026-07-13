use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use tower_lsp::lsp_types::{notification::Notification, *};
use tower_lsp::Client;
use tracing::{info, warn};

use crate::common::util;
use crate::host::config::Spec42Config;
use crate::views::dto::SemanticIndexReadyNotificationDto;
use crate::workspace::state::ServerState;
use crate::workspace::{parse_scanned_entries, scan_sysml_files, RuntimeConfig, WorkspaceHandle};
use workspace_session::{RelinkToken, TracksRelink};

use super::capabilities::server_capabilities;
use super::diagnostics::{publish_document_diagnostics, publish_workspace_diagnostics};
use super::lifecycle::{scan_roots, workspace_roots_from_initialize};

static WORKSPACE_DIAGNOSTICS_DEBOUNCE_GEN: AtomicU64 = AtomicU64::new(0);
const WORKSPACE_DIAGNOSTICS_DEBOUNCE_MS: u64 = 450;
const SEMANTIC_RELINK_DEBOUNCE_MS: u64 = 700;

fn schedule_workspace_diagnostics_republish(
    client: &Client,
    handle: &WorkspaceHandle,
    config: &Arc<Spec42Config>,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
) {
    let generation = WORKSPACE_DIAGNOSTICS_DEBOUNCE_GEN.fetch_add(1, Ordering::SeqCst) + 1;
    let client = client.clone();
    let handle = handle.clone();
    let config = Arc::clone(config);
    let runtime_config = Arc::clone(runtime_config);
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(WORKSPACE_DIAGNOSTICS_DEBOUNCE_MS)).await;
        if WORKSPACE_DIAGNOSTICS_DEBOUNCE_GEN.load(Ordering::SeqCst) != generation {
            return;
        }
        let lifecycle = handle.snapshot().session.lifecycle();
        if !crate::workspace::state::supports_semantic_queries(lifecycle) {
            return;
        }
        publish_workspace_diagnostics(&client, &handle, &config, &runtime_config, None).await;
    });
}

/// Schedules an async semantic relink with a debounce (see `SEMANTIC_RELINK_DEBOUNCE_MS`).
///
/// `token` is issued by [`workspace::WorkspaceSession::schedule_relink`] and
/// encapsulates the current relink generation and snapshot version.
/// Only the relink task whose token is still current when the debounce
/// fires will run; all earlier tasks self-cancel via
/// [`workspace_session::TracksRelink::is_token_current`].
fn schedule_semantic_relink_after_change(
    client: &Client,
    handle: &WorkspaceHandle,
    config: &Arc<Spec42Config>,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
    changed_uri: Url,
    token: RelinkToken,
) {
    let client = client.clone();
    let handle = handle.clone();
    let config = Arc::clone(config);
    let runtime_config = Arc::clone(runtime_config);
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(SEMANTIC_RELINK_DEBOUNCE_MS)).await;

        let snapshot = {
            let snap = handle.snapshot();
            if !snap.is_token_current(&token) {
                return;
            }
            (
                snap.index.clone(),
                snap.library_paths.clone(),
                // Library files are not stored in the index when loaded from the
                // graph cache (cache hit path). Pass the library graph snapshot so
                // library types survive the workspace rebuild regardless of whether
                // library_paths is empty or not.
                snap.library_graph_snapshot.clone(),
            )
        };
        let (index, library_paths, base_graph) = snapshot;
        let perf_logging_enabled = runtime_config
            .get()
            .expect("initialize precedes all other LSP requests")
            .perf_logging_enabled;
        let library_snapshot_uris = base_graph.as_ref().map(|g| g.all_uris().len()).unwrap_or(0);
        let relink_start = Instant::now();
        let staged = tokio::task::spawn_blocking(move || {
            // Wave 1: structural relink only (`evaluate: false`) — publish diagnostics from
            // this as fast as possible. Expression evaluation runs separately afterward, as
            // Wave 2 (`schedule_expression_evaluation`), so a slow whole-graph evaluation pass
            // never delays the near-instant structural feedback (unresolved references, etc.)
            // a live edit should get. See Track C in `docs/engineering`.
            crate::workspace::rebuild_semantic_graph_staged(
                &index,
                &library_paths,
                base_graph,
                false,
            )
        })
        .await;
        let Ok((new_graph, new_symbols, relink_metrics)) = staged else {
            client
                .log_message(
                    MessageType::WARNING,
                    "Async semantic relink failed before completion.",
                )
                .await;
            return;
        };

        // Compute diagnostics URIs from the locally-known (pre-commit) index/library_paths.
        // This function only ever reads raw source/parsed data, never the semantic graph, so
        // the pre-relink snapshot's index is exactly as good as a post-commit read would be.
        let snap_for_diag = handle.snapshot();
        let mut diag_uris = crate::workspace::import_graph::workspace_uris_importing_declarations_from(
            &snap_for_diag.index,
            &snap_for_diag.library_paths,
            &changed_uri,
        );
        drop(snap_for_diag);
        // Always include the changed file — it was skipped during the fast
        // graph update and needs diagnostics from the fully-resolved graph.
        if !diag_uris.contains(&changed_uri) {
            diag_uris.push(changed_uri.clone());
        }

        // `report_relink_result` is now synchronized via `mutate`: awaiting it guarantees the
        // committed graph/lifecycle are visible to any subsequent `handle.snapshot()` call,
        // closing the race where diagnostics collection could previously read a stale
        // (pre-commit) lifecycle and wrongly suppress transient-startup diagnostic codes.
        let committed = handle
            .report_relink_result(token, new_graph, new_symbols)
            .await
            .unwrap_or(false);

        if !committed {
            // Superseded by a newer relink token — that newer relink is already in flight and
            // will publish its own (fresher) diagnostics, so publishing here would just
            // redundantly republish stale results.
            return;
        }

        publish_workspace_diagnostics(&client, &handle, &config, &runtime_config, Some(&diag_uris))
            .await;

        // Wave 2: evaluate expressions in the background against the structural graph just
        // committed, and republish diagnostics again once that lands (e.g.
        // `analysis_constraint_failed`, which depends on evaluation having run). Read the
        // version *after* `report_relink_result` above so a superseding edit that arrives
        // between now and Wave 2's debounce firing is correctly detected as stale.
        let post_relink_version = handle.snapshot().session.version();
        schedule_expression_evaluation(
            &client,
            &handle,
            &config,
            &runtime_config,
            post_relink_version,
        );

        log_perf(
            &client,
            perf_logging_enabled,
            "backend:asyncSemanticRelink",
            vec![
                ("uri", format!("{:?}", changed_uri.as_str())),
                ("generation", token.generation().to_string()),
                ("librarySnapshotUris", library_snapshot_uris.to_string()),
                ("relinkTotalMs", relink_metrics.total_ms.to_string()),
                (
                    "relinkRebuildGraphsMs",
                    relink_metrics.rebuild_graphs_ms.to_string(),
                ),
                (
                    "relinkCrossDocumentEdgesMs",
                    relink_metrics.cross_document_edges_ms.to_string(),
                ),
                (
                    "relinkCrossEdgeResolutionMs",
                    relink_metrics.cross_edge_resolution_ms.to_string(),
                ),
                (
                    "relinkWorkspaceRelationshipLinkingMs",
                    relink_metrics.workspace_relationship_linking_ms.to_string(),
                ),
                (
                    "relinkPendingRelationshipResolutionMs",
                    relink_metrics
                        .pending_relationship_resolution_ms
                        .to_string(),
                ),
                (
                    "relinkExpressionEvaluationMs",
                    relink_metrics.expression_evaluation_ms.to_string(),
                ),
                (
                    "relinkRefreshSymbolsMs",
                    relink_metrics.refresh_symbols_ms.to_string(),
                ),
                ("diagUrisCount", diag_uris.len().to_string()),
                ("elapsedMs", relink_start.elapsed().as_millis().to_string()),
            ],
        )
        .await;
    });
}

/// "Wave 2" of the two-wave diagnostics split (see Track C design notes): runs expression
/// evaluation against the structural graph Wave 1 (`schedule_semantic_relink_after_change`)
/// just committed, debounced by `WORKSPACE_DIAGNOSTICS_DEBOUNCE_MS` (reusing the same constant
/// this file already uses elsewhere for "let things settle before doing more work" — not a new
/// number). `expected_version` is the session version right after Wave 1 committed; checked
/// both before starting evaluation (skip the work entirely if already superseded) and again by
/// `report_evaluation_result`'s own version gate at commit time (skip publishing a stale
/// result if a newer edit landed while evaluation was running).
///
/// Publishes workspace-wide (`target_uris: None`), not scoped to the edited file: unlike
/// Wave 1's structural relink, `evaluate_workspace_graph` evaluates expressions across the
/// *entire* graph with no per-file scoping, so evaluation-derived diagnostics
/// (`analysis_constraint_failed`, `analysis_evaluation_unresolved`, etc.) can change on any file
/// in the workspace, not just the one that was edited. Publishing only the edited file here
/// left every other affected file stuck showing whatever Wave 1 last published for it
/// (structural diagnostics with no evaluation attribute at all) until that specific file was
/// itself edited — i.e. evaluation diagnostics could get cleared by an edit elsewhere and never
/// come back.
fn schedule_expression_evaluation(
    client: &Client,
    handle: &WorkspaceHandle,
    config: &Arc<Spec42Config>,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
    expected_version: u64,
) {
    let client = client.clone();
    let handle = handle.clone();
    let config = Arc::clone(config);
    let runtime_config = Arc::clone(runtime_config);
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(WORKSPACE_DIAGNOSTICS_DEBOUNCE_MS)).await;

        if handle.snapshot().session.version() != expected_version {
            return; // superseded before evaluation even started — don't waste the work
        }
        let graph = handle.snapshot().semantic_graph.clone(); // cheap Arc clone

        let evaluated = tokio::task::spawn_blocking(move || {
            let mut graph = graph;
            crate::semantic::evaluate_workspace_graph(&mut graph);
            graph
        })
        .await;
        let Ok(evaluated_graph) = evaluated else {
            return;
        };

        let committed = handle
            .report_evaluation_result(expected_version, evaluated_graph)
            .await
            .unwrap_or(false);
        if committed {
            // Workspace-wide, not `[changed_uri]` — see the doc comment above.
            publish_workspace_diagnostics(&client, &handle, &config, &runtime_config, None).await;
        }
    });
}

async fn log_perf(client: &Client, enabled: bool, event: &str, fields: Vec<(&str, String)>) {
    if !enabled {
        return;
    }
    let details = fields
        .into_iter()
        .map(|(key, value)| format!("\"{}\":{}", key, value))
        .collect::<Vec<_>>()
        .join(",");
    client
        .log_message(
            MessageType::INFO,
            format!("[SysML][perf] {{\"event\":\"{}\",{}}}", event, details),
        )
        .await;
}

fn workspace_file_count(state: &ServerState) -> usize {
    state
        .semantic_graph
        .workspace_uris_excluding_libraries(&state.library_paths)
        .len()
}

pub(crate) struct SemanticIndexReady;

impl Notification for SemanticIndexReady {
    type Params = SemanticIndexReadyNotificationDto;
    const METHOD: &'static str = "spec42/semanticIndexReady";
}

pub(crate) fn semantic_index_ready_notification(
    state: &ServerState,
) -> SemanticIndexReadyNotificationDto {
    SemanticIndexReadyNotificationDto {
        lifecycle: "ready".to_string(),
        semantic_state_version: state.session.version(),
        workspace_file_count: workspace_file_count(state),
    }
}

/// Sends the `spec42/semanticIndexReady` LSP notification to the client.
/// The session must already be in `Ready` state before calling this.
async fn send_semantic_ready_notification(client: &Client, handle: &WorkspaceHandle) {
    let params = semantic_index_ready_notification(&handle.snapshot());
    client.send_notification::<SemanticIndexReady>(params).await;
}


mod startup;
mod sync;
pub(crate) use startup::{initialize, initialized};
pub(crate) use sync::{did_change, did_change_configuration, did_change_watched_files, did_close, did_open};

#[cfg(test)]
mod tests {
    use super::sync::watched_file_content_already_current;
    use super::*;

    #[test]
    fn semantic_index_ready_notification_includes_version_and_file_count() {
        let mut state = ServerState::default();
        state.session.begin_startup();
        // Simulate 6 bumps so the version reaching Ready is 7.
        for _ in 0..6 {
            state.session.bump_version();
        }
        state.session.complete_startup();
        let params = semantic_index_ready_notification(&state);
        assert_eq!(params.lifecycle, "ready");
        assert_eq!(params.semantic_state_version, 8); // begin(1) + 6 bumps + complete(1) = 8
        assert_eq!(params.workspace_file_count, 0);
    }

    /// Fix for the redundant-save full-rebuild bug: a `didChangeWatchedFiles` event whose disk
    /// content matches what the server already has tracked (the normal "I edited in VS Code,
    /// then saved" case, since `didChange` already updated the in-memory copy) must be
    /// recognized as a no-op so `did_change_watched_files` can skip the expensive
    /// `refresh_document` call entirely.
    #[tokio::test]
    async fn watched_file_content_already_current_when_matching_tracked_content() {
        let uri = Url::parse("file:///demo.sysml").expect("uri");
        let mut state = ServerState::default();
        state.index.insert(
            uri.clone(),
            crate::workspace::state::IndexEntry {
                content: "package Demo { part def Thing; }".to_string(),
                parsed: None,
                parse_metadata: Default::default(),
                include_in_semantic_graph: true,
            },
        );
        let handle = WorkspaceHandle::spawn(state);

        assert!(watched_file_content_already_current(
            &handle,
            &uri,
            "package Demo { part def Thing; }"
        ));
    }

    /// Genuinely different disk content (an external edit, e.g. another editor or `git
    /// checkout`) must NOT be treated as a no-op — the full refresh path must still run.
    #[tokio::test]
    async fn watched_file_content_not_current_when_content_differs() {
        let uri = Url::parse("file:///demo.sysml").expect("uri");
        let mut state = ServerState::default();
        state.index.insert(
            uri.clone(),
            crate::workspace::state::IndexEntry {
                content: "package Demo { part def Thing; }".to_string(),
                parsed: None,
                parse_metadata: Default::default(),
                include_in_semantic_graph: true,
            },
        );
        let handle = WorkspaceHandle::spawn(state);

        assert!(!watched_file_content_already_current(
            &handle,
            &uri,
            "package Demo { part def Renamed; }"
        ));
    }

    /// A URI the server has never seen before (not in `index` at all) must not be treated as
    /// "already current" — it needs the normal ingest path, not a skip.
    #[tokio::test]
    async fn watched_file_content_not_current_when_uri_unknown() {
        let uri = Url::parse("file:///unknown.sysml").expect("uri");
        let handle = WorkspaceHandle::spawn(ServerState::default());

        assert!(!watched_file_content_already_current(&handle, &uri, "anything"));
    }
}
