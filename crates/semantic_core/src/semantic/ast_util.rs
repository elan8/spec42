//! Helpers for working with sysml-v2-parser AST: span/range conversion and name extraction.

use crate::semantic::text_span::{TextPosition, TextRange};
use sysml_v2_parser::ast::Identification;
use sysml_v2_parser::Span;

/// Converts sysml-v2-parser Span (1-based line/column) to 0-based TextRange.
pub fn span_to_range(span: &Span) -> TextRange {
    let (start_line, start_char, end_line, end_char) = span.to_lsp_range();
    TextRange::new(
        TextPosition::new(start_line, start_char),
        TextPosition::new(end_line, end_char),
    )
}

pub fn text_range_to_json(range: TextRange) -> serde_json::Value {
    serde_json::json!({
        "start": {
            "line": range.start.line,
            "character": range.start.character,
        },
        "end": {
            "line": range.end.line,
            "character": range.end.character,
        },
    })
}

/// Returns the display name from Identification (name, or short_name, or empty string).
pub fn identification_name(ident: &Identification) -> String {
    ident
        .name
        .as_deref()
        .or(ident.short_name.as_deref())
        .unwrap_or("")
        .to_string()
}
