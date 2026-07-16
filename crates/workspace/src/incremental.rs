//! Shared incremental workspace engine (Tier 2 unified-incremental-engine).
//!
//! Wraps a [`SemanticGraph`] plus the documents currently indexed into it, and exposes a
//! full-load operation and a single-document incremental patch operation, both delegating to
//! `sysml_model`'s shared pipeline primitives (`build_and_link_graph_parallel`,
//! `patch_graph_for_document`) rather than re-implementing the build/link sequence — see
//! `docs/engineering/TIER2-UNIFIED-INCREMENTAL-ENGINE-DESIGN.md`.
//!
//! This is the one engine every live consumer uses directly: `lsp_server`'s `ServerState`
//! (`rebuild_all_document_links` / `rebuild_semantic_graph_staged`) and Babel42's
//! `EditorSession` both hold an `IncrementalWorkspace` and call `apply_document` on every
//! edit. `crate::engine::Spec42Engine`'s `update_snapshot` path also builds on it internally
//! (`snapshot::update::try_incremental_update`), behind the `experimental_incremental_updates`
//! flag, for stateless CLI/MCP/HTTP callers that want a frozen `HostWorkspaceSnapshot` rather
//! than an owned, long-lived session.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;
use url::Url;

use sysml_model::{
    IbdDataDto, SysmlVisualizationResultDto, VisualizationBuildMeta, WorkspaceRenderSnapshot,
};

use crate::error::WorkspaceResult;
use crate::parse_cache;
use crate::semantic::{
    SemanticGraph, WorkspaceParsedDocument, build_and_link_graph_parallel,
    link_parsed_documents_parallel_from, patch_graph_for_document,
};
use crate::snapshot::{HostSemanticProjection, HostValidationReport};
use crate::{SysmlDocument, SysmlDocumentSourceKind};

/// Timing and size metrics for one [`IncrementalWorkspace`] operation.
///
/// Coarser than `lsp_server`'s current `RebuildAllDocumentLinksMetrics` (which splits the
/// graph-update step into 7 sub-phases: remove-nodes, rebuild-graphs, cross-edge-resolution,
/// workspace-relationship-linking, pending-relationship-resolution, expression-evaluation,
/// refresh-symbols). Those finer phases live *inside* `sysml_model`'s pipeline functions
/// (`build_and_link_graph_parallel`, `patch_graph_for_document`), which this engine calls as
/// a single unit rather than re-implementing their internal sequencing a second time — doing
/// otherwise would recreate the exact "two places implement the same sequence, and they
/// drift" bug shape Tier 2 Phase 3b/Step 5 already found and fixed three times. Getting
/// finer-grained timing than `parse_ms`/`graph_update_ms` would require `sysml_model`'s
/// pipeline functions to return their own phase breakdown — not designed here; see the open
/// questions in the Phase 2 write-up.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WorkspaceUpdateMetrics {
    /// Documents involved in this operation (all of them for `load`, always 1 for
    /// `apply_document`/`remove_document`).
    pub document_count: usize,
    /// Time spent parsing (and, for `apply_document`, checking/populating the parse cache).
    /// Always `0` for `load`, which parses internally inside `build_and_link_graph_parallel`
    /// and does not yet expose that sub-timing separately — folded into `graph_update_ms`.
    pub parse_ms: u32,
    /// Time spent building/patching and re-linking the graph (parse-inclusive for `load`).
    pub graph_update_ms: u32,
    /// Total wall time for the whole operation.
    pub total_ms: u32,
    /// Node count in the graph after this operation.
    pub node_count: usize,
    /// Edge count in the graph after this operation.
    pub edge_count: usize,
}

fn elapsed_ms(start: Instant) -> u32 {
    start.elapsed().as_millis().max(1) as u32
}

/// A semantic graph plus the documents currently merged into it, updatable either by a full
/// reload or by patching one document at a time.
#[derive(Debug, Clone, Default)]
pub struct IncrementalWorkspace {
    graph: SemanticGraph,
    documents: HashMap<Url, WorkspaceParsedDocument>,
}

impl IncrementalWorkspace {
    pub fn new() -> Self {
        Self::default()
    }

    /// Reconstruct an engine from a previously computed graph and document set — e.g. to
    /// resume patching a `HostWorkspaceSnapshot` that was assembled elsewhere. `graph`'s
    /// `Arc` backing means this doesn't deep-copy the graph itself.
    pub fn from_parts(graph: SemanticGraph, documents: Vec<WorkspaceParsedDocument>) -> Self {
        Self {
            graph,
            documents: documents
                .into_iter()
                .map(|doc| (doc.uri.clone(), doc))
                .collect(),
        }
    }

