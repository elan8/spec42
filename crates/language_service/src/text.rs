//! Position arithmetic for editor services: LSP offsets, and the line prefix completion reads.
//!
//! What is *under* the cursor is a syntax question, and `ParsedSource::token_at` and
//! `unit_literal_at` answer it. What remains here counts UTF-16 code units and slices a line,
//! which is protocol arithmetic and knows nothing about SysML.

use sysml_query::resolved_slice::TextPosition;

/// Length of `text` in UTF-16 code units — the unit LSP character offsets are expressed in.
///
/// The single owner: hosts computing a character offset must not re-derive it, because a copy
/// that counts `char`s instead silently disagrees on anything outside the basic plane.
pub fn utf16_len(text: &str) -> u32 {
    text.encode_utf16().count() as u32
}

/// Converts an LSP-style (line, character) position to a byte offset in `text`.
/// Positions are expressed in UTF-16 code units, so this helper only returns offsets that
/// land on valid UTF-8 boundaries.
pub fn position_to_byte_offset(source: &str, line: u32, character: u32) -> Option<usize> {
    let lines: Vec<&str> = source.split('\n').collect();
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
/// Where a position in `current` falls in `previous`, an older revision of the same document.
///
/// Lines are matched from both ends: a position on an unchanged line keeps its column on the
/// corresponding line. A position inside the edited lines has no counterpart in `previous`; it
/// maps to the end of the last unchanged line before the edit, or the document start when the
/// edit begins on the first line.
pub fn position_in_previous_revision(
    previous: &str,
    current: &str,
    position: TextPosition,
) -> TextPosition {
    if previous == current {
        return position;
    }
    let previous_lines: Vec<&str> = previous.lines().collect();
    let current_lines: Vec<&str> = current.lines().collect();
    let common_prefix = previous_lines
        .iter()
        .zip(&current_lines)
        .take_while(|(left, right)| left == right)
        .count();
    let common_suffix = previous_lines[common_prefix..]
        .iter()
        .rev()
        .zip(current_lines[common_prefix..].iter().rev())
        .take_while(|(left, right)| left == right)
        .count();
    let line = position.line as usize;
    if line < common_prefix {
        return position;
    }
    let current_suffix_start = current_lines.len() - common_suffix;
    if line >= current_suffix_start {
        let previous_line = line - current_suffix_start + (previous_lines.len() - common_suffix);
        return TextPosition::new(previous_line as u32, position.character);
    }
    match common_prefix.checked_sub(1) {
        Some(anchor) => TextPosition::new(anchor as u32, utf16_len(previous_lines[anchor])),
        None => TextPosition::new(0, 0),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_to_byte_offset() {
        let text = "abc\ndef\nghi";
        assert_eq!(position_to_byte_offset(text, 0, 0), Some(0));
        assert_eq!(position_to_byte_offset(text, 0, 2), Some(2));
        assert_eq!(position_to_byte_offset(text, 1, 0), Some(4));
    }

    #[test]
    fn unchanged_lines_keep_their_position_across_revisions() {
        let previous = "package P {\n    part a;\n}\n";
        let current = "package P {\n    part a;\n    part b : B\n}\n";
        assert_eq!(
            position_in_previous_revision(previous, current, TextPosition::new(1, 5)),
            TextPosition::new(1, 5)
        );
        assert_eq!(
            position_in_previous_revision(previous, current, TextPosition::new(3, 1)),
            TextPosition::new(2, 1),
            "a line after the edit shifts by the lines the edit inserted"
        );
    }

    #[test]
    fn an_edited_line_maps_to_the_end_of_the_line_before_the_edit() {
        let previous = "package P {\n    part a;\n}\n";
        let current = "package P {\n    part a;\n    part b : B\n}\n";
        assert_eq!(
            position_in_previous_revision(previous, current, TextPosition::new(2, 14)),
            TextPosition::new(1, 11)
        );
        assert_eq!(
            position_in_previous_revision("part a;", "part ab;", TextPosition::new(0, 6)),
            TextPosition::new(0, 0),
            "an edit on the first line has no unchanged line before it"
        );
    }

    #[test]
    fn identical_revisions_map_every_position_to_itself() {
        let text = "package P {\n}\n";
        assert_eq!(
            position_in_previous_revision(text, text, TextPosition::new(1, 1)),
            TextPosition::new(1, 1)
        );
    }
}
