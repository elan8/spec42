//! Cached workspace visualization artifacts and response cache for the LSP server.

use std::collections::HashMap;
use std::time::Instant;

use semantic_core::{
    build_sysml_visualization_from_artifacts, build_workspace_visualization_artifacts,
    IbdDataDto, VisualizationBuildMeta, VisualizationBuildOptions, WorkspaceParsedDocument,
    WorkspaceVisualizationArtifacts,
};
use tower_lsp::lsp_types::Url;

use crate::common::util;
use crate::workspace::state::{IndexEntry, ServerState};
use crate::workspace::viz_cache::{
    VisualizationCacheKey, VisualizationResponseCacheEntry, WorkspaceVizArtifactEntry,
};

fn workspace_parsed_documents_for_visualization(
    index: &HashMap<Url, IndexEntry>,
    workspace_uris: &[Url],
) -> Vec<WorkspaceParsedDocument> {
    workspace_uris
        .iter()
        .filter_map(|uri| {
            let entry = index.get(uri)?;
            Some(WorkspaceParsedDocument {
                uri: uri.clone(),
                content: entry.content.clone(),
                parsed: entry.parsed.as_ref()?.clone(),
                parse_time_ms: entry.parse_metadata.parse_time_ms,
                parse_cached: entry.parse_metadata.parse_cached,
            })
        })
        .collect()
}

fn workspace_root_matches(left: &Url, right: &Url) -> bool {
    let left = util::normalize_file_uri(left);
    let right = util::normalize_file_uri(right);
    left == right
}

fn artifacts_valid(entry: &WorkspaceVizArtifactEntry, state: &ServerState, root: &Url) -> bool {
    entry.semantic_state_version == state.semantic_state_version
        && workspace_root_matches(&entry.workspace_root_uri, root)
}

fn response_cache_valid(
    entry: &VisualizationResponseCacheEntry,
    semantic_state_version: u64,
    root: &Url,
) -> bool {
    entry.semantic_state_version == semantic_state_version
        && workspace_root_matches(&entry.workspace_root_uri, root)
}

fn visualization_response_is_cacheable(response: &semantic_core::SysmlVisualizationResultDto) -> bool {
    if !response.model_ready {
        return false;
    }
    if response.view == "interconnection-view" && response.interconnection_scene.is_none() {
        return false;
    }
    true
}

pub(crate) fn clear_workspace_viz_caches(state: &mut ServerState) {
    state.workspace_viz_caches.clear();
}

pub(crate) fn ensure_workspace_artifacts(
    state: &mut ServerState,
    workspace_root_uri: &Url,
) -> Result<WorkspaceVisualizationArtifacts, String> {
    let workspace_root_uri = util::normalize_file_uri(workspace_root_uri);
    if let Some(entry) = state.workspace_viz_caches.artifacts.as_ref() {
        if artifacts_valid(entry, state, &workspace_root_uri) {
            return Ok(entry.artifacts.clone());
        }
    }

    let workspace_uris = semantic_core::workspace_uris_for_root(
        &state.semantic_graph,
        &state.library_paths,
        &workspace_root_uri,
    );
    let viz_docs = workspace_parsed_documents_for_visualization(&state.index, &workspace_uris);
    let artifacts = build_workspace_visualization_artifacts(
        &state.semantic_graph,
        &viz_docs,
        &state.library_paths,
        &workspace_root_uri,
    )?;
    state.workspace_viz_caches.artifacts = Some(WorkspaceVizArtifactEntry {
        semantic_state_version: state.semantic_state_version,
        workspace_root_uri,
        artifacts: artifacts.clone(),
    });
    state.workspace_viz_caches.responses = None;
    Ok(artifacts)
}

pub(crate) fn cached_merged_ibd(
    state: &ServerState,
    workspace_root_uri: &Url,
) -> Option<IbdDataDto> {
    let workspace_root_uri = util::normalize_file_uri(workspace_root_uri);
    let entry = state.workspace_viz_caches.artifacts.as_ref()?;
    if !artifacts_valid(entry, state, &workspace_root_uri) {
        return None;
    }
    Some(entry.artifacts.full_ibd.clone())
}

pub(crate) struct VisualizationBuildOutcome {
    pub response: semantic_core::SysmlVisualizationResultDto,
    pub meta: VisualizationBuildMeta,
}

pub(crate) fn build_visualization_with_cache(
    state: &mut ServerState,
    workspace_root_uri: &Url,
    view: &str,
    selected_view: Option<&str>,
    build_start: Instant,
    options: VisualizationBuildOptions,
) -> Result<VisualizationBuildOutcome, String> {
    let workspace_root_uri = util::normalize_file_uri(workspace_root_uri);
    let semantic_state_version = state.semantic_state_version;
    let cache_key = VisualizationCacheKey {
        view: view.to_string(),
        selected_view: selected_view.map(str::to_string),
    };

    if let Some(entry) = state.workspace_viz_caches.responses.as_ref() {
        if response_cache_valid(entry, semantic_state_version, &workspace_root_uri) {
            if let Some(cached) = entry.entries.get(&cache_key) {
                if visualization_response_is_cacheable(cached) {
                    return Ok(VisualizationBuildOutcome {
                        response: cached.clone(),
                        meta: VisualizationBuildMeta {
                            cache_hit: true,
                            ..VisualizationBuildMeta::default()
                        },
                    });
                }
            }
        }
    }

    let workspace_uris = semantic_core::workspace_uris_for_root(
        &state.semantic_graph,
        &state.library_paths,
        &workspace_root_uri,
    );
    let viz_docs = workspace_parsed_documents_for_visualization(&state.index, &workspace_uris);

    let artifacts_start = Instant::now();
    let artifacts = ensure_workspace_artifacts(state, &workspace_root_uri)?;
    let (response, mut meta) = build_sysml_visualization_from_artifacts(
        &state.semantic_graph,
        &viz_docs,
        &artifacts,
        view,
        selected_view,
        build_start,
        options,
    )?;
    meta.cache_hit = false;
    if meta.ibd_ms == 0 {
        meta.ibd_ms = artifacts_start.elapsed().as_millis().max(1) as u32;
    }

    if !state
        .workspace_viz_caches
        .responses
        .as_ref()
        .is_some_and(|entry| {
            response_cache_valid(entry, semantic_state_version, &workspace_root_uri)
        })
    {
        state.workspace_viz_caches.responses = Some(VisualizationResponseCacheEntry {
            semantic_state_version,
            workspace_root_uri: workspace_root_uri.clone(),
            entries: HashMap::new(),
        });
    }
    if visualization_response_is_cacheable(&response) {
        state
            .workspace_viz_caches
            .responses
            .as_mut()
            .expect("response cache initialized")
            .entries
            .insert(cache_key, response.clone());
    }

    Ok(VisualizationBuildOutcome {
        response,
        meta,
    })
}

pub(crate) fn primary_workspace_root(state: &ServerState) -> Option<Url> {
    state
        .workspace_roots
        .first()
        .map(util::normalize_file_uri)
}
