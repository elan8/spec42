//! Workspace load request parameters.

use std::path::PathBuf;

/// When host validation diagnostics are collected during workspace load.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValidationTiming {
    /// Collect validation during `load_workspace` (default).
    #[default]
    Eager,
    /// Skip validation during load; call `HostWorkspaceSnapshot::ensure_validation()` on demand.
    Deferred,
}

#[derive(Debug, Clone)]
pub struct WorkspaceLoadRequest {
    pub targets: Vec<PathBuf>,
    pub workspace_root: Option<PathBuf>,
    pub strict_diagnostics: bool,
    pub validation_timing: ValidationTiming,
}

impl WorkspaceLoadRequest {
    pub fn single_target(path: PathBuf) -> Self {
        Self {
            targets: vec![path],
            workspace_root: None,
            strict_diagnostics: false,
            validation_timing: ValidationTiming::Eager,
        }
    }

    pub fn with_workspace_root(mut self, workspace_root: Option<PathBuf>) -> Self {
        self.workspace_root = workspace_root;
        self
    }

    pub fn with_strict_diagnostics(mut self, strict: bool) -> Self {
        self.strict_diagnostics = strict;
        self
    }

    pub fn with_validation_timing(mut self, timing: ValidationTiming) -> Self {
        self.validation_timing = timing;
        self
    }
}
