use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::ai_tools::perform_explain_diagnostic;
use crate::ai_tools::{ExplainDiagnosticArgs, ModelSummaryArgs};
use crate::api::error::{ApiError, ApiResult};
use crate::api::paths::resolve_workspace_path;
use crate::api::types::{
    DiagramExportRequest, DoctorHttpResponse, ElementDetail, ElementSearchResponse,
    ExplainDiagnosticHttpResponse, HealthResponse, MetaResponse, ModelProjectionRequest,
    ModelProjectionResponse, ModelSummaryHttpResponse, ModelSummaryRequest, ReadyResponse,
    ValidateRequest, ValidateResponse,
};
use crate::api::ApiServerState;
use crate::cli::{CheckArgs, OutputFormat};
use crate::diagrams;
use crate::mcp::diagnostic_catalog;
use crate::{perform_check, perform_check_with_semantics, perform_doctor, perform_model_summary};
use workspace::{HostSemanticModelNode, HostSemanticModelRelationship, HostSemanticProjection};

#[derive(Debug, Deserialize)]
pub struct ExplainQuery {
    pub path: Option<String>,
    pub line: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ElementsQuery {
    pub q: Option<String>,
    pub kind: Option<String>,
    pub uri: Option<String>,
    #[serde(default = "default_dot")]
    pub path: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

use serde::Deserialize;

fn default_dot() -> String {
    ".".to_string()
}

fn default_limit() -> usize {
    100
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

pub async fn ready(State(state): State<Arc<ApiServerState>>) -> ApiResult<Json<ReadyResponse>> {
    if !state.workspace_root.is_dir() {
        return Err(ApiError::not_ready(format!(
            "Workspace root is not a directory: {}",
            state.workspace_root.display()
        )));
    }
    Ok(Json(ReadyResponse {
        status: "ready",
        workspace_root: state.workspace_root.display().to_string(),
    }))
}

pub async fn meta(State(state): State<Arc<ApiServerState>>) -> Json<MetaResponse> {
    Json(MetaResponse {
        spec42_version: env!("CARGO_PKG_VERSION").to_string(),
        api_version: "v1",
        workspace_root: state.workspace_root.display().to_string(),
    })
}

pub async fn doctor(
    State(state): State<Arc<ApiServerState>>,
) -> ApiResult<Json<DoctorHttpResponse>> {
    let cli = state.cli.clone();
    let report = tokio::task::spawn_blocking(move || perform_doctor(&cli))
        .await
        .map_err(|err| ApiError::internal(format!("Doctor task failed: {err}")))?
        .map_err(ApiError::from_string)?;
    Ok(Json(report))
}

pub async fn validate(
    State(state): State<Arc<ApiServerState>>,
    Json(body): Json<ValidateRequest>,
) -> ApiResult<Json<ValidateResponse>> {
    let resolved = resolve_workspace_path(&state.workspace_root, &body.path)?;
    let cli = state.cli.clone();
    let workspace_root = state.workspace_root.clone();
    let args = CheckArgs {
        path: resolved,
        workspace_root: Some(workspace_root),
        format: OutputFormat::Json,
        warnings_as_errors: body.warnings_as_errors,
        baseline: None,
        strict_diagnostics: false,
    };
    let report = tokio::task::spawn_blocking(move || perform_check(&cli, &args))
        .await
        .map_err(|err| ApiError::internal(format!("Validate task failed: {err}")))?
        .map_err(ApiError::from_string)?;
    Ok(Json(report))
}

pub async fn model_summary(
    State(state): State<Arc<ApiServerState>>,
    Json(body): Json<ModelSummaryRequest>,
) -> ApiResult<Json<ModelSummaryHttpResponse>> {
    let resolved = resolve_workspace_path(&state.workspace_root, &body.path)?;
    let cli = state.cli.clone();
    let workspace_root = state.workspace_root.clone();
    let max_nodes = body.max_nodes;
    let summary = tokio::task::spawn_blocking(move || {
        perform_model_summary(
            &cli,
            &ModelSummaryArgs {
                path: resolved,
                workspace_root: Some(workspace_root),
                max_nodes,
            },
        )
    })
    .await
    .map_err(|err| ApiError::internal(format!("Model summary task failed: {err}")))?
    .map_err(ApiError::from_string)?;
    Ok(Json(summary))
}

pub async fn model_projection(
    State(state): State<Arc<ApiServerState>>,
    Json(body): Json<ModelProjectionRequest>,
) -> ApiResult<Json<ModelProjectionResponse>> {
    let resolved = resolve_workspace_path(&state.workspace_root, &body.path)?;
    let cli = state.cli.clone();
    let workspace_root = state.workspace_root.clone();
    let report = tokio::task::spawn_blocking(move || {
        perform_check_with_semantics(
            &cli,
            &CheckArgs {
                path: resolved,
                workspace_root: Some(workspace_root),
                format: OutputFormat::Json,
                warnings_as_errors: false,
                baseline: None,
                strict_diagnostics: false,
            },
        )
    })
    .await
    .map_err(|err| ApiError::internal(format!("Model projection task failed: {err}")))?
    .map_err(ApiError::from_string)?;
    Ok(Json(report))
}

pub async fn diagnostic_codes() -> Json<crate::api::types::DiagnosticCodesResponse> {
    Json(crate::api::types::DiagnosticCodesResponse {
        codes: diagnostic_catalog::all_codes()
            .into_iter()
            .map(str::to_string)
            .collect(),
    })
}

pub async fn explain_diagnostic(
    State(state): State<Arc<ApiServerState>>,
    Path(code): Path<String>,
    Query(query): Query<ExplainQuery>,
) -> ApiResult<Json<ExplainDiagnosticHttpResponse>> {
    let path = query
        .path
        .as_deref()
        .map(|p| resolve_workspace_path(&state.workspace_root, p))
        .transpose()?;
    let cli = state.cli.clone();
    let workspace_root = state.workspace_root.clone();
    let line = query.line;
    let response = tokio::task::spawn_blocking(move || {
        perform_explain_diagnostic(
            &cli,
            &ExplainDiagnosticArgs {
                code,
                path,
                workspace_root: Some(workspace_root),
                line,
            },
        )
    })
    .await
    .map_err(|err| ApiError::internal(format!("Explain diagnostic task failed: {err}")))?
    .map_err(ApiError::from_string)?;
    Ok(Json(response))
}

pub async fn elements(
    State(state): State<Arc<ApiServerState>>,
    Query(query): Query<ElementsQuery>,
) -> ApiResult<Json<ElementSearchResponse>> {
    let limit = query.limit.clamp(1, 5000);
    let resolved = resolve_workspace_path(&state.workspace_root, &query.path)?;
    let projection = load_projection(&state, resolved).await?;
    let q_lower = query.q.as_deref().map(str::to_lowercase);
    let kind = query.kind.as_deref().map(str::to_lowercase);
    let uri = query.uri.clone();

    let mut matches: Vec<HostSemanticModelNode> = projection
        .nodes
        .into_iter()
        .filter(|node| {
            if let Some(ref needle) = q_lower {
                if !node.qualified_name.to_lowercase().contains(needle)
                    && !node.name.to_lowercase().contains(needle)
                {
                    return false;
                }
            }
            if let Some(ref expected_kind) = kind {
                if node.element_kind.as_str().to_lowercase() != *expected_kind {
                    return false;
                }
            }
            if let Some(ref expected_uri) = uri {
                if node.uri != *expected_uri {
                    return false;
                }
            }
            true
        })
        .collect();

    let total = matches.len();
    let truncated = total > limit;
    matches.truncate(limit);

    Ok(Json(ElementSearchResponse {
        items: matches,
        total,
        truncated,
    }))
}

pub async fn element_by_name(
    State(state): State<Arc<ApiServerState>>,
    Path(qualified_name): Path<String>,
    Query(query): Query<ElementsQuery>,
) -> ApiResult<Json<ElementDetail>> {
    let resolved = resolve_workspace_path(&state.workspace_root, &query.path)?;
    let projection = load_projection(&state, resolved).await?;
    let element = projection
        .nodes
        .iter()
        .find(|node| node.qualified_name == qualified_name)
        .cloned()
        .ok_or_else(|| {
            ApiError::not_found(format!("No element with qualified name '{qualified_name}'"))
        })?;

    let incoming: Vec<HostSemanticModelRelationship> = projection
        .relationships
        .iter()
        .filter(|rel| rel.target == qualified_name)
        .cloned()
        .collect();
    let outgoing: Vec<HostSemanticModelRelationship> = projection
        .relationships
        .iter()
        .filter(|rel| rel.source == qualified_name)
        .cloned()
        .collect();

    Ok(Json(ElementDetail {
        element,
        incoming_relationships: incoming,
        outgoing_relationships: outgoing,
    }))
}

pub async fn diagrams_export(
    State(state): State<Arc<ApiServerState>>,
    Json(body): Json<DiagramExportRequest>,
) -> ApiResult<Response> {
    let resolved = resolve_workspace_path(&state.workspace_root, &body.path)?;
    let cli = state.cli.clone();
    let workspace_root = state.workspace_root.clone();
    let view = body.view.clone();
    let selected_view = body.selected_view.clone();
    let format = body.format;

    let (body_text, content_type) = tokio::task::spawn_blocking(move || {
        diagrams::render_diagram_for_path(
            &cli,
            resolved.as_path(),
            Some(workspace_root.as_path()),
            &view,
            selected_view.as_deref(),
            format,
        )
    })
    .await
    .map_err(|err| ApiError::internal(format!("Diagram export task failed: {err}")))?
    .map_err(ApiError::from_string)?;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
    Ok((StatusCode::OK, headers, body_text).into_response())
}

pub async fn openapi_json() -> ApiResult<Response> {
    let yaml = include_str!("../../../../docs/api/spec42-readonly-v1.openapi.yaml");
    let value: serde_json::Value = serde_yaml::from_str(yaml)
        .map_err(|err| ApiError::internal(format!("Failed to parse OpenAPI YAML: {err}")))?;
    let json = serde_json::to_string_pretty(&value)
        .map_err(|err| ApiError::internal(format!("Failed to serialize OpenAPI JSON: {err}")))?;
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    Ok((StatusCode::OK, headers, json).into_response())
}

async fn load_projection(
    state: &ApiServerState,
    path: std::path::PathBuf,
) -> ApiResult<HostSemanticProjection> {
    let cli = state.cli.clone();
    let workspace_root = state.workspace_root.clone();
    let report = tokio::task::spawn_blocking(move || {
        perform_check_with_semantics(
            &cli,
            &CheckArgs {
                path,
                workspace_root: Some(workspace_root),
                format: OutputFormat::Json,
                warnings_as_errors: false,
                baseline: None,
                strict_diagnostics: false,
            },
        )
    })
    .await
    .map_err(|err| ApiError::internal(format!("Projection task failed: {err}")))?
    .map_err(ApiError::from_string)?;
    Ok(report.semantic_model)
}
