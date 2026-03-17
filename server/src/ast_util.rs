//! Helpers for working with sysml-parser AST: span/range conversion and name extraction.

use sysml_parser::ast::Identification;
use sysml_parser::Span;
use tower_lsp::lsp_types::{Position, Range};

/// 0-based source range (LSP convention) for semantic tokens and range checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRange {
    pub start_line: u32,
    pub start_character: u32,
    pub end_line: u32,
    pub end_character: u32,
}

/// Converts sysml-parser Span (1-based line/column) to LSP Range (0-based).
pub fn span_to_range(span: &Span) -> Range {
    let (start_line, start_char, end_line, end_char) = span.to_lsp_range();
    Range::new(
        Position::new(start_line, start_char),
        Position::new(end_line, end_char),
    )
}

/// Converts Span to our SourceRange (0-based) for semantic token range matching.
pub fn span_to_source_range(span: &Span) -> SourceRange {
    let (start_line, start_char, end_line, end_char) = span.to_lsp_range();
    SourceRange {
        start_line,
        start_character: start_char,
        end_line,
        end_character: end_char,
    }
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
