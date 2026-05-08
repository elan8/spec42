use crate::{SemanticGraph, SysmlVisualizationResultDto, SysmlVisualizationViewCandidateDto};

/// Lightweight non-LSP visualization selection entrypoint.
///
/// This graph-first API keeps visualization logic independent from workspace/path scanning.
pub fn build_sysml_visualization_from_graph(
    _graph: &SemanticGraph,
    view: &str,
    selected_view: Option<&str>,
) -> Result<SysmlVisualizationResultDto, String> {
    let fallback_id = selected_view.unwrap_or("Views::general").to_string();
    let fallback_name = selected_view
        .and_then(|value| value.rsplit("::").next())
        .map(|name| name.replace('-', " "))
        .unwrap_or_else(|| "General View".to_string());
    let mut view_candidates = vec![
        SysmlVisualizationViewCandidateDto {
            id: "Views::general".to_string(),
            name: "General View".to_string(),
            renderer_view: Some("general-view".to_string()),
            supported: true,
            view_type: Some("general".to_string()),
            description: Some("semantic_core default view candidate".to_string()),
        },
        SysmlVisualizationViewCandidateDto {
            id: "Views::interconnection".to_string(),
            name: "Interconnection View".to_string(),
            renderer_view: Some("interconnection-view".to_string()),
            supported: true,
            view_type: Some("interconnection".to_string()),
            description: Some("semantic_core default view candidate".to_string()),
        },
        SysmlVisualizationViewCandidateDto {
            id: "Views::actionFlow".to_string(),
            name: "Action Flow View".to_string(),
            renderer_view: Some("action-flow-view".to_string()),
            supported: true,
            view_type: Some("action-flow".to_string()),
            description: Some("semantic_core default view candidate".to_string()),
        },
        SysmlVisualizationViewCandidateDto {
            id: "Views::stateTransition".to_string(),
            name: "State Transition View".to_string(),
            renderer_view: Some("state-transition-view".to_string()),
            supported: true,
            view_type: Some("state-transition".to_string()),
            description: Some("semantic_core default view candidate".to_string()),
        },
        SysmlVisualizationViewCandidateDto {
            id: "Views::sequence".to_string(),
            name: "Sequence View".to_string(),
            renderer_view: Some("sequence-view".to_string()),
            supported: true,
            view_type: Some("sequence".to_string()),
            description: Some("semantic_core default view candidate".to_string()),
        },
    ];
    if !view_candidates.iter().any(|candidate| candidate.id == fallback_id) {
        view_candidates.push(SysmlVisualizationViewCandidateDto {
            id: fallback_id.clone(),
            name: fallback_name.clone(),
            renderer_view: Some(view.to_string()),
            supported: true,
            view_type: Some("general".to_string()),
            description: Some("semantic_core default view candidate".to_string()),
        });
    }

    Ok(SysmlVisualizationResultDto {
        version: 1,
        view: view.to_string(),
        workspace_root_uri: String::new(),
        view_candidates,
        selected_view: Some(fallback_id),
        selected_view_name: Some(fallback_name),
        empty_state_message: None,
        package_groups: None,
        graph: None,
        general_view_graph: None,
        workspace_model: None,
        activity_diagrams: None,
        sequence_diagrams: None,
        ibd: None,
        stats: None,
    })
}
