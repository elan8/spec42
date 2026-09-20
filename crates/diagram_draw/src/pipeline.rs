//! Visualization payload → prepare → layout → SVG. This is the fully native headless path
//! (spec42 #176): QuickJS is not involved.

use serde_json::Value;

use crate::draw_input::DrawError;
use crate::json_util::{as_string, field};
use crate::layout::layout_to_draw_input;
use crate::prepare::prepare_view_data;
use crate::theme::{Theme, LIGHT};

#[derive(Debug)]
pub enum PipelineError {
    Draw(DrawError),
    Layout(diagram_layout::LayoutError),
    InvalidPayload(String),
}

impl std::fmt::Display for PipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineError::Draw(err) => write!(f, "{err}"),
            PipelineError::Layout(err) => write!(f, "Native diagram layout failed: {err}"),
            PipelineError::InvalidPayload(message) => {
                write!(f, "invalid visualization payload: {message}")
            }
        }
    }
}

impl std::error::Error for PipelineError {}

impl From<DrawError> for PipelineError {
    fn from(value: DrawError) -> Self {
        PipelineError::Draw(value)
    }
}

impl From<diagram_layout::LayoutError> for PipelineError {
    fn from(value: diagram_layout::LayoutError) -> Self {
        PipelineError::Layout(value)
    }
}

/// Prepare + layout a visualization payload into the JSON `render_svg_from_json` draws from.
pub fn draw_input_from_payload(payload: &Value) -> Result<Value, PipelineError> {
    let prepared = prepare_view_data(payload);
    let view = as_string(field(&prepared, "view"), "general-view");
    if matches!(
        view.as_str(),
        "browser-view" | "grid-view" | "geometry-view"
    ) {
        return Err(PipelineError::Draw(DrawError::UnsupportedView(view)));
    }
    Ok(layout_to_draw_input(&prepared)?)
}

pub fn render_svg_from_payload(
    payload: &Value,
    width: f64,
    height: f64,
) -> Result<String, PipelineError> {
    render_svg_from_payload_with_theme(payload, &LIGHT, width, height)
}

pub fn render_svg_from_payload_str(
    payload_json: &str,
    width: f64,
    height: f64,
) -> Result<String, PipelineError> {
    let payload: Value = serde_json::from_str(payload_json)
        .map_err(|err| PipelineError::InvalidPayload(err.to_string()))?;
    render_svg_from_payload(&payload, width, height)
}

pub fn render_svg_from_payload_with_theme(
    payload: &Value,
    theme: &Theme,
    width: f64,
    height: f64,
) -> Result<String, PipelineError> {
    let draw_input = draw_input_from_payload(payload)?;
    Ok(crate::draw_input::render_svg_from_json_with_theme(
        &draw_input,
        theme,
        width,
        height,
    )?)
}
