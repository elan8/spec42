//! Orchestrates semantic graph materialize → link → pending resolve.

use std::collections::HashSet;
use std::time::Instant;

use crate::semantic::analysis_typing::prepare_analysis_evaluation_context;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::graph_builder::build_graph_from_doc;
use crate::semantic::library_loader::declared_packages_in_content;
use crate::semantic::relationships::{
    link_workspace_relationships, resolve_workspace_pending_relationships,
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
    for document in &workspace_docs {
        workspace_packages.extend(declared_packages_in_content(&document.content));
    }

    for document in workspace_docs.into_iter().chain(library_docs) {
        let parse_start = Instant::now();
        let Ok(parsed) = sysml_v2_parser::parse(&document.content) else {
            continue;
        };
        let parse_time_ms = parse_start.elapsed().as_millis().max(1) as u32;
        let doc_graph = build_graph_from_doc(&parsed, &document.uri);
        if document.source_kind == SysmlDocumentSourceKind::Library {
            graph.merge_skip_existing_qualified_names(doc_graph, &workspace_packages);
        } else {
            graph.merge(doc_graph);
        }
        parsed_docs.push(WorkspaceParsedDocument {
            uri: document.uri.clone(),
            content: document.content.clone(),
            parsed,
            parse_time_ms,
            parse_cached: false,
        });
    }

    link_workspace_relationships(&mut graph);
    prepare_analysis_evaluation_context(&mut graph);
    resolve_workspace_pending_relationships(&mut graph);

    Ok((graph, parsed_docs))
}
