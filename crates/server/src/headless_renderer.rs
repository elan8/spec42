//! Headless SVG export for diagram products (CLI / `POST /v1/diagrams/export` / generator smoke).
//!
//! Every shipped view prepares and draws in `diagram_draw`. QuickJS is not on this path.

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

/// Renders a diagram product for a file or a document, drawing `nesting` levels of parts in an
/// interconnection view, in `theme`. The figure is as large as its content.
pub fn render_nested_svg(
    payload_json: &str,
    nesting: diagram_draw::Nesting,
    theme: &diagram_draw::theme::Theme,
) -> Result<String, String> {
    diagram_draw::pipeline::render_svg_from_payload_str_styled(payload_json, nesting, theme)
        .map_err(|err| match err {
            PipelineError::Draw(DrawError::UnsupportedView(_)) => err.to_string(),
            other => format!("Native diagram export failed: {other}"),
        })
}
