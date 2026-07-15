//! Version-keyed workspace view render cache shared by LSP and other hosts.

use std::collections::HashMap;
use std::time::Instant;

use language_service::uri::normalize_uri;
use sysml_model::{
    ModelExplorerBundle, SemanticGraph, SysmlVisualizationResultDto, VisualizationBuildMeta,
    WorkspaceParsedDocument, WorkspaceRenderSnapshot, materialize_model_explorer_bundle,
};
use url::Url;

use crate::{build_view_catalog, render_view};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ViewCacheKey {
    pub view: String,
    pub selected_view: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ViewRenderCacheEntry {
    pub semantic_state_version: u64,
    pub workspace_root_uri: Url,
    pub snapshot: WorkspaceRenderSnapshot,
    pub model_explorer: Option<ModelExplorerBundle>,
    pub visualization_responses: HashMap<ViewCacheKey, SysmlVisualizationResultDto>,
}

#[derive(Debug, Clone, Default)]
pub struct ViewRenderCache {
    entry: Option<ViewRenderCacheEntry>,
}

#[derive(Debug, Clone)]
pub struct VisualizationBuildOutcome {
    pub response: SysmlVisualizationResultDto,
    pub meta: VisualizationBuildMeta,
}

fn workspace_root_matches(left: &Url, right: &Url) -> bool {
    normalize_uri(left) == normalize_uri(right)
}

/// Returns true when `response` is safe to store and serve from the warm cache.
pub fn visualization_response_is_cacheable(response: &SysmlVisualizationResultDto) -> bool {
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

/// Pick the longest workspace root whose path prefix contains `uri`.
pub fn workspace_root_for_uri(uri: &Url, roots: &[Url]) -> Option<Url> {
    let normalized_uri = normalize_uri(uri);
    roots
        .iter()
        .map(normalize_uri)
        .filter(|root| normalized_uri.path().starts_with(root.path()))
        .max_by_key(|root| root.path().len())
}

impl ViewRenderCache {
    pub fn clear(&mut self) {
        self.entry = None;
    }

    pub fn is_valid(&self, semantic_state_version: u64, workspace_root_uri: &Url) -> bool {
        let Some(entry) = self.entry.as_ref() else {
            return false;
        };
        entry.semantic_state_version == semantic_state_version
            && workspace_root_matches(&entry.workspace_root_uri, workspace_root_uri)
    }

    pub fn cached_response(
        &self,
        semantic_state_version: u64,
        workspace_root_uri: &Url,
        cache_key: &ViewCacheKey,
    ) -> Option<SysmlVisualizationResultDto> {
        let entry = self.entry.as_ref()?;
        if !self.is_valid(semantic_state_version, workspace_root_uri) {
            return None;
        }
        let cached = entry.visualization_responses.get(cache_key)?;
        if visualization_response_is_cacheable(cached) {
            Some(cached.clone())
        } else {
            None
        }
    }

    pub fn model_explorer(
        &self,
        semantic_state_version: u64,
        workspace_root_uri: &Url,
    ) -> Option<ModelExplorerBundle> {
        let entry = self.entry.as_ref()?;
        if !self.is_valid(semantic_state_version, workspace_root_uri) {
            return None;
        }
        entry.model_explorer.clone()
    }

    pub fn ensure_snapshot(
        &mut self,
        graph: &SemanticGraph,
        parsed_documents: &[WorkspaceParsedDocument],
        library_urls: &[Url],
        workspace_root_uri: &Url,
        semantic_state_version: u64,
    ) -> Result<(), String> {
        let workspace_root_uri = normalize_uri(workspace_root_uri);
        if self.is_valid(semantic_state_version, &workspace_root_uri) {
            return Ok(());
        }

        let workspace_uris =
            sysml_model::workspace_uris_for_root(graph, library_urls, &workspace_root_uri);
        let viz_docs: Vec<WorkspaceParsedDocument> = workspace_uris
            .iter()
            .filter_map(|uri| {
                parsed_documents
                    .iter()
                    .find(|doc| normalize_uri(&doc.uri) == normalize_uri(uri))
                    .cloned()
            })
            .collect();
        let snapshot = build_view_catalog(
            graph,
            &viz_docs,
            library_urls,
            &workspace_root_uri,
            semantic_state_version,
        )?;
        self.entry = Some(ViewRenderCacheEntry {
            semantic_state_version,
            workspace_root_uri,
            snapshot,
            model_explorer: None,
            visualization_responses: HashMap::new(),
        });
        Ok(())
    }

    pub fn materialize_model_explorer(
        &mut self,
        graph: &SemanticGraph,
        parsed_documents: &[WorkspaceParsedDocument],
        library_urls: &[Url],
        workspace_root_uri: &Url,
        semantic_state_version: u64,
    ) -> Result<ModelExplorerBundle, String> {
        let workspace_root_uri = normalize_uri(workspace_root_uri);
        self.ensure_snapshot(
            graph,
            parsed_documents,
            library_urls,
            &workspace_root_uri,
            semantic_state_version,
        )?;
        let entry = self.entry.as_mut().expect("render cache initialized");
        if entry.model_explorer.is_none() {
            entry.model_explorer = Some(materialize_model_explorer_bundle(graph, &entry.snapshot));
        }
        Ok(entry
            .model_explorer
            .as_ref()
            .expect("model explorer")
            .clone())
    }

    pub fn build_visualization(
        &mut self,
        graph: &SemanticGraph,
        parsed_documents: &[WorkspaceParsedDocument],
        library_urls: &[Url],
        workspace_root_uri: &Url,
        semantic_state_version: u64,
        view: &str,
        selected_view: Option<&str>,
        build_start: Instant,
    ) -> Result<VisualizationBuildOutcome, String> {
        let workspace_root_uri = normalize_uri(workspace_root_uri);
        let cache_key = ViewCacheKey {
            view: view.to_string(),
            selected_view: selected_view.map(str::to_string),
        };

        if let Some(cached) =
            self.cached_response(semantic_state_version, &workspace_root_uri, &cache_key)
        {
            return Ok(VisualizationBuildOutcome {
                response: cached,
                meta: VisualizationBuildMeta {
                    cache_hit: true,
                    ..VisualizationBuildMeta::default()
                },
            });
        }

        self.ensure_snapshot(
            graph,
            parsed_documents,
            library_urls,
            &workspace_root_uri,
            semantic_state_version,
        )?;

        let snapshot = self
            .entry
            .as_ref()
            .expect("render cache initialized")
            .snapshot
            .clone();
        let workspace_uris = snapshot.workspace_uris.clone();
        let viz_docs: Vec<WorkspaceParsedDocument> = workspace_uris
            .iter()
            .filter_map(|uri| {
                parsed_documents
                    .iter()
                    .find(|doc| normalize_uri(&doc.uri) == normalize_uri(uri))
                    .cloned()
            })
            .collect();

        let cached_full_ibd = self
            .entry
            .as_ref()
            .and_then(|entry| entry.model_explorer.as_ref())
            .map(|bundle| &bundle.full_ibd);
        let (response, mut meta, _resolved_full_ibd) = render_view(
            graph,
            &viz_docs,
            &snapshot,
            view,
            selected_view,
            build_start,
            cached_full_ibd,
        )?;
        meta.cache_hit = false;

        if visualization_response_is_cacheable(&response) {
            self.entry
                .as_mut()
                .expect("render cache initialized")
                .visualization_responses
                .insert(cache_key, response.clone());
        }

        Ok(VisualizationBuildOutcome { response, meta })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use sysml_model::{FileSystemDocumentProvider, build_semantic_graph_with_provider};

    fn drone_workspace_inputs() -> (
        SemanticGraph,
        Vec<WorkspaceParsedDocument>,
        Vec<Url>,
        Url,
        u64,
    ) {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/drone");
        let workspace_root_uri =
            Url::from_directory_path(repo_root.canonicalize().unwrap()).unwrap();
        let provider =
            FileSystemDocumentProvider::new(repo_root.clone(), Some(repo_root), Vec::new());
        let (semantic_graph, parsed_docs) =
            build_semantic_graph_with_provider(&provider).expect("semantic graph");
        (
            semantic_graph,
            parsed_docs,
            Vec::new(),
            workspace_root_uri,
            1,
        )
    }

    #[test]
    fn warm_interconnection_visualization_hits_response_cache() {
        let (graph, docs, library_urls, root, version) = drone_workspace_inputs();
        let mut cache = ViewRenderCache::default();
        let cold = cache
            .build_visualization(
                &graph,
                &docs,
                &library_urls,
                &root,
                version,
                "interconnection-view",
                Some("connections"),
                Instant::now(),
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

        let warm = cache
            .build_visualization(
                &graph,
                &docs,
                &library_urls,
                &root,
                version,
                "interconnection-view",
                Some("connections"),
                Instant::now(),
            )
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
