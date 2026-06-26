//! Dispatch render preparation by renderer view id.

use crate::semantic::dto::SysmlVisualizationResultDto;
use crate::semantic::prepared_view::dto::PreparedViewDto;
use crate::semantic::prepared_view::preparers::{
    prepare_activity_prepared_view, prepare_browser_prepared_view, prepare_geometry_prepared_view,
    prepare_graph_prepared_view, prepare_grid_prepared_view, prepare_interconnection_prepared_view,
    prepare_sequence_prepared_view, prepare_state_prepared_view,
};

pub fn prepare_view_from_visualization(
    response: &SysmlVisualizationResultDto,
) -> Result<PreparedViewDto, String> {
    match response.view.as_str() {
        "interconnection-view" => prepare_interconnection_prepared_view(response),
        "general-view" => Ok(prepare_graph_prepared_view(response)),
        "browser-view" => Ok(prepare_browser_prepared_view(response)),
        "grid-view" => Ok(prepare_grid_prepared_view(response)),
        "geometry-view" => Ok(prepare_geometry_prepared_view(response)),
        "action-flow-view" => Ok(prepare_activity_prepared_view(response)),
        "state-transition-view" => Ok(prepare_state_prepared_view(response)),
        "sequence-view" => Ok(prepare_sequence_prepared_view(response)),
        other => Err(format!("unsupported prepared view renderer: {other}")),
    }
}
