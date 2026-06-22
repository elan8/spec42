//! Cached workspace render snapshot and lazy view bundles for the LSP server.

use std::collections::HashMap;
use std::time::Instant;

use semantic_core::{
    build_render_snapshot, build_sysml_visualization_from_render_snapshot_with_meta,
    empty_merged_ibd, full_ibd_for_render_snapshot, materialize_model_explorer_bundle,
    IbdArtifactMode, ModelExplorerBundle, VisualizationBuildMeta, VisualizationBuildOptions,
    WorkspaceParsedDocument,
};
use tower_lsp::lsp_types::Url;

use crate::common::util;
use crate::workspace::state::{IndexEntry, ServerState};
use crate::workspace::viz_cache::{VisualizationCacheKey, WorkspaceRenderCacheEntry};

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

fn render_cache_valid(entry: &WorkspaceRenderCacheEntry, state: &ServerState, root: &Url) -> bool {
    entry.semantic_state_version == state.semantic_state_version
        && workspace_root_matches(&entry.workspace_root_uri, root)
}

fn visualization_response_is_cacheable(
    response: &semantic_core::SysmlVisualizationResultDto,
) -> bool {
    if !response.model_ready {
        return false;
    }
    if response.view == "interconnection-view"
        && response.interconnection_scene.is_none()
        && response.prepared_view.is_none()
    {
        return false;
    }
    true
}

fn cached_visualization_response(
    state: &ServerState,
    workspace_root_uri: &Url,
    cache_key: &VisualizationCacheKey,
) -> Option<semantic_core::SysmlVisualizationResultDto> {
    let entry = state.workspace_render_cache.entry.as_ref()?;
    if !render_cache_valid(entry, state, workspace_root_uri) {
        return None;
    }
    let cached = entry.visualization_responses.get(cache_key)?;
    if visualization_response_is_cacheable(cached) {
        Some(cached.clone())
    } else {
        None
    }
}

pub(crate) fn clear_workspace_viz_caches(state: &mut ServerState) {
    state.workspace_render_cache.clear();
}

pub(crate) fn ensure_render_snapshot<'a>(
    state: &'a mut ServerState,
    workspace_root_uri: &Url,
) -> Result<&'a semantic_core::WorkspaceRenderSnapshot, String> {
    let workspace_root_uri = util::normalize_file_uri(workspace_root_uri);
    let cache_hit = state
        .workspace_render_cache
        .entry
        .as_ref()
        .is_some_and(|entry| render_cache_valid(entry, state, &workspace_root_uri));
    if cache_hit {
        return Ok(&state
            .workspace_render_cache
            .entry
            .as_ref()
            .expect("render cache hit")
            .snapshot);
    }

    let workspace_uris = semantic_core::workspace_uris_for_root(
        &state.semantic_graph,
        &state.library_paths,
        &workspace_root_uri,
    );
    let viz_docs = workspace_parsed_documents_for_visualization(&state.index, &workspace_uris);
    let snapshot = build_render_snapshot(
        &state.semantic_graph,
        &viz_docs,
        &state.library_paths,
        &workspace_root_uri,
        state.semantic_state_version,
    )?;
    state.workspace_render_cache.entry = Some(WorkspaceRenderCacheEntry {
        semantic_state_version: state.semantic_state_version,
        workspace_root_uri: workspace_root_uri.clone(),
        snapshot,
        model_explorer: None,
        prepared_views: HashMap::new(),
        visualization_responses: HashMap::new(),
    });
    Ok(&state
        .workspace_render_cache
        .entry
        .as_ref()
        .expect("render cache initialized")
        .snapshot)
}

pub(crate) fn materialize_model_explorer(
    state: &mut ServerState,
    workspace_root_uri: &Url,
) -> Result<ModelExplorerBundle, String> {
    let workspace_root_uri = util::normalize_file_uri(workspace_root_uri);
    ensure_render_snapshot(state, &workspace_root_uri)?;
    let entry = state
        .workspace_render_cache
        .entry
        .as_mut()
        .expect("render cache initialized");
    if entry.model_explorer.is_none() {
        entry.model_explorer = Some(materialize_model_explorer_bundle(
            &state.semantic_graph,
            &entry.snapshot,
        ));
    }
    Ok(entry.model_explorer.as_ref().expect("model explorer").clone())
}

pub(crate) fn ensure_workspace_artifacts(
    state: &mut ServerState,
    workspace_root_uri: &Url,
    ibd_artifact_mode: IbdArtifactMode,
) -> Result<semantic_core::WorkspaceVisualizationArtifacts, String> {
    let workspace_root_uri = util::normalize_file_uri(workspace_root_uri);
    let snapshot = ensure_render_snapshot(state, &workspace_root_uri)?.clone();
    let full_ibd = if ibd_artifact_mode == IbdArtifactMode::FullWorkspace {
        let cached = state
            .workspace_render_cache
            .entry
            .as_ref()
            .and_then(|entry| entry.model_explorer.as_ref())
            .map(|bundle| &bundle.full_ibd);
        full_ibd_for_render_snapshot(&state.semantic_graph, &snapshot, cached)
    } else {
        empty_merged_ibd()
    };
    Ok(semantic_core::view_index_to_artifacts(
        &snapshot.view_index,
        full_ibd,
    ))
}

