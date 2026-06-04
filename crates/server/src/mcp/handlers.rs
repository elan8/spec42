use std::path::PathBuf;

use serde::Serialize;
use serde_json::Value;
use tower_lsp::lsp_types::NumberOrString;

use crate::cli::{CheckArgs, OutputFormat};
use crate::mcp::diagnostic_catalog;
use crate::mcp::schemas::{
    Spec42CheckParams, Spec42DoctorParams, Spec42ExplainDiagnosticParams, Spec42ModelSummaryParams,
};
use crate::{
    build_model_summary, cli_from_global, perform_check, perform_check_with_semantics, perform_doctor,
};

#[derive(Debug, Serialize)]
pub struct CatalogInfo {
    pub code: String,
    pub severity: String,
    pub meaning: String,
    pub typical_fix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor_quick_fixes: Option<Vec<String>>,
}

pub fn handle_spec42_check(arguments: Value) -> Result<Value, String> {
    let params: Spec42CheckParams =
        serde_json::from_value(arguments).map_err(|e| format!("Invalid arguments for spec42_check: {e}"))?;

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
        serde_json::to_value(&report).map_err(|e| format!("Failed to serialize validation report: {e}"))
    }
}

pub fn handle_spec42_doctor(arguments: Value) -> Result<Value, String> {
    let params: Spec42DoctorParams =
        serde_json::from_value(arguments).map_err(|e| format!("Invalid arguments for spec42_doctor: {e}"))?;

    let cli = cli_from_global(&params.global);
    let report = perform_doctor(&cli)?;
    serde_json::to_value(&report).map_err(|e| format!("Failed to serialize doctor report: {e}"))
}

pub fn handle_spec42_model_summary(arguments: Value) -> Result<Value, String> {
    let params: Spec42ModelSummaryParams = serde_json::from_value(arguments)
        .map_err(|e| format!("Invalid arguments for spec42_model_summary: {e}"))?;

    let cli = cli_from_global(&params.global);
    let check_args = CheckArgs {
        path: PathBuf::from(&params.path),
        workspace_root: params.workspace_root.map(PathBuf::from),
        format: OutputFormat::Json,
        warnings_as_errors: false,
        baseline: None,
    };

    let report = perform_check_with_semantics(&cli, &check_args)?;
    let summary = build_model_summary(report, params.max_nodes);
    serde_json::to_value(&summary).map_err(|e| format!("Failed to serialize model summary: {e}"))
}

#[derive(Debug, Serialize)]
pub struct ExplainDiagnosticResponse {
    pub code: String,
    pub catalog: Option<CatalogInfo>,
    pub known_codes_sample: Vec<&'static str>,
    pub instances: Vec<ExplainDiagnosticInstance>,
}

#[derive(Debug, Serialize)]
pub struct ExplainDiagnosticInstance {
    pub uri: String,
    pub message: String,
    pub line: u32,
    pub character: u32,
}

pub fn handle_spec42_explain_diagnostic(arguments: Value) -> Result<Value, String> {
    let params: Spec42ExplainDiagnosticParams = serde_json::from_value(arguments).map_err(|e| {
        format!("Invalid arguments for spec42_explain_diagnostic: {e}")
    })?;

    let catalog = diagnostic_catalog::lookup(&params.code).map(|entry| CatalogInfo {
        code: entry.code.to_string(),
        severity: entry.severity.to_string(),
        meaning: entry.meaning.to_string(),
        typical_fix: entry.typical_fix.to_string(),
        editor_quick_fixes: entry
            .editor_quick_fixes
            .map(|fixes| fixes.iter().map(|s| (*s).to_string()).collect()),
    });
    let mut instances = Vec::new();

    if let Some(path) = &params.path {
        let cli = cli_from_global(&params.global);
        let check_args = CheckArgs {
            path: PathBuf::from(path),
            workspace_root: params.workspace_root.map(PathBuf::from),
            format: OutputFormat::Json,
            warnings_as_errors: false,
            baseline: None,
        };
        let report = perform_check(&cli, &check_args)?;
        for doc in &report.documents {
            for diagnostic in &doc.diagnostics {
                let Some(NumberOrString::String(found)) = &diagnostic.code else {
                    continue;
                };
                if found != &params.code {
                    continue;
                }
                let line = diagnostic.range.start.line;
                if let Some(filter_line) = params.line {
                    if line != filter_line {
                        continue;
                    }
                }
                instances.push(ExplainDiagnosticInstance {
                    uri: doc.uri.clone(),
                    message: diagnostic.message.clone(),
                    line,
                    character: diagnostic.range.start.character,
                });
            }
        }
    }

    let response = ExplainDiagnosticResponse {
        code: params.code.clone(),
        catalog,
        known_codes_sample: diagnostic_catalog::all_codes(),
        instances,
    };

    serde_json::to_value(&response)
        .map_err(|e| format!("Failed to serialize explain diagnostic response: {e}"))
}