    /// The current semantic graph. `SemanticGraph` is `Arc`-backed, so this clone is cheap.
    pub fn graph(&self) -> SemanticGraph {
        self.graph.clone()
    }

    /// The parsed documents currently merged into the graph, sorted by URI for a
    /// deterministic order (internal storage is a `HashMap`, whose iteration order isn't).
    pub fn documents(&self) -> Vec<WorkspaceParsedDocument> {
        let mut docs: Vec<WorkspaceParsedDocument> = self.documents.values().cloned().collect();
        docs.sort_by(|a, b| a.uri.as_str().cmp(b.uri.as_str()));
        docs
    }

    pub fn document_count(&self) -> usize {
        self.documents.len()
    }

    pub fn document(&self, uri: &Url) -> Option<&WorkspaceParsedDocument> {
        self.documents.get(uri)
    }

    /// Full load: discard current state and build fresh from `documents`.
    ///
    /// Delegates to [`build_and_link_graph_parallel`] — the single place this sequence is
    /// implemented (Tier 2 Phase 3b Step 5). Always re-parses every document from raw
    /// content, even if it's already in this engine's own parse cache. Callers that already
    /// hold parsed documents (or want to use the cache on a full load) should use
    /// [`Self::load_parsed`] or [`Self::load_with_cache`] instead.
    pub fn load(&mut self, documents: &[SysmlDocument]) -> WorkspaceUpdateMetrics {
        let total_start = Instant::now();
        let build_start = Instant::now();
        let (graph, parsed_docs) = build_and_link_graph_parallel(documents);
        let graph_update_ms = elapsed_ms(build_start);

        self.graph = graph;
        self.documents = parsed_docs
            .into_iter()
            .map(|doc| (doc.uri.clone(), doc))
            .collect();

        WorkspaceUpdateMetrics {
            document_count: documents.len(),
            parse_ms: 0,
            graph_update_ms,
            total_ms: elapsed_ms(total_start),
            node_count: self.graph.graph.node_count(),
            edge_count: self.graph.graph.edge_count(),
        }
    }

    /// Full load from documents this engine has already parsed — e.g. a caller with its own
    /// pre-parsed document index (`lsp_server`'s `IndexEntry.parsed`) that would otherwise
    /// have to throw that work away to call [`Self::load`]. Delegates to
    /// [`link_parsed_documents_parallel`] — the merge/link half of
    /// [`build_and_link_graph_parallel`] with the parse step already done by the caller.
    pub fn load_parsed(
        &mut self,
        documents: Vec<(SysmlDocumentSourceKind, WorkspaceParsedDocument)>,
    ) -> WorkspaceUpdateMetrics {
        self.load_parsed_from(SemanticGraph::new(), documents, true)
    }

    /// [`Self::load_parsed`], but merging `documents` onto `base_graph` instead of starting
    /// from an empty graph — for a caller that already has part of the graph built (e.g. a
    /// cached library subgraph) and only wants to merge/link the rest. Delegates to
    /// [`link_parsed_documents_parallel_from`], which resolves cross-document edges against
    /// `base_graph`'s existing URIs too, not just `documents`'.
    ///
    /// `evaluate: false` skips expression evaluation (structural relink only) — the caller is
    /// then responsible for running [`sysml_model::evaluate_workspace_graph`] itself as a
    /// separate, later step (e.g. `lsp_server`'s live-edit relink publishes structural
    /// diagnostics from the `evaluate: false` result immediately, then evaluates in a debounced
    /// background task afterward — see `docs/engineering` Track C).
    pub fn load_parsed_from(
        &mut self,
        base_graph: SemanticGraph,
        documents: Vec<(SysmlDocumentSourceKind, WorkspaceParsedDocument)>,
        evaluate: bool,
    ) -> WorkspaceUpdateMetrics {
        let total_start = Instant::now();
        let document_count = documents.len();

        let build_start = Instant::now();
        let (graph, parsed_docs) =
            link_parsed_documents_parallel_from(base_graph, documents, evaluate);
        let graph_update_ms = elapsed_ms(build_start);

        self.graph = graph;
        self.documents = parsed_docs
            .into_iter()
            .map(|doc| (doc.uri.clone(), doc))
            .collect();

        WorkspaceUpdateMetrics {
            document_count,
            parse_ms: 0,
            graph_update_ms,
            total_ms: elapsed_ms(total_start),
            node_count: self.graph.graph.node_count(),
            edge_count: self.graph.graph.edge_count(),
        }
    }

