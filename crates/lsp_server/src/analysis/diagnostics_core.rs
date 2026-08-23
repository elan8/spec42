//! One document's LSP diagnostics, from the publication the workspace last built.

use tower_lsp::lsp_types::{Diagnostic, Url};

use sysml_diagnostics::{
    postprocess_document_diagnostics, PostprocessPolicy, ReportingPolicy, SemanticDiagnostic,
};
use sysml_query::resolved_slice::PublishedModel;

use crate::analysis::diagnostics_adapter::semantic_to_lsp_diagnostic;

/// The LSP diagnostics one document reports.
///
/// Every value comes from `model`; this maps them into the protocol's types and applies the
/// host's presentation policy. A workspace with no publication yet reports nothing, which is
/// distinct from a publication that reported nothing: the caller gates on session lifecycle
/// before asking.
pub(crate) fn collect_document_diagnostics(
    model: Option<&PublishedModel>,
    uri: &Url,
    reporting: ReportingPolicy,
    postprocess: PostprocessPolicy,
) -> Vec<Diagnostic> {
    collect_reported_diagnostics(model, uri, reporting, postprocess)
        .into_iter()
        .map(semantic_to_lsp_diagnostic)
        .collect()
}

/// The same report, before the protocol's types: the neutral values the reporting policy settled.
///
/// Batch validation needs these to ask `sysml_diagnostics` its advice question about the report it
/// is about to render, rather than re-deriving one from the protocol projection.
pub(crate) fn collect_reported_diagnostics(
    model: Option<&PublishedModel>,
    uri: &Url,
    reporting: ReportingPolicy,
    postprocess: PostprocessPolicy,
) -> Vec<SemanticDiagnostic> {
    let Some(model) = model else {
        return Vec::new();
    };
    let diagnostics = sysml_diagnostics::document_diagnostics(model, uri, reporting);
    postprocess_document_diagnostics(diagnostics, postprocess)
}

/// Batch validation reports only what the parser rejected for a document that does not parse.
pub(crate) fn validation_reporting(strict: bool) -> ReportingPolicy {
    ReportingPolicy::strict(strict)
}

pub(crate) fn validation_postprocess_options(strict: bool) -> PostprocessPolicy {
    PostprocessPolicy {
        suppress_semantic_after_parse_error: strict,
    }
}

/// Interactive editing reports everything: the author is mid-keystroke, and the semantic answers
/// about the last coherent model state are still the best available.
pub(crate) fn lsp_reporting() -> ReportingPolicy {
    ReportingPolicy::default()
}

pub(crate) fn lsp_postprocess_options() -> PostprocessPolicy {
    PostprocessPolicy {
        suppress_semantic_after_parse_error: false,
    }
}
