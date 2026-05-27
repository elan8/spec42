use std::time::Instant;

use sysml_v2_parser::RootNamespace;
use url::Url;

use crate::semantic::graph::SemanticGraph;
use crate::semantic::graph_builder::build_graph_from_doc;
use crate::semantic::relationships::link_workspace_relationships;
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

    link_workspace_relationships(&mut graph);

    Ok((graph, parsed_docs))
}

/// Build semantic graph from a pluggable document provider.
pub fn build_semantic_graph_with_provider(
    provider: &impl SysmlDocumentProvider,
) -> Result<(SemanticGraph, Vec<WorkspaceParsedDocument>), String> {
    let documents = provider.load_documents()?;
    build_semantic_graph_from_documents(&documents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};

    #[test]
    fn builds_graph_from_mixed_uri_schemes() {
        let workspace_doc = SysmlDocument::from_memory_path(
            "workspace",
            "Workspace.sysml",
            "package W { part def Thing {} }".to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("workspace doc");
        let library_doc = SysmlDocument::from_uri(
            "surreal://org-1/project-1/lib/Library.sysml",
            "package L { part def ExternalThing {} }".to_string(),
            Some("Library.sysml".to_string()),
            SysmlDocumentSourceKind::External,
            None,
            None,
        )
        .expect("library doc");

        let (graph, parsed) =
            build_semantic_graph_from_documents(&[workspace_doc, library_doc]).expect("graph");

        assert_eq!(parsed.len(), 2, "both documents should be parsed");
        assert!(
            graph.node_ids_by_qualified_name.contains_key("W::Thing"),
            "workspace declaration should be present"
        );
        assert!(
            graph.node_ids_by_qualified_name.contains_key("L::ExternalThing"),
            "custom-scheme declaration should be present"
        );
    }

    #[test]
    fn partial_document_sets_keep_missing_type_unlinked() {
        let analysis_doc = SysmlDocument::from_memory_path(
            "workspace",
            "AnalysisCases.sysml",
            r#"
package AnalysisCases {
  analysis def A {
    subject robot : MissingRobot;
    return ref ok { return true; }
  }
}
"#
            .to_string(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("analysis doc");

        let (graph, _parsed) = build_semantic_graph_from_documents(&[analysis_doc]).expect("graph");
        let subject_id = graph
            .node_ids_by_qualified_name
            .get("AnalysisCases::A::robot")
            .and_then(|ids| ids.first())
            .expect("subject node");
        let subject = graph.get_node(subject_id).expect("subject");
        let typing_targets = graph.outgoing_targets_by_kind(subject, crate::RelationshipKind::Typing);
        assert!(
            typing_targets.is_empty(),
            "missing dependency should remain unresolved for strict subset behavior"
        );
    }
}
