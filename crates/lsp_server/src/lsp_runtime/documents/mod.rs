use std::sync::Arc;
use std::time::{Duration, Instant};

use tower_lsp::lsp_types::{notification::Notification, *};
use tower_lsp::Client;
use tracing::info;

use crate::common::util;
use crate::host::config::Spec42Config;
use crate::session::state::ServerState;
use crate::session::{parse_scanned_documents, scan_sysml_files, RuntimeConfig, WorkspaceHandle};
use crate::views::dto::SemanticIndexReadyNotificationDto;

use super::capabilities::server_capabilities;
use super::diagnostics::{publish_document_diagnostics, publish_workspace_diagnostics};
use super::lifecycle::{project_boundary_for_uri, scan_roots, workspace_roots_from_initialize};
use super::project_registry::ProjectRegistry;

/// Ordered source ingestion schedules one bounded worker; delivery has its own per-owner loop.
async fn publish_semantic_change(
    client: &Client,
    handle: &WorkspaceHandle,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
    _changed_uri: Url,
    _source_version: i32,
) {
    ensure_analysis_notifications(client, handle, runtime_config);
    handle.request_rebuild();
}

struct AnalysisStatusChanged;
impl Notification for AnalysisStatusChanged {
    type Params = ();
    const METHOD: &'static str = crate::views::dto::ANALYSIS_STATUS_CHANGED_METHOD;
}

fn ensure_analysis_notifications(
    client: &Client,
    handle: &WorkspaceHandle,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
) {
    if !handle.claim_analysis_notifications() {
        return;
    }
    let mut events = handle.analysis_events();
    let client = client.clone();
    let handle = handle.clone();
    let runtime_config = Arc::clone(runtime_config);
    tokio::spawn(async move {
        use crate::session::handle::AnalysisEvent;
        loop {
            let event = events.borrow_and_update().clone();
            // A wake-up only: the client reads the current registry's canonical status.
            client.send_notification::<AnalysisStatusChanged>(()).await;
            if let AnalysisEvent::Closed = event {
                return;
            }
            if let AnalysisEvent::Completed { inputs, result } = event {
                if handle.snapshot().session.is_input_current(&inputs) {
                    match result {
                        Ok(report) => {
                            let diagnostics_started = Instant::now();
                            let mut diagnostic_fanout = Duration::ZERO;
                            if report.outcome
                                == sysml_query::publication::PublicationOutcome::Published
                            {
                                let snapshot = handle.snapshot();
                                let readiness = snapshot
                                    .diagnostic_readiness
                                    .needs_publication_notification()
                                    .then(|| snapshot.diagnostic_readiness.token());
                                drop(snapshot);
                                if let Some(readiness) = readiness {
                                    send_semantic_ready_notification(&client, &handle, readiness)
                                        .await;
                                }
                                // A coalesced job can include changes to several documents. A
                                // workspace sweep also clears former dependants; open libraries
                                // remain authoring surfaces and must receive their diagnostics.
                                let snapshot = handle.snapshot();
                                let mut targets = crate::session::state::workspace_sweep_targets(
                                    &*snapshot, false,
                                );
                                targets.extend(
                                    snapshot.editor_documents().map(|(uri, _)| uri.clone()),
                                );
                                targets.sort_unstable_by(|a, b| a.as_str().cmp(b.as_str()));
                                targets.dedup();
                                drop(snapshot);
                                publish_workspace_diagnostics(
                                    &client,
                                    &handle,
                                    &runtime_config,
                                    Some(&targets),
                                )
                                .await;
                                diagnostic_fanout = diagnostics_started.elapsed();
                                if handle.snapshot().session.is_input_current(&inputs) {
                                    refresh_semantic_tokens(&client, &runtime_config).await;
                                }
                            }
                            if runtime_config
                                .get()
                                .is_some_and(|config| config.perf_logging_enabled)
                            {
                                let snapshot = handle.snapshot();
                                if snapshot.session.is_input_current(&inputs) {
                                    let versions = snapshot
                                        .editor_documents()
                                        .filter_map(|(uri, entry)| {
                                            entry
                                                .editor_version()
                                                .map(|version| (uri.as_str(), version))
                                        })
                                        .collect::<std::collections::BTreeMap<_, _>>();
                                    let ms = |duration: Duration| {
                                        (duration.as_secs_f64() * 1000.0).to_string()
                                    };
                                    let mut fields = vec![
                                        (
                                            "editorVersions",
                                            serde_json::to_string(&versions)
                                                .expect("editor versions serialize"),
                                        ),
                                        (
                                            "modelDigest",
                                            serde_json::to_string(
                                                &snapshot
                                                    .session
                                                    .current()
                                                    .publication()
                                                    .model_digest()
                                                    .to_string(),
                                            )
                                            .expect("model digest serializes"),
                                        ),
                                        (
                                            "rebuildMs",
                                            ms(report.preparation
                                                + report.preparation_queue
                                                + report.construction
                                                + report.admission),
                                        ),
                                        ("outcome", format!("\"{:?}\"", report.outcome)),
                                        ("preparationMs", ms(report.preparation)),
                                        ("preparationQueueMs", ms(report.preparation_queue)),
                                        ("constructionWallMs", ms(report.construction)),
                                        ("admissionWallMs", ms(report.admission)),
                                        ("diagnosticFanoutMs", ms(diagnostic_fanout)),
                                        ("sourceParseMs", ms(report.source_parsing)),
                                    ];
                                    if let Some(phases) = report.phases {
                                        fields.extend([
                                            ("buildParseMs", ms(phases.parse)),
                                            ("loweringMs", ms(phases.lowering)),
                                            ("resolutionTotalMs", ms(phases.resolution)),
                                            ("resolveMs", ms(phases.resolve)),
                                            ("evaluationMs", ms(phases.evaluation)),
                                            ("indexingMs", ms(phases.indexing)),
                                            ("checksMs", ms(phases.diagnostics)),
                                            ("namespaceChecksMs", ms(phases.namespace_checks)),
                                            (
                                                "documentsLowered",
                                                phases.documents_lowered.to_string(),
                                            ),
                                            (
                                                "documentsReused",
                                                phases.documents_reused.to_string(),
                                            ),
                                            ("sourcesParsed", phases.sources_parsed.to_string()),
                                        ]);
                                    }
                                    log_perf(&client, true, "backend:publication", fields).await;
                                }
                            }
                        }
                        Err(error) => {
                            client
                                .log_message(
                                    MessageType::ERROR,
                                    format!("Analysis actor failed: {error:?}"),
                                )
                                .await
                        }
                    }
                    report_publication_failure(&client, &handle).await;
                }
            }
            if events.changed().await.is_err() {
                return;
            }
        }
    });
}

