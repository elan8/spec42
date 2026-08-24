//! Position arithmetic for editor services: LSP offsets, and the line prefix completion reads.
//!
//! What is *under* the cursor is a syntax question, and `ParsedSource::token_at` and
//! `unit_literal_at` answer it. What remains here counts UTF-16 code units and slices a line,
//! which is protocol arithmetic and knows nothing about SysML.

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
}
