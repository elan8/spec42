//! Syntax-fidelity helpers owned by the language service.

use sysml_query::resolved_slice::{TextPosition, TextRange};
use sysml_v2_parser::ast::{Identification, QualifiedIdentification, Span};
use sysml_v2_parser::ParsedDocument;

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

/// The authored label of a namespace-owning declaration.
///
/// Unlike [`identification_name`], a package/library-package/namespace name may be a *qualified*
/// path (`package A::B { ... }`). The simple alternative carries its own label, but the qualified
/// one is an arena identity owned by the document, so rendering it back to authored text needs
/// that document rather than the node alone.
pub(crate) fn qualified_identification_name(
    document: &ParsedDocument,
    identification: &QualifiedIdentification,
) -> String {
    use sysml_v2_parser::ast::DeclarationName;
    match identification.name.as_ref() {
        Some(DeclarationName::Simple(name)) => name.clone(),
        Some(DeclarationName::Qualified(name)) => document
            .qualified_declaration_name(*name)
            .map(|view| view.authored_text().to_string())
            .unwrap_or_default(),
        None => identification.short_name.clone().unwrap_or_default(),
    }
}
