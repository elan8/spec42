//! Workspace visualization cache storage (no ServerState dependency).

use std::collections::HashMap;

use semantic_core::{SysmlVisualizationResultDto, WorkspaceVisualizationArtifacts};
use tower_lsp::lsp_types::Url;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct VisualizationCacheKey {
    pub view: String,
    pub selected_view: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct WorkspaceVizArtifactEntry {
    pub semantic_state_version: u64,
    pub workspace_root_uri: Url,
    pub ibd_artifact_mode: semantic_core::IbdArtifactMode,
    pub artifacts: WorkspaceVisualizationArtifacts,
}

#[derive(Debug, Clone)]
pub(crate) struct VisualizationResponseCacheEntry {
    pub semantic_state_version: u64,
    pub workspace_root_uri: Url,
    pub entries: HashMap<VisualizationCacheKey, SysmlVisualizationResultDto>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct WorkspaceVizCaches {
    pub artifacts: Option<WorkspaceVizArtifactEntry>,
    pub responses: Option<VisualizationResponseCacheEntry>,
}

impl WorkspaceVizCaches {
    pub(crate) fn clear(&mut self) {
        self.artifacts = None;
        self.responses = None;
    }
}
