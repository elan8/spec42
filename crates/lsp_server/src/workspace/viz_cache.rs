//! Workspace render snapshot cache storage (no ServerState dependency).

use std::collections::HashMap;

use sysml_model::{
    ModelExplorerBundle, SysmlVisualizationResultDto, WorkspaceRenderSnapshot,
};
use tower_lsp::lsp_types::Url;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct VisualizationCacheKey {
    pub view: String,
    pub selected_view: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct WorkspaceRenderCacheEntry {
    pub semantic_state_version: u64,
    pub workspace_root_uri: Url,
    pub snapshot: WorkspaceRenderSnapshot,
    pub model_explorer: Option<ModelExplorerBundle>,
    pub visualization_responses: HashMap<VisualizationCacheKey, SysmlVisualizationResultDto>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct WorkspaceRenderCache {
    pub entry: Option<WorkspaceRenderCacheEntry>,
}

impl WorkspaceRenderCache {
    pub(crate) fn clear(&mut self) {
        self.entry = None;
    }
}
