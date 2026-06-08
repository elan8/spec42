//! Shared agent surfaces (MCP tools and CLI) for explain-diagnostic and model-summary.

use std::path::PathBuf;

use serde::Serialize;
use tower_lsp::lsp_types::NumberOrString;

use crate::cli::{CheckArgs, Cli, OutputFormat};
use crate::mcp::diagnostic_catalog;
use crate::{
    build_model_summary, perform_check, perform_check_with_semantics, ModelSummaryResponse,
};

#[derive(Debug, Clone)]
pub struct ExplainDiagnosticArgs {
    pub code: String,
    pub path: Option<PathBuf>,
    pub workspace_root: Option<PathBuf>,
    pub line: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct ModelSummaryArgs {
    pub path: PathBuf,
    pub workspace_root: Option<PathBuf>,
    pub max_nodes: usize,
}

#[derive(Debug, Serialize)]
pub struct CatalogInfo {
    pub code: String,
    pub severity: String,
    /// `spec_constraint` (normative SysML) or `modeling_guidance` (heuristic/tooling hint).
    pub alignment: String,
    pub meaning: String,
    pub typical_fix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor_quick_fixes: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct ExplainDiagnosticResponse {
    pub code: String,
    pub catalog: Option<CatalogInfo>,
    pub known_codes_sample: Vec<String>,
    pub instances: Vec<ExplainDiagnosticInstance>,
}

#[derive(Debug, Serialize)]
pub struct ExplainDiagnosticInstance {
    pub uri: String,
    pub message: String,
    pub line: u32,
    pub character: u32,
}

pub fn perform_explain_diagnostic(
    cli: &Cli,
    args: &ExplainDiagnosticArgs,
) -> Result<ExplainDiagnosticResponse, String> {
    let catalog = diagnostic_catalog::lookup(&args.code).map(|entry| CatalogInfo {
        code: entry.code.to_string(),
        severity: entry.severity.to_string(),
        alignment: diagnostic_catalog::alignment(entry.code).to_string(),
        meaning: entry.meaning.to_string(),
        typical_fix: entry.typical_fix.to_string(),
        editor_quick_fixes: entry
            .editor_quick_fixes
            .map(|fixes| fixes.iter().map(|s| (*s).to_string()).collect()),
    });
    let mut instances = Vec::new();

    if let Some(path) = &args.path {
        let check_args = CheckArgs {
            path: path.clone(),
            workspace_root: args.workspace_root.clone(),
            format: OutputFormat::Json,
            warnings_as_errors: false,
            baseline: None,
        };
        let report = perform_check(cli, &check_args)?;
        for doc in &report.documents {
            for diagnostic in &doc.diagnostics {
                let Some(NumberOrString::String(found)) = &diagnostic.code else {
                    continue;
                };
                if found != &args.code {
                    continue;
                }
                let line = diagnostic.range.start.line;
                if let Some(filter_line) = args.line {
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

    Ok(ExplainDiagnosticResponse {
        code: args.code.clone(),
        catalog,
        known_codes_sample: diagnostic_catalog::all_codes()
            .into_iter()
            .map(str::to_string)
            .collect(),
        instances,
    })
}

pub fn perform_model_summary(
    cli: &Cli,
    args: &ModelSummaryArgs,
) -> Result<ModelSummaryResponse, String> {
    let check_args = CheckArgs {
        path: args.path.clone(),
        workspace_root: args.workspace_root.clone(),
        format: OutputFormat::Json,
        warnings_as_errors: false,
        baseline: None,
    };
    let report = perform_check_with_semantics(cli, &check_args)?;
    Ok(build_model_summary(report, args.max_nodes))
}
