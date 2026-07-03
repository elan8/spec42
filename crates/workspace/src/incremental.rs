//! Standalone incremental workspace engine (Tier 2 unified-incremental-engine, Phase 2).
//!
//! Wraps a [`SemanticGraph`] plus the documents currently indexed into it, and exposes a
//! full-load operation and a single-document incremental patch operation, both delegating to
//! `sysml_model`'s shared pipeline primitives (`build_and_link_graph_parallel`,
//! `patch_graph_for_document`) rather than re-implementing the build/link sequence — see
//! `docs/engineering/TIER2-UNIFIED-INCREMENTAL-ENGINE-DESIGN.md`.
//!
//! Not wired into [`crate::engine::Spec42Engine`]/`HostWorkspaceSnapshot` or `lsp_server`
//! yet. Phase 2 is deliberately standalone and equivalence-tested against both the full-load
//! primitive and the incremental-patch primitive before anything depends on it.

use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

use url::Url;

use crate::parse_cache;
use crate::semantic::{
    build_and_link_graph_parallel, patch_graph_for_document, SemanticGraph,
    WorkspaceParsedDocument,
};
use crate::SysmlDocument;

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

    /// The current semantic graph. `SemanticGraph` is `Arc`-backed, so this clone is cheap.
    pub fn graph(&self) -> SemanticGraph {
        self.graph.clone()
    }

    /// The parsed documents currently merged into the graph, in unspecified order.
    pub fn documents(&self) -> Vec<WorkspaceParsedDocument> {
        self.documents.values().cloned().collect()
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
    /// implemented (Tier 2 Phase 3b Step 5). Does not yet route through this engine's own
    /// parse cache: `build_and_link_graph_parallel` parses from raw document content
    /// internally, so a document already in the cache is re-parsed anyway on a full load.
    /// Reusing the cache here would need either a new `sysml_model` entry point that accepts
    /// already-parsed documents, or looping this engine's own `apply_document` per document
    /// (losing `build_and_link_graph_parallel`'s parallel merge/link) — left as an open
    /// question rather than picked here, since neither is needed for Phase 2's scope
    /// (standalone, equivalence-tested, unused by anything yet).
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SysmlDocumentSourceKind;

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

        assert_eq!(node_qualified_names(&engine.graph()), node_qualified_names(&expected_graph));
        assert_eq!(edge_triples(&engine.graph()), edge_triples(&expected_graph));
        assert_eq!(engine.document_count(), expected_parsed.len());
        assert_eq!(metrics.document_count, documents.len());
        assert!(metrics.node_count > 0);
        assert!(metrics.edge_count > 0);
        assert!(metrics.total_ms >= metrics.graph_update_ms);
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

        assert_eq!(node_qualified_names(&engine.graph()), node_qualified_names(&expected_graph));
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
        assert!(!node_qualified_names(&engine.graph())
            .iter()
            .any(|name| name.starts_with("AnalysisCases")));
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
        assert!(!engine
            .document(&document.uri)
            .expect("document indexed")
            .parse_cached, "first apply_document should be a cache miss");

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
}
