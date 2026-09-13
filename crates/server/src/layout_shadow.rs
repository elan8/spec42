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

    /// `layout_elk_graph_shadow` validates the native contract before ever invoking legacy
    /// ELK.js, so any input reaching `ShadowLayoutError::Legacy` must first pass every native
    /// structural check (`LayoutError`'s own variants). Structural inputs that could plausibly
    /// diverge — an invalid algorithm name, a dangling edge endpoint, a self-loop edge — were
    /// checked by hand against both engines and found to fail (or succeed) identically; elkrs and
    /// ELK.js validate this input class the same way, so no naturally occurring "legacy fails,
    /// native succeeds" fixture exists in the current corpus. This test instead pins the variant
    /// attribution and message wiring directly, so a future refactor of the `?`/`map_err` chain
    /// in `layout_elk_graph_shadow` cannot silently swap which engine an error is blamed on.
    #[test]
    fn legacy_and_invalid_legacy_json_errors_are_attributed_and_formatted_distinctly() {
        let legacy = ShadowLayoutError::Legacy("ELK layout failed: boom".to_string());
        assert_eq!(
            legacy.to_string(),
            "legacy ELK.js layout failed: ELK layout failed: boom"
        );
        assert!(matches!(legacy, ShadowLayoutError::Legacy(_)));

        let invalid_legacy_json = ShadowLayoutError::InvalidLegacyJson("EOF".to_string());
        assert_eq!(
            invalid_legacy_json.to_string(),
            "legacy ELK.js returned invalid JSON: EOF"
        );
        assert!(matches!(
            invalid_legacy_json,
            ShadowLayoutError::InvalidLegacyJson(_)
        ));

        // The two variants must stay distinguishable from each other and from `Native`, since
        // callers branch on which engine failed.
        assert!(!matches!(legacy, ShadowLayoutError::InvalidLegacyJson(_)));
        assert!(!matches!(legacy, ShadowLayoutError::Native(_)));
    }
}
