use std::path::PathBuf;

use serde_json::Value;

use crate::ai_tools::{
    perform_explain_diagnostic, perform_model_summary, ExplainDiagnosticArgs, ModelSummaryArgs,
};
use crate::cli::{CheckArgs, OutputFormat};
use crate::mcp::schemas::{
    Spec42CheckParams, Spec42DoctorParams, Spec42ExplainDiagnosticParams, Spec42ModelSummaryParams,
};
use crate::{cli_from_global, perform_check, perform_check_with_semantics, perform_doctor};

pub use crate::ai_tools::{CatalogInfo, ExplainDiagnosticInstance, ExplainDiagnosticResponse};

pub fn handle_spec42_check(arguments: Value) -> Result<Value, String> {
    let params: Spec42CheckParams = serde_json::from_value(arguments)
        .map_err(|e| format!("Invalid arguments for spec42_check: {e}"))?;

    let cli = cli_from_global(&params.global);
    let check_args = CheckArgs {
        path: PathBuf::from(&params.path),
        workspace_root: params.workspace_root.map(PathBuf::from),
        format: OutputFormat::Json,
        warnings_as_errors: false,
        baseline: None,
    };

    if params.include_semantic_model {
        let report = perform_check_with_semantics(&cli, &check_args)?;
        serde_json::to_value(&report)
            .map_err(|e| format!("Failed to serialize semantic validation report: {e}"))
    } else {
        let report = perform_check(&cli, &check_args)?;
        serde_json::to_value(&report)
            .map_err(|e| format!("Failed to serialize validation report: {e}"))
    }
}

pub fn handle_spec42_doctor(arguments: Value) -> Result<Value, String> {
    let params: Spec42DoctorParams = serde_json::from_value(arguments)
        .map_err(|e| format!("Invalid arguments for spec42_doctor: {e}"))?;

    let cli = cli_from_global(&params.global);
    let report = perform_doctor(&cli)?;
    serde_json::to_value(&report).map_err(|e| format!("Failed to serialize doctor report: {e}"))
}

pub fn handle_spec42_model_summary(arguments: Value) -> Result<Value, String> {
    let params: Spec42ModelSummaryParams = serde_json::from_value(arguments)
        .map_err(|e| format!("Invalid arguments for spec42_model_summary: {e}"))?;

    let cli = cli_from_global(&params.global);
    let summary = perform_model_summary(
        &cli,
        &ModelSummaryArgs {
            path: PathBuf::from(&params.path),
            workspace_root: params.workspace_root.map(PathBuf::from),
            max_nodes: params.max_nodes,
        },
    )?;
    serde_json::to_value(&summary).map_err(|e| format!("Failed to serialize model summary: {e}"))
}

pub fn handle_spec42_explain_diagnostic(arguments: Value) -> Result<Value, String> {
    let params: Spec42ExplainDiagnosticParams = serde_json::from_value(arguments)
        .map_err(|e| format!("Invalid arguments for spec42_explain_diagnostic: {e}"))?;

    let cli = cli_from_global(&params.global);
    let response = perform_explain_diagnostic(
        &cli,
        &ExplainDiagnosticArgs {
            code: params.code,
            path: params.path.map(PathBuf::from),
            workspace_root: params.workspace_root.map(PathBuf::from),
            line: params.line,
        },
    )?;
    serde_json::to_value(&response)
        .map_err(|e| format!("Failed to serialize explain diagnostic response: {e}"))
}
