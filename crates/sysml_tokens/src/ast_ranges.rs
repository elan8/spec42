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

/// Narrow a wide declaration AST span to the declared name token only.
///
/// Keeps `state` / `item` / `def` as lexer keywords while still classifying the
/// definition name (e.g. `Idle`) with the AST token type.
///
/// The declared name always sits on the span's first line, right after the leading keyword(s),
/// even when the declaration's body (and so the whole node's span) continues across further
/// lines -- only that first line is searched, rather than giving up whenever the span isn't
/// single-line. Without this, every character within a multi-line body -- e.g. each bare
/// `entry;` / `standard;` literal inside a multi-line `enum def ProductTier { ... }` -- fell back
/// to inheriting the wide, unnarrowed range's own token type (the enum's "class" color), instead
/// of getting no color (or their own) at all.
pub fn narrow_declaration_name_range(source: &str, range: &SourceRange) -> Option<SourceRange> {
    let line = source.lines().nth(range.start_line as usize)?;
    let start = range.start_character as usize;
    let end = if range.start_line == range.end_line {
        range.end_character as usize
    } else {
        line.chars().count()
    };
    if end <= start {
        return None;
    }
    let slice: String = line.chars().skip(start).take(end - start).collect();
    let (name_start, name_end) = declaration_name_bounds_in_slice(&slice)?;
    let name_start = start + name_start;
    let name_end = start + name_end;
    Some(SourceRange {
        start_line: range.start_line,
        start_character: name_start as u32,
        end_line: range.start_line,
        end_character: name_end as u32,
    })
}

fn declaration_name_bounds_in_slice(slice: &str) -> Option<(usize, usize)> {
    const DEF_PREFIX: &str = "def ";
    if let Some(def_idx) = slice.find(DEF_PREFIX) {
        return identifier_bounds(&slice[def_idx + DEF_PREFIX.len()..]).map(
            |(rel_start, rel_end)| {
                (
                    def_idx + DEF_PREFIX.len() + rel_start,
                    def_idx + DEF_PREFIX.len() + rel_end,
                )
            },
        );
    }
    for prefix in [
        "package ",
        "library ",
        "alias ",
        "view ",
        "viewpoint ",
        "rendering ",
    ] {
        if let Some(idx) = slice.find(prefix) {
            let after = idx + prefix.len();
            return identifier_bounds(&slice[after..])
                .map(|(rel_start, rel_end)| (after + rel_start, after + rel_end));
        }
    }
    None
}

/// Post-process AST semantic ranges so definition headers highlight names only.
pub fn refine_declaration_ranges(
    source: &str,
    ranges: &[(SourceRange, u32)],
) -> Vec<(SourceRange, u32)> {
    ranges
        .iter()
        .map(|(range, ty)| {
            let should_narrow = matches!(
                *ty,
                TYPE_CLASS | TYPE_INTERFACE | TYPE_FUNCTION | TYPE_NAMESPACE | TYPE_TYPE
            );
            if should_narrow {
                if let Some(narrowed) = narrow_declaration_name_range(source, range) {
                    return (narrowed, *ty);
                }
            }
            (range.clone(), *ty)
        })
        .collect()
}

fn identifier_bounds(slice: &str) -> Option<(usize, usize)> {
    let trimmed = slice.trim_start();
    let ws = slice.len() - trimmed.len();
    let bytes = trimmed.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    let mut end = 0usize;
    if bytes[0] == b'\'' {
        end = 1;
        while end < bytes.len() && bytes[end] != b'\'' {
            end += 1;
        }
        if end < bytes.len() {
            end += 1;
        }
    } else if bytes[0] == b'<' {
        end = 1;
        while end < bytes.len() && bytes[end] != b'>' {
            end += 1;
        }
        if end < bytes.len() {
            end += 1;
        }
        let rest = &trimmed[end..];
        let rest_trim = rest.trim_start();
        let inner_ws = rest.len() - rest_trim.len();
        end += inner_ws;
        let ident = identifier_bounds(rest_trim)?;
        end += ident.1;
    } else {
        while end < bytes.len() && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'_') {
            end += 1;
        }
    }
    if end == 0 {
        return None;
    }
    Some((ws, ws + end))
}
