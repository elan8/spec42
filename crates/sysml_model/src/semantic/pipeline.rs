//! Orchestrates semantic graph materialize → link → pending resolve.

use std::collections::HashSet;
use std::time::Instant;

use rayon::prelude::*;
use url::Url;

use crate::semantic::analysis_typing::prepare_analysis_evaluation_context;
use crate::semantic::evaluation::evaluate_expressions;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::graph_builder::build_graph_from_doc;
use crate::semantic::library_loader::declared_packages_from_parsed;
use crate::semantic::model::SemanticEdge;
use crate::semantic::relationships::{
    add_cross_document_edges_for_uri, link_workspace_derivations, link_workspace_relationships,
    resolve_cross_document_edges_for_uri, resolve_workspace_pending_relationships,
};
use crate::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};
use crate::semantic::workspace_graph::WorkspaceParsedDocument;

/// Build, merge, link, and resolve pending relationships for pre-loaded documents.
pub fn build_and_link_graph(
    documents: &[SysmlDocument],
) -> Result<(SemanticGraph, Vec<WorkspaceParsedDocument>), String> {
    let mut graph = SemanticGraph::new();
    let mut parsed_docs = Vec::new();

    let mut workspace_docs = Vec::new();
    let mut library_docs = Vec::new();
    for document in documents {
        match document.source_kind {
            SysmlDocumentSourceKind::Library => library_docs.push(document),
            SysmlDocumentSourceKind::Workspace | SysmlDocumentSourceKind::External => {
                workspace_docs.push(document)
            }
        }
    }

    let mut workspace_packages = HashSet::new();

    for document in workspace_docs {
        let parse_start = Instant::now();
        let Ok(parsed) = sysml_v2_parser::parse(&document.content) else {
            continue;
        };
        workspace_packages.extend(declared_packages_from_parsed(&parsed));
        let parse_time_ms = parse_start.elapsed().as_millis().max(1) as u32;
        let doc_graph = build_graph_from_doc(&parsed, &document.uri);
        graph.merge(doc_graph);
        parsed_docs.push(WorkspaceParsedDocument {
            uri: document.uri.clone(),
            content: document.content.clone(),
            parsed,
            parse_time_ms,
            parse_cached: false,
        });
    }

    for document in library_docs {
        let parse_start = Instant::now();
        let Ok(parsed) = sysml_v2_parser::parse(&document.content) else {
            continue;
        };
        let parse_time_ms = parse_start.elapsed().as_millis().max(1) as u32;
        let doc_graph = build_graph_from_doc(&parsed, &document.uri);
        graph.merge_skip_existing_qualified_names(doc_graph, &workspace_packages);
        parsed_docs.push(WorkspaceParsedDocument {
            uri: document.uri.clone(),
            content: document.content.clone(),
            parsed,
            parse_time_ms,
            parse_cached: false,
        });
    }

    finalize_and_evaluate(&mut graph);

    Ok((graph, parsed_docs))
}

fn parse_and_build(document: &SysmlDocument) -> Option<(SemanticGraph, WorkspaceParsedDocument)> {
    let parse_start = Instant::now();
    let parsed = sysml_v2_parser::parse(&document.content).ok()?;
    let parse_time_ms = parse_start.elapsed().as_millis().max(1) as u32;
    let doc_graph = build_graph_from_doc(&parsed, &document.uri);
    let entry = WorkspaceParsedDocument {
        uri: document.uri.clone(),
        content: document.content.clone(),
        parsed,
        parse_time_ms,
        parse_cached: false,
    };
    Some((doc_graph, entry))
}

