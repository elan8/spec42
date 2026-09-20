//! Headless SVG export for diagram products (CLI / `POST /v1/diagrams/export` / generator smoke).
//!
//! The five shipped views prepare, lay out (elkrs), and draw in `diagram_draw`. QuickJS is not
//! on this path. Browser / grid / geometry stay TypeScript-only and return an unsupported-view
//! error here.

use diagram_draw::{render_svg_from_payload_str, DrawError, PipelineError};

const DEFAULT_WIDTH: f64 = 1280.0;
const DEFAULT_HEIGHT: f64 = 900.0;

pub fn render_shared_svg(payload_json: &str) -> Result<String, String> {
    render_native_svg(payload_json)
}

pub fn render_native_svg(payload_json: &str) -> Result<String, String> {
    render_svg_from_payload_str(payload_json, DEFAULT_WIDTH, DEFAULT_HEIGHT).map_err(
        |err| match err {
            PipelineError::Draw(DrawError::UnsupportedView(_)) => err.to_string(),
            other => format!("Native diagram export failed: {other}"),
        },
    )
}
