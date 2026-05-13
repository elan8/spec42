use std::time::Instant;

use crate::semantic::visualization_workspace::build_sysml_visualization_from_graph_and_documents;
use crate::semantic::workspace_graph::WorkspaceParsedDocument;
use crate::{SemanticGraph, SysmlVisualizationResultDto};

/// Graph-first visualization entrypoint aligned with the Spec42 LSP kernel’s view catalog and selection.
///
/// Pass the same [`WorkspaceParsedDocument`] slice returned from
/// [`crate::semantic::workspace_graph::build_semantic_graph_with_provider`] (or
/// [`crate::semantic::workspace_graph::build_semantic_graph_from_documents`]) so view definitions
/// and usages are discovered from the AST, not from name heuristics.
///
/// This delegates to the full workspace pipeline (activity, sequence, IBD scope, workspace model,
/// package groups, stats) with `file:///library/` as the only library path and a workspace root
/// inferred from the document URIs.
pub fn build_sysml_visualization_from_graph(
    graph: &SemanticGraph,
    documents: &[WorkspaceParsedDocument],
    view: &str,
    selected_view: Option<&str>,
) -> Result<SysmlVisualizationResultDto, String> {
    build_sysml_visualization_from_graph_and_documents(
        graph,
        documents,
        view,
        selected_view,
        Instant::now(),
    )
}