    /// Runs expression evaluation on the current graph in place — the deferred "Wave 2" step
    /// for a graph built with `evaluate: false` (see [`Self::load_parsed_from`]).
    pub fn evaluate(&mut self) {
        sysml_model::evaluate_workspace_graph(&mut self.graph);
    }

    /// Full load that parses `documents` in parallel through this engine's own
    /// [`crate::parse_cache`] (when `cache_dir` is `Some`) before merging/linking — the
    /// combination [`Self::load`] can't give a caller that wants both a full rebuild and
    /// cache reuse. Falls back to a fresh parse per document on a cache miss or when
    /// `cache_dir` is `None`, same as [`Self::apply_document`].
    pub fn load_with_cache(
        &mut self,
        documents: &[SysmlDocument],
        cache_dir: Option<&Path>,
    ) -> WorkspaceUpdateMetrics {
        let total_start = Instant::now();

        let parse_start = Instant::now();
        let entries: Vec<(SysmlDocumentSourceKind, WorkspaceParsedDocument)> = documents
            .par_iter()
            .filter_map(|document| {
                let (parsed, parse_cached) = self.parse_with_cache(document, cache_dir);
                parsed.map(|root| {
                    (
                        document.source_kind,
                        WorkspaceParsedDocument {
                            uri: document.uri.clone(),
                            content: document.content.clone(),
                            parsed: root,
                            parse_time_ms: 1,
                            parse_cached,
                        },
                    )
                })
            })
            .collect();
        let parse_ms = elapsed_ms(parse_start);

        let mut metrics = self.load_parsed(entries);
        metrics.parse_ms = parse_ms;
        metrics.total_ms = elapsed_ms(total_start);
        metrics
    }

    /// Incremental patch: re-parse and re-link a single document, leaving every other
    /// document's nodes untouched.
    ///
    /// When `cache_dir` is `Some`, the parse is served from (and, on a miss, written back
    /// to) this engine's relocated [`crate::parse_cache`] before falling back to a fresh
    /// parse — the reuse `load` does not get (see its doc comment).
    pub fn apply_document(
        &mut self,
        document: &SysmlDocument,
        cache_dir: Option<&Path>,
    ) -> WorkspaceUpdateMetrics {
        let total_start = Instant::now();

        let parse_start = Instant::now();
        let (parsed, parse_cached) = self.parse_with_cache(document, cache_dir);
        let parse_ms = elapsed_ms(parse_start);

        let build_start = Instant::now();
        patch_graph_for_document(&mut self.graph, &document.uri, parsed.as_ref(), true);
        let graph_update_ms = elapsed_ms(build_start);

        match parsed {
            Some(root) => {
                self.documents.insert(
                    document.uri.clone(),
                    WorkspaceParsedDocument {
                        uri: document.uri.clone(),
                        content: document.content.clone(),
                        parsed: root,
                        parse_time_ms: parse_ms.max(1),
                        parse_cached,
                    },
                );
            }
            None => {
                self.documents.remove(&document.uri);
            }
        }

        WorkspaceUpdateMetrics {
            document_count: self.documents.len(),
            parse_ms,
            graph_update_ms,
            total_ms: elapsed_ms(total_start),
            node_count: self.graph.graph.node_count(),
            edge_count: self.graph.graph.edge_count(),
        }
    }

    /// Remove a document's nodes from the graph and re-link (e.g. the file was deleted).
    pub fn remove_document(&mut self, uri: &Url) -> WorkspaceUpdateMetrics {
        let total_start = Instant::now();
        patch_graph_for_document(&mut self.graph, uri, None, true);
        self.documents.remove(uri);
        let graph_update_ms = elapsed_ms(total_start);

        WorkspaceUpdateMetrics {
            document_count: self.documents.len(),
            parse_ms: 0,
            graph_update_ms,
            total_ms: graph_update_ms,
            node_count: self.graph.graph.node_count(),
            edge_count: self.graph.graph.edge_count(),
        }
    }

    fn parse_with_cache(
        &self,
        document: &SysmlDocument,
        cache_dir: Option<&Path>,
    ) -> (Option<sysml_v2_parser::RootNamespace>, bool) {
        if let Some(dir) = cache_dir {
            let hash = parse_cache::content_hash(document.content.as_bytes());
            if let Some(root) = parse_cache::load(dir, &hash) {
                return (Some(root), true);
            }
            let parsed = sysml_v2_parser::parse(&document.content).ok();
            if let Some(root) = &parsed {
                parse_cache::store(dir, &hash, root);
            }
            return (parsed, false);
        }
        (sysml_v2_parser::parse(&document.content).ok(), false)
    }
}