/// Parses, builds, and links a semantic graph from many documents in parallel — the
/// full-workspace equivalent of [`patch_graph_for_document`]. Same end result as
/// [`build_and_link_graph`] (same nodes, same edges), computed differently: parsing and
/// per-document graph building run in parallel (two phases — workspace documents first,
/// since library merging needs the complete set of workspace-declared package names to
/// avoid shadowing — each phase internally parallel), and cross-document edges are
/// resolved via parallel per-URI resolution (see [`link_workspace_derivations`]'s doc
/// comment) instead of the sequential whole-graph scan inside
/// [`link_workspace_relationships`].
///
/// See `docs/engineering/TIER2-PHASE3B-STEP5-FULL-REBUILD-DESIGN.md` for why this exists
/// and the equivalence testing this function's own test module is expected to carry.
pub fn build_and_link_graph_parallel(
    documents: &[SysmlDocument],
) -> (SemanticGraph, Vec<WorkspaceParsedDocument>) {
    let (workspace_docs, library_docs): (Vec<&SysmlDocument>, Vec<&SysmlDocument>) = documents
        .iter()
        .partition(|document| !matches!(document.source_kind, SysmlDocumentSourceKind::Library));

    let mut graph = SemanticGraph::new();
    let mut parsed_docs = Vec::new();

    // Phase 1: workspace documents. Must finish (and its declared-package set must be
    // complete) before phase 2 starts.
    let workspace_parsed: Vec<(SemanticGraph, WorkspaceParsedDocument)> = workspace_docs
        .par_iter()
        .filter_map(|document| parse_and_build(document))
        .collect();
    let workspace_packages: HashSet<String> = workspace_parsed
        .iter()
        .flat_map(|(_, entry)| declared_packages_from_parsed(&entry.parsed))
        .collect();
    for (doc_graph, entry) in workspace_parsed {
        graph.merge(doc_graph);
        parsed_docs.push(entry);
    }

    // Phase 2: library documents, merged skipping anything the workspace already declared.
    let library_parsed: Vec<(SemanticGraph, WorkspaceParsedDocument)> = library_docs
        .par_iter()
        .filter_map(|document| parse_and_build(document))
        .collect();
    for (doc_graph, entry) in library_parsed {
        graph.merge_skip_existing_qualified_names(doc_graph, &workspace_packages);
        parsed_docs.push(entry);
    }

    // Parallel cross-document edge resolution, replacing the sequential typing/
    // specializes/subject scan inside `link_workspace_relationships`.
    let uris: Vec<Url> = documents.iter().map(|document| document.uri.clone()).collect();
    let resolved_edges: Vec<_> = uris
        .par_iter()
        .flat_map(|uri| resolve_cross_document_edges_for_uri(&graph, uri))
        .collect();
    for (src_id, tgt_id, kind) in resolved_edges {
        if let (Some(&src_idx), Some(&tgt_idx)) = (
            graph.node_index_by_id.get(&src_id),
            graph.node_index_by_id.get(&tgt_id),
        ) {
            graph.graph.add_edge(src_idx, tgt_idx, SemanticEdge::plain(kind));
        }
    }
    graph.invalidate_query_indexes();

    link_workspace_derivations(&mut graph);
    prepare_analysis_evaluation_context(&mut graph);
    resolve_workspace_pending_relationships(&mut graph);
    evaluate_expressions(&mut graph);
    graph.invalidate_query_indexes();

    (graph, parsed_docs)
}

/// Link, prepare analysis context, and resolve pending relationships after graph mutation.
pub fn finalize_workspace_graph(graph: &mut SemanticGraph) {
    link_workspace_relationships(graph);
    prepare_analysis_evaluation_context(graph);
    resolve_workspace_pending_relationships(graph);
    // Edge additions above go via graph.graph.add_edge() directly, bypassing
    // insert_workspace_edge. Invalidate here so the first post-finalization query
    // builds the edge index with all edges present.
    graph.invalidate_query_indexes();
}

/// [`finalize_workspace_graph`] plus expression evaluation. Use this (not
/// `finalize_workspace_graph` directly) wherever a graph needs to end up in a fully
/// up-to-date, query-ready state — i.e. after a full build or a settled incremental
/// update. Callers that want a fast, low-latency patch during rapid live edits (deferring
/// evaluation to a later catch-up pass) should call `finalize_workspace_graph` directly
/// instead, the same way `patch_graph_for_document`'s `evaluate: false` path does.
pub fn finalize_and_evaluate(graph: &mut SemanticGraph) {
    finalize_workspace_graph(graph);
    evaluate_expressions(graph);
    graph.invalidate_query_indexes();
}

