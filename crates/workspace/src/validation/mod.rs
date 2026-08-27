//! The batch validation path: turn a set of target paths into one [`HostValidationReport`].
//!
//! This is a *batch* pipeline over the engine — it walks a directory, publishes once, and
//! collects the diagnostics the publication settled. It lives in the batch host, not in the
//! editor host: the LSP never calls it, and `server`'s `spec42 check`/MCP/HTTP surfaces are its
//! only production callers.
//!
//! There is exactly one report type ([`HostValidationReport`], shared with the snapshot's own
//! eager validation) and exactly one entry point ([`validate_paths`]). The reporting policy is
//! chosen by the caller through [`ValidationRequest::strict_diagnostics`]; protocol projection
//! (LSP diagnostics, SARIF, JUnit, text) belongs to whichever host renders the report.

use std::path::PathBuf;
use std::sync::Arc;

use crate::engine::Spec42Engine;
use crate::snapshot::HostValidationReport;

mod built_workspace;
mod report;

/// A validation pipeline hook, shared by reference so a host can install the same hook in
/// several requests.
pub type PipelineHook = Arc<dyn ValidationPipelineHook>;

/// Optional host hook around the batch validation path, for downstream edition composition.
///
/// Hooks may observe the request and adjust the rendered report; they cannot decide what a
/// diagnostic means.
pub trait ValidationPipelineHook: Send + Sync {
    fn before_validate(&self, _request: &ValidationRequest) -> Result<(), String> {
        Ok(())
    }
    fn after_validate(&self, _report: &mut HostValidationReport) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ValidationRequest {
    pub targets: Vec<PathBuf>,
    pub workspace_root: Option<PathBuf>,
    pub library_paths: Vec<PathBuf>,
    pub parallel_enabled: bool,
    /// When true, skip semantic checks after parse errors and suppress shadowed semantic
    /// warnings (the `spec42 check` reporting policy).
    pub strict_diagnostics: bool,
}

/// Validates `request.targets` with `engine`, returning the one report shape.
pub fn validate_paths(
    engine: &Spec42Engine,
    hooks: &[PipelineHook],
    request: ValidationRequest,
) -> Result<HostValidationReport, String> {
    built_workspace::validate_paths(engine, hooks, request)
}

pub use built_workspace::{
    built_workspace_input_from_snapshot, report_from_built_workspace, BuiltWorkspaceInput,
};