async fn refresh_semantic_tokens(
    client: &Client,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
) {
    if runtime_config
        .get()
        .is_some_and(|config| config.semantic_tokens_refresh_supported)
    {
        // Clients are external peers; an unanswered refresh must not stall later delivery.
        let _ =
            tokio::time::timeout(Duration::from_secs(2), client.semantic_tokens_refresh()).await;
    }
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
        .index
        .keys()
        .filter(|uri| !crate::common::util::uri_under_any_library(uri, &state.library_paths))
        .count()
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
///
/// Diagnostics stay deferred until this send completes and the actor accepts its epoch.
/// Semantic edits do not reset this host-only barrier; startup and library replacement do.
async fn send_semantic_ready_notification(
    client: &Client,
    handle: &WorkspaceHandle,
    token: crate::session::ReadinessToken,
) {
    let snapshot = handle.snapshot();
    if !snapshot.diagnostic_readiness.is_current(&token)
        || snapshot.diagnostic_readiness.source_inputs_pending()
        || (!snapshot.diagnostic_readiness.needs_initial_notification()
            && snapshot.session.analysis_status()
                != sysml_query::publication::AnalysisStatus::Current)
    {
        return;
    }
    drop(snapshot);
    if !handle
        .begin_readiness_notification(token.clone())
        .await
        .unwrap_or(false)
    {
        return;
    }
    let snapshot = handle.snapshot();
    if !snapshot.diagnostic_readiness.is_current(&token) {
        return;
    }
    let params = semantic_index_ready_notification(&snapshot);
    drop(snapshot);
    client.send_notification::<SemanticIndexReady>(params).await;
    let _ = handle.announce_readiness(token).await;

    report_publication_failure(client, handle).await;
}

/// A retained publication must be visible as stale analysis, with the owning phase's cause.
async fn report_publication_failure(client: &Client, handle: &WorkspaceHandle) {
    if let Some(failure) = handle.unreported_publication_failure() {
        client
            .show_message(MessageType::ERROR, publication_failure_message(&failure))
            .await;
    }
}

fn publication_failure_message(
    failure: &sysml_query::publication::PublicationBuildFailure,
) -> String {
    format!("Spec42 could not analyze the current project; the previous analysis remains visible. {failure}")
}

mod startup;
mod sync;
pub(crate) use startup::{initialize, initialized};
pub(crate) use sync::{
    did_change, did_change_configuration, did_change_watched_files, did_close, did_open,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retained_publication_failure_message_exposes_the_cause() {
        let failure = sysml_query::publication::PublicationBuildFailure::construction_worker(
            "worker panicked",
        );
        assert_eq!(publication_failure_message(&failure),
            "Spec42 could not analyze the current project; the previous analysis remains visible. ConstructionWorker: worker panicked");
    }

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
}
