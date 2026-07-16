//! Helpers for working with sysml-v2-parser AST: span/range conversion and name extraction.

use sysml_v2_parser::ast::Identification;
use sysml_v2_parser::Span;
use sysml_v2_parser::ast::TypingRelationship;

/// Accept both legacy textual type names and parser 0.35 typed relationships.
pub trait TypeNameRef {
    fn type_name_ref(&self) -> &str;
}

impl TypeNameRef for str {
    fn type_name_ref(&self) -> &str { self }
}

impl TypeNameRef for String {
    fn type_name_ref(&self) -> &str { self }
}

impl TypeNameRef for TypingRelationship {
    fn type_name_ref(&self) -> &str {
        self.target
            .first()
            .and_then(|target| target.value.local_name())
            .unwrap_or_default()
    }
}

use crate::types::{
    TYPE_CLASS, TYPE_FUNCTION, TYPE_INTERFACE, TYPE_NAMESPACE, TYPE_PROPERTY, TYPE_TYPE,
};

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

/// Locate a whole-word occurrence of `word` inside `span` on a single source line.
pub fn word_range_within_span(
    source: &str,
    span: &SourceRange,
    word: &str,
) -> Option<SourceRange> {
    if word.is_empty() || span.start_line != span.end_line {
        return None;
    }
    let line = source.lines().nth(span.start_line as usize)?;
    let line_start = span.start_character as usize;
    let line_end = span.end_character as usize;
    if line_end <= line_start {
        return None;
    }
    let slice: String = line
        .chars()
        .skip(line_start)
        .take(line_end - line_start)
        .collect();
    let mut search_at = 0usize;
    while let Some(rel) = slice[search_at..].find(word) {
        let abs = search_at + rel;
        let before_ok = abs == 0 || !is_ident_byte(slice.as_bytes().get(abs - 1).copied());
        let after = abs + word.len();
        let after_ok = after >= slice.len() || !is_ident_byte(slice.as_bytes().get(after).copied());
        if before_ok && after_ok {
            let char_start = slice[..abs].chars().count();
            let char_end = char_start + word.chars().count();
            return Some(SourceRange {
                start_line: span.start_line,
                start_character: line_start as u32 + char_start as u32,
                end_line: span.end_line,
                end_character: line_start as u32 + char_end as u32,
            });
        }
        search_at = abs + 1;
    }
    None
}

fn is_ident_byte(b: Option<u8>) -> bool {
    b.is_some_and(|b| b.is_ascii_alphanumeric() || b == b'_')
}

/// Push a definition shell span (refined to the declared name during merge) plus optional specializes type.
pub fn push_ident_definition_spans(
    span: &Span,
    specializes_span: Option<&Span>,
    token_type: u32,
    out: &mut Vec<(SourceRange, u32)>,
) {
    out.push((span_to_source_range(span), token_type));
    if let Some(s) = specializes_span {
        out.push((span_to_source_range(s), TYPE_TYPE));
    }
}

/// Push usage name/type ranges using parser spans when present, otherwise source lookup in the node span.
pub fn push_usage_name_type_spans<T: TypeNameRef + ?Sized>(
    source: &str,
    node_span: &Span,
    name: &str,
    type_name: Option<&T>,
    name_span: Option<&Span>,
    type_span: Option<&Span>,
    out: &mut Vec<(SourceRange, u32)>,
) {
    if let Some(s) = name_span {
        out.push((span_to_source_range(s), TYPE_PROPERTY));
    } else if let Some(r) = word_range_within_span(source, &span_to_source_range(node_span), name) {
        out.push((r, TYPE_PROPERTY));
    }
    if let Some(s) = type_span {
        out.push((span_to_source_range(s), TYPE_TYPE));
    } else if let Some(type_name) = type_name.map(TypeNameRef::type_name_ref) {
        if let Some(r) = word_range_within_span(source, &span_to_source_range(node_span), type_name) {
            out.push((r, TYPE_TYPE));
        }
    }
}
