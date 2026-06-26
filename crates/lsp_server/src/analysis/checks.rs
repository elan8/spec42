use tower_lsp::lsp_types::{Diagnostic, Url};

use crate::analysis::diagnostics_adapter::semantic_to_lsp_diagnostic;
use crate::host::config::DiagnosticsHostContext;
use crate::semantic::SemanticGraph;

/// Returns LSP diagnostics for semantic rules in the given document.
/// Semantic rule evaluation is owned by sysml_model.
pub fn compute_semantic_diagnostics(
    graph: &SemanticGraph,
    uri: &Url,
    _ctx: DiagnosticsHostContext,
) -> Vec<Diagnostic> {
    sysml_model::collect_diagnostics_from_graph(
        graph,
        uri,
        sysml_model::DiagnosticsOptions {
            include_hints: false,
        },
    )
    .into_iter()
    .map(semantic_to_lsp_diagnostic)
    .collect()
}

/// Implements [crate::host::config::SemanticCheckProvider] for use in [crate::host::config::Spec42Config].
#[derive(Debug, Default)]
pub struct DefaultSemanticChecks;

impl crate::host::config::SemanticCheckProvider for DefaultSemanticChecks {
    fn compute_diagnostics_with_context(
        &self,
        graph: &SemanticGraph,
        uri: &Url,
        ctx: DiagnosticsHostContext,
    ) -> Vec<Diagnostic> {
        compute_semantic_diagnostics(graph, uri, ctx)
    }
}
