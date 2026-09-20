//! `spec42/draw`: native prepare + layout + SVG for the webview (issue #176 / #181).
//!
//! Stateless like `spec42/layout`: the client supplies the product JSON, canvas size, colour
//! scheme, and renderer-owned disclosure state; the server echoes identity fields and returns
//! SVG. `SPEC42_LAYOUT_ENGINE=legacy` declines this request. There is no client drawing fallback
//! — the webview stays on its last SVG (or an inert placeholder) until native drawing is
//! available again.

use diagram_draw::DisclosureState;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct DrawParams {
    pub(crate) model_digest: String,
    pub(crate) view_handle: String,
    pub(crate) presentation_revision: u64,
    pub(crate) product: Value,
    #[serde(default)]
    pub(crate) width: Option<f64>,
    #[serde(default)]
    pub(crate) height: Option<f64>,
    #[serde(default)]
    pub(crate) color_scheme: Option<String>,
    #[serde(default)]
    pub(crate) disclosure: Option<DisclosureState>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DrawEngine {
    Native,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DrawResult {
    pub(crate) model_digest: String,
    pub(crate) view_handle: String,
    pub(crate) presentation_revision: u64,
    pub(crate) svg: String,
    pub(crate) engine: DrawEngine,
}

pub(crate) fn canvas_size(value: Option<f64>, fallback: f64) -> f64 {
    match value {
        Some(size) if size.is_finite() && size > 0.0 => size,
        _ => fallback,
    }
}
