//! `spec42/layout`: native ELK layout for a webview-supplied graph (issue #119).
//!
//! Stateless by design, unlike `spec42_generate`/`spec42_diagram_views`: layout is a pure
//! function of the client-supplied graph JSON, so there is no workspace publication to look up
//! and no `state_for_uri` coupling. `model_digest`/`view_handle`/`presentation_revision` are
//! echoed back verbatim, never interpreted -- the server has no independent notion of webview
//! presentation state; the client is the one that compares the echo against its own current
//! values to reject a stale or superseded response.
//!
//! `diagram_layout` (the native `elkrs` adapter) is this handler's only engine. The legacy
//! ELK.js/QuickJS engine lives in `crates/server`'s `elk_layout` module, which `lsp_server`
//! cannot depend on -- `crates/sysml_query/tests/architecture.rs` pins the launch-only edge the
//! other way (`server` depends on `lsp_server`, not the reverse). `SPEC42_LAYOUT_ENGINE=legacy`
//! therefore cannot mean "run the legacy engine here"; it means "decline, so the client's own
//! existing in-webview `layoutPrepared`/elk.js fallback takes over" -- the same path a request
//! failure or an offline extension host already falls back to.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct LayoutParams {
    pub(crate) model_digest: String,
    pub(crate) view_handle: String,
    pub(crate) presentation_revision: u64,
    pub(crate) graph: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LayoutEngine {
    Native,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LayoutResult {
    pub(crate) model_digest: String,
    pub(crate) view_handle: String,
    pub(crate) presentation_revision: u64,
    pub(crate) layout: Value,
    pub(crate) engine: LayoutEngine,
}

/// `true` when `SPEC42_LAYOUT_ENGINE` explicitly asks for the legacy engine. There is no legacy
/// engine to run here (see module docs), so this is read as "decline this request", not as a
/// second code path to execute.
pub(crate) fn legacy_engine_requested() -> bool {
    std::env::var("SPEC42_LAYOUT_ENGINE").as_deref() == Ok("legacy")
}
