//! Syntax-fidelity helpers owned by the language service.

use sysml_query::resolved_slice::{TextPosition, TextRange};
use sysml_v2_parser::ast::{Identification, Span};

pub(crate) fn span_to_range(span: &Span) -> TextRange {
    let (start_line, start_character, end_line, end_character) = span.to_lsp_range();
    TextRange::new(
        TextPosition::new(start_line, start_character),
        TextPosition::new(end_line, end_character),
    )
}

pub(crate) fn identification_name(identification: &Identification) -> String {
    identification
        .name
        .as_deref()
        .or(identification.short_name.as_deref())
        .unwrap_or("")
        .to_string()
}
