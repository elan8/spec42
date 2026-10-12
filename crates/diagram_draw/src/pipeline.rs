//! Visualization payload → prepare → layout → SVG. This is the fully native path
//! (spec42 #176 / #181): QuickJS is not involved. Headless export omits disclosure so nested
//! membership stays visible; `spec42/draw` supplies renderer-owned expansion state.

use serde_json::Value;

use crate::disclosure::{apply_general_disclosure, DisclosureState};
use crate::draw_input::DrawError;
use crate::layout::layout_to_draw_input;
use crate::nesting::{apply_nesting, Nesting};
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
/// Headless export uses this (no disclosure projection).
pub fn draw_input_from_payload(payload: &Value) -> Result<Value, PipelineError> {
    draw_input_from_payload_with_disclosure(payload, None)
}

pub fn draw_input_from_payload_with_disclosure(
    payload: &Value,
    disclosure: Option<&DisclosureState>,
) -> Result<Value, PipelineError> {
    draw_input_from_payload_with_presentation(payload, disclosure, Nesting::All)
}

/// Prepare + layout with the renderer-owned presentation choices: which nested nodes a general
/// view discloses, and how many levels below its exposed elements a view draws.
pub fn draw_input_from_payload_with_presentation(
    payload: &Value,
    disclosure: Option<&DisclosureState>,
    nesting: Nesting,
) -> Result<Value, PipelineError> {
    let mut prepared = prepare_view_data(payload).map_err(PipelineError::InvalidPayload)?;
    if let Some(state) = disclosure {
        apply_general_disclosure(&mut prepared, state);
    }
    apply_nesting(&mut prepared, nesting);
    Ok(layout_to_draw_input(&prepared)?)
}

/// Renders a payload for a document or a file: the light theme and a nesting limit.
pub fn render_svg_from_payload_str_nested(
    payload_json: &str,
    nesting: Nesting,
) -> Result<String, PipelineError> {
    render_svg_from_payload_str_styled(payload_json, nesting, &LIGHT)
}

/// As `render_svg_from_payload_str_nested`, in the theme a document's style gives.
pub fn render_svg_from_payload_str_styled(
    payload_json: &str,
    nesting: Nesting,
    theme: &Theme,
) -> Result<String, PipelineError> {
    let payload: Value = serde_json::from_str(payload_json)
        .map_err(|err| PipelineError::InvalidPayload(err.to_string()))?;
    let draw_input = draw_input_from_payload_with_presentation(&payload, None, nesting)?;
    Ok(crate::draw_input::render_svg_from_json_with_theme(
        &draw_input,
        theme,
        0.0,
        0.0,
    )?)
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
    render_svg_from_payload_with_options(payload, theme, width, height, None)
}

pub fn render_svg_from_payload_with_options(
    payload: &Value,
    theme: &Theme,
    width: f64,
    height: f64,
    disclosure: Option<&DisclosureState>,
) -> Result<String, PipelineError> {
    let draw_input = draw_input_from_payload_with_disclosure(payload, disclosure)?;
    Ok(crate::draw_input::render_svg_from_json_with_theme(
        &draw_input,
        theme,
        width,
        height,
    )?)
}
