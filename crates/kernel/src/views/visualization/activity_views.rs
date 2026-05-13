use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::Url;

use crate::views::dto::SysmlVisualizationParamsDto;

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
