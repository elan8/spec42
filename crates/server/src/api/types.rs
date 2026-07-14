use serde::{Deserialize, Serialize};

use crate::ai_tools::ExplainDiagnosticResponse;
use crate::cli::DiagramExportFormat;
use crate::{DoctorReport, ModelSummaryResponse, SemanticValidationReport, ValidationReport};
use workspace::{HostSemanticModelNode, HostSemanticModelRelationship};

#[derive(Debug, Clone, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReadyResponse {
    pub status: &'static str,
    pub workspace_root: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MetaResponse {
    pub spec42_version: String,
    pub api_version: &'static str,
    pub workspace_root: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ValidateRequest {
    #[serde(default = "default_dot_path")]
    pub path: String,
    #[serde(default)]
    pub warnings_as_errors: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelSummaryRequest {
    pub path: String,
    #[serde(default = "default_max_nodes")]
    pub max_nodes: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelProjectionRequest {
    pub path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiagramExportRequest {
    pub path: String,
    pub view: String,
    #[serde(default)]
    pub selected_view: Option<String>,
    pub format: DiagramExportFormat,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticCodesResponse {
    pub codes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ElementSearchResponse {
    pub items: Vec<HostSemanticModelNode>,
    pub total: usize,
    pub truncated: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ElementDetail {
    pub element: HostSemanticModelNode,
    pub incoming_relationships: Vec<HostSemanticModelRelationship>,
    pub outgoing_relationships: Vec<HostSemanticModelRelationship>,
}

pub type ValidateResponse = ValidationReport;
pub type ModelSummaryHttpResponse = ModelSummaryResponse;
pub type ModelProjectionResponse = SemanticValidationReport;
pub type DoctorHttpResponse = DoctorReport;
pub type ExplainDiagnosticHttpResponse = ExplainDiagnosticResponse;

fn default_dot_path() -> String {
    ".".to_string()
}

fn default_max_nodes() -> usize {
    500
}
