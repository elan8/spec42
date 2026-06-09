//! State-machine diagram extraction driven by the workspace `SemanticGraph`.

use std::collections::HashSet;

use url::Url;

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
    if exposed_ids.is_empty() {
        return machines.to_vec();
    }

    machines
        .iter()
        .filter(|machine| {
            exposed_ids.iter().any(|exposed_id| {
                machine.id == *exposed_id
                    || machine.id.starts_with(&format!("{exposed_id}::"))
                    || format!("{}::{}", machine.package_path, machine.name).trim_matches(':')
                        == exposed_id
            })
        })
        .cloned()
        .collect()
}
