//! State-machine diagram extraction driven by the workspace `SemanticGraph`.

use std::collections::HashSet;

use url::Url;

use crate::semantic::exposed_ids::filter_by_exposed_ids;
use crate::semantic::extracted_model::StateMachineDto;
use crate::SemanticGraph;

mod graph_extractor;

pub fn build_workspace_state_machines(
    semantic_graph: &SemanticGraph,
    workspace_uris: &[Url],
) -> Vec<StateMachineDto> {
    graph_extractor::extract_state_machines(semantic_graph, workspace_uris)
}

pub fn filter_state_machines_by_exposed_ids(
    machines: &[StateMachineDto],
    exposed_ids: &HashSet<String>,
) -> Vec<StateMachineDto> {
    filter_by_exposed_ids(machines, exposed_ids, |machine| {
        (
            machine.id.as_str(),
            machine.package_path.as_str(),
            machine.name.as_str(),
        )
    })
}