/// Patches `graph` in place for a single changed document: removes that document's
/// existing nodes, rebuilds and merges its subgraph (if parsed content is provided), and
/// refreshes cross-document edges touching it.
///
/// When `evaluate` is `true`, also relinks workspace relationships, resolves pending
/// relationships, and re-evaluates expressions across the graph (via
/// [`finalize_and_evaluate`]). Pass `false` to skip those steps for a fast, low-latency
/// patch (e.g. on every keystroke) and call [`finalize_and_evaluate`] later once edits
/// settle.
pub fn patch_graph_for_document(
    graph: &mut SemanticGraph,
    uri: &Url,
    parsed: Option<&sysml_v2_parser::RootNamespace>,
    evaluate: bool,
) {
    graph.remove_nodes_for_uri(uri);
    let Some(parsed) = parsed else {
        return;
    };
    let doc_graph = build_graph_from_doc(parsed, uri);
    graph.merge(doc_graph);
    add_cross_document_edges_for_uri(graph, uri);
    if evaluate {
        finalize_and_evaluate(graph);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic::model::RelationshipKind;

    fn parse(_uri: &Url, content: &str) -> sysml_v2_parser::RootNamespace {
        sysml_v2_parser::parse(content).expect("parse")
    }

    #[test]
    fn patch_with_none_clears_the_uris_nodes() {
        let uri = Url::parse("file:///demo.sysml").expect("uri");
        let mut graph = SemanticGraph::new();
        let parsed = parse(&uri, "package Demo { part def Engine; }");
        patch_graph_for_document(&mut graph, &uri, Some(&parsed), true);
        assert!(!graph.nodes_for_uri(&uri).is_empty());

        patch_graph_for_document(&mut graph, &uri, None, true);
        assert!(graph.nodes_for_uri(&uri).is_empty());
    }

    #[test]
    fn patch_matches_manual_build_merge_and_cross_edges() {
        let uri = Url::parse("file:///demo.sysml").expect("uri");
        let content = "package Demo { part def Engine; part motor : Engine; }";
        let parsed = parse(&uri, content);

        let mut patched = SemanticGraph::new();
        patch_graph_for_document(&mut patched, &uri, Some(&parsed), false);

        let mut manual = SemanticGraph::new();
        manual.remove_nodes_for_uri(&uri);
        let doc_graph = build_graph_from_doc(&parsed, &uri);
        manual.merge(doc_graph);
        add_cross_document_edges_for_uri(&mut manual, &uri);

        let patched_names: std::collections::BTreeSet<_> = patched
            .nodes_for_uri(&uri)
            .iter()
            .map(|node| node.id.qualified_name.clone())
            .collect();
        let manual_names: std::collections::BTreeSet<_> = manual
            .nodes_for_uri(&uri)
            .iter()
            .map(|node| node.id.qualified_name.clone())
            .collect();
        assert_eq!(patched_names, manual_names);
        assert!(!patched_names.is_empty());
    }

    #[test]
    fn evaluate_false_skips_expression_evaluation() {
        let uri = Url::parse("file:///demo.sysml").expect("uri");
        let parsed = parse(
            &uri,
            "package Demo { part def Rocket { attribute mass = 1 + 2; } }",
        );
        let mut graph = SemanticGraph::new();
        patch_graph_for_document(&mut graph, &uri, Some(&parsed), false);

        let mass = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.name == "mass")
            .expect("mass attribute node");
        assert!(
            !mass.attributes.contains_key("evaluatedValue"),
            "evaluate: false should not populate evaluatedValue"
        );
    }

    #[test]
    fn evaluate_true_populates_evaluated_value() {
        let uri = Url::parse("file:///demo.sysml").expect("uri");
        let parsed = parse(
            &uri,
            "package Demo { part def Rocket { attribute mass = 1 + 2; } }",
        );
        let mut graph = SemanticGraph::new();
        patch_graph_for_document(&mut graph, &uri, Some(&parsed), true);

        let mass = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|node| node.name == "mass")
            .expect("mass attribute node");
        assert_eq!(
            mass.attributes.get("evaluatedValue"),
            Some(&serde_json::json!(3))
        );
    }

    /// Equivalence fixture for `build_and_link_graph` vs. `build_and_link_graph_parallel`
    /// (Tier 2 Phase 3b Step 5a — see `docs/engineering/TIER2-PHASE3B-STEP5-FULL-REBUILD-DESIGN.md`).
    /// Exercises cross-document typing (`part mobility : MobilitySubsystem` resolved via
    /// import) and cross-document subject edges (`subject robot : AutonomousFloorCleaningRobot`,
    /// resolved via `private import Architecture::*`) — the two edge kinds
    /// `resolve_cross_document_edges_for_uri` handles and the sequential
    /// `link_workspace_relationships` scan is being replaced for. Also includes a
    /// same-document `specializes` (`:>`) edge and a library document merged with
    /// `merge_skip_existing_qualified_names`, to exercise both phases. Also includes a
    /// cross-document derivation connection (`StakeholderNeeds`/`SystemRequirements`,
    /// mirroring `requirement_derivation_semantics.rs`), exercising the
    /// `link_workspace_derivations` rewiring that runs after parallel cross-document edge
    /// resolution — the one thing that phase does not cover.
    fn equivalence_fixture_documents() -> Vec<SysmlDocument> {
        const ARCHITECTURE: &str = r#"
package Architecture {
  part def MobilitySubsystem {
    attribute drivePowerW : Real = 28;
  }
  part def AutonomousFloorCleaningRobot {
    part mobility : MobilitySubsystem;
  }
  part def PremiumFloorCleaningRobot :> AutonomousFloorCleaningRobot;
}
"#;
        const ANALYSIS_CASES: &str = r#"
package AnalysisCases {
  private import Architecture::*;

  analysis def TotalPowerConsumptionAnalysis {
    attribute powerBudgetW : Real = 55;
    subject robot : AutonomousFloorCleaningRobot;
    return ref powerWithinBudget {
      return sum(robot.mobility.drivePowerW) <= powerBudgetW;
    }
  }
}
"#;
        const UNITS_LIBRARY: &str = r#"
package Units {
  attribute <m> 'metre' : LengthUnit;
}
"#;
        const STAKEHOLDER_NEEDS: &str = r#"
package StakeholderNeeds {
  requirement def CleanLargeAreas;
  requirement cleanLargeAreas : CleanLargeAreas;
}
"#;
        const SYSTEM_REQUIREMENTS: &str = r#"
package SystemRequirements {
  private import StakeholderNeeds::*;

  requirement def CleanAtLeastEightySquareMetersPerCharge;
  requirement cleanAtLeastEighty : CleanAtLeastEightySquareMetersPerCharge;

  #derivation connection {
    end #original ::> cleanLargeAreas;
    end #derive ::> cleanAtLeastEighty;
  }
}
"#;
        vec![
            SysmlDocument::from_memory_path(
                "equivalence",
                "Architecture.sysml",
                ARCHITECTURE.to_string(),
                SysmlDocumentSourceKind::Workspace,
                None,
                None,
            )
            .expect("architecture uri"),
            SysmlDocument::from_memory_path(
                "equivalence",
                "AnalysisCases.sysml",
                ANALYSIS_CASES.to_string(),
                SysmlDocumentSourceKind::Workspace,
                None,
                None,
            )
            .expect("analysis uri"),
            SysmlDocument::from_memory_path(
                "equivalence",
                "Units.sysml",
                UNITS_LIBRARY.to_string(),
                SysmlDocumentSourceKind::Library,
                None,
                None,
            )
            .expect("library uri"),
            SysmlDocument::from_memory_path(
                "equivalence",
                "StakeholderNeeds.sysml",
                STAKEHOLDER_NEEDS.to_string(),
                SysmlDocumentSourceKind::Workspace,
                None,
                None,
            )
            .expect("stakeholder needs uri"),
            SysmlDocument::from_memory_path(
                "equivalence",
                "SystemRequirements.sysml",
                SYSTEM_REQUIREMENTS.to_string(),
                SysmlDocumentSourceKind::Workspace,
                None,
                None,
            )
            .expect("system requirements uri"),
        ]
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

    #[test]
    fn parallel_build_matches_sequential_build_nodes_and_edges() {
        let documents = equivalence_fixture_documents();

        let (sequential_graph, sequential_parsed) =
            build_and_link_graph(&documents).expect("sequential build");
        let (parallel_graph, parallel_parsed) = build_and_link_graph_parallel(&documents);

        let sequential_nodes = node_qualified_names(&sequential_graph);
        let parallel_nodes = node_qualified_names(&parallel_graph);
        assert_eq!(
            sequential_nodes, parallel_nodes,
            "node sets differ between sequential and parallel full build"
        );
        assert!(!sequential_nodes.is_empty(), "fixture should produce nodes");

        let sequential_edges = edge_triples(&sequential_graph);
        let parallel_edges = edge_triples(&parallel_graph);
        assert_eq!(
            sequential_edges, parallel_edges,
            "edge sets differ between sequential and parallel full build"
        );
        assert!(!sequential_edges.is_empty(), "fixture should produce edges");

        let derivation_edge = (
            "StakeholderNeeds::cleanLargeAreas".to_string(),
            "SystemRequirements::cleanAtLeastEighty".to_string(),
            RelationshipKind::Derivation.as_str().to_string(),
        );
        assert!(
            sequential_edges.contains(&derivation_edge),
            "sequential build should contain the fixture's derivation-connection edge"
        );
        assert!(
            parallel_edges.contains(&derivation_edge),
            "parallel build should contain the fixture's derivation-connection edge \
             (link_workspace_derivations is the only phase that wires it)"
        );

        assert_eq!(
            sequential_parsed.len(),
            parallel_parsed.len(),
            "parsed document count should match"
        );
    }

    #[test]
    fn parallel_build_evaluates_expressions_like_sequential_build() {
        let documents = equivalence_fixture_documents();
        let (sequential_graph, _) = build_and_link_graph(&documents).expect("sequential build");
        let (parallel_graph, _) = build_and_link_graph_parallel(&documents);

        let find_drive_power = |graph: &SemanticGraph| {
            graph
                .graph
                .node_weights()
                .find(|node| node.name == "drivePowerW")
                .and_then(|node| node.attributes.get("evaluatedValue"))
                .cloned()
        };
        let sequential_value = find_drive_power(&sequential_graph);
        let parallel_value = find_drive_power(&parallel_graph);
        assert_eq!(sequential_value, parallel_value);
        assert_eq!(sequential_value, Some(serde_json::json!(28)));
    }
}
