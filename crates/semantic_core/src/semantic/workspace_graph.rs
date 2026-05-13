use std::time::Instant;

use sysml_v2_parser::RootNamespace;
use url::Url;

use crate::semantic::graph::SemanticGraph;
use crate::semantic::graph_builder::build_graph_from_doc;
use crate::semantic::relationships::add_cross_document_edges_for_uri;
use crate::semantic::source::{SysmlDocument, SysmlDocumentProvider};

#[derive(Debug, Clone)]
pub struct WorkspaceParsedDocument {
    pub uri: Url,
    pub content: String,
    pub parsed: RootNamespace,
    pub parse_time_ms: u32,
    pub parse_cached: bool,
}

/// Build a merged semantic graph from pre-loaded SysML documents.
///
/// Returns the merged graph and parsed documents used to build it.
pub fn build_semantic_graph_from_documents(
    documents: &[SysmlDocument],
) -> Result<(SemanticGraph, Vec<WorkspaceParsedDocument>), String> {
    let mut graph = SemanticGraph::new();
    let mut parsed_docs = Vec::new();

    for document in documents {
        let parse_start = Instant::now();
        let Ok(parsed) = sysml_v2_parser::parse(&document.content) else {
            continue;
        };
        let parse_time_ms = parse_start.elapsed().as_millis().max(1) as u32;
        graph.merge(build_graph_from_doc(&parsed, &document.uri));
        parsed_docs.push(WorkspaceParsedDocument {
            uri: document.uri.clone(),
            content: document.content.clone(),
            parsed,
            parse_time_ms,
            parse_cached: false,
        });
    }

    for doc in &parsed_docs {
        add_cross_document_edges_for_uri(&mut graph, &doc.uri);
    }

    Ok((graph, parsed_docs))
}

/// Build semantic graph from a pluggable document provider.
pub fn build_semantic_graph_with_provider(
    provider: &impl SysmlDocumentProvider,
) -> Result<(SemanticGraph, Vec<WorkspaceParsedDocument>), String> {
    let documents = provider.load_documents()?;
    build_semantic_graph_from_documents(&documents)
}
