//! Feature-gated seam for measuring native layout without changing the renderer's result.
//!
//! Callers receive the legacy ELK.js output as primary plus the native output and an exact JSON
//! equality signal. A failure in either engine is surfaced; shadow mode never silently falls back.

use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShadowLayoutError {
    #[error("legacy ELK.js layout failed: {0}")]
    Legacy(String),
    #[error("legacy ELK.js returned invalid JSON: {0}")]
    InvalidLegacyJson(String),
    #[error("native elkrs layout failed: {0}")]
    Native(#[from] diagram_layout::LayoutError),
}

#[derive(Debug, Serialize)]
pub struct ShadowLayoutResult {
    /// The unchanged result that existing server consumers should render during shadow rollout.
    pub primary: Value,
    /// Normalized native output retained for diagnostics and corpus comparison.
    pub native: Value,
    /// Strict JSON equality. Geometry parity remains the release gate in `elkrs_parity`.
    pub exactly_equal: bool,
}

pub fn layout_elk_graph_shadow(input: &str) -> Result<ShadowLayoutResult, ShadowLayoutError> {
    // Validate the native contract first so unsupported graph ownership is reported precisely
    // instead of being obscured by whatever the legacy worker does with that input.
    let native = diagram_layout::layout_json(input)?;
    let legacy_json =
        super::elk_layout::layout_elk_graph(input).map_err(ShadowLayoutError::Legacy)?;
    let primary: Value = serde_json::from_str(&legacy_json)
        .map_err(|error| ShadowLayoutError::InvalidLegacyJson(error.to_string()))?;
    let exactly_equal = primary == native;
    Ok(ShadowLayoutResult {
        primary,
        native,
        exactly_equal,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadows_without_changing_the_primary_result() {
        let input = serde_json::json!({
            "id": "root",
            "layoutOptions": {
                "elk.algorithm": "layered",
                "elk.direction": "RIGHT",
                "elk.json.edgeCoords": "ROOT"
            },
            "children": [
                { "id": "a", "width": 100.0, "height": 40.0 },
                { "id": "b", "width": 100.0, "height": 40.0 }
            ],
            "edges": [{ "id": "e", "sources": ["a"], "targets": ["b"] }]
        });
        let legacy = super::super::elk_layout::layout_elk_graph(&input.to_string()).unwrap();
        let result = layout_elk_graph_shadow(&input.to_string()).unwrap();
        assert_eq!(
            result.primary,
            serde_json::from_str::<Value>(&legacy).unwrap()
        );
        assert!(result.native["edges"][0]["sections"].is_array());
    }

    #[test]
    fn surfaces_native_contract_failures() {
        let input = serde_json::json!({
            "id": "root",
            "children": [{ "id": "container", "edges": [{ "id": "nested" }] }]
        });
        assert!(matches!(
            layout_elk_graph_shadow(&input.to_string()),
            Err(ShadowLayoutError::Native(
                diagram_layout::LayoutError::NestedInputEdges { .. }
            ))
        ));
    }
}
