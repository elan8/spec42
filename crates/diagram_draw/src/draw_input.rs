//! JSON dispatcher used by the native drawing path (spec42 #176).
//!
//! Callers that already have laid-out JSON (parity fixtures) can render directly. Headless
//! export instead runs [`crate::pipeline`]: visualization payload → prepare → elkrs layout →
//! this dispatcher. The five dump shapes are:
//!
//! - General View: `{ title, view, meta, nodes, edges }`
//! - Interconnection View: the General View shape plus `interconnectionLayout`
//! - Sequence View: a `PreparedView` (`{ title, view, nodes, edges, meta }`)
//! - Action-flow / state-transition: `{ prepared, behaviorLayout }`

use serde::Deserialize;
use serde_json::Value;

use crate::behavior_common::{BehaviorLayoutResult, PreparedView};
use crate::theme::{Theme, LIGHT};
use crate::types::{GeneralViewGraph, InterconnectionViewGraph};
use crate::{
    render_action_flow_view_svg, render_general_view_svg, render_interconnection_view_svg,
    render_sequence_view_svg, render_state_transition_view_svg,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrawError {
    UnsupportedView(String),
    InvalidInput(String),
}

impl std::fmt::Display for DrawError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DrawError::UnsupportedView(view) => {
                write!(f, "diagram_draw has no native renderer for view `{view}`")
            }
            DrawError::InvalidInput(message) => write!(f, "invalid native draw input: {message}"),
        }
    }
}

impl std::error::Error for DrawError {}

#[derive(Deserialize)]
struct BehaviorDrawInput {
    prepared: PreparedView,
    #[serde(rename = "behaviorLayout")]
    behavior_layout: BehaviorLayoutResult,
}

fn view_of(input: &Value) -> &str {
    input
        .get("view")
        .and_then(Value::as_str)
        .or_else(|| {
            input
                .get("prepared")
                .and_then(Value::as_object)
                .and_then(|prepared| prepared.get("view"))
                .and_then(Value::as_str)
        })
        .unwrap_or("general-view")
}

fn decode<T: for<'de> Deserialize<'de>>(input: &Value, what: &str) -> Result<T, DrawError> {
    serde_json::from_value(input.clone())
        .map_err(|err| DrawError::InvalidInput(format!("failed to decode {what}: {err}")))
}

/// Render an SVG from a dumped draw-input document, using the light theme the golden fixtures use.
pub fn render_svg_from_json(input: &Value, width: f64, height: f64) -> Result<String, DrawError> {
    render_svg_from_json_with_theme(input, &LIGHT, width, height)
}

pub fn render_svg_from_str(json: &str, width: f64, height: f64) -> Result<String, DrawError> {
    let input: Value = serde_json::from_str(json)
        .map_err(|err| DrawError::InvalidInput(format!("draw-input JSON is invalid: {err}")))?;
    render_svg_from_json(&input, width, height)
}

pub fn render_svg_from_json_with_theme(
    input: &Value,
    theme: &Theme,
    width: f64,
    height: f64,
) -> Result<String, DrawError> {
    if input.get("behaviorLayout").is_some() {
        let fixture: BehaviorDrawInput = decode(input, "action-flow/state-transition draw input")?;
        return match fixture.prepared.view.as_str() {
            "action-flow-view" => Ok(render_action_flow_view_svg(
                &fixture.prepared,
                &fixture.behavior_layout,
                theme,
                width,
                height,
            )),
            "state-transition-view" => Ok(render_state_transition_view_svg(
                &fixture.prepared,
                &fixture.behavior_layout,
                theme,
                width,
                height,
            )),
            other => Err(DrawError::UnsupportedView(other.to_string())),
        };
    }

    match view_of(input) {
        "interconnection-view" => {
            let graph: InterconnectionViewGraph = decode(input, "interconnection-view draw input")?;
            Ok(render_interconnection_view_svg(
                &graph, theme, width, height,
            ))
        }
        "sequence-view" => {
            let prepared: PreparedView = decode(input, "sequence-view draw input")?;
            Ok(render_sequence_view_svg(&prepared, theme, width, height))
        }
        "action-flow-view" | "state-transition-view" => Err(DrawError::InvalidInput(
            "action-flow-view and state-transition-view require `{ prepared, behaviorLayout }`"
                .into(),
        )),
        "browser-view" | "grid-view" | "geometry-view" => {
            Err(DrawError::UnsupportedView(view_of(input).to_string()))
        }
        "general-view" => {
            let graph: GeneralViewGraph = decode(input, "general-view draw input")?;
            Ok(render_general_view_svg(&graph, theme, width, height))
        }
        other => Err(DrawError::UnsupportedView(other.to_string())),
    }
}
