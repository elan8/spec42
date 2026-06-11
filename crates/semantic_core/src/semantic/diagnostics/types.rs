use crate::semantic::text_span::{TextPosition, TextRange};
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticRelatedInfo {
    pub uri: Url,
    pub range: TextRange,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticDiagnostic {
    pub uri: Url,
    pub range: TextRange,
    pub severity: DiagnosticSeverity,
    pub source: String,
    pub code: String,
    pub message: String,
    pub related_information: Vec<DiagnosticRelatedInfo>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DiagnosticsOptions<'a> {
    pub include_hints: bool,
    /// In-memory workspace/library sources (e.g. QUDV catalog files from library closure).
    pub indexed_sources: &'a [(&'a Url, &'a str)],
}

impl SemanticDiagnostic {
    pub fn unknown(uri: Url, message: impl Into<String>) -> Self {
        Self {
            uri,
            range: TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 0)),
            severity: DiagnosticSeverity::Warning,
            source: "semantic".to_string(),
            code: "semantic_diagnostic".to_string(),
            message: message.into(),
            related_information: Vec::new(),
        }
    }
}