/// Compute a [`HostValidationReport`] directly from an [`IncrementalWorkspace`]'s current
/// graph/documents, without building a [`crate::snapshot::HostWorkspaceSnapshot`].
///
/// A thin, same-crate call into `snapshot::facts::collect_host_validation_report` — the exact
/// function [`crate::snapshot::build::build_workspace_snapshot`] already uses internally.
/// Exists so embedders that hold an [`IncrementalWorkspace`] directly (rather than going
/// through the `snapshot` pipeline) can still get diagnostics, without paying for the rest of
/// a snapshot's eager derived fields (`language_workspace`/`render_snapshot`/
/// `semantic_projection`) they don't need. See
/// `docs/engineering/TIER2-UNIFIED-INCREMENTAL-ENGINE-DESIGN.md`'s "Phase 5" write-up.
///
/// # Errors
///
/// Returns an error when target file URLs cannot be resolved.
pub fn validate_workspace(
    graph: &SemanticGraph,
    documents: &[SysmlDocument],
    library_urls: &[Url],
    target_files: &[PathBuf],
    workspace_root: Option<&Path>,
    library_paths_display: &[PathBuf],
    strict_diagnostics: bool,
) -> WorkspaceResult<HostValidationReport> {
    crate::snapshot::facts::collect_host_validation_report(
        graph,
        documents,
        library_urls,
        target_files,
        workspace_root,
        library_paths_display,
        strict_diagnostics,
    )
}

/// Compute the [`HostSemanticProjection`] for a graph's current state, without building a
/// [`crate::snapshot::HostWorkspaceSnapshot`].
///
/// A thin, same-crate call into `snapshot::facts::project_host_semantic_model` — the same
/// pattern as [`validate_workspace`]/`collect_host_validation_report`.
///
/// # Errors
///
/// Returns an error when target file URLs cannot be resolved.
pub fn project_semantic_model(
    graph: &SemanticGraph,
    target_files: &[PathBuf],
) -> WorkspaceResult<HostSemanticProjection> {
    crate::snapshot::facts::project_host_semantic_model(graph, target_files, &[])
}

/// Build the view catalog for a graph's current state — the render-snapshot half of what
/// [`crate::snapshot::HostWorkspaceSnapshot::view_catalog`] builds, without requiring a
/// snapshot.
///
/// # Errors
///
/// Returns an error when the render snapshot cannot be built.
pub fn build_view_catalog(
    graph: &SemanticGraph,
    documents: &[WorkspaceParsedDocument],
    library_urls: &[Url],
    workspace_root_uri: &Url,
    schema_version: u64,
) -> Result<WorkspaceRenderSnapshot, String> {
    sysml_model::build_render_snapshot(
        graph,
        documents,
        library_urls,
        workspace_root_uri,
        schema_version,
    )
}

/// Render one view against a graph's current state and its [`WorkspaceRenderSnapshot`] view
/// catalog (from [`build_view_catalog`]).
///
/// `cached_full_ibd` lets callers reuse a previously computed merged IBD (see
/// [`sysml_model::full_ibd_for_render_snapshot`]); pass `None` to always recompute. The third
/// element of the returned tuple is `Some(ibd)` exactly when a real merged IBD was computed or
/// reused (i.e. the `general-view`/`interconnection-view` empty-shortcut below was *not* taken)
/// — callers that memoize the merged IBD across calls (`HostWorkspaceSnapshot::prepare_view`'s
/// `full_ibd_cache`) should only store this when it is `Some`, never the empty placeholder,
/// or a later view needing the real merged IBD would incorrectly reuse an empty one.
///
/// This is the single place the `general-view`/`interconnection-view` IBD-scope decision
/// lives — `HostWorkspaceSnapshot::prepare_view` and `lsp_server`'s
/// `build_visualization_with_cache` each independently re-derived this condition before; both
/// now call this instead.
///
/// # Errors
///
/// Returns an error when the visualization cannot be built for the requested view.
pub fn render_view(
    graph: &SemanticGraph,
    documents: &[WorkspaceParsedDocument],
    view_catalog: &WorkspaceRenderSnapshot,
    view: &str,
    selected_view: Option<&str>,
    build_start: Instant,
    cached_full_ibd: Option<&IbdDataDto>,
) -> Result<
    (
        SysmlVisualizationResultDto,
        VisualizationBuildMeta,
        Option<IbdDataDto>,
    ),
    String,
