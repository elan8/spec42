//! Position and word resolution for LSP (line/character to byte offset, word at cursor, etc.).

#[cfg(test)]
use tower_lsp::lsp_types::{Position, Range};

/// Converts an LSP (line, character) position to a byte offset in `text`.
/// LSP positions are expressed in UTF-16 code units, so this helper only returns offsets that
/// land on valid UTF-8 boundaries.
pub fn position_to_byte_offset(text: &str, line: u32, character: u32) -> Option<usize> {
    let lines: Vec<&str> = text.split('\n').collect();
    let line_str = *lines.get(line as usize)?;
    let target_utf16 = character;
    let mut seen_utf16 = 0u32;
    let mut byte_in_line = line_str.len();

    for (byte_idx, ch) in line_str.char_indices() {
        if seen_utf16 == target_utf16 {
            byte_in_line = byte_idx;
            break;
        }
        seen_utf16 += ch.len_utf16() as u32;
        if seen_utf16 > target_utf16 {
            return None;
        }
    }
    let line_utf16_len = line_str.encode_utf16().count() as u32;
    if seen_utf16 != target_utf16 && target_utf16 != line_utf16_len {
        return None;
    }

    let line_start = lines
        .iter()
        .take(line as usize)
        .map(|l| l.len() + 1)
        .sum::<usize>();
    Some(line_start + byte_in_line)
}

/// Returns the LSP (line, start_char, end_char) and the word at the given position.
/// A word is a contiguous run of identifier characters (alphanumeric, underscore, or `:` for qualified names).
pub fn word_at_position(text: &str, line: u32, character: u32) -> Option<(u32, u32, u32, String)> {
    fn is_ident_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_' || c == ':' || c == '>'
    }
    let line_str = text.lines().nth(line as usize)?;
    let char_in_line = character as usize;
    let line_chars: Vec<char> = line_str.chars().collect();
    if line_chars.is_empty() || char_in_line > line_chars.len() {
        return None;
    }
    let mut start = char_in_line;
    while start > 0 && is_ident_char(line_chars[start - 1]) {
        start -= 1;
    }
    let mut end = char_in_line;
    while end < line_chars.len() && is_ident_char(line_chars[end]) {
        end += 1;
    }
    if start >= end {
        return None;
    }
    let word: String = line_chars[start..end].iter().collect();
    Some((line, start as u32, end as u32, word))
}

/// Returns the text of the line up to (but not including) the given (line, character).
pub fn line_prefix_at_position(text: &str, line: u32, character: u32) -> String {
    let line_str = match text.lines().nth(line as usize) {
        Some(l) => l,
        None => return String::new(),
    };
    line_str.chars().take(character as usize).collect()
}

/// Returns the last token (identifier or keyword prefix) before the cursor for completion.
/// Iterates by character to handle multi-byte UTF-8 correctly.
pub fn completion_prefix(line_prefix: &str) -> &str {
    fn is_ident_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_' || c == ':' || c == '>'
    }
    let trimmed = line_prefix.trim_end();
    let chars: Vec<char> = trimmed.chars().collect();
    if chars.is_empty() {
        return trimmed;
    }
    let mut n_trailing = 0;
    for c in chars.iter().rev() {
        if is_ident_char(*c) {
            n_trailing += 1;
        } else {
            break;
        }
    }
    let start_char_idx = chars.len().saturating_sub(n_trailing);
    let byte_start = trimmed
        .char_indices()
        .nth(start_char_idx)
        .map(|(o, _)| o)
        .unwrap_or(trimmed.len());
    trimmed.get(byte_start..).unwrap_or("")
}

/// Simple position for tests. 0-based line and character.
#[cfg(test)]
#[derive(Debug, Clone)]
pub struct SourcePosition {
    pub line: u32,
    pub character: u32,
    pub length: u32,
}

/// Converts AST source position to an LSP Range.
#[cfg(test)]
pub fn source_position_to_range(pos: &SourcePosition) -> Range {
    Range::new(
        Position::new(pos.line, pos.character),
        Position::new(pos.line, pos.character + pos.length),
    )
}

/// Simple range for tests. 0-based.
#[cfg(test)]
#[derive(Debug, Clone)]
pub struct SourceRange {
    pub start_line: u32,
    pub start_character: u32,
    pub end_line: u32,
    pub end_character: u32,
}

/// Converts AST source range to an LSP Range.
#[cfg(test)]
pub fn source_range_to_range(r: &SourceRange) -> Range {
    Range::new(
        Position::new(r.start_line, r.start_character),
        Position::new(r.end_line, r.end_character),
    )
}

