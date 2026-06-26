//! Sequence-diagram extraction driven by the workspace `SemanticGraph`.
//!
//! Identification of scenarios, lifelines, message kinds, activations, and
//! fragments is based on the resolved specialization closure of the workspace
//! semantic model rather than per-file AST string matching, which means
//! cross-file specialization (e.g. `CheckoutFlow :> CommerceInteractionScenario
//! :> InteractionScenario`) is recognized correctly.

use url::Url;

use crate::semantic::extracted_model::SequenceDiagramDto;
use crate::SemanticGraph;

mod graph_extractor;

pub fn build_workspace_sequence_diagrams(
    semantic_graph: &SemanticGraph,
    workspace_uris: &[Url],
) -> Vec<SequenceDiagramDto> {
    graph_extractor::extract_sequence_diagrams(semantic_graph, workspace_uris)
}

pub fn filter_sequence_diagrams_by_exposed_ids(
    diagrams: &[SequenceDiagramDto],
    exposed_ids: &std::collections::HashSet<String>,
) -> Vec<SequenceDiagramDto> {
    if exposed_ids.is_empty() {
        return diagrams.to_vec();
    }

    diagrams
        .iter()
        .filter(|diagram| {
            exposed_ids.iter().any(|exposed_id| {
                diagram.id == *exposed_id
                    || diagram.id.starts_with(&format!("{exposed_id}::"))
                    || format!("{}::{}", diagram.package_path, diagram.name).trim_matches(':')
                        == exposed_id
            })
        })
        .cloned()
        .collect()
}
