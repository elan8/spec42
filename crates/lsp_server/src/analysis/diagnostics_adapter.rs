use sysml_diagnostics::{
    DiagnosticRelatedInfo, DiagnosticSeverity as CoreSeverity, SemanticDiagnostic,
};
use sysml_query::resolved_slice::TextRange;
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, NumberOrString,
    Position, Range,
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

/// The publication's range in the protocol's shape.
///
/// Both are zero-based line/UTF-16 character pairs, which is the contract the resolution owner
/// publishes and the one LSP defines, so this is a shape change and not a coordinate conversion.
fn to_lsp_range(range: TextRange) -> Range {
    Range {
        start: Position::new(range.start.line, range.start.character),
        end: Position::new(range.end.line, range.end.character),
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
