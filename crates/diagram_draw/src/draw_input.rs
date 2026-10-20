//! JSON dispatcher used by the native drawing path (spec42 #176 / #181).
//!
//! Callers that already have laid-out JSON (parity fixtures) can render directly. Headless
//! export instead runs [`crate::pipeline`]: visualization payload → prepare → elkrs layout
//! (skipped for sequence / browser / grid / geometry) → this dispatcher. The dump shapes are:
//!
//! - General View: `{ title, view, meta, nodes, edges }`
//! - Interconnection View: the General View shape plus `interconnectionLayout`
//! - Sequence / Browser / Grid / Geometry: a `PreparedView` (`{ title, view, nodes, edges, meta }`)
//! - Action-flow / state-transition: `{ prepared, behaviorLayout }`

use serde::Deserialize;
use serde_json::Value;

use crate::behavior_common::{BehaviorLayoutResult, PreparedView};
use crate::theme::{Theme, LIGHT};
use crate::types::{GeneralViewGraph, InterconnectionViewGraph};
use crate::{
    render_action_flow_view_svg, render_browser_view_svg, render_general_view_svg,
    render_geometry_view_svg, render_grid_view_svg, render_interconnection_view_svg,
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
    // Validate the view-specific input before presenting an empty state: malformed or
    // unsupported input must keep its error rather than becoming an apparently valid view.
    let rendered = render_validated_svg(input, theme, width, height)?;
    let prepared = input.get("prepared").unwrap_or(input);
    if prepared
        .get("nodes")
        .and_then(Value::as_array)
        .is_some_and(Vec::is_empty)
    {
        let title = prepared.get("title").and_then(Value::as_str).unwrap_or("");
        let empty_state = crate::svg::Element::new("g")
            .attr("class", "viz-empty-state")
            .child(
                crate::svg::Element::new("text")
                    .attr("x", "8")
                    .attr("y", "24")
                    .attr("font-size", "14")
                    .attr("fill", theme.text_primary)
                    .text("No elements to display."),
            )
            .child(
                crate::svg::Element::new("text")
                    .attr("x", "8")
                    .attr("y", "48")
                    .attr("font-size", "12")
                    .attr("fill", theme.text_secondary)
                    .text("Check the view's exposes and filters."),
            );
        // An empty view is still the view: its «view» frame encloses the explanation
        // (SysML v2 §§8.2.3.1, 8.2.3.26).
        let (frame, (left, top, right, bottom)) = crate::view_frame::view_frame(
            if title.is_empty() { "View" } else { title },
            (0.0, 0.0, 320.0, 64.0),
            theme,
        );
        let root = crate::svg::Element::new("g")
            .attr("class", "viz-root")
            .child(frame)
            .child(empty_state);
        return Ok(crate::render_svg_document(
            theme,
            width,
            height,
            title,
            (left, top, right - left, bottom - top),
            None,
            root,
        ));
    }
    Ok(rendered)
}

fn render_validated_svg(
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
        "browser-view" => {
            let prepared: PreparedView = decode(input, "browser-view draw input")?;
            Ok(render_browser_view_svg(&prepared, theme, width, height))
        }
        "grid-view" => {
            let prepared: PreparedView = decode(input, "grid-view draw input")?;
            Ok(render_grid_view_svg(&prepared, theme, width, height))
        }
        "geometry-view" => {
            let prepared: PreparedView = decode(input, "geometry-view draw input")?;
            Ok(render_geometry_view_svg(&prepared, theme, width, height))
        }
        "action-flow-view" | "state-transition-view" => Err(DrawError::InvalidInput(
            "action-flow-view and state-transition-view require `{ prepared, behaviorLayout }`"
                .into(),
        )),
        "general-view" => {
            let graph: GeneralViewGraph = decode(input, "general-view draw input")?;
            Ok(render_general_view_svg(&graph, theme, width, height))
        }
        other => Err(DrawError::UnsupportedView(other.to_string())),
    }
}
