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
use sysml_query::publication::RelinkToken;

use super::capabilities::server_capabilities;
use super::diagnostics::{publish_document_diagnostics, publish_workspace_diagnostics};
use super::lifecycle::{scan_roots, workspace_roots_from_initialize};

static WORKSPACE_DIAGNOSTICS_DEBOUNCE_GEN: AtomicU64 = AtomicU64::new(0);
const WORKSPACE_DIAGNOSTICS_DEBOUNCE_MS: u64 = 450;

fn schedule_workspace_diagnostics_republish(
    client: &Client,
    handle: &WorkspaceHandle,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
) {
    let generation = WORKSPACE_DIAGNOSTICS_DEBOUNCE_GEN.fetch_add(1, Ordering::SeqCst) + 1;
    let client = client.clone();
    let handle = handle.clone();
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
        publish_workspace_diagnostics(&client, &handle, &runtime_config, None).await;
    });
}

/// Commits the edit token and publishes the one canonical semantic rebuild before returning.
/// Requests arriving after `didOpen`/`didChange` therefore observe that exact source revision;
/// there is no second host-side graph build or symbol derivation racing the authority.
async fn publish_semantic_change(
    client: &Client,
    handle: &WorkspaceHandle,
    runtime_config: &Arc<std::sync::OnceLock<RuntimeConfig>>,
    changed_uri: Url,
    token: RelinkToken,
) {
    let old = handle.snapshot();
    let workspace_uris = old
        .index
        .keys()
        .filter(|uri| {
            !crate::common::util::uri_under_any_library(uri, &old.library_paths)
                && !crate::common::util::uri_under_any_library(uri, &old.standard_library_paths)
        })
        .cloned()
        .collect::<Vec<_>>();
    let mut diagnostic_uris = crate::workspace::import_graph::affected_diagnostic_documents(
        old.published_model(),
        workspace_uris,
        &changed_uri,
    )
    .into_uris();
    if !diagnostic_uris.contains(&changed_uri) {
        diagnostic_uris.push(changed_uri);
    }
    drop(old);

    if handle
        .report_relink_result(token, Vec::new())
        .await
        .unwrap_or(false)
    {
        publish_workspace_diagnostics(client, handle, runtime_config, Some(&diagnostic_uris)).await;
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
async fn send_semantic_ready_notification(client: &Client, handle: &WorkspaceHandle) {
    let params = semantic_index_ready_notification(&handle.snapshot());
    client.send_notification::<SemanticIndexReady>(params).await;
}

mod startup;
mod sync;
pub(crate) use startup::{initialize, initialized};
pub(crate) use sync::{
    did_change, did_change_configuration, did_change_watched_files, did_close, did_open,
};

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
            crate::workspace::state::IndexEntry::for_test(&uri, "package Demo { part def Thing; }"),
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
            crate::workspace::state::IndexEntry::for_test(&uri, "package Demo { part def Thing; }"),
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

        assert!(!watched_file_content_already_current(
            &handle, &uri, "anything"
        ));
    }
}
