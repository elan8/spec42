//! Helpers for working with sysml-v2-parser AST: span/range conversion and name extraction.

use sysml_v2_parser::ast::Identification;
use sysml_v2_parser::Span;

use crate::types::{TYPE_CLASS, TYPE_FUNCTION, TYPE_INTERFACE, TYPE_NAMESPACE, TYPE_TYPE};

/// 0-based source range (LSP convention) for semantic tokens and range checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRange {
    pub start_line: u32,
    pub start_character: u32,
    pub end_line: u32,
    pub end_character: u32,
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

/// Narrow a wide declaration AST span to the declared name token only.
///
/// Keeps `state` / `item` / `def` as lexer keywords while still classifying the
/// definition name (e.g. `Idle`) with the AST token type.
pub fn narrow_declaration_name_range(source: &str, range: &SourceRange) -> Option<SourceRange> {
    if range.start_line != range.end_line {
        return None;
    }
    let line = source.lines().nth(range.start_line as usize)?;
    let start = range.start_character as usize;
    let end = range.end_character as usize;
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
        end_line: range.end_line,
        end_character: name_end as u32,
    })
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

fn declaration_name_bounds_in_slice(slice: &str) -> Option<(usize, usize)> {
    const DEF_PREFIX: &str = "def ";
    if let Some(def_idx) = slice.find(DEF_PREFIX) {
        return identifier_bounds(&slice[def_idx + DEF_PREFIX.len()..])
            .map(|(rel_start, rel_end)| (def_idx + DEF_PREFIX.len() + rel_start, def_idx + DEF_PREFIX.len() + rel_end));
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
