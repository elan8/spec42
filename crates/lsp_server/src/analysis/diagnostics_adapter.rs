use crate::common::text_span::to_lsp_range;
use sysml_diagnostics::{
    DiagnosticRelatedInfo, DiagnosticSeverity as CoreSeverity, SemanticDiagnostic,
};
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, NumberOrString,
};

pub fn semantic_to_lsp_diagnostic(diagnostic: SemanticDiagnostic) -> Diagnostic {
    Diagnostic {
        range: to_lsp_range(diagnostic.range),
        severity: Some(match diagnostic.severity {
            CoreSeverity::Error => DiagnosticSeverity::ERROR,
            CoreSeverity::Warning => DiagnosticSeverity::WARNING,
            CoreSeverity::Information => DiagnosticSeverity::INFORMATION,
        }),
        code: Some(NumberOrString::String(diagnostic.code)),
        code_description: None,
        source: Some(diagnostic.source),
        message: diagnostic.message,
        related_information: map_related_information(&diagnostic.related_information),
        tags: None,
        data: None,
    }
}

fn map_related_information(
    related: &[DiagnosticRelatedInfo],
) -> Option<Vec<DiagnosticRelatedInformation>> {
    if related.is_empty() {
        return None;
    }
    Some(
        related
            .iter()
            .map(|info| DiagnosticRelatedInformation {
                location: Location {
                    uri: info.uri.clone(),
                    range: to_lsp_range(info.range),
                },
                message: info.message.clone(),
            })
            .collect(),
    )
}
