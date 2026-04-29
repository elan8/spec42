use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::Url;

use crate::views::dto::SysmlVisualizationParamsDto;
use crate::views::extracted_model::{extract_activity_diagrams, ActivityDiagramDto};

fn normalize_package_path(value: &str) -> String {
    value.replace('.', "::").trim().to_string()
}

fn diagram_matches_package_filter(
    diagram: &ActivityDiagramDto,
    package_ref: &str,
    package_name: Option<&str>,
) -> bool {
    let diagram_path = normalize_package_path(&diagram.package_path);
    let normalized_ref = normalize_package_path(package_ref);
    let normalized_name = package_name.map(normalize_package_path);

    if !normalized_ref.is_empty()
        && (diagram_path == normalized_ref
            || diagram_path.starts_with(&format!("{normalized_ref}::")))
    {
        return true;
    }

    if let Some(name) = normalized_name {
        if !name.is_empty()
            && (diagram_path == name || diagram_path.starts_with(&format!("{name}::")))
        {
            return true;
        }
    }

    false
}

pub(crate) fn build_workspace_activity_diagrams(
    index: &std::collections::HashMap<Url, crate::workspace::state::IndexEntry>,
    workspace_uris: &[Url],
    package_filter: Option<(&str, Option<&str>)>,
) -> Vec<ActivityDiagramDto> {
    let mut diagrams = Vec::new();
    for workspace_uri in workspace_uris {
        let Some(entry) = index.get(workspace_uri) else {
            continue;
        };
        let Some(parsed) = entry.parsed.as_ref() else {
            continue;
        };
        let source_uri = workspace_uri.as_str().to_string();
        let mut extracted = extract_activity_diagrams(parsed);
        for diagram in &mut extracted {
            if diagram.uri.is_none() {
                diagram.uri = Some(source_uri.clone());
            }
            for action in &mut diagram.actions {
                if action.uri.is_none() {
                    action.uri = Some(source_uri.clone());
                }
            }
        }
        diagrams.extend(extracted);
    }

    if let Some((package_ref, package_name)) = package_filter {
        diagrams
            .retain(|diagram| diagram_matches_package_filter(diagram, package_ref, package_name));
    }

    diagrams
}

pub(crate) fn parse_sysml_visualization_params(
    v: &serde_json::Value,
) -> Result<(Url, String, Option<String>)> {
    let params =
        if let Ok(params) = serde_json::from_value::<SysmlVisualizationParamsDto>(v.clone()) {
            params
        } else if let Some(arr) = v.as_array() {
            let first = arr.first().ok_or_else(|| {
                tower_lsp::jsonrpc::Error::invalid_params(
                    "sysml/visualization params array must have at least one element",
                )
            })?;
            if let Some(obj) = first.as_object() {
                SysmlVisualizationParamsDto {
                    workspace_root_uri: obj
                        .get("workspaceRootUri")
                        .and_then(|value| value.as_str())
                        .unwrap_or_default()
                        .to_string(),
                    view: obj
                        .get("view")
                        .and_then(|value| value.as_str())
                        .or_else(|| arr.get(1).and_then(|value| value.as_str()))
                        .unwrap_or_default()
                        .to_string(),
                    selected_view: obj
                        .get("selectedView")
                        .and_then(|value| value.as_str())
                        .or_else(|| arr.get(2).and_then(|value| value.as_str()))
                        .map(ToString::to_string),
                }
            } else {
                SysmlVisualizationParamsDto {
                    workspace_root_uri: first.as_str().unwrap_or_default().to_string(),
                    view: arr
                        .get(1)
                        .and_then(|value| value.as_str())
                        .unwrap_or_default()
                        .to_string(),
                    selected_view: arr
                        .get(2)
                        .and_then(|value| value.as_str())
                        .map(ToString::to_string),
                }
            }
        } else {
            return Err(tower_lsp::jsonrpc::Error::invalid_params(
                "sysml/visualization params must include workspaceRootUri and view",
            ));
        };

    if params.workspace_root_uri.trim().is_empty() || params.view.trim().is_empty() {
        return Err(tower_lsp::jsonrpc::Error::invalid_params(
            "sysml/visualization params must include workspaceRootUri and view",
        ));
    }

    let workspace_root_uri = Url::parse(&params.workspace_root_uri).map_err(|_| {
        tower_lsp::jsonrpc::Error::invalid_params("sysml/visualization: invalid workspaceRootUri")
    })?;

    Ok((
        crate::common::util::normalize_file_uri(&workspace_root_uri),
        params.view,
        params.selected_view,
    ))
}

pub(crate) fn top_level_package_for_node_id(node_id: &str) -> String {
    normalize_package_path(node_id)
        .split("::")
        .next()
        .unwrap_or("")
        .to_string()
}

pub(crate) fn filter_activity_diagrams_by_graph(
    diagrams: &[ActivityDiagramDto],
    graph: &crate::views::dto::SysmlGraphDto,
) -> Vec<ActivityDiagramDto> {
    let mut action_keys = std::collections::HashSet::new();
    for node in &graph.nodes {
        let kind = node.element_type.to_lowercase();
        if kind.contains("action") || kind.contains("perform") {
            action_keys.insert((node.name.clone(), top_level_package_for_node_id(&node.id)));
        }
    }

    diagrams
        .iter()
        .filter(|diagram| {
            let package = normalize_package_path(&diagram.package_path)
                .split("::")
                .next()
                .unwrap_or("")
                .to_string();
            action_keys.contains(&(diagram.name.clone(), package))
        })
        .cloned()
        .collect()
}
