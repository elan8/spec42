use tower_lsp::lsp_types::Url;

use crate::views::extracted_model::{extract_sequence_diagrams, SequenceDiagramDto};

fn stamp_fragment_uri(
    fragment: &mut crate::views::extracted_model::SequenceFragmentDto,
    uri: &str,
) {
    fragment.uri = Some(uri.to_string());
    for operand in &mut fragment.operands {
        operand.uri = Some(uri.to_string());
        for nested in &mut operand.fragments {
            stamp_fragment_uri(nested, uri);
        }
    }
    for nested in &mut fragment.fragments {
        stamp_fragment_uri(nested, uri);
    }
}

fn stamp_diagram_uri(diagram: &mut SequenceDiagramDto, uri: &str) {
    diagram.uri = Some(uri.to_string());
    for lifeline in &mut diagram.lifelines {
        lifeline.uri = Some(uri.to_string());
    }
    for message in &mut diagram.messages {
        message.uri = Some(uri.to_string());
    }
    for activation in &mut diagram.activations {
        activation.uri = Some(uri.to_string());
    }
    for fragment in &mut diagram.fragments {
        stamp_fragment_uri(fragment, uri);
    }
}

pub(crate) fn build_workspace_sequence_diagrams(
    index: &std::collections::HashMap<Url, crate::workspace::state::IndexEntry>,
    workspace_uris: &[Url],
) -> Vec<SequenceDiagramDto> {
    let mut diagrams = Vec::new();
    for workspace_uri in workspace_uris {
        let Some(entry) = index.get(workspace_uri) else {
            continue;
        };
        let Some(parsed) = entry.parsed.as_ref() else {
            continue;
        };
        let mut extracted = extract_sequence_diagrams(parsed);
        let uri = workspace_uri.as_str().to_string();
        for diagram in &mut extracted {
            stamp_diagram_uri(diagram, &uri);
        }
        diagrams.extend(extracted);
    }
    diagrams
}

pub(crate) fn filter_sequence_diagrams_by_exposed_ids(
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
                    || format!("{}::{}", diagram.package_path, diagram.name).trim_matches(':') == exposed_id
            })
        })
        .cloned()
        .collect()
}
