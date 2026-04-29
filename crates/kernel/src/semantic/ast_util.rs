//! Helpers for working with sysml-v2-parser AST: span/range conversion and name extraction.

use sysml_v2_parser::ast::Identification;
use sysml_v2_parser::Span;
use tower_lsp::lsp_types::{Position, Range};

/// Converts sysml-v2-parser Span (1-based line/column) to LSP Range (0-based).
pub(crate) fn span_to_range(span: &Span) -> Range {
    let (start_line, start_char, end_line, end_char) = span.to_lsp_range();
    Range::new(
        Position::new(start_line, start_char),
        Position::new(end_line, end_char),
    )
}

/// Returns the display name from Identification (name, or short_name, or empty string).
pub(crate) fn identification_name(ident: &Identification) -> String {
    ident
        .name
        .as_deref()
        .or(ident.short_name.as_deref())
        .unwrap_or("")
        .to_string()
}
