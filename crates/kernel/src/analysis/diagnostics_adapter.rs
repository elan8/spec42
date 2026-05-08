use semantic_core::{DiagnosticSeverity as CoreSeverity, SemanticDiagnostic};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString};

use crate::common::text_span::to_lsp_range;

pub fn semantic_to_lsp_diagnostic(diagnostic: SemanticDiagnostic) -> Diagnostic {
    Diagnostic {
        range: to_lsp_range(diagnostic.range),
        severity: Some(match diagnostic.severity {
            CoreSeverity::Error => DiagnosticSeverity::ERROR,
            CoreSeverity::Warning => DiagnosticSeverity::WARNING,
            CoreSeverity::Information => DiagnosticSeverity::INFORMATION,
            CoreSeverity::Hint => DiagnosticSeverity::HINT,
        }),
        code: Some(NumberOrString::String(diagnostic.code)),
        code_description: None,
        source: Some(diagnostic.source),
        message: diagnostic.message,
        related_information: None,
        tags: None,
        data: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use semantic_core::{DiagnosticSeverity as CoreSeverity, SemanticDiagnostic, TextPosition, TextRange};
    use tower_lsp::lsp_types::DiagnosticSeverity as LspSeverity;
    use url::Url;

    #[test]
    fn maps_semantic_diagnostic_to_lsp_diagnostic() {
        let semantic = SemanticDiagnostic {
            uri: Url::parse("file:///test.sysml").expect("uri"),
            range: TextRange::new(TextPosition::new(3, 2), TextPosition::new(3, 12)),
            severity: CoreSeverity::Warning,
            source: "semantic".to_string(),
            code: "unresolved_type_reference".to_string(),
            message: "type ref unresolved".to_string(),
            related_information: Vec::new(),
        };
        let lsp = semantic_to_lsp_diagnostic(semantic);
        assert_eq!(lsp.range.start.line, 3);
        assert_eq!(lsp.range.start.character, 2);
        assert_eq!(lsp.severity, Some(LspSeverity::WARNING));
        assert_eq!(
            lsp.code,
            Some(NumberOrString::String("unresolved_type_reference".to_string()))
        );
    }
}
