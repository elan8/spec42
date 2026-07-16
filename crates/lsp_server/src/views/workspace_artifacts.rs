//! LSP adapter for the protocol-neutral `workspace::ViewRenderCache`.

use std::collections::HashMap;
use std::time::Instant;

use sysml_model::{ModelExplorerBundle, WorkspaceParsedDocument};
use tower_lsp::lsp_types::Url;
use workspace::VisualizationBuildOutcome;

use crate::common::util;
use crate::workspace::handle::WorkspaceHandle;
use crate::workspace::state::{IndexEntry, ServerState};

pub(crate) use workspace::workspace_root_for_uri;

pub(crate) fn workspace_parsed_documents_from_index(
    index: &HashMap<Url, IndexEntry>,
) -> Vec<WorkspaceParsedDocument> {
    index
        .iter()
        .filter_map(|(uri, entry)| {
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

pub(crate) fn primary_workspace_root(state: &ServerState) -> Option<Url> {
    state.workspace_roots.first().map(util::normalize_file_uri)
}

pub(crate) async fn materialize_model_explorer_with_cache(
    handle: &WorkspaceHandle,
    state: &ServerState,
    workspace_root_uri: &Url,
) -> Result<ModelExplorerBundle, String> {
    let expected_version = state.session.version();
    let workspace_root_uri = util::normalize_file_uri(workspace_root_uri);
    if let Some(bundle) = state
        .render_cache
        .model_explorer(expected_version, &workspace_root_uri)
    {
        return Ok(bundle);
    }

    let parsed_docs = workspace_parsed_documents_from_index(&state.index);
    let mut cache = state.render_cache.clone();
    let bundle = cache.materialize_model_explorer(
        &state.semantic_graph,
        &parsed_docs,
        &state.library_paths,
        &workspace_root_uri,
        expected_version,
    )?;
    let _ = handle
        .update_render_cache(expected_version, move |c| *c = cache)
        .await;
    Ok(bundle)
}

pub(crate) async fn build_visualization_with_cache(
    handle: &WorkspaceHandle,
    state: &ServerState,
    workspace_root_uri: &Url,
    view: &str,
    selected_view: Option<&str>,
    build_start: Instant,
) -> Result<VisualizationBuildOutcome, String> {
    let expected_version = state.session.version();
    let workspace_root_uri = util::normalize_file_uri(workspace_root_uri);
    let cache_key = workspace::ViewCacheKey {
        view: view.to_string(),
        selected_view: selected_view.map(str::to_string),
    };
    if let Some(cached) =
        state
            .render_cache
            .cached_response(expected_version, &workspace_root_uri, &cache_key)
    {
        return Ok(VisualizationBuildOutcome {
            response: cached,
            meta: sysml_model::VisualizationBuildMeta {
                cache_hit: true,
                ..sysml_model::VisualizationBuildMeta::default()
            },
        });
    }

    let parsed_docs = workspace_parsed_documents_from_index(&state.index);
    let mut cache = state.render_cache.clone();
    let outcome = cache.build_visualization(
        &state.semantic_graph,
        &parsed_docs,
        &state.library_paths,
        &workspace_root_uri,
        expected_version,
        view,
        selected_view,
        build_start,
    )?;
    let _ = handle
        .update_render_cache(expected_version, move |c| *c = cache)
        .await;
    Ok(outcome)
}

#[cfg(test)]
mod cache_tests {
    use super::*;
    use crate::workspace::state::{IndexEntry, ParseMetadata, ServerState};
    use std::path::PathBuf;
    use sysml_model::{build_semantic_graph_with_provider, FileSystemDocumentProvider};

    fn drone_workspace_state() -> (WorkspaceHandle, ServerState, Url) {
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
        let mut session = workspace::WorkspaceSession::new();
        session.begin_startup();
        session.complete_startup();
        let state = ServerState {
            workspace_roots: vec![workspace_root_uri.clone()],
            session,
            index,
            symbol_table: Vec::new(),
            semantic_graph,
            ..ServerState::default()
        };
        let handle = WorkspaceHandle::spawn(state.clone());
        (handle, state, workspace_root_uri)
    }

    #[tokio::test]
    async fn warm_interconnection_visualization_hits_response_cache() {
        let (handle, state, root) = drone_workspace_state();
        let cold = build_visualization_with_cache(
            &handle,
            &state,
            &root,
            "interconnection-view",
            Some("connections"),
            Instant::now(),
        )
        .await
        .expect("cold visualization");
        assert!(
            !cold.meta.cache_hit,
            "first visualization build should miss cache"
        );
        assert!(
            cold.response.prepared_view.is_some(),
            "cold response should include preparedView for cacheability"
        );

        let state = handle.snapshot();
        let warm = build_visualization_with_cache(
            &handle,
            &state,
            &root,
            "interconnection-view",
            Some("connections"),
            Instant::now(),
        )
        .await
        .expect("warm visualization");
        assert!(
            warm.meta.cache_hit,
            "second visualization build should hit response cache"
        );
    }

    #[test]
    fn workspace_root_for_uri_prefers_longest_matching_root() {
        let uri = Url::parse("file:///c:/work/a/b/model.sysml").expect("uri");
        let roots = vec![
            Url::parse("file:///c:/work/a/").expect("root a"),
            Url::parse("file:///c:/work/a/b/").expect("root b"),
        ];
        let resolved = workspace_root_for_uri(&uri, &roots).expect("matching root");
        assert_eq!(resolved.path(), "/c:/work/a/b");
    }
}
