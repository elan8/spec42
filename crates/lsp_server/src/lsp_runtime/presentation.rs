//! One coherent, source-editor-independent request for a view catalog and its selected product.
use std::sync::Arc;

use sysml_query::publication::{AnalysisStatus, SessionLifecycle};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::Url;

use super::generation::{DiagramPresentationParams, DiagramPresentationResult};
use super::Backend;
use crate::session::ServerState;

fn can_deliver(
    state: &ServerState,
    token: &sysml_query::publication::PublicationToken,
    same_owner: bool,
) -> bool {
    same_owner && state.session.is_publication_current(token) && pending_outcome(state).is_none()
}

/// Read the canonical lifecycle; the diagram layer owns no parallel freshness state.
fn pending_outcome(state: &ServerState) -> Option<DiagramPresentationResult> {
    let semantic_state_version = state.session.version().to_string();
    if state.session.lifecycle() == SessionLifecycle::Closed {
        return Some(DiagramPresentationResult::Unavailable {
            message: "The project session is closed.".into(),
        });
    }
    match state.session.analysis_status() {
        AnalysisStatus::FailedRetained => Some(DiagramPresentationResult::Failed {
            semantic_state_version,
            message: state
                .publication_failure
                .as_ref()
                .map(|failure| failure.to_string())
                .unwrap_or_else(|| "Analysis failed; the previous publication is retained.".into()),
        }),
        AnalysisStatus::CancelledRetained => Some(DiagramPresentationResult::Cancelled {
            semantic_state_version,
        }),
        AnalysisStatus::Updating => Some(DiagramPresentationResult::Updating {
            semantic_state_version,
        }),
        AnalysisStatus::Current
            if state.diagnostic_readiness.source_inputs_pending()
                || matches!(
                    state.session.lifecycle(),
                    SessionLifecycle::Cold | SessionLifecycle::Indexing
                ) =>
        {
            Some(DiagramPresentationResult::Updating {
                semantic_state_version,
            })
        }
        AnalysisStatus::Current => None,
    }
}

impl Backend {
    pub(super) async fn diagram_presentation(
        &self,
        params: DiagramPresentationParams,
    ) -> Result<DiagramPresentationResult> {
        let uri = Url::parse(&params.project_uri).map_err(|error| {
            tower_lsp::jsonrpc::Error::invalid_params(format!(
                "invalid project context URI: {error}"
            ))
        })?;
        let handle = match self.projects.query_handle_for_uri(&uri, None) {
            Ok(handle) => handle,
            Err(message) => return Ok(DiagramPresentationResult::Unavailable { message }),
        };
        // Subscribe before taking the snapshot: a completion racing this read always produces a
        // wake-up. The client subscribes before issuing its initial request, and coalesces wakes
        // received in flight. This closes both sides of the check/subscribe race without polling.
        super::documents::ensure_analysis_notifications(
            &self.client,
            &handle,
            &self.runtime_config,
        );
        let state = handle.snapshot();
        if let Some(outcome) = pending_outcome(&state) {
            return Ok(outcome);
        }
        let token = state.session.publication();
        let version = state.session.version().to_string();
        let publication = Arc::clone(state.session.current());
        drop(state);
        let service = Arc::clone(&self.generator_service);
        let result = tokio::task::spawn_blocking(move || {
            service.presentation(publication, &params, version)
        })
        .await
        .map_err(|error| tower_lsp::jsonrpc::Error {
            code: tower_lsp::jsonrpc::ErrorCode::InternalError,
            message: format!("diagram worker failed: {error}").into(),
            data: None,
        })?;
        let current_handle = self.projects.query_handle_for_uri(&uri, None).ok();
        let current = handle.snapshot();
        if !can_deliver(
            &current,
            &token,
            current_handle.is_some_and(|owner| owner.same_session(&handle)),
        ) {
            return Ok(DiagramPresentationResult::Superseded);
        }
        Ok(
            result.unwrap_or_else(|message| DiagramPresentationResult::Failed {
                semantic_state_version: current.session.version().to_string(),
                message,
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovery_pending_failed_and_cancelled_are_explicit() {
        let mut state = ServerState::default();
        assert!(matches!(
            pending_outcome(&state),
            Some(DiagramPresentationResult::Updating { .. })
        ));
        state.session.complete_startup();
        assert!(pending_outcome(&state).is_none());
        state.diagnostic_readiness.begin_startup_sources();
        assert!(matches!(
            pending_outcome(&state),
            Some(DiagramPresentationResult::Updating { .. })
        ));
        state.session.invalidate_inputs();
        state.session.finish_preparation_failure();
        assert!(matches!(
            pending_outcome(&state),
            Some(DiagramPresentationResult::Failed { .. })
        ));
        state.session.invalidate_inputs();
        state.session.finish_preparation_cancelled();
        assert!(matches!(
            pending_outcome(&state),
            Some(DiagramPresentationResult::Cancelled { .. })
        ));
        state.session.close();
        assert!(matches!(
            pending_outcome(&state),
            Some(DiagramPresentationResult::Unavailable { .. })
        ));
    }

    #[test]
    fn completed_work_requires_the_current_owner_and_version_even_with_the_same_digest() {
        let mut state = ServerState::default();
        state.session.complete_startup();
        let token = state.session.publication();
        let digest = state.session.current().publication().model_digest();
        assert!(can_deliver(&state, &token, true));
        assert!(!can_deliver(&state, &token, false));
        state.session.bump_version();
        assert_eq!(state.session.current().publication().model_digest(), digest);
        assert!(!can_deliver(&state, &token, true));
        let current = state.session.publication();
        state.session.invalidate_inputs();
        assert!(!can_deliver(&state, &current, true));
    }

    #[test]
    fn lifecycle_versions_are_lossless_on_the_wire_and_bindings_match_the_owner() {
        let value = serde_json::to_value(DiagramPresentationResult::Updating {
            semantic_state_version: "9007199254740993".into(),
        })
        .unwrap();
        assert_eq!(value["semanticStateVersion"], "9007199254740993");
        let checked_in = include_str!("../../../../vscode/src/generated/diagram-presentation.ts");
        assert_eq!(
            checked_in,
            super::super::generation::diagram_presentation_typescript()
        );
    }
}