> {
    let options = sysml_model::visualization_build_options(view);
    let uses_empty_shortcut = options.ibd_build_scope
        == sysml_model::IbdBuildScope::ViewExposedPackages
        && (view == "general-view"
            || (view == "interconnection-view" && options.slim_interconnection_payload));
    let (full_ibd, resolved_full_ibd) = if uses_empty_shortcut {
        (sysml_model::empty_merged_ibd(), None)
    } else {
        let resolved =
            sysml_model::full_ibd_for_render_snapshot(graph, view_catalog, cached_full_ibd);
        (resolved.clone(), Some(resolved))
    };
    let (response, meta) = sysml_model::build_sysml_visualization_from_render_snapshot_with_meta(
        graph,
        documents,
        view_catalog,
        view,
        selected_view,
        build_start,
        full_ibd,
        options,
    )?;
    Ok((response, meta, resolved_full_ibd))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc(scope: &str, path: &str, content: &str) -> SysmlDocument {
        SysmlDocument::from_memory_path(
            scope,
            path,
            content.to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("document uri")
    }

    fn node_qualified_names(graph: &SemanticGraph) -> std::collections::BTreeSet<String> {
        graph
            .graph
            .node_weights()
            .map(|node| node.id.qualified_name.clone())
            .collect()
    }

    fn edge_triples(graph: &SemanticGraph) -> std::collections::BTreeSet<(String, String, String)> {
        graph
            .graph
            .edge_indices()
            .map(|edge_idx| {
                let (src, tgt) = graph
                    .graph
                    .edge_endpoints(edge_idx)
                    .expect("edge has endpoints");
                (
                    graph.graph[src].id.qualified_name.clone(),
                    graph.graph[tgt].id.qualified_name.clone(),
                    graph.graph[edge_idx].kind.as_str().to_string(),
                )
            })
            .collect()
    }

    fn fixture_documents() -> Vec<SysmlDocument> {
        vec![
            doc(
                "incremental",
                "Architecture.sysml",
                r#"
package Architecture {
  part def MobilitySubsystem {
    attribute drivePowerW : Real = 28;
  }
  part def Robot {
    part mobility : MobilitySubsystem;
  }
}
"#,
            ),
            doc(
                "incremental",
                "AnalysisCases.sysml",
                r#"
package AnalysisCases {
  private import Architecture::*;

  analysis def PowerAnalysis {
    attribute powerBudgetW : Real = 55;
    subject robot : Robot;
    return ref withinBudget {
      return robot.mobility.drivePowerW <= powerBudgetW;
    }
  }
}
"#,
            ),
        ]
    }

    /// Equivalence: `IncrementalWorkspace::load` should produce exactly the nodes/edges/
    /// documents `build_and_link_graph_parallel` produces directly — `load` is a thin
    /// wrapper, not a reimplementation.
    #[test]
    fn load_matches_build_and_link_graph_parallel_directly() {
        let documents = fixture_documents();
        let (expected_graph, expected_parsed) = build_and_link_graph_parallel(&documents);

        let mut engine = IncrementalWorkspace::new();
        let metrics = engine.load(&documents);

        assert_eq!(
            node_qualified_names(&engine.graph()),
            node_qualified_names(&expected_graph)
        );
        assert_eq!(edge_triples(&engine.graph()), edge_triples(&expected_graph));
        assert_eq!(engine.document_count(), expected_parsed.len());
        assert_eq!(metrics.document_count, documents.len());
        assert!(metrics.node_count > 0);
        assert!(metrics.edge_count > 0);
        assert!(metrics.total_ms >= metrics.graph_update_ms);
    }

    /// Equivalence for `load_parsed` (Phase 4): pre-parsing the fixture and feeding it
    /// through `load_parsed` should produce the same graph as `load`, which parses
    /// internally — this is what lets a caller with its own pre-parsed document index (e.g.
    /// `lsp_server`'s `IndexEntry`) skip re-parsing on a full rebuild.
    #[test]
    fn load_parsed_matches_load() {
        let documents = fixture_documents();

        let mut loaded = IncrementalWorkspace::new();
        loaded.load(&documents);

        let entries: Vec<(SysmlDocumentSourceKind, WorkspaceParsedDocument)> = documents
            .iter()
            .map(|document| {
                let parsed = sysml_v2_parser::parse(&document.content).expect("parse");
                (
                    document.source_kind,
                    WorkspaceParsedDocument {
                        uri: document.uri.clone(),
                        content: document.content.clone(),
                        parsed,
                        parse_time_ms: 1,
                        parse_cached: true,
                    },
                )
            })
            .collect();
        let mut from_parsed = IncrementalWorkspace::new();
        let metrics = from_parsed.load_parsed(entries);

        assert_eq!(
            node_qualified_names(&from_parsed.graph()),
            node_qualified_names(&loaded.graph())
        );
        assert_eq!(
            edge_triples(&from_parsed.graph()),
            edge_triples(&loaded.graph())
        );
        assert_eq!(metrics.document_count, documents.len());
        assert_eq!(
            metrics.parse_ms, 0,
            "load_parsed does no parsing of its own"
        );
    }

    /// Equivalence for `load_parsed_from` (Phase 4): merging the remaining documents onto a
    /// base graph that already contains one of them (mirroring `lsp_server`'s
    /// `rebuild_semantic_graph_staged` reusing a cached library subgraph) should produce the
    /// same graph as loading everything together.
    #[test]
    fn load_parsed_from_matches_load_when_base_graph_holds_one_document() {
        let documents = fixture_documents();

        let mut loaded = IncrementalWorkspace::new();
        loaded.load(&documents);

        fn parsed_entry(
            document: &SysmlDocument,
        ) -> (SysmlDocumentSourceKind, WorkspaceParsedDocument) {
            let parsed = sysml_v2_parser::parse(&document.content).expect("parse");
            (
                document.source_kind,
                WorkspaceParsedDocument {
                    uri: document.uri.clone(),
                    content: document.content.clone(),
                    parsed,
                    parse_time_ms: 1,
                    parse_cached: true,
                },
            )
        }

        let mut base = IncrementalWorkspace::new();
        base.load_parsed(vec![parsed_entry(&documents[0])]);

        let remaining: Vec<(SysmlDocumentSourceKind, WorkspaceParsedDocument)> =
            documents[1..].iter().map(parsed_entry).collect();
        let mut merged = IncrementalWorkspace::new();
        merged.load_parsed_from(base.graph(), remaining, true);

        assert_eq!(
            node_qualified_names(&merged.graph()),
            node_qualified_names(&loaded.graph())
        );
        assert_eq!(edge_triples(&merged.graph()), edge_triples(&loaded.graph()));
    }

    /// Equivalence + cache-reuse check for `load_with_cache` (Phase 4, closing the Phase 2
    /// open question): a second `load_with_cache` call against the same cache directory
    /// should produce the same graph as the first, and every document should come back
    /// marked as a cache hit.
    #[test]
    fn load_with_cache_matches_load_and_reuses_cache_on_second_call() {
        let documents = fixture_documents();
        let cache_dir = tempfile::tempdir().expect("tempdir");

        let mut first = IncrementalWorkspace::new();
        let first_metrics = first.load_with_cache(&documents, Some(cache_dir.path()));
        assert!(
            first.documents().iter().all(|doc| !doc.parse_cached),
            "first load_with_cache call should be all cache misses"
        );

        let mut second = IncrementalWorkspace::new();
        let second_metrics = second.load_with_cache(&documents, Some(cache_dir.path()));
        assert!(
            second.documents().iter().all(|doc| doc.parse_cached),
            "second load_with_cache call should be all cache hits"
        );

        assert_eq!(
            node_qualified_names(&first.graph()),
            node_qualified_names(&second.graph())
        );
        assert_eq!(edge_triples(&first.graph()), edge_triples(&second.graph()));
        assert_eq!(first_metrics.document_count, second_metrics.document_count);

        let mut loaded = IncrementalWorkspace::new();
        loaded.load(&documents);
        assert_eq!(
            node_qualified_names(&loaded.graph()),
            node_qualified_names(&second.graph())
        );
    }

    /// Equivalence: incrementally patching one document via `apply_document` should produce
    /// the same graph as a fresh `load` of the post-edit document set — the same "incremental
    /// equals full rebuild after the edit" parity `workspace/tests/incremental_parity.rs`
    /// already checks at the `HostWorkspaceSnapshot` layer, checked here at the engine layer.
    #[test]
    fn apply_document_matches_full_reload_after_edit() {
        let mut documents = fixture_documents();
        let mut engine = IncrementalWorkspace::new();
        engine.load(&documents);

        let updated_architecture = doc(
            "incremental",
            "Architecture.sysml",
            r#"
package Architecture {
  part def MobilitySubsystem {
    attribute drivePowerW : Real = 40;
  }
  part def Robot {
    part mobility : MobilitySubsystem;
  }
}
"#,
        );
        documents[0] = updated_architecture.clone();

        let metrics = engine.apply_document(&updated_architecture, None);

        let (expected_graph, _) = build_and_link_graph_parallel(&documents);

        assert_eq!(
            node_qualified_names(&engine.graph()),
            node_qualified_names(&expected_graph)
        );
        assert_eq!(edge_triples(&engine.graph()), edge_triples(&expected_graph));
        assert_eq!(metrics.document_count, 2);
        assert!(metrics.parse_ms >= 1);
    }

    /// Same parity check, but for the evaluated-attribute side effect specifically — the
    /// exact class of bug (`evaluate_expressions` silently skipped) Tier 2 Phase 3b Steps
    /// 1-4 found in the two hand-written pipelines this engine is designed to replace.
    #[test]
    fn apply_document_evaluates_expressions() {
        let mut engine = IncrementalWorkspace::new();
        let initial = doc(
            "incremental",
            "Demo.sysml",
            "package Demo { part def Rocket { attribute mass = 1; } }",
        );
        engine.load(&[initial]);

        let updated = doc(
            "incremental",
            "Demo.sysml",
            "package Demo { part def Rocket { attribute mass = 1 + 2; } }",
        );
        engine.apply_document(&updated, None);

        let graph = engine.graph();
        let mass = graph
            .graph
            .node_weights()
            .find(|node| node.name == "mass")
            .expect("mass attribute node");
        assert_eq!(
            mass.attributes.get("evaluatedValue"),
            Some(&serde_json::json!(3))
        );
    }

    /// `from_parts` plus `apply_document` should be indistinguishable from building the
    /// whole thing via `load` in one call — the shape `snapshot::update`'s incremental path
    /// (Phase 3) relies on: reconstruct engine state from a previous snapshot's graph and
    /// document set, then patch.
    #[test]
    fn from_parts_then_apply_document_matches_load() {
        let documents = fixture_documents();
        let mut loaded = IncrementalWorkspace::new();
        loaded.load(&documents);

        let mut reconstructed =
            IncrementalWorkspace::from_parts(loaded.graph(), loaded.documents());
        assert_eq!(
            node_qualified_names(&reconstructed.graph()),
            node_qualified_names(&loaded.graph())
        );
        assert_eq!(reconstructed.document_count(), loaded.document_count());

        let updated_analysis = doc(
            "incremental",
            "AnalysisCases.sysml",
            r#"
package AnalysisCases {
  private import Architecture::*;

  analysis def PowerAnalysis {
    attribute powerBudgetW : Real = 100;
    subject robot : Robot;
    return ref withinBudget {
      return robot.mobility.drivePowerW <= powerBudgetW;
    }
  }
}
"#,
        );
        reconstructed.apply_document(&updated_analysis, None);

        let mut expected_documents = documents;
        expected_documents[1] = updated_analysis;
        let (expected_graph, _) = build_and_link_graph_parallel(&expected_documents);

        assert_eq!(
            node_qualified_names(&reconstructed.graph()),
            node_qualified_names(&expected_graph)
        );
        assert_eq!(
            edge_triples(&reconstructed.graph()),
            edge_triples(&expected_graph)
        );
    }

    #[test]
    fn remove_document_clears_its_nodes() {
        let documents = fixture_documents();
        let mut engine = IncrementalWorkspace::new();
        engine.load(&documents);
        let uri = documents[1].uri.clone();
        assert!(engine.document(&uri).is_some());

        let metrics = engine.remove_document(&uri);

        assert!(engine.document(&uri).is_none());
        assert_eq!(metrics.document_count, 1);
        assert!(
            !node_qualified_names(&engine.graph())
                .iter()
                .any(|name| name.starts_with("AnalysisCases"))
        );
    }

    #[test]
    fn apply_document_uses_parse_cache_when_provided() {
        let cache_dir = tempfile::tempdir().expect("tempdir");
        let mut engine = IncrementalWorkspace::new();
        let document = doc(
            "incremental",
            "Cached.sysml",
            "package Cached { part def Engine; }",
        );
        engine.load(std::slice::from_ref(&document));

        let first = engine.apply_document(&document, Some(cache_dir.path()));
        assert!(
            !engine
                .document(&document.uri)
                .expect("document indexed")
                .parse_cached,
            "first apply_document should be a cache miss"
        );

        let second = engine.apply_document(&document, Some(cache_dir.path()));
        assert_eq!(
            second.document_count, first.document_count,
            "re-applying the identical document should not change the document count"
        );
        assert!(
            engine
                .document(&document.uri)
                .expect("document indexed")
                .parse_cached,
            "second apply_document with the same content should hit the parse cache"
        );
    }

    #[test]
    fn validate_workspace_matches_collect_host_validation_report_directly() {
        let mut engine = IncrementalWorkspace::new();
        let documents = fixture_documents();
        engine.load(&documents);

        let library_urls: Vec<Url> = Vec::new();
        let target_files: Vec<PathBuf> = Vec::new();
        let library_paths_display: Vec<PathBuf> = Vec::new();

        let via_wrapper = validate_workspace(
            &engine.graph(),
            &documents,
            &library_urls,
            &target_files,
            None,
            &library_paths_display,
            false,
        )
        .expect("validate_workspace succeeds");

        let via_direct_call = crate::snapshot::facts::collect_host_validation_report(
            &engine.graph(),
            &documents,
            &library_urls,
            &target_files,
            None,
            &library_paths_display,
            false,
        )
        .expect("collect_host_validation_report succeeds");

        assert_eq!(
            via_wrapper.summary.document_count,
            via_direct_call.summary.document_count
        );
        assert_eq!(
            via_wrapper.summary.error_count,
            via_direct_call.summary.error_count
        );
        assert_eq!(via_wrapper.workspace_root, via_direct_call.workspace_root);
        assert_eq!(
            via_wrapper.resolved_library_paths,
            via_direct_call.resolved_library_paths
        );
    }

    #[test]
    fn build_view_catalog_matches_build_render_snapshot_directly() {
        let mut engine = IncrementalWorkspace::new();
        let documents = fixture_documents();
        engine.load(&documents);

        let library_urls: Vec<Url> = Vec::new();
        let workspace_root_uri = Url::parse("file:///workspace/").expect("valid url");

        let via_wrapper = build_view_catalog(
            &engine.graph(),
            &engine.documents(),
            &library_urls,
            &workspace_root_uri,
            1,
        )
        .expect("build_view_catalog succeeds");

        let via_direct_call = sysml_model::build_render_snapshot(
            &engine.graph(),
            &engine.documents(),
            &library_urls,
            &workspace_root_uri,
            1,
        )
        .expect("build_render_snapshot succeeds");

        assert_eq!(via_wrapper.version, via_direct_call.version);
        assert_eq!(
            via_wrapper.workspace_uris.len(),
            via_direct_call.workspace_uris.len()
        );
        assert_eq!(
            via_wrapper.view_index.view_candidates.len(),
            via_direct_call.view_index.view_candidates.len()
        );
    }

    #[test]
    fn render_view_matches_build_sysml_visualization_from_render_snapshot_with_meta_directly() {
        let mut engine = IncrementalWorkspace::new();
        let documents = fixture_documents();
        engine.load(&documents);

        let library_urls: Vec<Url> = Vec::new();
        let workspace_root_uri = Url::parse("file:///workspace/").expect("valid url");
        let view_catalog = build_view_catalog(
            &engine.graph(),
            &engine.documents(),
            &library_urls,
            &workspace_root_uri,
            1,
        )
        .expect("build_view_catalog succeeds");

        let (via_wrapper, _meta, resolved_full_ibd) = render_view(
            &engine.graph(),
            &engine.documents(),
            &view_catalog,
            "general-view",
            None,
            Instant::now(),
            None,
        )
        .expect("render_view succeeds");

        // "general-view" takes the empty-IBD shortcut, so nothing should be cached.
        assert!(resolved_full_ibd.is_none());

        let options = sysml_model::visualization_build_options("general-view");
        let (via_direct_call, _meta) =
            sysml_model::build_sysml_visualization_from_render_snapshot_with_meta(
                &engine.graph(),
                &engine.documents(),
                &view_catalog,
                "general-view",
                None,
                Instant::now(),
                sysml_model::empty_merged_ibd(),
                options,
            )
            .expect("build_sysml_visualization_from_render_snapshot_with_meta succeeds");

        assert_eq!(via_wrapper.view, via_direct_call.view);
        assert_eq!(via_wrapper.model_ready, via_direct_call.model_ready);
        assert_eq!(
            via_wrapper.view_candidates.len(),
            via_direct_call.view_candidates.len()
        );
    }

    #[test]
    fn project_semantic_model_matches_project_host_semantic_model_directly() {
        let mut engine = IncrementalWorkspace::new();
        let documents = fixture_documents();
        engine.load(&documents);
        let target_files: Vec<PathBuf> = Vec::new();

        let via_wrapper = project_semantic_model(&engine.graph(), &target_files)
            .expect("project_semantic_model succeeds");
        let via_direct_call = crate::snapshot::facts::project_host_semantic_model(
            &engine.graph(),
            &target_files,
            &[],
        )
        .expect("project_host_semantic_model succeeds");

        assert_eq!(via_wrapper.nodes.len(), via_direct_call.nodes.len());
        assert_eq!(
            via_wrapper.relationships.len(),
            via_direct_call.relationships.len()
        );
    }
}