pub(crate) fn cached_merged_ibd(
    state: &ServerState,
    workspace_root_uri: &Url,
) -> Option<semantic_core::IbdDataDto> {
    let workspace_root_uri = util::normalize_file_uri(workspace_root_uri);
    let entry = state.workspace_render_cache.entry.as_ref()?;
    if !render_cache_valid(entry, state, &workspace_root_uri) {
        return None;
    }
    entry.model_explorer.as_ref().map(|bundle| bundle.full_ibd.clone())
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
    let cache_key = VisualizationCacheKey {
        view: view.to_string(),
        selected_view: selected_view.map(str::to_string),
    };

    if let Some(cached) = cached_visualization_response(state, &workspace_root_uri, &cache_key) {
        return Ok(VisualizationBuildOutcome {
            response: cached,
            meta: VisualizationBuildMeta {
                cache_hit: true,
                ..VisualizationBuildMeta::default()
            },
        });
    }

    ensure_render_snapshot(state, &workspace_root_uri)?;

    let snapshot = state
        .workspace_render_cache
        .entry
        .as_ref()
        .expect("render cache initialized")
        .snapshot
        .clone();
    let workspace_uris = snapshot.workspace_uris.clone();
    let viz_docs = workspace_parsed_documents_for_visualization(&state.index, &workspace_uris);

    let ibd_artifact_mode = if options.ibd_build_scope == semantic_core::IbdBuildScope::ViewExposedPackages
        && ((options.slim_interconnection_payload && view == "interconnection-view")
            || view == "general-view")
    {
        IbdArtifactMode::Deferred
    } else {
        IbdArtifactMode::FullWorkspace
    };

    let full_ibd = if ibd_artifact_mode == IbdArtifactMode::FullWorkspace {
        let cached = state
            .workspace_render_cache
            .entry
            .as_ref()
            .and_then(|entry| entry.model_explorer.as_ref())
            .map(|bundle| &bundle.full_ibd);
        full_ibd_for_render_snapshot(&state.semantic_graph, &snapshot, cached)
    } else {
        empty_merged_ibd()
    };
    let (response, mut meta) = build_sysml_visualization_from_render_snapshot_with_meta(
        &state.semantic_graph,
        &viz_docs,
        &snapshot,
        view,
        selected_view,
        build_start,
        full_ibd,
        options,
    )?;
    meta.cache_hit = false;

    if visualization_response_is_cacheable(&response) {
        if let Some(prepared) = response.prepared_view.clone() {
            state
                .workspace_render_cache
                .entry
                .as_mut()
                .expect("render cache initialized")
                .prepared_views
                .insert(cache_key.clone(), prepared);
        }
        state
            .workspace_render_cache
            .entry
            .as_mut()
            .expect("render cache initialized")
            .visualization_responses
            .insert(cache_key, response.clone());
    }

    Ok(VisualizationBuildOutcome { response, meta })
}

pub(crate) fn primary_workspace_root(state: &ServerState) -> Option<Url> {
    state.workspace_roots.first().map(util::normalize_file_uri)
}

#[cfg(test)]
mod cache_tests {
    use super::*;
    use crate::workspace::state::{IndexEntry, ParseMetadata, SemanticLifecycle, ServerState};
    use semantic_core::{build_semantic_graph_with_provider, FileSystemDocumentProvider};
    use std::path::PathBuf;

    fn drone_workspace_state() -> (ServerState, Url) {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/drone");
        let workspace_root_uri =
            Url::from_directory_path(repo_root.canonicalize().unwrap()).unwrap();
        let provider =
            FileSystemDocumentProvider::new(repo_root.clone(), Some(repo_root), Vec::new());
        let (semantic_graph, parsed_docs) =
            build_semantic_graph_with_provider(&provider).expect("semantic graph");
        let index = parsed_docs
            .into_iter()
            .map(|doc| {
                (
                    doc.uri.clone(),
                    IndexEntry {
                        content: doc.content,
                        parsed: Some(doc.parsed),
                        parse_metadata: ParseMetadata {
                            parse_time_ms: doc.parse_time_ms,
                            parse_cached: doc.parse_cached,
                        },
                        include_in_semantic_graph: true,
                    },
                )
            })
            .collect();
        let state = ServerState {
            workspace_roots: vec![workspace_root_uri.clone()],
            semantic_lifecycle: SemanticLifecycle::Ready,
            semantic_state_version: 1,
            index,
            symbol_table: Vec::new(),
            semantic_graph,
            ..ServerState::default()
        };
        (state, workspace_root_uri)
    }

    #[test]
    fn warm_interconnection_visualization_hits_response_cache() {
        let (mut state, root) = drone_workspace_state();
        let options = semantic_core::interconnection_build_options("interconnection-view");
        let cold = build_visualization_with_cache(
            &mut state,
            &root,
            "interconnection-view",
            Some("connections"),
            Instant::now(),
            options.clone(),
        )
        .expect("cold visualization");
        assert!(
            !cold.meta.cache_hit,
            "first visualization build should miss cache"
        );
        assert!(
            cold.response.prepared_view.is_some(),
            "cold response should include preparedView for cacheability"
        );

        let warm = build_visualization_with_cache(
            &mut state,
            &root,
            "interconnection-view",
            Some("connections"),
            Instant::now(),
            options,
        )
        .expect("warm visualization");
        assert!(
            warm.meta.cache_hit,
            "second visualization build should hit response cache"
        );
    }
}
