use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

use kernel::{CustomRpcContext, CustomRpcProvider};
use serde::{Deserialize, Serialize};
use tower_lsp::jsonrpc::{Error, Result};
use tower_lsp::lsp_types::Url;

use crate::software_architecture::{
    analyze_rust_workspace, SoftwareArchitectureModel, SoftwareWorkspaceModel,
};

const SOFTWARE_VISUALIZATION: &str = "software/visualization";
const SOFTWARE_ANALYZE_WORKSPACE: &str = "software/analyzeWorkspace";
const SOFTWARE_PROJECT_VIEW: &str = "software/projectView";

pub struct SoftwareRpcProvider;

impl CustomRpcProvider for SoftwareRpcProvider {
    fn custom_method_names(&self) -> Vec<String> {
        vec![
            SOFTWARE_VISUALIZATION.to_string(),
            SOFTWARE_ANALYZE_WORKSPACE.to_string(),
            SOFTWARE_PROJECT_VIEW.to_string(),
        ]
    }

    fn try_handle(
        &self,
        method: &str,
        params: serde_json::Value,
        _context: CustomRpcContext<'_>,
    ) -> Result<Option<serde_json::Value>> {
        let result = match method {
            SOFTWARE_VISUALIZATION => {
                let result = software_visualization_result(params)?;
                Some(to_json_value(&result)?)
            }
            SOFTWARE_ANALYZE_WORKSPACE => {
                let result = software_analyze_workspace_result(params)?;
                Some(to_json_value(&result)?)
            }
            SOFTWARE_PROJECT_VIEW => {
                let result = software_project_view_result(params)?;
                Some(to_json_value(&result)?)
            }
            _ => None,
        };
        Ok(result)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SoftwareVisualizationParamsDto {
    workspace_root_uri: String,
    view: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SoftwareAnalyzeWorkspaceParamsDto {
    workspace_root_uri: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SoftwareProjectViewParamsDto {
    workspace_root_uri: String,
    view: String,
    model: SoftwareWorkspaceModelDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PositionDto {
    line: u32,
    character: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RangeDto {
    start: PositionDto,
    end: PositionDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SourceAnchorDto {
    file_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<RangeDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SoftwareComponentDto {
    id: String,
    name: String,
    kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
    crate_name: String,
    module_path: String,
    anchors: Vec<SourceAnchorDto>,
    is_external: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SoftwareDependencyDto {
    from: String,
    to: String,
    kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_anchor: Option<SourceAnchorDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SoftwareArchitectureModelDto {
    components: Vec<SoftwareComponentDto>,
    dependencies: Vec<SoftwareDependencyDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SoftwareAnalysisSummaryDto {
    crate_count: usize,
    module_count: usize,
    dependency_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SoftwareWorkspaceModelDto {
    workspace_root: String,
    architecture: SoftwareArchitectureModelDto,
    summary: SoftwareAnalysisSummaryDto,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct GraphNodeDto {
    id: String,
    node_type: String,
    name: String,
    parent_id: Option<String>,
    attributes: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct GraphEdgeDto {
    source: String,
    target: String,
    edge_type: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct GraphDto {
    nodes: Vec<GraphNodeDto>,
    edges: Vec<GraphEdgeDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SoftwareVisualizationResultDto {
    version: u32,
    view: String,
    workspace_root_uri: String,
    graph: GraphDto,
    model: SoftwareWorkspaceModelDto,
    build_time_ms: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SoftwareAnalyzeWorkspaceResultDto {
    version: u32,
    workspace_model: SoftwareWorkspaceModelDto,
}

fn parse_object_or_singleton_array<T: for<'de> Deserialize<'de>>(
    params: &serde_json::Value,
) -> Result<T> {
    serde_json::from_value::<T>(params.clone()).or_else(|_| {
        if let Some(first) = params.as_array().and_then(|items| items.first()) {
            serde_json::from_value(first.clone())
        } else {
            serde_json::from_value(serde_json::Value::Null)
        }
    })
    .map_err(|err| Error::invalid_params(err.to_string()))
}

fn parse_workspace_root_uri(uri: &str, method: &str) -> Result<(Url, PathBuf)> {
    let uri = Url::parse(uri).map_err(|_| {
        Error::invalid_params(format!("{method}: invalid workspaceRootUri (not a URI)"))
    })?;
    let path = uri.to_file_path().map_err(|_| {
        Error::invalid_params(format!("{method}: invalid workspaceRootUri (not a file URI)"))
    })?;
    Ok((uri, path))
}

fn software_visualization_result(params: serde_json::Value) -> Result<SoftwareVisualizationResultDto> {
    let parsed: SoftwareVisualizationParamsDto = parse_object_or_singleton_array(&params)?;
    let (workspace_root_uri, workspace_path) =
        parse_workspace_root_uri(&parsed.workspace_root_uri, SOFTWARE_VISUALIZATION)?;
    let build_start = Instant::now();
    let model = analyze_rust_workspace(&workspace_path);
    Ok(build_visualization_response(
        workspace_root_uri.as_str().to_string(),
        parsed.view,
        model,
        build_start,
    ))
}

fn software_analyze_workspace_result(
    params: serde_json::Value,
) -> Result<SoftwareAnalyzeWorkspaceResultDto> {
    let parsed: SoftwareAnalyzeWorkspaceParamsDto = parse_object_or_singleton_array(&params)?;
    let (_workspace_root_uri, workspace_path) =
        parse_workspace_root_uri(&parsed.workspace_root_uri, SOFTWARE_ANALYZE_WORKSPACE)?;
    let model = analyze_rust_workspace(&workspace_path);
    Ok(SoftwareAnalyzeWorkspaceResultDto {
        version: 0,
        workspace_model: workspace_model_to_dto(&model),
    })
}

fn software_project_view_result(params: serde_json::Value) -> Result<SoftwareVisualizationResultDto> {
    let parsed: SoftwareProjectViewParamsDto = parse_object_or_singleton_array(&params)?;
    let (workspace_root_uri, _workspace_path) =
        parse_workspace_root_uri(&parsed.workspace_root_uri, SOFTWARE_PROJECT_VIEW)?;
    let build_start = Instant::now();
    Ok(build_visualization_response(
        workspace_root_uri.as_str().to_string(),
        parsed.view,
        workspace_model_from_dto(&parsed.model),
        build_start,
    ))
}

fn build_visualization_response(
    workspace_root_uri: String,
    view: String,
    model: SoftwareWorkspaceModel,
    build_start: Instant,
) -> SoftwareVisualizationResultDto {
    SoftwareVisualizationResultDto {
        version: 0,
        view,
        workspace_root_uri,
        graph: build_graph(&model.architecture),
        model: workspace_model_to_dto(&model),
        build_time_ms: build_start.elapsed().as_millis().max(1) as u32,
    }
}

fn build_graph(model: &SoftwareArchitectureModel) -> GraphDto {
    GraphDto {
        nodes: model
            .components
            .iter()
            .map(|component| GraphNodeDto {
                id: component.id.clone(),
                node_type: component.kind.clone(),
                name: component.name.clone(),
                parent_id: component.parent_id.clone(),
                attributes: HashMap::from([
                    (
                        "crateName".to_string(),
                        serde_json::Value::String(component.crate_name.clone()),
                    ),
                    (
                        "modulePath".to_string(),
                        serde_json::Value::String(component.module_path.clone()),
                    ),
                    ("isExternal".to_string(), serde_json::Value::Bool(component.is_external)),
                ]),
            })
            .collect(),
        edges: model
            .dependencies
            .iter()
            .map(|dependency| GraphEdgeDto {
                source: dependency.from.clone(),
                target: dependency.to.clone(),
                edge_type: dependency.kind.clone(),
            })
            .collect(),
    }
}

fn workspace_model_to_dto(model: &SoftwareWorkspaceModel) -> SoftwareWorkspaceModelDto {
    SoftwareWorkspaceModelDto {
        workspace_root: model.workspace_root.clone(),
        architecture: SoftwareArchitectureModelDto {
            components: model
                .architecture
                .components
                .iter()
                .map(|component| SoftwareComponentDto {
                    id: component.id.clone(),
                    name: component.name.clone(),
                    kind: component.kind.clone(),
                    parent_id: component.parent_id.clone(),
                    crate_name: component.crate_name.clone(),
                    module_path: component.module_path.clone(),
                    anchors: component
                        .anchors
                        .iter()
                        .map(|anchor| SourceAnchorDto {
                            file_path: anchor.file_path.clone(),
                            range: anchor.range.map(|range| RangeDto {
                                start: PositionDto {
                                    line: range.start.line,
                                    character: range.start.character,
                                },
                                end: PositionDto {
                                    line: range.end.line,
                                    character: range.end.character,
                                },
                            }),
                        })
                        .collect(),
                    is_external: component.is_external,
                })
                .collect(),
            dependencies: model
                .architecture
                .dependencies
                .iter()
                .map(|dependency| SoftwareDependencyDto {
                    from: dependency.from.clone(),
                    to: dependency.to.clone(),
                    kind: dependency.kind.clone(),
                    source_anchor: dependency.source_anchor.as_ref().map(|anchor| SourceAnchorDto {
                        file_path: anchor.file_path.clone(),
                        range: anchor.range.map(|range| RangeDto {
                            start: PositionDto {
                                line: range.start.line,
                                character: range.start.character,
                            },
                            end: PositionDto {
                                line: range.end.line,
                                character: range.end.character,
                            },
                        }),
                    }),
                })
                .collect(),
        },
        summary: SoftwareAnalysisSummaryDto {
            crate_count: model.summary.crate_count,
            module_count: model.summary.module_count,
            dependency_count: model.summary.dependency_count,
        },
    }
}

fn workspace_model_from_dto(model: &SoftwareWorkspaceModelDto) -> SoftwareWorkspaceModel {
    use crate::software_architecture::{
        SoftwareAnalysisSummary, SoftwareArchitectureModel, SoftwareComponent, SoftwareDependency,
        SourceAnchor,
    };

    SoftwareWorkspaceModel {
        workspace_root: model.workspace_root.clone(),
        architecture: SoftwareArchitectureModel {
            components: model
                .architecture
                .components
                .iter()
                .map(|component| SoftwareComponent {
                    id: component.id.clone(),
                    name: component.name.clone(),
                    kind: component.kind.clone(),
                    parent_id: component.parent_id.clone(),
                    crate_name: component.crate_name.clone(),
                    module_path: component.module_path.clone(),
                    anchors: component
                        .anchors
                        .iter()
                        .map(|anchor| SourceAnchor {
                            file_path: anchor.file_path.clone(),
                            range: anchor.range.as_ref().map(|range| tower_lsp::lsp_types::Range {
                                start: tower_lsp::lsp_types::Position {
                                    line: range.start.line,
                                    character: range.start.character,
                                },
                                end: tower_lsp::lsp_types::Position {
                                    line: range.end.line,
                                    character: range.end.character,
                                },
                            }),
                        })
                        .collect(),
                    is_external: component.is_external,
                })
                .collect(),
            dependencies: model
                .architecture
                .dependencies
                .iter()
                .map(|dependency| SoftwareDependency {
                    from: dependency.from.clone(),
                    to: dependency.to.clone(),
                    kind: dependency.kind.clone(),
                    source_anchor: dependency.source_anchor.as_ref().map(|anchor| SourceAnchor {
                        file_path: anchor.file_path.clone(),
                        range: anchor.range.as_ref().map(|range| tower_lsp::lsp_types::Range {
                            start: tower_lsp::lsp_types::Position {
                                line: range.start.line,
                                character: range.start.character,
                            },
                            end: tower_lsp::lsp_types::Position {
                                line: range.end.line,
                                character: range.end.character,
                            },
                        }),
                    }),
                })
                .collect(),
        },
        summary: SoftwareAnalysisSummary {
            crate_count: model.summary.crate_count,
            module_count: model.summary.module_count,
            dependency_count: model.summary.dependency_count,
        },
    }
}

fn to_json_value<T: Serialize>(value: &T) -> Result<serde_json::Value> {
    serde_json::to_value(value).map_err(|_| Error::internal_error())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::Instant;

    #[test]
    fn provider_handles_analyze_workspace_method() {
        let temp = tempfile::tempdir().expect("temp dir");
        fs::write(
            temp.path().join("Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .expect("write Cargo.toml");
        fs::create_dir_all(temp.path().join("src")).expect("create src");
        fs::write(temp.path().join("src/lib.rs"), "pub struct Demo;\n").expect("write lib");

        let workspace_root_uri = Url::from_directory_path(temp.path())
            .expect("workspace uri")
            .to_string();
        let params = serde_json::json!({
            "workspaceRootUri": workspace_root_uri
        });
        let cfg = kernel::Spec42Config::new();
        let provider = SoftwareRpcProvider;
        let result = provider
            .try_handle(
                SOFTWARE_ANALYZE_WORKSPACE,
                params,
                kernel::CustomRpcContext {
                    config: &cfg,
                    server_name: "spec42-test",
                    server_start_time: Instant::now(),
                },
            )
            .expect("rpc result");
        let payload = result.expect("method handled");
        assert_eq!(payload.get("version"), Some(&serde_json::Value::from(0_u32)));
        assert!(
            payload.get("workspaceModel").is_some(),
            "expected workspaceModel in analyze response"
        );
    }

    #[test]
    fn provider_ignores_unknown_method() {
        let cfg = kernel::Spec42Config::new();
        let provider = SoftwareRpcProvider;
        let result = provider
            .try_handle(
                "software/unknown",
                serde_json::json!({}),
                kernel::CustomRpcContext {
                    config: &cfg,
                    server_name: "spec42-test",
                    server_start_time: Instant::now(),
                },
            )
            .expect("rpc result");
        assert!(result.is_none());
    }
}
