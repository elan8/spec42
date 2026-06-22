//! Workspace load request parameters.

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct WorkspaceLoadRequest {
    pub targets: Vec<PathBuf>,
    pub workspace_root: Option<PathBuf>,
    pub strict_diagnostics: bool,
}

impl WorkspaceLoadRequest {
    pub fn single_target(path: PathBuf) -> Self {
        Self {
            targets: vec![path],
            workspace_root: None,
            strict_diagnostics: false,
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
}
