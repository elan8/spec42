//! Orchestrates semantic graph materialize → link → pending resolve.

use std::collections::HashSet;
use std::time::Instant;

use url::Url;

use crate::semantic::analysis_typing::prepare_analysis_evaluation_context;
use crate::semantic::evaluation::evaluate_expressions;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::graph_builder::build_graph_from_doc;
use crate::semantic::library_loader::declared_packages_from_parsed;
use crate::semantic::relationships::{
    add_cross_document_edges_for_uri, link_workspace_relationships,
    resolve_workspace_pending_relationships,
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
}
