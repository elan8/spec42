use tower_lsp::lsp_types::{Diagnostic, Url};

use crate::analysis::diagnostics_adapter::semantic_to_lsp_diagnostic;
use crate::analysis::diagnostics_postprocess::{
    postprocess_document_diagnostics, DiagnosticsPostprocessOptions,
};
use crate::semantic::SemanticGraph;

pub(crate) fn collect_document_diagnostics(
    semantic_graph: &SemanticGraph,
    library_paths: &[Url],
    uri: &Url,
    text: &str,
    postprocess: DiagnosticsPostprocessOptions,
) -> Vec<Diagnostic> {
    let unit_registry = sysml_model::UnitRegistry::from_graph(semantic_graph);
    let diagnostics: Vec<Diagnostic> = sysml_model::collect_document_diagnostics(
        semantic_graph,
        &unit_registry,
        !library_paths.is_empty(),
        uri,
        text,
        postprocess.skip_semantic_on_parse_error,
    )
    .into_iter()
    .map(semantic_to_lsp_diagnostic)
    .collect();

    postprocess_document_diagnostics(uri, diagnostics, postprocess)
}

pub(crate) fn validation_postprocess_options(strict: bool) -> DiagnosticsPostprocessOptions {
    DiagnosticsPostprocessOptions {
        suppress_semantic_after_parse_error: strict,
        skip_semantic_on_parse_error: strict,
    }
}

pub(crate) fn lsp_postprocess_options() -> DiagnosticsPostprocessOptions {
    DiagnosticsPostprocessOptions {
        suppress_semantic_after_parse_error: false,
        skip_semantic_on_parse_error: false,
    }
}
