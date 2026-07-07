//! Dispatch render preparation by renderer view id.
//!
//! Only `interconnection-view` is prepared here. For every other view kind
//! `response.graph`/`general_view_graph`/`ibd`/diagram data is always sent alongside this field
//! (unlike interconnection-view, which "slims" its payload down to `prepared_view` only — see
//! `visualization/response.rs`'s `slim_interconnection_payload`), so both the VS Code webview's
//! `prepareViewData` (`shared/diagram-renderer/src/prepare/index.ts`) and the headless SVG
//! exporter (`shared/diagram-renderer/src/headless-export.ts`) already recompute a correct
//! prepared view from that raw data whenever this field is absent. A Rust-side duplicate of that
//! TS logic used to exist for those view kinds and drifted out of parity with it (see
//! `docs/VIEW-RENDERING-ISSUES.md`, F-9) — it was removed rather than kept in sync by hand.

use crate::semantic::dto::SysmlVisualizationResultDto;
use crate::semantic::prepared_view::dto::PreparedViewDto;
use crate::semantic::prepared_view::preparers::prepare_interconnection_prepared_view;

pub fn prepare_view_from_visualization(
    response: &SysmlVisualizationResultDto,
) -> Result<PreparedViewDto, String> {
    match response.view.as_str() {
        "interconnection-view" => prepare_interconnection_prepared_view(response),
        other => Err(format!(
            "prepared view is only computed server-side for interconnection-view; {other} is recomputed client-side from raw graph/diagram data"
        )),
    }
}
