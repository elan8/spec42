//! Editor token indices for the syntax roles the parser authority publishes.
//!
//! The traversal that finds these spans lives in `sysml_resolution::syntax`, because walking the
//! AST is the parser authority's business. What each role should look like in an editor is
//! presentation policy, and that is this crate's business -- so the mapping from role to token
//! index lives here and nowhere else.

use sysml_query::syntax::{ParsedSource, SyntaxRange, SyntaxRole};

use crate::types::*;

/// 0-based source range (LSP convention) for semantic tokens and range checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRange {
    pub start_line: u32,
    pub start_character: u32,
    pub end_line: u32,
    pub end_character: u32,
}

impl From<SyntaxRange> for SourceRange {
    fn from(range: SyntaxRange) -> Self {
        Self {
            start_line: range.start_line,
            start_character: range.start_character,
            end_line: range.end_line,
            end_character: range.end_character,
        }
    }
}

/// The editor token index each syntax role is painted with.
fn token_index(role: SyntaxRole) -> u32 {
    match role {
        SyntaxRole::Namespace => TYPE_NAMESPACE,
        SyntaxRole::Class => TYPE_CLASS,
        SyntaxRole::Type => TYPE_TYPE,
        SyntaxRole::Property => TYPE_PROPERTY,
        SyntaxRole::Interface => TYPE_INTERFACE,
        SyntaxRole::Function => TYPE_FUNCTION,
    }
}

/// AST-driven semantic token ranges for a document the authority already parsed.
pub fn ast_semantic_ranges(document: &ParsedSource, _source: &str) -> Vec<(SourceRange, u32)> {
    document
        .token_roles()
        .into_iter()
        .map(|(range, role)| (range.into(), token_index(role)))
        .collect()
}
